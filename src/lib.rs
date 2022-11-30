use wasm_bindgen::prelude::*;
use nwjs_sys::nw::{nw, try_nw};
use workflow_log::log_trace;

#[wasm_bindgen]
pub fn initialize() {
    let nw = try_nw().expect("NW Object not found");
    log_trace!("nw: {:?}", nw);

    nw::Window::open("home.html");
    log_trace!("nw.Window.open(\"home.html\")");

    let window = nw::Window::get();
    log_trace!("nw.Window.get(): {:?}", window);

    nw::Window::open("page1.html");
    log_trace!("nw.Window.open(\"page1.html\")");

    let options = nw::window::WindowOptions::new()
        .title("Test page")
        .width(500)
        .height(600);

    nw::Window::open_with_options("page1.html", &options);
    log_trace!("nw.Window.open(\"page1.html\", {})", options);
}
