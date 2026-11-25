// Config block: Hide shots — подавление mark/tracers при silent-выстреле; без записи в игру.
#[derive(Clone, Copy, Debug, Default)]
pub struct HideShot {
    pub suppress_marker: bool,
    pub suppress_tracer: bool,
    pub suppress_sound: bool,
}

#[inline]
pub fn should_hide(silent: bool, force: bool) -> bool {
    silent || force
}

#[inline]
pub fn build(silent: bool, force: bool) -> HideShot {
    let hide = should_hide(silent, force);
    HideShot {
        suppress_marker: hide,
        suppress_tracer: hide,
        suppress_sound: hide,
    }
}
