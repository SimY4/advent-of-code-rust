use std::iter::successors;

fn digits(i: u32) -> Vec<u8> {
    let mut digits = successors(Some(i), |i| if *i > 0 { Some(i / 10) } else { None })
        .map(|i| (i % 10) as u8)
        .collect::<Vec<u8>>();
    digits.reverse();
    digits
}

pub fn solve(input: &str) -> usize {
    let range = input
        .split('-')
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    (range[0]..=range[1])
        .map(digits)
        .filter(|ds| ds.windows(2).any(|pair| pair[0] == pair[1]))
        .filter(|ds| ds.windows(2).all(|pair| pair[0] <= pair[1]))
        .count()
}

pub fn solve2(input: &str) -> usize {
    let range = input
        .split('-')
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    (range[0]..=range[1])
        .map(digits)
        .filter(|ds| {
            ds.iter()
                .fold(
                    &mut Vec::new(),
                    |acc: &mut std::vec::Vec<(u8, usize)>, digit| {
                        match acc.last_mut() {
                            Some(pair) if pair.0 == *digit => *pair = (*digit, pair.1 + 1),
                            Some(_) | None => acc.push((*digit, 1)),
                        };
                        acc
                    },
                )
                .iter()
                .any(|v| v.1 == 2)
        })
        .filter(|ds| ds.windows(2).all(|pair| pair[0] <= pair[1]))
        .count()
}

pub const INPUT: &str = "264360-746325";
