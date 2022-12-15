//nw.Window.open('home.html', {}, function(win) {
    //nw.Window.get().showDevTools();
//});


(async()=>{
    window.$nwjs = await import('/root/wasm/nwjs.js');
    // window.$nwjs = $nwjs;
    const wasm = await window.$nwjs.default('/root/wasm/nwjs_bg.wasm');
    //console.log("wasm", wasm, workflow)
    //$nwjs.init_console_panic_hook();
    //$nwjs.show_panic_hook_logs();
    window.$nwjs.initialize();
})();

