use super::{Matrix, Field};

impl<K, const X: usize, const Y: usize> Matrix<K, X, Y>
        where K: Field + PartialEq {
    pub fn rank(&self) -> usize {
        let mut copy = self.clone();
        copy.to_row_echelon_form()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rank() {
        let a: Matrix<i32,1,1> = [[0].into()].into();
        assert_eq!(0, a.rank());
        let a: Matrix<i32,2,2> = [
            [2, 1].into(),
            [1, 1].into(),
        ].into();
        assert_eq!(2, a.rank());
        let a: Matrix<i32,3,3> = [
            [2, 1, 3].into(),
            [2, 1, 5].into(),
            [2, 1, 5].into(),
        ].into();
        assert_eq!(2, a.rank());
    }
}
