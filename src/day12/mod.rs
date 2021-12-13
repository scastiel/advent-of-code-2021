use std::{collections::HashSet, fmt::Debug, rc::Rc};

mod input;

#[derive(PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small(String),
    Big(String),
}

impl Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start => write!(f, "start"),
            Self::End => write!(f, "end"),
            Self::Small(name) | Self::Big(name) => write!(f, "{}", name),
        }
    }
}

enum PathValidation {
    Complete,
    Valid,
    Invalid,
}

impl Cave {
    pub fn new(name: &str) -> Cave {
        match name {
            "start" => Cave::Start,
            "end" => Cave::End,
            name if name.to_lowercase() == name => Cave::Small(name.to_string()),
            name if name.to_uppercase() == name => Cave::Big(name.to_string()),
            _ => panic!("Invalid cave name: {}", name),
        }
    }

    pub fn matches(&self, name: &str) -> bool {
        match self {
            Cave::Start => name == "start",
            Cave::End => name == "end",
            Cave::Small(n) | Cave::Big(n) => name == n,
        }
    }

    pub fn is_start(&self) -> bool {
        matches!(self, Cave::Start)
    }

    pub fn is_small(&self) -> bool {
        matches!(self, Cave::Small(_))
    }
}

#[derive(Default, Debug)]
struct CaveMap {
    caves: Vec<Rc<Cave>>,
    connections: Vec<(Rc<Cave>, Rc<Cave>)>,
}

impl CaveMap {
    pub fn new(input: &[(&str, &str)]) -> CaveMap {
        let mut map = CaveMap::default();
        for (from, to) in input {
            map.add_connection(from, to);
        }
        map
    }

    fn get_start(&self) -> Rc<Cave> {
        self.caves
            .iter()
            .find_map(|cave| {
                if cave.is_start() {
                    Some(Rc::clone(cave))
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn get_cave(&mut self, name: &str) -> Rc<Cave> {
        if let Some(cave) = self.caves.iter().find(|&cave| cave.matches(name)) {
            Rc::clone(cave)
        } else {
            let cave = Rc::new(Cave::new(name));
            self.caves.push(Rc::clone(&cave));
            cave
        }
    }

    pub fn add_connection(&mut self, from: &str, to: &str) {
        let from = self.get_cave(from);
        let to = self.get_cave(to);
        self.connections.push((from, to));
    }

    fn linked_to<'a>(&'a self, cave: &'a Rc<Cave>) -> impl Iterator<Item = Rc<Cave>> + 'a {
        self.connections.iter().filter_map(move |(from, to)| {
            if from == cave {
                Some(Rc::clone(to))
            } else if to == cave {
                Some(Rc::clone(from))
            } else {
                None
            }
        })
    }

    fn add_path(path: &[Rc<Cave>], next: Rc<Cave>, paths: &mut HashSet<Vec<Rc<Cave>>>) {
        let mut new_path = path.to_vec();
        new_path.push(next);
        paths.insert(new_path);
    }

    pub fn paths_ex1(&self) -> HashSet<Vec<Rc<Cave>>> {
        self.paths(&Self::validate_path_ex1)
    }

    pub fn paths_ex2(&self) -> HashSet<Vec<Rc<Cave>>> {
        self.paths(&Self::validate_path_ex2)
    }

    fn paths(
        &self,
        validate_path: &dyn Fn(&Self, &[Rc<Cave>], &Rc<Cave>) -> PathValidation,
    ) -> HashSet<Vec<Rc<Cave>>> {
        let mut paths: HashSet<Vec<Rc<Cave>>> = HashSet::default();
        let mut wip_paths: HashSet<Vec<Rc<Cave>>> = HashSet::default();
        wip_paths.insert(vec![self.get_start()]);
        while !wip_paths.is_empty() {
            let path = wip_paths.iter().next().unwrap().clone();
            wip_paths.remove(&path);
            for next in self.linked_to(path.last().unwrap()) {
                match validate_path(self, &path, &next) {
                    PathValidation::Complete => Self::add_path(&path, next, &mut paths),
                    PathValidation::Valid => Self::add_path(&path, next, &mut wip_paths),
                    _ => {}
                }
            }
        }
        paths
    }

    fn has_duplicates(&self, path: &[Rc<Cave>]) -> bool {
        self.caves
            .iter()
            .filter(|cave| cave.is_small())
            .any(|cave| path.iter().filter(|c| cave.eq(c)).count() == 2)
    }

    fn validate_path_ex1(&self, path: &[Rc<Cave>], next: &Rc<Cave>) -> PathValidation {
        match **next {
            Cave::Start => PathValidation::Invalid,
            Cave::End => PathValidation::Complete,
            Cave::Big(_) => PathValidation::Valid,
            Cave::Small(_) => {
                if path.contains(next) {
                    PathValidation::Invalid
                } else {
                    PathValidation::Valid
                }
            }
        }
    }

    fn validate_path_ex2(&self, path: &[Rc<Cave>], next: &Rc<Cave>) -> PathValidation {
        match **next {
            Cave::Start => PathValidation::Invalid,
            Cave::End => PathValidation::Complete,
            Cave::Big(_) => PathValidation::Valid,
            Cave::Small(_) => {
                if path.contains(next) && self.has_duplicates(path) {
                    PathValidation::Invalid
                } else {
                    PathValidation::Valid
                }
            }
        }
    }
}

pub fn main() {
    let input = input::exercise();
    let map = CaveMap::new(&input);
    println!("Ex1, Number of paths: {:?}", map.paths_ex1().len());
    println!("Ex2, Number of paths: {:?}", map.paths_ex2().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_example1() {
        let map = CaveMap::new(&input::example1());
        assert_eq!(map.paths_ex1().len(), 10);
    }

    #[test]
    fn ex1_example2() {
        let map = CaveMap::new(&input::example2());
        assert_eq!(map.paths_ex1().len(), 19);
    }

    #[test]
    fn ex1_example3() {
        let map = CaveMap::new(&input::example3());
        assert_eq!(map.paths_ex1().len(), 226);
    }

    #[test]
    fn ex1_exercise() {
        let map = CaveMap::new(&input::exercise());
        assert_eq!(map.paths_ex1().len(), 4912);
    }

    #[test]
    fn ex2_example1() {
        let map = CaveMap::new(&input::example1());
        assert_eq!(map.paths_ex2().len(), 36);
    }

    #[test]
    fn ex2_example2() {
        let map = CaveMap::new(&input::example2());
        assert_eq!(map.paths_ex2().len(), 103);
    }

    #[test]
    fn ex2_example3() {
        let map = CaveMap::new(&input::example3());
        assert_eq!(map.paths_ex2().len(), 3509);
    }

    #[test]
    #[ignore]
    fn ex2_exercise() {
        let map = CaveMap::new(&input::exercise());
        assert_eq!(map.paths_ex2().len(), 150004);
    }
}
