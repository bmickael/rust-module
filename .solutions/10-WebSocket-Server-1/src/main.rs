use ws::listen;

struct Instance {
    sender: ws::Sender,
}

use ws::{Message, Result};
impl ws::Handler for Instance {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        println!("Socket Opened ! handshake = {:?}", shake);
        Ok(())
    }
    fn on_shutdown(&mut self) {}
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("message: {}", msg);
        self.sender.send(msg)
    }
    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        println!("Socket Closed ! code = {:?} reason = {}", code, reason);
    }
    fn on_error(&mut self, err: ws::Error) {
        println!("Socket ERROR ! err = {}", err);
    }
}

fn main() {
    println!("Begin");
    listen("0.0.0.0:8082", |out: ws::Sender| -> Instance {
        println!("Inside first closure");
        Instance { sender: out }
    })
    .expect("Cannot Listen");
    println!("End");
}
