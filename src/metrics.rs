#[derive(Debug)]
pub struct Metrics {
    /// How many rays were eventually reflected into the background
    pub reflected_into_background: u64,
    // How many rays hit the reflection limit
    pub rays_out_of_reflect: u64,
    // How many rays were traced in total
    pub rays_traced_total: u64,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            reflected_into_background: 0,
            rays_out_of_reflect: 0,
            rays_traced_total: 0,
        }
    }

    pub fn describe(&self) -> String {
        format!(
            "{}% of reflected rays eventually hit the background",
            self.reflected_into_background * 100 / self.rays_traced_total
        )
    }
}
