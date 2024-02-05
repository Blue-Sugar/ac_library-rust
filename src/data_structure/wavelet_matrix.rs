use crate::data_structure::bit_vector::BitVector;

#[allow(unused)]
struct WaveletMatrix {
    size: usize,
    matrix: Vec<BitVector>,
    sum: Vec<Vec<usize>>,
    zeros: Vec<usize>,
}

#[allow(unused)]
impl WaveletMatrix {
    pub fn build(array: &Vec<usize>, bits: usize) -> Self {
        assert!(0 < bits && bits < 64);
        assert!(array.iter().all(|p| *p < (1 << bits)));
        let size = array.len();
        let mut array = array.clone();
        let mut buf = vec![false; size];
        let mut tmp = [Vec::with_capacity(size), Vec::with_capacity(size)];
        let mut matrix = Vec::with_capacity(bits);
        let mut sum = vec![];
        let mut zeros = Vec::with_capacity(bits);
        for i in (0..bits).rev() {
            for (buf, a) in buf.iter_mut().zip(array.iter()) {
                let x = ((*a >> i) & 1) == 1;
                *buf = x;
                tmp[x as usize].push(*a);
            }
            matrix.push(BitVector::build(&buf));
            zeros.push(tmp[0].len());
            array.clear();
            array.extend(tmp[0].drain(..));
            array.extend(tmp[1].drain(..));
            let mut s = array.clone();
            s.push(0);
            for i in (1..s.len()).rev() {
                s[i - 1] += s[i];
            }
            sum.push(s);
        }
        matrix.reverse();
        sum.reverse();
        zeros.reverse();
        WaveletMatrix {
            size: size,
            matrix: matrix,
            sum: sum,
            zeros: zeros,
        }
    }

    pub fn at(&self, mut k:usize) -> usize {
        assert!(k < self.size);
        let mut res = 0;
        for (bits, zero) in self.matrix.iter().zip(self.zeros.iter()).rev() {
            let x = bits.at(k);
            let cnt = bits.rank(k);
            res = (res << 1) | x as usize;
            if x {
                k = zero + cnt;
            } else {
                k -= cnt;
            }
        }
        res
    }

    fn _rank(&self, mut r: usize, val: usize) -> usize {
        assert!(r <= self.size);
        assert!(val < (1 << self.matrix.len()));
        let mut l = 0;
        for (i, (bits, zero)) in self.matrix.iter().zip(self.zeros.iter()).enumerate().rev() {
            let l_cnt = bits.rank(l);
            let r_cnt = bits.rank(r);
            if (val >> i) & 1 == 1 {
                l = zero + l_cnt;
                r = zero + r_cnt;
            } else {
                l -= l_cnt;
                r -= r_cnt;
            }
        }
        r - l
    }

    pub fn rank_in_range(&self, l: usize, r: usize, val: usize) -> usize {
        assert!(l <= r);
        self._rank(r, val) - self._rank(l, val)
    }

    pub fn select(&self, k: usize, val: usize) -> Option<usize> {
        if k > self._rank(self.size, val) {
            return None;
        }
        let mut ok = 0;
        let mut ng = self.size;
        while ng - ok > 1 {
            let mid = (ok + ng) / 2;
            if self._rank(mid, val) < k {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        Some(ok)
    }

    pub fn sum_in_range_less_than_val(&self, mut l: usize, mut r: usize, val: usize) -> usize {
        assert!(r <= self.size);
        assert!(l <= r);
        let mut res = 0;
        for (i, ((bits, zero), sum)) in self.matrix.iter().zip(self.zeros.iter()).zip(self.sum.iter()).enumerate().rev() {
            let l_cnt = bits.rank(l);
            let r_cnt = bits.rank(r);
            if (val >> i) & 1 == 1 {
                res += sum[l - l_cnt] - sum[r - r_cnt];
                l = zero + l_cnt;
                r = zero + r_cnt;
            } else {
                l -= l_cnt;
                r -= r_cnt;
            }
        }
        res
    }

    pub fn cnt_in_range_less_than_val(&self, mut l: usize, mut r: usize, val: usize) -> usize {
        assert!(l <= r);
        assert!(r <= self.size);
        if val >= (1 << self.matrix.len()) {
            return r - l;
        }
        let mut res = 0;
        for (i, (bits, zero)) in self.matrix.iter().zip(self.zeros.iter()).enumerate().rev() {
            let l_cnt = bits.rank(l);
            let r_cnt = bits.rank(r);
            if (val >> i) & 1 == 1 {
                l = zero + l_cnt;
                r = zero + r_cnt;
                res += (r - l) - (r_cnt - l_cnt);
            } else {
                l -= l_cnt;
                r -= r_cnt;
            }
        }
        res
    }
}