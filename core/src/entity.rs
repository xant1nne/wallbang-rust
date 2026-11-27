// Config block: CHUNK_SLOT_COUNT, CHUNK_SLOT_SIZE из wallbang_config::constants.
use crate::{memory, modules};
use wallbang_config::constants::{CHUNK_SLOT_COUNT, CHUNK_SLOT_SIZE};
use wallbang_config::offsets::{
    DW_ENTITY_LIST, DW_GAME_ENTITY_SYSTEM, DW_HIGHEST_ENTITY_INDEX, ENTITY_LIST_OFFSET,
};

pub const MAX_ENTRIES: usize = 32_768;

pub struct EntityList;

impl EntityList {
    #[inline]
    pub unsafe fn base() -> Option<usize> {
        let client = modules::module_base("client.dll")?;
        memory::read_ptr_unchecked(client + DW_ENTITY_LIST)
    }

    #[inline]
    pub unsafe fn highest_index() -> Option<usize> {
        let client = modules::module_base("client.dll")?;
        let system = memory::read_ptr_unchecked(client + DW_GAME_ENTITY_SYSTEM)?;
        let value = memory::read_unchecked::<i32>(system + DW_HIGHEST_ENTITY_INDEX)?;
        if value < 0 {
            None
        } else {
            Some((value as usize).min(MAX_ENTRIES - 1))
        }
    }

    /// Чанк = 512 слотов по 0x70 байт; первый указатель чанка — CEntityIdentity.
    #[inline]
    pub unsafe fn slot(index: usize) -> Option<usize> {
        if index >= MAX_ENTRIES {
            return None;
        }
        let base = Self::base()?;
        let chunk_ptr =
            memory::read_ptr_unchecked(base + ENTITY_LIST_OFFSET + (index / CHUNK_SLOT_COUNT) * 8)?;
        let slot_address = chunk_ptr + (index % CHUNK_SLOT_COUNT) * CHUNK_SLOT_SIZE;
        memory::read_ptr_unchecked(slot_address)
    }

    /// Single pass over the entity list without per-slot Win32/VirtualQuery calls.
    pub unsafe fn iterate() -> Vec<(usize, usize)> {
        let mut out = Vec::with_capacity(256);
        let Some(client) = modules::module_base("client.dll") else {
            return out;
        };
        let Some(base) = memory::read_ptr_unchecked(client + DW_ENTITY_LIST) else {
            return out;
        };
        let Some(system) = memory::read_ptr_unchecked(client + DW_GAME_ENTITY_SYSTEM) else {
            return out;
        };
        let Some(highest) = memory::read_unchecked::<i32>(system + DW_HIGHEST_ENTITY_INDEX) else {
            return out;
        };
        if highest < 0 {
            return out;
        }
        let highest = (highest as usize).min(MAX_ENTRIES - 1);
        let mut chunk_index = usize::MAX;
        let mut chunk = 0usize;
        for index in 0..=highest {
            let wanted = index / CHUNK_SLOT_COUNT;
            if wanted != chunk_index {
                chunk_index = wanted;
                chunk = memory::read_ptr_unchecked(base + ENTITY_LIST_OFFSET + wanted * 8)
                    .unwrap_or(0);
            }
            if chunk == 0 {
                continue;
            }
            if let Some(instance) = memory::read_ptr_unchecked(
                chunk + (index % CHUNK_SLOT_COUNT) * CHUNK_SLOT_SIZE,
            ) {
                out.push((index, instance));
            }
        }
        out
    }
}
