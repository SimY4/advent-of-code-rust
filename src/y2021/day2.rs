enum Direction {
    Forward,
    Up,
    Down,
}

fn parse_line(line: &str) -> (Direction, i32) {
    match line.split_whitespace().collect::<Vec<&str>>()[..2] {
        ["forward", n] => (Direction::Forward, n.parse::<i32>().unwrap()),
        ["up", n] => (Direction::Up, n.parse::<i32>().unwrap()),
        ["down", n] => (Direction::Down, n.parse::<i32>().unwrap()),
        _ => unreachable!(),
    }
}

pub fn solve(input: &str) -> i32 {
    let (x, y) = input
        .lines()
        .map(parse_line)
        .fold((0, 0), |(x, y), (dir, n)| match dir {
            Direction::Forward => (x + n, y),
            Direction::Up => (x, y - n),
            Direction::Down => (x, y + n),
        });
    x * y
}

pub fn solve2(input: &str) -> i32 {
    let (x, y, _) =
        input
            .lines()
            .map(parse_line)
            .fold((0, 0, 0), |(x, y, aim), (dir, n)| match dir {
                Direction::Forward => (x + n, y + aim * n, aim),
                Direction::Up => (x, y, aim - n),
                Direction::Down => (x, y, aim + n),
            });
    x * y
}

