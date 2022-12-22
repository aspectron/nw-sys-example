
use nw_sys::utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use workflow_log::log_error;
use workflow_log::{log_trace, log_info};
use workflow_dom::utils::window;
use nw_sys::result::Result;
use nw_sys::prelude::*;
// use workflow_nw::app::Callback;
use workflow_nw::prelude::*;
use workflow_wasm::timers::{set_interval, IntervalHandle};
use workflow_wasm::callback::{Callback, CallbackClosure, AsCallback};
use web_sys::HtmlVideoElement;
use workflow_html::{html, Html, Render};
use nw_sys::chrome::notifications;

static mut APP:Option<Arc<ExampleApp>> = None;


#[derive(Clone)]
pub struct ExampleApp{
    pub inner: Arc<Application>,
    pub htmls: Arc<Mutex<Vec<Html>>>,
    pub interval_handle: Arc<Mutex<Option<IntervalHandle>>>,
}


impl ExampleApp{
    fn new()->Result<Arc<Self>>{

        if let Some(app) = app(){
            return Ok(app);
        } 

        let app = Arc::new(Self{
            inner: Application::new()?,
            htmls: Arc::new(Mutex::new(Vec::new())),
            interval_handle: Arc::new(Mutex::new(None)),
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
                let mut close_callback = Callback::<dyn FnMut()->Result<()>>::default();
                let close_callback_clone = close_callback.clone();
                close_callback.set_closure(move || ->Result<()>{
                    log_trace!("win.closed: {:?}", win_clone);
                    win_clone.close_with_force();
                    let _a = close_callback_clone.clone();
                    //remove this listener from app
                    Ok(())
                });

                let win_clone2 = win.clone();
                let maximize_callback = Callback::<dyn FnMut()>::new(move ||{
                // let maximize_callback = Callback::new(move ||{
                    log_trace!("win.maximize: {:?}", win_clone2);
                });

                // win.on("close", close_callback.into_js());
                // win.on("maximize", maximize_callback.into_js());
                win.on("close", close_callback.as_ref());
                win.on("maximize", maximize_callback.as_ref());

                inner.callbacks.insert(close_callback)?;
                inner.callbacks.insert(maximize_callback)?;

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
                nw_sys::app::close_all_windows();
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

    // @surinder - any way to simplify this call signature? is it possible to determine JsValue for CallbackClosure from the return value?
    let callback = Callback::<CallbackClosure<JsValue>>::new(move |value:JsValue|->std::result::Result<(), JsValue>{
    // let callback = Callback::new(move |value:JsValue|->std::result::Result<(), JsValue>{
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

    app.inner.callbacks.insert(callback)?;

    Ok(())
}


#[wasm_bindgen]
pub fn attach_notification_listeners()->Result<()>{
    let (app, _) = initialize_app()?;
    let app = &app.inner;

    //create event listeners
    let clicked_cb = Callback::<dyn FnMut(String)>::new(|id|{
        log_info!("Notification clicked: {id}");
    });
    notifications::on_clicked(clicked_cb.into_js());

    let button_click_cb = Callback::<dyn FnMut(String, u16)>::new(|id, btn_index|{
        log_info!("Notification button clicked: {id}, {btn_index}");
    });
    notifications::on_button_clicked(button_click_cb.into_js());

    let closed_cb = Callback::<dyn FnMut(String, bool)>::new(|id, by_user|{
        log_info!("Notification closed: {id}, {by_user}");
    });
    notifications::on_closed(closed_cb.into_js());

    app.callbacks.insert(clicked_cb)?;
    app.callbacks.insert(button_click_cb)?;
    app.callbacks.insert(closed_cb)?;

    Ok(())
}

#[wasm_bindgen]
pub fn basic_notification()->Result<()>{
    let (app, _) = initialize_app()?;

    // Create basic notification
    let options = notifications::Options::new()
        .title("Title text")
        .icon_url("/resources/icons/tray-icon@2x.png")
        .set_type(nw_sys::chrome::notifications::TemplateType::Basic)
        .message("Message Text")
        .context_message("Context Message");

    let cb = Callback::<dyn FnMut(String)>::new(|v|{
        log_info!("notification create callback, id: {:?}", v)
    });
    notifications::create(None, &options, Some(cb.into_js()));

    app.inner.callbacks.insert(cb)?;

    Ok(())
}

#[wasm_bindgen]
pub fn notification_with_buttons(){
    // Create notification with buttons
    let button1 = notifications::Button::new()
        .title("Button A")
        .icon_url("/resources/icons/tray-icon@2x.png");

    let button2 = notifications::Button::new()
        .title("Button B")
        .icon_url("/resources/icons/tray-icon@2x.png");

    let options = notifications::Options::new()
        .title("Title text")
        .icon_url("/resources/icons/tray-icon@2x.png")
        .set_type(nw_sys::chrome::notifications::TemplateType::Basic)
        .message("Message Text")
        .buttons(vec![button1, button2]);

    notifications::create(None, &options, None);
}

#[wasm_bindgen]
pub fn notification_with_image(){
    // Create image notification
    let options = notifications::Options::new()
        .title("Title text")
        .icon_url("/resources/icons/tray-icon@2x.png")
        .set_type(nw_sys::chrome::notifications::TemplateType::Image)
        .message("Message Text")
        .image_url("/resources/setup/document.png");

    notifications::create(None, &options, None);
}

#[wasm_bindgen]
pub fn notification_with_items(){
    // Create notification with items

    let item1 = notifications::Item::new()
        .title("Item A")
        .message("Mesage A");
    let item2 = notifications::Item::new()
        .title("Item B")
        .message("Mesage B");

    let options = notifications::Options::new()
        .title("Title text")
        .icon_url("/resources/icons/tray-icon@2x.png")
        .set_type(nw_sys::chrome::notifications::TemplateType::List)
        .message("Message Text")
        .items(vec![item1, item2]);

    notifications::create(None, &options, None);
}

#[wasm_bindgen]
pub fn notification_with_progress()->Result<()>{
    let (app, _) = initialize_app()?;

    // Create notification with progress
    let mut progress = 50;
    let options = notifications::Options::new()
        .title("Title text")
        .icon_url("/resources/icons/tray-icon@2x.png")
        .set_type(nw_sys::chrome::notifications::TemplateType::Progress)
        .message("Mesage text")
        .progress(progress);

    static mut ID:u16 = 0;
    let noti_id = format!("{:?}", unsafe{ID+=1; ID});
    log_info!("noti_id: {noti_id}");

    notifications::create(Some(noti_id.clone()), &options, None);

    let app_clone = app.clone();
    let mut cb = Callback::<dyn FnMut()>::default();
    let cb_id = cb.get_id();
    cb.set_closure(move ||{
        progress += 10;
        log_info!("progress: {progress}");
        let options = options.clone().progress(progress);
        notifications::update(&noti_id, &options, None);
        if progress == 100{
            let _ = app_clone.inner.callbacks.remove(&cb_id);
            *app_clone.interval_handle.lock().unwrap() = None;
        }
    });

    let h = set_interval(cb.closure().unwrap().as_ref(), 1000).unwrap();
    *app.interval_handle.lock().unwrap() = Some(h);
    app.inner.callbacks.insert(cb)?;

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
    let mut cb = Callback::<dyn FnMut(String, String)->Result<()>>::default();
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
    app.inner.callbacks.insert(cb)?;

    let app_clone = app.clone();
    
    let mut cb = Callback::<dyn FnMut(String, String, u16, String)->Result<()>>::default();
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
    app.inner.callbacks.insert(cb)?;

    let mut cb = Callback::<dyn FnMut(u16)->Result<()>>::default();
    cb.set_closure(move |id|->Result<()>{
        log_info!("removed: id:{:?}", id);
        Ok(())
    });

    dcm::on("removed", cb.into_js());
    app.inner.callbacks.insert(cb)?;

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
