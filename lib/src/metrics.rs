use std::time::Duration;

#[derive(Debug)]
pub struct Metrics {
    // How many rays were traced in total
    pub rays_traced_total: usize,
    pub time_spent: Duration,
}

impl Metrics {
    pub fn new(rays_traced_total: usize) -> Self {
        Metrics {
            rays_traced_total,
            time_spent: Duration::from_millis(0),
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "{}ns per ray\n{} rays\n{} seconds",
            self.time_spent.as_nanos() / (self.rays_traced_total as u128),
            self.rays_traced_total,
            self.time_spent.as_secs(),
        )
    }
}
