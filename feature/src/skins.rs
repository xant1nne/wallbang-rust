// Config block: Celerity econ port; all schema offsets are aliased below.
// Runs on FrameStageNotify stage 7, never on the Present/render thread.
// Attribute storage uses tier0's allocator; foreign attribute buffers are preserved.
use parking_lot::Mutex;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use wallbang_config::collections::HashMap;
use wallbang_config::settings::AppliedSkin;
use wallbang_config::{offsets::*, signatures::*, skin_catalog};
use wallbang_core::entity::EntityList;
use wallbang_core::mutation::MutationGuard;
use wallbang_core::pawn::Pawn;
use wallbang_core::skin_status as status;
use wallbang_core::state::GameState;
use wallbang_core::{memory, modules, mutation, pattern};

const FRAME_STAGE: i32 = 7;
const REAPPLY_FRAMES: u8 = 1;
const REPAIR_INTERVAL: Duration = Duration::from_millis(250);
const ATTRIBUTE_SIZE: usize = 0x48;
const ATTRIBUTE_COUNT: usize = 3;
const MAX_ATTRIBUTE_COUNT: usize = 5;
const ITEM_NAME_DESCRIPTION: usize = 0x200; // Celerity C_EconItemView cached name pointer.
const ATTRIBUTE_INDEX: usize = 0x30;
const ATTRIBUTE_VALUE: usize = 0x34;
const ATTRIBUTE_INITIAL: usize = 0x38;
const CUSTOM_NAME_SIZE: usize = 161;
const ITEM_SIZE: usize = ITEM_INITIALIZED_TAGS + 1;
const UNSAFE_ENTITY_FLAGS: u32 = 0x2 | 0x4 | 0x8 | 0x10 | 0x200 | 0x400;
// Build 14201 RTTI: C_BaseModelEntity -> CCompositeMaterialOwner at +0x610.
const COMPOSITE_OWNER: usize = 0x610;
// OnDataChanged slot 11 only identifies weapon classes. It is NOT the material
// builder. HUD models consume the weapon's item view via the native builder.
const ON_DATA_CHANGED_SLOT: usize = 11;
const REGEN_COUNT: usize = 0x4B8;
const REGEN_ARRAY: usize = 0x4C0;
const COMPOSITE_COUNT: usize = 0x4A0;
const COMPOSITE_ARRAY: usize = 0x4A8;
const COMPOSITE_OWNER_SIZE: usize = 0x4E0;
const COMPOSITE_REF_SIZE: usize = 0x24;
const MAX_REGEN_ITEMS: i32 = 256;
const MAX_CHILDREN: usize = 64;

struct Backend {
    data_changed: usize,
    build_materials: usize,
    material_context: usize,
    regenerate: usize,
    set_model: usize,
    mesh: usize,
    subclass: usize,
    body: usize,
    alloc: usize,
    free: usize,
}

static BACKEND: OnceLock<Option<Backend>> = OnceLock::new();
static RECORDS: OnceLock<Mutex<HashMap<usize, Record>>> = OnceLock::new();
static AGENT: Mutex<Option<ModelChange>> = Mutex::new(None);
// This capability is retained solely for restoring this backend's own saved
// fields on the engine thread. New application always acquires a fresh guard.
static CLEANUP_GUARD: Mutex<Option<MutationGuard>> = Mutex::new(None);
static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);
static SHUTDOWN_COMPLETE: AtomicBool = AtomicBool::new(false);

fn records() -> &'static Mutex<HashMap<usize, Record>> {
    RECORDS.get_or_init(|| Mutex::new(HashMap::default()))
}

pub fn initialize() -> bool {
    if modules::build_supported() != Some(true) {
        return false;
    }
    BACKEND
        .get_or_init(|| {
            let scan = |signature| pattern::scan_module("client.dll", signature);
            let body_site = scan(SIG_SET_BODY_GROUP_SITE)?;
            let relative = unsafe { memory::read::<i32>(body_site + 1)? };
            Some(Backend {
                data_changed: scan(SIG_ECON_DATA_CHANGED)?,
                build_materials: scan(SIG_BUILD_ECON_MATERIALS)?,
                material_context: scan(SIG_ECON_MATERIAL_CONTEXT)?,
                regenerate: scan(SIG_REGENERATE_SKIN)?,
                set_model: scan(SIG_SET_MODEL)?,
                mesh: scan(SIG_SET_MESH_GROUP)?,
                subclass: scan(SIG_UPDATE_SUBCLASS)?,
                body: (body_site + 5).wrapping_add_signed(relative as isize),
                alloc: modules::export("tier0.dll", c"MemAlloc_AllocFunc")?,
                free: modules::export("tier0.dll", c"MemAlloc_FreeFunc")?,
            })
        })
        .is_some()
}

unsafe fn entity_token(entity: usize) -> Option<u32> {
    let identity = memory::read_ptr_unchecked(entity + ENTITY_IDENTITY)?;
    let flags = memory::read_unchecked::<u32>(identity + IDENTITY_FLAGS)?;
    if flags & UNSAFE_ENTITY_FLAGS != 0 {
        return None;
    }
    let handle = memory::read_unchecked::<u32>(identity + IDENTITY_HANDLE)?;
    (handle != u32::MAX).then_some(handle)
}

unsafe fn resolve(handle: u32) -> Option<usize> {
    if handle == u32::MAX {
        return None;
    }
    let entity = EntityList::slot((handle & 0x7FFF) as usize)?;
    let identity = memory::read_ptr_unchecked(entity + ENTITY_IDENTITY)?;
    let flags = memory::read_unchecked::<u32>(identity + IDENTITY_FLAGS)?;
    let raw = entity_token(entity)?;
    let serial = (raw >> 15).wrapping_sub(flags & 1);
    ((raw & 0x7FFF) | serial << 15 == handle).then_some(entity)
}

