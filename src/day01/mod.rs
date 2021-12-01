mod input;

pub fn main() {
    let input: Vec<u32> = input::exercise();
    println!("Ex 01 = {}", ex01(&input));
    println!("Ex 02 = {}", ex02(&input));
}

fn ex01(input: &[u32]) -> usize {
    let n = input.len();
    assert!(n >= 1);
    let mut res = 0;
    for i in 1..n {
        if input[i] > input[i - 1] {
            res += 1;
        }
    }
    res
}

fn ex02(input: &[u32]) -> usize {
    let n = input.len();
    assert!(n >= 3);
    let mut res = 0;
    for i in 3..n {
        if input[i] > input[i - 3] {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_example() {
        let input = input::example();
        assert_eq!(ex01(&input), 7);
    }

    #[test]
    fn ex2_example() {
        let input = input::example();
        assert_eq!(ex02(&input), 5);
    }
}
