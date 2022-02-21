use crate::*;

impl<T: Default + Copy, const ROWS: usize, const COLUMNS: usize> MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
    [(); COLUMNS * ROWS]:,
{
    /// Returns the transposition of `self` as a new matrix.
    /// ```
    /// use static_la::MatrixSxS;
    /// let a = MatrixSxS::<i32,2,3>::from([[1,2,3],[4,5,6]]);
    /// assert_eq!(a.transpose(),MatrixSxS::<i32,3,2>::from([[1,4],[2,5],[3,6]]));
    /// ```
    pub fn transpose(&self) -> MatrixSxS<T, COLUMNS, ROWS> {
        let mut data = [Default::default(); COLUMNS * ROWS];
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                data[j * ROWS + i] = self.data[i * COLUMNS + j].clone();
            }
        }
        MatrixSxS { data }
    }
}
impl<T: Default + Clone, const ROWS: usize> MatrixSxD<T, ROWS> {
    /// Returns the transposition of `self` as a new matrix.
    /// ```
    /// use static_la::*;
    /// use std::convert::TryFrom;
    /// let a = MatrixSxD::<i32,2>::try_from([vec![1,2,3],vec![4,5,6]]).unwrap();
    /// assert_eq!(a.transpose(),MatrixDxS::<i32,2>::from(vec![[1,4],[2,5],[3,6]]));
    /// ```
    pub fn transpose(&self) -> MatrixDxS<T, ROWS> {
        let mut data = vec![Default::default(); ROWS * self.columns];
        for i in 0..ROWS {
            for j in 0..self.columns {
                data[j * ROWS + i] = self.data[i * self.columns + j].clone();
            }
        }
        MatrixDxS {
            data,
            rows: self.columns,
        }
    }
}
impl<T: Default + Clone, const COLUMNS: usize> MatrixDxS<T, COLUMNS> {
    /// Returns the transposition of `self` as a new matrix.
    /// ```
    /// use static_la::*;
    /// use std::convert::TryFrom;
    /// let a = MatrixDxS::<i32,3>::from(vec![[1,2,3],[4,5,6]]);
    /// assert_eq!(a.transpose(),MatrixSxD::<i32,3>::try_from([vec![1,4],vec![2,5],vec![3,6]]).unwrap());
    /// ```
    pub fn transpose(&self) -> MatrixSxD<T, COLUMNS> {
        let mut data = vec![Default::default(); self.rows * COLUMNS];
        for i in 0..self.rows {
            for j in 0..COLUMNS {
                data[j * self.rows + i] = self.data[i * COLUMNS + j].clone();
            }
        }
        MatrixSxD {
            data,
            columns: self.rows,
        }
    }
}
impl<T: Default + Clone> MatrixDxD<T> {
    /// Returns the transposition of `self` as a new matrix.
    /// ```
    /// use static_la::MatrixDxD;
    /// use std::convert::TryFrom;
    /// let a = MatrixDxD::try_from(vec![vec![1,2,3],vec![4,5,6]]).unwrap();
    /// assert_eq!(a.transpose(),MatrixDxD::try_from(vec![vec![1,4],vec![2,5],vec![3,6]]).unwrap());
    /// ```
    pub fn transpose(&self) -> MatrixDxD<T> {
        let mut data = vec![Default::default(); self.rows * self.columns];
        for i in 0..self.rows {
            for j in 0..self.columns {
                data[j * self.rows + i] = self.data[i * self.columns + j].clone();
            }
        }
        MatrixDxD {
            data,
            rows: self.columns,
            columns: self.rows,
        }
    }
}
