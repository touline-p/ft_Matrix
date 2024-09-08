use super::{Field, Matrix, Vector};

impl<K, const X: usize, const Y: usize> Matrix<K, X, Y>
    where K: Field + PartialEq {
    pub fn to_row_echelon_form(&mut self) -> usize {
        let mut last_pivot: K = K::unity();
        let mut rank = 0;
        let mut index_x = 0;
        let mut index_y = 0;

        while index_x < X && index_y < Y {
            if self[index_y][index_x] == K::default() {
                for index in index_y..Y {
                    if self[index][index_x] != K::default() {
                        self.swap_line(index, index_y);
                        break ;
                    }
                }
            }
            if self[index_y][index_x] == K::default() {
                index_x += 1;
                continue ;
            }
            self.gaussian_pivot_elimination(index_x, index_y, last_pivot);
            last_pivot = self[index_y][index_x];
            rank += 1;
            index_x += 1;
            index_y += 1;
        }
        rank
    }
}

impl<K: Field, const X: usize, const Y: usize> Matrix<K, X, Y> {
    pub fn swap_line(&mut self, a: usize, b: usize) {
        let mut tmp: Vector<K, X> = self[a];
        self[a] = self[b];
        tmp.iter_mut().for_each(|x| *x = -*x);
        self[b] = tmp;
    }
}
pub fn swap<K: Field, const SIZE: usize>(a :&mut Vector<K, SIZE>, b: &mut Vector<K, SIZE>) {
    let mut tmp: Vector<K, SIZE> = *a;
    *a = *b;
    tmp.iter_mut().for_each(|x| *x = -*x);
    *b = tmp;
}

impl <K: Field, const X: usize, const Y: usize> Matrix<K, X, Y> {
    pub fn gaussian_pivot_elimination(
        &mut self,
        index_x: usize,
        index_y: usize,
        last_pivot: K) {
        let pivot_coef = self[index_y][index_x];
        for nulified in index_y..Y {
            let coef_self = self[nulified][index_x];
            if nulified == index_y {
                continue ;
            }
            (0..X).for_each(|index| {
                self[nulified][index] =
                    (pivot_coef * self[nulified][index]
                    - coef_self * self[index_y][index])
                    / last_pivot;
            });
        }

    }

    pub fn gaussian_pivot_elimination_inv(
        &mut self,
        index_x: usize,
        index_y: usize,
        last_pivot: K,
        inverse: &mut Matrix<K, X, Y>) {
        let pivot_coef = self[index_y][index_x];
        for nulified in 0..Y {
            let coef_self = self[nulified][index_x];
            if nulified == index_y {
                continue ;
            }
            (0..X).for_each(|index| {
                self[nulified][index] =
                    (pivot_coef * self[nulified][index]
                    - coef_self * self[index_y][index])
                    / last_pivot;
                inverse[nulified][index] =
                    (pivot_coef * inverse[nulified][index]
                    - coef_self * inverse[index_y][index])
                    / last_pivot;
            });
        }

    }
}
