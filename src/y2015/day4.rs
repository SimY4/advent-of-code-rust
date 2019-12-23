use md5;

fn _solve(input: &str, prefix: &str) -> Option<i32> {
    (1..).find(|i| {
        let digest = md5::compute(format!("{}{}", input, i));
        let hex = format!("{:x}", digest);
        hex.starts_with(prefix)
    })
}

pub fn solve(input: &str) -> Option<i32> {
    _solve(input, "00000")
}

pub fn solve2(input: &str) -> Option<i32> {
    _solve(input, "000000")
}

pub const INPUT: &str = "bgvyzdsv";