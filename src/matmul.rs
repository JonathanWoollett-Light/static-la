extern crate openblas_src;

use crate::blas::Matrix;
use crate::*;
use std::fmt::Debug;
use std::ops::{AddAssign, Mul};

/// Matrix multiplication trait defining the operation on generic matrices.
pub trait Matmul<T, B: Matrix<T>>: InternalMatmul<T, B> {
    /// ```text
    /// ┌───────┐        ┌─────┐  ┌─────┐
    /// │ 1 1 1 │        │ 1 2 │  │ 4 5 │
    /// │ 2 1 2 │.matmul(│ 2 1 │)=│ 6 9 │
    /// └───────┘        │ 1 2 │  └─────┘
    ///                  └─────┘
    /// ```
    /// - M: Rows of `self` and rows of `Self::Output`.
    /// - K: Columns of `self` and rows of `other`.
    /// - N: Columns of `other` and columns of `Self::Output`.
    fn matmul(&self, b: &B) -> Self::Output;
}
impl<
        T: Default + Mul<Output = T> + AddAssign + Copy + Debug,
        B: Matrix<T>,
        A: InternalMatmul<T, B>,
    > Matmul<T, B> for A
{
    default fn matmul(&self, b: &B) -> Self::Output {
        let mut c = self.pre(b);
        let (m, n, k) = (self.rows(), b.columns(), self.columns());
        for l_index in 0..m {
            for k_index in 0..k {
                for n_index in 0..n {
                    let (i, j, k) = (
                        l_index * n + n_index,
                        l_index * k + k_index,
                        k_index * n + n_index,
                    );
                    c.data_mut()[i] += self.data()[j] * b.data()[k];
                }
            }
        }
        c
    }
}
/// `f32` specialization
impl<B: Matrix<f32>, A: InternalMatmul<f32, B>> Matmul<f32, B> for A {
    fn matmul(&self, b: &B) -> Self::Output {
        let mut c = self.pre(b);
        let (m, n, k) = (
            self.rows() as i32,
            b.columns() as i32,
            self.columns() as i32,
        );
        unsafe {
            cblas::sgemm(
                cblas::Layout::RowMajor,
                cblas::Transpose::None,
                cblas::Transpose::None,
                m,
                n,
                k,
                1.,
                self.data(),
                k,
                b.data(),
                n,
                1.,
                c.data_mut(),
                n,
            );
        }
        c
    }
}
/// `f64` specialization
impl<B: Matrix<f64>, A: InternalMatmul<f64, B>> Matmul<f64, B> for A {
    fn matmul(&self, b: &B) -> Self::Output {
        let mut c = self.pre(b);
        let (m, n, k) = (
            self.rows() as i32,
            b.columns() as i32,
            self.columns() as i32,
        );
        unsafe {
            cblas::dgemm(
                cblas::Layout::RowMajor,
                cblas::Transpose::None,
                cblas::Transpose::None,
                m,
                n,
                k,
                1.,
                self.data(),
                k,
                b.data(),
                n,
                1.,
                c.data_mut(),
                n,
            );
        }
        c
    }
}
/// Specific internal matrix multiplication trait defining the non-generic interactions between combinations of matrix types.
///
/// This requires an implementation for every combination.
pub trait InternalMatmul<T, B: Matrix<T>>: Matrix<T> {
    type Output: Matrix<T>;
    /// Non-generic pre-operation work (i.e. bounds checking & returning the new matrix to fill)
    fn pre(&self, b: &B) -> Self::Output;
}
// MatrixDxD
// --------------------------------------------------
// MatrixDxD @ MatrixSxS
impl<T: Default + Copy, const K: usize, const N: usize> InternalMatmul<T, MatrixSxS<T, K, N>>
    for MatrixDxD<T>
where
    [(); K * N]:,
{
    type Output = MatrixDxS<T, N>;
    fn pre(&self, _: &MatrixSxS<T, K, N>) -> Self::Output {
        assert_eq!(self.columns, K, "Non-matching columns to rows");

        let m = self.rows;
        let data = vec![Default::default(); N * m];
        Self::Output { data, rows: m }
    }
}
impl<T: Default + Copy, const N: usize> InternalMatmul<T, MatrixDxS<T, N>> for MatrixDxD<T> {
    type Output = MatrixDxS<T, N>;
    fn pre(&self, other: &MatrixDxS<T, N>) -> Self::Output {
        assert_eq!(self.columns, other.rows, "Non-matching columns to rows");

        let m = self.rows;
        let data = vec![Default::default(); m * N];
        Self::Output { data, rows: m }
    }
}
impl<T: Default + Copy, const K: usize> InternalMatmul<T, MatrixSxD<T, K>> for MatrixDxD<T> {
    type Output = MatrixDxD<T>;
    fn pre(&self, other: &MatrixSxD<T, K>) -> Self::Output {
        let (m, n) = (self.rows, other.columns);
        let data = vec![Default::default(); m * n];
        Self::Output {
            data,
            rows: m,
            columns: n,
        }
    }
}
impl<T: Default + Copy> InternalMatmul<T, MatrixDxD<T>> for MatrixDxD<T> {
    type Output = MatrixDxD<T>;
    fn pre(&self, other: &MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.columns, other.rows, "Non-matching columns to rows");

