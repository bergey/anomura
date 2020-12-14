use std::cmp::Ordering;

enum Set {
    Leaf,
    Node(Set, i32, Set)
}

const empty : Set = Set::Leaf;

fn insert(x: i32, set: Set) -> Set {
    match set {
        Set::Leaf => Set::Node(Set::Leaf, x, Set::Leaf),
        Set::Node(left, i, right) => match x.cmp(&i) {
            Ordering::Equal => set,
            Ordering::Less => Set::Node(insert(x, left), i, right),
            Ordering::Greater => Set::Node(left, i, insert(x, right))
        }
    }
}

fn contains(x: i32, set: Set) -> bool {
    match set {
        Set::Leaf => false,
        Set::Node(left, i, right) => match x.cmp(&i) {
            Ordering::Equal => true,
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
        assert_eq!(contains(1, empty), false);
    }

    #[test]
    fn contains_once_inserted() {
        assert_eq!(contains(1, insert(1, empty)), true);
    }

    #[test]
    fn contains_each_inserted() {
        let set = insert(1, insert(2, insert(3, empty)));
        assert!(contains(1, set));
        assert!( contains(2, set) );
        assert!( contains(3, set));
    }

    #[test]
    fn does_not_contain() {
        let set = insert(1, insert(2, insert(3, empty)));
        assert_eq!(contains(4, set), false);
    }
}
