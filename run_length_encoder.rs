//! Streaming run-length encoding for text-like character iterators.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    pub symbol: char,
    pub count: usize,
}

pub struct RunLengthEncoder<I: Iterator<Item = char>> {
    input: I,
    pending: Option<char>,
}

impl<I: Iterator<Item = char>> RunLengthEncoder<I> {
    pub fn new(input: I) -> Self { Self { input, pending: None } }
}

impl<I: Iterator<Item = char>> Iterator for RunLengthEncoder<I> {
    type Item = Run;

    fn next(&mut self) -> Option<Self::Item> {
        let symbol = self.pending.take().or_else(|| self.input.next())?;
        let mut count = 1;
        while let Some(next) = self.input.next() {
            if next == symbol { count += 1; }
            else { self.pending = Some(next); break; }
        }
        Some(Run { symbol, count })
    }
}

pub fn encode(text: &str) -> Vec<Run> {
    RunLengthEncoder::new(text.chars()).collect()
}

#[cfg(test)]
mod tests {
    use super::{encode, Run};

    #[test]
    fn groups_adjacent_symbols() {
        assert_eq!(encode("aaabbc"), vec![
            Run { symbol: 'a', count: 3 },
            Run { symbol: 'b', count: 2 },
            Run { symbol: 'c', count: 1 },
        ]);
    }
}
