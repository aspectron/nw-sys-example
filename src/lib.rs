use wasm_bindgen::prelude::*;
use nwjs_sys::nw::{nw, try_nw};
use workflow_log::log_trace;
use workflow_wasm::listener::Listener;
use nwjs_sys::result::Result;
use nwjs_sys::window::NWWindow;
use std::sync::{Arc, Mutex};

static mut APP:Option<Arc<ExampleApp>> = None;

pub struct ExampleApp{
    pub win_listeners:Arc<Mutex<Vec<Listener<NWWindow>>>>
}

impl ExampleApp{
    fn new()->Arc<Self>{
        let app = Arc::new(Self{
            win_listeners:Arc::new(Mutex::new(vec![]))
        });

        unsafe{
            APP = Some(app.clone());
        };

        app
    }

    fn create_window(&self)->Result<()>{
        let options = nw::window::Options::new()
            .title("Test page")
            .width(200)
            .height(200)
            .left(0);

        let listener = Listener::new(|win:NWWindow|->std::result::Result<(), JsValue>{
            log_trace!("win: {:?}", win);
            log_trace!("win.x: {:?}", win.x());
            win.move_by(300, 0);
            win.set_x(1);
            win.set_y(1);

            log_trace!("win.title: {}", win.title());
            win.set_title("Hello");
            log_trace!("win.set_title(\"Hello\")");
            log_trace!("win.title: {}", win.title());

            Ok(())
        });

        nw::Window::open_with_options_and_callback("page1.html", &options, listener.into_js());

        log_trace!("nw.Window.open(\"page1.html\", {})", options);

        self.win_listeners.lock()?.push(listener);

        Ok(())
    }
}


#[wasm_bindgen]
pub fn initialize()->Result<()>{
    let nw = try_nw().expect("NW Object not found");
    log_trace!("nw: {:?}", nw);

    let app = ExampleApp::new();

    nw::Window::open("home.html");
    log_trace!("nw.Window.open(\"home.html\")");

    let window = nw::Window::get();
    log_trace!("nw.Window.get(): {:?}", window);

    //nw::Window::open("page1.html");
    //log_trace!("nw.Window.open(\"page1.html\")");

    
    /*
    let options = nw::window::Options::new()
        .title("Test page")
        .width(500)
        .height(600);

    nw::Window::open_with_options("page1.html", &options);
    log_trace!("nw.Window.open(\"page1.html\", {})", options);
    */
    

    app.create_window()?;
    
    Ok(())
}
