#[allow(unused)]
struct Eratosthenes {
    max_value: usize,
    is_prime: Vec<bool>,
    min_factor: Vec<isize>,
}

#[allow(unused)]
impl Eratosthenes {
    pub fn build(max_value: usize) -> Self {
        let mut is_prime = vec![true; max_value];
        let mut min_factor = vec![-1; max_value];
        is_prime[0] = false;
        is_prime[1] = true;
        min_factor[0] = 0;
        min_factor[1] = 1;

        for i in 2..=max_value {
            if !is_prime[i] {
                continue;
            }
            min_factor[i] = i as isize;
            for j in 2.. {
                if i * j > max_value {
                    break;
                }
                is_prime[i * j] = false;
                if min_factor[i * j] == -1 {
                    min_factor[i * j] = i as isize;
                }
            }
        }

        Eratosthenes {
            max_value: max_value,
            is_prime: is_prime,
            min_factor: min_factor,
        }
    }

    pub fn prime_factorization(&self, mut m: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];
        while m > 1 {
            let p = self.min_factor[m];
            let mut exp = 0;
            while self.min_factor[m] == p {
                m /= p as usize;
                exp += 1;
            }
            res.push((p as usize, exp));
        }
        res
    }

    pub fn dividors(&self, mut m: usize) -> Vec<usize> {
        let mut res = vec![1];
        let prime_factorization = self.prime_factorization(m);
        for &(p, exp) in &prime_factorization {
            for i in 0..res.len() {
                let mut v = 1;
                for j in 0..exp {
                    v *= p;
                    res.push(res[i] * v);
                }
            }
        }
        res.sort();
        res
    }
}