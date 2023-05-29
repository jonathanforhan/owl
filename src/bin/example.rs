#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate owl;

fn main() {
    // let display: *mut wl_display = unsafe { wl_display_create() };
    //
    // let socket: *const c_char = unsafe { wl_display_add_socket_auto(display) };
    //
    // let msg: &CStr = unsafe { CStr::from_ptr(socket) };
    // let msg = msg.to_str().unwrap();
    //
    // println!("Running Wayland display on {msg}");
    //
    // unsafe { wl_display_run(display) };
    // unsafe { wl_display_destroy(display) };
}
