use std::ops::{Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign};

use std::fmt::Debug;

#[rustfmt::skip]
pub trait Field:
    Default + Copy + Clone
        + Add<Output = Self> + AddAssign
        + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign
        + Div<Output = Self> + DivAssign
        + Unity
        + Debug
{
}

pub trait Unity {
    fn unity() -> Self;
}

macro_rules! impl_unity {
    ($($type:ident),*) => {
        $(impl Unity for $type {
            fn unity() -> Self {
                1 as Self
            }
        })*
    };
}



macro_rules! impl_field {
    ($($type:ident),*) => {
        $(impl Field for $type {})*
    };
}

macro_rules! impl_abs_unchanged {
    ($($type:ident),*) => {
        $(impl Abs for $type {
            fn abs(self) -> Self {
                self
            }
        })*
    };
}

macro_rules! impl_abs {
    ($($type:ident),*) => {
        $(impl Abs for $type {
            fn abs(self) -> Self {
                if self > Self::default() {
                    self
                } else {
                    -self
                }
            }
        })*
    };
}

impl_field!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
impl_unity!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);
impl_abs!(i8, i16, i32, i64, f32, f64);
impl_abs_unchanged!(u8, u16, u32, u64);

pub trait Abs: PartialOrd + Default {
    fn abs(self) -> Self;
}
