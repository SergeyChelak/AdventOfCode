pub trait CheckedOps {
    fn checked_add(self, rhs: Self) -> Option<Self>
    where
        Self: Sized;
    fn checked_sub(self, rhs: Self) -> Option<Self>
    where
        Self: Sized;
}

macro_rules! impl_checked_ops {
    ($($t:ty),*) => {
        $(
            impl CheckedOps for $t {
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    Self::checked_add(self, rhs)
                }
                fn checked_sub(self, rhs: Self) -> Option<Self> {
                    Self::checked_sub(self, rhs)
                }
            }
        )*
    };
}

impl_checked_ops!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
