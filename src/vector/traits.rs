use std::ops::{Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign};

#[rustfmt::skip]
pub trait Field:
    Default + Copy + Clone
        + Add<Output = Self> + AddAssign
        + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign
        + Div<Output = Self> + DivAssign
{
}

macro_rules! impl_field {
    ($($type:ident),*) => {
        $(impl Field for $type {})*
    };
}

impl_field!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

pub trait Scalar<T: Field>: Sized {
    fn multiplier(&self, other: T) -> T;
}


