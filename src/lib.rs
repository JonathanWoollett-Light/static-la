#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(array_methods)]
#![feature(array_chunks)]
#![feature(adt_const_params)]
//! An extremely minimal super static type safe implementation of matrix types.
//!
//! While [`ndarray`](https://docs.rs/ndarray/latest/ndarray/) offers no compile time type checking
//!  on dimensionality and [`nalgebra`](https://docs.rs/nalgebra/latest/nalgebra/) offers some
//!  finnicky checking, this offers the maximum possible checking.
//!
//! When performing the addition of a [`MatrixDxS`] (a matrix with a known number of columns at
//!  compile time) and a [`MatrixSxD`] (a matrix with a known number of rows at compile time) you
//!  get a [`MatrixSxS`] (a matrix with a known number of rows and columns at compile time) since
//!  now both the number of rows and columns are known at compile time. This then allows this
//!  infomation to propagate through your program providing excellent compile time checking.
//!
//! That being said... I made this in a weekend and there is a tiny amount of functionality.
//!
//! *Note: Some examples may be ignored but all are tested, in rustdoc or in a plain test.*
//!
//! An example of how types will propagate through a program:
//! ```ignore
//! use static_la::*;
//! // MatrixSxS<i32,2,3>
//! let a = MatrixSxS::from([[1,2,3],[4,5,6]]);
//! // MatrixDxS<i32,3>
//! let b = MatrixDxS::from(vec![[2,2,2],[3,3,3]]);
//! // MatrixSxS<i32,2,3>
//! let c = (a.clone() + b.clone()) - a.clone();
//! // MatrixDxS<i32,3>
//! let d = c.add_rows(b);
//! // MatrixSxS<i32,4,3>
//! let e = MatrixSxS::from([[1,2,3],[4,5,6],[7,8,9],[10,11,12]]);
//! // MatrixSxS<i32,4,6>
//! let f = d.add_columns(e);
//! ```
//!
//! In this example the only operations which cannot be fully checked at compile time are:
//! 1. `a.clone() + b.clone()`
//! 2. `d.add_columns(e)`
//!
//!
//! ### Construction
//! ```rust
//! use std::convert::TryFrom;
//! use static_la::*;
//! // From shaped arrays/vecs
//! let dxd = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
//! let dxs = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
//! let sxd = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
//! let sxs = MatrixSxS::<i32,2,3>::from([[1, 2, 3], [4, 5, 6]]);
//! // From a given shape and array/vec
//! let dxd = MatrixDxD::try_from((2,3,vec![1, 2, 3, 4, 5, 6])).unwrap();
//! let dxs = MatrixDxS::<_,2>::try_from((3,vec![1, 2, 3,4, 5, 6])).unwrap();
//! let sxd = MatrixSxD::<_,3>::try_from((2,vec![1, 2, 3, 4, 5, 6])).unwrap();
//! let sxs = MatrixSxS::<_,2,3>::from([1, 2, 3, 4, 5, 6]);
//! // From a given shape and slice
//! let dxd = MatrixDxD::try_from((2,3,&[1, 2, 3, 4, 5, 6])).unwrap();
//! let dxs = MatrixDxS::<_,2>::try_from((3,&[1, 2, 3,4, 5, 6])).unwrap();
//! let sxd = MatrixSxD::<_,3>::try_from((2,&[1, 2, 3, 4, 5, 6])).unwrap();
//! let sxs = MatrixSxS::<_,2,3>::from(&[1, 2, 3, 4, 5, 6]);
//! // ┌───────┐
//! // │ 1 2 3 │
//! // │ 4 5 6 │
//! // └───────┘
//! // From a given shape and value
//! let dxd = MatrixDxD::from((2,3,5));
//! let dxs = MatrixDxS::<_,3>::from((2,5));
//! let sxd = MatrixSxD::<_,2>::from((3,5));
//! let sxs = MatrixSxS::<_,2,3>::from(5);
//! // ┌───────┐
//! // │ 5 5 5 │
//! // │ 5 5 5 │
//! // └───────┘
//! ```
//! ### Indexing
//! ```
//! use static_la::MatrixDxS;
//! let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
//! assert_eq!(a[(0,0)], 1);
//! assert_eq!(a[(0,1)], 2);
//! assert_eq!(a[(0,2)], 3);
//! assert_eq!(a[(1,0)], 4);
//! assert_eq!(a[(1,1)], 5);
//! assert_eq!(a[(1,2)], 6);
//! ```
//! ### Expansion
//! ```ignore
//! use std::convert::TryFrom;
//! use static_la::*;
//! let a = MatrixDxD::try_from(vec![vec![1, 2]]).unwrap();
//! // ┌─────┐
//! // │ 1 2 │
//! // └─────┘
//! let b = a.add_rows(MatrixDxS::from(vec![[4, 5]]));
//! // ┌─────┐
//! // │ 1 2 │
//! // │ 4 5 │
//! // └─────┘
//! let c = b.add_columns(MatrixSxS::from([[3], [6]]));
//! // ┌───────┐
//! // │ 1 2 3 │
//! // │ 4 5 6 │
//! // └───────┘
//! assert_eq!(c,MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap());
//! ```
//! ### Arithmetic
//! ```ignore
//! use std::convert::TryFrom;
//! use static_la::*;
//! let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
//! let mut b = a + MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
//! assert_eq!(b, MatrixDxS::from(vec![[8, 10, 12], [14, 16, 18]]));
//! b -= MatrixDxS::from(vec![[3, 6, 9], [12, 15, 18]]);
//! assert_eq!(b, MatrixDxS::from(vec![[5, 4, 3], [2, 1, 0]]));
//! let c = b * MatrixSxS::from([[2, 2, 2], [3, 3, 3]]);
//! assert_eq!(c, MatrixSxS::from([[10, 8, 6], [6, 3, 0]]));
//! ```
//! ### Slicing
//! ```ignore
//! use std::convert::TryFrom;
//! use static_la::*;
//! let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
//! assert_eq!(a.slice_dxd((0..1, 0..2)), MatrixDxD::try_from(vec![vec![&1, &2]]).unwrap());
//! assert_eq!(a.slice_dxs::<{ 0..2 }>(0..1), MatrixDxS::from(vec![[&1, &2]]));
//! assert_eq!(a.slice_sxd::<{ 0..1 }>(0..2), MatrixSxD::try_from([vec![&1, &2]]).unwrap());
//! assert_eq!(a.slice_sxs::<{ 0..1 }, { 0..2 }>(), MatrixSxS::from([[&1, &2]]));
//! ```

