use crate::*;
use std::fmt::Debug;
use std::ops::{AddAssign, Mul};

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
    fn matmul(&self, other: &T) -> Self::Output;
}

// - L: Rows of `self` and rows of `Self::Output`
// - M: Columns of `self` and rows of `other`
// - N: Columns of `other` and rows of `Self::Output`

// Inline allows some specific optimization when the dimensions are const?
/// `c = a @ b`
#[inline]
fn matmul<T: Debug + Mul<Output = T> + AddAssign + Copy + Debug>(
    (l_len, n_len, m_len): (usize, usize, usize),
    (a, b, c): (&[T], &[T], &mut [T]),
) {
    debug_assert_eq!(a.len(), l_len * m_len);
    debug_assert_eq!(b.len(), m_len * n_len);
    debug_assert_eq!(c.len(), l_len * n_len);

    for l in 0..l_len {
        for m in 0..m_len {
            for n in 0..n_len {
                let (i, j, k) = (l * n_len + n, l * m_len + m, m * n_len + n);
                c[i] += a[j] * b[k];
                // c[l][n] += a[l][m] * b[m][n];
            }
        }
    }
}

// MatrixDxD
// --------------------------------------------------
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T>, const M: usize, const N: usize>
    Matmul<MatrixSxS<T, M, N>> for MatrixDxD<T>
