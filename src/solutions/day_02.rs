use crate::solutions::SolutionImpl;

#[derive(Clone, Debug, Default)]
pub struct Day02 {
    ranges: Vec<Range>,
}

impl SolutionImpl for Day02 {
    fn parse_input(&mut self, input: &[String]) {
        let line = if input.len() > 1 {
            // Join lines into a single string
            &input[0..].join("")
        } else {
            &input[0]
        };

        self.ranges = line
            .split(',')
            .map(|block| {
                let (a, b) = block.split_once('-').expect("No range delimiter");
                let start: u64 = a.parse().expect("Invalid start of range");
                let end: u64 = b.parse().expect("Invalid end of range");

                Range { start, end }
            })
            .collect();
    }

    fn part1(&self) -> String {
        let is_valid_id = |id: u64| {
            let digits: Vec<u32> = id
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();

            // Split digits into two even halves
            let mid = digits.len() / 2;
            let left_half = &digits[..mid];
            let right_half = &digits[mid..];

            left_half != right_half
        };

        let invalid_id_sum: u64 = self
            .ranges
            .iter()
            .flat_map(|range| range.start..=range.end)
            .filter(|&id| !is_valid_id(id))
            .sum();

        invalid_id_sum.to_string()
    }

    fn part2(&self) -> String {
        let is_valid_id = |id: u64| {
            let digits: Vec<u32> = id
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();

            for chunk_size in 1..=(digits.len() / 2) {
                let chunks = digits.chunks(chunk_size);
                let first_chunk = chunks.clone().next().unwrap();

                if chunks.into_iter().all(|c| c == first_chunk) {
                    return false;
                }
            }

            true
        };

        let invalid_id_sum: u64 = self
            .ranges
            .iter()
            .flat_map(|range| range.start..=range.end)
            .filter(|&id| !is_valid_id(id))
            .sum();

        invalid_id_sum.to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_owned(),
        ]
    }

    #[test]
    fn parse_input() {
        let mut solution = Day02::default();
        let test_input = get_test_input();
        solution.parse_input(&test_input);

        assert_eq!(solution.ranges.len(), 11);

        assert_eq!(solution.ranges[0], Range { start: 11, end: 22 });
        assert_eq!(
            solution.ranges[1],
            Range {
                start: 95,
                end: 115
            }
        );
        assert_eq!(
            solution.ranges[2],
            Range {
                start: 998,
                end: 1_012
            }
        );
        assert_eq!(
            solution.ranges[10],
            Range {
                start: 2_121_212_118,
                end: 2_121_212_124
            }
        );
    }

    #[test]
    fn part1() {
        let mut solution = Day02::default();
        let test_input = get_test_input();
        solution.parse_input(&test_input);

        let result = solution.part1();
        assert_eq!(result, "1227775554");
    }

    #[test]
    fn part2() {
        let mut solution = Day02::default();
        let test_input = get_test_input();
        solution.parse_input(&test_input);

        let result = solution.part2();
        assert_eq!(result, "4174379265");
    }
}
