use std::{collections::HashSet, fs::read_to_string};

use itertools::Itertools;

use crate::utils::grid::{Coord, Grid};

pub fn day_main() {
    let input = read_to_string("input/day15.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = i64;

fn part1(input: &str) -> RiddleResult {
    let (grid, movements) = input.split_once("\n\n").unwrap();
    let mut grid = Grid::parse(grid);
    let mut robot = grid.entries().find(|(_r, c)| **c == '@').unwrap().0;
    let directions = |d| match d {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!(),
    };
    for m in movements.chars().filter(|c| *c != '\n') {
        let dir = directions(m);
        let space = (1..)
            .map(|i| (robot.0 + i * dir.0, robot.1 + i * dir.1))
            .take_while(|p| grid.get(*p).is_some() && (grid[*p] == '.' || grid[*p] == 'O'))
            .find(|p| grid[*p] == '.');
        if let Some(p) = space {
            if (p.0 - robot.0).abs() + (p.1 - robot.1).abs() > 1 {
                // this means: the free spot is not a direct neighbor of the robot, i.e. there are boxes
                grid[p] = 'O';
            }
            grid[robot] = '.';
            robot = (robot.0 + dir.0, robot.1 + dir.1);
            grid[robot] = '@';
        }
    }
    grid.entries()
        .filter(|(_, c)| **c == 'O')
        .map(|((x, y), _)| y * 100 + x)
        .sum()
}

fn part2(input: &str) -> RiddleResult {
    let (grid, movements) = input.split_once("\n\n").unwrap();
    let grid = grid
        .lines()
        .map(|line| {
            {
                line.chars().flat_map(|c| {
                    match c {
                        '.' => "..",
                        '@' => "@.",
                        '#' => "##",
                        'O' => "[]",
                        _ => panic!(),
                    }
                    .chars()
                })
            }
            .join("")
        })
        .join("\n");

    let mut grid = Grid::parse(grid.as_str());

    let mut robot = grid.entries().find(|(_r, c)| **c == '@').unwrap().0;
    let directions = |d| match d {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!(),
    };
    for m in movements.chars().filter(|c| *c != '\n') {
        let dir = directions(m);
        if let Some(tiles_to_move) = movable(robot, dir, &grid, false) {
            let old = tiles_to_move
                .iter()
                .map(|tile| (*tile, grid[*tile]))
                .collect_vec();
            tiles_to_move.iter().for_each(|tile| grid[*tile] = '.');
            old.into_iter()
                .for_each(|(tile, c)| grid[(tile.0 + dir.0, tile.1 + dir.1)] = c);
            robot = (robot.0 + dir.0, robot.1 + dir.1);
        }
    }
    grid.entries()
        .filter(|(_, c)| **c == '[')
        .map(|((x, y), _)| y * 100 + x)
        .sum()
}

fn movable(
    from: (i64, i64),
    dir: (i64, i64),
    grid: &Grid<char>,
    ignore_other_half: bool,
) -> Option<HashSet<Coord>> {
    match grid[from] {
        '.' => Some(HashSet::new()),
        '@' => {
            let next = movable((from.0 + dir.0, from.1 + dir.1), dir, grid, false);
            next.map(|mut v| {
                v.insert(from);
                v
            })
        }
        '[' => {
            if dir.0 != 0 {
                // sideway movement is "regular"
                let next = movable((from.0 + dir.0, from.1 + dir.1), dir, grid, false);
                next.map(|mut v| {
                    v.insert(from);
                    v
                })
            } else {
                // up/down always means the other part of the crate has to move in parallel
                let mut next1 = movable((from.0 + dir.0, from.1 + dir.1), dir, grid, false)?;
                if !ignore_other_half {
                    let next2 = movable((from.0 + 1, from.1), dir, grid, true)?;
                    next1.extend(next2);
                }
                next1.insert(from);
                Some(next1)
            }
        }
        ']' => {
            if dir.0 != 0 {
                // sideway movement is "regular"
                let next = movable((from.0 + dir.0, from.1 + dir.1), dir, grid, false);
                next.map(|mut v| {
                    v.insert(from);
                    v
                })
            } else {
                // up/down always means the other part of the crate has to move in parallel
                let mut next1 = movable((from.0 + dir.0, from.1 + dir.1), dir, grid, false)?;
                if !ignore_other_half {
                    let next2 = movable((from.0 - 1, from.1), dir, grid, true)?;
                    next1.extend(next2);
                }
                next1.insert(from);
                Some(next1)
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const TEST_LARGE: &str = r"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 2028);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_LARGE), 9021);
    }
}
