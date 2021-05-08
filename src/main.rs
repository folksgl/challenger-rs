use std::sync::mpsc;
use std::thread;

mod uci;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let producer_handle = thread::spawn(move || uci::producer(sender));
    let consumer_handle = thread::spawn(move || uci::consumer(receiver));

    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}