/// [`std::ops::Add`] Arithmetic addition operations.
mod add;
/// [`std::ops::AddAssign`] Arithmetic addition operations.
mod add_assign;
/// Functionality to expand matrices.
mod add_rows;
pub use add_rows::AddRows;
/// Functionality to expand matrices.
mod add_columns;
pub use add_columns::AddColumns;
/// [`std::ops::Div`] Arithmetic division operations.
mod div;
/// [`std::ops::DivAssign`] Arithmetic division operations.
mod div_assign;
/// [`std::fmt::Display`] Format implementations.
mod fmt;
/// [`std::convert::From`] Value-to-value conversions.
mod from;
/// [`std::ops::Index`] Indexing operations.
mod index;
/// Iterations functionality.
mod iter;
/// Matrix multiplication functionality.
mod matmul;
pub use matmul::Matmul;
/// [`std::ops::Mul`] Arithmetic multiplication operations.
mod mul;
/// [`std::ops::MulAssign`] Arithmetic multiplication operations.
mod mul_assign;
/// [`std::cmp::PartialEq`] Partial equality comparison operations.
mod partial_eq;
/// Slicing functionality.
mod slice;
pub use slice::*;
/// [`std::ops::Sub`] Arithmetic subtraction operations.
mod sub;
/// [`std::ops::SubAssign`] Arithmetic subtraction operations.
mod sub_assign;
/// [`std::convert::TryFrom`] Fallible value-to-value conversions.
mod try_from;

// Matrix types
// --------------------------------------------------
/// A `dynamic x dynamic` matrix where neither dimension is known at compile time.
/// ```
/// use std::convert::TryFrom;
/// let _ = static_la::MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
/// ```
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct MatrixDxD<T> {
    /// Underlying data.
    data: Vec<T>,
    columns: usize,
    rows: usize,
}
impl<T> MatrixDxD<T> {
    /// Number of elements in matrix.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Number of columns.
    pub fn columns(&self) -> usize {
        self.columns
    }
    /// Number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }
}
impl<T: std::iter::Sum<T> + Clone> MatrixDxD<T> {
    /// Gets sum of all elements.
    pub fn sum(&self) -> T {
        self.data.iter().cloned().sum()
    }
    /// Gets the sum of each row.
    pub fn row_sum(&self) -> ColumnVectorD<T> {
        ColumnVectorD::from(
            self.data
                .chunks_exact(self.columns)
                .map(|r| [r.iter().cloned().sum()])
                .collect::<Vec<_>>(),
        )
    }
    /// Gets the sum of each column.
    pub fn column_sum(&self) -> RowVectorD<T> {
        RowVectorD::try_from([(0..self.columns)
            .map(|i| {
                self.data
                    .iter()
                    .skip(i)
                    .step_by(self.columns)
                    .cloned()
                    .sum()
            })
            .collect::<Vec<_>>()])
        .unwrap()
    }
}
/// A `dynamic x static` matrix where the columns dimension is known at compile time.
/// ```
/// let _ = static_la::MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
/// ```
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct MatrixDxS<T, const COLUMNS: usize> {
    /// Underlying data.
    data: Vec<T>,
    rows: usize,
}
impl<T, const COLUMNS: usize> MatrixDxS<T, COLUMNS> {
    /// Number of elements in matrix.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Number of columns.
    pub const fn columns(&self) -> usize {
        COLUMNS
    }
    /// Number of rows.
    pub fn rows(&self) -> usize {
        self.rows
    }
}
impl<T: std::iter::Sum<T> + Copy + Default + std::fmt::Debug, const COLUMNS: usize>
    MatrixDxS<T, COLUMNS>
