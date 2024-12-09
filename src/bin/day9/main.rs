use std::collections::VecDeque;

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");
fn main() {
    println!("Day 9 Part 1: {}", part1());
    println!("Day 9 Part 2: {}", part2());
}

pub fn part1() -> usize {
    let mut file_blocks: VecDeque<_> = INPUT
        .trim()
        .chars()
        .step_by(2)
        .enumerate()
        .flat_map(|(i, b)| itertools::repeat_n(i, (b as u8 - b'0') as usize))
        .collect();
    let mut gap_sizes: VecDeque<_> = INPUT
        .trim()
        .chars()
        .skip(1)
        .step_by(2)
        .map(|b| b as u8 - b'0')
        .collect();

    let mut final_disk = vec![];

    while file_blocks.len() > 0 {
        let first = file_blocks.front().unwrap().clone();
        while let Some(front) = file_blocks.front() {
            if *front != first {
                break;
            }
            final_disk.push(file_blocks.pop_front().unwrap());
        }
        if let Some(gap) = gap_sizes.pop_front() {
            for _ in 0..gap {
                if let Some(back) = file_blocks.pop_back() {
                    final_disk.push(back)
                } else {
                    break;
                }
            }
        }
    }

    final_disk
        .iter()
        .enumerate()
        .map(|(idx, id)| id * idx)
        .sum()
}

pub fn part2() -> usize {
    let blocks = INPUT
        .trim()
        .chars()
        .step_by(2)
        .enumerate()
        .map(|(i, b)| (i, (b as u8 - b'0')))
        .collect_vec();
    let mut gaps: Vec<_> = INPUT
        .trim()
        .chars()
        .skip(1)
        .step_by(2)
        .map(|b| b as u8 - b'0')
        .collect_vec();

    let mut moves = vec![];
    for (block_id, block_size) in blocks.iter().rev() {
        let Some(gap) = gaps.iter().position(|g| g >= block_size) else {
            continue;
        };
        if gap >= *block_id {
            continue;
        }
        moves.push((*block_id, block_size, gap));
        gaps[gap] -= block_size;
    }

    let filled_gaps = gaps.iter().enumerate().map(|(i, gap_size)| {
        let mut vec = moves
            .iter()
            .filter(|(_, _, g)| *g == i)
            .map(|(id, s, _)| itertools::repeat_n(*id, **s as usize))
            .flatten()
            .collect_vec();
        vec.append(&mut itertools::repeat_n(0, *gap_size as usize).collect_vec());
        vec
    });

    let unmoved_blocks = blocks.iter().map(|(id, size)| {
        itertools::repeat_n(
            if moves.iter().any(|(block_id, _, _)| block_id == id) {
                0
            } else {
                *id
            },
            *size as usize,
        )
        .collect_vec()
    });

    unmoved_blocks
        .interleave(filled_gaps)
        .flatten()
        .enumerate()
        .map(|(i, block)| i * block)
        .sum()
}
