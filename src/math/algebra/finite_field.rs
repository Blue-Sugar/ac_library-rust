use std::ops::*;

const _MOD_7: usize = 1_000_000_007;
const _MOD_3: usize = 1_000_000_003;
const MOD: usize = 998_244_353;

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

