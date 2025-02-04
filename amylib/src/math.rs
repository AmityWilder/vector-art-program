use std::ops::RangeInclusive;

pub trait As<T: ?Sized> {
    fn lossy_into(self) -> T;
}

impl<T> As<T> for T {
    #[inline(always)]
    fn lossy_into(self) -> T {
        self
    }
}

macro_rules! impl_as {
    ($($Self:ident => $($Into:ident),+;)+) => {
        $($(impl As<$Into> for $Self {
            #[inline(always)]
            fn lossy_into(self) -> $Into {
                self as $Into
            }
        })+)+
    };

    ($($Self:ident <=> $($Into:ident),+;)+) => {
        impl_as!{
            $($Self => $($Into),+;)+
            $($($Into => $Self;)+)+
        }
    };

    ($Self:ident <=>> $($First:ident $(, $Rest:ident)*)?;) => {
        $(
            impl_as!{ $Self <=> $First $(, $Rest)*; }
            impl_as!{ $First <=>> $($Rest),*; }
        )?
    };
}

impl_as!{ f32 <=>> f64, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128; }

pub trait Numeric: PartialOrd {
    const ZERO: Self;
    const MIN: Self;
    const MAX: Self;
}

macro_rules! impl_numeric {
    ($($T:ty),+ $(,)?) => {
        $(impl Numeric for $T {
            const ZERO: $T = 0 as $T;
            const MIN: $T = <$T>::MIN;
            const MAX: $T = <$T>::MAX;
        })+
    };
}

impl_numeric!{ f32, f64, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128 }

pub trait FloatEquals {
    /// Check whether two given floats are almost equal
    fn near_eq(self, rhs: Self) -> bool;
}

impl FloatEquals for f32 {
    fn near_eq(self, rhs: Self) -> bool {
        (self - rhs).abs() <= (Self::EPSILON * self.abs().max(rhs.abs().max(1.0)))
    }
}

impl FloatEquals for f64 {
    fn near_eq(self, rhs: Self) -> bool {
        (self - rhs).abs() <= (Self::EPSILON * self.abs().max(rhs.abs().max(1.0)))
    }
}

pub trait CanRepresent<T: Copy> {
    /// Returns true if `value` is within numeric bounds of `Self`.
    fn can_contain(value: T) -> bool;

    /// Returns true if `value` can be losslessly represented with `Self`.
    fn can_represent(value: T) -> bool;
}

impl<T: Copy> CanRepresent<T> for T {
    #[inline(always)]
    fn can_contain(_: T) -> bool {
        true
    }

    #[inline(always)]
    fn can_represent(_: T) -> bool {
        true
    }
}

macro_rules! impl_can_rep {
    ($($T:ident -> $Self:ident {
        min = $MIN:expr,
        max = $MAX:expr,
        $(step = $step:literal,)?
    })+) => {
        $(impl CanRepresent<$T> for $Self {
            #[deny(clippy::cast_precision_loss)]
            fn can_contain(value: $T) -> bool {
                const MIN: $T = $MIN;
                const MAX: $T = $MAX;
                value.is_normal() && MIN <= value && value <= MAX
            }

            fn can_represent(value: $T) -> bool {
                Self::can_contain(value)$( && $step && value.near_eq(value.round()))?
            }
        })+
    };
}

impl_can_rep!{
    f32 -> u8 {
        min = 0.0,
        max = u8::MAX as f32,
        step = true,
    }
    f32 -> u16 {
        min = 0.0,
        max = u16::MAX as f32,
        step = true,
    }
    f32 -> u32 {
        min = 0.0,
        max = u32::MAX as f32,
        step = true,
    }
    f32 -> u64 {
        min = 0.0,
        max = u64::MAX as f32,
        step = true,
    }
    f32 -> i8 {
        min = i8::MIN as f32,
        max = i8::MAX as f32,
        step = true,
    }
    f32 -> i16 {
        min = i16::MIN as f32,
        max = i16::MAX as f32,
        step = true,
    }
    f32 -> i32 {
        min = i32::MIN as f32,
        max = i32::MAX as f32,
        step = true,
    }
    f32 -> i64 {
        min = i64::MIN as f32,
        max = i64::MAX as f32,
        step = true,
    }

    f64 -> u8 {
        min = 0.0,
        max = u8::MAX as f64,
        step = true,
    }
    f64 -> u16 {
        min = 0.0,
        max = u16::MAX as f64,
        step = true,
    }
    f64 -> u32 {
        min = 0.0,
        max = u32::MAX as f64,
        step = true,
    }
    f64 -> u64 {
        min = 0.0,
        max = u64::MAX as f64,
        step = true,
    }
    f64 -> i8 {
        min = i8::MIN as f64,
        max = i8::MAX as f64,
        step = true,
    }
    f64 -> i16 {
        min = i16::MIN as f64,
        max = i16::MAX as f64,
        step = true,
    }
    f64 -> i32 {
        min = i32::MIN as f64,
        max = i32::MAX as f64,
        step = true,
    }
    f64 -> i64 {
        min = i64::MIN as f64,
        max = i64::MAX as f64,
        step = true,
    }
}

