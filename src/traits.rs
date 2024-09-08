use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

pub trait FractionField:
    Field + Ord + Rem<Output = Self> + Neg<Output = Self>
{}

#[rustfmt::skip]
pub trait Field:
    Default + Copy + Clone
        + Add<Output = Self> + AddAssign
        + Sub<Output = Self> + SubAssign
        + Mul<Output = Self> + MulAssign
        + Div<Output = Self> + DivAssign
        + Unity
        + Neg<Output = Self>
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
impl_unity!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

macro_rules! impl_fraction_field {
    ($($type:ident),*) => {
        $(impl FractionField for $type {})*
    };
}
impl_fraction_field!(i8, i16, i32, i64);


macro_rules! impl_field {
    ($($type:ident),*) => {
        $(impl Field for $type {})*
    };
}
impl_field!(i8, i16, i32, i64, f32, f64);

macro_rules! impl_abs_unchanged {
    ($($type:ident),*) => {
        $(impl Abs for $type {
            fn abs(self) -> Self {
                self
            }
        })*
    };
}
impl_abs_unchanged!(u8, u16, u32, u64);

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
impl_abs!(i8, i16, i32, i64, f32, f64);


pub trait Abs: PartialOrd + Default {
    fn abs(self) -> Self;
}
