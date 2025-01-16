#![feature(iter_array_chunks)]

advent_of_code::solution!(2);

struct SkipIter<I> {
    iter: I,
    index_to_skip: usize,
    current: usize,
}

impl<I> SkipIter<I> {
    fn new(iter: I, index_to_skip: usize) -> Self {
        Self {
            iter,
            index_to_skip,
            current: 0,
        }
    }
}

impl<I> Iterator for SkipIter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index_to_skip == self.current {
            self.current += 2;
            let _ = self.iter.next();
            self.iter.next()
        } else {
            self.current += 1;
            self.iter.next()
        }
    }
}

fn is_safe(mut nums: impl Iterator<Item = i32>) -> bool {
    let mut increasing = None;
    let mut prev = nums.next().unwrap();
    for next in nums {
        match next - prev {
            -3..=-1 => match increasing {
                None => increasing = Some(false),
                Some(true) => return false,
                Some(false) => {}
            },
            1..=3 => match increasing {
                None => increasing = Some(true),
                Some(true) => {}
                Some(false) => return false,
            },
            _ => return false,
        }
        prev = next;
    }
    true
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|l| is_safe(l.split_whitespace().map(|n| n.parse::<i32>().unwrap())))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|l| l.split_whitespace().map(|n| n.parse::<i32>().unwrap()))
            .filter(|ns| (0..8).any(|v| is_safe(SkipIter::new(ns.clone(), v))))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