where
    [(); M * N]:,
{
    type Output = MatrixDxS<T, N>;
    fn matmul(&self, other: &MatrixSxS<T, M, N>) -> Self::Output {
        assert_eq!(self.columns, M, "Non-matching columns to rows");

        let l_len = self.rows;
        let mut data = vec![Default::default(); N * l_len];
        matmul((l_len, N, M), (&self.data, &other.data, &mut data));
        Self::Output { data, rows: l_len }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T>, const N: usize>
    Matmul<MatrixDxS<T, N>> for MatrixDxD<T>
{
    type Output = MatrixDxS<T, N>;
    fn matmul(&self, other: &MatrixDxS<T, N>) -> Self::Output {
        assert_eq!(self.columns, other.rows, "Non-matching columns to rows");

        let (l, m) = (self.rows, self.columns);
        let mut data = vec![Default::default(); N * l];
        matmul((l, N, m), (&self.data, &other.data, &mut data));
        Self::Output { data, rows: l }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug, const M: usize>
    Matmul<MatrixSxD<T, M>> for MatrixDxD<T>
{
    type Output = MatrixDxD<T>;
    fn matmul(&self, other: &MatrixSxD<T, M>) -> Self::Output {
        let (l, n) = (self.rows, other.columns);
        let mut data = vec![Default::default(); l * n];
        matmul((l, n, M), (&self.data, &other.data, &mut data));
        Self::Output {
            data,
            rows: l,
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

        let (l, m, n) = (self.rows, self.columns, other.columns);
        let mut data = vec![Default::default(); l * n];
        matmul((l, n, m), (&self.data, &other.data, &mut data));
        Self::Output {
            data,
            rows: l,
            columns: n,
        }
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T>, const M: usize, const N: usize>
    Matmul<MatrixSxS<T, M, N>> for MatrixDxS<T, M>
where
    [(); M * N]:,
{
    type Output = MatrixDxS<T, N>;
    fn matmul(&self, other: &MatrixSxS<T, M, N>) -> Self::Output {
        let l = self.rows;
        let mut data = vec![Default::default(); l * N];
        matmul((l, N, M), (&self.data, &other.data, &mut data));
        Self::Output { data, rows: l }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T>, const M: usize, const N: usize>
    Matmul<MatrixDxS<T, N>> for MatrixDxS<T, M>
{
    type Output = MatrixDxS<T, N>;
    fn matmul(&self, other: &MatrixDxS<T, N>) -> Self::Output {
        let l = self.rows;
        let mut data = vec![Default::default(); l * N];
        matmul((l, N, M), (&self.data, &other.data, &mut data));
        Self::Output { data, rows: l }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug, const M: usize>
    Matmul<MatrixSxD<T, M>> for MatrixDxS<T, M>
{
    type Output = MatrixDxD<T>;
    fn matmul(&self, other: &MatrixSxD<T, M>) -> Self::Output {
        let (l, n) = (self.rows, other.columns);
        let mut data = vec![Default::default(); l * n];
        matmul((l, n, M), (&self.data, &other.data, &mut data));
        Self::Output {
            data,
            rows: l,
            columns: n,
        }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug, const M: usize>
    Matmul<MatrixDxD<T>> for MatrixDxS<T, M>
{
    type Output = MatrixDxD<T>;
    fn matmul(&self, other: &MatrixDxD<T>) -> Self::Output {
        assert_eq!(M, other.rows, "Non-matching columns to rows");

        let (l, n) = (self.rows, other.columns);
        let mut data = vec![Default::default(); l * n];
        matmul((l, n, M), (&self.data, &other.data, &mut data));
        Self::Output {
            data,
            rows: l,
            columns: n,
        }
    }
}
// MatrixSxD
// --------------------------------------------------
impl<
        T: Debug + Default + Copy + AddAssign + Mul<Output = T>,
        const L: usize,
        const M: usize,
        const N: usize,
    > Matmul<MatrixSxS<T, M, N>> for MatrixSxD<T, L>
where
    [(); L * N]:,
    [(); M * N]:,
{
    type Output = MatrixSxS<T, L, N>;
    fn matmul(&self, other: &MatrixSxS<T, M, N>) -> Self::Output {
        let mut data = [Default::default(); L * N];
        matmul((L, N, M), (&self.data, &other.data, &mut data));
        Self::Output { data }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T>, const L: usize, const N: usize>
    Matmul<MatrixDxS<T, N>> for MatrixSxD<T, L>
where
    [(); L * N]:,
{
    type Output = MatrixSxS<T, L, N>;
    fn matmul(&self, other: &MatrixDxS<T, N>) -> Self::Output {
        assert_eq!(self.columns, other.rows, "Non-matching columns to rows");

        let m = self.columns;
        let mut data = [Default::default(); L * N];
        matmul((L, N, m), (&self.data, &other.data, &mut data));
        Self::Output { data }
    }
}
impl<
        T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug,
        const L: usize,
        const M: usize,
    > Matmul<MatrixSxD<T, M>> for MatrixSxD<T, L>
{
    type Output = MatrixSxD<T, L>;
    fn matmul(&self, other: &MatrixSxD<T, M>) -> Self::Output {
        let n = other.columns;
        let mut data = vec![Default::default(); L * n];
        matmul((L, n, M), (&self.data, &other.data, &mut data));
        Self::Output { data, columns: n }
    }
}
impl<T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug, const L: usize>
    Matmul<MatrixDxD<T>> for MatrixSxD<T, L>
{
    type Output = MatrixSxD<T, L>;
    fn matmul(&self, other: &MatrixDxD<T>) -> Self::Output {
        assert_eq!(self.columns, other.rows, "Non-matching columns to rows");

        let (n, m) = (other.columns, self.columns);
        let mut data = vec![Default::default(); L * n];
        matmul((L, n, m), (&self.data, &other.data, &mut data));
        Self::Output { data, columns: n }
    }
}
// MatrixSxS
// --------------------------------------------------
impl<
        T: Debug + Default + Copy + AddAssign + Mul<Output = T>,
        const L: usize,
        const M: usize,
        const N: usize,
    > Matmul<MatrixSxS<T, M, N>> for MatrixSxS<T, L, M>
where
    [(); L * M]:,
    [(); M * N]:,
    [(); L * N]:,
{
    type Output = MatrixSxS<T, L, N>;
    fn matmul(&self, other: &MatrixSxS<T, M, N>) -> Self::Output {
        let mut data = [Default::default(); L * N];
        matmul((L, N, M), (&self.data, &other.data, &mut data));
        Self::Output { data }
    }
}
impl<
        T: Debug + Default + Copy + AddAssign + Mul<Output = T>,
        const L: usize,
        const M: usize,
        const N: usize,
    > Matmul<MatrixDxS<T, N>> for MatrixSxS<T, L, M>
where
    [(); L * M]:,
    [(); L * N]:,
{
    type Output = MatrixSxS<T, L, N>;
    fn matmul(&self, other: &MatrixDxS<T, N>) -> Self::Output {
        let mut data = [Default::default(); L * N];
        matmul((L, N, M), (&self.data, &other.data, &mut data));
        Self::Output { data }
    }
}
impl<
        T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug,
        const L: usize,
        const M: usize,
    > Matmul<MatrixSxD<T, M>> for MatrixSxS<T, L, M>
where
    [(); L * M]:,
{
    type Output = MatrixSxD<T, L>;
    fn matmul(&self, other: &MatrixSxD<T, M>) -> Self::Output {
        let n = other.columns;
        let mut data = vec![Default::default(); L * n];
        matmul((L, n, M), (&self.data, &other.data, &mut data));
        Self::Output { data, columns: n }
    }
}
impl<
        T: Debug + Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug,
        const L: usize,
        const M: usize,
    > Matmul<MatrixDxD<T>> for MatrixSxS<T, L, M>
where
    [(); L * M]:,
{
    type Output = MatrixSxD<T, L>;
    fn matmul(&self, other: &MatrixDxD<T>) -> Self::Output {
        assert_eq!(M, other.rows, "Non-matching columns to rows");

        let n = other.columns;
        let mut data = vec![Default::default(); L * n];
        matmul((L, n, M), (&self.data, &other.data, &mut data));
        Self::Output { data, columns: n }
    }
}
// Tests
// --------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::*;
    use std::convert::TryFrom;
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
        let a = MatrixSxS::from([[1, 3, 5], [2, 4, 6]]);
        let b = MatrixSxS::from([[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(&b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
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
