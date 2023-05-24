use wasm_bindgen::prelude::*;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let terminal_div = document.create_element("div").unwrap();
    body.append_child(&terminal_div).unwrap();

    let terminal = Terminal::new();
    terminal.open(&terminal_div.dyn_into::<web_sys::HtmlElement>().unwrap());
    terminal.write("Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ");

    let terminal = std::rc::Rc::new(terminal);
    let clone = terminal.clone();
    let cb_clone = clone.clone();

    // Connexion to the websocket server
    let socket = WebSocket::new("ws://127.0.0.1:8082")?;
    let socket_clone = socket.clone(); // clone for message cb

    // on_open
    let cb_on_open = Closure::<dyn FnMut()>::new(move || {
        log::info!("The Socket has been opened !");
    });
    socket.set_onopen(Some(cb_on_open.as_ref().unchecked_ref()));
    cb_on_open.forget();

    // on_error
    let cb_on_error = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
        log::error!("An error has occured! error = {:?}", e);
    });
    socket.set_onerror(Some(cb_on_error.as_ref().unchecked_ref()));
    cb_on_error.forget();

    // on_message
    let cb_on_message = Closure::<dyn FnMut(_)>::new(move |msg: MessageEvent| match msg
        .data()
        .dyn_into::<js_sys::JsString>(
    ) {
        Ok(message) => {
            log::info!("New message disponible: {}", message);
            let s: String = message.into();
            cb_clone.write("\n\r");
            cb_clone.write(&s);
            cb_clone.write("\n\r");
        }
        Err(err) => log::error!("Not a good message! error = {:?}", err),
    });
    socket.set_onmessage(Some(cb_on_message.as_ref().unchecked_ref()));
    cb_on_message.forget();

    let mut content = String::new();
    let cb = Closure::wrap(Box::new(move |keyboard_event: web_sys::KeyboardEvent| {
        match keyboard_event.type_().as_str() {
            "keydown" => match keyboard_event.key().as_str() {
                "Enter" => {
                    match socket_clone.send_with_str(&content) {
                        Ok(_) => log::info!("Message sended!"),
                        Err(err) => log::error!(
                            "An error has occured while sending message! error = {:?}",
                            err
                        ),
                    }
                    content.clear();
                }
                _default => {
                    content.push_str(_default);
                    clone.write(_default);
                }
            },
            _ => {}
        }
    }) as Box<dyn FnMut(_)>);
    terminal.attachCustomKeyEventHandler(&cb);
    cb.forget(); // Remember to forget

    // return of start function, may catch errors into index.js
    Ok(())
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    type Terminal;

    #[wasm_bindgen(constructor)]
    fn new() -> Terminal;

    #[wasm_bindgen(method)]
    fn open(this: &Terminal, element: &web_sys::HtmlElement);

    #[wasm_bindgen(method)]
    fn write(this: &Terminal, chain: &str);

    #[wasm_bindgen(method)]
    fn attachCustomKeyEventHandler(
        this: &Terminal,
        closure: &Closure<dyn FnMut(web_sys::KeyboardEvent)>,
    );
}
