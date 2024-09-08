use super::{Fraction, FractionField};

impl<K: FractionField> Into<(K, K)> for Fraction<K> {
    fn into(self) -> (K, K) {
        (self.numerator, self.denominator)
    }
}

impl<K: FractionField> Into<Fraction<K>> for (K, K) {
    fn into(self) -> Fraction<K> {
        let (numerator, denominator) = self;
        Fraction {
            numerator,
            denominator,
        }
    }
}

impl<K: FractionField> From<K> for Fraction<K> {
    fn from(numerator: K) -> Self {
        Fraction {
            numerator,
            denominator: K::unity()
        }
    }
}
