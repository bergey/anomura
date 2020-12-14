fn quicksort(a : &mut [i32]) {
    if a.len() > 1 {
        let q = partition(a);
        if q > 0 { quicksort(&mut a[..q]); }
        if q < a.len() { quicksort(&mut a[q+1..]); }
    }
}

fn partition(a : &mut [i32]) -> usize {
    let last = a.len() - 1;
    let x = a[last];
    let mut i = 0;
    for j in 0..(a.len()-1) {
        if a[j] <= x {
            a.swap(i, j);
            i = i+1;
        }
    }
    a.swap(i, last);
    i
}

fn hoare_partition(a: &mut[i32]) -> usize {
    let x = a[0];
    let mut i = 0;
    let mut j = a.len() - 1;
    loop {
        while a[j] > x {
            j = j - 1;
        }
        while a[i] < x {
            i = i + 1;
        }
        if i < j {
            a.swap(i, j);
        } else {
            return i;
        }
    }
}

fn hoare_quicksort(a : &mut[i32]) {
    if a.len() > 1 {
        let q = hoare_partition(a);
        if q > 0 { hoare_quicksort(&mut a[..q]); }
        if q < a.len() { hoare_quicksort(&mut a[q+1..]); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn quicksort_already_sorted() {
        let mut sorted = vec![1, 2, 3];
        quicksort(&mut sorted);
        assert_eq!(sorted, [1, 2, 3]);
    }

    #[test]
    fn quicksort_reversed() {
        let mut sorted = vec![3, 2, 1];
        quicksort(&mut sorted);
        assert_eq!(sorted, [1, 2, 3]);
    }

    #[test]
    fn quicksort_longer() {
        let mut v = vec![7, 3, 1, 9, 2, 4, 6, 5, 10, 8];
        quicksort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn hoare_quicksort_already_sorted() {
        let mut sorted = vec![1, 2, 3];
        hoare_quicksort(&mut sorted);
        assert_eq!(sorted, [1, 2, 3]);
    }

    #[test]
    fn hoare_quicksort_reversed() {
        let mut sorted = vec![3, 2, 1];
        hoare_quicksort(&mut sorted);
        assert_eq!(sorted, [1, 2, 3]);
    }

    #[test]
    fn hoare_quicksort_longer() {
        let mut v = vec![7, 3, 1, 9, 2, 4, 6, 5, 10, 8];
        hoare_quicksort(&mut v);
        assert_eq!(v, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
