use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let producer = thread::spawn(move || {
        for i in 0..5 {
            tx.send(i * i).unwrap();
        }
    });

    let consumer = thread::spawn(move || {
        for received in rx {
            println!("Got squared value: {}", received);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
