extern crate openblas_src;

use crate::*;
use std::fmt::Debug;
use std::ops::{AddAssign, Mul};

// /// Internal trait used for specialization.
// pub auto trait NotF32 {}
// /// Internal trait impl used for specialization.
// impl !NotF32 for f32 {}

/// Internal matrix multiplication trait
pub trait InternalMatmul: Sized {
    fn matmul(
        // An `m` by `k` row-major matrix.
        a: &[Self],
        // An `k` by `n` row-major matrix.
        b: &[Self],
        // An `m` by `n` row-major matrix.
        c: &mut [Self],
        // Rows of `a` and rows of `c`.
        m: usize,
        // Columns of `b` and columns of `c`.
        n: usize,
        // Columns of `a` and rows of `b`.
        k: usize,
    );
}
/// Default matrix multiplication implementation.
impl<T: Debug + Mul<Output = T> + AddAssign + Copy + Debug> InternalMatmul for T {
    default fn matmul(a: &[T], b: &[T], c: &mut [T], m: usize, n: usize, k: usize) {
        debug_assert_eq!(a.len(), m * k);
        debug_assert_eq!(b.len(), k * n);
        debug_assert_eq!(c.len(), m * n);

        for l_index in 0..m {
            for m_index in 0..k {
                for n_index in 0..n {
                    let (i, j, k) = (
                        l_index * n + n_index,
                        l_index * k + m_index,
                        m_index * n + n_index,
                    );
                    c[i] += a[j] * b[k];
                }
            }
        }
    }
}
/// `f32` matrix multiplication specialization.
impl InternalMatmul for f32 {
    fn matmul(a: &[f32], b: &[f32], c: &mut [f32], m: usize, n: usize, k: usize) {
        assert_eq!(a.len(), m * k);
        assert_eq!(b.len(), n * k);
        assert_eq!(c.len(), m * n);
        let (m, n, k) = (m as i32, n as i32, k as i32);
        unsafe {
            cblas::sgemm(
                cblas::Layout::RowMajor,
                cblas::Transpose::None,
                cblas::Transpose::None,
                m,
                n,
                k,
                1.,
                a,
                k,
                b,
                n,
                1.,
                c,
                n,
            );
        }
    }
}
/// `f64` matrix multiplication specialization.
impl InternalMatmul for f64 {
    fn matmul(a: &[f64], b: &[f64], c: &mut [f64], m: usize, n: usize, k: usize) {
        assert_eq!(a.len(), m * k);
        assert_eq!(b.len(), n * k);
        assert_eq!(c.len(), m * n);
        let (m, n, k) = (m as i32, n as i32, k as i32);
        unsafe {
            cblas::dgemm(
                cblas::Layout::RowMajor,
                cblas::Transpose::None,
                cblas::Transpose::None,
                m,
                n,
                k,
                1.,
                a,
                k,
                b,
                n,
                1.,
                c,
                n,
            );
        }
    }
}

/// A trait for matrix multiplication.
pub trait Matmul<T> {
    type Output;
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
    fn matmul(&self, other: &T) -> Self::Output;
}

// MatrixDxD
// --------------------------------------------------
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T>, const K: usize, const N: usize>
    Matmul<MatrixSxS<T, K, N>> for MatrixDxD<T>
