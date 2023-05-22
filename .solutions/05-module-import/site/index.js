import("./node_modules/module-import/module_import.js").then((mod) => {
    let ctx = new mod.Context();
    console.log(ctx.toString());
    console.log(ctx.toJSON());
    mod.greet("WebAssembly with npm");
    document.addEventListener('keydown', (event) => {
        console.log(event.key);
        ctx.eval_key(event.key);
    });
});
