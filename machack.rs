extern mod cocoa;

use glut::mouse_callback_tls_key;

use core::cast::transmute;
use core::local_data::local_data_get;
use machack::cocoa::base::{SEL, class_addMethod, id, msg_send_double, objc_getClass};
use machack::cocoa::base::{sel_registerName};

extern fn scrollWheelImpl(this: id, _cmd: SEL, event: id) {
    unsafe {
        let sel_deltaY = sel_registerName(transmute(&"deltaY"[0]));
        let delta_y = msg_send_double(event, sel_deltaY);
        let button = if delta_y == 0.0 {
            return
        } else if delta_y > 0.0 {
            3
        } else {
            4
        };
        let callback = local_data_get(mouse_callback_tls_key).get();
        (*callback)(button, 1, 0, 0);
    }
}

pub fn perform_scroll_wheel_hack() {
    unsafe {
        let sel_scrollWheel = sel_registerName(transmute(&"scrollWheel:"[0]));
        let class_GLUTView = objc_getClass(transmute(&"GLUTView"[0]));
        class_addMethod(class_GLUTView,
                        sel_scrollWheel,
                        transmute(scrollWheelImpl),
                        transmute(&"v@:@"[0]));
    }
}

