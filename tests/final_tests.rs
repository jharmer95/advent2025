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

#[test]
fn day02_part1() {
    let input = get_input("inputs/day_02.txt").expect("Could not parse path!");
    let mut solution = Day02::default();
    solution.parse_input(&input);
    let output = solution.part1();
    assert_eq!(output, "18595663903");
}

#[test]
fn day02_part2() {
    let input = get_input("inputs/day_02.txt").expect("Could not parse path!");
    let mut solution = Day02::default();
    solution.parse_input(&input);
    let output = solution.part2();
    assert_eq!(output, "19058204438");
}

#[test]
fn day03_part1() {
    let input = get_input("inputs/day_03.txt").expect("Could not parse path!");
    let mut solution = Day03::default();
    solution.parse_input(&input);
    let output = solution.part1();
    assert_eq!(output, "17100");
}

#[test]
fn day03_part2() {
    let input = get_input("inputs/day_03.txt").expect("Could not parse path!");
    let mut solution = Day03::default();
    solution.parse_input(&input);
    let output = solution.part2();
    assert_eq!(output, "170418192256861");
}
