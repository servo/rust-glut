#[allow(non_uppercase_statics)];

extern mod cocoa;

use glut::{mouse_callback_tls_key, mouse_wheel_callback_tls_key};

use std::cast::transmute;
use std::libc::{c_int, c_void};
use std::local_data::local_data_get;
use std::ptr::null;
use machack::cocoa::base::{SEL, class_addMethod, id, msg_send_double, msg_send_id, objc_getClass};
use machack::cocoa::base::{sel_registerName};

#[link_args="-framework Carbon"]
extern {
    // Carbon API.
    #[fast_ffi]
    fn GetEventKind(eventRef: *c_void) -> u32;
}

// From CarbonEvents.h.
static kEventMouseScroll: u32 = 11;

extern fn scrollWheelImpl(_this: id, _cmd: SEL, event: id) {
    unsafe {
        // Get the underlying Carbon event to figure out if deviceDelta{Y,X} are available.
        let sel__eventRef = sel_registerName(transmute(&"_eventRef"[0]));
        let eventRef: *c_void = transmute(msg_send_id(event, sel__eventRef));
        let is_scroll = eventRef != null() && GetEventKind(eventRef) == kEventMouseScroll;

        // Use precise scrolling if available; otherwise, use coarse-grained scrolling.
        let (delta_x, delta_y) = if is_scroll {
            let sel_deviceDeltaY = sel_registerName(transmute(&"deviceDeltaY"[0]));
            let delta_y = msg_send_double(event, sel_deviceDeltaY);
            let sel_deviceDeltaX = sel_registerName(transmute(&"deviceDeltaX"[0]));
            let delta_x = msg_send_double(event, sel_deviceDeltaX);
            (delta_x, delta_y)
        } else {
            let sel_deltaY = sel_registerName(transmute(&"deltaY"[0]));
            let delta_y = msg_send_double(event, sel_deltaY);
            let sel_deltaX = sel_registerName(transmute(&"deltaX"[0]));
            let delta_x = msg_send_double(event, sel_deltaX);
            (delta_x * 30.0, delta_y * 30.0)
        };

        // First, try the wheel func.
        match local_data_get(mouse_wheel_callback_tls_key) {
            None => {}  // Continue.
            Some(callback) => {
                (*callback)(0, (delta_y * 10000.0) as c_int, 0, 0);
                (*callback)(1, (delta_x * 10000.0) as c_int, 0, 0);
                return
            }
        }

        // Fall through to the mouse func.
        let button = if delta_y == 0.0 {
            return
        } else if delta_y > 0.0 {
            3
        } else {
            4
        };

        match local_data_get(mouse_callback_tls_key) {
            None => {} // Ignore.
            Some(callback) => (*callback)(button, 1, 0, 0),
        }
    }
}

extern fn magnifyWithEvent(_this: id, _cmd: SEL, event: id) {
    unsafe {
        let sel_magnification = sel_registerName(transmute(&"magnification"[0]));
        let magnification = msg_send_double(event, sel_magnification) + 1.0;

        match local_data_get(mouse_wheel_callback_tls_key) {
            None => {}
            Some(callback) => (*callback)(2, (magnification * 10000.0) as c_int, 0, 0),
        }
    }
}

pub fn perform_scroll_wheel_hack() {
    unsafe {
        let class_GLUTView = objc_getClass(transmute(&"GLUTView"[0]));

        let sel_scrollWheel = sel_registerName(transmute(&"scrollWheel:"[0]));
        class_addMethod(class_GLUTView,
                        sel_scrollWheel,
                        transmute(scrollWheelImpl),
                        transmute(&"v@:@"[0]));

        let sel_magnifyWithEvent = sel_registerName(transmute(&"magnifyWithEvent:"[0]));
        class_addMethod(class_GLUTView,
                        sel_magnifyWithEvent,
                        transmute(magnifyWithEvent),
                        transmute(&"v@:@"[0]));
    }
}

