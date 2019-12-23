pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub fn run() {
    println!("y2015:");
    println!("  day1:");
    println!("      solution #1: {}", day1::solve(day1::INPUT));
    println!("      solution #2: {}", day1::solve2(day1::INPUT));
    println!("  day2:");
    println!("      solution #1: {}", day2::solve(day2::INPUT));
    println!("      solution #2: {}", day2::solve2(day2::INPUT));
    println!("  day3:");
    println!("      solution #1: {}", day3::solve(day3::INPUT));
    println!("      solution #2: {}", day3::solve2(day3::INPUT));
    println!("  day4:");
    println!("      solution #1: {:?}", day4::solve(day4::INPUT));
    println!("      solution #2: {:?}", day4::solve2(day4::INPUT));
}