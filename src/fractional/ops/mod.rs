use std::ops::{Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign, Neg};

use super::Fraction;

use crate::traits::FractionField;

impl<K: FractionField> Neg for Fraction<K> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Fraction {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }
}

impl<K: FractionField> Add for Fraction<K> {
    type Output = Fraction<K>;
    fn add(self, rhs: Self) -> Self::Output {
        Fraction {
            numerator: self.numerator * rhs.denominator + rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        }.simplify()
    }
}

impl<K: FractionField> AddAssign for Fraction<K> {
    fn add_assign(&mut self, rhs: Self) {
        self.numerator = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        self.denominator = self.denominator * rhs.denominator;
        self.simplify();
    }
}
impl<K: FractionField> Sub for Fraction<K> {
    type Output = Fraction<K>;
    fn sub(self, rhs: Self) -> Self::Output {
        Fraction {
            numerator: self.numerator * rhs.denominator - rhs.numerator * self.denominator,
            denominator: self.denominator * rhs.denominator,
        }.simplify()
    }
}

impl<K: FractionField> SubAssign for Fraction<K> {
    fn sub_assign(&mut self, rhs: Self) {
        self.numerator = self.numerator * rhs.denominator - rhs.numerator * self.denominator;
        self.denominator = self.denominator * rhs.denominator;
        self.simplify();
    }
}

impl<K: FractionField> Mul for Fraction<K> {
    type Output = Fraction<K>;
    fn mul(self, rhs: Self) -> Self::Output {
        Fraction {
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
        }.simplify()
    }
}

impl<K: FractionField> MulAssign for Fraction<K> {
    fn mul_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.numerator;
        self.denominator *= rhs.denominator;
        self.simplify();
    }
}

impl<K: FractionField> Div for Fraction<K> {
    type Output = Fraction<K>;
    fn div(self, rhs: Self) -> Self::Output {
        Fraction {
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
        }.simplify()
    }
}

impl<K: FractionField> DivAssign for Fraction<K> {
    fn div_assign(&mut self, rhs: Self) {
        self.numerator *= rhs.denominator;
        self.denominator *= rhs.numerator;
        self.simplify();
    }
}
