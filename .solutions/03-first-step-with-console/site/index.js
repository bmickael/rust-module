import { Terminal } from 'xterm';
const terminal = new Terminal();
terminal.open(document.getElementById('terminal'));
terminal.write('Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ');

import { greet, eval_key } from 'first-step-with-console';
greet("WebAssembly with npm");
document.addEventListener('keydown', (event) => {
    // May use event.keyCode instead
    var result = eval_key(event.key);
    terminal.write(result);
});
