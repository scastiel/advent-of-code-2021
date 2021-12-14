use std::{cell::RefCell, collections::HashMap, ops::AddAssign};

mod input;

type InsertionRules = HashMap<(char, char), char>;
type OccurrencesMap = HashMap<char, f64>;
type PairsMap = HashMap<(char, char), usize>;

pub struct Input {
    template: String,
    insertion_rules: InsertionRules,
}

fn inc_map<K, V>(map: &mut HashMap<K, V>, key: K, inc: V)
where
    K: std::cmp::Eq + std::hash::Hash,
    V: AddAssign + Default,
{
    *map.entry(key).or_insert_with(V::default) += inc;
}

fn pairs(template: &'_ str) -> impl Iterator<Item = (char, char)> + '_ {
    template.chars().zip(template.chars().skip(1))
}

fn first_last(template: &str) -> (char, char) {
    (
        template.chars().next().unwrap(),
        template.chars().rev().next().unwrap(),
    )
}

pub fn calc(template: &str, insertion_rules: &InsertionRules, gen: usize) -> OccurrencesMap {
    let mut occurrences = OccurrencesMap::new();

    let (first, last) = first_last(template);
    inc_map(&mut occurrences, first, 0.5);
    inc_map(&mut occurrences, last, 0.5);

    let curr_map = RefCell::new(PairsMap::new());
    for (fst, snd) in pairs(template) {
        inc_map(&mut *curr_map.borrow_mut(), (fst, snd), 1);
    }

    for _ in (1..=gen).rev() {
        let mut next_map = PairsMap::new();
        for ((fst, snd), &n) in curr_map.borrow().iter() {
            let ins = *insertion_rules.get(&(*fst, *snd)).unwrap();
            inc_map(&mut next_map, (*fst, ins), n);
            inc_map(&mut next_map, (ins, *snd), n);
        }
        *curr_map.borrow_mut() = next_map;
    }

    for ((fst, snd), &n) in curr_map.borrow().iter() {
        inc_map(&mut occurrences, *fst, 0.5 * n as f64);
        inc_map(&mut occurrences, *snd, 0.5 * n as f64);
    }

    occurrences
}

fn min_max_occurrences(occurrences: &OccurrencesMap) -> (f64, f64) {
    let (min, max) = occurrences.values().fold(
        (None, None),
        |(min, max): (Option<f64>, Option<f64>), &num| {
            (
                min.map(|min| min.min(num)).or(Some(num)),
                max.map(|max| max.max(num)).or(Some(num)),
            )
        },
    );
    (min.unwrap(), max.unwrap())
}

pub fn main() {
    let Input {
        template,
        insertion_rules,
    } = input::exercise();
    // println!("Input: {:?}", template);

    let occurrences = calc(&template, &insertion_rules, 10);
    // println!("Occurrences: {:?}", occurrences);
    let (min, max) = min_max_occurrences(&occurrences);
    println!(
        "After 10 iterations, difference: {:?}",
        (max - min).round() as u64
    );

    let occurrences = calc(&template, &insertion_rules, 40);
    // println!("Occurrences: {:?}", occurrences);
    let (min, max) = min_max_occurrences(&occurrences);
    println!(
        "After 40 iterations, difference: {:?}",
        (max - min).round() as u64
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_example() {
        let input = input::example();
        let occurrences = calc(&input.template, &input.insertion_rules, 10);
        let (min, max) = min_max_occurrences(&occurrences);
        assert_eq!((max - min).round() as u64, 1588);
    }

    #[test]
    fn ex1_exercise() {
        let input = input::exercise();
        let occurrences = calc(&input.template, &input.insertion_rules, 10);
        let (min, max) = min_max_occurrences(&occurrences);
        assert_eq!((max - min).round() as u64, 3342);
    }

    #[test]
    fn ex2_example() {
        let input = input::example();
        let occurrences = calc(&input.template, &input.insertion_rules, 40);
        let (min, max) = min_max_occurrences(&occurrences);
        assert_eq!((max - min).round() as u64, 2188189693529);
    }

    #[test]
    fn ex2_exercise() {
        let input = input::exercise();
        let occurrences = calc(&input.template, &input.insertion_rules, 40);
        let (min, max) = min_max_occurrences(&occurrences);
        assert_eq!((max - min).round() as u64, 3776553567525);
    }
}
