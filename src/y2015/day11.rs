use std::iter::successors;

const FORBIDDEN: [char; 3] = ['i', 'o', 'l'];

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
                    ((ch as u8) + 1u8) as char
                }
                _ => ch
            })
        })
        .collect::<Vec<char>>();
    new_pass.reverse();
    new_pass
}

fn meet_requirement(password: &Vec<char>) -> bool {
    password.windows(3).any(|triple| triple.windows(2).all(|pair| pair[0] <= pair[1])) &&
        !password.iter().any(|ch| FORBIDDEN.contains(ch)) &&
        password.windows(2).any(|pair| pair.iter().min() == pair.iter().max())
}

pub fn solve(input: &str) -> Option<String> {
    successors(Some(input.chars().collect::<Vec<char>>()), |password| Some(increment(password)))
        .filter(meet_requirement)
        .next()
        .map(|v| v.iter().collect::<String>())
}

pub fn solve2(input: &str) -> Option<String> {
    successors(Some(input.chars().collect::<Vec<char>>()), |password| Some(increment(password)))
        .filter(meet_requirement)
        .skip(1)
        .next()
        .map(|v| v.iter().collect::<String>())
}

pub const INPUT: &str = "hxbxwxba";
