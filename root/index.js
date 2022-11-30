

//nw.Window.open('home.html', {}, function(win) {
    //nw.Window.get().showDevTools();
//});

(async()=>{
    let nwjsExample = await import('../nwjs/nwjs.js');
    window.nwjsExample = nwjsExample;
    const wasm = await nwjsExample.default('/nwjs/nwjs_bg.wasm');
    //console.log("wasm", wasm, workflow)
    //nwjsExample.init_console_panic_hook();
    //nwjsExample.show_panic_hook_logs();
    nwjsExample.initialize();
})();
