use crate::traits::{Field, FractionField};
use crate::function::pgcd;

pub mod into;
pub mod ops;
pub mod debug;
pub mod numeric_value;
pub mod cmp;

impl<K: FractionField> Field for Fraction<K> {}

#[derive(Copy, Clone)]
pub struct Fraction<K> {
    numerator: K,
    denominator: K,
}


impl<K: FractionField> Fraction<K> {
    fn simplify(self) -> Self {
        let (numerator, denominator) = self.into();
        let pgcd = pgcd(numerator.clone(), denominator.clone());
        if pgcd == K::default() {
            Fraction {
                numerator,
                denominator
            }
        } else {
            Fraction {
                numerator: numerator / pgcd,
                denominator: denominator / pgcd
            }
        }
    }
}

