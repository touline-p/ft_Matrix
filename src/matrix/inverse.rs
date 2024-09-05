use crate::traits::Unity;

use super::{Matrix, Field, Vector, MatrixResult};

impl<K, const X: usize> Matrix::<K, X, X>
    where K: Unity + PartialEq + Default + Field {
    pub fn inverse(&self) -> MatrixResult<Matrix<K, X, X>> {
        let mut dest_matrix: Matrix<K, X, X> = Matrix::identity();
        let mut copy = self.clone();


        let mut pivot:K = K::unity();

        for index_x in 0..X {
            let mut index_other = index_x + 1;
            while index_other < X && copy[index_x][index_x] == K::default() {
                let mut tmp: Vector<K, X> = copy[index_x];
                copy[index_x] = copy[index_other];
                tmp.iter_mut().for_each(|x| *x -= *x - *x - *x);
                copy[index_other] = tmp;

                let mut tmp: Vector<K, X> = dest_matrix[index_x];
                dest_matrix[index_x] = dest_matrix[index_other];
                tmp.iter_mut().for_each(|x| *x -= *x - *x - *x);
                dest_matrix[index_other] = tmp;
                index_other += 1;
            }

            let vector: Vector<K, X> = copy[index_x].clone();
            let vector_i: Vector<K, X> = dest_matrix[index_x].clone();
            for index_other in 0..X {
                if index_other == index_x {
                    continue ;
                }
                copy[index_other].nulify_index_inv(index_x, &vector, pivot, &mut dest_matrix[index_other], &vector_i);
            }
            pivot = copy[index_x][index_x].clone();
            println!("{pivot:?}");
            println!("{dest_matrix:?}");
        }

        // divide to make pivot 1
        //
        // substract line to all others
        dest_matrix = dest_matrix / pivot;


        Ok(dest_matrix)
    }



    pub fn identity() -> Self {
        (0..X).map(
            |x|
            (0..X).map(
                |y| if x == y { K::unity() } else { K::default() }
            ).collect())
        .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::Matrix;

    #[test]
    fn test_identity() {
        let id = Matrix::<i32, 1, 1>::identity();
        let ida : Matrix::<i32, 1, 1> = [[1].into()].into();
        assert_eq!(id, ida);
        let id = Matrix::<i32, 2, 2>::identity();
        let ida : Matrix::<i32, 2, 2> = [[1, 0].into(), [0, 1].into()].into();
        assert_eq!(id, ida);
    }

    #[test]
    fn test_inverse_of_complex_matrix() {
        let ida : Matrix::<f64, 2, 2> = [
            [1., 2.].into(),
            [2., 1.].into(),
        ].into();
        let inv = ida.inverse();
        println!("{inv:?}");
        let ida : Matrix::<f64, 3, 3> = [
            [1.,2.,1.].into(),
            [2.,1.,4.].into(),
            [6.,8.,13.].into(),
        ].into();
        let inv = ida.inverse();
        println!("{inv:?}");
    }

    #[test]
    fn test_inverse_of_id_is_id() {
        let id = Matrix::<i32, 1, 1>::identity();
        let ida : Matrix::<i32, 1, 1> = id.inverse().unwrap();
        assert_eq!(id, ida);
        let id = Matrix::<i32, 2, 2>::identity();
        let ida : Matrix::<i32, 2, 2> = id.inverse().unwrap();
        assert_eq!(id, ida);
        let id = Matrix::<i32, 3, 3>::identity();
        let ida : Matrix::<i32, 3, 3> = id.inverse().unwrap();
        assert_eq!(id, ida);
    }
}
