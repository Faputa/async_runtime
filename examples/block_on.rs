use std::{thread, time::Duration};

use async_runtime::block_on;
use futures::channel::oneshot;

fn main() {
    let (s, r) = oneshot::channel();

    // Spawn a thread that will send a message through the channel.
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        s.send("Hello, world!").unwrap();
    });

    // Block until the message is received.
    let msg = block_on(async {
        println!("Awaiting...");
        r.await.unwrap()
    });

    println!("{}", msg);
}
