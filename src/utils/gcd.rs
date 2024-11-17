use std::{
    mem::swap,
    ops::{ShrAssign, SubAssign},
};

use num_traits::PrimInt;

pub trait Gcd
where
    Self: PrimInt + ShrAssign<u32> + SubAssign,
{
    /// NOTE: Exists for the internal LCM implementation.
    #[deprecated(note = "use crate::utils::gcd::Gcd::gcd() instead.")]
    #[must_use]
    fn __gcd_no_zero_check(mut self, mut other: Self) -> u32 {
        let k = {
            let i = self.trailing_zeros();
            let j = other.trailing_zeros();
            i.min(j)
        };

        while !other.is_zero() {
            if self > other {
                swap(&mut self, &mut other)
            }
            other -= self;
            other >>= other.trailing_zeros();
        }

        (self.to_u32().unwrap()) << k
    }
    fn gcd(self, other: Self) -> u32 {
        if self.is_zero() {
            return self.to_u32().unwrap();
        } else if other.is_zero() {
            return other.to_u32().unwrap();
        }
        #[allow(deprecated)]
        self.__gcd_no_zero_check(other)
    }
}

macro_rules! auto_impl_gcd {
    ($($x:ty),+ $(,)?) => ($(
        impl $crate::utils::gcd::Gcd for $x {}
    )*)
}

auto_impl_gcd!(i8, i16, i32, i128, isize, u8, u16, u32, u64, usize);
