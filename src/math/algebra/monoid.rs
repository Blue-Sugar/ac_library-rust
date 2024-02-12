#[allow(unused)]
pub trait Monoid: Clone + Copy {
    fn e() -> Self;
    fn op(&self, rhs: &Self) -> Self;
}
#[allow(unused)]
pub trait Act<S>: Clone + Copy {
    fn act(&self, x: &S) -> S;
}



#[allow(unused)]
impl Monoid for (isize, isize) {
    fn e() -> Self {
        (0, 1)
    }
    fn op(&self,rhs: &Self) -> Self {
        (self.0 + rhs.0, self.1 + rhs.1)
    }
}
#[derive(Clone, Copy)]
#[allow(unused)]
enum Action {
    UnitAction,
    UpdateAction(isize),
    AddAction(isize),
    MulAction(isize),
    AffineAction(isize, isize),
}
impl Monoid for Action {
    fn e() -> Self {
        Action::UnitAction
    }
    fn op(&self,rhs: &Self) -> Self {
        match self {
            Action::UnitAction => *rhs,
            Action::UpdateAction(u) => {
                match rhs {
                    Action::UnitAction => *self,
                    Action::UpdateAction(v) => Action::UpdateAction(*v),
                    Action::AddAction(b) => Action::UpdateAction(u + b),
                    Action::MulAction(b) => Action::UpdateAction(u * b),
                    Action::AffineAction(b, d) => Action::UpdateAction(u * b + d),
                }
            }
            Action::AddAction(a) => {
                match rhs {
                    Action::UnitAction => *self,
                    Action::UpdateAction(v) => Action::UpdateAction(*v),
                    Action::AddAction(b) => Action::AddAction(a + b),
                    Action::MulAction(b) => Action::AffineAction(*b, a * b),
                    Action::AffineAction(b, d) => Action::AffineAction(*b, a * b + d),
                }
            }
            Action::MulAction(a) => {
                match rhs {
                    Action::UnitAction => *self,
                    Action::UpdateAction(v) => Action::UpdateAction(*v),
                    Action::AddAction(b) => Action::AffineAction(*a, *b),
                    Action::MulAction(b) => Action::MulAction(a * b),
                    Action::AffineAction(b, d) => Action::AffineAction(a * b, *d),
                }
            }
            Action::AffineAction(a, c) => {
                match rhs {
                    Action::UnitAction => *self,
                    Action::UpdateAction(v) => Action::UpdateAction(*v),
                    Action::AddAction(b) => Action::AffineAction(*a, c + b),
                    Action::MulAction(b) => Action::AffineAction(a * b, b * c),
                    Action::AffineAction(b, d) => Action::AffineAction(a * b, c * b + d),
                }
            }
        }
    }
}
impl Act<(isize, isize)> for Action {
    fn act(&self, x: &(isize, isize)) -> (isize, isize) {
        match self {
            Action::UnitAction => *x,
            Action::UpdateAction(u) => (*u, x.1),
            Action::AddAction(a) => (x.0 + a * x.1, x.1),
            Action::MulAction(a) => (a * x.0, x.1),
            Action::AffineAction(a, c) => (a * x.0 + c * x.1, x.1),
        }
    }
}
