use super::board::Board;
use std::cell::RefCell;
use std::fmt::Debug;

pub struct Victory<'a> {
    pub number: u32,
    pub board: RefCell<Board<'a>>,
}

impl<'a> Victory<'a> {
    pub fn new(number: u32, board: RefCell<Board<'a>>) -> Victory<'a> {
        Victory { number, board }
    }

    pub fn sum(&self) -> u32 {
        self.board.borrow().sum_unmarked_numbers()
    }

    pub fn product(&self) -> u32 {
        self.number * self.sum()
    }
}

impl<'a> Debug for Victory<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "number = {}, sum = {}, product = {}",
            self.number,
            self.sum(),
            self.product()
        )?;
        Ok(())
    }
}

#[derive(Default)]
pub struct Victories<'a> {
    pub first: Option<Victory<'a>>,
    pub last: Option<Victory<'a>>,
}

impl<'a> Debug for Victories<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "First = {:?}, ", self.first)?;
        write!(f, "Last: {:?}", self.last)?;
        Ok(())
    }
}
