pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// Just a two-dimensional array.
pub struct Grid<T, const W: usize, const H: usize>
where
    [T; W * H]: Sized,
{
    array: [T; W * H],
}

impl<T, const W: usize, const H: usize> Grid<T, W, H>
where
    [T; W * H]: Sized,
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
    fn indexer(&self) -> impl Fn(usize) -> Point {
        |i| Point { x: i % W, y: i / W }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.array
    }

    pub fn as_slice(&mut self) -> &[T] {
        &self.array
    }

    /// Recompute each item in the grid using the provided setter function.
    /// The setter function takes the item's location in the grid and outputs
    /// the desired value of that item.
    pub fn set_all_parallel<F>(&mut self, setter: F)
    where
        F: Send + Sync + Fn(Point) -> T,
        T: Send,
    {
        use rayon::prelude::*;
        let to_point = self.indexer();
        self.as_mut_slice()
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, item)| {
                *item = setter(to_point(i));
            });
    }
}

impl<T, const W: usize, const H: usize> Default for Grid<T, W, H>
where
    [T; W * H]: Sized,
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            array: [Default::default(); W * H],
        }
    }
}
