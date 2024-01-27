#[allow(unused)]
fn run_length_encoding<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}

use std::hash::Hash;
#[allow(unused)]
fn same_pair<T: Eq + Hash>(a: Vec<T>) -> usize {
    let mut res = 0;
    let mut h = std::collections::HashMap::new();
    for a in a.iter() {
        if h.contains_key(a) {
            res += h.get(a).unwrap();
            h.insert(a, h.get(a).unwrap() + 1);
        } else {
            h.insert(a, 1);
        }
    }
    res
}

#[allow(unused)]
fn is_palindrome<T: Eq + Clone>(s: Vec<T>) -> bool {
    let mut t = s.clone();
    t.reverse();
    s == t
}