unsafe fn model_name(entity: usize) -> Option<String> {
    let node = memory::read_ptr_unchecked(entity + PAWN_SCENE_NODE)?;
    let name = memory::read_ptr_unchecked(node + SCENE_NODE_MODEL_STATE + MODEL_STATE_NAME)?;
    memory::read_cstr_unchecked(name, 260).filter(|s| !s.is_empty())
}

unsafe fn set_model(backend: &Backend, entity: usize, model: &str, _guard: &MutationGuard) {
    if let Ok(path) = CString::new(model) {
        let call: unsafe extern "system" fn(usize, *const i8) =
            std::mem::transmute(backend.set_model);
        call(entity, path.as_ptr());
    }
}

unsafe fn set_mesh(backend: &Backend, entity: usize, mask: u64, _guard: &MutationGuard) {
    if let Some(node) = memory::read_ptr_unchecked(entity + PAWN_SCENE_NODE) {
        let call: unsafe extern "system" fn(usize, u64) = std::mem::transmute(backend.mesh);
        call(node, mask);
    }
}

unsafe fn virtual_function(entity: usize, slot: usize) -> Option<usize> {
    let table = memory::read_ptr_unchecked(entity)?;
    let address = memory::read_ptr_unchecked(table + slot * 8)?;
    let base = modules::module_base("client.dll")?;
    (address >= base && address < base + modules::module_size("client.dll")?).then_some(address)
}

#[derive(Clone, Copy)]
enum RefreshTarget {
    Weapon,
    Hud,
}

unsafe fn refresh(
    backend: &Backend,
    entity: usize,
    item: usize,
    target: RefreshTarget,
    _guard: &MutationGuard,
) -> bool {
    let Some(changed) = virtual_function(entity, ON_DATA_CHANGED_SLOT) else {
        return false;
    };
    if matches!(target, RefreshTarget::Weapon) && changed != backend.data_changed {
        return false;
    }
    if !memory::validate_range(item, ITEM_SIZE) {
        return false;
    }
    let owner = entity + COMPOSITE_OWNER;
    if !validate_material_arrays(owner) {
        return false;
    }
    let before = material_counts(owner);
    let definition = memory::read::<i16>(item + ITEM_DEFINITION).unwrap_or(0);
    let paint =
        memory::read::<i32>(item.saturating_sub(ECON_ITEM) + ECON_FALLBACK_PAINT).unwrap_or(-1);
    let start = Instant::now();
    dispatch_refresh(backend, entity, item);
    let us = start.elapsed().as_micros() as u64;
    REBUILDS.fetch_add(1, Ordering::Relaxed);
    *LAST_REBUILD.lock() = Some(MaterialReadback {
        entity,
        definition,
        paint,
        before,
        after: material_counts(owner),
        us,
    });
    true
}

unsafe fn validate_material_arrays(owner: usize) -> bool {
    if !memory::validate_range(owner, COMPOSITE_OWNER_SIZE) {
        return false;
    }
    for (count_offset, array_offset, element_size) in [
        (REGEN_COUNT, REGEN_ARRAY, 0usize),
        (COMPOSITE_COUNT, COMPOSITE_ARRAY, COMPOSITE_REF_SIZE),
    ] {
        let count = memory::read::<i32>(owner + count_offset).unwrap_or(-1);
        if count == 0 {
            continue;
        }
        if !(1..=MAX_REGEN_ITEMS).contains(&count) {
            return false;
        }
        let array = memory::read_ptr(owner + array_offset).unwrap_or(0);
        if !memory::validate_range(array, count as usize * 8) {
            return false;
        }
        // 4B8/4C0 stores opaque material-service handles. Only 4A0/4A8
        // contains pointers the native owner dereferences (refcount at +20).
        if element_size == 0 {
            continue;
        }
        for index in 0..count as usize {
            let Some(value) = memory::read_ptr(array + index * 8) else {
                return false;
            };
            if value != 0 && !memory::validate_range(value, element_size) {
                return false;
            }
        }
    }
    true
}

unsafe fn dispatch_refresh(backend: &Backend, entity: usize, item: usize) {
    let regenerate: unsafe extern "system" fn(usize, bool) =
        std::mem::transmute(backend.regenerate);
    let context: unsafe extern "system" fn(usize) -> bool =
        std::mem::transmute(backend.material_context);
    let build: unsafe extern "system" fn(usize, usize, bool, bool, usize) =
        std::mem::transmute(backend.build_materials);
    regenerate(entity + COMPOSITE_OWNER, true);
    // Build 14201 callsites 0x829F70 and 0xCA9C6A pass r8d=0 (not workshop
    // preview mode). Arg4 is inverted inside 0x7FEF20 (xor r15b, 1) into the
    // low-priority flag; local default weapons have m_iAccountID=0 so context()
    // returns false unless forced true for the local player's equipment.
    let _ = context(item);
    build(entity, item, false, true, 0);
}

#[derive(Clone, Copy)]
struct MaterialReadback {
    entity: usize,
    definition: i16,
    paint: i32,
    before: [i32; 2],
    after: [i32; 2],
    us: u64,
}
static LAST_REBUILD: Mutex<Option<MaterialReadback>> = Mutex::new(None);
static REBUILDS: AtomicU64 = AtomicU64::new(0);
static RESETS: AtomicU64 = AtomicU64::new(0);
static STAGE_COUNT: AtomicU64 = AtomicU64::new(0);
static STAGE_US: AtomicU64 = AtomicU64::new(0);
static STAGE_MAX_US: AtomicU64 = AtomicU64::new(0);
struct StageTimer(Instant);
impl Drop for StageTimer {
    fn drop(&mut self) {
        let us = self.0.elapsed().as_micros() as u64;
        STAGE_US.fetch_add(us, Ordering::Relaxed);
        STAGE_MAX_US.fetch_max(us, Ordering::Relaxed);
        STAGE_COUNT.fetch_add(1, Ordering::Relaxed);
    }
}
unsafe fn material_counts(owner: usize) -> [i32; 2] {
    [
        memory::read(owner + COMPOSITE_COUNT).unwrap_or(-1),
        memory::read(owner + REGEN_COUNT).unwrap_or(-1),
    ]
}
pub fn take_profile() -> String {
    let calls = STAGE_COUNT.swap(0, Ordering::Relaxed);
    let avg = STAGE_US.swap(0, Ordering::Relaxed) / calls.max(1);
    let max = STAGE_MAX_US.swap(0, Ordering::Relaxed);
    let builds = REBUILDS.swap(0, Ordering::Relaxed);
    let resets = RESETS.swap(0, Ordering::Relaxed);
    let last = LAST_REBUILD
        .lock()
        .map(|r| {
            format!(
                " last=0x{:X} def={} paint={} refs={:?}->{:?} rebuild={}us",
                r.entity, r.definition, r.paint, r.before, r.after, r.us
            )
        })
        .unwrap_or_default();
    format!("skin profile: stages={calls} mean={avg}us max={max}us rebuilds={builds} field_resets={resets}{last}")
}

