use std::fmt::Display;

mod input;

pub struct Matrix {
    matrix: Vec<Vec<u8>>,
}

const NEIGHBOUR_DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Matrix {
    pub fn new(input: &str) -> Matrix {
        let matrix = input
            .lines()
            .map(|line| line.trim().chars().map(|c| c as u8 - b'0').collect())
            .collect();
        Matrix { matrix }
    }

    fn width(&self) -> usize {
        self.matrix[0].len()
    }

    fn height(&self) -> usize {
        self.matrix.len()
    }

    fn neighbours(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        NEIGHBOUR_DELTAS
            .into_iter()
            .filter_map(|(di, dj)| {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni >= 0 && ni < self.height() as i32 && nj >= 0 && nj < self.width() as i32 {
                    Some((ni as usize, nj as usize))
                } else {
                    None
                }
            })
            .collect()
    }

    fn handle(
        &mut self,
        ni: usize,
        nj: usize,
        flashed: &[(usize, usize)],
        todo: &mut Vec<(usize, usize)>,
    ) {
        self.matrix[ni][nj] += 1;
        if self.matrix[ni][nj] > 9 && !todo.contains(&(ni, nj)) && !flashed.contains(&(ni, nj)) {
            todo.push((ni, nj));
        }
    }

    fn tick(&mut self) -> usize {
        let mut todo: Vec<(usize, usize)> = vec![];
        let mut flashed: Vec<(usize, usize)> = vec![];

        // Increment all cells by 1
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                self.handle(i, j, &flashed, &mut todo);
            }
        }

        // Handle cells to flash (recursively)
        while let Some((i, j)) = todo.pop() {
            flashed.push((i, j));
            for (ni, nj) in self.neighbours(i, j) {
                self.handle(ni, nj, &flashed, &mut todo);
            }
        }

        // Reset all cells that flashed to 0
        for &(i, j) in &flashed {
            self.matrix[i][j] = 0;
        }

        flashed.len()
    }

    fn tick_n_times(&mut self, n: usize) -> usize {
        (0..n).map(|_| self.tick()).sum()
    }

    fn tick_until_all_flash(&mut self) -> usize {
        for step in 1.. {
            let flashed = self.tick();
            if flashed == self.width() * self.height() {
                return step;
            }
        }
        panic!()
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height() {
            for j in 0..self.width() {
                write!(f, "{}", self.matrix[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn main() {
    let mut matrix = Matrix::new(input::exercise());
    let flashed = matrix.tick_n_times(100);
    println!("{}\nEx1, flashed = {:?}\n", matrix, flashed);

    let mut matrix = Matrix::new(input::exercise());
    let step = matrix.tick_until_all_flash();
    println!("{}\nEx2, step = {:?}\n", matrix, step);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_example() {
        let mut matrix = Matrix::new(input::example());
        assert_eq!(matrix.tick_n_times(100), 1656);
    }

    #[test]
    fn ex1_exercise() {
        let mut matrix = Matrix::new(input::exercise());
        assert_eq!(matrix.tick_n_times(100), 1601);
    }

    #[test]
    fn ex2_example() {
        let mut matrix = Matrix::new(input::example());
        assert_eq!(matrix.tick_until_all_flash(), 195);
    }

    #[test]
    fn ex2_exercise() {
        let mut matrix = Matrix::new(input::exercise());
        assert_eq!(matrix.tick_until_all_flash(), 368);
    }
}
