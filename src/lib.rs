
use nw_sys::utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use workflow_log::log_error;
use workflow_log::{log_trace, log_info};
use workflow_dom::utils::window;
use nw_sys::result::Result;
use nw_sys::prelude::*;
use workflow_nw::prelude::*;
use workflow_wasm::listener::{Callback, CallbackClosure};
use web_sys::HtmlVideoElement;
use workflow_html::{html, Html, Render};

static mut APP:Option<Arc<ExampleApp>> = None;


#[derive(Clone)]
pub struct ExampleApp{
    pub inner:Arc<App>,
    pub htmls:Arc<Mutex<Vec<Html>>>
}


impl ExampleApp{
    fn new()->Result<Arc<Self>>{

        if let Some(app) = app(){
            return Ok(app);
        } 

        let app = Arc::new(Self{
            inner: App::new()?,
            htmls: Arc::new(Mutex::new(Vec::new()))
        });

        unsafe{
            APP = Some(app.clone());
        };

        Ok(app)
    }

    pub fn test_synopsis(&self)->Result<()>{
        
        Ok(())
    }

    fn create_window(&self)->Result<()>{
        let options = nw_sys::window::Options::new()
            .title("Test page")
            .width(200)
            .height(200)
            //.set("frame", JsValue::from(false))
            //.set("transparent", JsValue::from(true))
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

                //test resize by method
                win.resize_by(200, 200);

                //test resize by method
                win.resize_to(400, 400);

                log_trace!("win.title: {}", win.title());
                win.set_title("Another Window");
                log_trace!("win.set_title(\"Another Window\")");
                log_trace!("win.title: {}", win.title());

                let win_clone = win.clone();
                let mut close_callback = Callback::<dyn FnMut()->Result<()>>::new();
                let close_callback_clone = close_callback.clone();
                close_callback.set_closure(move || ->Result<()>{
                    log_trace!("win.closed: {:?}", win_clone);
                    win_clone.close_with_force();
                    let _a = close_callback_clone.clone();
                    //remove this listener from app
                    Ok(())
                });

                let win_clone2 = win.clone();
                let maximize_callback = Callback::<dyn FnMut()>::with_closure(move ||{
                    log_trace!("win.maximize: {:?}", win_clone2);
                });

                win.on("close", close_callback.into_js());
                win.on("maximize", maximize_callback.into_js());

                inner.push_callback(close_callback)?;
                inner.push_callback(maximize_callback)?;

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
                //nw_sys::app::quit();
                nw_sys::app::close_all_windows();
                Ok(())
            })
            .build()?;

        nw_sys::app::register_global_hot_key(&shortcut);
        

        Ok(())
    }

    fn test_app_functions()->Result<()>{
        log_info!("nw_sys::app::start_path(): {:?}", nw_sys::app::start_path());
        log_info!("nw_sys::app::data_path(): {:?}", nw_sys::app::data_path());
        log_info!("nw_sys::app::manifest(): {:?}", nw_sys::app::manifest());
        
        let argv = nw_sys::app::argv()?;
        log_info!("argv: {:?}", argv);
        let full_argv = nw_sys::app::full_argv()?;
        log_info!("full_argv: {:?}", full_argv);
        let filtered_argv = nw_sys::app::filtered_argv()?;
        log_info!("filtered_argv: {:?}", filtered_argv);
        let list:Vec<js_sys::JsString> = filtered_argv.iter().map(|a| a.to_string()).collect();
        log_info!("filtered_argv as Vec<js_sys::JsString>: {:?}", list);
        
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
    let (app, is_nw) = initialize_app()?;
    if !is_nw{
        log_info!("TODO: initialize web-app");
        return Ok(());
    }
    app.create_context_menu()?;
    Ok(())
}


fn initialize_app()->Result<(Arc<ExampleApp>, bool)>{
    let is_nw = nw_sys::is_nw();

    let app = ExampleApp::new()?;
    Ok((app, is_nw))
}

#[wasm_bindgen]
pub fn initialize()->Result<()>{
    let (app, is_nw) = initialize_app()?;
    if !is_nw{
        log_info!("TODO: initialize web-app");
        return Ok(());
    }

    app.inner.create_window_with_callback(
        "/root/index.html",
        &nw_sys::window::Options::new()
            .new_instance(false)
            .width(1040)
            .height(800),
        |_win :nw_sys::Window|->std::result::Result<(), JsValue>{
            //app.create_context_menu()?;
            Ok(())
        }
    )?;

    let window = nw_sys::window::get();
    log_trace!("nw::window::get(): {:?}", window);

    app.create_menu()?;
    app.create_tray_icon()?;
    app.create_tray_icon_with_menu()?;

    app.add_shortcut()?;

    ExampleApp::test_app_functions()?;
    
    Ok(())
}

