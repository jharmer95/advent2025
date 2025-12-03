//! Integration tests

use advent2025::inputs::get_input;
use advent2025::solutions::*;

#[test]
fn day01_part1() {
    let input = get_input("inputs/day_01.txt").expect("Could not parse path!");
    let mut solution = Day01::default();
    solution.parse_input(&input);
    let output = solution.part1();
    assert_eq!(output, "1071");
}

#[test]
fn day01_part2() {
    let input = get_input("inputs/day_01.txt").expect("Could not parse path!");
    let mut solution = Day01::default();
    solution.parse_input(&input);
    let output = solution.part2();
    assert_eq!(output, "6700");
}