unsafe fn hud_weapon(pawn: usize, weapon: usize) -> Option<usize> {
    let arms = resolve(memory::read_unchecked::<u32>(pawn + PAWN_HUD_ARMS)?)?;
    let node = memory::read_ptr_unchecked(arms + PAWN_SCENE_NODE)?;
    let mut child = memory::read_ptr_unchecked(node + SCENE_CHILD)?;
    for _ in 0..MAX_CHILDREN {
        if child == 0 {
            break;
        }
        if let Some(entity) = memory::read_ptr_unchecked(child + SCENE_OWNER) {
            if let Some(owner) = memory::read_unchecked::<u32>(entity + ENTITY_OWNER_HANDLE)
                .and_then(|h| resolve(h))
            {
                if owner == weapon {
                    return Some(entity);
                }
            }
        }
        child = memory::read_ptr_unchecked(child + SCENE_SIBLING).unwrap_or(0);
    }
    None
}

struct Saved {
    address: usize,
    bytes: Vec<u8>,
}
struct Record {
    entity: usize,
    token: u32,
    item: usize,
    glove: bool,
    saved: Vec<Saved>,
    allocation: usize,
    original_model: Option<String>,
    original_mesh: u64,
    schedule: ApplySchedule,
    hud: Option<ModelChange>,
}

struct ModelChange {
    entity: usize,
    token: u32,
    original: String,
    mesh: u64,
    desired: String,
    frames: u8,
}
impl ModelChange {
    unsafe fn capture(entity: usize) -> Option<Self> {
        let node = memory::read_ptr(entity + PAWN_SCENE_NODE)?;
        Some(Self {
            entity,
            token: entity_token(entity)?,
            original: model_name(entity)?,
            mesh: memory::read(node + SCENE_NODE_MODEL_STATE + MODEL_STATE_MESH_MASK)?,
            desired: String::new(),
            frames: REAPPLY_FRAMES,
        })
    }
    unsafe fn restore(self, backend: &Backend, guard: &MutationGuard) {
        if entity_token(self.entity) == Some(self.token) {
            set_model(backend, self.entity, &self.original, guard);
            set_mesh(backend, self.entity, self.mesh, guard);
        }
    }
}

unsafe fn sync_agent(backend: &Backend, pawn: usize, desired: Option<&str>, guard: &MutationGuard) {
    let mut agent = AGENT.lock();
    if agent.as_ref().is_some_and(|a| {
        a.entity != pawn || entity_token(a.entity) != Some(a.token) || desired.is_none()
    }) {
        if let Some(change) = agent.take() {
            change.restore(backend, guard);
        }
    }
    let Some(desired) = desired else {
        return;
    };
    if agent.is_none() {
        *agent = ModelChange::capture(pawn);
    }
    if let Some(change) = agent.as_mut() {
        if change.desired != desired || model_name(pawn).as_deref() != Some(desired) {
            change.frames = REAPPLY_FRAMES;
            change.desired = desired.to_owned();
        }
        if change.frames > 0 {
            set_model(backend, pawn, desired, guard);
            change.frames -= 1;
        }
    }
}

#[derive(Clone, PartialEq)]
struct SkinKey {
    definition: i16,
    paint: u32,
    wear: f32,
    seed: i32,
    stattrak: bool,
    name: String,
}

#[derive(Default)]
struct ApplySchedule {
    key: Option<SkinKey>,
    hud: Option<(usize, u32)>,
    revision: u64,
    last_attempt: Option<Instant>,
    pending: bool,
}
impl ApplySchedule {
    fn due(
        &mut self,
        key: &SkinKey,
        hud: Option<(usize, u32)>,
        revision: u64,
        reset: bool,
        now: Instant,
    ) -> bool {
        if self.key.as_ref() != Some(key) || self.hud != hud || self.revision != revision {
            self.key = Some(key.clone());
            self.hud = hud;
            self.revision = revision;
            self.pending = true;
            self.last_attempt = None;
        }
        self.pending |= reset;
        if !self.pending
            || self
                .last_attempt
                .is_some_and(|last| now.duration_since(last) < REPAIR_INTERVAL)
        {
            return false;
        }
        self.last_attempt = Some(now);
        true
    }
    fn complete(&mut self) {
        self.pending = false;
    }
}

impl SkinKey {
    fn from_config(config: &AppliedSkin) -> Option<Self> {
        // Legacy preview indices are deliberately not treated as engine IDs.
        skin_catalog::paint_kit(config.paint_kit_id)?;
        // Glove paintkits use dedicated glove materials, so keep strict compatibility
        // for gloves; weapons and knives accept any valid catalog paintkit ID.
        if is_glove(config.def_index)
            && !skin_catalog::compatible(config.def_index, config.paint_kit_id)
        {
            return None;
        }
        Some(Self {
            definition: config.def_index,
            paint: config.paint_kit_id,
            wear: if config.wear.is_finite() {
                config.wear.clamp(0.0, 1.0)
            } else {
                0.01
            },
            seed: config.seed.clamp(0, 1000),
            stattrak: config.stattrak,
            name: config
                .custom_name
                .chars()
                .filter(|c| c.is_ascii_graphic() || *c == ' ')
                .take(160)
                .collect(),
        })
    }
}

