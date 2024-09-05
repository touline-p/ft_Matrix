use std::ops::IndexMut;

use super::Vector;
use crate::traits::Field;

use super::Matrix;

fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32, 4, 4> {
    Matrix::identity()
}

impl<K> Matrix<K, 3, 3>
        where K: Field + PartialEq,
            Vector<K, 3>: IndexMut<usize> {
    pub fn extend(self) -> Matrix<K, 4, 4> {
        let mut dest = Matrix::<K, 4, 4>::identity();
        self.into_iter()
        .zip(dest.iter_mut())
        .for_each(
            |(row_src, row_dst)|
            row_src.into_iter()
            .zip(row_dst.iter_mut())
            .for_each(
                |(src, dst)|
                *dst = src
            )
        );
        dest
    }
}

impl


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_default() {
        let m: Matrix<i32, 3, 3> = [
            [1, 0, 2].into(),
            [2, 3, 4].into(),
            [2, 3, 4].into(),
        ].into();
        let extended = m.extend();
        let cmp: Matrix<i32, 4, 4> = [
            [1, 0, 2, 0].into(),
            [2, 3, 4, 0].into(),
            [2, 3, 4, 0].into(),
            [0, 0, 0, 1].into(),
        ].into();
        assert_eq!(cmp, extended);

    }

}
