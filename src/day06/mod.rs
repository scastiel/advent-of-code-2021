use std::collections::HashMap;

mod input;

struct Fishes {
    count: HashMap<u8, usize>,
}

impl Fishes {
    pub fn new(timers: &[u8]) -> Fishes {
        let mut map = HashMap::new();
        for timer in timers {
            if let Some(n) = map.get_mut(timer) {
                *n += 1;
            } else {
                map.insert(*timer, 1);
            }
        }
        Fishes { count: map }
    }

    fn count_with_timer(&self, timer: u8) -> usize {
        *self.count.get(&timer).unwrap_or(&0)
    }

    fn next(&mut self) {
        let num_0 = self.count_with_timer(0);
        self.count.insert(0, self.count_with_timer(1));
        self.count.insert(1, self.count_with_timer(2));
        self.count.insert(2, self.count_with_timer(3));
        self.count.insert(3, self.count_with_timer(4));
        self.count.insert(4, self.count_with_timer(5));
        self.count.insert(5, self.count_with_timer(6));
        self.count.insert(6, self.count_with_timer(7) + num_0);
        self.count.insert(7, self.count_with_timer(8));
        self.count.insert(8, num_0);
    }

    pub fn add_days(&mut self, num_days: usize) {
        for _ in 0..num_days {
            self.next();
        }
    }

    pub fn num_fishes(&self) -> u64 {
        self.count.values().map(|&n| n as u64).sum::<u64>()
    }
}

pub fn main() {
    let mut timers = Fishes::new(&input::exercise());
    timers.add_days(80);
    println!("After 80 days: {} fishes", timers.num_fishes());

    let mut timers = Fishes::new(&input::exercise());
    timers.add_days(256);
    println!("After 256 days: {} fishes", timers.num_fishes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_example() {
        let mut timers = Fishes::new(&input::example());
        timers.add_days(80);
        assert_eq!(timers.num_fishes(), 5934)
    }

    #[test]
    fn ex1_exercise() {
        let mut timers = Fishes::new(&input::exercise());
        timers.add_days(80);
        assert_eq!(timers.num_fishes(), 377263)
    }

    #[test]
    fn ex2_example() {
        let mut timers = Fishes::new(&input::example());
        timers.add_days(256);
        assert_eq!(timers.num_fishes(), 26984457539)
    }

    #[test]
    fn ex2_exercise() {
        let mut timers = Fishes::new(&input::exercise());
        timers.add_days(256);
        assert_eq!(timers.num_fishes(), 1695929023803)
    }
}