pub trait RoundToInt: Numeric {
    /// Returns the integer part of `self` and casts.
    /// This means that non-integer numbers are always truncated towards zero.
    ///
    /// Returns None if result cannot be expressed as `T`.
    fn checked_trunci<T: Numeric + CanRepresent<Self>>(self) -> Option<T> where Self: As<T> + Copy;

    /// Returns the largest integer less than or equal to `self` and casts.
    ///
    /// Returns None if result cannot be expressed as `T`.
    fn checked_floori<T: Numeric + CanRepresent<Self>>(self) -> Option<T> where Self: As<T> + Copy;

    /// Returns the smallest integer greater than or equal to `self` and casts.
    ///
    /// Returns None if result cannot be expressed as `T`.
    fn checked_ceili<T: Numeric + CanRepresent<Self>>(self) -> Option<T> where Self: As<T> + Copy;

    /// Returns the nearest integer to `self` and casts. If a value is half-way between two
    /// integers, round away from `0.0`.
    ///
    /// Returns None if result cannot be expressed as `T`.
    fn checked_roundi<T: Numeric + CanRepresent<Self>>(self) -> Option<T> where Self: As<T> + Copy;

    /// Returns the nearest integer to a `self` and casts. Rounds half-way cases to the `self`
    /// with an even least significant digit.
    ///
    /// Returns None if result cannot be expressed as `T`.
    fn checked_round_ties_eveni<T: Numeric + CanRepresent<Self>>(self) -> Option<T> where Self: As<T> + Copy;

    /// Returns the integer part of `self` and casts.
    /// This means that non-integer numbers are always truncated towards zero.
    fn trunci<T: Numeric>(self) -> T where Self: As<T>;

    /// Returns the largest integer less than or equal to `self` and casts.
    fn floori<T: Numeric>(self) -> T where Self: As<T>;

    /// Returns the smallest integer greater than or equal to `self` and casts.
    fn ceili<T: Numeric>(self) -> T where Self: As<T>;

    /// Returns the nearest integer to `self` and casts. If a value is half-way between two
    /// integers, round away from `0.0`.
    fn roundi<T: Numeric>(self) -> T where Self: As<T>;

    /// Returns the nearest integer to a `self` and casts. Rounds half-way cases to the `self`
    /// with an even least significant digit.
    fn round_ties_eveni<T: Numeric>(self) -> T where Self: As<T>;
}

impl RoundToInt for f32 {
    fn checked_trunci<T: Numeric + CanRepresent<Self>>(self) -> Option<T> where Self: As<T> + Copy {
        let x = self.trunc();
        T::can_contain(x) // dont need full "can_represent" because we are truncating anyway
            .then(|| x.lossy_into())
    }

    fn checked_floori<T: Numeric + CanRepresent<Self>>(self) -> Option<T> where Self: As<T> + Copy {
        let x = self.floor();
        T::can_contain(x) // dont need full "can_represent" because we are truncating anyway
            .then(|| x.lossy_into())
    }

    fn checked_ceili<T: Numeric + CanRepresent<Self>>(self) -> Option<T> where Self: As<T> + Copy {
        let x = self.ceil();
        T::can_contain(x) // dont need full "can_represent" because we are truncating anyway
            .then(|| x.lossy_into())
    }

    fn checked_roundi<T: Numeric + CanRepresent<Self>>(self) -> Option<T> where Self: As<T> + Copy {
        let x = self.round();
        T::can_contain(x) // dont need full "can_represent" because we are truncating anyway
            .then(|| x.lossy_into())
    }

    fn checked_round_ties_eveni<T: Numeric + CanRepresent<Self>>(self) -> Option<T> where Self: As<T> + Copy {
        let x = self.round_ties_even();
        T::can_contain(x) // dont need full "can_represent" because we are truncating anyway
            .then(|| x.lossy_into())
    }

    fn trunci<T: Numeric>(self) -> T where Self: As<T> {
        self.trunc().lossy_into()
    }

    fn floori<T: Numeric>(self) -> T where Self: As<T> {
        self.floor().lossy_into()
    }

    fn ceili<T: Numeric>(self) -> T where Self: As<T> {
        self.ceil().lossy_into()
    }

