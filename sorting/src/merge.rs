use std::ops::Div;

fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut v : Vec<i32> = Vec::with_capacity(a.len() + b.len());

    while i < a.len() && j < b.len() {
    // pick smallest until one runs out
        if a[i] < b[j] {
            v.push(a[i]);
            i += 1;
        } else {
            v.push(b[j]);
            j += 1;
        }
    }

    // take the rest
    while i < a.len() {
        v.push(a[i]);
        i += 1;
    }
    while j < b.len() {
        v.push(b[j]);
        j += 1;
    }

    return v;
}

pub fn merge_sort(v: &[i32]) -> Vec<i32> {
    if v.len() > 1 {
        let (left, right) = v.split_at(v.len().div(2));
        merge(&merge_sort(left), &merge_sort(right))
    } else {
        v.to_vec()
    }
}

fn merge_inplace(a: &mut [i32], p: usize, q: usize, r: usize) {
    let mut i = p;
    let mut j = q;
    let mut v: Vec<i32> = Vec::with_capacity(r-p);

    while i < q && j < r {
        if a[i] < a[j] {
            v.push(a[i]);
            i += 1;
        } else {
            v.push(a[j]);
            j += 1;
        }
    }

    while i < q {
        v.push(a[i]);
        i += 1;
    }
    while j < r {
        v.push(a[j]);
        j += 1;
    }

    i = p;
    for elem in v {
        a[i] = elem;
        i += 1
    }
}

fn merge_inplace_helper(a: &mut [i32], p: usize, r: usize) {
    if r - p > 1 {
        let q = (r + p).div(2);
        merge_inplace_helper(a, p, q);
        merge_inplace_helper(a, q, r);
        merge_inplace(a, p, q, r);
    }
}

pub fn merge_inplace_sort(a: &mut [i32]) {
    let l = a.len();
    merge_inplace_helper(a, 0, l);
}
         
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn merge_already_sorted() {
        let sorted = merge_sort(&vec![1, 2, 3]);
        assert_eq!(sorted, [1, 2, 3]);
    }

    #[test]
    fn merge_reversed() {
        let sorted = merge_sort(&vec![3, 2, 1]);
        assert_eq!(sorted, [1, 2, 3]);
    }

    
    #[test]
    fn merge_mut_already_sorted() {
        let mut sorted = vec![1, 2, 3];
        merge_inplace_sort(&mut sorted);
        assert_eq!(sorted, [1, 2, 3]);
    }

    #[test]
    fn merge_mut_reversed() {
        let mut sorted = vec![3, 2, 1];
        merge_inplace_sort(&mut sorted);
        assert_eq!(sorted, [1, 2, 3]);
    }
}
