use ws::listen;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;

struct Instance {
    // sender: ws::Sender,
    writer: std::sync::mpsc::Sender<String>,
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
        // self.sender.send(msg)
        self.writer.send(msg.to_string()).unwrap();
        Ok(())
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
    // Create a simple streaming channel
    let (writer, reader) = channel::<String>();
    let senders = Arc::new(Mutex::new(Vec::<ws::Sender>::new()));
    let senders_clone = senders.clone();

    thread::spawn(move|| {
        loop {
            let msg = reader.recv().unwrap();
            let msg = &msg;
            senders_clone.lock().unwrap().iter().for_each(|sender| {
                match sender.send(msg.to_string()) {
                    Ok(_) => println!("Broadcasted !"),
                    Err(e) => println!("Fail to send ! err = {:?}", e),
                }
            });
        }
    });
    
    listen("0.0.0.0:8082", |out: ws::Sender| -> Instance {
        println!("Inside first closure");
        senders.clone().lock().unwrap().push(out.clone());
        // Instance { sender: out, writer: writer.clone() }
        Instance { writer: writer.clone() }
    })
    .expect("Cannot Listen");
    println!("End");
}
