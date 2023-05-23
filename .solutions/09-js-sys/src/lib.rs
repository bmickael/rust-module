use wasm_bindgen::prelude::*;
use xterm_js_rs::xterm::Terminal;
use xterm_js_rs::xterm::TerminalOptions;

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
    ($t:tt) => {
        console_log!("{:?}", $t)
    };
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let terminal_div = document.create_element("div").unwrap();
    body.append_child(&terminal_div).unwrap();
    let terminal_div = document.get_element_by_id("terminal").unwrap(); // Alternative if already exists into HTML

    //let terminal = Terminal::new();
    let terminal = Terminal::new(&TerminalOptions::new());
    //terminal.open(&terminal_div.dyn_into::<web_sys::HtmlElement>().unwrap());
    terminal.open(terminal_div.dyn_into::<web_sys::HtmlElement>().unwrap());

    log::info!("Some Info");
    log::warn!("Some Warning");
    log::error!("Some Error");

    terminal.write("Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ");

    let mut content = String::new();

    fn get_string_to_write(content: &mut String, input: &str) -> String {
        match input {
            "Enter" => {
                let mut output = String::new();
                output += "\n\r";
                output += match content.as_str() {
                    "fizz" => "buzz",
                    "buzz" => "fizz",
                    _default => _default,
                };
                output += "\n\r$ ";
                content.clear();
                output
            }
            _default => {
                content.push_str(_default);
                _default.to_string()
            }
        }
    }
    // let cb = Closure::wrap(Box::new(move |event: web_sys::Event| {
    //     let keyboard_event = event.clone().dyn_into::<web_sys::KeyboardEvent>().unwrap();
    //     terminal.write(&get_string_to_write(
    //         &mut content,
    //         keyboard_event.key().as_str(),
    //     ));
    // }) as Box<dyn FnMut(_)>);
    // document
    //     .add_event_listener_with_callback("keydown", &cb.as_ref().unchecked_ref())
    //     .unwrap();

    let ptr: *const Terminal = &terminal as *const _;
    let cb = Closure::new(move |event: web_sys::KeyboardEvent| {
        let keyboard_event = event;
        match keyboard_event.type_().as_str() {
            "keydown" => terminal.write(&get_string_to_write(
                &mut content,
                keyboard_event.key().as_str(),
            )),
            _ => {}
        }
    });
    unsafe {
        // (*ptr).attach_custom_key_event_handler(&cb.into_js_value().dyn_ref::<js_sys::Function>().unwrap());
        (*ptr).attach_custom_key_event_handler(cb.as_ref().unchecked_ref());
    }

    // let terminal = std::sync::Arc::new(terminal);
    // let clone = terminal.clone();
    // let cb = Closure::wrap(Box::new(move |event: web_sys::Event| {
    //     let keyboard_event = event.clone().dyn_into::<web_sys::KeyboardEvent>().unwrap();
    //     console_log!("{}", keyboard_event.type_());
    //     match keyboard_event.type_().as_str() {
    //         "keydown" => clone.write(&get_string_to_write(
    //             &mut content,
    //             keyboard_event.key().as_str(),
    //         )),
    //         _ => {}
    //     }
    // }) as Box<dyn FnMut(_)>);
    // terminal.attachCustomKeyEventHandler(&cb);

    cb.forget();
    Ok(())
}

#[wasm_bindgen]
pub fn process_array(array: &Array) -> u32 {
    let length = array.length();
    let mut sum = 0;
    for i in 0..length {
        let value = array.get(i).as_f64().unwrap_or(0.0);
        sum += value as u32;
    }
    sum
}

#[wasm_bindgen]
pub fn process_array_2(array: &[f32]) -> u32 {
    let length = array.len();
    let mut sum = 0;
    for i in 0..length {
        let value = array.get(i).map(|v| *v as f64).unwrap_or(0.0);
        sum += value as u32;
    }
    sum
}

use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "GetJsArray")]
pub fn get_js_array() -> Array {
    let array = Array::new();
    array.push(&JsValue::from(1));
    array.push(&JsValue::from(2));
    array.push(&JsValue::from(3));

    let mut value = std::f64::consts::FRAC_1_PI;
    let length = array.length();
    for i in 0..length {
        value += array.get(i).as_f64().unwrap();
        array.set(i, JsValue::from(value));
    }
    array
}

use js_sys::{Object, Reflect};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = "GetJsObject")]
pub fn get_js_object() -> Object {
    let object = Object::new();
    Reflect::set(&object, &"name".into(), &"John".into()).unwrap();
    Reflect::set(&object, &"age".into(), &30.into()).unwrap();

    let _name = Reflect::get(&object, &"name".into())
        .unwrap()
        .as_string()
        .unwrap();
    let _age = Reflect::get(&object, &"age".into())
        .unwrap()
        .as_f64()
        .unwrap();
    object
}

use js_sys::eval;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn call_js_function() {
    let message = "Les carottes sont cuites!";
    let code = format!("alert('{} --- {} ---{}')", "***", message, "***");
    eval(&code).unwrap();
}

use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn call_js2_function() {
    let greet_fn = Function::new_no_args("alert");
    greet_fn.call1(&JsValue::null(), &JsValue::from("Hello from WebAssembly!"));
}

// #[wasm_bindgen(module = "xterm")]
// extern "C" {
//     type Terminal;

//     #[wasm_bindgen(constructor)]
//     fn new() -> Terminal;

//     #[wasm_bindgen(method)]
//     fn open(this: &Terminal, element: &web_sys::HtmlElement);

//     #[wasm_bindgen(method)]
//     fn write(this: &Terminal, chain: &str);

//     #[wasm_bindgen(method, js_name = "attachCustomKeyEventHandler")]
//     fn attach_custom_key_event_handler(this: &Terminal, closure: &Closure<dyn FnMut(web_sys::KeyboardEvent)>);
// }
