use std::{collections::HashMap, fmt::Debug};

mod input;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Line {
    pub from: Point,
    pub to: Point,
}

impl Line {
    pub fn new(from: Point, to: Point) -> Line {
        Line { from, to }
    }

    fn is_horizontal(&self) -> bool {
        self.from.x == self.to.x
    }

    fn is_vertical(&self) -> bool {
        self.from.y == self.to.y
    }

    fn is_diagonal(&self) -> bool {
        (self.from.x - self.to.x).abs() == (self.from.y - self.to.y).abs()
    }

    fn variations(&self) -> (i32, i32) {
        let var_x = var(self.to.x - self.from.x);
        let var_y = var(self.to.y - self.from.y);
        (var_x, var_y)
    }

    pub fn points(&self, diagonals_settings: DiagonalsSetting) -> Vec<Point> {
        if !(self.is_horizontal()
            || self.is_vertical()
            || (diagonals_settings == DiagonalsSetting::WithDiagonals && self.is_diagonal()))
        {
            return vec![];
        }
        let (var_x, var_y) = self.variations();
        let Point { mut x, mut y } = self.from;
        let Point { x: to_x, y: to_y } = self.to;
        let mut points = vec![Point::new(x, y)];
        while x != to_x || y != to_y {
            x += var_x;
            y += var_y;
            points.push(Point::new(x, y));
        }
        points
    }
}

#[derive(Default)]
pub struct Grid {
    covered: HashMap<Point, usize>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum DiagonalsSetting {
    WithDiagonals,
    WithoutDiagonals,
}

impl Grid {
    pub fn with_lines(lines: &[Line], diagonals_setting: DiagonalsSetting) -> Grid {
        let mut grid = Grid::default();
        grid.add_lines(lines, diagonals_setting);
        grid
    }

    pub fn add_lines(&mut self, lines: &[Line], diagonals_setting: DiagonalsSetting) {
        lines
            .iter()
            .for_each(|line| self.add_line(line, diagonals_setting));
    }

    fn add_line(&mut self, line: &Line, diagonals_setting: DiagonalsSetting) {
        for point in line.points(diagonals_setting) {
            self.add_point(point);
        }
    }

    fn add_point(&mut self, point: Point) {
        if let Some(n) = self.covered.get_mut(&point) {
            *n += 1;
        } else {
            self.covered.insert(point, 1);
        }
    }

    fn bottom_right_corner(&self) -> Point {
        let points = self.covered.keys();
        let mut bottom_right = Point::new(0, 0);
        for point in points {
            bottom_right.x = bottom_right.x.max(point.x);
            bottom_right.y = bottom_right.y.max(point.y);
        }
        bottom_right
    }

    pub fn count_points_with_two_lines(&self) -> usize {
        self.covered.values().filter(|&&n| n >= 2).count()
    }
}

impl<'a> Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bottom_right = self.bottom_right_corner();
        for y in 0..=bottom_right.y {
            for x in 0..=bottom_right.x {
                if let Some(&n) = self.covered.get(&Point::new(x, y)) {
                    write!(f, "{}", n)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn var(n: i32) -> i32 {
    match n {
        0 => 0,
        n if n > 0 => 1,
        _ => -1,
    }
}

pub fn main() {
    let lines = input::exercise();

    let grid = Grid::with_lines(&lines, DiagonalsSetting::WithoutDiagonals);
    println!(
        "Without diagonals, number of points with at least two lines: {}",
        grid.count_points_with_two_lines()
    );

    let grid = Grid::with_lines(&lines, DiagonalsSetting::WithDiagonals);
    println!(
        "With diagonals, number of points with at least two lines: {}",
        grid.count_points_with_two_lines()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_example() {
        let lines = input::example();
        let grid = Grid::with_lines(&lines, DiagonalsSetting::WithoutDiagonals);
        assert_eq!(grid.count_points_with_two_lines(), 5);
    }

    #[test]
    fn ex1_exercise() {
        let lines = input::exercise();
        let grid = Grid::with_lines(&lines, DiagonalsSetting::WithoutDiagonals);
        assert_eq!(grid.count_points_with_two_lines(), 5632);
    }

    #[test]
    fn ex2_example() {
        let lines = input::example();
        let grid = Grid::with_lines(&lines, DiagonalsSetting::WithDiagonals);
        assert_eq!(grid.count_points_with_two_lines(), 12);
    }

    #[test]
    fn ex2_exercise() {
        let lines = input::exercise();
        let grid = Grid::with_lines(&lines, DiagonalsSetting::WithDiagonals);
        assert_eq!(grid.count_points_with_two_lines(), 22213);
    }
}
