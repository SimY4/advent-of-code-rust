fn _solve(input: &str, times: usize) -> usize {
    (1..=times)
        .fold(input.chars().collect::<Vec<char>>(), |vec, _| {
            vec.iter()
                .fold(&mut Vec::new(), |acc: &mut Vec<(char, usize)>, ch| {
                    match acc.last_mut() {
                        Some(pair) if pair.0 == *ch => *pair = (*ch, pair.1 + 1),
                        Some(_) | None => acc.push((*ch, 1)),
                    };
                    acc
                })
                .iter()
                .flat_map(|(ch, cnt)| format!("{}{}", cnt, ch).chars().collect::<Vec<char>>())
                .collect::<Vec<char>>()
        })
        .len()
}

pub fn solve(input: &str) -> usize {
    _solve(input, 40)
}

pub fn solve2(input: &str) -> usize {
    _solve(input, 50)
}

pub const INPUT: &str = "1113222113";