#[wasm_bindgen]
pub fn test_synopsis()->Result<()>{
    let (app, _) = initialize_app()?;
    app.test_synopsis()?;
    Ok(())
}

#[wasm_bindgen]
pub fn capture_window(image_id:String)->Result<()>{
    let options = nw_sys::window::CaptureConfig::new()
        .format("png");

    let closure = Closure::new::<Box<dyn FnMut(String)>>(Box::new(move |src|{
        log_info!("src: {:?}", src);
        let el = document().get_element_by_id(&image_id).unwrap();
        let _ = el.set_attribute("src", &src);
    }));

    nw_sys::window::get().capture_page_with_config(closure.as_ref().unchecked_ref(), &options);

    closure.forget();

    Ok(())
}

#[wasm_bindgen]
pub fn print_window(){
    let options = nw_sys::window::PrintOptions::new()
        .autoprint(false)
        .footer_string("footer message")
        .header_string("header message")
        .scale_factor(150)
        .should_print_backgrounds(true)
        //.margin(nw::window::PrintMargin::Custom(Some(100), None, Some(100), None))
        .margin(nw_sys::window::PrintMargin::Default)
        .landscape(true);
    nw_sys::window::get().print(&options);
}

#[wasm_bindgen]
pub fn test_shell_open_external(){
    nw_sys::shell::open_external("https://github.com/nwjs/nw.js");
}
#[wasm_bindgen]
pub fn test_shell_open_item()->Result<()>{
    let path = nw_sys::app::start_path();
    log_trace!("path: {:?}", path);
    //TODO: this path fails under compiled app (.app/.exe file)
    nw_sys::shell::open_item(&(path+"/root/index.html"));// path/to/file.txt
    Ok(())
}
#[wasm_bindgen]
pub fn test_shell_show_item()->Result<()>{
    let path = nw_sys::app::start_path();
    log_trace!("path: {:?}", path);
    //TODO: this path fails under compiled app (.app/.exe file)
    nw_sys::shell::show_item_in_folder(&(path+"/root/index.html"));// absolute/path/to/file.txt
    Ok(())
}

#[wasm_bindgen]
pub fn test_clipboard()->Result<()>{
    let clip = nw_sys::clipboard::get();
    let types = clip.get_available_types();
    log_info!("clipboard data types: {:?}", types);
    let mut query_list = Vec::new();
    for data_type in types{
        query_list.push(nw_sys::clipboard::DataRead::from((data_type, None)));
    }
    query_list.push(nw_sys::clipboard::DataRead::from(("png".to_string(), None)));

    //log_info!("clipboard query_list: {:?}", query_list);
    let result = clip.get_data_array(query_list)?;
    log_info!("clip.get_data_array(): {:?}", result);

    log_info!("clip.set(\"Hello World\")");
    clip.set("Hello world");
    log_info!("clip.get(): {:?} should be \"Hello World\"", clip.get());

    Ok(())
}

#[wasm_bindgen]
pub fn read_screens_info()->Result<()>{
    nw_sys::screen::init_once();
    let info = nw_sys::screen::screens()?;
    log_info!("screens infos: {:#?}", info);
    Ok(())
}


fn render_media(video_element_id:String, stream_id:String)->Result<()>{
    log_info!("stream_id: {:?}", stream_id);
    let (app, _) = initialize_app()?;

    let video_constraints = VideoConstraints::new()
        .source_id(&stream_id)
        .max_height(1000);

    let video_el_id = video_element_id.clone();
    workflow_nw::media::get_user_media(
        video_constraints,
        None,
        Arc::new(move |value|{
            //log_info!("get_user_media result: {:?}", value);

            if let Some(media_stream) = value{
                let el = document().get_element_by_id(&video_el_id).unwrap();
                match el.dyn_into::<HtmlVideoElement>(){
                    Ok(el)=>{
                        el.set_src_object(Some(&media_stream));
                    }
                    Err(err)=>{
                        log_error!("Unable to cast element to HtmlVideoElement: element = {:?}", err);
                    }
                }

                let _ = app.inner.set_media_stream(Some(media_stream));
            }
        })
    )?;
    Ok(())
}

