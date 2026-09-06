#[derive(Debug, PartialEq)]
pub struct Summary {
    pub accepted: usize,
    pub rejected: usize,
    pub mean: f64,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
}

/// Summarize finite numbers in a zero-based CSV column.
pub fn summarize<'a, I>(rows: I, column: usize) -> Summary
where
    I: IntoIterator<Item = &'a str>,
{
    let mut accepted = 0;
    let mut rejected = 0;
    let mut sum = 0.0;
    let mut minimum = None;
    let mut maximum = None;
    for row in rows {
        let Some(field) = row.split(',').nth(column) else { rejected += 1; continue };
        match field.trim().parse::<f64>() {
            Ok(value) if value.is_finite() => {
                accepted += 1;
                sum += value;
                minimum = Some(minimum.map_or(value, |current| current.min(value)));
                maximum = Some(maximum.map_or(value, |current| current.max(value)));
            }
            _ => rejected += 1,
        }
    }
    Summary { accepted, rejected, mean: if accepted == 0 { 0.0 } else { sum / accepted as f64 }, minimum, maximum }
}

fn main() {
    println!("{:?}", summarize(["Ada,91", "Linus,88", "bad"], 1));
}
