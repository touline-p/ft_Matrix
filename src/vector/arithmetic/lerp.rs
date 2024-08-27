use core::f32;
use std::ops::Mul;

use super::Field;

pub fn lerp<V: Field + Mul<f32, Output = V>>(u: V, v: V, t: f32) -> V {
    u + (v - u) * t
}

#[cfg(test)]
mod test {
    use super::lerp;

    #[test]
    fn lerp_basic() {
        assert_eq!(0.0, lerp(0., 1., 0.));

        assert_eq!(1.0, lerp(0., 1., 1.));

        assert_eq!(0.5, lerp(0., 1., 0.5));

        assert_eq!(27.3, lerp(21., 42., 0.3));

//println!("{}", lerp(Vector::from([2., 1.]), Vector::from(4., 2.), 0.3));
// [2.6]
// [1.3]
//println!("{}", lerp(Matrix::from([[2., 1.], [3., 4.]]), Matrix::from([[20.,
//10.], [30., 40.]]), 0.5));
// [[11., 5.5]
// [16.5, 22.]]
    }
}
