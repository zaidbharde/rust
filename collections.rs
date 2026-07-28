//! Collections — Vec, HashMap, HashSet, BTreeMap, VecDeque.

use std::collections::{HashMap, HashSet, BTreeMap, VecDeque};

fn main() {
    println!("=== Collections ===");

    // ── Vec ───────────────────────────────────────────────────────────
    println!("\n--- Vec ---");
    let mut v: Vec<i32> = Vec::new();
    v.extend([5, 3, 1, 4, 2, 3, 1]);
    println!("  Raw     : {:?}", v);

    v.sort();
    v.dedup();    // remove consecutive duplicates (after sort = all dupes)
    println!("  Sorted+dedup: {:?}", v);

    let evens: Vec<_> = v.iter().filter(|&&x| x % 2 == 0).collect();
    println!("  Evens   : {:?}", evens);

    let doubled: Vec<_> = v.iter().map(|&x| x * 2).collect();
    println!("  Doubled : {:?}", doubled);

    let sum: i32 = v.iter().sum();
    println!("  Sum     : {}", sum);

    // ── HashMap ───────────────────────────────────────────────────────
    println!("\n--- HashMap ---");
    let mut scores: HashMap<&str, u32> = HashMap::new();
    scores.insert("Alice",   95);
    scores.insert("Bob",     82);
    scores.insert("Charlie", 78);
    scores.insert("Diana",   91);

    // entry API — insert only if missing
    scores.entry("Eve").or_insert(88);
    scores.entry("Alice").or_insert(0);   // Alice already exists, unchanged

    println!("  Scores : {:?}", scores);
    println!("  Alice  : {}", scores["Alice"]);
    println!("  Unknown: {:?}", scores.get("Frank"));

    // Update value
    if let Some(score) = scores.get_mut("Bob") {
        *score += 5;
    }
    println!("  Bob after +5: {}", scores["Bob"]);

    // Word frequency counter
    let text = "the quick brown fox jumps over the lazy dog the fox";
    let mut freq: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    let mut freq_vec: Vec<_> = freq.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));
    println!("\n  Word frequencies:");
    for (word, count) in &freq_vec {
        println!("    {:10} : {}", word, count);
    }

    // ── HashSet ───────────────────────────────────────────────────────
    println!("\n--- HashSet ---");
    let set_a: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set_b: HashSet<i32> = [3, 4, 5, 6, 7].iter().cloned().collect();

    let mut union: Vec<_>        = set_a.union(&set_b).collect();
    let mut inter: Vec<_>        = set_a.intersection(&set_b).collect();
    let mut diff:  Vec<_>        = set_a.difference(&set_b).collect();

    union.sort(); inter.sort(); diff.sort();
    println!("  A         : {:?}", { let mut v: Vec<_> = set_a.iter().collect(); v.sort(); v });
    println!("  B         : {:?}", { let mut v: Vec<_> = set_b.iter().collect(); v.sort(); v });
    println!("  Union     : {:?}", union);
    println!("  Intersect : {:?}", inter);
    println!("  Diff A-B  : {:?}", diff);

    // ── BTreeMap (sorted) ─────────────────────────────────────────────
    println!("\n--- BTreeMap (sorted) ---");
    let mut btree: BTreeMap<&str, i32> = BTreeMap::new();
    for (k, v) in [("banana", 3), ("apple", 1), ("cherry", 2), ("date", 4)] {
        btree.insert(k, v);
    }
    println!("  Sorted entries:");
    for (k, v) in &btree { println!("    {} → {}", k, v); }

    // ── VecDeque ─────────────────────────────────────────────────────
    println!("\n--- VecDeque ---");
    let mut dq: VecDeque<i32> = VecDeque::new();
    dq.push_back(1); dq.push_back(2); dq.push_back(3);
    dq.push_front(0);
    println!("  Deque   : {:?}", dq);
    println!("  Pop front: {:?}", dq.pop_front());
    println!("  Pop back : {:?}", dq.pop_back());
    println!("  Deque   : {:?}", dq);
}
