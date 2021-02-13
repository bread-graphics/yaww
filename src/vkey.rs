// Generated by yaww-vkey, licensed under MIT/Apache2 License

use gluten_keyboard::Key;
use winapi::{ctypes::c_int, um::winuser};

#[inline]
pub fn convert_vkey_to_key(vk: c_int, extended: bool) -> Option<Key> {
    match (vk, extended) {
        (winuser::VK_BACK, _) => Some(Key::Backspace),
        (winuser::VK_TAB, _) => Some(Key::Tab),
        (winuser::VK_CLEAR, _) => Some(Key::Clear),
        (winuser::VK_RETURN, _) => Some(Key::Enter),
        (winuser::VK_SHIFT, false) => Some(Key::LeftShift),
        (winuser::VK_SHIFT, true) => Some(Key::RightShift),
        (winuser::VK_CONTROL, false) => Some(Key::LeftControl),
        (winuser::VK_CONTROL, true) => Some(Key::RightControl),
        (winuser::VK_MENU, false) => Some(Key::LeftAlt),
        (winuser::VK_MENU, true) => Some(Key::RightAlt),
        (winuser::VK_PAUSE, _) => Some(Key::Pause),
        (winuser::VK_CAPITAL, _) => Some(Key::CapsLock),
        (winuser::VK_KANA, _) => Some(Key::Kana),
        (winuser::VK_SPACE, _) => Some(Key::Space),
        (winuser::VK_PRIOR, _) => Some(Key::PageUp),
        (winuser::VK_NEXT, _) => Some(Key::PageDown),
        (winuser::VK_END, _) => Some(Key::End),
        (winuser::VK_HOME, _) => Some(Key::Home),
        (winuser::VK_LEFT, _) => Some(Key::Left),
        (winuser::VK_UP, _) => Some(Key::Up),
        (winuser::VK_RIGHT, _) => Some(Key::Right),
        (winuser::VK_DOWN, _) => Some(Key::Down),
        (winuser::VK_INSERT, _) => Some(Key::Insert),
        (winuser::VK_DELETE, _) => Some(Key::Delete),
        (winuser::VK_HELP, _) => Some(Key::Help),
        (48, _) => Some(Key::Zero),
        (49, _) => Some(Key::One),
        (50, _) => Some(Key::Two),
        (51, _) => Some(Key::Three),
        (52, _) => Some(Key::Four),
        (53, _) => Some(Key::Five),
        (54, _) => Some(Key::Six),
        (55, _) => Some(Key::Seven),
        (56, _) => Some(Key::Eight),
        (57, _) => Some(Key::Nine),
        (65, _) => Some(Key::A),
        (66, _) => Some(Key::B),
        (67, _) => Some(Key::C),
        (68, _) => Some(Key::D),
        (69, _) => Some(Key::E),
        (70, _) => Some(Key::F),
        (71, _) => Some(Key::G),
        (72, _) => Some(Key::H),
        (73, _) => Some(Key::I),
        (74, _) => Some(Key::J),
        (75, _) => Some(Key::K),
        (76, _) => Some(Key::L),
        (77, _) => Some(Key::M),
        (78, _) => Some(Key::N),
        (79, _) => Some(Key::O),
        (80, _) => Some(Key::P),
        (81, _) => Some(Key::Q),
        (82, _) => Some(Key::R),
        (83, _) => Some(Key::S),
        (84, _) => Some(Key::T),
        (85, _) => Some(Key::U),
        (86, _) => Some(Key::V),
        (87, _) => Some(Key::W),
        (88, _) => Some(Key::X),
        (89, _) => Some(Key::Y),
        (90, _) => Some(Key::Z),
        _ => None,
    }
}

#[inline]
pub fn convert_key_to_vkey(key: Key) -> c_int {
    match key {
        Key::Backspace => winuser::VK_BACK,
        Key::Tab => winuser::VK_TAB,
        Key::Clear => winuser::VK_CLEAR,
        Key::Enter => winuser::VK_RETURN,
        Key::LeftShift => winuser::VK_SHIFT,
        Key::RightShift => winuser::VK_SHIFT,
        Key::LeftControl => winuser::VK_CONTROL,
        Key::RightControl => winuser::VK_CONTROL,
        Key::LeftAlt => winuser::VK_MENU,
        Key::RightAlt => winuser::VK_MENU,
        Key::Pause => winuser::VK_PAUSE,
        Key::CapsLock => winuser::VK_CAPITAL,
        Key::Kana => winuser::VK_KANA,
        Key::Space => winuser::VK_SPACE,
        Key::PageUp => winuser::VK_PRIOR,
        Key::PageDown => winuser::VK_NEXT,
        Key::End => winuser::VK_END,
        Key::Home => winuser::VK_HOME,
        Key::Left => winuser::VK_LEFT,
        Key::Up => winuser::VK_UP,
        Key::Right => winuser::VK_RIGHT,
        Key::Down => winuser::VK_DOWN,
        Key::Insert => winuser::VK_INSERT,
        Key::Delete => winuser::VK_DELETE,
        Key::Help => winuser::VK_HELP,
        Key::Zero => 48,
        Key::One => 49,
        Key::Two => 50,
        Key::Three => 51,
        Key::Four => 52,
        Key::Five => 53,
        Key::Six => 54,
        Key::Seven => 55,
        Key::Eight => 56,
        Key::Nine => 57,
        Key::A => 65,
        Key::B => 66,
        Key::C => 67,
        Key::D => 68,
        Key::E => 69,
        Key::F => 70,
        Key::G => 71,
        Key::H => 72,
        Key::I => 73,
        Key::J => 74,
        Key::K => 75,
        Key::L => 76,
        Key::M => 77,
        Key::N => 78,
        Key::O => 79,
        Key::P => 80,
        Key::Q => 81,
        Key::R => 82,
        Key::S => 83,
        Key::T => 84,
        Key::U => 85,
        Key::V => 86,
        Key::W => 87,
        Key::X => 88,
        Key::Y => 89,
        Key::Z => 90,
        _ => panic!("gluten_keyboard key does not map to winuser key"),
    }
}