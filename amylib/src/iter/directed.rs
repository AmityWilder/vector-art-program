#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Forward,
    Reverse,
}

pub use Direction::{Forward, Reverse};

impl Direction {
    pub const fn rev(&self) -> Self {
        match self {
            Forward => Reverse,
            Reverse => Forward,
        }
    }

    pub const fn is_rev(self) -> bool {
        matches!(self, Reverse)
    }
}

pub struct Directed<I> {
    iter: I,
    dir: Direction,
}

impl<I> Directed<I> {
    fn new(iter: I, dir: Direction) -> Self {
        Self { iter, dir }
    }

    pub fn redirect(&mut self, dir: Direction) {
        self.dir = dir;
    }

    pub fn rev(&mut self) -> &mut Self {
        self.dir = self.dir.rev();
        self
    }
}

impl<I: DoubleEndedIterator> Iterator for Directed<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            Forward => self.iter.next(),
            Reverse => self.iter.next_back(),
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.iter.count()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.dir {
            Forward => self.iter.nth(n),
            Reverse => self.iter.nth_back(n),
        }
    }

    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        match self.dir {
            Forward => self.iter.fold(init, f),
            Reverse => self.iter.rfold(init, f),
        }
    }

    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        match self.dir {
            Forward => self.iter.find(predicate),
            Reverse => self.iter.rfind(predicate),
        }
    }
}

pub trait CDirection {
    type Iter<I: DoubleEndedIterator>: DoubleEndedIterator;
    const DIR: Direction;
    fn make<I: DoubleEndedIterator>(iter: I) -> Self::Iter<I>;
}

pub type CDirected<I, D: CDirection> = <D as CDirection>::Iter<I>;

pub struct CForward;
pub struct CReverse;

impl CDirection for CForward {
    type Iter<I: DoubleEndedIterator> = I;

    const DIR: Direction = Forward;

    #[inline]
    fn make<I: DoubleEndedIterator>(iter: I) -> Self::Iter<I> {
        iter
    }
}

impl CDirection for CReverse {
    type Iter<I: DoubleEndedIterator> = std::iter::Rev<I>;

    const DIR: Direction = Reverse;

    #[inline]
    fn make<I: DoubleEndedIterator>(iter: I) -> Self::Iter<I> {
        iter.rev()
    }
}

pub trait DirectibleDoubleEndedIterator: DoubleEndedIterator {
    #[inline]
    fn dir(self, dir: Direction) -> Directed<Self> where Self: Sized {
        Directed::new(self, dir)
    }

    #[inline]
    fn cdir<D: CDirection>(self) -> D::Iter<Self> where Self: Sized {
        D::make(self)
    }
}

impl<I: DoubleEndedIterator> DirectibleDoubleEndedIterator for I {}
