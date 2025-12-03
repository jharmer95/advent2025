use std::cell::RefCell;

#[derive(Debug)]
pub struct Solution {
    pimpl: Box<RefCell<dyn SolutionImpl>>,
}

impl Solution {
    #[must_use]
    pub const fn new(pimpl: Box<RefCell<dyn SolutionImpl>>) -> Self {
        Self { pimpl }
    }

    pub fn parse_input(&self, input: &[String]) {
        self.pimpl.borrow_mut().parse_input(input);
    }

    #[must_use]
    pub fn part1(&self) -> String {
        self.pimpl.borrow().part1()
    }

    #[must_use]
    pub fn part2(&self) -> String {
        self.pimpl.borrow().part2()
    }
}

pub trait SolutionImpl
where
    Self: std::fmt::Debug,
{
    fn parse_input(&mut self, input: &[String]);
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
