use crate::*;

// MatrixSxS
// --------------------------------------------------
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
impl<T, const ROWS: usize, const COLUMNS: usize> MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    /// Returns a transposition mask over `self` which allows operations by reference on the transposition of `self` without cloning the underlying data.
    ///
    /// This is generally preferable over [`Self::transpose`].
    pub fn transpose_ref<'a>(&'a self) -> TransposeSxS<'a, T, ROWS, COLUMNS> {
        TransposeSxS(&self)
    }
}
/// A mask allowing operations on the transposition of a `MatrixSxD` by reference without cloning
///  the underlying data.
///
/// This type only implements operations in which it is consumed. This is because as part of any
///  operation it is in, it performs a transpose on the underlying data. While this allows
///  performing operations on the transposition of a matrix without cloning the underlying data
///  it means it is only performant to do this when you only need the transpose for 1 operation.
///  If you where to use this for multiple operations you would be performing the transposition
///  for each operation. So unless you have particularly heavy memory requirements it is best
///  to use [`MatrixSxS::transpose`] instead if you need to use the transposition in multiple operations.
#[derive(Debug)]
pub struct TransposeSxS<'a, T, const ROWS: usize, const COLUMNS: usize>(
    &'a MatrixSxS<T, ROWS, COLUMNS>,
)
where
    [(); ROWS * COLUMNS]:;
impl<'a, T, const ROWS: usize, const COLUMNS: usize> TransposeSxS<'a, T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    /// An iterator over elements in transpose order.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        (0..COLUMNS)
            .map(move |j| (0..ROWS).map(move |i| &self.0.data[i * COLUMNS + j]))
            .flatten()
    }
}
// MatrixSxD
// --------------------------------------------------
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
impl<T, const ROWS: usize> MatrixSxD<T, ROWS> {
    /// Returns a transposition mask over `self` which allows operations by reference on the transposition of `self` without cloning the underlying data.
    ///
    /// This is generally preferable over [`Self::transpose`].
    pub fn transpose_ref<'a>(&'a self) -> TransposeSxD<'a, T, ROWS> {
        TransposeSxD(&self)
    }
}
/// A mask allowing operations on the transposition of a `MatrixSxD` by reference without cloning
///  the underlying data.
///
/// This type only implements operations in which it is consumed. This is because as part of any
///  operation it is in, it performs a transpose on the underlying data. While this allows
///  performing operations on the transposition of a matrix without cloning the underlying data
///  it means it is only performant to do this when you only need the transpose for 1 operation.
///  If you where to use this for multiple operations you would be performing the transposition
///  for each operation. So unless you have particularly heavy memory requirements it is best
///  to use [`MatrixSxD::transpose`] instead if you need to use the transposition in multiple operations.
#[derive(Debug)]
pub struct TransposeSxD<'a, T, const ROWS: usize>(&'a MatrixSxD<T, ROWS>);
impl<'a, T, const ROWS: usize> TransposeSxD<'a, T, ROWS> {
    /// An iterator over elements in transpose order.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        (0..self.0.columns)
            .map(move |j| (0..ROWS).map(move |i| &self.0.data[i * self.0.columns + j]))
            .flatten()
    }
    /// Gets transposed number of rows.
    pub fn rows(&self) -> usize {
        self.0.columns
    }
}
// MatrixDxS
// --------------------------------------------------
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
impl<T, const COLUMNS: usize> MatrixDxS<T, COLUMNS> {
    /// Returns a transposition mask over `self` which allows operations by reference on the transposition of `self` without cloning the underlying data.
    ///
    /// This is generally preferable over [`Self::transpose`].
    pub fn transpose_ref<'a>(&'a self) -> TransposeDxS<'a, T, COLUMNS> {
        TransposeDxS(&self)
    }
}
/// A mask allowing operations on the transposition of a `MatrixDxS` by reference without cloning
///  the underlying data.
///
/// This type only implements operations in which it is consumed. This is because as part of any
///  operation it is in, it performs a transpose on the underlying data. While this allows
///  performing operations on the transposition of a matrix without cloning the underlying data
///  it means it is only performant to do this when you only need the transpose for 1 operation.
///  If you where to use this for multiple operations you would be performing the transposition
///  for each operation. So unless you have particularly heavy memory requirements it is best
///  to use [`MatrixDxS::transpose`] instead if you need to use the transposition in multiple operations.
#[derive(Debug)]
pub struct TransposeDxS<'a, T, const COLUMNS: usize>(&'a MatrixDxS<T, COLUMNS>);
impl<'a, T, const COLUMNS: usize> TransposeDxS<'a, T, COLUMNS> {
    /// An iterator over elements in transpose order.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        (0..COLUMNS)
            .map(move |j| (0..self.0.rows).map(move |i| &self.0.data[i * COLUMNS + j]))
            .flatten()
    }
    /// Gets transposed number of columns.
    pub fn columns(&self) -> usize {
        self.0.rows
    }
}
// MatrixDxD
// --------------------------------------------------
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
impl<T> MatrixDxD<T> {
    /// Returns a transposition mask over `self` which allows operations by reference on the transposition of `self` without cloning the underlying data.
    ///
    /// This is generally preferable over [`Self::transpose`].
    pub fn transpose_ref<'a>(&'a self) -> TransposeDxD<'a, T> {
        TransposeDxD(&self)
    }
}
/// A mask allowing operations on the transposition of a `MatrixDxD` by reference without cloning
///  the underlying data.
///
/// This type only implements operations in which it is consumed. This is because as part of any
///  operation it is in, it performs a transpose on the underlying data. While this allows
///  performing operations on the transposition of a matrix without cloning the underlying data
///  it means it is only performant to do this when you only need the transpose for 1 operation.
///  If you where to use this for multiple operations you would be performing the transposition
///  for each operation. So unless you have particularly heavy memory requirements it is best
///  to use [`MatrixDxD::transpose`] instead if you need to use the transposition in multiple operations.
#[derive(Debug)]
pub struct TransposeDxD<'a, T>(&'a MatrixDxD<T>);
impl<'a, T> TransposeDxD<'a, T> {
    /// An iterator over elements in transpose order.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        (0..self.0.columns)
            .map(move |j| (0..self.0.rows).map(move |i| &self.0.data[i * self.0.columns + j]))
            .flatten()
    }
    /// Gets transposed number of rows.
    pub fn rows(&self) -> usize {
        self.0.columns
    }
    /// Gets transposed number of columns.
    pub fn columns(&self) -> usize {
        self.0.rows
    }
}