    fn roundi<T: Numeric>(self) -> T where Self: As<T> {
        self.round().lossy_into()
    }

    fn round_ties_eveni<T: Numeric>(self) -> T where Self: As<T> {
        self.round_ties_even().lossy_into()
    }
}

pub trait Remap<T> {
    /// The tolerance within which `T` can be expected to unmap
    /// accurately back to `self`.
    const UNMAP_EPSILON: Self;

    /// Returns `self` linearly interpolated so that a `self` equal to
    /// `range_in.start()` becomes `range_out.start()` and a `self`
    /// equal to `range_in.end()` becomes `range_out.end()`.
    fn remap(self, range_in: RangeInclusive<Self>, range_out: RangeInclusive<T>) -> T where Self: Sized;

    /// Returns `self` linearly interpolated so that a `self`
    /// equal to 0.0 becomes 0 and a `self` equal to 1.0 becomes `T::MAX`.
    ///
    /// `self` is clamped to 0.0..=1.0 before remapping.
    ///
    /// If `T` and `Self` are both signed, `self` will be clamped to
    /// -1.0..=1.0 instead and -1.0 will become `T::MIN`.
    fn saturating_remap(self) -> T;

    fn unmap(value: T, range_out: RangeInclusive<T>, range_in: RangeInclusive<Self>) -> Self where Self: Sized;

    fn saturating_unmap(value: T) -> Self;
}

macro_rules! impl_remap {
    {
        for $Self:ty => $Into:ty;
        epsilon = $EPSILON:expr;
        saturate: ($MIN_IN:expr, $MAX_IN:expr) => ($MIN_OUT:expr, $MAX_OUT:expr);
        mod $tests:ident { $ROSTER:expr }
    } => {
        impl Remap<$Into> for $Self {
            const UNMAP_EPSILON: $Self = $EPSILON;

            #[inline]
            fn remap(self, range_in: RangeInclusive<$Self>, range_out: RangeInclusive<$Into>) -> $Into where Self: Sized {
                let range_out_min = range_out.start();
                let size_in  = range_in .end() - range_in.start();
                let size_out = range_out.end() - range_out_min;
                range_out_min + ((self / size_in) * size_out as $Self) as $Into
            }

            #[inline]
            fn unmap(value: $Into, range_out: RangeInclusive<$Into>, range_in: RangeInclusive<$Self>) -> $Self where Self: Sized {
                let range_out_min = range_out.start();
                let size_in  = range_in .end() - range_in.start();
                let size_out = range_out.end() - range_out_min;
                ((value - range_out_min) as $Self / size_out as $Self) * size_in
            }

            #[inline]
            fn saturating_remap(self) -> $Into {
                const MIN_IN: $Self = $MIN_IN;
                const MAX_IN: $Self = $MAX_IN;
                const MAX_OUT: $Self = $MAX_OUT as $Self;
                (self.clamp(MIN_IN, MAX_IN) * MAX_OUT) as $Into
            }

            #[inline]
            fn saturating_unmap(value: $Into) -> $Self {
                const MAX_OUT: $Self = $MAX_OUT as $Self;
                (value as $Self) / MAX_OUT
            }
        }

        #[cfg(test)]
        mod $tests {
            use super::*;

            #[test]
            fn test_saturating_remap() {
                for pair in $ROSTER {
                    let (t, u): ($Self, $Into) = pair;
                    let remapped: $Into = (t).saturating_remap();
                    assert_eq!(
                        remapped,
                        u,
                        concat!(stringify!($Self), " should remap accurately to ", stringify!($Into)),
                    );
                }
            }

            #[test]
            fn test_saturating_unmap() {
                for pair in $ROSTER {
                    let (t, u): ($Self, $Into) = pair;
                    let unmapped: $Self = <$Self>::saturating_unmap(u);
                    let (min, max) = (t - <$Self as Remap<$Into>>::UNMAP_EPSILON, t + <$Self as Remap<$Into>>::UNMAP_EPSILON);
                    assert!(
                        min <= unmapped && unmapped <= max,
                        concat!(stringify!($Self), " should unmap accurately from ", stringify!($Into), " within \u{00B1}{:?}\n expected {:?} \u{2264} {:?} \u{2264} {:?}"),
                        <$Self as Remap<$Into>>::UNMAP_EPSILON, min, unmapped, max,
                    );
                }
            }
        }
    };
}

impl_remap!{
    for f32 => u8;
    epsilon = 1.0 / u8::MAX as f32;
    saturate: (0.0f32, 1.0f32) => (0u8, u8::MAX);
    mod test_remap_f32_u8 {
        (0..=255).map(|x| (x as f32 / 255.0, x))
    }
}
