pub trait Union<Rhs = Self> {
    type Output;
    #[must_use]
    fn union(self, rhs: Rhs) -> Self::Output;
}

pub trait Intersection<Rhs = Self> {
    type Output;
    #[must_use]
    fn intersection(self, rhs: Rhs) -> Self::Output;
}
