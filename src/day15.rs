use std::fs::read_to_string;

use crate::utils::grid::Grid;

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

fn part2(_input: &str) -> RiddleResult {
    0
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

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 2028);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
