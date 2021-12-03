mod input;

pub fn main() {
    let input = input::exercise();
    println!("Input: {:?}", input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = input::example();
        assert!(input);
    }
}
