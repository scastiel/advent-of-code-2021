use std::fmt::Debug;

mod input;

#[derive(Debug)]
enum MatrixErr {
    OnlyNosInRow(usize),
    OnlyNosInCol(usize),
}

#[derive(PartialEq, Clone)]
enum State {
    Yes,
    No,
    Maybe,
}

#[derive(Clone)]
struct Matrix {
    size: usize,
    matrix: Vec<Vec<State>>,
    rows_labels: Vec<String>,
    cols_labels: Vec<String>,
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;
        for j in 0..self.size {
            write!(f, "{}", self.cols_labels[j])?;
        }
        writeln!(f)?;
        for i in 0..self.size {
            write!(f, "{}", self.rows_labels[i])?;
            for j in 0..self.size {
                let c = match self.matrix[i][j] {
                    State::Yes => 'Y',
                    State::Maybe => ' ',
                    State::No => 'x',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        write!(f, "Sure values: ")?;
        for (row_index, col_index) in self.sure_values() {
            write!(
                f,
                "{}<-{} ",
                self.rows_labels[row_index], self.cols_labels[col_index]
            )?;
        }
        writeln!(f)?;
        Ok(())
    }
}

impl Matrix {
    pub fn new(rows_labels: Vec<String>, cols_labels: Vec<String>) -> Self {
        let size = rows_labels.len();
        assert_eq!(cols_labels.len(), size);
        Self {
            size,
            rows_labels,
            cols_labels,
            matrix: (0..size)
                .map(|_| (0..size).map(|_| State::Maybe).collect())
                .collect(),
        }
    }

    pub fn eliminate(
        &mut self,
        row_index: usize,
        col_index: usize,
    ) -> Result<(bool, Vec<(usize, usize)>), MatrixErr> {
        if !matches!(self.matrix[row_index][col_index], State::No) {
            self.matrix[row_index][col_index] = State::No;
            self.update()
        } else {
            Ok((false, vec![]))
        }
    }

    fn count_in_row(&self, row_index: usize, matcher: &dyn Fn(&State) -> bool) -> usize {
        self.matrix[row_index]
            .iter()
            .filter(|&s| matcher(s))
            .count()
    }

    fn count_in_col(&self, col_index: usize, matcher: &dyn Fn(&State) -> bool) -> usize {
        self.matrix
            .iter()
            .filter(|&row| matcher(&row[col_index]))
            .count()
    }

    fn update_to_yes(&mut self, row_index: usize, col_index: usize) -> Result<bool, MatrixErr> {
        match &self.matrix[row_index][col_index] {
            State::Maybe => {
                self.matrix[row_index][col_index] = State::Yes;
                Ok(true)
            }
            State::Yes => Ok(false),
            State::No => Ok(false), //Err(MatrixErr::ChangingNoToYes(row_index, col_index)),
        }
    }

    fn update_to_no(&mut self, row_index: usize, col_index: usize) -> bool {
        if let State::Maybe = self.matrix[row_index][col_index] {
            self.matrix[row_index][col_index] = State::No;
            true
        } else {
            false
        }
    }

    fn update_to_no_in_row(&mut self, row_index: usize) -> bool {
        let mut modified = false;
        for col_index in 0..self.size {
            modified = self.update_to_no(row_index, col_index) || modified;
        }
        modified
    }

    fn update_to_yes_in_row(&mut self, row_index: usize) -> Result<Vec<(usize, usize)>, MatrixErr> {
        let mut modified = vec![];
        for col_index in 0..self.size {
            if self.update_to_yes(row_index, col_index)? {
                modified.push((row_index, col_index));
            }
        }
        Ok(modified)
    }

    fn update_to_no_in_col(&mut self, col_index: usize) -> bool {
        let mut modified = false;
        for row_index in 0..self.size {
            modified = self.update_to_no(row_index, col_index) || modified;
        }
        modified
    }

    fn update_to_yes_in_col(&mut self, col_index: usize) -> Result<Vec<(usize, usize)>, MatrixErr> {
        let mut modified = vec![];
        for row_index in 0..self.size {
            if self.update_to_yes(row_index, col_index)? {
                modified.push((row_index, col_index));
            }
        }
        Ok(modified)
    }

    fn update(&mut self) -> Result<(bool, Vec<(usize, usize)>), MatrixErr> {
        let mut modified = false;
        let mut updated_to_yes = vec![];

        loop {
            let mut modified_here = false;

            // Check rows
            for row_index in 0..self.size {
                if self.count_in_row(row_index, &|s| matches!(s, State::Yes)) == 1 {
                    modified_here = self.update_to_no_in_row(row_index) || modified_here;
                }

                if self.count_in_row(row_index, &|s| matches!(s, State::Maybe)) == 1 {
                    let mut updated = self.update_to_yes_in_row(row_index)?;
                    modified_here = modified_here || !updated.is_empty();
                    updated_to_yes.append(&mut updated);
                }

                if self.count_in_row(row_index, &|s| matches!(s, State::No)) == self.size {
                    return Err(MatrixErr::OnlyNosInRow(row_index));
                }
            }

            // Check columns
            for col_index in 0..self.size {
                if self.count_in_col(col_index, &|s| matches!(s, State::Yes)) == 1 {
                    modified_here = self.update_to_no_in_col(col_index) || modified_here;
                }

                if self.count_in_col(col_index, &|s| matches!(s, State::Maybe)) == 1 {
                    let mut updated = self.update_to_yes_in_col(col_index)?;
                    modified_here = modified_here || !updated.is_empty();
                    updated_to_yes.append(&mut updated);
                }

                if self.count_in_col(col_index, &|s| matches!(s, State::No)) == self.size {
                    return Err(MatrixErr::OnlyNosInCol(col_index));
                }
            }

            modified = modified || modified_here;
            if !modified_here {
                break;
            }
        }

        Ok((modified, updated_to_yes))
    }

    pub fn sure_values(&self) -> Vec<(usize, usize)> {
        (0..self.size)
            .flat_map(|row_index| {
                (0..self.size).filter_map(move |col_index| {
                    if matches!(self.matrix[row_index][col_index], State::Yes) {
                        Some((row_index, col_index))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn get_nos(&self) -> Vec<(usize, usize)> {
        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(col_index, state)| {
                        if matches!(state, State::No) {
                            Some((row_index, col_index))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }

    pub fn get_maybes_in_col(&self, col_index: usize) -> Vec<usize> {
        self.matrix
            .iter()
            .enumerate()
            .filter_map(|(row_index, row)| {
                if matches!(row[col_index], State::Maybe) {
                    Some(row_index)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_maybes(&self) -> Vec<(usize, usize)> {
        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(col_index, state)| {
                        if matches!(state, State::Maybe) {
                            Some((row_index, col_index))
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }
}

#[derive(Clone)]
struct Game {
    inputs: Vec<String>,
    matrix: Matrix,
    bar_matrices: Vec<Vec<Matrix>>,
    sure_values: Option<Vec<(usize, usize)>>,
}

impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:?}", self.matrix)?;
        Ok(())
    }
}

const DIGITS_BARS: &[(usize, &str)] = &[
    (0, "abcefg"),
    (1, "cf"),
    (2, "acdeg"),
    (3, "acdfg"),
    (4, "bcdf"),
    (5, "abdfg"),
    (6, "abdefg"),
    (7, "acf"),
    (8, "abcdefg"),
    (9, "abcdfg"),
];

fn chars_to_indices(chars: &str) -> Vec<usize> {
    chars
        .chars()
        .map(|ch| ch as usize - 'a' as usize)
        .collect::<Vec<_>>()
}

impl Game {
    pub fn new(inputs: &[String]) -> Self {
        let digit_matrix = Matrix::new(
            (0..=9).map(|c| format!("{}", c)).collect(),
            (0..=9).map(|c| format!("{}", c)).collect(),
        );
        let bar_matrices = (0..=9)
            .map(|_| {
                (0..=9)
                    .map(|_| {
                        Matrix::new(
                            ('a'..='g').map(|c| format!("{}", c)).collect(),
                            ('a'..='g').map(|c| format!("{}", c)).collect(),
                        )
                    })
                    .collect()
            })
            .collect();
        Self {
            inputs: inputs.iter().map(|s| s.to_string()).collect(),
            matrix: digit_matrix,
            bar_matrices,
            sure_values: None,
        }
    }

    fn possible_digits_for_len(&self, len: usize) -> Vec<usize> {
        DIGITS_BARS
            .iter()
            .filter_map(|&(digit, bars)| if bars.len() == len { Some(digit) } else { None })
            .collect()
    }

    pub fn add_digit(&mut self, col_index: usize) {
        let digit = &self.inputs[col_index];
        let possible = self.possible_digits_for_len(digit.len());
        for row_index in 0..10 {
            if !possible.contains(&row_index) {
                let (_, updated_to_yes) = self.matrix.eliminate(row_index, col_index).unwrap();
                self.post_update(row_index, col_index, &updated_to_yes)
                    .unwrap();
            }
        }
    }

    fn post_update(
        &mut self,
        row_index: usize,
        col_index: usize,
        updated_to_yes: &[(usize, usize)],
    ) -> Result<(), MatrixErr> {
        for maybe_row in self.matrix.get_maybes_in_col(col_index) {
            let bar_row_indices = chars_to_indices(DIGITS_BARS[maybe_row].1); // a b c e f g
            let bar_col_indices = chars_to_indices(&self.inputs[col_index]); // c b d g e f
            let bar_matrix = &mut self.bar_matrices[maybe_row][col_index];

            for r in 0..=6_usize {
                for c in 0..=6_usize {
                    let on_row = bar_row_indices.contains(&r);
                    let on_col = bar_col_indices.contains(&c);
                    if on_row && !on_col || !on_row && on_col {
                        let res = bar_matrix.eliminate(r, c);
                        if res.is_err() {
                            self.matrix.eliminate(row_index, col_index)?;
                        }
                    }
                }
            }
        }

        for &(yes_row, yes_col) in updated_to_yes {
            let bar_row_indices = chars_to_indices(DIGITS_BARS[yes_row].1); // c, f
            let bar_col_indices = chars_to_indices(&self.inputs[yes_col]); // b, e
            let yes_bar_matrix = &self.bar_matrices[yes_row][yes_col];
            let new_nos = yes_bar_matrix.get_nos();

            for row_index in 0..=9_usize {
                for col_index in 0..=9_usize {
                    let bar_matrix = &mut self.bar_matrices[row_index][col_index];
                    for r in 0..=6_usize {
                        for c in 0..=6_usize {
                            let on_row = bar_row_indices.contains(&r);
                            let on_col = bar_col_indices.contains(&c);
                            if on_row && !on_col || !on_row && on_col || new_nos.contains(&(r, c)) {
                                let res = bar_matrix.eliminate(r, c);
                                if res.is_err() {
                                    self.matrix.eliminate(row_index, col_index)?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn finish(&mut self) {
        let maybes = &self.matrix.get_maybes();
        if maybes.is_empty() {
            self.sure_values = Some(self.matrix.sure_values());
        } else {
            for col_index in 0..=9 {
                let row_indices: Vec<usize> = maybes
                    .iter()
                    .filter_map(|&(r, c)| if c == col_index { Some(r) } else { None })
                    .collect();
                if row_indices.len() >= 2 {
                    for &row_index in &row_indices {
                        let mut game_clone = self.clone();
                        let res = game_clone.set_yes(row_index, col_index);
                        if res.is_ok() {
                            let sure_values = game_clone.matrix.sure_values();
                            if sure_values.len() == 10 {
                                self.sure_values = Some(sure_values);
                            }
                        }
                    }
                }
            }
        }
    }

    fn set_yes(&mut self, row_index: usize, col_index: usize) -> Result<(), MatrixErr> {
        for other_row_index in 0..=9 {
            if other_row_index != row_index {
                let (_, updated_to_yes) = self.matrix.eliminate(other_row_index, col_index)?;
                self.post_update(row_index, col_index, &updated_to_yes)?;
            }
        }
        Ok(())
    }

    fn get_digit(&self, output: &str) -> Option<usize> {
        if let Some(sure_values) = self.sure_values.as_deref() {
            sure_values.iter().find_map(|&(to, from)| {
                if self.inputs[from] == output {
                    Some(to)
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    pub fn get_result(&self, outputs: &[String]) -> Option<u32> {
        let digit1 = self.get_digit(&outputs[0])?;
        let digit2 = self.get_digit(&outputs[1])?;
        let digit3 = self.get_digit(&outputs[2])?;
        let digit4 = self.get_digit(&outputs[3])?;
        Some(digit1 as u32 * 1000 + digit2 as u32 * 100 + digit3 as u32 * 10 + digit4 as u32)
    }
}

pub fn sort_chars(input: &&str) -> String {
    let mut chars = input.chars().collect::<Vec<_>>();
    chars.sort_unstable();
    chars.iter().collect::<String>()
}

pub fn main() {
    let notes = input::exercise();
    println!("Ex2: {}", find_number(notes))
}

fn find_number(notes: Vec<(Vec<&str>, Vec<&str>)>) -> u32 {
    let mut sum = 0;
    for (inputs, outputs) in notes {
        let inputs = inputs.iter().map(sort_chars).collect::<Vec<_>>();
        let outputs = outputs.iter().map(sort_chars).collect::<Vec<_>>();
        let mut game = Game::new(&inputs);
        for i in 0..=9 {
            game.add_digit(i);
        }
        game.finish();
        sum += game.get_result(&outputs).unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::{find_number, input};

    #[test]
    fn ex2_example() {
        let notes = input::example();
        assert_eq!(find_number(notes), 61229);
    }

    #[test]
    fn ex2_exercise() {
        let notes = input::exercise();
        assert_eq!(find_number(notes), 978171);
    }
}