impl Record {
    unsafe fn capture(entity: usize, item: usize, glove: bool) -> Option<Self> {
        let token = entity_token(entity)?;
        if !glove && (virtual_function(entity, ON_DATA_CHANGED_SLOT).is_none()) {
            return None;
        }
        if !memory::validate_range(item, ITEM_SIZE) {
            return None;
        }
        let mut saved = Vec::new();
        let mut save = |address: usize, size: usize| -> Option<()> {
            if !memory::validate_range(address, size) {
                return None;
            }
            saved.push(Saved {
                address,
                bytes: std::slice::from_raw_parts(address as *const u8, size).to_vec(),
            });
            Some(())
        };
        for (offset, size) in [
            (ITEM_RESTORE_MATERIAL, 1),
            (ITEM_DEFINITION, 2),
            (ITEM_QUALITY, 4),
            (ITEM_ID_HIGH, 4),
            (ITEM_ID_LOW, 4),
            (ITEM_INITIALIZED, 1),
            (ITEM_INITIALIZED_TAGS, 1),
            (ITEM_ATTRIBUTE_VECTOR, 16),
            (ITEM_CUSTOM_NAME, CUSTOM_NAME_SIZE),
        ] {
            save(item + offset, size)?;
        }
        if !glove {
            for (offset, size) in [
                (ECON_ATTRIBUTES_INITIALIZED, 1),
                (ECON_OWNER_XUID, 8),
                (ECON_FALLBACK_PAINT, 16),
                (ENTITY_SUBCLASS, 4),
            ] {
                save(entity + offset, size)?;
            }
        }
        let node = memory::read_ptr(entity + PAWN_SCENE_NODE).unwrap_or(0);
        let original_mesh =
            memory::read::<u64>(node + SCENE_NODE_MODEL_STATE + MODEL_STATE_MESH_MASK).unwrap_or(1);
        let original_model = model_name(entity);
        if !glove
            && is_knife(memory::read::<i16>(item + ITEM_DEFINITION)?)
            && original_model.is_none()
        {
            return None;
        }
        Some(Self {
            entity,
            token,
            item,
            glove,
            saved,
            allocation: 0,
            original_model,
            original_mesh,
            schedule: ApplySchedule::default(),
            hud: None,
        })
    }

    unsafe fn alive(&self) -> bool {
        entity_token(self.entity) == Some(self.token)
    }

    unsafe fn matches(&self, key: &SkinKey) -> bool {
        memory::read_unchecked::<bool>(self.item + ITEM_INITIALIZED) == Some(true)
            && memory::read_unchecked::<i16>(self.item + ITEM_DEFINITION) == Some(key.definition)
            && memory::read_unchecked::<u32>(self.item + ITEM_ID_HIGH) == Some(u32::MAX)
            && memory::read_unchecked::<u32>(self.item + ITEM_ID_LOW) == Some(u32::MAX)
            && memory::read_unchecked::<usize>(self.item + ITEM_ATTRIBUTE_VECTOR + 8)
                == Some(self.allocation)
            && self.allocation != 0
            && memory::read_unchecked::<u64>(self.item + ITEM_ATTRIBUTE_VECTOR)
                == Some((ATTRIBUTE_COUNT + if key.stattrak { 2 } else { 0 }) as u64)
            && (self.glove
                || (memory::read_unchecked::<bool>(self.entity + ECON_ATTRIBUTES_INITIALIZED)
                    == Some(true)
                    && memory::read_unchecked::<i32>(self.entity + ECON_FALLBACK_PAINT)
                        == Some(key.paint as i32)
                    && memory::read_unchecked::<i32>(self.entity + ECON_FALLBACK_SEED)
                        == Some(key.seed)
                    && memory::read_unchecked::<f32>(self.entity + ECON_FALLBACK_WEAR)
                        == Some(key.wear)
                    && memory::read_unchecked::<u64>(self.entity + ECON_OWNER_XUID) == Some(0)))
    }