pub const INPUT: &str = "forward 7\n\
                         forward 9\n\
                         forward 3\n\
                         down 5\n\
                         down 9\n\
                         forward 6\n\
                         down 2\n\
                         forward 2\n\
                         forward 8\n\
                         forward 3\n\
                         forward 5\n\
                         forward 5\n\
                         forward 8\n\
                         down 6\n\
                         forward 8\n\
                         forward 2\n\
                         up 8\n\
                         down 8\n\
                         forward 6\n\
                         down 4\n\
                         down 5\n\
                         forward 2\n\
                         down 6\n\
                         forward 7\n\
                         down 9\n\
                         forward 9\n\
                         down 2\n\
                         down 7\n\
                         up 6\n\
                         up 3\n\
                         up 7\n\
                         down 9\n\
                         forward 1\n\
                         forward 1\n\
                         down 4\n\
                         down 9\n\
                         forward 4\n\
                         up 4\n\
                         forward 8\n\
                         forward 9\n\
                         down 7\n\
                         down 4\n\
                         up 6\n\
                         down 8\n\
                         down 2\n\
                         forward 8\n\
                         forward 6\n\
                         down 3\n\
                         forward 2\n\
                         forward 6\n\
                         down 3\n\
                         forward 1\n\
                         forward 8\n\
                         down 8\n\
                         down 9\n\
                         forward 5\n\
                         forward 4\n\
                         forward 8\n\
                         down 7\n\
                         forward 4\n\
                         forward 3\n\
                         forward 6\n\
                         down 3\n\
                         forward 6\n\
                         forward 6\n\
                         down 9\n\
                         down 9\n\
                         down 9\n\
                         down 2\n\
                         down 7\n\
                         down 4\n\
                         forward 3\n\
                         up 7\n\
                         up 3\n\
                         down 1\n\
                         forward 4\n\
                         up 9\n\
                         forward 4\n\
                         forward 2\n\
                         down 2\n\
                         forward 9\n\
                         up 4\n\
                         forward 5\n\
                         down 8\n\
                         up 7\n\
                         down 5\n\
                         down 1\n\
                         up 7\n\
                         up 4\n\
                         forward 5\n\
                         up 8\n\
                         up 3\n\
                         down 2\n\
                         down 1\n\
                         down 2\n\
                         forward 3\n\
                         up 1\n\
                         forward 1\n\
                         forward 1\n\
                         down 1\n\
                         down 6\n\
                         down 6\n\
                         up 4\n\
                         down 4\n\
                         down 4\n\
                         forward 6\n\
                         down 6\n\
                         forward 7\n\
                         forward 5\n\
                         up 7\n\
                         down 9\n\
                         down 6\n\
                         forward 5\n\
                         forward 6\n\
                         forward 2\n\
                         down 4\n\
                         forward 5\n\
                         forward 8\n\
                         down 8\n\
                         down 6\n\
                         forward 2\n\
                         forward 8\n\
                         down 3\n\
                         forward 6\n\
                         down 1\n\
                         forward 5\n\
                         down 8\n\
                         up 1\n\
                         forward 6\n\
                         down 7\n\
                         forward 4\n\
                         down 8\n\
                         down 8\n\
                         forward 8\n\
                         down 6\n\
                         down 3\n\
                         forward 2\n\
                         forward 8\n\
                         forward 9\n\
                         forward 4\n\
                         forward 3\n\
                         down 4\n\
                         forward 3\n\
                         down 9\n\
                         down 1\n\
                         forward 2\n\
                         forward 3\n\
                         forward 7\n\
                         down 1\n\
                         forward 6\n\
                         forward 8\n\
                         forward 6\n\
                         forward 2\n\
                         down 8\n\
                         up 9\n\
                         forward 6\n\
                         forward 8\n\
                         down 7\n\
                         down 5\n\
                         up 4\n\
                         forward 9\n\
                         up 7\n\
                         up 3\n\
                         forward 3\n\
                         down 6\n\
                         forward 4\n\
                         forward 2\n\
                         down 3\n\
                         forward 9\n\
                         forward 5\n\
                         up 7\n\
                         down 9\n\
                         up 4\n\
                         down 3\n\
                         forward 8\n\
                         up 1\n\
                         forward 2\n\
                         forward 8\n\
                         forward 8\n\
                         forward 5\n\
                         down 7\n\
                         up 6\n\
                         down 9\n\
                         down 4\n\
                         forward 2\n\
                         down 5\n\
                         down 2\n\
                         down 2\n\
                         forward 6\n\
                         down 2\n\
                         forward 9\n\
                         forward 1\n\
                         up 1\n\
                         forward 4\n\
                         down 1\n\
                         forward 3\n\
                         down 3\n\
                         forward 4\n\
                         up 5\n\
                         up 3\n\
                         forward 6\n\
                         forward 8\n\
                         forward 2\n\
                         forward 6\n\
                         up 5\n\
                         down 9\n\
                         down 8\n\
                         forward 3\n\
                         down 5\n\
                         forward 8\n\
                         forward 1\n\
                         down 9\n\
                         up 3\n\
                         down 2\n\
                         down 9\n\
                         up 8\n\
                         down 2\n\
                         up 7\n\
                         up 2\n\
                         up 3\n\
                         down 9\n\
                         down 1\n\
                         down 7\n\
                         down 1\n\
                         forward 1\n\
                         down 9\n\
                         down 6\n\
                         forward 3\n\
                         up 7\n\
                         up 8\n\
                         down 5\n\
                         down 6\n\
                         up 2\n\
                         forward 8\n\
                         down 4\n\
                         up 1\n\
                         forward 4\n\
                         up 4\n\
                         forward 2\n\
                         down 4\n\
                         forward 4\n\
                         down 9\n\
                         up 4\n\
                         forward 8\n\
                         up 7\n\
                         forward 1\n\
                         down 3\n\
                         up 7\n\
                         forward 5\n\
                         down 5\n\
                         forward 2\n\
                         forward 7\n\
                         forward 3\n\
                         down 8\n\
                         forward 4\n\
                         forward 9\n\
                         up 2\n\
                         down 4\n\
                         down 5\n\
                         forward 4\n\
                         down 4\n\
                         up 6\n\
                         down 8\n\
                         up 1\n\
                         down 1\n\
                         up 6\n\
                         up 6\n\
                         down 7\n\
                         down 7\n\
                         forward 2\n\
                         forward 4\n\
                         forward 8\n\
                         down 8\n\
                         down 4\n\
                         down 4\n\
                         down 7\n\
                         forward 4\n\
                         down 3\n\
                         forward 5\n\
                         forward 5\n\
                         forward 7\n\
                         down 7\n\
                         forward 1\n\
                         down 8\n\
                         up 4\n\
                         up 9\n\
                         up 3\n\
                         up 6\n\
                         forward 5\n\
                         forward 5\n\
                         forward 4\n\
                         forward 9\n\
                         down 9\n\
                         forward 4\n\
                         forward 1\n\
                         up 8\n\
                         up 2\n\
                         down 9\n\
                         up 4\n\
                         forward 2\n\
                         up 8\n\
                         forward 6\n\
                         forward 2\n\
                         up 9\n\
                         down 3\n\
                         forward 3\n\
                         up 7\n\
                         down 7\n\
                         forward 4\n\
                         forward 7\n\
                         forward 3\n\
                         down 4\n\
                         down 5\n\
                         forward 7\n\
                         up 3\n\
                         up 1\n\
                         down 4\n\
                         forward 6\n\
                         down 1\n\
                         forward 1\n\
                         down 4\n\
                         down 3\n\
                         forward 9\n\
                         forward 4\n\
                         down 9\n\
                         down 3\n\
                         forward 2\n\
                         forward 5\n\
                         forward 6\n\
                         down 3\n\
                         forward 5\n\
                         down 9\n\
                         forward 2\n\
                         forward 9\n\
                         down 7\n\
                         down 4\n\
                         down 3\n\
                         down 1\n\
                         up 2\n\
                         forward 6\n\
                         forward 4\n\
                         down 9\n\
                         down 2\n\
                         forward 2\n\
                         forward 9\n\
                         down 3\n\
                         forward 8\n\
                         down 8\n\
                         forward 5\n\
                         down 4\n\
                         forward 4\n\
                         up 6\n\
                         up 3\n\
                         down 3\n\
                         down 9\n\
                         forward 5\n\
                         forward 8\n\
                         down 2\n\
                         forward 9\n\
                         forward 5\n\
                         up 9\n\
                         forward 2\n\
                         forward 3\n\
                         forward 4\n\
                         up 8\n\
                         up 1\n\
                         up 6\n\
                         down 5\n\
                         down 8\n\
                         down 4\n\
                         forward 6\n\
                         up 2\n\
                         forward 1\n\
                         forward 7\n\
                         up 8\n\
                         forward 5\n\
                         up 9\n\
                         forward 7\n\
                         down 6\n\
                         up 5\n\
                         up 7\n\
                         up 1\n\
                         down 3\n\
                         up 6\n\
                         forward 1\n\
                         up 1\n\
                         forward 2\n\
                         forward 4\n\
                         forward 5\n\
                         up 3\n\
                         up 8\n\
                         up 1\n\
                         up 6\n\
                         up 3\n\
                         down 5\n\
                         down 4\n\
                         up 8\n\
                         down 9\n\
                         up 7\n\
                         down 6\n\
                         down 9\n\
                         forward 5\n\
                         forward 3\n\
                         down 9\n\
                         down 3\n\
                         down 6\n\
                         up 3\n\
                         up 8\n\
                         down 4\n\
                         down 1\n\
                         up 9\n\
                         up 9\n\
                         forward 8\n\
                         down 7\n\
                         forward 1\n\
                         forward 4\n\
                         down 8\n\
                         forward 2\n\
                         down 4\n\
                         forward 7\n\
                         forward 3\n\
                         forward 5\n\
                         forward 1\n\
                         up 2\n\
                         down 9\n\
                         down 5\n\
                         up 6\n\
                         down 3\n\
                         forward 1\n\
                         up 9\n\
                         forward 6\n\
                         forward 1\n\
                         forward 4\n\
                         up 7\n\
                         forward 6\n\
                         down 1\n\
                         forward 9\n\
                         forward 1\n\
                         forward 3\n\
                         down 9\n\
                         down 8\n\
                         down 5\n\
                         forward 4\n\
                         down 7\n\
                         up 1\n\
                         forward 8\n\
                         up 4\n\
                         forward 6\n\
                         down 2\n\
                         forward 4\n\
                         forward 7\n\
                         down 8\n\
                         forward 6\n\
                         down 7\n\
                         forward 7\n\
                         up 7\n\
                         forward 4\n\
                         down 8\n\
                         down 8\n\
                         forward 8\n\
                         forward 6\n\
                         down 9\n\
                         down 8\n\
                         down 6\n\
                         down 2\n\
                         down 4\n\
                         forward 7\n\
                         forward 3\n\
                         down 8\n\
                         down 5\n\
                         forward 2\n\
                         down 9\n\
                         down 7\n\
                         up 1\n\
                         up 5\n\
                         forward 6\n\
                         up 8\n\
                         up 7\n\
                         up 4\n\
                         down 6\n\
                         down 6\n\
                         down 8\n\
                         down 9\n\
                         down 2\n\
                         forward 6\n\
                         forward 6\n\
                         forward 2\n\
                         up 9\n\
                         forward 6\n\
                         forward 9\n\
                         forward 8\n\
                         down 5\n\
                         down 3\n\
                         forward 1\n\
                         forward 8\n\
                         forward 1\n\
                         forward 3\n\
                         down 4\n\
                         forward 5\n\
                         forward 1\n\
                         forward 6\n\
                         down 8\n\
                         down 9\n\
                         forward 3\n\
                         forward 2\n\
                         forward 1\n\
                         forward 3\n\
                         up 7\n\
                         down 7\n\
                         down 2\n\
                         forward 3\n\
                         down 5\n\
                         down 2\n\
                         down 7\n\
                         down 9\n\
                         down 5\n\
                         down 7\n\
                         down 9\n\
                         up 7\n\
                         forward 7\n\
                         forward 9\n\
                         forward 8\n\
                         forward 5\n\
                         down 1\n\
                         up 6\n\
                         up 6\n\
                         forward 5\n\
                         up 6\n\
                         down 8\n\
                         up 6\n\
                         forward 2\n\
                         down 9\n\
                         down 5\n\
                         up 8\n\
                         up 7\n\
                         down 8\n\
                         down 7\n\
                         up 3\n\
                         down 5\n\
                         forward 6\n\
                         forward 2\n\
                         down 6\n\
                         forward 6\n\
                         forward 1\n\
                         forward 5\n\
                         forward 3\n\
                         down 4\n\
                         forward 3\n\
                         down 1\n\
                         up 7\n\
                         forward 3\n\
                         forward 9\n\
                         forward 3\n\
                         forward 4\n\
                         down 9\n\
                         forward 6\n\
                         down 1\n\
                         up 6\n\
                         forward 2\n\
                         forward 1\n\
                         down 2\n\
                         down 1\n\
                         down 9\n\
                         forward 1\n\
                         up 8\n\
                         down 1\n\
                         up 3\n\
                         forward 3\n\
                         forward 1\n\
                         up 6\n\
                         down 1\n\
                         down 7\n\
                         down 2\n\
                         forward 5\n\
                         down 4\n\
                         forward 4\n\
                         forward 9\n\
                         down 7\n\
                         forward 6\n\
                         down 4\n\
                         forward 8\n\
                         down 5\n\
                         forward 6\n\
                         down 6\n\
                         down 6\n\
                         down 9\n\
                         forward 3\n\
                         forward 2\n\
                         forward 7\n\
                         forward 6\n\
                         forward 8\n\
                         up 6\n\
                         forward 7\n\
                         down 2\n\
                         up 4\n\
                         forward 6\n\
                         forward 3\n\
                         forward 9\n\
                         down 1\n\
                         forward 9\n\
                         down 1\n\
                         forward 6\n\
                         down 9\n\
                         forward 7\n\
                         forward 9\n\
                         forward 6\n\
                         up 3\n\
                         down 3\n\
                         forward 3\n\
                         up 1\n\
                         down 8\n\
                         forward 7\n\
                         down 4\n\
                         forward 7\n\
                         forward 7\n\
                         down 1\n\
                         forward 5\n\
                         down 6\n\
                         forward 6\n\
                         down 8\n\
                         down 2\n\
                         down 7\n\
                         forward 9\n\
                         forward 7\n\
                         forward 2\n\
                         down 5\n\
                         forward 7\n\
                         forward 8\n\
                         forward 5\n\
                         forward 5\n\
                         up 1\n\
                         down 1\n\
                         up 4\n\
                         forward 5\n\
                         forward 8\n\
                         down 4\n\
                         up 8\n\
                         forward 8\n\
                         up 2\n\
                         down 1\n\
                         down 9\n\
                         up 9\n\
                         down 9\n\
                         forward 3\n\
                         forward 1\n\
                         down 7\n\
                         down 2\n\
                         forward 5\n\
                         up 7\n\
                         forward 9\n\
                         forward 1\n\
                         down 4\n\
                         down 8\n\
                         down 2\n\
                         up 1\n\
                         up 6\n\
                         forward 9\n\
                         down 3\n\
                         down 2\n\
                         forward 5\n\
                         forward 4\n\
                         down 5\n\
                         down 4\n\
                         up 4\n\
                         forward 4\n\
                         down 3\n\
                         up 3\n\
                         down 7\n\
                         down 7\n\
                         forward 1\n\
                         forward 4\n\
                         forward 7\n\
                         forward 5\n\
                         down 4\n\
                         down 7\n\
                         forward 1\n\
                         forward 9\n\
                         down 4\n\
                         forward 8\n\
                         up 4\n\
                         down 9\n\
                         down 9\n\
                         up 6\n\
                         up 3\n\
                         forward 2\n\
                         forward 3\n\
                         up 7\n\
                         forward 7\n\
                         down 4\n\
                         forward 5\n\
                         forward 5\n\
                         up 2\n\
                         down 5\n\
                         down 9\n\
                         forward 9\n\
                         forward 7\n\
                         forward 1\n\
                         up 5\n\
                         up 5\n\
                         forward 8\n\
                         forward 3\n\
                         forward 2\n\
                         down 4\n\
                         down 6\n\
                         down 2\n\
                         forward 5\n\
                         down 3\n\
                         down 9\n\
                         forward 8\n\
                         forward 7\n\
                         forward 7\n\
                         down 1\n\
                         up 3\n\
                         down 8\n\
                         down 9\n\
                         forward 6\n\
                         up 6\n\
                         down 6\n\
                         forward 2\n\
                         forward 3\n\
                         forward 7\n\
                         up 8\n\
                         down 8\n\
                         down 7\n\
                         forward 2\n\
                         down 2\n\
                         up 7\n\
                         up 9\n\
                         forward 1\n\
                         forward 1\n\
                         forward 1\n\
                         forward 1\n\
                         forward 1\n\
                         up 8\n\
                         down 3\n\
                         up 8\n\
                         down 5\n\
                         down 3\n\
                         up 4\n\
                         forward 4\n\
                         down 3\n\
                         down 4\n\
                         down 3\n\
                         up 3\n\
                         down 3\n\
                         up 2\n\
                         up 6\n\
                         down 9\n\
                         down 6\n\
                         up 8\n\
                         up 7\n\
                         down 1\n\
                         down 7\n\
                         down 3\n\
                         forward 3\n\
                         forward 5\n\
                         down 4\n\
                         down 7\n\
                         forward 1\n\
                         forward 8\n\
                         up 9\n\
                         up 2\n\
                         forward 3\n\
                         up 1\n\
                         forward 7\n\
                         down 7\n\
                         down 5\n\
                         forward 9\n\
                         up 9\n\
                         forward 3\n\
                         down 2\n\
                         up 4\n\
                         down 2\n\
                         down 1\n\
                         down 9\n\
                         down 9\n\
                         forward 3\n\
                         forward 4\n\
                         down 2\n\
                         down 6\n\
                         up 8\n\
                         down 5\n\
                         forward 7\n\
                         forward 4\n\
                         up 3\n\
                         forward 2\n\
                         down 4\n\
                         down 8\n\
                         forward 4\n\
                         forward 6\n\
                         forward 8\n\
                         down 6\n\
                         down 8\n\
                         up 2\n\
                         forward 5\n\
                         up 7\n\
                         down 9\n\
                         down 6\n\
                         forward 7\n\
                         up 3\n\
                         down 9\n\
                         forward 2\n\
                         down 6\n\
                         up 6\n\
                         down 6\n\
                         down 3\n\
                         down 2\n\
                         down 8\n\
                         down 4\n\
                         forward 8\n\
                         up 7\n\
                         forward 9\n\
                         forward 4\n\
                         down 3\n\
                         forward 3\n\
                         down 9\n\
                         down 2\n\
                         forward 2\n\
                         forward 1\n\
                         down 4\n\
                         down 3\n\
                         down 8\n\
                         up 6\n\
                         down 4\n\
                         forward 3\n\
                         down 7\n\
                         forward 8\n\
                         down 7\n\
                         forward 6\n\
                         forward 2\n\
                         forward 7\n\
                         forward 6\n\
                         forward 4\n\
                         up 4\n\
                         forward 2\n\
                         down 4\n\
                         down 2\n\
                         forward 3\n\
                         down 2\n\
                         up 9\n\
                         down 6\n\
                         forward 5\n\
                         up 6\n\
                         forward 1\n\
                         up 1\n\
                         down 3\n\
                         up 4\n\
                         forward 1\n\
                         down 6\n\
                         forward 9\n\
                         up 2\n\
                         forward 4\n\
                         up 9\n\
                         up 5\n\
                         down 5\n\
                         forward 3\n\
                         down 9\n\
                         forward 5\n\
                         down 3\n\
                         forward 7\n\
                         forward 5\n\
                         forward 9\n\
                         up 5\n\
                         down 4\n\
                         down 2\n\
                         forward 9\n\
                         down 3\n\
                         down 8\n\
                         down 9\n\
                         forward 2\n\
                         down 8\n\
                         up 6\n\
                         down 4\n\
                         down 2\n\
                         up 9\n\
                         forward 8\n\
                         forward 8\n\
                         down 8\n\
                         forward 4\n\
                         down 7\n\
                         forward 2\n\
                         up 7\n\
                         forward 7\n\
                         down 4\n\
                         forward 4\n\
                         down 3\n\
                         forward 9\n\
                         down 9\n\
                         forward 6\n\
                         down 5\n\
                         down 9\n\
                         up 5\n\
                         forward 7\n\
                         forward 2\n\
                         down 3\n\
                         down 7\n\
                         down 2\n\
                         forward 3\n\
                         down 4\n\
                         up 3\n\
                         down 1\n\
                         forward 9\n\
                         down 4\n\
                         down 8\n\
                         up 9\n\
                         forward 7\n\
                         down 8\n\
                         forward 9\n\
                         down 2\n\
                         up 2\n\
                         down 1\n\
                         down 1\n\
                         forward 6\n\
                         forward 2\n\
                         forward 3\n\
                         down 5\n\
                         down 1\n\
                         down 1\n\
                         up 4\n\
                         forward 8\n\
                         down 3\n\
                         down 1\n\
                         forward 9\n\
                         forward 7\n\
                         forward 2\n\
                         up 8\n\
                         up 6\n\
                         down 7\n\
                         down 6\n\
                         forward 3\n\
                         down 2\n\
                         down 9\n\
                         up 7\n\
                         forward 5\n\
                         up 9\n\
                         down 9\n\
                         down 4\n\
                         down 8\n\
                         down 5\n\
                         down 8\n\
                         down 8\n\
                         forward 6\n\
                         forward 1\n\
                         forward 4\n\
                         forward 7\n\
                         down 7\n\
                         down 6\n\
                         forward 4\n\
                         forward 7\n\
                         forward 6\n\
                         down 7\n\
                         forward 4\n\
                         forward 9\n\
                         up 3\n\
                         forward 9\n\
                         forward 5\n\
                         forward 1\n\
                         up 2\n\
                         down 1\n\
                         down 5\n\
                         forward 9\n\
                         up 4\n\
                         forward 6\n\
                         up 3\n\
                         up 6\n\
                         forward 8\n\
                         down 6\n\
                         forward 5\n\
                         down 3\n\
                         forward 2\n\
                         forward 7\n\
                         down 4\n\
                         up 8\n\
                         forward 6\n\
                         up 7\n\
                         up 9\n\
                         forward 3\n\
                         down 3\n\
                         down 7\n\
                         down 7\n\
                         down 1\n\
                         down 6\n\
                         down 9\n\
                         up 1\n\
                         forward 6\n\
                         forward 6\n\
                         down 3\n\
                         forward 7\n\
                         down 8\n\
                         forward 1\n\
                         down 7\n\
                         down 4\n\
                         down 3\n\
                         down 4\n\
                         down 4\n\
                         forward 7\n\
                         down 3\n\
                         forward 6\n\
                         up 9\n\
                         forward 3";
