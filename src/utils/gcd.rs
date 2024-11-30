use std::{
    mem::swap,
    ops::{ShrAssign, SubAssign},
};

use num_traits::PrimInt;

pub trait Gcd
where
    Self: PrimInt + ShrAssign<i8> + SubAssign,
{
    /// NOTE: Exists for the internal LCM implementation.
    #[deprecated(note = "use crate::utils::gcd::Gcd::gcd() instead.")]
    #[must_use]
    fn __gcd_no_zero_check(mut self, mut other: Self) -> Self {
        let k = {
            let i = self.trailing_zeros();
            let j = other.trailing_zeros();
            i.min(j) as usize
        };

        loop {
            if self > other {
                swap(&mut self, &mut other)
            }
            other -= self;

            if other.is_zero() {
                return self << k;
            }

            other >>= other.trailing_zeros() as i8;
        }
    }
    fn gcd(self, other: Self) -> Self {
        if self.is_zero() {
            return self;
        } else if other.is_zero() {
            return other;
        }
        #[allow(deprecated)]
        self.__gcd_no_zero_check(other)
    }
}

/// Implemented based on this implementation[1].
///
/// [1]: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
pub trait ExtendedGcd: Gcd {
    /// Returns: (Self::BezoutCoefficient, Self::Gcd)
    /// where:
    /// - Self::BezoutCoefficient = (Self, Self)
    /// - Self::Gcd = Self
    fn extended_gcd(self, other: Self) -> ((Self, Self), Self) {
        let (mut prev_r, mut r) = (self, other);
        let (mut prev_s, mut s) = (Self::one(), Self::zero());
        let (mut prev_t, mut t) = (Self::zero(), Self::one());

        let mut q;
        while r != Self::zero() {
            q = prev_r / r;
            (prev_r, r) = (r, prev_r - q * r);
            (prev_s, s) = (s, prev_s - q * s);
            (prev_t, t) = (t, prev_t - q * t);
        }

        ((prev_s, prev_t), prev_r)
    }
}

macro_rules! gcd_impl {
    ($name:ident for $($x:ty)*) => ($(
        impl $name for $x {}
    )*)
}

gcd_impl!(Gcd for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
gcd_impl!(ExtendedGcd for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
