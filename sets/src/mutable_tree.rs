use std::cmp::Ordering;

enum Set {
    Leaf,
    Node(Box<Set>, i32, Box<Set>)
}
use self::Set::{Leaf, Node};

const EMPTY : Set = Set::Leaf;

fn insert(x: i32, set: &mut Set) {
    match *set {
        Leaf => *set = Node(Box::new(Leaf), x, Box::new(Leaf)),
        Node(ref mut left, i, ref mut right) => match x.cmp(&i) {
            Ordering::Equal => (),
            Ordering::Less => insert(x, left),
            Ordering::Greater => insert(x, right)
        }
    }
}

fn contains(x: i32, set: &Set) -> bool {
    match *set {
        Leaf => false,
        Node(ref left, i, ref right) => match x.cmp(&i) {
            Ordering:: Equal => true,
            Ordering::Less => contains(x, left),
            Ordering::Greater => contains(x, right)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn empty_does_not_contain() {
        assert_eq!(contains(1, &EMPTY), false);
    }

    #[test]
    fn contains_once_inserted() {
        let mut set = EMPTY;
        insert(1, &mut set);
        assert_eq!(contains(1, &set), true);
    }

    #[test]
    fn contains_each_inserted() {
        let mut set = EMPTY;
        insert(1, &mut set);
        insert(2, &mut set);
        insert(3, &mut set);
        assert!(contains(1, &set));
        assert!( contains(2, &set) );
        assert!( contains(3, &set));
    }

    #[test]
    fn does_not_contain() {
        let mut set = EMPTY;
        insert(1, &mut set);
        insert(2, &mut set);
        insert(3, &mut set);
        assert_eq!(contains(4, &set), false);
    }
}
