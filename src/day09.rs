use std::fs::read_to_string;

pub fn day_main() {
    let input = read_to_string("input/day09.txt").unwrap();
    let input = input.trim();
    println!(" part1: {}", part1(input));
    println!(" part2: {}", part2(input));
}

type RiddleResult = usize;

fn part1(input: &str) -> RiddleResult {
    let mut disk = Vec::with_capacity(input.len() * 10);
    for (i, l) in input.chars().enumerate() {
        let l = l.to_digit(10).unwrap();
        let content = if i % 2 == 0 {
            // file
            Some(i / 2) // id based on order of apperance and every second one is a file
        } else {
            None
        };
        for _ in 0..l {
            disk.push(content);
        }
    }
    // dbg!(disk);
    let mut free = disk
        .iter()
        .enumerate()
        .find(|(_, v)| v.is_none())
        .unwrap()
        .0;
    let mut last_block = disk.len() - 1;

    while free < last_block {
        if disk[free].is_some() {
            break;
        }
        disk[free] = disk[last_block];
        disk[last_block] = None;
        while last_block > 0 && disk[last_block].is_none() {
            last_block -= 1;
        }
        while free < disk.len() && disk[free].is_some() {
            free += 1;
        }
    }
    disk.into_iter()
        .take(free)
        .enumerate()
        .map(|(i, v)| i * v.unwrap())
        .sum()
}

fn part2(input: &str) -> RiddleResult {
    let mut disk = Vec::with_capacity(input.len() * 10);
    // (start_index, len, file_id)
    let mut files: Vec<(usize, u32, usize)> = Vec::with_capacity(input.len() / 2 + 1);
    for (i, l) in input.chars().enumerate() {
        let l = l.to_digit(10).unwrap();
        let content = if i % 2 == 0 {
            // file
            Some(i / 2) // id based on order of apperance and every second one is a file
        } else {
            None
        };
        if i % 2 == 0 {
            files.push((disk.len(), l, i / 2));
        }
        for _ in 0..l {
            disk.push(content);
        }
    }
    while let Some((start_index, length, file_id)) = files.pop() {
        let mut seeker = 0;
        let mut found = None;
        while disk[seeker] != Some(file_id) {
            if disk[seeker..seeker + length as usize]
                .iter()
                .all(|v| v.is_none())
            {
                found = Some(seeker);
                break;
            }
            seeker += 1;
        }
        if let Some(empty_start) = found {
            for i in 0..length as usize {
                disk[empty_start + i] = Some(file_id);
                disk[start_index + i] = None;
            }
        }
    }

    disk.into_iter()
        .enumerate()
        .map(|(i, v)| if let Some(value) = v { i * value } else { 0 })
        .sum()
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const TEST_INPUT: &str = r"2333133121414131402";

    #[test]
    fn test1() {
        assert_eq!(part1(TEST_INPUT), 1928);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(TEST_INPUT), 2858);
    }
}
