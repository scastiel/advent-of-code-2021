use self::{
    board::Board,
    victories::{Victories, Victory},
};
use std::cell::RefCell;

mod board;
mod input;
mod victories;

fn mark_number_in_boards<'a>(
    boards: &'a [RefCell<Board<'a>>],
    number: u32,
) -> Option<&'a RefCell<Board<'a>>> {
    // println!("Number = {}", number);
    let mut winner = None;
    for board_cell in boards {
        let mut board = board_cell.borrow_mut();
        if !board.wins() {
            board.mark(number);
            // println!("Board = \n{:?}", board);
            if winner.is_none() && board.wins() {
                winner = Some(board_cell);
            }
        }
    }
    winner
}

fn play<'a>(numbers: &'a [u32], boards: &'a [RefCell<Board<'a>>]) -> Victories<'a> {
    let mut victories = Victories::default();
    for &number in numbers {
        if let Some(winning_board) = mark_number_in_boards(boards, number) {
            // println!("Winning board:\n{:?}", winning_board.borrow());
            if victories.first.is_none() {
                victories.first = Some(Victory::new(number, winning_board.clone()));
            }
            victories.last = Some(Victory::new(number, winning_board.clone()));
        }
    }
    victories
}

pub fn main() {
    let boards = input::exercise_boards();
    let numbers = input::exercise_numbers();

    let victories = play(numbers, &boards);
    println!("{:?}", victories);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let boards = input::example_boards();
        let numbers = input::example_numbers();
        let Victories { first, last } = play(numbers, &boards);

        let first = first.expect("Expected first victory");
        assert_eq!(first.number, 24);
        assert_eq!(first.sum(), 188);
        assert_eq!(first.product(), 4512);

        let last = last.expect("Expected last victory");
        assert_eq!(last.number, 13);
        assert_eq!(last.sum(), 148);
        assert_eq!(last.product(), 1924);
    }

    #[test]
    fn exercise() {
        let boards = input::exercise_boards();
        let numbers = input::exercise_numbers();
        let Victories { first, last } = play(numbers, &boards);

        let first = first.expect("Expected first victory");
        assert_eq!(first.number, 65);
        assert_eq!(first.sum(), 835);
        assert_eq!(first.product(), 54275);

        let last = last.expect("Expected last victory");
        assert_eq!(last.number, 51);
        assert_eq!(last.sum(), 258);
        assert_eq!(last.product(), 13158);
    }
}