    unsafe fn apply(
        &mut self,
        backend: &Backend,
        key: &SkinKey,
        rebuild: bool,
        guard: &MutationGuard,
    ) -> bool {
        if self.allocation != 0
            && memory::read::<usize>(self.item + ITEM_ATTRIBUTE_VECTOR + 8) != Some(self.allocation)
        {
            // The engine took ownership/replaced the vector. Never write/free the
            // old pointer again; snapshot its replacement before attaching ours.
            self.allocation = 0;
            if let Some(saved) = self
                .saved
                .iter_mut()
                .find(|saved| saved.address == self.item + ITEM_ATTRIBUTE_VECTOR)
            {
                let Some(current) = memory::read::<[u8; 16]>(saved.address) else {
                    return false;
                };
                saved.bytes = current.to_vec();
            }
        }
        if self.allocation == 0 {
            let allocate: unsafe extern "system" fn(usize) -> usize =
                std::mem::transmute(backend.alloc);
            self.allocation = allocate(ATTRIBUTE_SIZE * MAX_ATTRIBUTE_COUNT);
            if self.allocation == 0 {
                return false;
            }
        }
        let attrs = attributes(key.paint, key.seed, key.wear, key.stattrak);
        guard.write(self.allocation, attrs);
        // Detach (do not free) the original engine attributes; restoration owns them.
        guard.write(self.item + ITEM_ATTRIBUTE_VECTOR + 8, self.allocation);
        guard.write(
            self.item + ITEM_ATTRIBUTE_VECTOR,
            (ATTRIBUTE_COUNT + if key.stattrak { 2 } else { 0 }) as u64,
        );
        guard.write(self.item + ITEM_DEFINITION, key.definition as u16);
        guard.write(self.item + ITEM_ID_HIGH, u32::MAX);
        guard.write(self.item + ITEM_ID_LOW, u32::MAX);
        guard.write(
            self.item + ITEM_QUALITY,
            if key.stattrak { 9i32 } else { 3i32 },
        );
        guard.write(self.item + ITEM_RESTORE_MATERIAL, true);
        guard.write(self.item + ITEM_INITIALIZED, true);
        guard.write(self.item + ITEM_INITIALIZED_TAGS, false);
        let mut name = [0u8; CUSTOM_NAME_SIZE];
        name[..key.name.len()].copy_from_slice(key.name.as_bytes());
        guard.write(self.item + ITEM_CUSTOM_NAME, name);
        if self.glove {
            let body: unsafe extern "system" fn(usize, *const i8, u32) =
                std::mem::transmute(backend.body);
            body(self.entity, c"first_or_third_person".as_ptr(), 1);
            guard.write(self.entity + PAWN_REAPPLY_GLOVES, true);
        } else {
            guard.write(self.entity + ECON_FALLBACK_PAINT, key.paint as i32);
            guard.write(self.entity + ECON_FALLBACK_SEED, key.seed);
            guard.write(self.entity + ECON_FALLBACK_WEAR, key.wear);
            guard.write(
                self.entity + ECON_FALLBACK_STATTRAK,
                if key.stattrak { 0i32 } else { -1i32 },
            );
            guard.write(self.entity + ECON_ATTRIBUTES_INITIALIZED, true);
            guard.write(self.entity + ECON_OWNER_XUID, 0u64);
            if rebuild && is_knife(key.definition) {
                let Some(model) = skin_catalog::model(key.definition) else {
                    return false;
                };
                set_model(backend, self.entity, model, guard);
                guard.write(
                    self.entity + ENTITY_SUBCLASS,
                    token_hash(&key.definition.to_string()),
                );
                let subclass: unsafe extern "system" fn(usize) =
                    std::mem::transmute(backend.subclass);
                subclass(self.entity);
            }
            let mask = mesh_group_mask(key.definition, key.paint);
            if rebuild {
                set_mesh(backend, self.entity, mask, guard);
                if !refresh(
                    backend,
                    self.entity,
                    self.item,
                    RefreshTarget::Weapon,
                    guard,
                ) {
                    return false;
                }
            }
        }
        guard.write(self.item + ITEM_NAME_DESCRIPTION, 0usize);
        true
    }

    unsafe fn restore(self, backend: &Backend, guard: &MutationGuard) {
        if !self.alive() {
            return;
        } // Destroyed entities release their own game allocations.
        let restore_hud = self
            .hud
            .as_ref()
            .filter(|hud| entity_token(hud.entity) == Some(hud.token))
            .map(|hud| hud.entity);
        if let Some(hud) = self.hud {
            hud.restore(backend, guard);
        }
        let current = memory::read::<usize>(self.item + ITEM_ATTRIBUTE_VECTOR + 8).unwrap_or(0);
        for saved in self.saved {
            // If the engine replaced the vector, do not overwrite its new ownership.
            if saved.address == self.item + ITEM_ATTRIBUTE_VECTOR && current != self.allocation {
                continue;
            }
            for (index, value) in saved.bytes.into_iter().enumerate() {
                guard.write(saved.address + index, value);
            }
        }
        if current == self.allocation && self.allocation != 0 {
            let free: unsafe extern "system" fn(usize) = std::mem::transmute(backend.free);
            free(self.allocation);
        }
        if self.glove {
            guard.write(self.entity + PAWN_REAPPLY_GLOVES, true);
        } else {
            if let Some(model) = self.original_model {
                set_model(backend, self.entity, &model, guard);
            }
            let subclass: unsafe extern "system" fn(usize) = std::mem::transmute(backend.subclass);
            subclass(self.entity);
            set_mesh(backend, self.entity, self.original_mesh, guard);
            refresh(
                backend,
                self.entity,
                self.item,
                RefreshTarget::Weapon,
                guard,
            );
            if let Some(hud) = restore_hud {
                refresh(backend, hud, self.item, RefreshTarget::Hud, guard);
            }
        }
    }
}

fn is_knife(definition: i16) -> bool {
    definition == 42 || definition == 59 || (500..=526).contains(&definition)
}

fn is_glove(definition: i16) -> bool {
    definition == 4725 || (5027..=5035).contains(&definition)
}

/// CS2 knives only use mesh group 1; legacy group 2 applies only to guns whose
/// paint kit sets `use_legacy_model = 1`.
fn mesh_group_mask(definition: i16, paint: u32) -> u64 {
    if !is_knife(definition)
        && skin_catalog::paint_kit(paint).is_some_and(|kit| kit.legacy_model)
    {
        2
    } else {
        1
    }
}

fn attributes(
    paint: u32,
    seed: i32,
    wear: f32,
    stattrak: bool,
) -> [u8; ATTRIBUTE_SIZE * MAX_ATTRIBUTE_COUNT] {
    let mut bytes = [0u8; ATTRIBUTE_SIZE * MAX_ATTRIBUTE_COUNT];
    for (index, (definition, value)) in [(6u16, paint as f32), (7, seed as f32), (8, wear)]
        .into_iter()
        .enumerate()
    {
        let start = index * ATTRIBUTE_SIZE;
        bytes[start + ATTRIBUTE_INDEX..start + ATTRIBUTE_INDEX + 2]
            .copy_from_slice(&definition.to_le_bytes());
        bytes[start + ATTRIBUTE_VALUE..start + ATTRIBUTE_VALUE + 4]
            .copy_from_slice(&value.to_le_bytes());
        bytes[start + ATTRIBUTE_INITIAL..start + ATTRIBUTE_INITIAL + 4]
            .copy_from_slice(&value.to_le_bytes());
    }
    if stattrak {
        // Valve attributes 80/81 are stored-as-integer; both initial values are zero.
        for (slot, id) in [(3, 80u16), (4, 81u16)] {
            let start = slot * ATTRIBUTE_SIZE + ATTRIBUTE_INDEX;
            bytes[start..start + 2].copy_from_slice(&id.to_le_bytes());
        }
    }
    bytes
}

