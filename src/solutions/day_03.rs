use crate::solutions::SolutionImpl;

#[derive(Clone, Debug, Default)]
pub struct Day03 {
    banks: Vec<Vec<u8>>,
}

impl SolutionImpl for Day03 {
    fn parse_input(&mut self, input: &[String]) {
        self.banks = input
            .iter()
            .map(|line| {
                let mut bank = Vec::<u8>::with_capacity(line.len());

                for c in line.chars() {
                    bank.push(u8::try_from(c.to_digit(10).unwrap()).unwrap());
                }

                bank
            })
            .collect();
    }

    fn part1(&self) -> String {
        let total = self.banks.iter().fold(0, |acc, bank| {
            let mut max: (u64, usize) = (0, 0);

            for (ind, v) in bank.iter().take(bank.len() - 1).enumerate() {
                if u64::from(*v) > max.0 {
                    max = (u64::from(*v), ind);
                }
            }

            let mut max2 = 0;

            for v in bank.iter().skip(max.1 + 1) {
                if u64::from(*v) > max2 {
                    max2 = u64::from(*v);
                }
            }

            acc + max.0 * 10 + max2
        });

        total.to_string()
    }

    fn part2(&self) -> String {
        let total = self.banks.iter().fold(0, |acc, bank| {
            let mut maxes = [0u64; 12];
            let mut indexes = [0; 13];

            for i in 0..12 {
                for (ind, v) in bank
                    .iter()
                    .take(bank.len() - (11 - i))
                    .skip(indexes[i])
                    .enumerate()
                {
                    if u64::from(*v) > maxes[i] {
                        maxes[i] = u64::from(*v);
                        indexes[i + 1] = ind + 1 + indexes[i];
                    }
                }
            }

            let mut multiplier = 100_000_000_000;

            let bank_sum = maxes.iter().fold(0, |acc, val| {
                let ret = acc + *val * multiplier;
                multiplier /= 10;
                ret
            });

            acc + bank_sum
        });

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "987654321111111".to_owned(),
            "811111111111119".to_owned(),
            "234234234234278".to_owned(),
            "818181911112111".to_owned(),
        ]
    }

    #[test]
    fn parse_input() {
        let mut solution = Day03::default();
        let test_input = get_test_input();
        solution.parse_input(&test_input);

        assert_eq!(solution.banks.len(), test_input.len());
        assert_eq!(
            solution.banks[0],
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]
        );
    }

    #[test]
    fn part1() {
        let mut solution = Day03::default();
        let test_input = get_test_input();
        solution.parse_input(&test_input);

        let result = solution.part1();
        assert_eq!(result, "357");
    }

    #[test]
    fn part2() {
        let mut solution = Day03::default();
        let test_input = get_test_input();
        solution.parse_input(&test_input);

        let result = solution.part2();
        assert_eq!(result, "3121910778619");
    }
}
