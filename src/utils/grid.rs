use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

/// A grid structure, indexed by (x, y) tuples. The top-left coordinate is (0, 0).
pub struct Grid<T> {
    content_width: i64,
    content_height: i64,
    content: Vec<T>,
}

type Coord = (i64, i64);

impl<T> Grid<T> {
    pub fn from(mut source: HashMap<Coord, T>) -> Grid<T> {
        let x_max = source.keys().max_by_key(|c| c.0).unwrap().0;
        let y_max = source.keys().max_by_key(|c| c.1).unwrap().1;
        let content_width = x_max + 1;
        let content_height = y_max + 1;
        let mut content = Vec::with_capacity((content_width * content_height) as usize);
        for y in 0..content_height {
            for x in 0..content_width {
                let v = source
                    .remove(&(x, y))
                    .unwrap_or_else(|| panic!("no entry for {x}, {y}"));
                content.push(v);
            }
        }
        Grid {
            content_width,
            content_height,
            content,
        }
    }

    fn index_of(&self, c: Coord) -> usize {
        (c.1 * self.content_width + c.0) as usize
    }

    pub fn get(&self, c: Coord) -> &T {
        &self.content[self.index_of(c)]
    }
    pub fn get_mut(&mut self, c: Coord) -> &mut T {
        let index_of = self.index_of(c);
        &mut self.content[index_of]
    }

    pub fn set(&mut self, c: Coord, value: T) {
        let index_of = self.index_of(c);
        self.content[index_of] = value;
    }

    pub fn contains_key(&self, c: Coord) -> bool {
        0 <= c.0 && c.0 < self.content_width && 0 <= c.1 && c.1 < self.content_height
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Coord) -> &Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        self.get_mut(index)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::Grid;

    #[test]
    fn init_and_read() {
        let grid: Grid<char> = Grid::from(HashMap::from_iter([
            ((0, 0), '.'),
            ((0, 1), '.'),
            ((1, 0), '.'),
            ((1, 1), '#'),
        ]));
        assert_eq!(&'.', grid.get((0, 0)));
        assert_eq!(&'.', grid.get((0, 1)));
        assert_eq!(&'.', grid.get((1, 0)));
        assert_eq!(&'#', grid.get((1, 1)));
    }
    #[test]
    fn mutate_by_index() {
        let mut grid = generate();
        grid[(0, 1)] = 'x';
        assert_eq!(&'x', grid.get((0, 1)));
    }

    fn generate() -> Grid<char> {
        let grid: Grid<char> = Grid::from(HashMap::from_iter([
            ((0, 0), '.'),
            ((0, 1), '.'),
            ((1, 0), '.'),
            ((1, 1), '.'),
        ]));
        grid
    }
}
