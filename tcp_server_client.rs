//! TCP Server & Client — echo server with multi-threading.
//! Run with: cargo run -- server  (in one terminal)
//!           cargo run -- client  (in another)

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::env;
use std::time::Duration;

const ADDR: &str = "127.0.0.1:7878";

// ── Server ────────────────────────────────────────────────────────────
fn run_server() {
    let listener = TcpListener::bind(ADDR).expect("Failed to bind");
    println!("🦀 TCP Server listening on {}", ADDR);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_client(stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    println!("  [+] Connected: {}", peer);

    let mut writer = stream.try_clone().unwrap();
    let reader     = BufReader::new(&stream);

    writeln!(writer, "Welcome to the Rust echo server! Type 'quit' to exit.").unwrap();

    for line in reader.lines() {
        match line {
            Ok(msg) => {
                println!("  [{}] → {}", peer, msg);
                if msg.trim() == "quit" {
                    writeln!(writer, "Goodbye!").unwrap();
                    break;
                }
                writeln!(writer, "Echo: {}", msg).unwrap();
            }
            Err(_) => break,
        }
    }
    println!("  [-] Disconnected: {}", peer);
}

// ── Client ────────────────────────────────────────────────────────────
fn run_client() {
    thread::sleep(Duration::from_millis(100));   // give server time to start

    let stream = TcpStream::connect(ADDR).expect("Could not connect");
    println!("🦀 Connected to {}", ADDR);

    let mut writer = stream.try_clone().unwrap();
    let reader     = BufReader::new(&stream);
    let mut lines  = reader.lines();

    // Print welcome message
    if let Some(Ok(welcome)) = lines.next() {
        println!("  Server: {}", welcome);
    }

    let messages = ["Hello, server!", "Rust is great!", "quit"];
    for msg in &messages {
        println!("  Client → {}", msg);
        writeln!(writer, "{}", msg).unwrap();

        if let Some(Ok(response)) = lines.next() {
            println!("  Server → {}", response);
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("both");

    match mode {
        "server" => run_server(),
        "client" => run_client(),
        _ => {
            // Demo: run both in the same process
            let server = thread::spawn(run_server);
            thread::sleep(Duration::from_millis(50));
            run_client();
            drop(server);   // server runs forever, just drop for demo
        }
    }
}
