use crate::c;

use super::{Event, Input};
use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Backend {
    display: *mut c::_XDisplay,
    screen: i32,
    root_window: c::Window,
    running: Cell<bool>,
}

impl Backend {
    pub fn new() -> Result<Self, String> {
        let display = unsafe { c::XOpenDisplay(std::ptr::null()) };
        if display.is_null() {
            return Err(String::from("Opening Display: display is NULL"));
        }

        let screen = unsafe { c::XDefaultScreen(display) };

        let root_window = unsafe { c::XRootWindow(display, screen) };

        let running = Cell::new(false);

        Ok(Self {
            display,
            screen,
            root_window,
            running,
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

    pub fn keycode_to_keysym(&self, keycode: u8) -> Result<u64, String> {
        let mut keysyms_per_keycode_return: i32 = 0;

        let ptr = unsafe {
            c::XGetKeyboardMapping(self.display, keycode, 1, &mut keysyms_per_keycode_return)
        };
        if ptr == std::ptr::null_mut() {
            return Err(String::from("keycode_to_keysym: keysym is NULL"));
        }
        let keysym = (unsafe { *ptr }).clone();
        unsafe { c::XFree(ptr as *mut core::ffi::c_void) };
        Ok(keysym)
    }

    pub fn is_closed(&self) -> bool {
        return self.display == std::ptr::null_mut();
    }

    pub fn run<F>(&self, callback: F) -> Result<(), String>
    where
        F: Fn(&Self, Event),
    {
        self.running.set(true);

        let mut event = c::XEvent {
            xany: c::XAnyEvent {
                type_: 0,
                serial: 0,
                send_event: 0,
                display: self.display,
                window: 0,
            },
        };
        while unsafe { c::XNextEvent(self.display, &mut event) } == 0 && self.running.get() {
            callback(self, Event::from_xevent(event, self)?);
        }
        Ok(())
    }

    pub fn stop(&self) {
        self.running.set(false);
    }

    pub fn close(&self) -> Result<(), String> {
        if self.is_closed() {
            return Err(String::from("Closing Display: display is NULL"));
        }
        unsafe { c::XCloseDisplay(self.display) };
        Ok(())
    }
}
