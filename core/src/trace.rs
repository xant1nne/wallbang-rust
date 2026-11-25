// Config block: MASK_SOLID, TRACE_SURFACE_STRIDE, TRACE_SURFACE_LIMIT; вся геометрия —
// 1:1 порт Celerity `src/features/shared/world_trace.hpp`.
//
// Только чтение мира: вызываются игровые trace-функции, никаких записей.
use crate::math::Vec3;
use crate::modules;
use crate::pattern::Pattern;
use std::sync::OnceLock;
use wallbang_config::signatures::{
    SIG_TRACE_CREATE, SIG_TRACE_GET_INFO, SIG_TRACE_INIT_DATA, SIG_TRACE_INIT_FILTER,
    SIG_TRACE_INIT_INFO,
};

/// MASK_SOLID из Celerity: solid world + players + windows.
pub const MASK_SOLID: u64 = 0x2014003;
/// Размер одной записи поверхности внутри trace-массива.
const TRACE_SURFACE_STRIDE: usize = 0x38;
/// Индекс поверхности должен быть меньше этого значения.
const TRACE_SURFACE_LIMIT: usize = 0x80;
/// Максимальное количество слотов в массиве трассы (как в C++ вызове create(..., 4, true)).
const TRACE_MAX_SURFACES: usize = 4;

#[repr(C)]
struct TraceInfo {
    start_frac: f32,
    end_frac: f32,
    damage: f32,
    max_secondary_traces: i32,
    surf_start: u16,
    handle: u16,
    flags: u8,
    _pad: [u8; 3],
}

#[repr(C, align(16))]
struct TraceData {
    _pad0: [u8; 0x8],
    array_ptr: *mut u8,
    _pad10: [u8; 0x1C10],
    current_surface: i32,
    _pad1c24: [u8; 0x4],
    trace_info: *mut TraceInfo,
    total_surfaces: i32,
    _pad_rest: [u8; 0x12C],
}

#[repr(C, align(16))]
struct TraceFilter {
    data: [u8; 164],
}

#[repr(C, align(16))]
struct GameTrace {
    surface: *mut u8,
    hit_entity: *mut u8,
    hitbox_data: *mut u8,
    _pad1: [u8; 0x38],
    contents: u32,
    _pad2: [u8; 0x24],
    start_pos: Vec3,
    end_pos: Vec3,
    normal: Vec3,
    _pad3: [u8; 0x10],
    fraction: f32,
    _pad4: [u8; 0x0B],
    all_solid: bool,
    _pad5: [u8; 0x4],
}

type FnInitData = unsafe extern "system" fn(*mut TraceData);
type FnInitInfo = unsafe extern "system" fn(*mut GameTrace);
type FnInitFilter = unsafe extern "system" fn(*mut TraceFilter, usize, u64, i32, i32) -> *mut u8;
type FnCreate =
    unsafe extern "system" fn(*mut TraceData, Vec3, Vec3, *mut TraceFilter, i32, bool) -> bool;
type FnGetInfo = unsafe extern "system" fn(*mut TraceData, *mut GameTrace, f32, *mut u8);

#[derive(Clone, Copy, Default)]
struct TraceFns {
    init_data: Option<FnInitData>,
    init_info: Option<FnInitInfo>,
    init_filter: Option<FnInitFilter>,
    create: Option<FnCreate>,
    get_info: Option<FnGetInfo>,
}

#[derive(Clone, Copy, Debug)]
pub struct TraceResult {
    pub fraction: f32,
    pub normal: Vec3,
}

impl Default for TraceResult {
    fn default() -> Self {
        Self {
            fraction: 1.0,
            normal: Vec3::default(),
        }
    }
}

/// Резолвит пять trace-функций один раз; `None`-поля означают, что подпись не
/// нашлась и трассировка недоступна (edgejump/jumpbug тогда просто не работают).
fn functions() -> &'static TraceFns {
    static FNS: OnceLock<TraceFns> = OnceLock::new();
    FNS.get_or_init(|| {
        let base = match modules::module_base("client.dll") {
            Some(base) => base,
            None => return TraceFns::default(),
        };
        let size = match modules::module_size("client.dll") {
            Some(size) => size,
            None => return TraceFns::default(),
        };
        let resolve = |text: &str| -> usize {
            Pattern::parse(text)
                .and_then(|pattern| pattern.scan(base, size))
                .unwrap_or(0)
        };
        let fns = TraceFns {
            init_data: unsafe {
                std::mem::transmute::<usize, Option<FnInitData>>(resolve(SIG_TRACE_INIT_DATA))
            },
            init_info: unsafe {
                std::mem::transmute::<usize, Option<FnInitInfo>>(resolve(SIG_TRACE_INIT_INFO))
            },
            init_filter: unsafe {
                std::mem::transmute::<usize, Option<FnInitFilter>>(resolve(
                    SIG_TRACE_INIT_FILTER,
                ))
            },
            create: unsafe {
                std::mem::transmute::<usize, Option<FnCreate>>(resolve(SIG_TRACE_CREATE))
            },
            get_info: unsafe {
                std::mem::transmute::<usize, Option<FnGetInfo>>(resolve(SIG_TRACE_GET_INFO))
            },
        };
        fns
    })
}

/// Порт `world_trace::get()`: все пять функций обязательны.
#[inline]
pub fn available() -> bool {
    let fns = functions();
    fns.init_data.is_some()
        && fns.init_info.is_some()
        && fns.init_filter.is_some()
        && fns.create.is_some()
        && fns.get_info.is_some()
}

/// 1:1 порт `world_trace::trace`: трассирует отрезок `from -> from+delta`,
/// пропуская `skip_pawn`. Возвращает fraction и нормаль поверхности.
pub fn trace(from: Vec3, delta: Vec3, skip_pawn: usize, mask: u64) -> TraceResult {
    let fns = functions();
    let (
        Some(init_data),
        Some(init_info),
        Some(init_filter),
        Some(create),
        Some(get_info),
    ) = (
        fns.init_data,
        fns.init_info,
        fns.init_filter,
        fns.create,
        fns.get_info,
    )
    else {
        return TraceResult::default();
    };
    unsafe {
        let mut data: TraceData = std::mem::zeroed();
        let mut filter: TraceFilter = std::mem::zeroed();
        let mut game_trace: GameTrace = std::mem::zeroed();
        init_data(&mut data);
        init_filter(
            &mut filter,
            skip_pawn,
            mask,
            TRACE_MAX_SURFACES as i32,
            7,
        );
        create(
            &mut data,
            from,
            delta,
            &mut filter,
            TRACE_MAX_SURFACES as i32,
            true,
        );
        init_info(&mut game_trace);
        if data.total_surfaces > 0 && !data.trace_info.is_null() {
            let entry = data.trace_info;
            let start_frac = (*entry).start_frac;
            let surface_index = ((*entry).handle & 0x7FFF) as usize;
            if surface_index < TRACE_SURFACE_LIMIT && !data.array_ptr.is_null() {
                let surface_entry = data.array_ptr.add(surface_index * TRACE_SURFACE_STRIDE);
                get_info(&mut data, &mut game_trace, start_frac, surface_entry);
                let fraction = game_trace.fraction;
                if fraction.is_finite() {
                    return TraceResult {
                        fraction,
                        normal: game_trace.normal,
                    };
                }
            }
        }
    }
    TraceResult::default()
}

/// Порт `world_trace::trace_fraction`.
#[inline]
pub fn trace_fraction(from: Vec3, delta: Vec3, skip_pawn: usize, mask: u64) -> f32 {
    trace(from, delta, skip_pawn, mask).fraction
}
