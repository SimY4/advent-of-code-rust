fn position(cursor: i32, ch: char) -> i32 {
    match ch {
        '(' => cursor + 1,
        ')' => cursor - 1,
        _ => unreachable!(),
    }
}

pub fn solve(input: &str) -> i32 {
    input.chars().fold(0, position)
}

pub fn solve2(input: &str) -> Option<usize> {
    input
        .chars()
        .scan(0, |acc, ch| {
            *acc = position(*acc, ch);
            Some(*acc)
        })
        .position(|pos| pos < 0)
        .and_then(|i| Some(i + 1))
}

pub const INPUT: &str = "((((()(()(((((((()))(((()((((()())(())()(((()((((((()((()(()(((()(()((())))()((()()())))))))))()((((((())((()))(((((()(((((((((()()))((()(())()((())((()(()))((()))()))()(((((()(((()()))()())((()((((())()())()((((())()(()(()(((()(())(()(())(((((((())()()(((())(()(()(()(())))(()((((())((()))(((()(()()(((((()()(()(((()(((((())()))()((()(()))()((()((((())((((())(()(((())()()(()()()()()(())((((())((())(()()))()((((())))((((()())()((((())((()())((())(())(((((()((((()(((()((((())(()(((()()))()))((((((()((())()())))(((()(()))(()()(()(((()(()))((()()()())((()()()(((())())()())())())((()))(()(()))(((((()(()(())((()(())(())()((((()())()))((((())(())((())())((((()(((())(())((()()((((()((((((()(())()()(()(()()((((()))(())()())()))(())))(())))())()()(())(()))()((()(()(())()()))(()())))))(()))(()()))(())(((((()(()(()()((())()())))))((())())((())(()(())((()))(())(((()((((((((()()()(()))()()(((()))()((()()(())(())())()(()(())))(((((()(())(())(()))))())()))(()))()(()(((((((()((((())))())())())())()((((((((((((((()()((((((()()()())())()())())())(())(())))())((()())((()(()))))))()))))))))))))))))())((())((())()()))))))(((()((()(()()))((())(()()))()()())))(())))))))(()(((())))())()())))()()(())()))()(()))())((()()))))(()))))()))(()()(())))))))()(((()))))()(()))(())())))))()))((()))((()))())(())))))))))((((())()))()))()))())(())()()(())))())))(()())()))((()()(())))(())((((((()(())((()(((()(()()(())))()))))))()))()(()((()))()(()))(()(((())((((())())(())(()))))))))())))))))())())))))())))))()()(((())()(()))))))))())))))(())()()()))()))()))(()(())()()())())))))))())()(()(()))))()()()))))())(()))))()()))))()())))))(((())()()))(()))))))))))()()))))()()()))))(()())())()()())()(()))))()(()))(())))))))(((((())(())())()()))()()))(())))))()(()))))(())(()()))()())()))()))()))()))))())()()))())())))(()))(()))))))())()(((())()))))))))()))()())))())))())))()))))))))))()()))(()()))))))(())()(()))))())(()))))(()))))(()())))))())())()()))))())()))))))))(()))))()))))))()(()())))))))()))())))())))())))())))))))())(()()))))))(()())())))()())()))))))))))))))())))()(())))()))())()()(())(()()))(())))())()())(()(()(()))))())))))))))))())(()))()))()))))(())()())()())))))))))))()()))))))))))))())())))))(()())))))))))))())(())))()))))))))())())(()))()))(())))()))()()(())()))))))()((((())()))())())))))()))()))))((()())()))))())))(())))))))))))))))))()))))()()())()))()()))))())()))((()())))())))(()))(()())))))))()))()))))(())))))))(())))))())()()(()))())()))()()))))())()()))))())()))())))))))(()))))()())()))))))))(()))())))(()))()))))(())()))())())(())())())))))))((((())))))()))()))()())()(())))()))()))()())(()())()()(()())()))))())())))))(()))()))))())(()()(())))))(())()()((())())))))(())(())))))))())))))))))()(())))))))()())())())()(()))))))))(()))))))))())()()))()(()))))))()))))))())))))))(())))()()(())()())))))(((())))()((())()))())))(()()))())(())())))()(((()())))))()(()()())))()()(()()(()()))())()(()()()))())()()))()())(()))))())))))())))(())()()))))(()))))(())(()))(())))))()()))()))))())()))()()(())())))((()))())()))))))()()))))((()(()))))()()))))))())))))())(()((()())))))))))))()())())))()))(()))))))(()))(())()())))(()))))))))())()()()()))))(()())))))))((())))()))(()))(())(())()())()))))))))(())))())))(()))()()))(()()))(()))())))()(())))())((()((()(())))((())))()))))((((())())()())))(())))()))))))())(()()((())))())()(()())))))(()())()))())))))))((())())))))))(()(()))())()()(()()(((()(((()())))))()))))))()(())(()()((()()(())()()))())()())()))()())())())))))))(((())))))))()()))))))(((())()))(()()))(()()))))(()(()()((((())()())((()()))))(()(())))))()((()()()())()()((()((()()))(()))(((()()()))(((())))()(((())()))))))((()(())())))(()())(((((()(()))(()((()))(()())()))))(()(()))()(()))(())(((())(()()))))()()))(((()))))(()()()()))())))((()()()(())()))()))))()()))()))))))((((((()()()))))())((()()(((()))))(()(())(()()())())())))()(((()()))(())((())))(()))(()()()())((())())())(()))))()))()((()(())()(()()(())(()))(())()))(())(()))))(())(())())(()()(()((()()((())))((()))()((())))(((()()()()((((()))(()()))()()()(((())((())())(()()(()()()))()((())(())()))())(((()()(())))()((()()())()())(()(())())(((())(())())((())(())()(((()()))(())))((())(()())())(())((()()()((((((())))((()(((((())()))()))(())(()()))()))(())()()))(())((()()())()()(()))())()((())))()((()()())((((()())((())())())((()((()))()))((())((()()(()((()()(((())(()()))))((()((())()(((())(()((())())((())(()((((((())())()(()())()(())(((())((((((()(())(()((()()()((()()(()()()())))()()(((((()()))()((((((()))()(()(()(()(((()())((()))())()((()))(())))()))()()))())()()))())((((())(()(()))(((((((())(((()(((((()(((()()((((())(((())())))(()()()(()(()))()))((((((()))((()(((()(())((()((((()((((((())(((((())))(((()(()))))(((()(((())()((())(()((()))(((()()(((())((((()(()(((((()))(((()(((((((()(()()()(()(()(()()())(())(((((()(())())()())(()(()(()))()(()()()())(()()(()((()))()((())())()(()))((())(()))()(()))()(((()(()(()((((((()()()()())()(((((()()(((()()()((()(((((()))((((((((()()()(((((()))))))(()()()(())(()))(()()))))(())()))(((((()(((((()()(()(()())(((()))((((()((()(()(()((()(()((())))()(((()((()))((()))(((((((((()((()((()(())))()((((()((()()))((())(((()(((((()()(()(()()((()(()()()(((((((())())()())))))((((()()(()))()))(()((())()(()(((((((((()()(((()(()())(()((()())((())())((((()(((()(((()((((()((()((((()(()((((((())((((((((((((()()(()()((((((((((((((()((()()))()((((((((((((())((((()(()())((()(()(()))()(((((()()(((()()))()())(())((()(((((()((())(((((()((()(((((()))()()((((())()((((())(((((((((()(())(()(())))())(()((())(((())(())(())())(()(()(())()()((()((())()(((()(((((()(())))()(((()((())))((()()()(((()(((()((()(()(())(()((()())(()(()(((()(((((((((())(()((((()()))(()((((()()()()(((()((((((((()(()()((((((()(()()(()((()((((((((((()()(((((((()())(())))(((()()))(((((()((()()())(()()((((())((()((((()))))(())((()(()()(((()(()(((()((((()(((((()))())())(()((())()))(((()())((())((())((((()((()((((((())(()((((()()))((((((())()(()))((()(((())((((((((((()()(((((()(((((()((()()()((((())))(()))()((()(())()()((()((((((((((()((())(())(((((()(()(()()))((((()((((()()((()(((()(((((((((()(()((()((()))((((((()(((())()()((()(((((((()())))()()(()((()((()()(((()(()()()()((((()((())((((()(((((((((()(((()()(((()(()(((()(((()((())()(()((()(()(()(()))()(((()))(()((((()((())((((())((((((())(()))(()((((())((()(()((((((((()()((((((()(()(()()()(())((()((()()(((()(((((((()()((()(((((((()))(((((()(((()(()()()(()(((()((()()((())(()(((((((((()(()((()((((((()()((())()))(((((()((())()())()(((((((((((()))((((()()()()())(()()(()(()()))()))(()))(()(((()()))())(()(()))()()((())(()())()())()(()))()))(()()(()((((((())((()(((((((((((()(())()((()(()((()((()(()((()((((((((((()()())((())()(())))((())()())()(((((()(()())((((()((()(())(()))(((())()((()))(((((())(()))()()(()))(((())((((()((((()(())))(((((((()))))())()())(())((())()(()()((()(()))()(()()(()()((()())((())((()()))((((()))()()))(()()(())()()(((((()(())((()((((()))()))(()())())(((()()(()()))(())))))(()))((())(((((()((((()))()((((()))()((())(((())))(((()())))((()(()()((";
