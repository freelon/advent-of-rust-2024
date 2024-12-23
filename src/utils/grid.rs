use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use itertools::Itertools;

/// A grid structure, indexed by (x, y) tuples. The top-left coordinate is (0, 0).
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Grid<T> {
    pub content_width: i64,
    pub content_height: i64,
    content: Vec<T>,
}

pub type Coord = (i64, i64);

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
        if !self.contains_key(c) {
            panic!(
                "point {c:?} out of range (width: {}, height: {})",
                self.content_width, self.content_height
            );
        }
        (c.1 * self.content_width + c.0) as usize
    }

    pub fn get(&self, c: Coord) -> Option<&T> {
        if !self.contains_key(c) {
            return None;
        }
        let index = self.index_of(c);
        if index < self.content.len() {
            Some(&self.content[self.index_of(c)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, c: Coord) -> Option<&mut T> {
        if !self.contains_key(c) {
            return None;
        }
        let index = self.index_of(c);
        if index < self.content.len() {
            Some(&mut self.content[index])
        } else {
            None
        }
    }

    pub fn set(&mut self, c: Coord, value: T) {
        let index_of = self.index_of(c);
        self.content[index_of] = value;
    }

    pub fn contains_key(&self, c: Coord) -> bool {
        0 <= c.0 && c.0 < self.content_width && 0 <= c.1 && c.1 < self.content_height
    }

    pub fn entries(&self) -> impl Iterator<Item = (Coord, &T)> {
        self.content.iter().enumerate().map(|(i, val)| {
            (
                (i as i64 % self.content_width, i as i64 / self.content_width),
                val,
            )
        })
    }

    pub fn entries_mut(&mut self) -> impl Iterator<Item = (Coord, &mut T)> {
        self.content.iter_mut().enumerate().map(|(i, val)| {
            (
                (i as i64 % self.content_width, i as i64 / self.content_width),
                val,
            )
        })
    }

    pub fn map_values<U>(self, f: fn(T) -> U) -> Grid<U> {
        let new_content = self.content.into_iter().map(f).collect_vec();
        Grid {
            content_width: self.content_width,
            content_height: self.content_height,
            content: new_content,
        }
    }
}

impl<T> Grid<T>
where
    T: Default,
{
    pub fn from_default(width: i64, height: i64) -> Grid<T> {
        let mut content: Vec<T> = Vec::with_capacity((width * height) as usize);
        content.resize_with((width * height) as usize, Default::default);
        Grid {
            content_width: width,
            content_height: height,
            content,
        }
    }
}

impl Grid<char> {
    pub fn parse(input: &str) -> Grid<char> {
        let content_width = input.lines().next().unwrap().len();
        let mut content_height = 0;
        let mut content = vec![];
        for line in input.lines() {
            content_height += 1;
            content.reserve(content_width);
            line.chars().for_each(|v| content.push(v));
        }
        Grid {
            content_width: content_width as i64,
            content_height,
            content,
        }
    }

    pub fn print(&self) {
        for line in &self.content.iter().chunks(self.content_width as usize) {
            for c in line.into_iter() {
                print!("{c}");
            }
            println!();
        }
    }
}

impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Coord) -> &Self::Output {
        &self.content[self.index_of(index)]
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let index_of = self.index_of(index);
        &mut self.content[index_of]
    }
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::Grid;
    use std::collections::HashMap;

    #[test]
    fn init_and_read() {
        let grid: Grid<char> = Grid::from(HashMap::from_iter([
            ((0, 0), '.'),
            ((0, 1), '.'),
            ((1, 0), '.'),
            ((1, 1), '#'),
        ]));
        assert_eq!('.', grid[(0, 0)]);
        assert_eq!('.', grid[(0, 1)]);
        assert_eq!('.', grid[(1, 0)]);
        assert_eq!('#', grid[(1, 1)]);
    }

    #[test]
    fn mutate_by_index() {
        let mut grid = generate();
        grid[(0, 1)] = 'x';
        assert_eq!('x', grid[(0, 1)]);
    }
    #[test]
    fn mutate_option() {
        let mut grid = generate();
        if let Some(value) = grid.get_mut((0, 1)) {
            *value = 'x';
        }
        assert_eq!('x', grid[(0, 1)]);
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

    #[test]
    fn from_str() {
        let s = "..\n.x";
        let m_str = Grid::parse(s);
        let mut m_gen = generate();
        m_gen[(1, 1)] = 'x';
        assert_eq!(m_gen, m_str);
    }

    #[test]
    fn entries() {
        let v = vec![
            ((0, 0), &'.'),
            ((1, 0), &'.'),
            ((0, 1), &'.'),
            ((1, 1), &'.'),
        ];
        assert_eq!(v, generate().entries().collect_vec());
    }

    #[should_panic]
    #[test]
    fn out_of_bounds_panics1() {
        generate()[(-1, 3)];
    }

    #[should_panic]
    #[test]
    fn out_of_bounds_panics2() {
        generate()[(1, 3)];
    }

    #[test]
    fn option_success() {
        assert_eq!(Some(&'.'), generate().get((0, 0)));
    }

    #[test]
    fn option_out_of_bounds() {
        assert_eq!(None, generate().get((30, 0)));
    }

    #[test]
    fn value_mapping() {
        let a: Grid<char> = generate();
        let b: Grid<String> = a.map_values(|old| format!("{old}string"));
        assert_eq!(".string".to_string(), b[(0, 0)]);
    }
}
