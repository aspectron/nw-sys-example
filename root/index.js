

//nw.Window.open('home.html', {}, function(win) {
    //nw.Window.get().showDevTools();
//});

(async()=>{
    let $nwjs = await import('../nwjs/nwjs.js');
    window.$nwjs = $nwjs;
    const wasm = await $nwjs.default('/nwjs/nwjs_bg.wasm');
    //console.log("wasm", wasm, workflow)
    //$nwjs.init_console_panic_hook();
    //$nwjs.show_panic_hook_logs();
    $nwjs.initialize();
})();
