use crate::solutions::SolutionImpl;

#[derive(Clone, Debug, Default)]
pub struct Day01 {
    commands: Vec<Command>,
}

impl SolutionImpl for Day01 {
    fn parse_input(&mut self, input: &[String]) {
        self.commands = input
            .iter()
            .map(|line| {
                let direction = match line.chars().next() {
                    Some('L') => Direction::Left,
                    Some('R') => Direction::Right,
                    _ => panic!("Invalid direction in input"),
                };

                let distance_str = &line[1..];
                let distance: i32 = distance_str.parse().expect("Invalid distance in input");

                Command {
                    direction,
                    distance,
                }
            })
            .collect();
    }

    fn part1(&self) -> String {
        let mut dial_position = 50;

        let zero_count = self
            .commands
            .iter()
            .filter(|command| {
                command.apply(&mut dial_position);
                dial_position == 0
            })
            .count();

        zero_count.to_string()
    }

    fn part2(&self) -> String {
        let mut dial_position = 50;

        let zero_count: i32 = self
            .commands
            .iter()
            .map(|command| command.apply(&mut dial_position))
            .sum();

        zero_count.to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Command {
    direction: Direction,
    distance: i32,
}

impl Command {
    const fn apply(self, position: &mut i32) -> i32 {
        if self.distance == 0 {
            return 0;
        }

        let start_at_zero = *position == 0;

        match self.direction {
            Direction::Left => {
                *position -= self.distance;
            }
            Direction::Right => {
                *position += self.distance;
            }
        }

        let cross_zero_count = if *position > 0 {
            *position / 100
        } else {
            *position / -100 + if start_at_zero { 0 } else { 1 }
        };

        *position %= 100;

        if *position < 0 {
            *position += 100;
        }

        cross_zero_count
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "L68".to_owned(),
            "L30".to_owned(),
            "R48".to_owned(),
            "L5".to_owned(),
            "R60".to_owned(),
            "L55".to_owned(),
            "L1".to_owned(),
            "L99".to_owned(),
            "R14".to_owned(),
            "L82".to_owned(),
        ]
    }

    #[test]
    fn parse_input() {
        let mut day01 = Day01::default();
        let test_input = get_test_input();
        day01.parse_input(&test_input);

        assert_eq!(day01.commands.len(), 10);
        assert_eq!(
            day01.commands[0],
            Command {
                direction: Direction::Left,
                distance: 68
            }
        );
        assert_eq!(
            day01.commands[1],
            Command {
                direction: Direction::Left,
                distance: 30
            }
        );
        assert_eq!(
            day01.commands[2],
            Command {
                direction: Direction::Right,
                distance: 48
            }
        );
        assert_eq!(
            day01.commands[3],
            Command {
                direction: Direction::Left,
                distance: 5
            }
        );
        assert_eq!(
            day01.commands[4],
            Command {
                direction: Direction::Right,
                distance: 60
            }
        );
        assert_eq!(
            day01.commands[5],
            Command {
                direction: Direction::Left,
                distance: 55
            }
        );
        assert_eq!(
            day01.commands[6],
            Command {
                direction: Direction::Left,
                distance: 1
            }
        );
        assert_eq!(
            day01.commands[7],
            Command {
                direction: Direction::Left,
                distance: 99
            }
        );
        assert_eq!(
            day01.commands[8],
            Command {
                direction: Direction::Right,
                distance: 14
            }
        );
        assert_eq!(
            day01.commands[9],
            Command {
                direction: Direction::Left,
                distance: 82
            }
        );
    }

    #[test]
    fn apply_command_left_moves_dial_position_back() {
        let mut dial_position = 50;

        Command {
            direction: Direction::Left,
            distance: 30,
        }
        .apply(&mut dial_position);
        assert_eq!(dial_position, 20);
    }

    #[test]
    fn apply_command_right_moves_dial_position_forward() {
        let mut dial_position = 50;

        Command {
            direction: Direction::Right,
            distance: 30,
        }
        .apply(&mut dial_position);
        assert_eq!(dial_position, 80);
    }

    #[test]
    fn apply_command_wraps_around_at_0() {
        let mut dial_position = 50;

        Command {
            direction: Direction::Left,
            distance: 60,
        }
        .apply(&mut dial_position);
        assert_eq!(dial_position, 90);

        Command {
            direction: Direction::Left,
            distance: 210,
        }
        .apply(&mut dial_position);
        assert_eq!(dial_position, 80);
    }

    #[test]
    fn apply_command_wraps_around_at_100() {
        let mut dial_position = 50;

        Command {
            direction: Direction::Right,
            distance: 60,
        }
        .apply(&mut dial_position);
        assert_eq!(dial_position, 10);

        Command {
            direction: Direction::Right,
            distance: 200,
        }
        .apply(&mut dial_position);
        assert_eq!(dial_position, 10);
    }

    #[test]
    fn part1() {
        let mut day01 = Day01::default();
        let test_input = get_test_input();
        day01.parse_input(&test_input);

        let result = day01.part1();
        assert_eq!(result, "3");
    }

    #[test]
    fn part2() {
        let mut day01 = Day01::default();
        let test_input = get_test_input();
        day01.parse_input(&test_input);

        let result = day01.part2();
        assert_eq!(result, "6");
    }
}
