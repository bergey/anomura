fn insertion(mut v: Vec<i32>) -> Vec<i32> {
    let mut j = 1;
    while j < v.len() {
        // insert v[j] into position
        let key = v[j];
        let mut i = j;
        while i > 0 && v[i-1] > key {
            v[i] = v[i-1];
            i = i - 1;
        }
        v[i] = key;
        j += 1;
    }
    return v;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn insertion_already_sorted() {
        assert!(insertion(vec![1, 2, 3]) == vec![1,2,3]);
    }

    #[test]
    fn insertion_reversed() {
        assert_eq!(insertion(vec![3, 2, 1]), [1, 2, 3]);
    }
}
