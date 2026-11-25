// Config block: feature-слой не модифицирует игру без MutationGuard.
// NOTE: Celerity camera port (fov/thirdperson/aspect) выключен из сборки: его
// OverrideView-хук ставился наивным патчем vtable-слота и крашил CS2 при
// инжекте. Вернём после проверки точки входа клиент-мода.
pub mod combat;
pub mod esp;
pub mod movement;
pub mod visuals;

pub use esp::draw_esp;
