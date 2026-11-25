// Config block: клиентские визуальные фичи, которым не нужен UserCmd (только
// запись в сущности). Всё под MutationGuard + allow_writes; без разрешения —
// полный no-op.
use wallbang_config::offsets::{PAWN_SPOTTED, PAWN_SPOTTED_BY_MASK};
use wallbang_core::state::GameState;
use wallbang_core::{memory, mutation};

/// Reveal radar: помечает врагов как «замеченных» (m_bSpotted + маска), как это
/// делает движок для подсказок радара. Пишем только чужих живых пехотинцев.
fn reveal_radar(state: &GameState) {
    let snapshot = state.snapshot();
    if !snapshot.valid {
        return;
    }
    for player in snapshot.players.iter() {
        if player.address == snapshot.local_address || !player.enemy {
            continue;
        }
        unsafe {
            memory::write(player.address + PAWN_SPOTTED, 1u8);
            memory::write(
                player.address + PAWN_SPOTTED_BY_MASK,
                [0xFFFF_FFFFu32, 0xFFFF_FFFFu32],
            );
        }
    }
}

/// Вызывается из Present-кадра после обновления snapshot.
pub fn on_frame(state: &GameState) {
    // Клиентские визуалы: читаем настройки один раз за кадр.
    let want_radar = state.settings().misc.reveal_radar;
    if !want_radar || !mutation::is_allowed() {
        return;
    }
    let Some(_guard) = mutation::acquire() else {
        return;
    };
    reveal_radar(state);
}
