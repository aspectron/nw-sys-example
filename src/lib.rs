use wasm_bindgen::prelude::*;
use workflow_log::log_trace;
use workflow_wasm::listener::Listener;
use nw_sys::result::Result;
use nw_sys::prelude::*;
use nw_sys::utils;

static mut APP:Option<Arc<ExampleApp>> = None;


#[derive(Clone)]
pub struct ExampleApp{
    pub win_listeners:Arc<Mutex<Vec<Listener<nw::Window>>>>,
    pub menu_listeners:Arc<Mutex<Vec<Listener<JsValue>>>>,
    pub listeners:Arc<Mutex<Vec<Listener<web_sys::MouseEvent>>>>
}


impl ExampleApp{
    fn new()->Arc<Self>{
        let app = Arc::new(Self{
            win_listeners:Arc::new(Mutex::new(vec![])),
            menu_listeners:Arc::new(Mutex::new(vec![])),
            listeners:Arc::new(Mutex::new(vec![]))
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

        let listener = Listener::new(|win:nw::Window|->std::result::Result<(), JsValue>{
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

    fn create_menu(&self)->Result<()>{
        let submenus = nw::Menu::new();
        let this = self.clone();
        let listener = Listener::new(move |_|->std::result::Result<(), JsValue>{
            log_trace!("Create window : menu clicked");
            this.create_window()?;
            Ok(())
        });

        
        let submenus_clone = submenus.clone();
        let listener3 = Listener::new(move |_|->std::result::Result<(), JsValue>{
            log_trace!("Menu 5 clicked");
            Ok(())
        });
        let listener3_clone = listener3.clone();
        let listener2 = Listener::new(move |_|->std::result::Result<(), JsValue>{
            let menu_item = &submenus_clone.items()[2];
            let menu_item_4 = &submenus_clone.items()[3];
            let menu_item_5 = &submenus_clone.items()[4];
            log_trace!("Menu 3 is checked: {:?}", menu_item.checked());
            log_trace!("Menu 3 key: {:?}", menu_item.key());
            menu_item.set_key("0");
            log_trace!("Menu 3 key after set_key(0): {:?}", menu_item.key());
            log_trace!("Menu 4 key: {:?}", menu_item_4.key());
            log_trace!("Menu 4 is enabled: {:?}", menu_item_4.enabled());
            menu_item_4.set_enabled(false);
            log_trace!("Menu 4 is enabled after set_enabled(false): {:?}", menu_item_4.enabled());
            menu_item_5.set_click(listener3_clone.into_js());

            log_trace!("Menu 5 submenu: {:?}", menu_item_5.submenu());
            let menu_options = nw::menu_item::Options::new()
                .label("Sub Menu 1");
            let sub_menu_item_1 = nw::MenuItem::new(&menu_options);
            let submenu = nw::Menu::new();
            submenu.append(&sub_menu_item_1);
            menu_item_5.set_submenu(&submenu);
            log_trace!("Menu 5 submenu: {:?}", menu_item_5.submenu());
            Ok(())
        });

        let menu_options = nw::menu_item::Options::new()
            .label("Create window")
            .key("8")
            .modifiers("ctrl")
            .click(listener.into_js());
        let menu_item_1 = nw::MenuItem::new(&menu_options);

        let menu_item_2 = nw::MenuItem::new(&nw::menu_item::Type::Separator.into());
        
        let menu_options:nw::menu_item::Options = nw::menu_item::Options::new()
            .set_type(nw::menu_item::Type::Checkbox)    
            .label("Menu 3")
            .key("9")
            .modifiers("cmd+shift")
            .click(listener2.into_js());
        let menu_item_3 = nw::MenuItem::new(&menu_options);

        let menu_options = nw::menu_item::Options::new()
            .set_type(nw::menu_item::Type::Checkbox)
            .label("Menu 4")
            .click(listener2.into_js());
        let menu_item_4 = nw::MenuItem::new(&menu_options);

        let menu_options = nw::menu_item::Options::new()
            .label("Menu 5");
        let menu_item_5 = nw::MenuItem::new(&menu_options);


        self.menu_listeners.lock()?.push(listener);
        self.menu_listeners.lock()?.push(listener2);
        self.menu_listeners.lock()?.push(listener3);
        
        submenus.append(&menu_item_1);
        submenus.append(&menu_item_2);
        submenus.append(&menu_item_3);
        submenus.append(&menu_item_4);
        submenus.append(&menu_item_5);

        
        let menu_options = nw::menu_item::Options::new()
            .label("Top Menu")
            .submenu(&submenus);

        let menubar = nw::Menu::new_with_options(&nw::menu::Type::Menubar.into());
        let mac_options = nw::menu::MacOptions::new()
            .hide_edit(true)
            .hide_window(true);
        menubar.create_mac_builtin_with_options("Example App", &mac_options);
        menubar.append(&nw::MenuItem::new(&menu_options));
        
        let window = nw::Window::get();
        window.set_menu(&menubar);
        //window.remove_menu();

        Ok(())
    }

    pub fn create_context_menu(&self)->Result<()>{
        let win = nw::Window::get();
        let dom_win = win.window();
        //log_trace!("dom_win: {}, {:?}", win.title(), dom_win);

        let body = utils::body(Some(dom_win));
        //log_trace!("body.inner_html: {:?}", body.inner_html());

        let listener = Listener::new(move |ev:web_sys::MouseEvent|->std::result::Result<(), JsValue>{
            ev.prevent_default();
            //let x = win.x() + ev.x();
            //let y = win.y() + ev.y();
            log_trace!("win :::: x:{}, y:{}", win.x(), win.y());
            log_trace!("contextmenu :::: x:{}, y:{}", ev.x(), ev.y());
            
            let menu_options = nw::menu_item::Options::new()
                .label("Sub Menu 1");
            let sub_menu_item_1 = nw::MenuItem::new(&menu_options);
            let popup_menu = nw::Menu::new();
            popup_menu.append(&sub_menu_item_1);
            popup_menu.popup(ev.x(), ev.y());
            Ok(())
        });

        body.add_event_listener_with_callback("contextmenu", listener.into_js())?;
        self.listeners.lock()?.push(listener);

        Ok(())
    }
}

fn app()->Arc<ExampleApp>{
    unsafe{APP.clone().unwrap()}
}

#[wasm_bindgen]
pub fn create_context_menu()->Result<()>{
    app().create_context_menu()?;
    Ok(())
}

#[wasm_bindgen]
pub fn initialize_app()->Result<()>{
    let nw = nw::try_nw().expect("NW Object not found");
    log_trace!("nw: {:?}", nw);

    let _app = ExampleApp::new();
    Ok(())
}

#[wasm_bindgen]
pub fn initialize()->Result<()>{
    let nw = nw::try_nw().expect("NW Object not found");
    log_trace!("nw: {:?}", nw);

    let app = ExampleApp::new();

    let listener = Listener::new(|_win:nw::Window|->std::result::Result<(), JsValue>{
        //app.create_context_menu()?;
        Ok(())
    });
    let options = nw::window::Options::new()
        .new_instance(false);
    nw::Window::open_with_options_and_callback("home.html", &options, listener.into_js());
    log_trace!("nw.Window.open(\"home.html\")");

    app.win_listeners.lock()?.push(listener);

    let window = nw::Window::get();
    log_trace!("nw.Window.get(): {:?}", window);

    app.create_menu()?;
    
    Ok(())
}
