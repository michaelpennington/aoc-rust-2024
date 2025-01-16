#![feature(iter_array_chunks)]

use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut l, mut r): (Vec<_>, Vec<_>) = input
        .lines()
        .flat_map(|l| l.split_whitespace().map(|n| n.parse::<u32>().unwrap()))
        .array_chunks()
        .map(|c| c.into())
        .unzip();
    l.sort();
    r.sort();
    (l, r)
}

fn generate_freqs(v: Vec<u32>) -> HashMap<u32, u32> {
    let mut out = HashMap::new();
    for n in v {
        *out.entry(n).or_insert(0) += 1;
    }
    out
}

pub fn part_one(input: &str) -> Option<u32> {
    let (l, r) = parse_input(input);
    Some(l.into_iter().zip(r).map(|(l, r)| l.abs_diff(r)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (l, r) = parse_input(input);
    let (l_freqs, r_freqs) = (generate_freqs(l), generate_freqs(r));
    Some(
        l_freqs
            .into_iter()
            .map(|(n, count)| n * count * r_freqs.get(&n).unwrap_or(&0))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
