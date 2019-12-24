pub mod day1;
pub mod day2;

pub fn run() {
    println!("y2019:");
    println!("  day1:");
    println!("      solution #1: {}", day1::solve(day1::INPUT));
    println!("      solution #2: {}", day1::solve2(day1::INPUT));
    println!("  day2:");
    println!("      solution #1: {:?}", day2::solve(day2::INPUT));
    println!("      solution #2: {:?}", day2::solve2(day2::INPUT));
}