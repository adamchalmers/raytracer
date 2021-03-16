pub struct Grid {
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid { width, height }
    }

    pub fn num_cells(&self) -> usize {
        self.width * self.height
    }

    pub fn to_point(&self, i: usize) -> Point {
        Point {
            x: i % self.width,
            y: i / self.width,
        }
    }

    pub fn to_index(&self, p: Point) -> usize {
        p.y * self.width + p.x
    }
}

pub struct Point {
    pub x: usize,
    pub y: usize,
}
