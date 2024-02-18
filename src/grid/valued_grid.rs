#[allow(unused)]
pub fn transpose<T>(a: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if a.is_empty() {
        return a;
    }
    let h = a.len();
    let w = a[0].len();
    assert!(a.iter().all(|a| a.len() == w));
    let mut res = (0..w).map(|_| Vec::with_capacity(h)).collect::<Vec<Vec<T>>>();
    for a in a {
        for (res, a) in res.iter_mut().zip(a) {
            res.push(a);
        }
    }
    res
}