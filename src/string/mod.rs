#[allow(unused)]
fn is_palindrome<T: Eq + Clone>(s: Vec<T>) -> bool {
    let mut t = s.clone();
    t.reverse();
    s == t
}