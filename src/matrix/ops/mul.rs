use crate::vector::Vector;

use super::{Matrix, Field};
use std::{fmt::Display, ops::{Mul, MulAssign}};

impl<K: Field, const X: usize, const Y: usize> Mul<K> for Matrix<K, X, Y> {
    type Output = Self;
    fn mul(self, rhs: K) -> Self::Output {
        self.into_iter().map(|x| x * rhs).collect()
    }
}
impl<K: Field, const X: usize, const Y: usize> Mul<K> for &Matrix<K, X, Y> {
    type Output = Matrix<K, X, Y>;
    fn mul(self, rhs: K) -> Self::Output {
        self.into_iter().map(|x| *x * rhs).collect()
    }
}
impl<K: Field, const X: usize, const Y: usize> MulAssign<K> for Matrix<K, X, Y> {
    fn mul_assign(&mut self, rhs: K) {
        self.iter_mut().for_each(|x| *x *= rhs);
    }
}

impl<K: Field + Display + std::fmt::Debug, const X: usize, const Y: usize> Mul<Matrix<K, Y, X>> for Matrix<K, X, Y>
{
    type Output = Self;

    fn mul(self, rhs: Matrix<K,Y,X>) -> Self::Output {
        //self.iter().map(
            //|row|
            //row.iter().enumerate()
//
//
        //).collect()
        println!("first:\n{self:?}\nrhs:\n{rhs:?}");

        self.iter().map(
            |row|
            (0..X).map(|index| row.iter().zip(rhs.matrix_col_iter(index))
            .fold(K::default(), |acc, (x, y)| {println!("{x}, {y}\n");acc + *x * *y})).collect::<Vector<K, X>>()
        ).collect()

    }
}
//impl<K: Field, const X: usize, const Y: usize> Mul for &Matrix<K, X, Y> {
    //type Output = Matrix<K, X, Y>;
    //fn mul(self, rhs: Self) -> Self::Output {
    //}
//}
//impl<K: Field, const X: usize, const Y: usize> MulAssign for Matrix<K, X, Y> {
    //fn mul_assign(&mut self, rhs: Self) {
//
    //}
//}
//
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul_matrix_times_matrix() {
        let u: Matrix<f64, 2, 2> = [
            [1., 2.].into(),
            [3., 4.].into(),
        ].into();
        let v: Matrix<f64,2,2> = [
            [5., 6.].into(),
            [7., 8.].into(),
        ].into();
        assert_eq!(u * v, [
            [19., 22.].into(),
            [43., 50.].into(),
        ].into());
        let u :Matrix<_,2,2> = [
            [1., 0.].into(),
            [0., 1.].into(),
        ].into();
        let v : Matrix<_,2,2> = [
            [2., 1.].into(),
            [4., 2.].into(),
        ].into();
        assert_eq!(u * v, [
            [2., 1.].into(),
            [4., 2.].into(),
        ].into());
        //let u = Matrix::new([
            //[3., -5.],
            //[6., 8.],
        //]);
        //let v = Matrix::new([
            //[2., 1.],
            //[4., 2.],
        //]);
        //assert_eq!(&(u * &v), &Matrix::new([
            //[-14., -7.],
            //[44., 22.],
        //]));
    }
}

