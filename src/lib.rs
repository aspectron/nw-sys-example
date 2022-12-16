
use nw_sys::utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use workflow_log::{log_trace, log_info};
use workflow_dom::utils::window;
use nw_sys::result::Result;
use nw_sys::prelude::*;
use workflow_nw::prelude::*;
use workflow_wasm::listener::Listener;

static mut APP:Option<Arc<ExampleApp>> = None;


#[derive(Clone)]
pub struct ExampleApp{
    pub inner:Arc<App>
}


impl ExampleApp{
    fn new()->Result<Arc<Self>>{
        let app = Arc::new(Self{
            inner: App::new()?
        });

        unsafe{
            APP = Some(app.clone());
        };

        Ok(app)
    }

    fn create_window(&self)->Result<()>{
        let options = nw::window::Options::new()
            .title("Test page")
            .width(200)
            .height(200)
            .set("frame", JsValue::from(false))
            .set("transparent", JsValue::from(true))
            .left(0);

        let inner = self.inner.clone();
        self.inner.create_window_with_callback(
            "/root/page2.html", 
            &options,
            move |win|->std::result::Result<(), JsValue>{
                log_trace!("win: {:?}", win);
                log_trace!("win.x: {:?}", win.x());
                win.move_by(300, 0);
                win.set_x(100);
                win.set_y(100);

                win.resize_by(200, 200);
                win.resize_to(400, 400);

                log_trace!("win.title: {}", win.title());
                win.set_title("Another Window");
                log_trace!("win.set_title(\"Another Window\")");
                log_trace!("win.title: {}", win.title());

                let win_clone = win.clone();
                let listener = Listener::new(move |_:JsValue|{
                    log_trace!("win.closed: {:?}", win_clone);
                    win_clone.close_with_force();
                    //remove this listener from app
                    Ok(())
                });

                let win_clone2 = win.clone();
                let maximize_listener = Listener::new(move |_:JsValue|{
                    log_trace!("win.maximize: {:?}", win_clone2);
                    Ok(())
                });

                win.on("close", listener.into_js());
                win.on("maximize", maximize_listener.into_js());

                inner.push_menu_listener(listener)?;
                inner.push_menu_listener(maximize_listener)?;

                Ok(())
            }
        )?;

        Ok(())
    }

    fn create_menu(&self)->Result<()>{

        let this = self.clone();
        let submenu_1 = MenuItemBuilder::new()
            .label("Create window")
            .key("8")
            .modifiers("ctrl")
            .callback(move |_|->std::result::Result<(), JsValue>{
                log_trace!("Create window : menu clicked");
                this.create_window()?;
                Ok(())
            }).build()?;
        
        let submenu_2 = MenuItemBuilder::new()
            .label("Say hello")
            .key("9")
            .modifiers("ctrl")
            .callback(move |_|->std::result::Result<(), JsValue>{
                window().alert_with_message("Hello")?;
                Ok(())
            }).build()?;
        
        let item = MenuItemBuilder::new()
            .label("Top Menu")
            .submenus(vec![submenu_1, menu_separator(), submenu_2])
            .build()?;

        
        MenubarBuilder::new("Example App")
            //.mac_hide_edit(true)
            .mac_hide_window(true)
            .append(item)
            .build(true)?;
        
        Ok(())
    }

    pub fn create_tray_icon(&self)->Result<()>{
        let _tray = TrayIconBuilder::new()
            .icon("resources/icons/tray-icon@2x.png")
            .icons_are_templates(false)
            .callback(|_|{
                window().alert_with_message("Tray Icon click")?;
                Ok(())
            })
            .build()?;
        Ok(())
    }

    pub fn create_tray_icon_with_menu(&self)->Result<()>{

        let submenu_1 = MenuItemBuilder::new()
            .label("Say hi")
            .key("6")
            .modifiers("ctrl")
            .callback(move |_|->std::result::Result<(), JsValue>{
                window().alert_with_message("hi")?;
                Ok(())
            }).build()?;

        let exit_menu = MenuItemBuilder::new()
            .label("Exit")
            .callback(move |_|->std::result::Result<(), JsValue>{
                window().alert_with_message("TODO: Exit")?;
                Ok(())
            }).build()?;

        let _tray = TrayIconBuilder::new()
            .icon("resources/icons/tray-icon@2x.png")
            .icons_are_templates(false)
            .submenus(vec![submenu_1, menu_separator(), exit_menu])
            .build()?;

        Ok(())
    }

    pub fn create_context_menu(self:Arc<Self>)->Result<()>{

        let item_1 = MenuItemBuilder::new()
            .label("Sub Menu 1")
            .callback(move |_|->std::result::Result<(), JsValue>{
                window().alert_with_message("Context menu 1 clicked")?;
                Ok(())
            }).build()?;

        let item_2 = MenuItemBuilder::new()
            .label("Sub Menu 2")
            .callback(move |_|->std::result::Result<(), JsValue>{
                window().alert_with_message("Context menu 2 clicked")?;
                Ok(())
            }).build()?;


        self.inner.create_context_menu(vec![item_1, item_2])?;

        Ok(())
    }

