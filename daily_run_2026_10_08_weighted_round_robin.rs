#[derive(Clone, Debug)]
pub struct Worker {
    pub name: String,
    pub weight: u32,
    current: i64,
}

/// Selects workers proportionally while keeping assignments smooth over time.
pub fn next_worker(workers: &mut [Worker]) -> Option<usize> {
    let total: i64 = workers.iter().map(|worker| worker.weight as i64).sum();
    if total == 0 { return None; }
    let mut selected = None;
    let mut best = i64::MIN;
    for (index, worker) in workers.iter_mut().enumerate() {
        worker.current += worker.weight as i64;
        if worker.current > best {
            best = worker.current;
            selected = Some(index);
        }
    }
    let index = selected?;
    workers[index].current -= total;
    Some(index)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_weight_ratio() {
        let mut workers = vec![
            Worker { name: "fast".into(), weight: 3, current: 0 },
            Worker { name: "slow".into(), weight: 1, current: 0 },
        ];
        let picks: Vec<_> = (0..8).map(|_| next_worker(&mut workers).unwrap()).collect();
        assert_eq!(picks.iter().filter(|&&i| i == 0).count(), 6);
        assert_eq!(picks.iter().filter(|&&i| i == 1).count(), 2);
    }
}
