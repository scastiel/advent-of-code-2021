use std::collections::{HashSet, VecDeque};

mod input;

#[derive(Debug)]
pub enum Fold {
    AlongX(i32),
    AlongY(i32),
}

#[derive(Debug)]
pub struct Input {
    pub dots: HashSet<(i32, i32)>,
    pub folds: VecDeque<Fold>,
}

impl Input {
    pub fn count_dots(&self) -> usize {
        self.dots.len()
    }

    fn fold_along_x(&mut self, fold_x: i32) {
        self.dots = self
            .dots
            .iter()
            .flat_map(|&(x, y)| vec![(2 * fold_x - x, y), (x, y)])
            .filter(|&(x, _)| x >= 0 && x < fold_x)
            .collect()
    }

    fn fold_along_y(&mut self, fold_y: i32) {
        self.dots = self
            .dots
            .iter()
            .flat_map(|&(x, y)| vec![(x, 2 * fold_y - y), (x, y)])
            .filter(|&(_, y)| y >= 0 && y < fold_y)
            .collect()
    }

    fn fold(&mut self, fold: Fold) {
        match fold {
            Fold::AlongX(fold_x) => self.fold_along_x(fold_x),
            Fold::AlongY(fold_y) => self.fold_along_y(fold_y),
        }
    }

    pub fn fold_once(&mut self) {
        let fold = self.folds.pop_front().expect("No more fold instruction.");
        self.fold(fold);
    }

    pub fn fold_all(&mut self) {
        while let Some(fold) = self.folds.pop_front() {
            self.fold(fold);
        }
    }

    pub fn size(&self) -> (i32, i32) {
        let mut max_x = 0;
        let mut max_y = 0;
        for &(x, y) in &self.dots {
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
        (max_x + 1, max_y + 1)
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (size_x, size_y) = self.size();
        for y in 0..size_y {
            for x in 0..size_x {
                write!(
                    f,
                    "{}",
                    if self.dots.contains(&(x, y)) {
                        '#'
                    } else {
                        ' '
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn main() {
    let mut input = input::exercise();
    println!("Number of dots before fold: {}", input.count_dots());

    input.fold_once();
    println!("Number of dots before one fold: {}", input.count_dots());

    input.fold_all();
    println!("\n{}", input);
    println!("Number of dots after all folds: {}", input.count_dots());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_example() {
        let mut input = input::example();
        input.fold_once();
        assert_eq!(input.count_dots(), 17);
    }

    #[test]
    fn ex1_exercise() {
        let mut input = input::exercise();
        input.fold_once();
        assert_eq!(input.count_dots(), 765);
    }

    #[test]
    fn ex2_example() {
        let mut input = input::example();
        input.fold_all();
        let result = format!("{}", input);
        assert_eq!(
            result,
            r"
#####
#   #
#   #
#   #
#####
"
            .trim_start()
        );
    }

    #[test]
    fn ex2_exercise() {
        let mut input = input::exercise();
        input.fold_all();
        let result = format!("{}", input);
        assert_eq!(
            result,
            r"
###  #### #  # #### #    ###   ##  #  #
#  #    # # #     # #    #  # #  # #  #
#  #   #  ##     #  #    #  # #    ####
###   #   # #   #   #    ###  # ## #  #
# #  #    # #  #    #    #    #  # #  #
#  # #### #  # #### #### #     ### #  #
"
            .trim_start()
        );
    }
}