where
    [(); 1 * COLUMNS]:,
{
    /// Gets sum of all elements.
    pub fn sum(&self) -> T {
        self.data.iter().cloned().sum()
    }
    /// Gets the sum of each row.
    pub fn row_sum(&self) -> ColumnVectorD<T> {
        ColumnVectorD::from(
            self.data
                .chunks_exact(COLUMNS)
                .map(|r| [r.iter().cloned().sum()])
                .collect::<Vec<_>>(),
        )
    }
    /// Gets the sum of each column.
    pub fn column_sum(&self) -> RowVectorS<T, COLUMNS> {
        // Will dropping the `Default` and `Copy` traits to use `try_into` affect performance?
        let mut sums: [T; COLUMNS] = [Default::default(); COLUMNS];
        for i in 0..COLUMNS {
            sums[i] = self.data.iter().skip(i).step_by(COLUMNS).cloned().sum();
        }
        RowVectorS::from([sums])
    }
}
/// A `static x dynamic` matrix where the rows dimension is known at compile time.
/// ```
/// use std::convert::TryFrom;
/// let _ = static_la::MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
/// ```
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct MatrixSxD<T, const ROWS: usize> {
    /// Underlying data.
    data: Vec<T>,
    columns: usize,
}
impl<T, const ROWS: usize> MatrixSxD<T, ROWS> {
    /// Number of elements in matrix.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Number of rows.
    pub const fn rows(&self) -> usize {
        ROWS
    }
    /// Number of columns.
    pub fn columns(&self) -> usize {
        self.columns
    }
}
use std::convert::TryFrom;
impl<T: std::iter::Sum<T> + Copy + Default + std::fmt::Debug, const ROWS: usize> MatrixSxD<T, ROWS>
where
    [(); ROWS * 1]:,
{
    /// Gets sum of all elements.
    pub fn sum(&self) -> T {
        self.data.iter().cloned().sum()
    }
    /// Gets the sum of each row.
    pub fn row_sum(&self) -> ColumnVectorS<T, ROWS> {
        let mut sums: [[T; 1]; ROWS] = [[Default::default(); 1]; ROWS];
        for (i, row) in (0..ROWS).zip(self.data.chunks_exact(self.columns)) {
            sums[i][0] = row.iter().cloned().sum();
        }
        ColumnVectorS::from(sums)
    }
    /// Gets the sum of each column.
    pub fn column_sum(&self) -> RowVectorD<T> {
        RowVectorD::try_from([(0..self.columns)
            .map(|i| {
                self.data
                    .iter()
                    .skip(i)
                    .step_by(self.columns)
                    .cloned()
                    .sum()
            })
            .collect::<Vec<_>>()])
        .unwrap()
    }
}
/// A `static x static` matrix where both dimensions are known at compile time.
/// ```
/// let _ = static_la::MatrixSxS::<i32,2,3>::from([[1, 2, 3], [4, 5, 6]]);
/// ```
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct MatrixSxS<T, const ROWS: usize, const COLUMNS: usize>
where
    [(); ROWS * COLUMNS]:,
{
    /// Underlying data.
    pub data: [T; ROWS * COLUMNS],
}
impl<T, const ROWS: usize, const COLUMNS: usize> MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    /// Number of elements in matrix.
    pub const fn len(&self) -> usize {
        ROWS * COLUMNS
    }
    /// Number of rows.
    pub const fn rows(&self) -> usize {
        ROWS
    }
    /// Number of columns.
    pub const fn columns(&self) -> usize {
        COLUMNS
    }
}
impl<
        T: std::iter::Sum<T> + Copy + Default + std::fmt::Debug,
        const ROWS: usize,
        const COLUMNS: usize,
    > MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
    [(); ROWS * 1]:,
    [(); 1 * COLUMNS]:,
{
    /// Gets sum of all elements.
    pub fn sum(&self) -> T {
        self.data.iter().cloned().sum()
    }
    /// Gets the sum of each row.
    pub fn row_sum(&self) -> ColumnVectorS<T, ROWS> {
        let mut sums: [[T; 1]; ROWS] = [[Default::default(); 1]; ROWS];
        for (i, row) in (0..ROWS).zip(self.data.chunks_exact(COLUMNS)) {
            sums[i][0] = row.iter().cloned().sum();
        }
        ColumnVectorS::from(sums)
    }
    /// Gets the sum of each column.
    pub fn column_sum(&self) -> RowVectorS<T, COLUMNS> {
        let mut sums: [T; COLUMNS] = [Default::default(); COLUMNS];
        for i in 0..COLUMNS {
            sums[i] = self.data.iter().skip(i).step_by(COLUMNS).cloned().sum();
        }
        RowVectorS::from([sums])
    }
}
// Vector aliases
// --------------------------------------------------
/// A matrix with 1 column and a dynamic number of rows.
///
/// E.g.
/// ```text
/// ┌───┐
/// │ 1 │
/// │ 2 │
/// │ 3 │
/// └───┘
/// ```
pub type ColumnVectorD<T> = MatrixDxS<T, 1>;
/// A matrix with 1 column and a static number of rows.
///
/// E.g.
/// ```text
/// ┌───┐
/// │ 1 │
/// │ 2 │
/// │ 3 │
/// └───┘
/// ```
pub type ColumnVectorS<T, const ROWS: usize> = MatrixSxS<T, ROWS, 1>;
/// A matrix with 1 row and a dynamic number of columns.
///
/// E.g.
/// ```text
/// ┌───────┐
/// │ 1 2 3 │
/// └───────┘
/// ```
pub type RowVectorD<T> = MatrixSxD<T, 1>;
/// A matrix with 1 row and a static number of columns.
///
/// E.g.
/// ```text
/// ┌───────┐
/// │ 1 2 3 │
/// └───────┘
/// ```
pub type RowVectorS<T, const COLUMNS: usize> = MatrixSxS<T, 1, COLUMNS>;

