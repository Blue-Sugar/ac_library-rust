#[allow(unused)]
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

#[allow(unused)]
fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

#[allow(unused)]
fn is_coprime(a: usize, b: usize) -> bool {
    gcd(a, b) == 1
}

#[allow(unused)]
fn ex_euclid(x: isize, y: isize) -> (isize, isize) {
    if y == 0 {
        return (x.signum(), 0);
    }
    let q = x.div_euclid(y);
    let r = x.rem_euclid(y);
    let p = ex_euclid(y, r);
    (p.1, p.0 - q * p.1)
}

#[allow(unused)]
fn chinese_remainder_theorem(
    a0: usize,
    m0: isize,
    a1: usize,
    m1: isize,
) -> usize {
    let (x0, x1) = ex_euclid(m0, m1);
    let x = a0 as isize * m1 as isize * x1 
        + a1 as isize * m0 as isize * x0;
    let mut res = x % (m0 * m1) as isize;
    if res < 0 {
        res += (m0 * m1) as isize;
    }
    res as usize
}

#[allow(unused)]
fn is_square(x: usize) -> bool {
    let mut ok = 0;
    let mut ng = x + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if mid * mid <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok * ok == x
}

#[allow(unused)]
fn prime_factorization(mut n: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for i in 2.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            let mut cnt = 1;
            n = n / i;
            while n % i == 0 {
                cnt += 1;
                n = n / i;
            }
            res.push((i, cnt));
        }
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}