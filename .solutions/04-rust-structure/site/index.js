import { Terminal } from 'xterm';
const terminal = new Terminal();
terminal.open(document.getElementById('terminal'));
terminal.write('Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ');

import("./node_modules/rust-structure/rust_structure.js").then((mod) => {
    let ctx = new mod.Context();
    console.log(ctx);
    console.log(ctx.dump());
    mod.greet("WebAssembly with npm");
    document.addEventListener('keydown', (event) => {
        console.log(ctx);
        console.log(ctx.value);
        ctx.value += 1;
        console.log(ctx.content);
        ctx.content = "Toto";
        console.log(event.key);
        var result = ctx.eval_key(event.key);
        terminal.write(result);
    });
});
