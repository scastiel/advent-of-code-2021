use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use self::input::Number;

mod input;

enum NodeValue {
    Reg(i64),
    Pair(Rc<RefCell<Node>>, Rc<RefCell<Node>>),
}

impl std::fmt::Debug for NodeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reg(n) => write!(f, "{}", n)?,
            Self::Pair(left, right) => write!(f, "[{:?}, {:?}]", *left.borrow(), *right.borrow())?,
        }
        Ok(())
    }
}

impl NodeValue {
    pub fn reg(value: i64) -> Rc<RefCell<NodeValue>> {
        Rc::new(RefCell::new(NodeValue::Reg(value)))
    }

    pub fn pair(left: &Rc<RefCell<Node>>, right: &Rc<RefCell<Node>>) -> Rc<RefCell<NodeValue>> {
        Rc::new(RefCell::new(NodeValue::Pair(
            Rc::clone(left),
            Rc::clone(right),
        )))
    }

    pub fn is_reg(&self) -> bool {
        matches!(self, &NodeValue::Reg(_))
    }

    pub fn left(&self) -> Rc<RefCell<Node>> {
        if let NodeValue::Pair(left, _) = self {
            return Rc::clone(left);
        }
        panic!()
    }

    pub fn right(&self) -> Rc<RefCell<Node>> {
        if let NodeValue::Pair(_, right) = self {
            return Rc::clone(right);
        }
        panic!()
    }

    pub fn reg_val(&self) -> i64 {
        if let NodeValue::Reg(n) = *self {
            return n;
        }
        panic!()
    }
}

struct Node {
    value: Rc<RefCell<NodeValue>>,
    parent: Option<Weak<RefCell<Node>>>,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value.borrow())?;
        Ok(())
    }
}

impl Node {
    pub fn new(value: Rc<RefCell<NodeValue>>) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            parent: None,
        }))
    }

    pub fn level(&self) -> usize {
        match &self.parent {
            None => 0,
            Some(parent) => {
                let parent = parent.upgrade().unwrap();
                let parent_level = parent.borrow().level();
                1 + parent_level
            }
        }
    }

    pub fn is_reg(&self) -> bool {
        self.value.borrow().is_reg()
    }

    pub fn left(&self) -> Rc<RefCell<Node>> {
        self.value.borrow().left()
    }

    pub fn right(&self) -> Rc<RefCell<Node>> {
        self.value.borrow().right()
    }

    pub fn reg_val(&self) -> i64 {
        self.value.borrow().reg_val()
    }
}

