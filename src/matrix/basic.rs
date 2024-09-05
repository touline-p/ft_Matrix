use core::fmt;
use std::{fmt::Debug, ops::{Index, IndexMut}};

use crate::vector::Vector;

use super::{Field, Matrix};

impl<K, const X: usize, const Y: usize> Matrix<K, X, Y>
    where K: Field {
    pub fn new() -> Self {
        Matrix::default()
    }

    pub fn shape() -> (usize, usize) {
        (X, Y)
    }

    pub fn is_square() -> bool {
        X == Y
    }
}

impl<K: Field, const X: usize, const Y: usize> From<[Vector<K, X>; Y]> for Matrix<K, X, Y> {
    fn from(rows: [Vector<K, X>; Y]) -> Self {
        Matrix { rows }
    }
}

impl<K: Field + Copy, const X: usize, const Y: usize> Default for Matrix<K, X, Y> {
    fn default() -> Self {
        Matrix { rows: [Vector::<K, X>::default(); Y]}
    }
}

impl<K: Field, const X: usize, const Y: usize> Index<usize> for Matrix<K, X, Y> {
    type Output = Vector<K, X>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}

impl<K: Field, const X: usize, const Y: usize> IndexMut<usize> for Matrix<K, X, Y> {
    fn index_mut(&mut self, index: usize) -> &mut Vector<K, X> {
        &mut self.rows[index]
    }
}

impl<K: Field, const SQUARE_DIM: usize> Matrix<K, SQUARE_DIM, SQUARE_DIM> {
    pub fn trace(&mut self) -> K {
        (0..SQUARE_DIM).map(|index| self[index][index]).fold(K::default(), |acc, x| acc + x)
    }
}

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

impl<K: Field, const SQUARE_DIM: usize> Matrix<K, SQUARE_DIM, SQUARE_DIM>
    where K: Field + PartialEq + Debug {
    pub fn determinant(&self) -> K {
        let mut a: Matrix<K, SQUARE_DIM, SQUARE_DIM> = self.clone();
        a.to_row_echelon_form();
        println!("final\n{a:?}");
        a[SQUARE_DIM-1][SQUARE_DIM-1]
    }
}


impl<K, const X: usize, const Y: usize> Matrix<K, X, Y>
    where K: Field + PartialEq {
    pub fn to_row_echelon_form(&mut self) -> usize {
        let mut pivot: K = K::unity();
        let mut inv_rank: usize = 0;
        let mut index_x = 0;
        while index_x + inv_rank < X {
            let mut index_other = index_x + 1;
            while index_other < X && self[index_x][index_x + inv_rank] == K::default() {
                let mut tmp: Vector<K, X> = self[index_x];
                self[index_x] = self[index_other];
                tmp.iter_mut().for_each(|x| *x = *x - *x - *x);
                self[index_other] = tmp;
                index_other += 1;
            }
            while inv_rank + index_x < X && self[index_x][index_x + inv_rank] == K::default() {
                inv_rank += 1;
            }
            if index_x + inv_rank == X {
                break ;
            }
            let vector: Vector<K, X> = self[index_x + inv_rank].clone();
            for index_other in 0..Y {
                if index_other == index_x {
                    continue ;
                }
                self[index_other].nulify_index(index_x + inv_rank, &vector, pivot);
            }
            pivot = self[index_x][index_x + inv_rank].clone();
            index_x += 1;
        }
        X - inv_rank
    }
}

impl<K: Debug, const X: usize, const Y: usize> fmt::Debug for Matrix<K, X, Y> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n")?;
        for contained in self {
            write!(f, "{:?}", contained)?;
        }
        write!(f, "]\n")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn determinant_test() {
        let mut a: Matrix<f64, 2, 2> = [
            [1., 2.].into(),
            [3., 4.].into(),
        ].into();

        a.determinant();
    }

    #[test]
    fn trace_matrix() {
        let mut a: Matrix<f64, 2, 2> = [
            [1., 2.].into(),
            [3., 4.].into(),
        ].into();
        assert_eq!(a.trace(), 5.);
        let mut b: Matrix<f64,2,2> = [
            [5., 6.].into(),
            [7., 8.].into(),
        ].into();
        assert_eq!(b.trace(), 13.);
        let mut c :Matrix<_,2,2> = [
            [1., 0.].into(),
            [0., 1.].into(),
        ].into();
        assert_eq!(c.trace(), 2.);
        let mut d : Matrix<_,2,2> = [
            [2., 1.].into(),
            [4., 2.].into(),
        ].into();
        assert_eq!(d.trace(), 4.);
    }

    #[test]
    fn transpose() {
        let mut a: Matrix<f64, 2, 3> = [
            [1., 2.].into(),
            [3., 4.].into(),
            [5., 6.].into(),
        ].into();
        let b = [
            [1., 3., 5.].into(),
            [2., 4., 6.].into(),
        ].into();
        assert_eq!(a.transpose(), b);
    }


    #[test]
    fn row_echelon_form_complexer() {
        let mut a: Matrix<i32, 4,4> = [
            [ 2,-3, 1, 4].into(),
            [-1, 0, 2,-3].into(),
            [ 3, 1,-2, 1].into(),
            [ 0, 2, 1,-1].into()].into();
        println!("before\n{a:?}");
        a.determinant();
        println!("{a:?}");
        let result: i32 = -73;
        assert_eq!(result, a.determinant())
    }
}

/*
Matrice 2 :
[[-4  2  3  1]
 [ 1 -1  0  2]
 [ 2  3 -1  0]
 [ 0  1  2 -3]]
Déterminant : -35

Matrice 3 :
[[ 1  2  3  4]
 [ 2  4  6  8]
 [-1  0  2 -3]
 [ 3  1 -2  1]]
Déterminant : 0  # La deuxième ligne est le double de la première

Matrice 4 :
[[-2  1  3  -1]
 [ 4  -2  -6  2]
 [ 1  0  2  -3]
 [ 3  1 -2  1]]
Déterminant : 0  # La deuxième ligne est le double de la première ligne changé de signe

Matrice 5 :
[[ 1  2  3  4]
 [-1 -2 -3 -4]
 [ 2  1  4  3]
 [-2 -1 -4 -3]]
Déterminant : 0  # La deuxième ligne est l'opposée de la première, et la quatrième est l'opposée de la troisième
*/
