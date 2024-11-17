use std::{
    mem::swap,
    ops::{ShrAssign, SubAssign},
};

use num_traits::PrimInt;

pub trait Gcd
where
    Self: PrimInt + ShrAssign<u32> + SubAssign,
{
    fn __gcd_no_zero_check(mut self, mut other: Self) -> Self {
        let k = {
            let i = self.trailing_zeros();
            let j = other.trailing_zeros();
            i.min(j) as usize
        };

        while !other.is_zero() {
            if self > other {
                swap(&mut self, &mut other)
            }
            other -= self;
            other >>= other.trailing_zeros();
        }

        self << k
    }
    fn gcd(self, other: Self) -> Self {
        if self.is_zero() {
            return self;
        } else if other.is_zero() {
            return other;
        }
        self.__gcd_no_zero_check(other)
    }
}

macro_rules! auto_impl_gcd {
    ($($x:ty),+ $(,)?) => ($(
        impl $crate::utils::gcd::Gcd for $x {}
    )*)
}

auto_impl_gcd!(i8, i16, i32, i128, isize, u8, u16, u32, u64, usize);
