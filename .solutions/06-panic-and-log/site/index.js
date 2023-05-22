import("./node_modules/panic-and-log/panic_and_log.js").then((mod) => {
    let ctx = new mod.Context();
    console.log(ctx);
    mod.greet("WebAssembly with npm");
    document.addEventListener('keydown', (event) => {
        console.log(event.key);
        ctx.eval_key(event.key);
    });
});
