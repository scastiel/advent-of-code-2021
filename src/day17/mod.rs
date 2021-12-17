use std::ops::RangeInclusive;

mod input;

type Num = i64;
type Pos = (Num, Num);
type Target = (RangeInclusive<Num>, RangeInclusive<Num>);

fn pos_n((vx0, vy0): Pos, n: Num) -> Pos {
    fn coord_n(n: Num, v0: Num) -> Num {
        assert!(n >= 0);
        if n == 0 {
            0
        } else {
            (v0 - (n - 1)..=v0).sum::<Num>()
        }
    }

    let xn = if n <= vx0 {
        coord_n(n, vx0)
    } else {
        coord_n(vx0, vx0)
    };
    let yn = coord_n(n, vy0);
    (xn, yn)
}

pub fn in_target((vx0, vy0): Pos, (xr, yr): &Target) -> Option<Num> {
    assert!(vx0 >= 0, "Assuming vx0 >= 0");
    let mut max_y = 0;
    for n in 0.. {
        let (x, y) = pos_n((vx0, vy0), n);
        max_y = max_y.max(y);
        if xr.contains(&x) && yr.contains(&y) {
            return Some(max_y);
        }

        let vx = (vx0 - n).max(0);
        let is_out_x = vx == 0 && x < *xr.start() || x > *xr.end();
        let is_out_y = y < *yr.start();
        if is_out_x || is_out_y {
            return None;
        }
    }
    unreachable!()
}

fn find_num_solutions_and_max_height(target: &Target) -> (usize, Num) {
    let vxs = 0..*target.0.end() + 1;
    let vys = *target.1.start()..=-*target.1.start();
    let vs = vxs.flat_map(|vx| vys.clone().map(move |vy| (vx, vy)));
    vs.fold((0, 0), |(num_solutions, max_height), (vx, vy)| {
        if let Some(height) = in_target((vx, vy), target) {
            (num_solutions + 1, max_height.max(height))
        } else {
            (num_solutions, max_height)
        }
    })
}

pub fn main() {
    let (num_solutions, max_height) = find_num_solutions_and_max_height(&input::example());
    println!(
        "Example: found {} solutions, max height = {}",
        num_solutions, max_height
    );
    let (num_solutions, max_height) = find_num_solutions_and_max_height(&input::exercise());
    println!(
        "Exercise: found {} solutions, max height = {}",
        num_solutions, max_height
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_n() {
        assert_eq!(pos_n((7, 2), 0), (0, 0));
        assert_eq!(pos_n((7, 2), 1), (7, 2));
        assert_eq!(pos_n((7, 2), 2), (13, 3));
        assert_eq!(pos_n((7, 2), 3), (18, 3));
        assert_eq!(pos_n((7, 2), 4), (22, 2));
        assert_eq!(pos_n((7, 2), 5), (25, 0));
        assert_eq!(pos_n((7, 2), 6), (27, -3));
        assert_eq!(pos_n((7, 2), 7), (28, -7));
        assert_eq!(pos_n((7, 2), 8), (28, -12));
        assert_eq!(pos_n((7, 2), 9), (28, -18));
    }

    #[test]
    fn test_passes_in_target() {
        let target: Target = (20..=30, -10..=-5);
        assert!(in_target((7, 2), &target).is_some());
        assert!(in_target((6, 3), &target).is_some());
        assert!(in_target((9, 0), &target).is_some());
        assert!(in_target((17, -4), &target).is_none());
        assert!(in_target((6, 0), &target).is_some());
        assert!(in_target((7, -1), &target).is_some());
    }

    #[test]
    fn example() {
        let (num_solutions, max_height) = find_num_solutions_and_max_height(&input::example());
        assert_eq!(num_solutions, 112);
        assert_eq!(max_height, 45);
    }

    #[test]
    fn exercise() {
        let (num_solutions, max_height) = find_num_solutions_and_max_height(&input::exercise());
        assert_eq!(num_solutions, 5644);
        assert_eq!(max_height, 13203);
    }
}
