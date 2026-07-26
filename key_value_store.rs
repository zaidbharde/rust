//! Persistent key-value store — in-memory HashMap backed by a WAL log file.
//! Supports get, set, delete, and crash recovery.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Clone)]
enum Command {
    Set(String, String),
    Delete(String),
}

impl Command {
    fn serialize(&self) -> String {
        match self {
            Command::Set(k, v)  => format!("SET {} {}\n", k, v),
            Command::Delete(k)  => format!("DEL {}\n", k),
        }
    }

    fn deserialize(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        match parts.as_slice() {
            ["SET", k, v] => Some(Command::Set(k.to_string(), v.to_string())),
            ["DEL", k]    => Some(Command::Delete(k.to_string())),
            _             => None,
        }
    }
}

pub struct KVStore {
    data:     HashMap<String, String>,
    log_path: String,
    log_file: File,
    ops:      usize,
}

impl KVStore {
    pub fn open(path: &str) -> std::io::Result<Self> {
        let mut data = HashMap::new();

        // Replay WAL log on startup (crash recovery)
        if Path::new(path).exists() {
            let file   = File::open(path)?;
            let reader = BufReader::new(file);
            let mut replayed = 0;

            for line in reader.lines().flatten() {
                if let Some(cmd) = Command::deserialize(&line) {
                    match cmd {
                        Command::Set(k, v) => { data.insert(k, v); }
                        Command::Delete(k) => { data.remove(&k); }
                    }
                    replayed += 1;
                }
            }
            if replayed > 0 {
                println!("  Recovered {} operations from WAL.", replayed);
            }
        }

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        Ok(Self {
            data,
            log_path: path.to_string(),
            log_file,
            ops: 0,
        })
    }

    pub fn set(&mut self, key: &str, value: &str) -> std::io::Result<()> {
        let cmd = Command::Set(key.to_string(), value.to_string());
        self.log_file.write_all(cmd.serialize().as_bytes())?;
        self.log_file.flush()?;
        self.data.insert(key.to_string(), value.to_string());
        self.ops += 1;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) -> std::io::Result<bool> {
        if self.data.remove(key).is_some() {
            let cmd = Command::Delete(key.to_string());
            self.log_file.write_all(cmd.serialize().as_bytes())?;
            self.log_file.flush()?;
            self.ops += 1;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn keys(&self) -> Vec<&String> {
        let mut keys: Vec<_> = self.data.keys().collect();
        keys.sort();
        keys
    }

    pub fn len(&self)      -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool  { self.data.is_empty() }

    pub fn compact(&mut self) -> std::io::Result<()> {
        // Rewrite the log with only the current state (removes deleted keys)
        let tmp = format!("{}.tmp", self.log_path);
        let mut f = File::create(&tmp)?;

        for (k, v) in &self.data {
            let cmd = Command::Set(k.clone(), v.clone());
            f.write_all(cmd.serialize().as_bytes())?;
        }
        f.flush()?;
        std::fs::rename(&tmp, &self.log_path)?;

        // Reopen the log file
        self.log_file = OpenOptions::new()
            .append(true)
            .open(&self.log_path)?;

        println!("  Log compacted: {} entries.", self.data.len());
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    println!("{}", "=".repeat(44));
    println!("  🗄️  Persistent Key-Value Store");
    println!("{}", "=".repeat(44));

    let log_path = "kvstore.wal";
    let mut store = KVStore::open(log_path)?;

    // Write some data
    store.set("name",    "Alice")?;
    store.set("age",     "30")?;
    store.set("city",    "London")?;
    store.set("country", "UK")?;
    store.set("lang",    "Rust")?;

    println!("\n  After writes:");
    for key in store.keys() {
        println!("    {} = {}", key, store.get(key).unwrap());
    }

    // Delete
    store.delete("city")?;
    println!("\n  After delete 'city': len={}", store.len());

    // Update
    store.set("age", "31")?;
    println!("  Updated age: {}", store.get("age").unwrap());

    // Compact
    store.compact()?;

    // Re-open (simulates crash recovery)
    println!("\n  Re-opening store...");
    let store2 = KVStore::open(log_path)?;
    println!("  Recovered keys: {:?}", store2.keys());

    // Cleanup
    std::fs::remove_file(log_path)?;
    Ok(())
}
