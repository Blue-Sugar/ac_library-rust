const WIDTH: usize = std::mem::size_of::<usize>() * 8;

#[allow(unused)]
pub struct BitVector {
    size: usize,
    array: Vec<usize>,
    sum: Vec<u32>,
}

#[allow(unused)]
impl BitVector {
    pub fn build(a: &[bool]) -> Self {
        let size = a.len();
        let mut array = vec![0; size / WIDTH + 1];
        for (buf, a) in array.iter_mut().zip(a.chunks(WIDTH)) {
            for (i, a) in a.iter().enumerate() {
                *buf |= (*a as usize) << i;
            }
        }
        let mut sum = vec![0; size / WIDTH + 1];
        let mut acc = 0;
        for (sum, buf) in sum[1..].iter_mut().zip(array.iter()) {
            acc += buf.count_ones();
            *sum = acc;
        }
        BitVector {
            size: size, 
            array: array,
            sum: sum,
        }
    }

    fn pos(n: usize) -> (usize, usize) {
        (n / WIDTH, n % WIDTH)
    }

    pub fn rank(&self, k: usize) -> usize {
        assert!(k <= self.size);
        let (q, r) = Self::pos(k);
        (self.sum[q] + (self.array[q] & ((1 << r) - 1)).count_ones()) as usize
    }

    pub fn at(&self, k: usize) -> bool {
        assert!(k < self.size);
        let (q, r) = Self::pos(k);
        (self.array[q] >> r) & 1 == 1
    }
}