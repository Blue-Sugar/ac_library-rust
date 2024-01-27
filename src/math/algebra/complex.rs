use std::ops::*;

const PI: f64 = std::f64::consts::PI;
#[allow(unused)]
#[derive(Clone, Copy)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

#[allow(unused)]
impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Complex {
            re: re,
            im: im,
        }
    }
}
#[allow(unused)]
impl Complex<f64> {
    pub fn arg(&self) -> f64 {
        let res = (self.im).atan2(self.re);
        (res + 2. * PI) % (2. * PI)
    }

    pub fn omega(n: isize) -> Self {
        let rad = 2.0 * std::f64::consts::PI / n as f64;
        Complex::new(rad.cos(), rad.sin())
    }
}

impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
impl<T: Sub<Output = T>> Sub for Complex<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}
impl<T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>> Mul for Complex<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}