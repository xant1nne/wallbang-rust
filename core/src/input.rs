// Config block: INPUT_RELATIVE_MOUSE_FLAG_OFFSET, INPUT_SET_RELATIVE_MOUSE_VFUNC.
use parking_lot::Mutex;

const MOUSEEVENTF_MOVE: u32 = 0x0001;
const INPUT_MOUSE: u32 = 0;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct MouseInput {
    dx: i32,
    dy: i32,
    mouse_data: u32,
    dw_flags: u32,
    time: u32,
    extra_info: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Input {
    kind: u32,
    _pad: u32,
    mouse: MouseInput,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[link(name = "user32")]
extern "system" {
    fn SendInput(count: u32, inputs: *const Input, size: i32) -> u32;
    fn GetAsyncKeyState(key: i32) -> i16;
    fn GetCursorPos(point: *mut Point) -> i32;
    fn SetCursorPos(x: i32, y: i32) -> i32;
}

/// Дробный остаток, чтобы потерянные subpixel-дели не копили ошибку.
#[derive(Debug, Default)]
pub struct MouseDeltas {
    rem_x: f32,
    rem_y: f32,
}

impl MouseDeltas {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn send(&mut self, dx: f32, dy: f32) -> (i32, i32) {
        self.rem_x += dx;
        self.rem_y += dy;
        let ix = self.rem_x.trunc() as i32;
        let iy = self.rem_y.trunc() as i32;
        self.rem_x -= ix as f32;
        self.rem_y -= iy as f32;
        if ix != 0 || iy != 0 {
            let mut input = Input {
                kind: INPUT_MOUSE,
                _pad: 0,
                mouse: MouseInput {
                    dx: ix,
                    dy: iy,
                    mouse_data: 0,
                    dw_flags: MOUSEEVENTF_MOVE,
                    time: 0,
                    extra_info: 0,
                },
            };
            unsafe {
                SendInput(
                    1,
                    &input as *const Input,
                    std::mem::size_of::<Input>() as i32,
                );
            }
            let _ = &mut input;
        }
        (ix, iy)
    }
}

static MOUSE: Mutex<Option<MouseDeltas>> = Mutex::new(None);

#[inline]
pub fn send_mouse_delta(dx: f32, dy: f32) -> (i32, i32) {
    let mut guard = MOUSE.lock();
    let state = guard.get_or_insert_with(MouseDeltas::new);
    state.send(dx, dy)
}

#[inline]
pub fn key_down(vk: u16) -> bool {
    unsafe { (GetAsyncKeyState(vk as i32) as u16 & 0x8000) != 0 }
}

#[inline]
pub fn cursor_position() -> Option<Point> {
    let mut p = Point::default();
    if unsafe { GetCursorPos(&mut p) } != 0 {
        Some(p)
    } else {
        None
    }
}

#[inline]
pub fn set_cursor_position(p: Point) {
    unsafe {
        SetCursorPos(p.x, p.y);
    }
}

/// Возврат相对ного режима мыши и курсора при открытии/закрытии меню.
pub struct CursorController {
    saved_cursor: Option<Point>,
    last_open: bool,
}

impl Default for CursorController {
    fn default() -> Self {
        Self::new()
    }
}

impl CursorController {
    pub fn new() -> Self {
        Self {
            saved_cursor: None,
            last_open: false,
        }
    }

    pub fn sync(&mut self, open: bool) {
        if open == self.last_open {
            return;
        }
        if open {
            self.saved_cursor = cursor_position();
            // ВАЖНО: вызывать inputsystem vfunc по непроверенному индексу опасно —
            // ударяло process при открытии меню по INSERT. Пока используем только Win32.
        } else if let Some(cursor) = self.saved_cursor.take() {
            set_cursor_position(cursor);
        }
        self.last_open = open;
    }
}
