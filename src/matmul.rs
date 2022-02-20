use crate::*;
use std::convert::TryInto;
use std::ops::{AddAssign, Mul};

/// A trait for matrix multiplication.
pub trait Matmul<T> {
    type Output;
    /// This is somewhat inefficient, it will often not be as efficient as [`std::vec::Vec::push`].
    /// ```text
    /// ┌───────┐        ┌─────┐  ┌─────┐
    /// │ 1 1 1 │        │ 1 2 │  │ 4 5 │
    /// │ 2 1 2 │.matmul(│ 2 1 │)=│ 6 9 │
    /// └───────┘        │ 1 2 │  └─────┘
    ///                  └─────┘
    /// ```
    fn matmul(self, other: T) -> Self::Output;
}

// - L: Rows of `self` and rows of `Self::Output`
// - M: Columns of `self` and rows of `other`
// - N: Columns of `other` and rows of `Self::Output`

// MatrixDxD
// --------------------------------------------------
impl<T: Default + Copy + AddAssign + Mul<Output = T>, const M: usize, const N: usize>
    Matmul<MatrixSxS<T, M, N>> for MatrixDxD<T>
{
    type Output = MatrixDxS<T, N>;
    fn matmul(self, other: MatrixSxS<T, M, N>) -> Self::Output {
        assert_eq!(self.data[0].len(), M, "Non-matching columns to rows");

        let temp_l = self.data.len();
        let mut data = vec![[Default::default(); N]; temp_l];
        for l in 0..temp_l {
            for n in 0..N {
                for m in 0..M {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
impl<T: Default + Copy + AddAssign + Mul<Output = T>, const N: usize> Matmul<MatrixDxS<T, N>>
    for MatrixDxD<T>
{
    type Output = MatrixDxS<T, N>;
    fn matmul(self, other: MatrixDxS<T, N>) -> Self::Output {
        assert_eq!(
            self.data[0].len(),
            other.data.len(),
            "Non-matching columns to rows"
        );

        let temp_l = self.data.len();
        let temp_m = self.data[0].len();
        let mut data = vec![[Default::default(); N]; temp_l];
        for l in 0..temp_l {
            for n in 0..N {
                for m in 0..temp_m {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
impl<T: Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug, const M: usize>
    Matmul<MatrixSxD<T, M>> for MatrixDxD<T>
{
    type Output = MatrixDxD<T>;
    fn matmul(self, other: MatrixSxD<T, M>) -> Self::Output {
        let temp_l = self.data.len();
        let temp_n = other.data[0].len();
        let mut data = Vec::with_capacity(temp_l);
        for l in 0..temp_l {
            data.push(vec![Default::default(); temp_n]);
            for n in 0..temp_n {
                for m in 0..M {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
impl<T: Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug> Matmul<MatrixDxD<T>>
    for MatrixDxD<T>
{
    type Output = MatrixDxD<T>;
    fn matmul(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(
            self.data[0].len(),
            other.data.len(),
            "Non-matching columns to rows"
        );

        let temp_l = self.data.len();
        let temp_m = self.data[0].len();
        let temp_n = other.data[0].len();
        let mut data = Vec::with_capacity(temp_l);
        for l in 0..temp_l {
            data.push(vec![Default::default(); temp_n]);
            for n in 0..temp_n {
                for m in 0..temp_m {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
// MatrixDxS
// --------------------------------------------------
impl<T: Default + Copy + AddAssign + Mul<Output = T>, const M: usize, const N: usize>
    Matmul<MatrixSxS<T, M, N>> for MatrixDxS<T, M>
{
    type Output = MatrixDxS<T, N>;
    fn matmul(self, other: MatrixSxS<T, M, N>) -> Self::Output {
        let temp_l = self.data.len();
        let mut data = vec![[Default::default(); N]; temp_l];
        for l in 0..temp_l {
            for n in 0..N {
                for m in 0..M {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
impl<T: Default + Copy + AddAssign + Mul<Output = T>, const M: usize, const N: usize>
    Matmul<MatrixDxS<T, N>> for MatrixDxS<T, M>
{
    type Output = MatrixDxS<T, N>;
    fn matmul(self, other: MatrixDxS<T, N>) -> Self::Output {
        let temp_l = self.data.len();
        let mut data = vec![[Default::default(); N]; temp_l];
        for l in 0..temp_l {
            for n in 0..N {
                for m in 0..M {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
impl<T: Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug, const M: usize>
    Matmul<MatrixSxD<T, M>> for MatrixDxS<T, M>
{
    type Output = MatrixDxD<T>;
    fn matmul(self, other: MatrixSxD<T, M>) -> Self::Output {
        let temp_l = self.data.len();
        let temp_n = other.data[0].len();
        let mut data = Vec::with_capacity(temp_l);
        for l in 0..temp_l {
            data.push(vec![Default::default(); temp_n]);
            for n in 0..temp_n {
                for m in 0..M {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
impl<T: Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug, const M: usize>
    Matmul<MatrixDxD<T>> for MatrixDxS<T, M>
{
    type Output = MatrixDxD<T>;
    fn matmul(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(M, other.data.len(), "Non-matching columns to rows");

        let temp_l = self.data.len();
        let temp_n = other.data[0].len();
        let mut data = Vec::with_capacity(temp_l);
        for l in 0..temp_l {
            data.push(vec![Default::default(); temp_n]);
            for n in 0..temp_n {
                for m in 0..M {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
// MatrixSxD
// --------------------------------------------------
impl<
        T: Default + Copy + AddAssign + Mul<Output = T>,
        const L: usize,
        const M: usize,
        const N: usize,
    > Matmul<MatrixSxS<T, M, N>> for MatrixSxD<T, L>
{
    type Output = MatrixSxS<T, L, N>;
    fn matmul(self, other: MatrixSxS<T, M, N>) -> Self::Output {
        let mut data = [[Default::default(); N]; L];
        for l in 0..L {
            for n in 0..N {
                // data[l][n] = self.data[l].iter().zip(other.data[..][n]).map(|(a,b)|*a*b).sum();
                for m in 0..M {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
impl<T: Default + Copy + AddAssign + Mul<Output = T>, const L: usize, const N: usize>
    Matmul<MatrixDxS<T, N>> for MatrixSxD<T, L>
{
    type Output = MatrixSxS<T, L, N>;
    fn matmul(self, other: MatrixDxS<T, N>) -> Self::Output {
        assert_eq!(
            self.data[0].len(),
            other.data.len(),
            "Non-matching columns to rows"
        );
        let mut data = [[Default::default(); N]; L];
        for l in 0..L {
            for n in 0..N {
                for m in 0..self.data[0].len() {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
impl<
        T: Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug,
        const L: usize,
        const M: usize,
    > Matmul<MatrixSxD<T, M>> for MatrixSxD<T, L>
{
    type Output = MatrixSxD<T, L>;
    fn matmul(self, other: MatrixSxD<T, M>) -> Self::Output {
        let n_temp = other.data[0].len();
        let mut data: [Vec<T>; L] = (0..L)
            .map(|_| vec![Default::default(); n_temp])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        for l in 0..L {
            for n in 0..n_temp {
                for m in 0..M {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
impl<T: Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug, const L: usize>
    Matmul<MatrixDxD<T>> for MatrixSxD<T, L>
{
    type Output = MatrixSxD<T, L>;
    fn matmul(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(
            self.data[0].len(),
            other.data.len(),
            "Non-matching columns to rows"
        );

        let n_temp = other.data[0].len();
        let mut data: [Vec<T>; L] = (0..L)
            .map(|_| vec![Default::default(); n_temp])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        for l in 0..L {
            for n in 0..n_temp {
                for m in 0..self.data[0].len() {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
// MatrixSxS
// --------------------------------------------------
impl<
        T: Default + Copy + AddAssign + Mul<Output = T>,
        const L: usize,
        const M: usize,
        const N: usize,
    > Matmul<MatrixSxS<T, M, N>> for MatrixSxS<T, L, M>
{
    type Output = MatrixSxS<T, L, N>;
    fn matmul(self, other: MatrixSxS<T, M, N>) -> Self::Output {
        let mut data = [[Default::default(); N]; L];
        for l in 0..L {
            for n in 0..N {
                // data[l][n] = self.data[l].iter().zip(other.data[..][n]).map(|(a,b)|*a*b).sum();
                for m in 0..M {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
impl<
        T: Default + Copy + AddAssign + Mul<Output = T>,
        const L: usize,
        const M: usize,
        const N: usize,
    > Matmul<MatrixDxS<T, N>> for MatrixSxS<T, L, M>
{
    type Output = MatrixSxS<T, L, N>;
    fn matmul(self, other: MatrixDxS<T, N>) -> Self::Output {
        let mut data = [[Default::default(); N]; L];
        for l in 0..L {
            for n in 0..N {
                for m in 0..M {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
impl<
        T: Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug,
        const L: usize,
        const M: usize,
    > Matmul<MatrixSxD<T, M>> for MatrixSxS<T, L, M>
{
    type Output = MatrixSxD<T, L>;
    fn matmul(self, other: MatrixSxD<T, M>) -> Self::Output {
        let n_temp = other.data[0].len();
        let mut data: [Vec<T>; L] = (0..L)
            .map(|_| vec![Default::default(); n_temp])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        for l in 0..L {
            for n in 0..n_temp {
                for m in 0..M {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
    }
}
impl<
        T: Default + Copy + AddAssign + Mul<Output = T> + std::fmt::Debug,
        const L: usize,
        const M: usize,
    > Matmul<MatrixDxD<T>> for MatrixSxS<T, L, M>
{
    type Output = MatrixSxD<T, L>;
    fn matmul(self, other: MatrixDxD<T>) -> Self::Output {
        assert_eq!(M, other.data.len(), "Non-matching columns to rows");

        let n_temp = other.data[0].len();
        let mut data: [Vec<T>; L] = (0..L)
            .map(|_| vec![Default::default(); n_temp])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        for l in 0..L {
            for n in 0..n_temp {
                for m in 0..M {
                    data[l][n] += self.data[l][m] * other.data[m][n];
                }
            }
        }
        Self::Output { data }
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
        let c = a.matmul(b);
        let d = MatrixDxD::try_from(vec![vec![76, 103], vec![100, 136]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_dxs() {
        let a = MatrixDxD::try_from(vec![vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(b);
        let d = MatrixDxS::from(vec![[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxd() {
        let a = MatrixDxD::try_from(vec![vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(b);
        let d = MatrixSxD::try_from([vec![76, 103], vec![100, 136]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn dxd_sxs() {
        let a = MatrixDxD::try_from(vec![vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs_dxs() {
        let a = MatrixDxS::from(vec![[1, 3, 5], [2, 4, 6]]);
        let b = MatrixDxS::from(vec![[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(b);
        let d = MatrixDxS::from(vec![[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxd() {
        let a = MatrixDxS::from(vec![[1, 3, 5], [2, 4, 6]]);
        let b = MatrixSxD::try_from([vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_dxd() {
        let a = MatrixDxS::from(vec![[1, 3, 5], [2, 4, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(b);
        let d = MatrixDxS::from(vec![[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn dxs_sxs() {
        let a = MatrixDxS::from(vec![[1, 3, 5], [2, 4, 6]]);
        let b = MatrixSxS::from([[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd_sxd() {
        let a = MatrixSxD::try_from([vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixSxD::try_from([vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(b);
        let d = MatrixSxD::try_from([vec![76, 103], vec![100, 136]]).unwrap();
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxs() {
        let a = MatrixSxD::try_from([vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixDxS::from(vec![[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_sxs() {
        let a = MatrixSxD::try_from([vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixSxS::from([[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxd_dxd() {
        let a = MatrixSxD::try_from([vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
        let b = MatrixDxD::try_from(vec![vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(b);
        let d = MatrixSxD::try_from([vec![76, 103], vec![100, 136]]).unwrap();
        assert_eq!(c, d);
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs_sxs() {
        let a = MatrixSxS::from([[1, 3, 5], [2, 4, 6]]);
        let b = MatrixSxS::from([[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxs() {
        let a = MatrixSxS::from([[1, 3, 5], [2, 4, 6]]);
        let b = MatrixDxS::from(vec![[7, 10], [8, 11], [9, 12]]);
        let c = a.matmul(b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_sxd() {
        let a = MatrixSxS::from([[1, 3, 5], [2, 4, 6]]);
        let b = MatrixSxD::try_from([vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
    #[test]
    fn sxs_dxd() {
        let a = MatrixSxS::from([[1, 3, 5], [2, 4, 6]]);
        let b = MatrixDxD::try_from(vec![vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
        let c = a.matmul(b);
        let d = MatrixSxS::from([[76, 103], [100, 136]]);
        assert_eq!(c, d);
    }
}
