use std::ops::Div;

use crate::vector::{Vector, Field};

impl<K, const SIZE: usize> Div<K> for Vector<K, SIZE>
    where K: Field + Div<K>
          {
    type Output = Vector<<K as Div<K>>::Output, SIZE>;

    fn div(self, rhs: K) -> Self::Output {
        self.iter().map(|x| *x / rhs).collect()
    }
}
