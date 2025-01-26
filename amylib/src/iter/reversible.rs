use std::num::NonZero;

pub trait ReversibleIterator: Iterator {
    fn prev(&mut self) -> Option<Self::Item>;

    #[inline]
    fn retreat_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        for i in 0..n {
            if self.prev().is_none() {
                return Err(unsafe { NonZero::new_unchecked(n - i) });
            }
        }
        Ok(())
    }

    #[inline]
    fn nth_prev(&mut self, n: usize) -> Option<Self::Item> {
        if self.retreat_by(n).is_err() {
            return None;
        }
        self.prev()
    }

    #[inline]
    fn pfold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.prev() {
            accum = f(accum, x);
        }
        accum
    }

    #[inline]
    fn pfind<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        while let Some(x) = self.prev() {
            if predicate(&x) {
                return Some(x);
            }
        }
        None
    }
}

impl<'a, I: ReversibleIterator + ?Sized> ReversibleIterator for &'a mut I {
    fn prev(&mut self) -> Option<I::Item> {
        (**self).prev()
    }
    fn retreat_by(&mut self, n: usize) -> Result<(), NonZero<usize>> {
        (**self).retreat_by(n)
    }
    fn nth_prev(&mut self, n: usize) -> Option<I::Item> {
        (**self).nth_prev(n)
    }
    fn pfold<B, F>(self, init: B, mut f: F) -> B
    where
        F: FnMut(B, Self::Item) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.prev() {
            accum = f(accum, x);
        }
        accum
    }
}
