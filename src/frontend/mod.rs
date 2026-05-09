mod desktop;

use winit::event::VirtualKeyCode;

pub use desktop::run_window;

#[derive(Debug, Clone)]
pub struct Controls {
    pub a: VirtualKeyCode,
    pub b: VirtualKeyCode,
    pub start: VirtualKeyCode,
    pub select: VirtualKeyCode,
    pub up: VirtualKeyCode,
    pub down: VirtualKeyCode,
    pub left: VirtualKeyCode,
    pub right: VirtualKeyCode,
    pub fast_forward: VirtualKeyCode,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            a: VirtualKeyCode::Z,
            b: VirtualKeyCode::X,
            start: VirtualKeyCode::Return,
            select: VirtualKeyCode::LShift,
            up: VirtualKeyCode::Up,
            down: VirtualKeyCode::Down,
            left: VirtualKeyCode::Left,
            right: VirtualKeyCode::Right,
            fast_forward: VirtualKeyCode::Space,
        }
    }
}

impl Controls {
    pub fn to_env_string(&self) -> String {
        [
            key_to_id(self.a),
            key_to_id(self.b),
            key_to_id(self.start),
            key_to_id(self.select),
            key_to_id(self.up),
            key_to_id(self.down),
            key_to_id(self.left),
            key_to_id(self.right),
            key_to_id(self.fast_forward),
        ]
        .join(",")
    }

    pub fn from_env_string(s: &str) -> Option<Self> {
        let mut it = s.split(',');
        Some(Self {
            a: id_to_key(it.next()?)?,
            b: id_to_key(it.next()?)?,
            start: id_to_key(it.next()?)?,
            select: id_to_key(it.next()?)?,
            up: id_to_key(it.next()?)?,
            down: id_to_key(it.next()?)?,
            left: id_to_key(it.next()?)?,
            right: id_to_key(it.next()?)?,
            fast_forward: id_to_key(it.next()?)?,
        })
    }
}

fn key_to_id(k: VirtualKeyCode) -> &'static str {
    match k {
        VirtualKeyCode::Z => "z",
        VirtualKeyCode::X => "x",
        VirtualKeyCode::C => "c",
        VirtualKeyCode::A => "a",
        VirtualKeyCode::S => "s",
        VirtualKeyCode::D => "d",
        VirtualKeyCode::Q => "q",
        VirtualKeyCode::W => "w",
        VirtualKeyCode::E => "e",
        VirtualKeyCode::R => "r",
        VirtualKeyCode::Up => "up",
        VirtualKeyCode::Down => "down",
        VirtualKeyCode::Left => "left",
        VirtualKeyCode::Right => "right",
        VirtualKeyCode::Return => "return",
        VirtualKeyCode::Space => "space",
        VirtualKeyCode::LShift => "lshift",
        VirtualKeyCode::RShift => "rshift",
        VirtualKeyCode::Tab => "tab",
        VirtualKeyCode::Back => "back",
        _ => "z",
    }
}

fn id_to_key(id: &str) -> Option<VirtualKeyCode> {
    Some(match id {
        "z" => VirtualKeyCode::Z,
        "x" => VirtualKeyCode::X,
        "c" => VirtualKeyCode::C,
        "a" => VirtualKeyCode::A,
        "s" => VirtualKeyCode::S,
        "d" => VirtualKeyCode::D,
        "q" => VirtualKeyCode::Q,
        "w" => VirtualKeyCode::W,
        "e" => VirtualKeyCode::E,
        "r" => VirtualKeyCode::R,
        "up" => VirtualKeyCode::Up,
        "down" => VirtualKeyCode::Down,
        "left" => VirtualKeyCode::Left,
        "right" => VirtualKeyCode::Right,
        "return" => VirtualKeyCode::Return,
        "space" => VirtualKeyCode::Space,
        "lshift" => VirtualKeyCode::LShift,
        "rshift" => VirtualKeyCode::RShift,
        "tab" => VirtualKeyCode::Tab,
        "back" => VirtualKeyCode::Back,
        _ => return None,
    })
}