// These tests performs the tests that are ignored in the rustdoc.
// They are ignored in rustdoc as they cause compiler errors.
#[cfg(test)]
mod tests {
    use crate::*;
    use std::convert::TryFrom;
    #[test]
    fn rustdoc1() {
        // MatrixSxS<i32,2,3>
        let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
        // MatrixDxS<i32,3>
        let b = MatrixDxS::from(vec![[2, 2, 2], [3, 3, 3]]);
        // MatrixSxS<i32,2,3>
        let c = (a.clone() + b.clone()) - a.clone();
        // MatrixDxS<i32,3>
        let d = c.add_rows(b);
        // MatrixSxS<i32,4,3>
        let e = MatrixSxS::from([[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]]);
        // MatrixSxS<i32,4,6>
        let _f = d.add_columns(e);
    }
    #[test]
    fn rustdoc2() {
        let a = MatrixDxD::try_from(vec![vec![1, 2]]).unwrap();
        let b = a.add_rows(MatrixDxS::from(vec![[4, 5]]));
        let c = b.add_columns(MatrixSxS::from([[3], [6]]));
        assert_eq!(
            c,
            MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap()
        );
    }
    #[test]
    fn rustdoc3() {
        let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        let mut b = a + MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        assert_eq!(b, MatrixDxS::from(vec![[8, 10, 12], [14, 16, 18]]));
        b -= MatrixDxS::from(vec![[3, 6, 9], [12, 15, 18]]);
        assert_eq!(b, MatrixDxS::from(vec![[5, 4, 3], [2, 1, 0]]));
        let c = b * MatrixSxS::from([[2, 2, 2], [3, 3, 3]]);
        assert_eq!(c, MatrixSxS::from([[10, 8, 6], [6, 3, 0]]));
    }
    #[test]
    fn rustdoc4() {
        let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
        assert_eq!(
            a.slice_dxd((0..1, 0..2)),
            MatrixDxD::try_from(vec![vec![&1, &2]]).unwrap()
        );
        assert_eq!(
            a.slice_dxs::<{ 0..2 }>(0..1),
            MatrixDxS::from(vec![[&1, &2]])
        );
        assert_eq!(
            a.slice_sxd::<{ 0..1 }>(0..2),
            MatrixSxD::try_from([vec![&1, &2]]).unwrap()
        );
        assert_eq!(
            a.slice_sxs::<{ 0..1 }, { 0..2 }>(),
            MatrixSxS::from([[&1, &2]])
        );
    }
}
