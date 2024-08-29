use super::{Vector, Field};
use crate::traits::Abs;

impl<K: Field, const SIZE: usize> Vector<K, SIZE> {
    pub fn norm_1(&self) -> K
        where K: PartialOrd + Abs {
        self.iter().fold(K::default(), |acc, x| acc + x.abs())
    }

    pub fn norm_inf(&self) -> K
        where K: PartialOrd + Abs {
        self.iter()
            .map(|x| {x.abs()})
            .fold(
                K::default(),
                |acc, x| {
                    if x > acc {
                        x
                    } else {
                        acc
                    }
                }
            )
    }

    pub fn norm(&self) -> f64
        where K: Into<f64>
    {
        self.iter().fold(K::default(), |acc, x| {acc + *x * *x}).into().sqrt()
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vector;
    #[test]
    fn vector_null() {
        let u = Vector::from([0., 0., 0.]);
        let result = 0.;
        assert_eq!(u.norm_1(), result);
        assert_eq!(u.norm(), result);
        assert_eq!(u.norm_inf(), result);
    }
    #[test]
    fn positiv_vec() {
        let u = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_1(), 6.0);
        assert_eq!(u.norm(), 3.7416573867739414);
        assert_eq!(u.norm_inf(), 3.0);
    }
    #[test]
    fn negativ_vec() {
        let u = Vector::from([-1., -2.]);
        assert_eq!(u.norm_1(), 3.0);
        assert_eq!(u.norm(), 2.23606797749979);
        assert_eq!(u.norm_inf(), 2.0);
    }
}
