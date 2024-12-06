use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn day_main() {
    let input = read_to_string("input/day06.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    let mut m = HashMap::new();
    let mut pos = None;
    input.lines().enumerate().for_each(|(y, line)| {
        line.char_indices().for_each(|(x, c)| {
            let x = x as i32;
            let y = y as i32;
            if c == '^' {
                pos = Some((x, y));
                m.insert((x, y), '.');
            } else {
                m.insert((x, y), c);
            }
        });
    });

    let mut pos = pos.unwrap();
    let mut dir = '^';
    let mut visited = HashSet::new();
    while m.contains_key(&pos) {
        let (x, y) = pos;
        // println!("at {x},{y} {dir}");
        visited.insert(pos);
        let next = match dir {
            '^' => (x, y - 1),
            'v' => (x, y + 1),
            '<' => (x - 1, y),
            '>' => (x + 1, y),
            _ => unreachable!(),
        };
        if let Some('#') = m.get(&next) {
            // println!("bump at {x},{y}");
            dir = match dir {
                '^' => '>',
                'v' => '<',
                '<' => '^',
                '>' => 'v',
                _ => unreachable!("asdf"),
            };
        } else {
            pos = next;
        }
    }
    visited.len()
}

fn part2(input: &str) -> RiddleResult {
    let mut m = HashMap::new();
    let mut pos = None;
    let mut opens = HashSet::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.char_indices().for_each(|(x, c)| {
            let x = x as i32;
            let y = y as i32;
            if c == '^' {
                pos = Some((x, y));
                m.insert((x, y), '.');
            } else {
                m.insert((x, y), c);
            }
            if c == '.' {
                opens.insert((x, y));
            }
        });
    });

    let mut pos = pos.unwrap();
    let mut dir = '^';
    opens
        .into_iter()
        .filter(|open| {
            let mut m = m.clone();
            m.insert(*open, '#');
            is_loop(m, pos, dir)
        })
        .count()
}

fn is_loop(m: HashMap<(i32, i32), char>, mut pos: (i32, i32), mut dir: char) -> bool {
    let mut visited = HashSet::new();
    while m.contains_key(&pos) {
        let (x, y) = pos;
        // println!("at {x},{y} {dir}");
        if visited.contains(&(pos, dir)) {
            return true;
        }
        visited.insert((pos, dir));
        let next = match dir {
            '^' => (x, y - 1),
            'v' => (x, y + 1),
            '<' => (x - 1, y),
            '>' => (x + 1, y),
            _ => unreachable!(),
        };
        if let Some('#') = m.get(&next) {
            // println!("bump at {x},{y}");
            dir = match dir {
                '^' => '>',
                'v' => '<',
                '<' => '^',
                '>' => 'v',
                _ => unreachable!("asdf"),
            };
        } else {
            pos = next;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 41);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }
}
