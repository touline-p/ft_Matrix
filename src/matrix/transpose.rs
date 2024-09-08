use super::{Field, Matrix, Vector};

impl<K: Field, const X: usize, const Y: usize> Matrix<K, X, Y> {
    pub fn transpose(&mut self) -> Matrix<K, Y, X> {
        (0..X).map(
            |index_x|
            (0..Y).map(
                |index_y|
                self[index_y][index_x]).collect::<Vector<K, Y>>()
        ).collect()
    }
}
