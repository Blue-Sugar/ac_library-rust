#[allow(unused)]
#[derive(Clone, Copy, Debug)]
struct Rational {
    nu: usize,
    de: usize,
}

#[allow(unused)]
impl Rational {
    pub fn new(nu: usize, de: usize) -> Self {
        let d = gcd(nu, de);
        Rational { nu: nu / d, de: de / d }
    }
}

use std::ops::*;
use std::cmp::*;
impl Add for Rational {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let nu = self.de * rhs.nu + self.nu * rhs.de;
        let de = self.de * rhs.de;
        Rational::new(nu, de)
    }
}
impl Sub for Rational {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let nu = self.nu * rhs.de - self.de * rhs.nu;
        let de = self.de * rhs.de;
        Rational::new(nu, de)
    }
}
impl Mul for Rational {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let nu = self.nu * rhs.nu;
        let de = self.de * rhs.de;
        Rational::new(nu, de)
    }
}
impl Div for Rational {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let nu = self.nu * rhs.de;
        let de = self.de * rhs.nu;
        Rational::new(nu, de)
    }
}
impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        if self.nu == 0 && other.nu == 0 {
            return true;
        }
        self.nu == other.nu && self.de == other.de
    }
}
impl Eq for Rational {
    fn assert_receiver_is_total_eq(&self) {
        
    }
}
impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let l = self.nu * other.de;
        let r = self.de * other.nu;
        if l < r {
            Some(Ordering::Less)
        } else if l > r {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}
impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[allow(unused)]
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {return a;}
    gcd(b, a % b)
}