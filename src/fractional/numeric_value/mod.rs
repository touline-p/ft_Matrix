use crate::traits::Unity;

use super::Fraction;

impl<K: Default + Unity> Default for Fraction<K> {
    fn default() -> Self {
        Fraction {
            numerator: K::default(),
            denominator: K::unity(),
        }
    }
}

impl<K: Default + Unity> Unity for Fraction<K> {
    fn unity() -> Self {
        Fraction {
            numerator: K::unity(),
            denominator: K::unity(),
        }
    }
}
