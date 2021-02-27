use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let producer_handle = thread::spawn(move || challenger_rs::producer(sender));
    let consumer_handle = thread::spawn(move || challenger_rs::consumer(receiver));

    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}
