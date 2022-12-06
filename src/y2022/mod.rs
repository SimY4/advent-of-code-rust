pub mod day1;
pub mod day6;

pub fn run() {
    println!("y2022:");
    println!("  day1:");
    println!("    ├ solution #1: {:?}", day1::solve(day1::INPUT));
    println!("    └ solution #2: {}", day1::solve2(day1::INPUT));
    println!("  day6:");
    println!("    ├ solution #1: {:?}", day6::solve(day6::INPUT));
    println!("    └ solution #2: {:?}", day6::solve2(day6::INPUT));
}
