use std::cmp::{Ord, PartialEq};
use std::ops::Mul;

use super::Fraction;

impl<K: PartialEq + Mul<Output = K> + Clone> PartialEq for Fraction<K> {
    fn eq(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone() == self.denominator.clone() * other.numerator.clone()
    }

    fn ne(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone() != self.denominator.clone() * other.numerator.clone()
    }
}

impl<K: Eq + Mul<Output = K> + Clone> Eq for Fraction<K> {
}

impl<K: PartialOrd + Mul<Output = K> + Clone> PartialOrd for Fraction<K> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let first = self.numerator.clone() * other.denominator.clone();
        let second = self.denominator.clone() * other.numerator.clone();
        first.partial_cmp(&second)
    }
}

impl<K: Ord + Mul<Output = K> + Clone> Ord for Fraction<K> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let first = self.numerator.clone() * other.denominator.clone();
        let second = self.denominator.clone() * other.numerator.clone();
        first.cmp(&second)
    }
}
