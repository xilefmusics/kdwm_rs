use super::super::Backend;
use super::{Event, Key};
use crate::c;

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub is_alt_down: bool,
    pub is_super_down: bool,
    pub is_shift_down: bool,
    pub is_control_down: bool,
    pub is_num_lock: bool,
    pub is_lock: bool,
    pub key: Key,
}

impl KeyEvent {
    pub fn from_xevent(xevent: c::XKeyEvent, backend: &Backend) -> Result<Self, String> {
        let state = xevent.state;
        let is_shift_down = (state & c::ShiftMask) > 0;
        let is_lock = (state & c::LockMask) > 0;
        let is_control_down = (state & c::ControlMask) > 0;
        let is_alt_down = (state & c::Mod1Mask) > 0;
        let is_num_lock = (state & c::Mod2Mask) > 0;
        let is_super_down = (state & c::Mod4Mask) > 0;

        let keycode = backend.keycode_to_keysym((xevent.keycode & 0xFF) as u8)?;
        let key = Key::from_keysym(keycode);

        Ok(Self {
            is_shift_down,
            is_lock,
            is_control_down,
            is_alt_down,
            is_num_lock,
            is_super_down,
            key,
        })
    }
}
