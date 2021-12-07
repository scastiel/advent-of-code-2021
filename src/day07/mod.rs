mod input;

pub fn main() {
    let positions = input::exercise();

    let cost = compute_best_cost(&positions, &cost_ex1);
    println!("Ex1 Cost = {}", cost);

    let cost = compute_best_cost(&positions, &cost_ex2);
    println!("Ex2 Cost = {}", cost);
}

type CostFn = dyn Fn(i32, i32) -> i32;

fn cost_ex1(from: i32, to: i32) -> i32 {
    (from - to).abs()
}

fn cost_ex2(from: i32, to: i32) -> i32 {
    let distance = (from - to).abs();
    distance * (distance + 1) / 2
}

fn compute_best_cost(positions: &[i32], cost_fn: &CostFn) -> i32 {
    assert!(
        !positions.is_empty(),
        "Positions must contain at least one item."
    );
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let compute_cost = |middle: i32| positions.iter().fold(0, |s, &p| s + cost_fn(p, middle));
    (min..=max).map(compute_cost).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_example() {
        let positions = input::example();
        let cost = compute_best_cost(&positions, &cost_ex1);
        assert_eq!(cost, 37);
    }

    #[test]
    fn ex1_exercise() {
        let positions = input::exercise();
        let cost = compute_best_cost(&positions, &cost_ex1);
        assert_eq!(cost, 344297);
    }

    #[test]
    fn ex2_example() {
        let positions = input::example();
        let cost = compute_best_cost(&positions, &cost_ex2);
        assert_eq!(cost, 168);
    }

    #[test]
    fn ex2_exercise() {
        let positions = input::exercise();
        let cost = compute_best_cost(&positions, &cost_ex2);
        assert_eq!(cost, 97164301);
    }
}
