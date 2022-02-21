#![feature(test)]
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let s = Vec::new();
    parens_with_prefix(n, &s)
}

fn parens_with_prefix(n: i32, prefix: &[u8]) -> Vec<String> {
    // eventually pass these in, but we can calculate from prefix
    let mut lefts = 0;
    let mut rights = 0;
    for c in prefix {
        match c {
            b'(' => lefts += 1,
            b')' => rights += 1,
            _ => (),
        };
    }
    let mut ret = Vec::new();
    if lefts < n {
        let mut s = Vec::new();
        s.extend(prefix);
        s.push(b'(');
        ret.extend(parens_with_prefix(n, &s));
    }
    if rights < lefts {
        let mut s = Vec::new();
        s.extend(prefix);
        s.push(b')');
        ret.extend(parens_with_prefix(n, &s));
    }
    if lefts == n && rights == n {
        let mut s = Vec::new();
        s.extend(prefix);
        unsafe {
            ret.push(String::from_utf8_unchecked(s));
        }
    }
    ret
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

}
