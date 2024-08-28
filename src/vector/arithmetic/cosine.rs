use super::{Vector, Field};

pub fn angle_cos<K: Field, const SIZE: usize>(u: &Vector<K, SIZE>, v: &Vector<K, SIZE>) -> f32
        where K: Into<f32> {
    u.dot(v).into() / (u.norm() * v.norm())
}


#[cfg(test)]
mod test {
    use super::angle_cos;
    #[test]
    fn vector_angle_droit() {
        let u = [0., 1.].into();
        let v = [1., 0.].into();
        assert_eq!(0., angle_cos(&u, &v));
    }

    #[test]
    fn vector_angle_plat() {
        let u = [0., 1.].into();
        let v = [0., -1.].into();
        assert_eq!(1., angle_cos(&u, &u));
        assert_eq!(-1., angle_cos(&u, &v));
    }
}
