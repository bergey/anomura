#![feature(test)]
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut prefix = Vec::new();
    let mut results = Vec::new();
    let mut lefts = 0;
    let mut rights = 0;

    'outer: loop {
        if lefts < n {
            prefix.push(b'(');
            lefts += 1;
        } else if rights < lefts {
            prefix.push(b')');
            rights += 1;
        } else if lefts == n && rights == n {
            unsafe {
                results.push(String::from_utf8_unchecked(prefix.clone()));
            }
            // backtrack / unwind
            while let Some(c) = prefix.pop() {
                match c {
                    b')' => rights -= 1,
                    b'(' => {
                        lefts -= 1;
                        if lefts > rights {
                            prefix.push(b')');
                            rights += 1;
                            break;
                        } else if lefts == 0 { // popped first paren
                            break 'outer;
                        } // else prefix is balanced, keep backtracking
                    },
                    _ => panic!("paren stack has non-paren byte"),
                }
            }
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn one() {
        let actual = generate_parenthesis(1);
        assert_eq!(actual, ["()"])
    }

    #[test]
    fn two() {
        let actual = generate_parenthesis(2);
        assert_eq!(actual, ["(())", "()()"])
    }

    #[test]
    fn three() {
        let actual = generate_parenthesis(3);
        assert_eq!(actual, ["((()))", "(()())", "(())()", "()(())", "()()()"])
    }

    #[bench]
    fn bench_four(b: &mut Bencher) {
        b.iter(|| generate_parenthesis(4))
    }

    #[bench]
    fn bench_eight(b: &mut Bencher) {
        b.iter(|| generate_parenthesis(8))
    }

    #[bench]
    fn bench_nine(b: &mut Bencher) {
        b.iter(|| generate_parenthesis(9))
    }

    #[bench]
    fn bench_ten(b: &mut Bencher) {
        b.iter(|| generate_parenthesis(10))
    }
}
