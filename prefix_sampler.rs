//! Deterministic weighted sampler backed by prefix sums.

#[derive(Debug, Clone)]
pub struct PrefixSampler<T> {
    items: Vec<T>,
    cumulative: Vec<f64>,
}

impl<T> PrefixSampler<T> {
    pub fn new(entries: impl IntoIterator<Item = (T, f64)>) -> Self {
        let mut items = Vec::new();
        let mut cumulative = Vec::new();
        let mut total = 0.0;
        for (item, weight) in entries {
            assert!(weight.is_finite() && weight > 0.0, "weights must be positive");
            total += weight;
            items.push(item);
            cumulative.push(total);
        }
        assert!(!items.is_empty(), "sampler cannot be empty");
        Self { items, cumulative }
    }

    pub fn total_weight(&self) -> f64 {
        *self.cumulative.last().expect("sampler is non-empty")
    }

    pub fn choose(&self, ticket: f64) -> &T {
        assert!(ticket >= 0.0 && ticket < self.total_weight());
        let index = self.cumulative.partition_point(|&bound| bound <= ticket);
        &self.items[index]
    }

    pub fn choose_normalized(&self, fraction: f64) -> &T {
        assert!((0.0..1.0).contains(&fraction));
        self.choose(fraction * self.total_weight())
    }
}
