mod input;

struct Heightmap {
    width: usize,
    height: usize,
    map: Vec<Vec<u8>>,
}

impl Heightmap {
    pub fn new(raw: &str) -> Self {
        let map: Vec<Vec<u8>> = raw
            .lines()
            .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
            .collect();
        let height = map.len();
        let width = map[0].len();
        Self { width, height, map }
    }

    pub fn neighbours(&self, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |(di, dj)| {
                let i = i as i32 + di;
                let j = j as i32 + dj;
                if i >= 0
                    && i < self.height as i32
                    && j >= 0
                    && j < self.width as i32
                    && (di != 0 || dj != 0)
                {
                    Some((i as usize, j as usize))
                } else {
                    None
                }
            })
    }

    pub fn neighbour_heights(&self, i: usize, j: usize) -> impl Iterator<Item = u8> + '_ {
        self.neighbours(i, j).map(move |(ni, nj)| self.map[ni][nj])
    }

    pub fn is_low_point(&self, i: usize, j: usize) -> bool {
        self.neighbour_heights(i, j).all(|h| h > self.map[i][j])
    }

    pub fn low_points(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.height).flat_map(move |i| {
            (0..self.width).filter_map(move |j| {
                if self.is_low_point(i, j) {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
    }

    pub fn low_points_heights(&self) -> impl Iterator<Item = u8> + '_ {
        self.low_points().map(|(i, j)| self.map[i][j])
    }

    pub fn total_risk_level(&self) -> u32 {
        self.low_points_heights()
            .map(|height| (height + 1) as u32)
            .sum()
    }

    pub fn basin_for_low_point(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut cells_to_visit: Vec<(usize, usize)> = vec![(i, j)];
        let mut basin_cells: Vec<(usize, usize)> = vec![];
        while let Some((i, j)) = cells_to_visit.pop() {
            for (ni, nj) in self.neighbours(i, j) {
                if !basin_cells.contains(&(ni, nj)) && self.map[ni][nj] != 9 {
                    cells_to_visit.push((ni, nj));
                    basin_cells.push((ni, nj));
                }
            }
        }
        basin_cells
    }

    pub fn basins(&self) -> impl Iterator<Item = Vec<(usize, usize)>> + '_ {
        self.low_points()
            .map(|(i, j)| self.basin_for_low_point(i, j))
    }

    pub fn basin_sizes(&self) -> impl Iterator<Item = usize> + '_ {
        self.basins().map(|basin| basin.len())
    }

    pub fn top_3_basin_sizes(&self) -> Vec<usize> {
        let mut sizes = self.basin_sizes().collect::<Vec<_>>();
        sizes.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
        sizes.truncate(3);
        sizes
    }

    pub fn multiply_top_3_basin_sizes(&self) -> usize {
        let sizes = self.top_3_basin_sizes();
        sizes[0] * sizes[1] * sizes[2]
    }
}

impl std::fmt::Debug for Heightmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                write!(f, "{}", self.map[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn main() {
    let heightmap = Heightmap::new(input::example());
    // println!("{:?}", heightmap);
    println!("Example:");
    println!("Total risk level: {:?}", heightmap.total_risk_level());
    println!(
        "Multiplied top 3 basin sizes: {:?}",
        heightmap.multiply_top_3_basin_sizes()
    );

    let heightmap = Heightmap::new(input::exercise());
    // println!("{:?}", heightmap);
    println!("Exercise:");
    println!("Total risk level: {:?}", heightmap.total_risk_level());
    println!(
        "Multiplied top 3 basin sizes: {:?}",
        heightmap.multiply_top_3_basin_sizes()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_example() {
        let heightmap = Heightmap::new(input::example());
        assert_eq!(heightmap.total_risk_level(), 15);
    }
    #[test]
    fn ex1_exercise() {
        let heightmap = Heightmap::new(input::exercise());
        assert_eq!(heightmap.total_risk_level(), 448);
    }

    #[test]
    fn ex2_example() {
        let heightmap = Heightmap::new(input::example());
        assert_eq!(heightmap.multiply_top_3_basin_sizes(), 1134);
    }

    #[test]
    fn ex2_exercise() {
        let heightmap = Heightmap::new(input::exercise());
        assert_eq!(heightmap.multiply_top_3_basin_sizes(), 1417248);
    }
}
