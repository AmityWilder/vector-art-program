use std::{fmt::{self, Write}, ops::{Bound, RangeBounds}};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AnyRange<Idx> {
    start: Bound<Idx>,
    end: Bound<Idx>,
}

macro_rules! impl_from {
    (for<$T:ident> $($Range:ty),+ $(,)?) => {
        $(impl<$T: Copy> From<$Range> for AnyRange<$T> {
            fn from(value: $Range) -> Self {
                Self {
                    start: value.start_bound().map(|x| *x),
                    end:   value.  end_bound().map(|x| *x),
                }
            }
        })+
    };
}

impl_from!{ for<Idx>
    std::ops::Range<Idx>,
    std::ops::RangeFrom<Idx>,
    std::ops::RangeTo<Idx>,
    std::ops::RangeInclusive<Idx>,
    std::ops::RangeToInclusive<Idx>,
    std::ops::RangeFull,
}

impl<Idx: fmt::Debug> fmt::Debug for AnyRange<Idx> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_char(if matches!(&self.start, Bound::Included(_)) { '[' } else { '(' })?;

        if let Bound::Included(x) | Bound::Excluded(x) = &self.start {
            write!(fmt, "{x:?}")?;
        }

        fmt.write_str("..")?;

        if let Bound::Included(x) | Bound::Excluded(x) = &self.end {
            write!(fmt, "{x:?}")?;
        }

        fmt.write_char(if matches!(&self.end, Bound::Included(_)) { ']' } else { ')' })?;

        Ok(())
    }
}

impl<Idx> RangeBounds<Idx> for AnyRange<Idx> {
    fn start_bound(&self) -> Bound<&Idx> {
        self.start.as_ref()
    }

    fn end_bound(&self) -> Bound<&Idx> {
        self.end.as_ref()
    }
}