fn token_hash(text: &str) -> u32 {
    let data = text.as_bytes();
    let mut hash = 0x31415926u32 ^ data.len() as u32;
    let mut chunks = data.chunks_exact(4);
    for chunk in &mut chunks {
        let mut word = u32::from_le_bytes(chunk.try_into().unwrap()).wrapping_mul(0x5BD1E995);
        word ^= word >> 24;
        hash = hash.wrapping_mul(0x5BD1E995) ^ word.wrapping_mul(0x5BD1E995);
    }
    let tail = chunks.remainder();
    for (i, byte) in tail.iter().enumerate() {
        hash ^= (*byte as u32) << (i * 8);
    }
    if !tail.is_empty() {
        hash = hash.wrapping_mul(0x5BD1E995);
    }
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0x5BD1E995);
    hash ^ (hash >> 15)
}

/// Full-handle checks prevent reapplying cached state to a reused entity slot.
pub fn on_frame_stage(stage: i32, state: &GameState) {
    if SHUTDOWN_REQUESTED.load(Ordering::Acquire) {
        if stage == FRAME_STAGE && !SHUTDOWN_COMPLETE.load(Ordering::Acquire) {
            restore_all();
            SHUTDOWN_COMPLETE.store(true, Ordering::Release);
        }
        return;
    }
    if stage != FRAME_STAGE {
        return;
    }
    let _timer = StageTimer(Instant::now());
    if modules::build_supported() != Some(true) {
        status::update(status::BUILD_MISMATCH, 0, 0, 0);
        return;
    }
    let (allow_writes, skins) = {
        let guard = state.settings();
        (guard.misc.allow_writes, guard.skins.clone())
    };
    if !allow_writes {
        restore_all();
        status::update(status::WRITES_BLOCKED, 0, 0, 0);
        return;
    }
    let Some(guard) = mutation::acquire() else {
        status::update(status::WRITES_BLOCKED, 0, 0, 0);
        return;
    };
    if !initialize() {
        status::update(status::BACKEND_MISSING, 0, 0, 0);
        return;
    }
    let backend = BACKEND.get().and_then(Option::as_ref).unwrap();
    let mut records = records().lock();
    if !skins.enabled {
        status::update(status::DISABLED, 0, 0, 0);
        unsafe {
            sync_agent(backend, 0, None, &guard);
        }
        for (_, record) in records.drain() {
            unsafe {
                record.restore(backend, &guard);
            }
        }
        CLEANUP_GUARD.lock().take();
        return;
    }
    let Some(pawn) = (unsafe { Pawn::local() }) else {
        status::update(status::NO_PAWN, 0, 0, 0);
        return;
    };
    if pawn.health().unwrap_or(0) <= 0 {
        status::update(status::NO_PAWN, 0, 0, 0);
        return;
    }
    let mut wanted = HashMap::default();
    {
        let mut cleanup = CLEANUP_GUARD.lock();
        if cleanup.is_none() {
            *cleanup = mutation::acquire();
        }
    }
    unsafe {
        let agent_model = skins
            .agent_enabled
            .then(|| skin_catalog::model(skins.agent_def_index))
            .flatten()
            .filter(|model| model.starts_with("agents/") && !model.contains("/shared/"));
        sync_agent(backend, pawn.address, agent_model, &guard);
        if let Some(services) = memory::read_ptr_unchecked(pawn.address + PAWN_WEAPON_SERVICES) {
            let vector = services + WEAPON_SERVICES_WEAPONS;
            let count = memory::read_unchecked::<u32>(vector + NET_VECTOR_SIZE).unwrap_or(0) as usize;
            if count <= MAX_OWNED_WEAPONS {
                if let Some(data) = memory::read_ptr_unchecked(vector + NET_VECTOR_DATA) {
                    for i in 0..count {
                        let Some(entity) = memory::read_unchecked::<u32>(data + i * 4)
                            .and_then(|h| resolve(h))
                        else {
                            continue;
                        };
                        let item = entity + ECON_ITEM;
                        let original_def = records
                            .get(&item)
                            .and_then(|r| {
                                r.saved.iter().find(|s| s.address == item + ITEM_DEFINITION)
                            })
                            .map(|s| i16::from_le_bytes([s.bytes[0], s.bytes[1]]))
                            .or_else(|| memory::read_unchecked::<i16>(item + ITEM_DEFINITION))
                            .unwrap_or(0);
                        let config = if is_knife(original_def) && skins.knife.def_index > 0 {
                            Some(&skins.knife)
                        } else {
                            skins.applied.iter().find(|s| s.def_index == original_def)
                        };
                        if let Some(key) = config.and_then(SkinKey::from_config) {
                            wanted.insert(item, (entity, false, key));
                        }
                    }
                }
            }
        }
        if is_glove(skins.gloves.def_index) {
            if let Some(key) = SkinKey::from_config(&skins.gloves) {
                wanted.insert(pawn.address + PAWN_ECON_GLOVES, (pawn.address, true, key));
            }
        }
        let obsolete: Vec<_> = records
            .iter()
            .filter_map(|(&item, record)| {
                (!wanted.contains_key(&item) || !record.alive()).then_some(item)
            })
            .collect();
        for item in obsolete {
            if let Some(record) = records.remove(&item) {
                record.restore(backend, &guard);
            }
        }
        let requested = wanted.len();
        let mut matched = 0usize;
        let mut hud_matched = 0usize;
        for (item, (entity, glove, key)) in wanted {
            if !records.contains_key(&item) {
                let Some(record) = Record::capture(entity, item, glove) else {
                    continue;
                };
                records.insert(item, record);
            }
            let record = records.get_mut(&item).unwrap();
            let reset = !record.matches(&key);
            let hud_entity = if glove {
                None
            } else {
                hud_weapon(pawn.address, entity)
            };
            let hud_changed = hud_entity.is_some_and(|hud| {
                record.hud.as_ref().is_none_or(|saved| {
                    saved.entity != hud || entity_token(hud) != Some(saved.token)
                })
            });
            if reset {
                RESETS.fetch_add(1, Ordering::Relaxed);
            }
            let hud_token = hud_entity.and_then(|hud| entity_token(hud).map(|token| (hud, token)));
            let rebuild = record.schedule.due(
                &key,
                hud_token,
                skins.apply_revision,
                reset,
                Instant::now(),
            );
            if !rebuild && !reset {
                matched += usize::from(!reset);
                hud_matched += usize::from(hud_entity.is_some() && !hud_changed);
                continue;
            }
            // Repair overwritten econ fields immediately. Only expensive model/
            // material work waits for the retry gate; do not leave wrong values
            // in place for 250 ms between network updates.
            if record.apply(backend, &key, rebuild, &guard) {
                let mut hud_model_ready = true;
                if rebuild && !glove {
                    if let Some(hud) = hud_entity {
                        if record.hud.as_ref().is_none_or(|saved| {
                            saved.entity != hud || entity_token(hud) != Some(saved.token)
                        }) {
                            if let Some(previous) = record.hud.take() {
                                previous.restore(backend, &guard);
                            }
                            record.hud = ModelChange::capture(hud);
                        }
                        if record.hud.is_none() {
                            continue;
                        }
                        if is_knife(key.definition) {
                            if let Some(model) = skin_catalog::model(key.definition) {
                                if model_name(hud).as_deref() != Some(model) {
                                    set_model(backend, hud, model, &guard);
                                    hud_model_ready = false;
                                }
                            }
                        }
                        let mask = mesh_group_mask(key.definition, key.paint);
                        set_mesh(backend, hud, mask, &guard);
                        if !refresh(backend, hud, item, RefreshTarget::Hud, &guard) {
                            continue;
                        }
                        hud_matched += 1;
                    }
                }
                matched += usize::from(record.matches(&key));
                if rebuild && hud_model_ready {
                    record.schedule.complete();
                }
            }
        }
        status::update(
            if requested == 0 {
                status::NO_TARGETS
            } else if matched == requested {
                status::FIELDS_MATCH
            } else {
                status::APPLYING
            },
            requested,
            matched,
            hud_matched,
        );
    }
}

