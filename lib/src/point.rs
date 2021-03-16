pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// Just a two-dimensional array.
pub struct Grid<T, const W: usize, const H: usize>
where
    [u8; W * H]: Sized,
{
    array: [T; W * H],
}

impl<T, const W: usize, const H: usize> Grid<T, W, H>
where
    [u8; W * H]: Sized,
{
    /// Get the element at the specified 2D location in the grid.
    pub fn get_at(&self, p: &Point) -> &T {
        &self.array[p.y * W + p.x]
    }

    /// How many elements are in the grid?
    pub fn size(&self) -> usize {
        W * H
    }

    pub fn height(&self) -> usize {
        H
    }

    pub fn width(&self) -> usize {
        W
    }

    /// Returns a closure which can translate 1d indices
    /// to 2d indices.
    pub fn indexer(&self) -> impl Fn(usize) -> Point {
        |i| Point { x: i % W, y: i / W }
    }

    pub fn mut_slice(&mut self) -> &mut [T] {
        &mut self.array
    }
}

impl<T, const W: usize, const H: usize> Default for Grid<T, W, H>
where
    [u8; W * H]: Sized,
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            array: [Default::default(); W * H],
        }
    }
}
