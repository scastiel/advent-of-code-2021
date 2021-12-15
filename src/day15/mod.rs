use std::collections::VecDeque;

mod input;

fn read_input(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| line.trim().chars().map(|c| c as u64 - '0' as u64).collect())
        .collect()
}

fn neighbours_iter(n: usize, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    [(1_i32, 0_i32), (0, 1), (-1, 0), (0, -1)]
        .into_iter()
        .map(move |(di, dj)| (i as i32 + di, j as i32 + dj))
        .filter(move |&(i, j)| i >= 0 && i < n as i32 && j >= 0 && j < n as i32)
        .map(|(i, j)| (i as usize, j as usize))
}

pub fn djikstra(map: &[Vec<u64>], num_tiles: usize) -> Vec<Vec<u64>> {
    let n = map.len() * num_tiles;
    let mut distances: Vec<Vec<u64>> = (0..n).map(|_| (0..n).map(|_| u64::MAX).collect()).collect();
    distances[0][0] = 0;
    let mut todo = VecDeque::from(vec![(0, 0)]);
    while let Some((i, j)) = todo.pop_front() {
        for (ni, nj) in neighbours_iter(n, i, j) {
            let cur_dist = distances[ni][nj];
            let new_dist = distances[i][j] + value_at(map, ni, nj);
            if new_dist < cur_dist {
                distances[ni][nj] = new_dist;
                todo.push_back((ni, nj));
            }
        }
    }
    distances
}

fn int_div(a: usize, b: usize) -> u64 {
    (a as f64 / b as f64).floor() as u64
}

fn value_at(map: &[Vec<u64>], i: usize, j: usize) -> u64 {
    let n = map.len();
    (map[i % n][j % n] + int_div(i, n) + int_div(j, n) - 1) % 9 + 1
}

fn print_matrix(matrix: &[Vec<u64>], n: usize, f: &dyn Fn(&[Vec<u64>], usize, usize) -> u64) {
    for i in 0..n {
        for j in 0..n {
            print!("{:2} ", f(matrix, i, j));
        }
        println!()
    }
}

#[allow(dead_code)]
fn print_distances(distances: &[Vec<u64>]) {
    print_matrix(distances, distances.len(), &|matrix, i, j| matrix[i][j]);
}

#[allow(dead_code)]
fn print_map(map: &[Vec<u64>], num_tiles: usize) {
    print_matrix(map, map.len() * num_tiles, &value_at);
}

fn min_distance(distances: &[Vec<u64>]) -> u64 {
    let n = distances.len();
    distances[n - 1][n - 1]
}

pub fn main() {
    let map = read_input(input::exercise());
    let num_tiles = 1;
    // println!("Input:");
    // print_map(&map, num_tiles);
    let distances = djikstra(&map, num_tiles);
    // println!("Distances:");
    // print_distances(&distances);
    println!("Min distance = {}", min_distance(&distances));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_example() {
        let map = read_input(input::example());
        let distances = djikstra(&map, 1);
        assert_eq!(min_distance(&distances), 40);
    }

    #[test]
    fn ex1_exercise() {
        let map = read_input(input::exercise());
        let distances = djikstra(&map, 1);
        assert_eq!(min_distance(&distances), 589);
    }

    #[test]
    fn ex2_example() {
        let map = read_input(input::example());
        let distances = djikstra(&map, 5);
        assert_eq!(min_distance(&distances), 315);
    }

    #[test]
    fn ex2_exercise() {
        let map = read_input(input::exercise());
        let distances = djikstra(&map, 5);
        assert_eq!(min_distance(&distances), 2885);
    }
}
