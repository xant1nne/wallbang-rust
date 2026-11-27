// Config block: feature-слой не модифицирует игру без MutationGuard.
// NOTE: Celerity camera port (fov/thirdperson/aspect) выключен из сборки: его
// OverrideView-хук ставился наивным патчем vtable-слота и крашил CS2 при
// инжекте. Вернём после проверки точки входа клиент-мода.
pub mod c3;
pub mod clantag;
pub mod combat;
pub mod cvar;
pub mod esp;
pub mod event_feed;
pub mod movement;
pub mod skins;
pub mod visuals;
pub mod world;
pub mod votes;

pub use esp::draw_esp;
