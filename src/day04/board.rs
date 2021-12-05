use std::fmt::Debug;

#[derive(Clone)]
pub struct Board<'a> {
    values: &'a [&'a [u32; 5]; 5],
    marked: Vec<(usize, usize)>,
}

impl<'a> Board<'a> {
    pub fn new(values: &'a [&'a [u32; 5]; 5]) -> Board<'a> {
        Board {
            values,
            marked: vec![],
        }
    }

    pub fn mark(&mut self, num: u32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.values[i][j] == num {
                    self.marked.push((i, j));
                }
            }
        }
    }

    pub fn wins(&self) -> bool {
        self.wins_rows() || self.wins_columns()
    }

    pub fn sum_unmarked_numbers(&self) -> u32 {
        self.unmarked_numbers().iter().sum()
    }

    fn wins_rows(&self) -> bool {
        (0..5)
            .into_iter()
            .any(|i| (0..5).into_iter().all(|j| self.is_marked((i, j))))
    }

    fn wins_columns(&self) -> bool {
        (0..5)
            .into_iter()
            .any(|j| (0..5).into_iter().all(|i| self.is_marked((i, j))))
    }

    fn is_marked(&self, (i, j): (usize, usize)) -> bool {
        self.marked.contains(&(i, j))
    }

    fn unmarked_numbers(&self) -> Vec<u32> {
        self.values
            .iter()
            .enumerate()
            .flat_map(|(i, &row)| {
                row.iter()
                    .enumerate()
                    .filter(move |&(j, _)| !self.is_marked((i, j)))
                    .map(|(_, &v)| v)
            })
            .collect()
    }
}

impl<'a> Debug for Board<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..5 {
            for j in 0..5 {
                let is_marked = self.marked.contains(&(i, j));
                if is_marked {
                    write!(f, "[{:>2}] ", self.values[i][j])?;
                } else {
                    write!(f, " {:>2}  ", self.values[i][j])?;
                };
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
