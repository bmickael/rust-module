use wasm_bindgen::prelude::*;

// Import the `window.alert` function from the Web.
// #[wasm_bindgen(js_namespace = window)]
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
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (unsafe {log(&format_args!($($t)*).to_string())})
}

macro_rules! console_dbg {
    ($t:tt) => (console_log!("{:?}", $t))
}


#[wasm_bindgen]
pub fn eval_key(key_value: &str) -> String {
    key_value.to_string()
}

#[wasm_bindgen]
pub fn eval_keycode(key_code: usize) -> String {
    console_log!("keyCode: {}", key_code);
    String::new()
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
    // alert(&format!("Hello, {}!", name));
    console_log!("Hello, {}!", name);

    console_log!("Les {} Carottes {} cuites", 3, "meuh");
    
    #[derive(Debug)]
    struct S {
        i: usize,
        j: usize,
    }
    let mut s = S { i: 3, j: 65 };

    console_dbg!(s);
    s.i += 1;
    console_dbg!(s);

    console_log!("test");
    console_log!("Hello, {}!", name);
    console_log!("Les {} sont {}", "carottes", "cuites");
}