where
    [(); K * N]:,
{
    type Output = MatrixDxS<T, N>;
    fn matmul(&self, other: &MatrixSxS<T, K, N>) -> Self::Output {
        assert_eq!(self.columns, K, "Non-matching columns to rows");

        let m = self.rows;
        let mut data = vec![Default::default(); N * m];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, m, N, K);
        Self::Output { data, rows: m }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T>, const N: usize>
    Matmul<MatrixDxS<T, N>> for MatrixDxD<T>
{
    type Output = MatrixDxS<T, N>;
    fn matmul(&self, other: &MatrixDxS<T, N>) -> Self::Output {
        assert_eq!(self.columns, other.rows, "Non-matching columns to rows");

        let (m, k) = (self.rows, self.columns);
        let mut data = vec![Default::default(); m * N];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, m, N, k);
        Self::Output { data, rows: m }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug, const K: usize>
    Matmul<MatrixSxD<T, K>> for MatrixDxD<T>
{
    type Output = MatrixDxD<T>;
    fn matmul(&self, other: &MatrixSxD<T, K>) -> Self::Output {
        let (m, n) = (self.rows, other.columns);
        let mut data = vec![Default::default(); m * n];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, m, n, K);
        Self::Output {
            data,
            rows: m,
            columns: n,
        }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug> Matmul<MatrixDxD<T>>
    for MatrixDxD<T>
{
    type Output = MatrixDxD<T>;
    fn matmul(&self, other: &MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.columns, other.rows, "Non-matching columns to rows");

        let (m, k, n) = (self.rows, self.columns, other.columns);
        let mut data = vec![Default::default(); m * n];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, m, n, k);
        Self::Output {
            data,
            rows: m,
            columns: n,
        }
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T>, const K: usize, const N: usize>
    Matmul<MatrixSxS<T, K, N>> for MatrixDxS<T, K>
where
    [(); K * N]:,
{
    type Output = MatrixDxS<T, N>;
    fn matmul(&self, other: &MatrixSxS<T, K, N>) -> Self::Output {
        let m = self.rows;
        let mut data = vec![Default::default(); m * N];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, m, N, K);
        Self::Output { data, rows: m }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T>, const K: usize, const N: usize>
    Matmul<MatrixDxS<T, N>> for MatrixDxS<T, K>
{
    type Output = MatrixDxS<T, N>;
    fn matmul(&self, other: &MatrixDxS<T, N>) -> Self::Output {
        let m = self.rows;
        let mut data = vec![Default::default(); m * N];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, m, N, K);
        Self::Output { data, rows: m }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug, const K: usize>
    Matmul<MatrixSxD<T, K>> for MatrixDxS<T, K>
{
    type Output = MatrixDxD<T>;
    fn matmul(&self, other: &MatrixSxD<T, K>) -> Self::Output {
        let (m, n) = (self.rows, other.columns);
        let mut data = vec![Default::default(); m * n];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, m, n, K);
        Self::Output {
            data,
            rows: m,
            columns: n,
        }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug, const K: usize>
    Matmul<MatrixDxD<T>> for MatrixDxS<T, K>
{
    type Output = MatrixDxD<T>;
    fn matmul(&self, other: &MatrixDxD<T>) -> Self::Output {
        assert_eq!(K, other.rows, "Non-matching columns to rows");

        let (m, n) = (self.rows, other.columns);
        let mut data = vec![Default::default(); m * n];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, m, n, K);
        Self::Output {
            data,
            rows: m,
            columns: n,
        }
    }
}
// MatrixSxD
// --------------------------------------------------
impl<
        T: Debug + Default + Copy + AddAssign + Mul<Output = T>,
        const M: usize,
        const K: usize,
        const N: usize,
    > Matmul<MatrixSxS<T, K, N>> for MatrixSxD<T, M>
where
    [(); M * N]:,
    [(); K * N]:,
{
    type Output = MatrixSxS<T, M, N>;
    fn matmul(&self, other: &MatrixSxS<T, K, N>) -> Self::Output {
        let mut data = [Default::default(); M * N];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, M, N, K);
        Self::Output { data }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T>, const M: usize, const N: usize>
    Matmul<MatrixDxS<T, N>> for MatrixSxD<T, M>
where
    [(); M * N]:,
{
    type Output = MatrixSxS<T, M, N>;
    fn matmul(&self, other: &MatrixDxS<T, N>) -> Self::Output {
        assert_eq!(self.columns, other.rows, "Non-matching columns to rows");

        let k = self.columns;
        let mut data = [Default::default(); M * N];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, M, N, k);
        Self::Output { data }
    }
}
impl<
        T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug,
        const M: usize,
        const K: usize,
    > Matmul<MatrixSxD<T, K>> for MatrixSxD<T, M>
{
    type Output = MatrixSxD<T, M>;
    fn matmul(&self, other: &MatrixSxD<T, K>) -> Self::Output {
        let n = other.columns;
        let mut data = vec![Default::default(); M * n];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, M, n, K);
        Self::Output { data, columns: n }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug, const M: usize>
    Matmul<MatrixDxD<T>> for MatrixSxD<T, M>
{
    type Output = MatrixSxD<T, M>;
    fn matmul(&self, other: &MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.columns, other.rows, "Non-matching columns to rows");

        let (k, n) = (self.columns, other.columns);
        let mut data = vec![Default::default(); M * n];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, M, n, k);
        Self::Output { data, columns: n }
    }
}
// MatrixSxS
// --------------------------------------------------
impl<
        T: Debug + Default + Copy + AddAssign + Mul<Output = T>,
        const M: usize,
        const K: usize,
        const N: usize,
    > Matmul<MatrixSxS<T, K, N>> for MatrixSxS<T, M, K>
where
    [(); M * K]:,
    [(); K * N]:,
    [(); M * N]:,
{
    type Output = MatrixSxS<T, M, N>;
    fn matmul(&self, other: &MatrixSxS<T, K, N>) -> Self::Output {
        let mut data = [Default::default(); M * N];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, M, N, K);
        Self::Output { data }
    }
}
impl<
        T: Debug + Default + Copy + AddAssign + Mul<Output = T>,
        const M: usize,
        const K: usize,
        const N: usize,
    > Matmul<MatrixDxS<T, N>> for MatrixSxS<T, M, K>
where
    [(); M * K]:,
    [(); M * N]:,
{
    type Output = MatrixSxS<T, M, N>;
    fn matmul(&self, other: &MatrixDxS<T, N>) -> Self::Output {
        let mut data = [Default::default(); M * N];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, M, N, K);
        Self::Output { data }
    }
}
impl<
        T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug,
        const M: usize,
        const K: usize,
    > Matmul<MatrixSxD<T, K>> for MatrixSxS<T, M, K>
where
    [(); M * K]:,
{
    type Output = MatrixSxD<T, M>;
    fn matmul(&self, other: &MatrixSxD<T, K>) -> Self::Output {
        let n = other.columns;
        let mut data = vec![Default::default(); M * n];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, M, n, K);
        Self::Output { data, columns: n }
    }
}
impl<
        T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug,
        const M: usize,
        const K: usize,
    > Matmul<MatrixDxD<T>> for MatrixSxS<T, M, K>
where
    [(); M * K]:,
{
    type Output = MatrixSxD<T, M>;
    fn matmul(&self, other: &MatrixDxD<T>) -> Self::Output {
        assert_eq!(K, other.rows, "Non-matching columns to rows");

        let n = other.columns;
        let mut data = vec![Default::default(); M * n];
        InternalMatmul::matmul(&self.data, &other.data, &mut data, M, n, K);
        Self::Output { data, columns: n }
    }
}
// Tests
// --------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::*;
    use std::convert::TryFrom;
    #[test]
    fn special() {
        let l = MatrixSxS::<f32, 3, 3>::from([[1., 0., 0.], [2., 4., 0.], [3., 5., 6.]]);
        let lt = l.transpose();
        println!("{}", lt);
        let a = l.matmul(&lt);
        println!("{}", a);
    }
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
