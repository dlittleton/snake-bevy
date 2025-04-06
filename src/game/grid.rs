use std::iter::repeat_n;

/**
 * Basic 2 dimensional grid to store and retrieve values
 */
pub struct Grid<T> {
    values: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn new(width: usize, height: usize, init: T) -> Self {
        let values: Vec<Vec<_>> = (0..width)
            .map(|_| repeat_n(init, height).collect())
            .collect();

        Self { values }
    }
}

impl<T> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.values[x][y]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.values[x][y]
    }

    pub fn width(&self) -> usize {
        self.values.len()
    }

    pub fn height(&self) -> usize {
        self.values[0].len()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.values
            .iter()
            .enumerate()
            .flat_map(|(x, v)| v.iter().enumerate().map(move |(y, value)| (x, y, value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let g = Grid::new(10, 5, 0);

        assert_eq!(g.width(), 10);
        assert_eq!(g.height(), 5);
    }

    #[test]
    fn test_get_init_value() {
        let g = Grid::new(10, 5, 3);

        assert_eq!(*g.get(2, 2), 3);
    }

    #[test]
    fn test_mutate_value() {
        let mut g = Grid::new(10, 5, 0);

        *g.get_mut(2, 2) = 5;

        assert_eq!(*g.get(0, 0), 0);
        assert_eq!(*g.get(2, 2), 5);
    }
}
