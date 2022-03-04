use crate::c;

#[derive(Debug, Clone)]
pub enum Input {
    SubstructureRedirect,
    SubstructureNotify,
    KeyPress,
}

impl Input {
    fn mask(&self) -> u32 {
        match self {
            Self::SubstructureRedirect => c::SubstructureRedirectMask,
            Self::SubstructureNotify => c::SubstructureNotifyMask,
            Self::KeyPress => c::KeyPressMask,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Backend {
    display: *mut c::_XDisplay,
    screen: i32,
    root_window: c::Window,
}

impl Backend {
    pub fn new() -> Result<Self, String> {
        let display = unsafe { c::XOpenDisplay(std::ptr::null()) };
        if display.is_null() {
            return Err(String::from("Opening Display: display is NULL"));
        }

        let screen = unsafe { c::XDefaultScreen(display) };

        let root_window = unsafe { c::XRootWindow(display, screen) };

        Ok(Self {
            display,
            screen,
            root_window,
        })
    }

    pub fn screen_dimensions(&self) -> Result<(usize, usize), String> {
        if self.is_closed() {
            return Err(String::from("Getting screen_dimensions: display is NULL"));
        }
        Ok((
            unsafe { c::XDisplayWidth(self.display, self.screen) as usize },
            unsafe { c::XDisplayHeight(self.display, self.screen) as usize },
        ))
    }

    pub fn select_inputs(&self, inputs: Vec<Input>) -> Result<(), String> {
        if self.is_closed() {
            return Err(String::from("Selecting Input: display is NULL"));
        }
        let mut mask: u32 = 0;
        for input in inputs {
            mask = mask | input.mask();
        }
        unsafe { c::XSelectInput(self.display, self.root_window, mask as i64) };
        Ok(())
    }

    pub fn is_closed(&self) -> bool {
        return self.display == std::ptr::null_mut();
    }

    pub fn run(&self) {
        let mut event = c::XEvent {
            xany: c::XAnyEvent {
                type_: 0,
                serial: 0,
                send_event: 0,
                display: self.display,
                window: 0,
            },
        };
        while unsafe { c::XNextEvent(self.display, &mut event) } == 0 {
            let event = Event::from_xevent(event);
            println!("Event {:?}", event);
        }
    }

    pub fn close(&self) -> Result<(), String> {
        if self.is_closed() {
            return Err(String::from("Closing Display: display is NULL"));
        }
        unsafe { c::XCloseDisplay(self.display) };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct KeyPressEvent {
    is_alt_down: bool,
    is_super_down: bool,
    is_shift_down: bool,
    is_control_down: bool,
    is_num_lock: bool,
    is_lock: bool,
    keycode: u32,
}

impl KeyPressEvent {
    fn from_xevent(xevent: c::XKeyEvent) -> Self {
        let keycode = xevent.keycode;

        let is_shift_down = (keycode & c::ShiftMask) > 0;
        let is_lock = (keycode & c::LockMask) > 0;
        let is_control_down = (keycode & c::ControlMask) > 0;
        let is_alt_down = (keycode & c::Mod1Mask) > 0;
        let is_num_lock = (keycode & c::Mod2Mask) > 0;
        let is_super_down = (keycode & c::Mod4Mask) > 0;

        Self {
            is_shift_down,
            is_lock,
            is_control_down,
            is_alt_down,
            is_num_lock,
            is_super_down,
            keycode,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    KeyPress(KeyPressEvent),
}

impl Event {
    fn from_xevent(xevent: c::XEvent) -> Result<Self, String> {
        match unsafe { xevent.type_ } {
            2 => Ok(Self::KeyPress(KeyPressEvent::from_xevent(unsafe {
                xevent.xkey
            }))),
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
