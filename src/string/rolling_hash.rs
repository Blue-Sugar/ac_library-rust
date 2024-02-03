const MOD: usize = 999_999_937;
const BASE: usize = 3491;


#[allow(unused)]
struct RollingHash {
    s: Vec<u8>,
    prefix: Vec<FiniteField>,
    power: Vec<FiniteField>,
}

#[allow(unused)]
impl RollingHash {
    pub fn build(s: Vec<u8>) -> Self {
        let n = s.len();
        let mut prefix = vec![FiniteField::new(0); n + 1];
        let mut power = vec![FiniteField::new(1); n + 1];
        let base = FiniteField::new(BASE);
        for i in 0..n {
            let c = FiniteField::new((s[i] - b'a' + 1) as usize);
            prefix[i + 1] = prefix[i] * base + c;
            power[i + 1] = power[i] * base;
        }
        RollingHash {
            s: s,
            prefix: prefix,
            power: power,
        }
    }

    pub fn get(&self, l: usize, r: usize) -> FiniteField {
        self.prefix[r] - self.power[r - l] * self.prefix[l]
    }

    pub fn lcp(&self, l0: usize, r0: usize, l1: usize, r1: usize) -> usize {
        let mut ok = 0;
        let mut ng = (r0 - l0).min(r1 - l1) + 1;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if self.get(l0, l0 + mid) == self.get(l1, l1 + mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
}

use std::ops::*;
use std::cmp::*;

#[derive(Clone, Copy)]
#[allow(unused)]
struct FiniteField {
    value: usize,
}

#[allow(unused)]
impl FiniteField {
    pub fn new(x: usize) -> Self {
        FiniteField {
            value: x % MOD,
        }
    }

    pub fn inv(&self) -> FiniteField {
        let (x, mut y) = ex_euclid(MOD, self.value);
        y %= MOD as isize;
        if y < 0 {
            y += MOD as isize;
        }
        FiniteField::new(y as usize)
    }

    pub fn pow(&self, mut n: usize) -> FiniteField {
        let mut res = FiniteField::new(1);
        let mut a = self.clone();
        while n > 0 {
            if n % 2 == 1 {
                res *= a;
            }
            a *= a;
            n /= 2;
        }
        res
    }
}

impl Add for FiniteField {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        FiniteField::new(
            self.value + rhs.value,
        )
    }
}
impl AddAssign for FiniteField {
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        self.value %= MOD;
    }
}
impl Sub for FiniteField {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        FiniteField::new(
            MOD + self.value - rhs.value,
        )
    }
}
impl SubAssign for FiniteField {
    fn sub_assign(&mut self, rhs: Self) {
        self.value += MOD - rhs.value;
        self.value %= MOD;
    }
}
impl Mul for FiniteField {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        FiniteField::new(
            self.value * rhs.value
        )
    }
}
impl MulAssign for FiniteField {
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
        self.value %= MOD;
    }
}
impl Div for FiniteField {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}
impl DivAssign for FiniteField {
    fn div_assign(&mut self, rhs: Self) {
        self.value *= rhs.inv().value;
        self.value %= MOD;
    }
}
impl PartialEq for FiniteField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}
impl Eq for FiniteField {
    fn assert_receiver_is_total_eq(&self) {
    }
}

#[allow(unused)]
fn ex_euclid(mut a: usize, mut b: usize) -> (isize, isize) {
    let mut history = vec![];
    while b > 0 {
        history.push((a, b));
        (a, b) = (b, a % b);
    }
    let (mut x, mut y) = (1, 0);
    for &(a, b) in history.iter().rev() {
        (x, y) = (y, x - a as isize / b as isize * y);
    }
    (x, y)
}

