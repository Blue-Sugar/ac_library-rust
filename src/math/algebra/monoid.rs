#[allow(unused)]
pub trait Monoid: Clone + Copy {
    // return identity element
    fn e() -> Self;
    // monoid operator
    fn op(&self, rhs: &Self) -> Self;
}
