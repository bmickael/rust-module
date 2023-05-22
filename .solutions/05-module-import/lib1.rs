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
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Use `web_sys`'s global `window` function to get a handle on the global
        // window object.
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let terminal_div = document.create_element("div").unwrap();
        body.append_child(&terminal_div).unwrap();
        // let terminal_div = document.get_element_by_id("terminal").unwrap(); // Alternative if already exists into HTML
        
        let terminal = Terminal::new();
        terminal.open(&terminal_div);

        terminal.write("Hello from \x1B[1;3;31mxterm.js\x1B[0m $ "); 
        Self { content: String::new(), terminal }
    }
    
    #[wasm_bindgen]
    pub fn eval_key(&mut self, chain: &str) {
        let to_write = match chain {
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
        };
        self.terminal.write(&to_write);
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
    fn open(this: &Terminal, element: &web_sys::Element);
 
    #[wasm_bindgen(method)]
    fn write(this: &Terminal, chain: &str);   
}
