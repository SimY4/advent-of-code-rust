use std::iter::successors;

const FORBIDDEN: [char; 3] = ['i', 'o', 'l'];

fn next_char(ch: char) -> char {
    ((ch as u8) + 1u8) as char
}

fn increment(password: &Vec<char>) -> Vec<char> {
    let mut new_pass = password
        .iter()
        .cloned()
        .rev()
        .scan(true, |should_update, ch| {
            Some(match ch {
                'z' if *should_update => 'a',
                ch if *should_update => {
                    *should_update = false;
                    next_char(ch)
                }
                _ => ch,
            })
        })
        .collect::<Vec<char>>();
    new_pass.reverse();
    new_pass
}

fn meet_requirement(password: &Vec<char>) -> bool {
    password
        .windows(3)
        .any(|triple| next_char(triple[0]) == triple[1] && next_char(triple[1]) == triple[2])
        && !password.iter().any(|ch| FORBIDDEN.contains(ch))
        && {
            let pair_indexes = password
                .windows(2)
                .enumerate()
                .filter_map(|(i, pair)| if pair[0] == pair[1] { Some(i) } else { None })
                .collect::<Vec<usize>>();
            pair_indexes.windows(2)
                .any(|pair| pair[0] + 1 != pair[1])
        }
}

pub fn solve(input: &str) -> Option<String> {
    successors(Some(input.chars().collect::<Vec<char>>()), |password| {
        Some(increment(password))
    })
    .filter(meet_requirement)
    .next()
    .map(|v| v.iter().collect::<String>())
}

pub fn solve2(input: &str) -> Option<String> {
    successors(Some(input.chars().collect::<Vec<char>>()), |password| {
        Some(increment(password))
    })
    .filter(meet_requirement)
    .skip(1)
    .next()
    .map(|v| v.iter().collect::<String>())
}

pub const INPUT: &str = "hxbxwxba";
