use crate::solutions::SolutionImpl;

#[derive(Clone, Debug, Default)]
pub struct Day04 {
    grid: Vec<Vec<Space>>,
}

impl Day04 {
    fn adjacent_count(grid: &[Vec<Space>], x: usize, y: usize) -> Option<usize> {
        if y >= grid.len() || x >= grid[0].len() || grid[y][x] == Space::Empty {
            return None;
        }

        let y_high;
        let y_low;
        let x_high;
        let x_low;

        if y == 0 {
            y_high = 1;
            y_low = 0;
        } else if y == grid.len() - 1 {
            y_high = grid[0].len() - 1;
            y_low = y_high - 1;
        } else {
            y_high = y + 1;
            y_low = y - 1;
        }

        if x == 0 {
            x_high = 1;
            x_low = 0;
        } else if x == grid[0].len() - 1 {
            x_high = grid.len() - 1;
            x_low = x_high - 1;
        } else {
            x_high = x + 1;
            x_low = x - 1;
        }

        let mut roll_count = 0;

        for (i, row) in grid.iter().enumerate().take(y_high + 1).skip(y_low) {
            for (j, space) in row.iter().enumerate().take(x_high + 1).skip(x_low) {
                if i == y && j == x {
                    continue;
                }

                if *space == Space::Roll {
                    roll_count += 1;
                }
            }
        }

        Some(roll_count)
    }
}

impl SolutionImpl for Day04 {
    fn parse_input(&mut self, input: &[String]) {
        self.grid = Vec::with_capacity(input.len());

        for line in input {
            let mut grid_line = Vec::with_capacity(line.len());

            for ch in line.chars() {
                grid_line.push(Space::from(ch));
            }

            self.grid.push(grid_line);
        }
    }

    fn part1(&self) -> String {
        let mut access_count = 0;

        for (y, row) in self.grid.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if let Some(count) = Self::adjacent_count(&self.grid, x, y)
                    && count < 4
                {
                    access_count += 1;
                }
            }
        }

        access_count.to_string()
    }

    fn part2(&self) -> String {
        let mut removed_count = 0;

        let mut grid = self.grid.clone();

        loop {
            let mut removed = false;

            for (y, row) in self.grid.iter().enumerate() {
                for (x, _) in row.iter().enumerate() {
                    if let Some(count) = Self::adjacent_count(&grid, x, y)
                        && count < 4
                    {
                        removed_count += 1;
                        removed = true;
                        grid[y][x] = Space::Empty;
                    }
                }
            }

            if !removed {
                break;
            }
        }

        removed_count.to_string()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Space {
    Empty,
    Roll,
}

impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '@' => Self::Roll,
            _ => Self::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "..@@.@@@@.".to_owned(),
            "@@@.@.@.@@".to_owned(),
            "@@@@@.@.@@".to_owned(),
            "@.@@@@..@.".to_owned(),
            "@@.@@@@.@@".to_owned(),
            ".@@@@@@@.@".to_owned(),
            ".@.@.@.@@@".to_owned(),
            "@.@@@.@@@@".to_owned(),
            ".@@@@@@@@.".to_owned(),
            "@.@.@@@.@.".to_owned(),
        ]
    }

    #[test]
    fn parse_input() {
        let mut solution = Day04::default();
        let test_input = get_test_input();
        solution.parse_input(&test_input);

        assert_eq!(solution.grid.len(), test_input.len());
        assert_eq!(solution.grid[0].len(), test_input[0].len());

        assert_eq!(solution.grid[0][0], Space::Empty);
        assert_eq!(solution.grid[0][1], Space::Empty);
        assert_eq!(solution.grid[0][2], Space::Roll);
        assert_eq!(solution.grid[0][3], Space::Roll);
        assert_eq!(solution.grid[0][4], Space::Empty);
    }

    #[test]
    fn part1() {
        let mut solution = Day04::default();
        let test_input = get_test_input();
        solution.parse_input(&test_input);

        let result = solution.part1();
        assert_eq!(result, "13");
    }

    #[test]
    fn part2() {
        let mut solution = Day04::default();
        let test_input = get_test_input();
        solution.parse_input(&test_input);

        let result = solution.part2();
        assert_eq!(result, "43");
    }
}
