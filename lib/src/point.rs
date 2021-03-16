pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn from_index(i: usize, width: usize) -> Self {
        Point {
            x: i % width,
            y: i / width,
        }
    }

    pub fn to_index(&self, width: usize) -> usize {
        self.y * width + self.x
    }
}
