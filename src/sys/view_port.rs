//! Low-level bindings to ViewPort API.

use core::ffi::c_void;

use crate::opaque;

use super::canvas::Canvas;

opaque!(ViewPort);
opaque!(InputEvent);

pub type ViewPortDrawCallback = extern fn(*mut Canvas, *mut c_void);
pub type ViewPortInputCallback = extern fn(*mut InputEvent, *mut c_void);

extern "C" {
    pub fn view_port_alloc() -> *mut ViewPort;
    pub fn view_port_free(view_port: *mut ViewPort);
    pub fn view_port_enabled_set(view_port: *mut ViewPort, enabled: bool);
    pub fn view_port_draw_callback_set(view_port: *mut ViewPort, callback: ViewPortDrawCallback, context: *mut c_void);
    pub fn view_port_input_callback_set(view_port: *mut ViewPort, callback: ViewPortInputCallback, context: *mut c_void);
    pub fn view_port_update(view_port: *mut ViewPort);
}
