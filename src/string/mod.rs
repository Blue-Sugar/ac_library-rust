const INF: usize = std::usize::MAX / 2;
#[allow(unused)]
fn next(s: &Vec<u8>, from: u8, to: u8) -> Vec<Vec<usize>> {
    let mut res = vec![vec![INF; s.len() + 1]; (to - from + 1) as usize];
    for c in from..=to {
        let mut last = INF;
        for i in (0..s.len()).rev() {
            if s[i] == c {
                last = i;
            }
            res[(c - from) as usize][i] = last;
        }
    }
    res
}