fn restore_all() {
    let Some(guard) = CLEANUP_GUARD.lock().take() else {
        return;
    };
    if let Some(backend) = BACKEND.get().and_then(Option::as_ref) {
        unsafe {
            sync_agent(backend, 0, None, &guard);
        }
        for (_, record) in records().lock().drain() {
            unsafe {
                record.restore(backend, &guard);
            }
        }
    }
}

/// Native model/material restoration must run in FrameStageNotify, not the
/// external thread that invokes wb_shutdown.
pub fn request_shutdown() {
    SHUTDOWN_REQUESTED.store(true, Ordering::Release);
    let no_records = RECORDS
        .get()
        .is_none_or(|records| records.lock().is_empty());
    if no_records && AGENT.lock().is_none() {
        SHUTDOWN_COMPLETE.store(true, Ordering::Release);
    }
}

pub fn shutdown_complete() -> bool {
    SHUTDOWN_COMPLETE.load(Ordering::Acquire)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn material_handles_are_opaque_but_refcounted_composites_require_valid_pointers() {
        let mut owner = vec![0u8; COMPOSITE_OWNER_SIZE];
        let address = owner.as_mut_ptr() as usize;
        let handles = [1u64, 0];
        let resource = [0u8; COMPOSITE_REF_SIZE];
        let mut refs = [resource.as_ptr() as usize];
        unsafe {
            memory::write(address + REGEN_COUNT, 2i32);
            memory::write(address + REGEN_ARRAY, handles.as_ptr() as usize);
            memory::write(address + COMPOSITE_COUNT, 1i32);
            memory::write(address + COMPOSITE_ARRAY, refs.as_ptr() as usize);
            assert!(validate_material_arrays(address));
            refs[0] = 1;
            std::hint::black_box(&refs);
            assert!(!validate_material_arrays(address));
            memory::write(address + REGEN_COUNT, MAX_REGEN_ITEMS + 1);
            assert!(!validate_material_arrays(address));
        }
    }
    fn schedule_key() -> SkinKey {
        SkinKey {
            definition: 7,
            paint: 282,
            wear: 0.12,
            seed: 421,
            stattrak: false,
            name: String::new(),
        }
    }
    #[test]
    fn stable_fields_do_not_trigger_an_eight_frame_material_storm() {
        let now = Instant::now();
        let key = schedule_key();
        let mut schedule = ApplySchedule::default();
        assert!(schedule.due(&key, None, 0, false, now));
        schedule.complete();
        for ms in 1..=2000 {
            assert!(!schedule.due(&key, None, 0, false, now + Duration::from_millis(ms)));
        }
        assert!(
            schedule.due(&key, None, 1, false, now + Duration::from_millis(2001)),
            "explicit Apply must refresh unchanged settings"
        );
        schedule.complete();
        assert!(
            schedule.due(
                &key,
                Some((0x10000, 123)),
                1,
                false,
                now + Duration::from_millis(2002)
            ),
            "new HUD must refresh immediately"
        );
    }
    #[test]
    fn failed_builds_and_continuous_network_resets_are_rate_limited() {
        let now = Instant::now();
        let mut schedule = ApplySchedule::default();
        let mut key = schedule_key();
        let mut calls = 0;
        for ms in 0..=1000 {
            if schedule.due(
                &key,
                Some((0x10000, 123)),
                0,
                true,
                now + Duration::from_millis(ms),
            ) {
                calls += 1;
            }
        }
        assert_eq!(
            calls, 5,
            "at most four repairs/sec after the initial attempt"
        );
        key.paint = 302;
        assert!(
            schedule.due(
                &key,
                Some((0x10000, 123)),
                0,
                false,
                now + Duration::from_millis(1001)
            ),
            "new selection must not wait for a retry cooldown"
        );
    }
    static CALLS: Mutex<Vec<(&'static str, usize)>> = Mutex::new(Vec::new());
    unsafe extern "system" fn test_context(item: usize) -> bool {
        CALLS.lock().push(("context", item));
        true
    }
    unsafe extern "system" fn test_build(
        entity: usize,
        item: usize,
        workshop_preview: bool,
        high_priority: bool,
        extra: usize,
    ) {
        assert!(!workshop_preview && high_priority);
        assert_eq!(extra, 0);
        CALLS.lock().extend([("item", item), ("build", entity)]);
    }
    unsafe extern "system" fn test_regen(owner: usize, force: bool) {
        assert!(force);
        CALLS.lock().push(("regenerate", owner));
    }
    #[test]
    fn native_rebuild_clears_the_owner_then_builds_with_the_weapon_item_for_both_targets() {
        let backend = Backend {
            data_changed: 0,
            build_materials: test_build as *const () as usize,
            material_context: test_context as *const () as usize,
            regenerate: test_regen as *const () as usize,
            set_model: 0,
            mesh: 0,
            subclass: 0,
            body: 0,
            alloc: 0,
            free: 0,
        };
        CALLS.lock().clear();
        for entity in [0x10000, 0x30000] {
            unsafe {
                dispatch_refresh(&backend, entity, 0x20000);
            }
        }
        assert_eq!(
            *CALLS.lock(),
            [
                ("regenerate", 0x10000 + COMPOSITE_OWNER),
                ("context", 0x20000),
                ("item", 0x20000),
                ("build", 0x10000),
                ("regenerate", 0x30000 + COMPOSITE_OWNER),
                ("context", 0x20000),
                ("item", 0x20000),
                ("build", 0x30000)
            ]
        );
    }

    #[test]
    fn engine_wear_seed_and_attribute_resets_require_reapply() {
        let mut storage = vec![0u8; ECON_ITEM + ITEM_SIZE + 32];
        let mut attrs = attributes(282, 421, 0.12, false);
        let entity = storage.as_mut_ptr() as usize;
        let item = entity + ECON_ITEM;
        let allocation = attrs.as_mut_ptr() as usize;
        let key = SkinKey {
            definition: 7,
            paint: 282,
            wear: 0.12,
            seed: 421,
            stattrak: false,
            name: String::new(),
        };
        unsafe {
            memory::write(item + ITEM_INITIALIZED, true);
            memory::write(item + ITEM_DEFINITION, 7i16);
            memory::write(item + ITEM_ID_HIGH, u32::MAX);
            memory::write(item + ITEM_ID_LOW, u32::MAX);
            memory::write(item + ITEM_ATTRIBUTE_VECTOR, ATTRIBUTE_COUNT as u64);
            memory::write(item + ITEM_ATTRIBUTE_VECTOR + 8, allocation);
            memory::write(entity + ECON_ATTRIBUTES_INITIALIZED, true);
            memory::write(entity + ECON_FALLBACK_PAINT, 282i32);
            memory::write(entity + ECON_FALLBACK_SEED, 421i32);
            memory::write(entity + ECON_FALLBACK_WEAR, 0.12f32);
        }
        let record = Record {
            entity,
            token: 0,
            item,
            glove: false,
            saved: Vec::new(),
            allocation,
            original_model: None,
            original_mesh: 1,
            schedule: ApplySchedule::default(),
            hud: None,
        };
        unsafe {
            assert!(record.matches(&key));
            memory::write(entity + ECON_FALLBACK_WEAR, 0.5f32);
            assert!(!record.matches(&key));
            memory::write(entity + ECON_FALLBACK_WEAR, 0.12f32);
            memory::write(entity + ECON_FALLBACK_SEED, 0i32);
            assert!(!record.matches(&key));
            memory::write(entity + ECON_FALLBACK_SEED, 421i32);
            memory::write(item + ITEM_ATTRIBUTE_VECTOR + 8, 0usize);
            assert!(!record.matches(&key));
        }
    }
    #[test]
    fn attributes_use_float_values_and_celerity_stride() {
        let bytes = attributes(282, 421, 0.12, false);
        for (i, (id, value)) in [(6u16, 282.0f32), (7, 421.0), (8, 0.12)]
            .into_iter()
            .enumerate()
        {
            let start = i * ATTRIBUTE_SIZE;
            assert_eq!(
                &bytes[start + ATTRIBUTE_INDEX..start + ATTRIBUTE_INDEX + 2],
                &id.to_le_bytes()
            );
            assert_eq!(
                &bytes[start + ATTRIBUTE_VALUE..start + ATTRIBUTE_VALUE + 4],
                &value.to_le_bytes()
            );
            assert_eq!(
                &bytes[start + ATTRIBUTE_INITIAL..start + ATTRIBUTE_INITIAL + 4],
                &value.to_le_bytes()
            );
        }
        assert_eq!(bytes.len(), ATTRIBUTE_SIZE * MAX_ATTRIBUTE_COUNT);
    }
    #[test]
    fn stattrak_includes_the_integer_attributes_from_installed_items_game() {
        let bytes = attributes(282, 421, 0.12, true);
        for (slot, id) in [(3, 80u16), (4, 81u16)] {
            let start = slot * ATTRIBUTE_SIZE;
            assert_eq!(
                &bytes[start + ATTRIBUTE_INDEX..start + ATTRIBUTE_INDEX + 2],
                &id.to_le_bytes()
            );
            assert_eq!(
                &bytes[start + ATTRIBUTE_VALUE..start + ATTRIBUTE_VALUE + 4],
                &0u32.to_le_bytes()
            );
        }
    }
    #[test]
    fn invalid_profile_values_cannot_escape_econ_limits() {
        let config = AppliedSkin {
            def_index: 7,
            paint_kit_id: 282,
            wear: f32::NAN,
            seed: i32::MAX,
            custom_name: "a\0b\nтест".into(),
            ..AppliedSkin::default()
        };
        let key = SkinKey::from_config(&config).unwrap();
        assert_eq!(key.wear, 0.01);
        assert_eq!(key.seed, 1000);
        assert_eq!(key.name, "ab");
        assert!(SkinKey::from_config(&AppliedSkin {
            paint_kit_id: u32::MAX,
            ..config
        })
        .is_none());
    }
}
