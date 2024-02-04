#[allow(unused)]
pub struct WaveletMatrix {
    size: usize,
    mat: Vec<BitVector>,
    sum: Vec<Vec<u64>>,
    zeros: Vec<usize>,
}

#[allow(unused)]
impl WaveletMatrix {
    pub fn build(mut array: Vec<u64>, bits: usize) -> Self {
        assert!(0 < bits && bits <= 64);
        assert!(bits == 64 || array.iter().all(|p| *p < (1 << bits)));
        let size = array.len();
        let mut buf = vec![false; size];
        let mut tmp = [Vec::with_capacity(size), Vec::with_capacity(size)];
        let mut mat = Vec::with_capacity(bits);
        let mut sum = vec![];
        let mut zeros = Vec::with_capacity(bits);
        for i in (0..bits).rev() {
            for (buf, a) in buf.iter_mut().zip(array.iter()) {
                let x = ((*a >> i) & 1) == 1;
                *buf = x;
                tmp[x as usize].push(*a);
            }
            mat.push(BitVector::build(&buf));
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
        mat.reverse();
        sum.reverse();
        zeros.reverse();
        WaveletMatrix {
            size: size,
            mat: mat,
            sum,
            zeros: zeros,
        }
    }

    // k 番目へのアクセス
    pub fn access(&self, mut k: usize) -> u64 {
        assert!(k < self.size);
        let mut ans = 0;
        for (bits, zero) in self.mat.iter().zip(self.zeros.iter()).rev() {
            let x = bits.at(k);
            let c = bits.rank(k);
            ans = (ans << 1) | x as u64;
            if x {
                k = *zero + c;
            } else {
                k -= c;
            }
        }
        ans
    }

    // [0..k) からvalの個数を返す
    pub fn rank(&self, mut k: usize, val: u64) -> usize {
        assert!(k <= self.size);
        assert!(val < (1 << self.mat.len() as u64));
        let mut l = 0;
        for (i, (bits, zero)) in self.mat.iter().zip(self.zeros.iter()).enumerate().rev() {
            let l_cnt = bits.rank(l);
            let k_cnt = bits.rank(k);
            if (val >> i) & 1 == 1 {
                l = *zero + l_cnt;
                k = *zero + k_cnt;
            } else {
                l = l - l_cnt;
                k = k - k_cnt;
            }
        }
        k - l
    }

    // k 番目のvalの位置を返す
    pub fn select(&self, k: usize, val: u64) -> Option<usize> {
        if k > self.rank(self.size, val) {
            return None;
        }
        let mut ok = 0;
        let mut ng = self.size;
        while ng - ok > 1 {
            let mid = (ng + ok) / 2;
            if self.rank(mid, val) < k {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        Some(ok)
    }

    // [l..r) からk 未満の和を返す
    pub fn quantile(&self, mut l: usize, mut r: usize, a: u64) -> u64 {
        assert!(r <= self.size);
        assert!(l < r);
        let mut ans = 0;
        for (i, ((bits, zero), sum)) in self.mat.iter().zip(self.zeros.iter()).zip(self.sum.iter()).enumerate().rev() {
            let l_cnt = bits.rank(l);
            let r_cnt = bits.rank(r);
            if a >> i & 1 == 1 {
                ans += sum[l - l_cnt] - sum[r - r_cnt];
                l = *zero + l_cnt;
                r = *zero + r_cnt;
            } else {
                l -= l_cnt;
                r -= r_cnt;
            }
        }
        ans
    }

    // [l_i..r_i) からk番目に小さいものを返す
    // 区間が重なってる場合は重なった分だけ要素が複製されたとみなす感じ
    pub fn quantile_multi(&self, range: &[(usize, usize)], mut k: usize) -> u64 {
        assert!(range.iter().all(|&(l, r)| l < r && r <= self.size));
        assert!(k < range.iter().fold(0, |s, a| s + a.1 - a.0));
        let mut range = range.iter().map(|p| (p.0, p.1, 0, 0)).collect::<Vec<_>>();
        let mut ans = 0;
        for (i, (bits, zero)) in self.mat.iter().zip(self.zeros.iter()).enumerate().rev() {
            let mut z = 0;
            for p in range.iter_mut() {
                p.2 = bits.rank(p.0);
                p.3 = bits.rank(p.1);
                z += (p.1 - p.0) - (p.3 - p.2);
            }
            if z <= k {
                k -= z;
                ans |= 1 << i;
                for p in range.iter_mut() {
                    p.0 = *zero + p.2;
                    p.1 = *zero + p.3;
                }
            } else {
                for p in range.iter_mut() {
                    p.0 -= p.2;
                    p.1 -= p.3;
                }
            }
            range.retain(|p| p.0 < p.1);
        }
        ans
    }

    // [s, e) 中で v < x となるvの個数を返す
    pub fn range_freq(&self, mut l: usize, mut r: usize, x: u64) -> usize {
        assert!(l <= r);
        assert!(r <= self.size);
        if self.mat.len() < 64 && x >= 1 << self.mat.len() {
            return r - l;
        }
        let mut ans = 0;
        for (i, (bits, zero)) in self.mat.iter().zip(self.zeros.iter()).enumerate().rev() {
            let l_cnt = bits.rank(l);
            let r_cnt = bits.rank(r);
            let z_cnt = (r - l) - (r_cnt - l_cnt);
            if x >> i & 1 == 1 {
                l = *zero + l_cnt;
                r = *zero + r_cnt;
                ans += z_cnt;
            } else {
                l = l - l_cnt;
                r = r - r_cnt;
            }
        }
        ans
    }
}

const WIDTH: usize = std::mem::size_of::<usize>() * 8;

pub struct BitVector {
    size: usize,
    array: Vec<usize>,
    sum: Vec<u32>,
}

impl BitVector {
    pub fn build(array: &[bool]) -> Self {
        let size = array.len();
        let mut buf = vec![0usize; size / WIDTH + 1];
        for (buf, a) in buf.iter_mut().zip(array.chunks(WIDTH)) {
            for (i, a) in a.iter().enumerate() {
                *buf |= (*a as usize) << i;
            }
        }
        let mut sum = vec![0; size / WIDTH + 1];
        let mut acc = 0;
        for (sum, buf) in sum[1..].iter_mut().zip(buf.iter()) {
            acc += buf.count_ones();
            *sum = acc;
        }
        BitVector {
            size,
            array: buf,
            sum,
        }
    }
    fn quot_rem(n: usize) -> (usize, usize) {
        (n / WIDTH, n % WIDTH)
    }
    pub fn rank(&self, k: usize) -> usize {
        assert!(k <= self.size);
        let (q, r) = Self::quot_rem(k);
        unsafe {
            let a = *self.sum.get_unchecked(q);
            let b = (*self.array.get_unchecked(q) & ((1 << r) - 1)).count_ones();
            (a + b) as usize
        }
    }
    pub fn at(&self, k: usize) -> bool {
        assert!(k < self.size);
        let (q, r) = Self::quot_rem(k);
        unsafe {
            *self.array.get_unchecked(q) >> r & 1 == 1
        }
    }
}
