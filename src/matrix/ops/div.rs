use std::ops::Div;

use crate::{traits::Field, matrix::Matrix};

impl<K, const X: usize, const Y: usize> Div<K> for Matrix<K, X, Y>
    where K: Field + Div<K>
          {
    type Output = Matrix<<K as Div<K>>::Output, X, Y>;

    fn div(self, rhs: K) -> Self::Output {
        self.iter().map(|x| *x / rhs).collect()
    }
}