#[wasm_bindgen]
pub fn choose_desktop_media(video_element_id:String)->Result<()>{
    let (app, _) = initialize_app()?;

    let callback = Callback::<CallbackClosure<JsValue>>::with_closure(move |value:JsValue|->std::result::Result<(), JsValue>{
        let mut stream_id = None;
        if value.is_string(){
            if let Some(id) = value.as_string(){
                if id.len() > 0{
                    stream_id = Some(id);
                }
            }
        }

        if let Some(stream_id) = stream_id{
            render_media(video_element_id.clone(), stream_id)?;
        }else{
            log_info!("no stream_id"); 
        }
        
        Ok(())
    });

    nw_sys::screen::choose_desktop_media(
        nw_sys::screen::MediaSources::ScreenAndWindow,
        callback.into_js()
    )?;

    app.inner.push_callback(callback)?;

    Ok(())
}


#[wasm_bindgen]
pub fn end_desktop_media()->Result<()>{
    if let Some(app) = app(){
        app.inner.stop_media_stream(None, None)?;
    }
    Ok(())
}

#[wasm_bindgen]
pub fn desktop_capture_monitor(video_element_id:String, container_id:String)->Result<()>{
    let (app, _) = initialize_app()?;

    nw_sys::screen::init_once();

    use nw_sys::screen::desktop_capture_monitor as dcm;
    let container = document().get_element_by_id(&container_id).unwrap();
    let container_id_clone = container_id.clone();
    let view_holder = container.query_selector(".view-holder").unwrap().unwrap();
    let mut cb = Callback::<dyn FnMut(String, String)->Result<()>>::new();
    cb.set_closure(move |id, thumbnail|->Result<()>{
        //log_info!("thumbnailchanged: id:{:?}, thumbnail:{:?}", id, thumbnail);

        let panel_el = document().query_selector(&format!("#{} [data-id=\"{}\"]", &container_id, id)).unwrap();
        if let Some(panel_el) = panel_el{
            let img = panel_el.query_selector("img")?.unwrap();
            img.set_attribute("src", &format!("data:image/png;base64,{}", thumbnail))?;
        }

        Ok(())
    });

    dcm::on("thumbnailchanged", cb.into_js());
    app.inner.push_callback(cb)?;

    let app_clone = app.clone();
    
    let mut cb = Callback::<dyn FnMut(String, String, u16, String)->Result<()>>::new();
    cb.set_closure(move |id:String, name:String, _order, w_type|->Result<()>{
        log_info!("added: id:{:?}, name:{:?}, order:{}, w_type:{:?}", id, name, _order, w_type);
        
        let contaner_el = view_holder.query_selector(&format!(".{} .items", w_type)).unwrap();
        let contaner_el = match contaner_el{
            Some(el) =>el,
            None=>{

                let tree = html!{
                    <div class={format!("panels {}", w_type)}>
                        <h1 class="title">{format!("{}s", w_type)}</h1>
                        <div class="items" @items></div>
                    </div>
                }?;
    
                tree.inject_into(&view_holder)?;

                tree.hooks().get("items").unwrap().clone()
            }
        };
        let box_el = contaner_el.query_selector(&format!("[data-id=\"{}\"]", &id)).unwrap();
        if let Some(_box_el) = box_el{
            //box_el
        }else{

            let video_element_id = video_element_id.clone();
            let id_clone = id.clone();
            let container_id_clone = container_id_clone.clone();
            let tree = html!{
                <div class="panel" data-id={id} !click={
                    let stream_id = nw_sys::screen::desktop_capture_monitor::register_stream(&id_clone);
                    let _ = render_media(video_element_id.clone(), stream_id);
                    let _ = stop_capture_monitor(container_id_clone.clone());
                }>
                    <h2 class="title">{name}</h2>
                    <img src="this-image-dont-exists.png"
                    onerror="if(!this.src!=this.dataset.default)this.src=this.dataset.default" 
                    data-default="data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjwvc3ZnPg=="
                    />
                </div>
            }?;

            tree.inject_into(&contaner_el)?;

            app_clone.htmls.lock().unwrap().push(tree);
        }

        Ok(())
    });

    dcm::on("added", cb.into_js());
    app.inner.push_callback(cb)?;

    let mut cb = Callback::<dyn FnMut(u16)->Result<()>>::new();
    cb.set_closure(move |id|->Result<()>{
        log_info!("removed: id:{:?}", id);
        Ok(())
    });

    dcm::on("removed", cb.into_js());
    app.inner.push_callback(cb)?;

    dcm::start(true, true);
    log_info!("dcm::started(): {}", dcm::started());
    container.class_list().add_1("started")?;
    
    Ok(())
}

#[wasm_bindgen]
pub fn stop_capture_monitor(el:String)->Result<()>{
    nw_sys::screen::desktop_capture_monitor::stop();
    document().get_element_by_id(&el).unwrap().class_list().remove_1("started")?;

    Ok(())
}
