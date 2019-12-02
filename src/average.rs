pub use std::iter::Sum;
pub use std::ops::Div;

/// Consumes values and yields their average.
pub struct Average<T> {
    v: Vec<T>,
}

impl<T> Average<T> {
    /// Get a new Average with no values.
    pub fn new() -> Self {
        Average { v: Vec::new() }
    }
}

// Anything that can be scaled by a number.
pub trait Scalable {
    fn scale(&self, f: f64) -> Self;
}

impl<T> Average<T>
where
    T: Sum<T> + Scalable,
{
    /// Take a value and add it to the average
    pub fn push(&mut self, t: T) {
        self.v.push(t)
    }

    /// Get the average of all previously-pushed values.
    pub fn average(self) -> T {
        let n = self.v.len();
        let total: T = self.v.into_iter().sum();
        let avg: T = total.scale(1.0 / n as f64);
        avg
    }
}
