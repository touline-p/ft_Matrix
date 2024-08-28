use super::{Vector, Field};

fn cross_product<K>(u: &Vector<K, 3>, v: &Vector<K, 3>) -> Vector::<K, 3>
        where K: Field {
    u.cross(v)
}

impl<K: Field> Vector<K, 3> {
    pub fn cross(&self, other: &Vector<K, 3>) -> Vector<K, 3> {
        [
            K::default(),
            K::default(),
            K::default()
        ].into()
    }
}