        let (m, n) = (self.rows, other.columns);
        let data = vec![Default::default(); m * n];
        Self::Output {
            data,
            rows: m,
            columns: n,
        }
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: Default + Copy, const K: usize, const N: usize> InternalMatmul<T, MatrixSxS<T, K, N>>
    for MatrixDxS<T, K>
where
    [(); K * N]:,
{
    type Output = MatrixDxS<T, N>;
    fn pre(&self, _: &MatrixSxS<T, K, N>) -> Self::Output {
        let m = self.rows;
        let data = vec![Default::default(); m * N];
        Self::Output { data, rows: m }
    }
}
impl<T: Default + Copy, const K: usize, const N: usize> InternalMatmul<T, MatrixDxS<T, N>>
    for MatrixDxS<T, K>
{
    type Output = MatrixDxS<T, N>;
    fn pre(&self, _: &MatrixDxS<T, N>) -> Self::Output {
        let m = self.rows;
        let data = vec![Default::default(); m * N];
        Self::Output { data, rows: m }
    }
}
impl<T: Default + Copy, const K: usize> InternalMatmul<T, MatrixSxD<T, K>> for MatrixDxS<T, K> {
    type Output = MatrixDxD<T>;
    fn pre(&self, other: &MatrixSxD<T, K>) -> Self::Output {
        let (m, n) = (self.rows, other.columns);
        let data = vec![Default::default(); m * n];
        Self::Output {
            data,
            rows: m,
            columns: n,
        }
    }
}
impl<T: Default + Copy, const K: usize> InternalMatmul<T, MatrixDxD<T>> for MatrixDxS<T, K> {
    type Output = MatrixDxD<T>;
    fn pre(&self, other: &MatrixDxD<T>) -> Self::Output {
        assert_eq!(K, other.rows, "Non-matching columns to rows");

        let (m, n) = (self.rows, other.columns);
        let data = vec![Default::default(); m * n];
        Self::Output {
            data,
            rows: m,
            columns: n,
        }
    }
}
// MatrixSxD
// --------------------------------------------------
impl<T: Default + Copy, const M: usize, const K: usize, const N: usize>
    InternalMatmul<T, MatrixSxS<T, K, N>> for MatrixSxD<T, M>
where
    [(); M * N]:,
    [(); K * N]:,
{
    type Output = MatrixSxS<T, M, N>;
    fn pre(&self, _: &MatrixSxS<T, K, N>) -> Self::Output {
        let data = [Default::default(); M * N];
        Self::Output { data }
    }
}
impl<T: Default + Copy, const M: usize, const N: usize> InternalMatmul<T, MatrixDxS<T, N>>
    for MatrixSxD<T, M>
where
    [(); M * N]:,
{
    type Output = MatrixSxS<T, M, N>;
    fn pre(&self, other: &MatrixDxS<T, N>) -> Self::Output {
        assert_eq!(self.columns, other.rows, "Non-matching columns to rows");

        let data = [Default::default(); M * N];
        Self::Output { data }
    }
}
impl<T: Default + Copy, const M: usize, const K: usize> InternalMatmul<T, MatrixSxD<T, K>>
    for MatrixSxD<T, M>
{
    type Output = MatrixSxD<T, M>;
    fn pre(&self, other: &MatrixSxD<T, K>) -> Self::Output {
        let n = other.columns;
        let data = vec![Default::default(); M * n];
        Self::Output { data, columns: n }
    }
}
impl<T: Default + Copy, const M: usize> InternalMatmul<T, MatrixDxD<T>> for MatrixSxD<T, M> {
    type Output = MatrixSxD<T, M>;
    fn pre(&self, other: &MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.columns, other.rows, "Non-matching columns to rows");

        let n = other.columns;
        let data = vec![Default::default(); M * n];
        Self::Output { data, columns: n }
    }
}
// MatrixSxS
// --------------------------------------------------
impl<T: Default + Copy, const M: usize, const K: usize, const N: usize>
    InternalMatmul<T, MatrixSxS<T, K, N>> for MatrixSxS<T, M, K>
where
    [(); M * K]:,
    [(); K * N]:,
    [(); M * N]:,
{
    type Output = MatrixSxS<T, M, N>;
    fn pre(&self, _: &MatrixSxS<T, K, N>) -> Self::Output {
        let data = [Default::default(); M * N];
        Self::Output { data }
    }
}
impl<T: Default + Copy, const M: usize, const K: usize, const N: usize>
    InternalMatmul<T, MatrixDxS<T, N>> for MatrixSxS<T, M, K>
