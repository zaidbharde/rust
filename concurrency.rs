//! Concurrency — threads, channels, Arc, Mutex, and parallel computation.

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

// ── Parallel sum ──────────────────────────────────────────────────────
fn parallel_sum(data: Vec<i64>, num_threads: usize) -> i64 {
    let chunk_size = (data.len() + num_threads - 1) / num_threads;
    let data = Arc::new(data);
    let mut handles = vec![];

    for i in 0..num_threads {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let start = i * chunk_size;
            let end   = (start + chunk_size).min(data.len());
            if start >= data.len() { return 0i64; }
            data[start..end].iter().sum::<i64>()
        });
        handles.push(handle);
    }

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

// ── Shared counter ────────────────────────────────────────────────────
fn shared_counter(num_threads: usize, increments: usize) -> usize {
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = vec![];

    for _ in 0..num_threads {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..increments {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for h in handles { h.join().unwrap(); }
    *counter.lock().unwrap()
}

// ── Message passing via channel ───────────────────────────────────────
fn channel_demo() {
    let (tx, rx) = mpsc::channel::<String>();

    let tx1 = tx.clone();
    let tx2 = tx.clone();

    // Producer 1
    thread::spawn(move || {
        for i in 0..3 {
            tx1.send(format!("Worker-1 message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Producer 2
    thread::spawn(move || {
        for i in 0..3 {
            tx2.send(format!("Worker-2 message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(15));
        }
    });

    drop(tx);   // drop original sender so receiver knows when to stop

    println!("  Messages received:");
    for msg in rx {
        println!("    → {}", msg);
    }
}

fn main() {
    println!("=== Concurrency ===");

    // ── Parallel sum ──────────────────────────────────────────────────
    println!("\n--- Parallel Sum ---");
    let data: Vec<i64> = (1..=1_000_000).collect();
    let expected: i64  = data.iter().sum();

    let start  = Instant::now();
    let result = parallel_sum(data, 4);
    let elapsed = start.elapsed();

    println!("  Expected  : {}", expected);
    println!("  Parallel  : {}", result);
    println!("  Match     : {}", expected == result);
    println!("  Time      : {:?}", elapsed);

    // ── Shared counter ────────────────────────────────────────────────
    println!("\n--- Shared Mutex Counter ---");
    let count = shared_counter(8, 1000);
    println!("  8 threads × 1000 increments = {}", count);

    // ── Channels ──────────────────────────────────────────────────────
    println!("\n--- Channel (multi-producer) ---");
    channel_demo();

    // ── Simple thread spawn ───────────────────────────────────────────
    println!("\n--- Thread spawn ---");
    let handles: Vec<_> = (0..5).map(|i| {
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 10));
            format!("Thread {} done", i)
        })
    }).collect();

    for h in handles {
        println!("  {}", h.join().unwrap());
    }
}
