//! File I/O — read, write, append, JSON-like serialization, and directory walk.

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

// ── Write file ────────────────────────────────────────────────────────
fn write_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// ── Read entire file ──────────────────────────────────────────────────
fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

// ── Read line by line ─────────────────────────────────────────────────
fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    BufReader::new(file).lines().collect()
}

// ── Append to file ────────────────────────────────────────────────────
fn append_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

// ── Simple CSV writer ─────────────────────────────────────────────────
fn write_csv(path: &str, headers: &[&str], rows: &[Vec<String>]) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "{}", headers.join(","))?;
    for row in rows {
        writeln!(file, "{}", row.join(","))?;
    }
    Ok(())
}

// ── Simple CSV reader ─────────────────────────────────────────────────
fn read_csv(path: &str) -> io::Result<(Vec<String>, Vec<Vec<String>>)> {
    let lines = read_lines(path)?;
    let mut iter = lines.into_iter();

    let headers = iter
        .next()
        .unwrap_or_default()
        .split(',')
        .map(String::from)
        .collect();

    let rows = iter
        .map(|line| line.split(',').map(String::from).collect())
        .collect();

    Ok((headers, rows))
}

// ── File metadata ─────────────────────────────────────────────────────
fn print_metadata(path: &str) -> io::Result<()> {
    let meta = fs::metadata(path)?;
    println!("  Size     : {} bytes", meta.len());
    println!("  Is file  : {}", meta.is_file());
    println!("  Is dir   : {}", meta.is_dir());
    println!("  Readonly : {}", meta.permissions().readonly());
    Ok(())
}

fn main() -> io::Result<()> {
    println!("=== File I/O ===");

    // ── Write & read ──────────────────────────────────────────────────
    println!("\n--- Write & Read ---");
    let content = "Hello, Rust file I/O!\nLine 2\nLine 3\nLine 4";
    write_file("demo.txt", content)?;
    println!("  Written to demo.txt");

    let read_back = read_file("demo.txt")?;
    println!("  Read back:\n{}", read_back.lines()
        .map(|l| format!("    {}", l))
        .collect::<Vec<_>>()
        .join("\n"));

    // ── Line by line ──────────────────────────────────────────────────
    println!("\n--- Line by line ---");
    let lines = read_lines("demo.txt")?;
    for (i, line) in lines.iter().enumerate() {
        println!("  L{}: {}", i + 1, line);
    }

    // ── Append ────────────────────────────────────────────────────────
    println!("\n--- Append ---");
    append_file("demo.txt", "Appended line!")?;
    println!("  Appended to demo.txt");
    let all_lines = read_lines("demo.txt")?;
    println!("  Total lines: {}", all_lines.len());

    // ── Metadata ──────────────────────────────────────────────────────
    println!("\n--- Metadata ---");
    print_metadata("demo.txt")?;

    // ── CSV ───────────────────────────────────────────────────────────
    println!("\n--- CSV ---");
    let headers = &["name", "age", "score"];
    let rows = vec![
        vec!["Alice".into(),   "30".into(), "95".into()],
        vec!["Bob".into(),     "25".into(), "82".into()],
        vec!["Charlie".into(), "35".into(), "78".into()],
    ];
    write_csv("data.csv", headers, &rows)?;
    println!("  Written data.csv");

    let (hdrs, data) = read_csv("data.csv")?;
    println!("  Headers : {:?}", hdrs);
    for row in &data {
        println!("  Row     : {:?}", row);
    }

    // ── Cleanup ───────────────────────────────────────────────────────
    fs::remove_file("demo.txt")?;
    fs::remove_file("data.csv")?;
    println!("\n  Cleaned up temp files.");

    Ok(())
}
