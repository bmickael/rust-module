use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    unsafe fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (unsafe {log(&format_args!($($t)*).to_string())})
}

#[allow(unused_macros)]
macro_rules! console_dbg {
    ($t:tt) => (console_log!("{:?}", $t))
}


#[wasm_bindgen]
pub struct Context {
    content: String,
    terminal: Terminal,
}


#[wasm_bindgen]
impl Context {
    // #[wasm_bindgen(constructor)]
    // pub fn new() -> Self {
    //     Self { content: String::new(), terminal: Terminal::new() }
    // }
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let terminal = Terminal::new();
        terminal.open("Toto");
        Self { content: String::new(), terminal }
    }

    
    #[wasm_bindgen]
    pub fn eval_key(&mut self, chain: &str) -> String {
        match chain {
            "Enter" => {
                let mut output = String::new();
                output += "\n\r";
                output += match self.content.as_str() {
                    "fizz" => "buzz",
                    "buzz" => "fizz",
                    _default => _default,
                };
                output += "\n\r$ ";
                self.content.clear();
                output
            },
            _default => {
                self.content.push_str(chain);
                chain.to_string()
            }
        }
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    console_log!("Hello, {}!", name);
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
     type Terminal;

    #[wasm_bindgen(constructor)]
    fn new() -> Terminal;

    #[wasm_bindgen(method)]
    fn open(this: &Terminal, element: &str); // &str est faux !!! regardez plutot HTMLElement
}
