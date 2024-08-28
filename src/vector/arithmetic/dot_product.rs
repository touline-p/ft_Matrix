use super::{Vector, Field};

impl<K: Field, const SIZE: usize> Vector<K, SIZE>{
    pub fn dot(&self, v: Vector<K, SIZE>) -> K {
        self.iter().zip(v).fold(K::default(), |acc, (x, y)| { acc + *x * y})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product_empty_vectors() {
        let v1: Vector<i32, 0> = [].into();
        let v2: Vector<i32, 0> = [].into();
        assert_eq!(v1.dot(v2), 0);
    }

    #[test]
    fn test_dot_product_equal_length_vectors() {
        let v1: Vector<i32, 3> = [1, 2, 3].into();
        let v2: Vector<i32, 3> = [4, 5, 6].into();
        assert_eq!(v1.dot(v2), 32);
    }

    #[test]
    #[allow(unused)]
    fn test_dot_product_different_length_vectors() {
        let v1: Vector<i32, 2> = [1, 2].into();
        let v2: Vector<i32, 3> = [4, 5, 6].into();
        // Ici, vous pouvez choisir de paniquer ou de retourner une valeur par défaut
        // assert!(v1.dot(v2).is_err()); // Si vous paniquez
        // assert_eq!(v1.dot(v2), 0); // Si vous retournez 0 par défaut
    }

    #[test]
    fn test_dot_product_floating_point_numbers() {
        let v1: Vector<f64, 2> = [1.5, 2.5].into();
        let v2: Vector<f64, 2> = [3.0, 4.0].into();
        assert_eq!(v1.dot(v2), 14.5);
    }

    #[test]
    fn test_dot_product_negative_numbers() {
        let v1: Vector<i32, 3> = [1, -2, 3].into();
        let v2: Vector<i32, 3> = [-4, 5, -6].into();
        assert_eq!(v1.dot(v2), -32);
    }
}
