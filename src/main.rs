use std::env;

use advent2025::inputs::get_input;

#[allow(clippy::wildcard_imports)]
use advent2025::solutions::*;

fn help() {
    eprintln!("Usage: advent2025 <N>");
    std::process::exit(1);
}

fn generate_solutions() -> Vec<Solution> {
    vec![
        Solution::new(Box::new(std::cell::RefCell::new(Day01::default()))),
        Solution::new(Box::new(std::cell::RefCell::new(Day02::default()))),
        Solution::new(Box::new(std::cell::RefCell::new(Day03::default()))),
        Solution::new(Box::new(std::cell::RefCell::new(Day04::default()))),
        // Add more days here as they are implemented
    ]
}

fn run_solution(solution: &Solution, day_num: usize) {
    println!("Day {day_num}:");

    let input_file = format!("inputs/day_{day_num:02}.txt");
    let input = get_input(&input_file).expect("Failed to read input file");

    solution.parse_input(&input);

    let p1_result = solution.part1();
    println!("Part 1: {p1_result}");

    let p2_result = solution.part2();
    println!("Part 2: {p2_result}");
}

fn main() {
    let mut args = env::args();

    if args.len() > 2 {
        help();
    }

    let solutions = generate_solutions();

    if args.len() == 1 {
        for (i, solution) in solutions.iter().enumerate() {
            run_solution(solution, i + 1);
            println!();
        }
    } else {
        let day_num = args.nth(1).unwrap().parse::<usize>().unwrap_or(0);

        if day_num == 0 || day_num > solutions.len() {
            help();
        }

        run_solution(&solutions[day_num - 1], day_num);
    }
}
