use std::fmt::Debug;

use super::Fraction;

impl<K: Debug> Debug for Fraction<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} / {:?}", self.numerator, self.denominator)
    }
}