    fn add_shortcut(&self)->Result<()>{
        let shortcut = ShortcutBuilder::new()
            .key("Ctrl+Shift+Q")
            .active(|_|{
                window().alert_with_message("Ctrl+Shift+Q pressed, App will close")?;
                nw::App::quit();
                Ok(())
            })
            .build()?;

        nw::App::register_global_hot_key(&shortcut);

        Ok(())
    }

    fn test_argv()->Result<()>{
        let argv = nw::App::argv()?;
        log_info!("argv: {:?}", argv);
        let full_argv = nw::App::full_argv()?;
        log_info!("full_argv: {:?}", full_argv);
        let filtered_argv = nw::App::filtered_argv()?;
        log_info!("filtered_argv: {:?}", filtered_argv);
        
        /*
        for a in filtered_argv{
            log_info!("\nregexp: {}", a.to_string());
            log_info!("   --remote-debugging-port=9005: {}", a.test("--remote-debugging-port=9005"));
            log_info!("   --url=http://localhost: {}", a.test("--url=http://localhost"));
        }
        */

        Ok(())
    }

}

fn app()->Option<Arc<ExampleApp>>{
    unsafe{APP.clone()}
}

#[wasm_bindgen]
pub fn create_context_menu()->Result<()>{
    if let Some(app) = app(){
        app.create_context_menu()?;
    }else{
        let is_nw = initialize_app()?;
        if !is_nw{
            log_info!("TODO: initialize web-app");
            return Ok(());
        }
        let app = app().expect("Unable to create app");
        app.create_context_menu()?;
    }
    Ok(())
}

#[wasm_bindgen]
pub fn initialize_app()->Result<bool>{
    let is_nw = nw::is_nw();

    let _app = ExampleApp::new()?;
    Ok(is_nw)
}

#[wasm_bindgen]
pub fn initialize()->Result<()>{
    let is_nw = initialize_app()?;
    if !is_nw{
        log_info!("TODO: initialize web-app");
        return Ok(());
    }

    let app = app().expect("Unable to create app");

    app.inner.create_window_with_callback(
        "/root/index.html",
        &nw::window::Options::new().new_instance(false),
        |_win:nw::Window|->std::result::Result<(), JsValue>{
            //app.create_context_menu()?;
            Ok(())
        }
    )?;

    let window = nw::Window::get();
    log_trace!("nw.Window.get(): {:?}", window);

    app.create_menu()?;
    app.create_tray_icon()?;
    app.create_tray_icon_with_menu()?;

    app.add_shortcut()?;

    ExampleApp::test_argv()?;
    
    Ok(())
}

#[wasm_bindgen]
pub fn capture_window(image_id:String)->Result<()>{
    let options = nw::window::CaptureConfig::new()
        .format("png");

    let closure = Closure::new::<Box<dyn FnMut(String)>>(Box::new(move |src|{
        log_info!("src: {:?}", src);
        let el = document().get_element_by_id(&image_id).unwrap();
        let _ = el.set_attribute("src", &src);
    }));

    nw::Window::get().capture_page_with_config(closure.as_ref().unchecked_ref(), &options);

    closure.forget();

    Ok(())
}

#[wasm_bindgen]
pub fn print_window(){
    let options = nw::window::PrintOptions::new()
        .autoprint(false)
        .footer_string("footer message")
        .header_string("header message")
        .scale_factor(150)
        .should_print_backgrounds(true)
        //.margin(nw::window::PrintMargin::Custom(Some(100), None, Some(100), None))
        .margin(nw::window::PrintMargin::Default)
        .landscape(true);
    nw::Window::get().print(&options);
}



#[wasm_bindgen]
pub fn test_shell_open_external(){
    nw::Shell::open_external("https://github.com/nwjs/nw.js");
}
#[wasm_bindgen]
pub fn test_shell_open_item()->Result<()>{
    nw::Shell::open_item("/Users/surindersingh/Documents/dev/as/flow/workflow-dev/workflow/README.md");
    Ok(())
}
#[wasm_bindgen]
pub fn test_shell_show_item()->Result<()>{
    nw::Shell::show_item_in_folder("/Users/surindersingh/Documents/dev/as/flow/workflow-dev/workflow/README.md");
    Ok(())
}

#[wasm_bindgen]
pub fn read_clipboard()->Result<()>{
    let clip = nw::Clipboard::get();
    let types = clip.read_available_types();
    log_info!("clipboard data types: {:?}", types);
    let mut query_list = Vec::new();
    for data_type in types{
        query_list.push(nw::clipboard::DataRead::from((data_type, None)));
    }
    query_list.push(nw::clipboard::DataRead::from(("png".to_string(), None)));

    //log_info!("clipboard query_list: {:?}", query_list);
    let result = clip.read_data_array(query_list)?;
    log_info!("clipboard result: {:?}", result);
    Ok(())
}
