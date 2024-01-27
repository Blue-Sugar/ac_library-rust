use crate::math::algebra::complex::*;

use itertools::Itertools;
use std::ops::*;

#[allow(unused)]
struct FPS<T> {
    coef: Vec<T>,
}

#[allow(unused)]
impl<T> FPS<T> {
    pub fn new(coef: Vec<T>) -> Self {
        FPS {
            coef: coef,
        }
    }
}
#[allow(unused)]
impl FPS<f64> {
    fn to_complex(&self) -> FPS<Complex<f64>> {
        let mut coef = vec![Complex::new(0.0, 0.0); self.coef.len()];
        for (coef, f) in coef.iter_mut().zip(&self.coef) {
            coef.re += f;
        }
        FPS::new(coef)
    }
}
#[allow(unused)]
impl FPS<Complex<f64>> {
    pub fn fft(&mut self, n: usize) {
        let k = n.next_power_of_two();
        for i in self.coef.len()..k {
            self.coef.push(Complex::new(0.0, 0.0));
        }
        if k == 1 {
            return;
        }
        let mut f0 = FPS::new(self.coef.clone().into_iter().step_by(2).collect_vec());
        let mut f1 = FPS::new(self.coef.clone().into_iter().skip(1).step_by(2).collect_vec());
        f0.fft(k / 2);
        f1.fft(k / 2);
        let omega = Complex::omega(k as isize);
        let mut pow_omega = Complex::new(1., 0.);
        for i in 0..k {
            self.coef[i] = f0.coef[i % (k / 2)] + pow_omega * f1.coef[i % (k / 2)];
            pow_omega = pow_omega * omega;
        }
    }

    pub fn inverse_fft(&mut self, n: usize) {
        let k = n.next_power_of_two();
        for i in self.coef.len()..k {
            self.coef.push(Complex::new(0.0, 0.0));
        }
        if k == 1 {
            return;
        }
        let mut f0 = FPS::new(self.coef.clone().into_iter().step_by(2).collect_vec());
        let mut f1 = FPS::new(self.coef.clone().into_iter().skip(1).step_by(2).collect_vec());
        f0.inverse_fft(k / 2);
        f1.inverse_fft(k / 2);
        let omega = Complex::omega(-1 * k as isize);
        let mut pow_omega = Complex::new(1., 0.);
        for i in 0..k {
            self.coef[i] = f0.coef[i % (k / 2)] + pow_omega * f1.coef[i % (k / 2)];
            pow_omega = pow_omega * omega;
        }
    }
}

trait Zero {
    fn e() -> Self;
}
impl<T: Copy + Add<Output = T> + Zero> Add for FPS<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let coef = self.coef.into_iter()
            .zip(rhs.coef.into_iter().chain(std::iter::repeat(T::e())))
            .map(|(f, g)| f + g)
            .collect_vec();
        FPS {
            coef: coef,
        }
    }
}
impl<T: Copy + Add<Output = T> + Sub<Output = T> + Zero> Sub for FPS<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let coef = self.coef.into_iter()
        .zip(rhs.coef.into_iter().chain(std::iter::repeat(T::e())))
        .map(|(f, g)| f - g)
        .collect_vec();
        FPS {
            coef: coef,
        }
    }
}
impl Mul for FPS<f64> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let n = self.coef.len() + rhs.coef.len() - 1;
        let mut f = self.to_complex();
        let mut g = rhs.to_complex();
        f.fft(n);
        g.fft(n);
        let coef = f.coef.iter().zip(&g.coef).map(
            |(f, g)| *f * *g
        ).collect_vec();
        let mut h = FPS::new(coef);
        h.inverse_fft(n);
        let mut res = vec![0.0; n];
        for (res, h) in res.iter_mut().zip(&h.coef) {
            *res = h.re / (n.next_power_of_two()) as f64;
        }
        Self::new(res)
    }
}