use itertools::Itertools;

#[allow(unused)]
struct Grid {
    h: usize,
    w: usize,
}

#[allow(unused)]
impl Grid {
    pub fn build(h: usize, w: usize) -> Self {
        Grid {
            h: h,
            w: w,
        }
    }

    pub fn neighbor4(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let d = [(1, 0), (!0, 0), (0, 1), (0, !0)];
        d.iter().map(
            |&(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy))
        ).filter(
            |&(x, y)| x < self.h && y < self.w
        ).collect_vec()
    }

    pub fn neighbor8(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let d = [(1, 0), (!0, 0), (0, 1), (0, !0),
            (1, 1), (1, !0), (!0, 1), (!0, !0)];
        d.iter().map(
            |&(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy))
        ).filter(
            |&(x, y)| x < self.h && y < self.w
        ).collect_vec()
    }
}