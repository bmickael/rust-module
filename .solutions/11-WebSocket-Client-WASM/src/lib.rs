use wasm_bindgen::prelude::*;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Connexion to the websocket server
    let socket = WebSocket::new("ws://127.0.0.1:8082")?;
    let socket_clone = socket.clone(); // clone for message cb

    // on_open
    let cb_on_open = Closure::<dyn FnMut()>::new(move || {
        log::info!("The Socket has been opened !");

        match socket_clone.send_with_str("Les carottes sont cuites!") {
            Ok(_) => log::info!("Message sended!"),
            Err(err) => log::error!("An error has occured while sending message! error = {:?}", err),
        }
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
    let cb_on_message = Closure::<dyn FnMut(_)>::new(move |msg: MessageEvent| {
        match msg.data().dyn_into::<js_sys::JsString>()  {
            Ok(message) => log::info!("New message disponible: {}", message),
            Err(err) => log::error!("Not a good message! error = {:?}", err),
        }
    });
    socket.set_onmessage(Some(cb_on_message.as_ref().unchecked_ref()));
    cb_on_message.forget();

    // return of start function, may catch errors into index.js
    Ok(())
}
