use crate::traits::Unity;

use super::{Matrix, Field, MatrixResult};

impl<K, const X: usize> Matrix::<K, X, X>
    where K: Unity + PartialEq + Default + Field {
    pub fn inverse(mut self) -> MatrixResult<Matrix<K, X, X>> {
        let mut dest_matrix: Self = Matrix::identity();
        let mut index_x = 0;
        let mut last_pivot:K = K::unity();

        while index_x < X {
            if self[index_x][index_x] == K::default() {
                for index in index_x..X {
                    if self[index][index_x] != K::default() {
                        self.swap_line(index, index_x);
                        dest_matrix.swap_line(index, index_x);
                        break ;
                    }
                }
            }
            if self[index_x][index_x] == K::default() {
                todo!();
            }
            self.gaussian_pivot_elimination_inv(index_x, index_x, last_pivot, &mut dest_matrix);
            last_pivot = self[index_x][index_x];
            index_x += 1;
        }
        dest_matrix = dest_matrix / last_pivot;
        Ok(dest_matrix)
    }
}


#[cfg(test)]
mod test {
    use crate::{fractional::Fraction, matrix::Matrix};

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
        assert!(false);
    }

    #[test]
    fn test_inverse_of_id_is_id() {
        let id = Matrix::<i32, 1, 1>::identity();
        let ida : Matrix::<i32, 1, 1> = id.clone().inverse().unwrap();
        assert_eq!(id, ida);
        let id = Matrix::<i32, 2, 2>::identity();
        let ida : Matrix::<i32, 2, 2> = id.clone().inverse().unwrap();
        assert_eq!(id, ida);
        let id = Matrix::<i32, 3, 3>::identity();
        let ida : Matrix::<i32, 3, 3> = id.clone().inverse().unwrap();
        assert_eq!(id, ida);
    }

    #[test]
    fn test_inverse_fractional() {
        let first_one : Matrix::<Fraction<i32>, 2, 2> = [
            [Fraction::from(1), Fraction::from(2)].into(),
            [Fraction::from(2), Fraction::from(1)].into(),
        ].into();
        println!("{:?}", first_one);
        if let Ok(calculated_inv) = first_one.inverse() {
            let hard_code : Matrix::<Fraction<i32>, 2, 2> = [
                [(-1, 3).into(), (2, 3).into()].into(),
                [(2, 3).into(), (-1, 3).into()].into(),
            ].into();

            assert_eq!(calculated_inv, hard_code);
            println!("{:?}", calculated_inv);
            assert!(false);
            return
        } else {
            assert!(false)
        }

    }

}