fn convert(number: &Number) -> Rc<RefCell<Node>> {
    match number {
        Number::Reg(n) => Node::new(NodeValue::reg(*n)),
        Number::Pair(left, right) => {
            let left_node = convert(left);
            let right_node = convert(right);
            let node = Node::new(NodeValue::pair(&left_node, &right_node));
            left_node.borrow_mut().parent = Some(Rc::downgrade(&node));
            right_node.borrow_mut().parent = Some(Rc::downgrade(&node));
            node
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

use Dir::*;

fn go_up(node: Rc<RefCell<Node>>, looking_for: Dir) -> Option<Rc<RefCell<Node>>> {
    match &node.borrow().parent {
        None => None,
        Some(parent) => {
            let parent = parent.upgrade().unwrap();
            let coming_from = if Rc::ptr_eq(&parent.borrow().left(), &node) {
                Left
            } else if Rc::ptr_eq(&parent.borrow().right(), &node) {
                Right
            } else {
                panic!();
            };
            if coming_from == looking_for {
                go_up(parent, looking_for)
            } else {
                match coming_from {
                    Left => Some(go_down(parent.borrow().right(), Left)),
                    Right => Some(go_down(parent.borrow().left(), Right)),
                }
            }
        }
    }
}

fn go_down(node: Rc<RefCell<Node>>, looking_for: Dir) -> Rc<RefCell<Node>> {
    match &*node.borrow().value.borrow() {
        NodeValue::Reg(_) => Rc::clone(&node),
        NodeValue::Pair(left, right) => match looking_for {
            Left => go_down(Rc::clone(left), Left),
            Right => go_down(Rc::clone(right), Right),
        },
    }
}

fn update(node: &Rc<RefCell<Node>>, dir: Dir) {
    if let Some(to_update) = go_up(Rc::clone(node), dir) {
        let to_update = to_update.borrow();
        *to_update.value.borrow_mut() =
            NodeValue::Reg(to_update.reg_val() + node.borrow().reg_val());
    }
}

fn set_to_zero(node: &Rc<RefCell<Node>>) {
    let mut node_mut = node.borrow_mut();
    (*node_mut).value = Rc::new(RefCell::new(NodeValue::Reg(0)));
}

fn explode(node: &Rc<RefCell<Node>>) {
    update(&node.borrow().left(), Left);
    update(&node.borrow().right(), Right);
    set_to_zero(node);
}

fn pair_to_explode(node: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
    match &*node.borrow().value.borrow() {
        NodeValue::Reg(_) => None,
        NodeValue::Pair(left, right) => {
            if node.borrow().level() >= 4 && left.borrow().is_reg() && right.borrow().is_reg() {
                return Some(Rc::clone(node));
            }
            pair_to_explode(left).or_else(|| pair_to_explode(right))
        }
    }
}

fn node_to_split(node: &Rc<RefCell<Node>>) -> Option<Rc<RefCell<Node>>> {
    match &*node.borrow().value.borrow() {
        NodeValue::Reg(n) => {
            if *n >= 10 {
                Some(Rc::clone(node))
            } else {
                None
            }
        }
        NodeValue::Pair(left, right) => node_to_split(left).or_else(|| node_to_split(right)),
    }
}

fn split(node: &Rc<RefCell<Node>>) {
    let v = node.borrow().reg_val();
    let half = v as f64 / 2.0;
    let left = Node::new(NodeValue::reg(half.floor() as i64));
    left.borrow_mut().parent = Some(Rc::downgrade(node));
    let right = Node::new(NodeValue::reg(half.ceil() as i64));
    right.borrow_mut().parent = Some(Rc::downgrade(node));
    node.borrow_mut().value = NodeValue::pair(&left, &right);
}

fn add(left: &Rc<RefCell<Node>>, right: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let node = Node::new(NodeValue::pair(left, right));
    left.borrow_mut().parent = Some(Rc::downgrade(&node));
    right.borrow_mut().parent = Some(Rc::downgrade(&node));
    reduce(&node);
    node
}

fn reduce(node: &Rc<RefCell<Node>>) {
    loop {
        let mut updated = false;
        if let Some(to_explode) = pair_to_explode(node) {
            // println!("Pair to explode: {:?}", to_explode.borrow());
            explode(&to_explode);
            // println!("After explode: {:?}", node.borrow());
            updated = true;
        } else if let Some(to_split) = node_to_split(node) {
            // println!("Pair to split: {:?}", to_split.borrow());
            split(&to_split);
            // println!("After split: {:?}", node.borrow());
            updated = true;
        }
        if !updated {
            break;
        }
    }
}

fn magnitude(node: &Rc<RefCell<Node>>) -> i64 {
    match &*node.borrow().value.borrow() {
        NodeValue::Reg(n) => *n,
        NodeValue::Pair(left, right) => 3 * magnitude(left) + 2 * magnitude(right),
    }
}

pub fn magnitude_total(numbers: &[Number]) -> i64 {
    let sum = numbers
        .iter()
        .map(convert)
        .reduce(|a, b| add(&a, &b))
        .unwrap();
    magnitude(&sum)
}

pub fn magnitude_max_pair(numbers: &[Number]) -> i64 {
    let mut max = 0;
    for n1 in numbers.iter() {
        for n2 in numbers.iter() {
            max = max.max(magnitude(&add(&convert(n1), &convert(n2))));
        }
    }
    max
}

pub fn main() {
    let numbers = input::exercise();

    println!("Total magnitude: {}", magnitude_total(&numbers));
    println!("Max pair magnitude = {}", magnitude_max_pair(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let numbers = input::example();
        assert_eq!(magnitude_total(&numbers), 3488);
        assert_eq!(magnitude_max_pair(&numbers), 3946);
    }

    #[test]
    fn exercise() {
        let numbers = input::exercise();
        assert_eq!(magnitude_total(&numbers), 4132);
        assert_eq!(magnitude_max_pair(&numbers), 4685);
    }
}
