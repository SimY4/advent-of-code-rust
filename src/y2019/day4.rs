fn digits(i: u32) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut n = i;
    while n > 9 {
        digits.push((n % 10) as u8);
        n = n / 10;
    };
    digits.push(n as u8);
    digits.reverse();
    digits
}

pub fn solve(input: &str) -> usize {
    let range = input.split('-')
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    (range[0]..=range[1])
        .map(digits)
        .filter(|ds| ds.windows(2).any(|pair| pair[0] == pair[1]))
        .filter(|ds| ds.windows(2).all(|pair| pair[0] <= pair[1]))
        .count()
}

pub fn solve2(input: &str) -> usize {
    let range = input.split('-')
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    (range[0]..=range[1])
        .map(digits)
        .filter(|ds| ds.iter()
            .fold(&mut Vec::new(), |acc, digit| {
                if acc.is_empty() {
                    acc.push(vec![digit]);
                } else {
                    let vec = acc.last_mut().unwrap();
                    if vec[0] == digit {
                        vec.push(digit);
                    } else {
                        acc.push(vec![digit])
                    }
                };
                acc
            }).iter()
            .any(|v| v.len() == 2))
        .filter(|ds| ds.windows(2).all(|pair| pair[0] <= pair[1]))
        .count()
}

pub const INPUT: &str = "264360-746325";