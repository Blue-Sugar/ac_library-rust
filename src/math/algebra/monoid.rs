#[allow(unused)]
pub trait Monoid: Clone + Copy {
    fn e() -> Self;
    fn op(&self, rhs: &Self) -> Self;
}
