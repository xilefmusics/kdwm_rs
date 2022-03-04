use super::super::Backend;
use super::KeyEvent;
use crate::c;

#[derive(Debug, Clone)]
pub enum Event {
    KeyPress(KeyEvent),
}

impl Event {
    pub fn from_xevent(xevent: c::XEvent, backend: &Backend) -> Result<Self, String> {
        match unsafe { xevent.type_ } {
            2 => Ok(Self::KeyPress(KeyEvent::from_xevent(
                unsafe { xevent.xkey },
                backend,
            )?)),
            3 => Err(String::from("KeyRelease not supported")),
            4 => Err(String::from("ButtonPress not supported")),
            5 => Err(String::from("ButtonRelease not supported")),
            6 => Err(String::from("MotionNotify not supported")),
            7 => Err(String::from("EnterNotify not supported")),
            8 => Err(String::from("LeaveNotify not supported")),
            9 => Err(String::from("FocusIn not supported")),
            10 => Err(String::from("FocusOut not supported")),
            11 => Err(String::from("KeymapNotify not supported")),
            12 => Err(String::from("Expose not supported")),
            13 => Err(String::from("GraphicExpose not supported")),
            14 => Err(String::from("NoExpose not supported")),
            15 => Err(String::from("VisibilityNotify not supported")),
            16 => Err(String::from("CreateNotify not supported")),
            17 => Err(String::from("DestroyNotify not supported")),
            18 => Err(String::from("UnmapNotify not supported")),
            19 => Err(String::from("MapNotify not supported")),
            20 => Err(String::from("MapRequest not supported")),
            21 => Err(String::from("ReparentNotify not supported")),
            22 => Err(String::from("ConfigureNotify not supported")),
            23 => Err(String::from("ConfigureRequest not supported")),
            24 => Err(String::from("GravityNotify not supported")),
            25 => Err(String::from("ResizeRequest not supported")),
            26 => Err(String::from("CirculateNotify not supported")),
            27 => Err(String::from("CirculateRequest not supported")),
            28 => Err(String::from("PropertyNotify not supported")),
            29 => Err(String::from("SelectionClear not supported")),
            30 => Err(String::from("SelectionRequest not supported")),
            31 => Err(String::from("SelectionNotify not supported")),
            32 => Err(String::from("ColormapNotify not supported")),
            33 => Err(String::from("ClientMessage not supported")),
            34 => Err(String::from("MappingNotify not supported")),
            35 => Err(String::from("GenericEvent not supported")),
            x => Err(format!("{} is not a XEventType", x)),
        }
    }
}
