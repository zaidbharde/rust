use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let forks: Vec<Arc<Mutex<()>>> = (0..5).map(|_| Arc::new(Mutex::new(()))).collect();
    let mut handles = vec![];

    for i in 0..5 {
        let left = Arc::clone(&forks[i]);
        let right = Arc::clone(&forks[(i + 1) % 5]);

        let handle = thread::spawn(move || {
            for _ in 0..2 {
                let (first, second) = if i % 2 == 0 { (&left, &right) } else { (&right, &left) };
                let _f1 = first.lock().unwrap();
                let _f2 = second.lock().unwrap();
                println!("Philosopher {} is eating", i);
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("All philosophers finished eating");
}
