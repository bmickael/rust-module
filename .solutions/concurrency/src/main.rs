use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    #[derive(Debug)]
    struct Shared {
        i: usize,
        j: usize,
    }
    let shared = Arc::new(Mutex::new(Shared { i: 42, j: 84 }));
    let clone = shared.clone();
    let _t = std::thread::spawn(move || {
        clone.lock().unwrap().i += 42;
        dbg!(clone);
    });
    shared.lock().unwrap().i += 42;
    dbg!(shared);

    std::thread::sleep(Duration::from_secs(3)); // Wait a bit before exit
}
