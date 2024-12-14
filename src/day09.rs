use std::{collections::BTreeMap, fs::read_to_string};

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
    let mut free: BTreeMap<usize, usize> = BTreeMap::new();
    // (start_index, len, file_id)
    let mut files: Vec<(usize, usize, usize)> = Vec::with_capacity(input.len() / 2 + 1);
    let mut head = 0;
    for (i, l) in input.chars().enumerate() {
        let l = l.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.push((head, l, i / 2));
        } else {
            free.insert(head, l);
        }
        head += l as usize;
    }
    for file in files.iter_mut().rev() {
        let (start_index, length, _file_id) = *file;
        let found = free
            .iter()
            .take_while(|f| *f.0 < start_index)
            .find(|f| *f.1 >= length);

        if let Some((&free_start, &free_length)) = found.clone() {
            free.remove(&free_start);
            free.insert(start_index, length);
            file.0 = free_start;
            if length < free_length {
                free.insert(free_start + length, free_length - length);
            }
        }
    }

    files
        .iter()
        .map(|(start_index, l, file_id)| {
            *file_id * (*start_index..start_index + l).sum::<RiddleResult>()
        })
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
