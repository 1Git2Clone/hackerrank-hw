use crate::utils::gcd::Gcd;

pub trait Lcm: Gcd {
    fn lcm(self, other: Self) -> Self {
        if self.is_zero() {
            return self;
        } else if other.is_zero() {
            return other;
        }
        #[allow(deprecated)] // Internal call to save second zero checks.
        {
            (self * other) / (self.__gcd_no_zero_check(other))
        }
    }
}

macro_rules! lcm_impl {
    ($name:ident for $($x:ty)*) => ($(
        impl $name for $x {}
    )*)
}

lcm_impl!(Lcm for i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);
