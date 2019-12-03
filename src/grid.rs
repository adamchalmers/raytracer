pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub v: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let mut v: Vec<T> = Vec::new();
        v.reserve(width * height);
        Grid { width, height, v }
    }

    pub fn get(&self, p: Point) -> &T {
        &self.v[p.y * self.width + p.x]
    }

    pub fn set(&mut self, p: Point, val: T) {
        self.v[p.y * self.width + p.x] = val;
    }
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}