where
    [(); M * K]:,
    [(); M * N]:,
{
    type Output = MatrixSxS<T, M, N>;
    fn pre(&self, _: &MatrixDxS<T, N>) -> Self::Output {
        let data = [Default::default(); M * N];
        Self::Output { data }
    }
}
impl<T: Default + Copy, const M: usize, const K: usize> InternalMatmul<T, MatrixSxD<T, K>>
    for MatrixSxS<T, M, K>
where
    [(); M * K]:,
{
    type Output = MatrixSxD<T, M>;
    fn pre(&self, other: &MatrixSxD<T, K>) -> Self::Output {
        let n = other.columns;
        let data = vec![Default::default(); M * n];
        Self::Output { data, columns: n }
    }
}
impl<T: Default + Copy, const M: usize, const K: usize> InternalMatmul<T, MatrixDxD<T>>
    for MatrixSxS<T, M, K>
where
    [(); M * K]:,
{
    type Output = MatrixSxD<T, M>;
    fn pre(&self, other: &MatrixDxD<T>) -> Self::Output {
        assert_eq!(K, other.rows, "Non-matching columns to rows");

        let n = other.columns;
        let data = vec![Default::default(); M * n];
        Self::Output { data, columns: n }
    }
}
// Tests
// --------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::*;
    use std::convert::TryFrom;
    // f32
    // --------------------------------------------------
    #[test]
    fn f32_dxd() {
        let a = MatrixDxD::<f32>::try_from(vec![vec![1., 3., 5.], vec![2., 4., 6.]]).unwrap();
        let b =
            MatrixDxD::<f32>::try_from(vec![vec![7., 10.], vec![8., 11.], vec![9., 12.]]).unwrap();
        let c = a.matmul(&b);
        let d = MatrixDxD::<f32>::try_from(vec![vec![76., 103.], vec![100., 136.]]).unwrap();
        assert_eq!(c, d);
    }
    // f64
    // --------------------------------------------------
    #[test]
    fn f64_dxd() {
        let a = MatrixDxD::<f64>::try_from(vec![vec![1., 3., 5.], vec![2., 4., 6.]]).unwrap();
        let b =
            MatrixDxD::<f64>::try_from(vec![vec![7., 10.], vec![8., 11.], vec![9., 12.]]).unwrap();
        let c = a.matmul(&b);
        let d = MatrixDxD::<f64>::try_from(vec![vec![76., 103.], vec![100., 136.]]).unwrap();
        assert_eq!(c, d);
    }
    // MatrixDxD
    // --------------------------------------------------
    #[test]
    fn dxd_dxd() {
        let a = MatrixDxD::try_from(vec![vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(&b);
        let d = MatrixDxD::try_from(vec![vec![76, 103], vec![100, 136]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_dxs() {
        let a = MatrixDxD::try_from(vec![vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(&b);
        let d = MatrixDxS::from(vec![[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxd() {
        let a = MatrixDxD::try_from(vec![vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(&b);
        let d = MatrixSxD::try_from([vec![76, 103], vec![100, 136]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxs() {
        let a = MatrixDxD::try_from(vec![vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(&b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs_dxs() {
        let a = MatrixDxS::from(vec![[1, 3, 5], [2, 4, 6]]);
        let b = MatrixDxS::from(vec![[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(&b);
        let d = MatrixDxS::from(vec![[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxd() {
        let a = MatrixDxS::from(vec![[1, 3, 5], [2, 4, 6]]);
        let b = MatrixSxD::try_from([vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(&b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_dxd() {
        let a = MatrixDxS::from(vec![[1, 3, 5], [2, 4, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(&b);
        let d = MatrixDxS::from(vec![[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxs() {
        let a = MatrixDxS::from(vec![[1, 3, 5], [2, 4, 6]]);
        let b = MatrixSxS::from([[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(&b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd_sxd() {
        let a = MatrixSxD::try_from([vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(&b);
        let d = MatrixSxD::try_from([vec![76, 103], vec![100, 136]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxs() {
        let a = MatrixSxD::try_from([vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(&b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_sxs() {
        let a = MatrixSxD::try_from([vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(&b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxd() {
        let a = MatrixSxD::try_from([vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(&b);
        let d = MatrixSxD::try_from([vec![76, 103], vec![100, 136]]).unwrap();
        assert_eq!(c, d);
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs_sxs() {
        let a = MatrixSxS::<i32, 2, 3>::from([[1, 3, 5], [2, 4, 6]]);
        let b = MatrixSxS::<i32, 3, 2>::from([[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(&b);
        let d = MatrixSxS::<i32, 2, 2>::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxs() {
        let a = MatrixSxS::from([[1, 3, 5], [2, 4, 6]]);
        let b = MatrixDxS::from(vec![[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(&b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_sxd() {
        let a = MatrixSxS::from([[1, 3, 5], [2, 4, 6]]);
        let b = MatrixSxD::try_from([vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(&b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxd() {
        let a = MatrixSxS::from([[1, 3, 5], [2, 4, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(&b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
}
