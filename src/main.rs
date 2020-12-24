use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let producer_handle = thread::spawn(move || challenger_rs::producer(tx));
    let consumer_handle = thread::spawn(move || challenger_rs::consumer(rx));

    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}
