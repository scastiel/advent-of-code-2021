mod input;

#[derive(PartialEq, Debug, Default)]
struct GammaEpsilon {
    gamma: u32,
    epsilon: u32,
}

#[derive(PartialEq, Debug, Default)]
struct OxygenCO2 {
    oxygen: u32,
    co2: u32,
}

pub fn main() {
    let diagnostic = input::exercise();
    let GammaEpsilon { gamma, epsilon } = calculate_gamma_epsilon(&diagnostic);
    println!(
        "Ex01: gamma = {}, epsilon = {}, result = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
    let OxygenCO2 { oxygen, co2 } = calculate_oxygen_co2(&diagnostic);
    println!(
        "Ex02: oxygen = {}, co2 = {}, result = {}",
        oxygen,
        co2,
        oxygen * co2,
    );
}

#[derive(PartialEq, Clone, Copy)]
enum Criteria {
    LeastFrequent,
    MostFrequent,
}

fn split<'a>(numbers: &[&'a str], index: usize, criteria: Criteria) -> Vec<&'a str> {
    let mut parts = (vec![], vec![]);
    for &number in numbers {
        if number.chars().nth(index).unwrap() == '0' {
            parts.0.push(number);
        } else {
            parts.1.push(number);
        }
    }
    let (most_frequent, least_frequent) = if parts.0.len() > parts.1.len() {
        (parts.0, parts.1)
    } else {
        (parts.1, parts.0)
    };
    match criteria {
        Criteria::LeastFrequent => least_frequent,
        Criteria::MostFrequent => most_frequent,
    }
}

fn iterate_split<'a>(numbers: &[&'a str], criteria: Criteria) -> &'a str {
    let mut numbers = numbers.to_vec();
    let mut index = 0;
    while numbers.len() > 1 {
        numbers = split(&numbers, index, criteria);
        index += 1;
    }
    numbers.get(0).unwrap()
}

fn parse_binary(binary: &str) -> u32 {
    binary
        .chars()
        .rev()
        .enumerate()
        .map(|(i, b)| b.to_digit(2).unwrap() * 2_u32.pow(i as u32))
        .sum()
}

fn calculate_oxygen_co2(diagnostic: &[&str]) -> OxygenCO2 {
    OxygenCO2 {
        oxygen: parse_binary(iterate_split(diagnostic, Criteria::MostFrequent)),
        co2: parse_binary(iterate_split(diagnostic, Criteria::LeastFrequent)),
    }
}

fn calculate_gamma_epsilon(diagnostic: &[&str]) -> GammaEpsilon {
    let n = diagnostic.len();
    assert!(n > 0);
    let size = diagnostic.get(0).unwrap().len();
    assert!(size > 0);
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for i in 0..size {
        let num_1 = diagnostic
            .iter()
            .filter(|&&number| number.chars().nth(i).unwrap() == '1')
            .count();
        if 2 * num_1 > n {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }
    GammaEpsilon {
        gamma: parse_binary(&gamma),
        epsilon: parse_binary(&epsilon),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_gamma_epsilon() {
        let diagnostic = input::example();
        assert_eq!(
            calculate_gamma_epsilon(&diagnostic),
            GammaEpsilon {
                gamma: 22,
                epsilon: 9
            }
        );
    }

    #[test]
    fn ex1() {
        let diagnostic = input::exercise();
        assert_eq!(
            calculate_gamma_epsilon(&diagnostic),
            GammaEpsilon {
                gamma: 1816,
                epsilon: 2279
            }
        );
    }

    #[test]
    fn test_split() {
        let numbers = input::example();
        assert_eq!(
            split(&numbers, 0, Criteria::MostFrequent),
            vec!["11110", "10110", "10111", "10101", "11100", "10000", "11001"]
        );
        assert_eq!(
            split(
                &["11110", "10110", "10111", "10101", "11100", "10000", "11001"],
                1,
                Criteria::MostFrequent
            ),
            vec!["10110", "10111", "10101", "10000"]
        );
        assert_eq!(
            split(&numbers, 0, Criteria::LeastFrequent),
            vec!["00100", "01111", "00111", "00010", "01010"]
        );
    }

    #[test]
    fn test_iterate_split() {
        let numbers = input::example();
        assert_eq!(iterate_split(&numbers, Criteria::MostFrequent), "10111");
        assert_eq!(iterate_split(&numbers, Criteria::LeastFrequent), "01010");
    }

    #[test]
    fn test_calculate_oxygen_co2() {
        let diagnostic = input::example();
        assert_eq!(
            calculate_oxygen_co2(&diagnostic),
            OxygenCO2 {
                oxygen: 23,
                co2: 10
            }
        );
    }

    #[test]
    fn ex2() {
        let diagnostic = input::exercise();
        assert_eq!(
            calculate_oxygen_co2(&diagnostic),
            OxygenCO2 {
                oxygen: 2031,
                co2: 2104
            }
        );
    }
}
