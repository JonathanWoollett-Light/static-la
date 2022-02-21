use crate::*;
use rand::distributions::Distribution;
use std::convert::TryInto;

impl<T> MatrixDxD<T> {
    /// Constructs a matrix of a given shape with values sampled from a given distribution.
    /// ```
    /// use static_la::MatrixDxD;
    /// use rand::distributions::{Uniform,Standard};
    /// let a = MatrixDxD::<i32>::distribution(2, 3, Uniform::from(0..10));
    /// let b = MatrixDxD::<i32>::distribution(2, 3, Standard);
    /// ```
    pub fn distribution<DIST: Distribution<T>>(
        rows: usize,
        columns: usize,
        distribution: DIST,
    ) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            data: distribution
                .sample_iter(&mut rng)
                .take(rows * columns)
                .collect(),
            rows,
            columns,
        }
    }
}
impl<T, const COLUMNS: usize> MatrixDxS<T, COLUMNS> {
    /// Constructs a matrix of a given shape with values sampled from a given distribution.
    /// ```
    /// use static_la::MatrixDxS;
    /// use rand::distributions::{Uniform,Standard};
    /// let a = MatrixDxS::<i32,3>::distribution(2, Uniform::from(0..10));
    /// let b = MatrixDxS::<i32,3>::distribution(2, Standard);
    /// ```
    pub fn distribution<DIST: Distribution<T>>(rows: usize, distribution: DIST) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            data: distribution
                .sample_iter(&mut rng)
                .take(rows * COLUMNS)
                .collect(),
            rows,
        }
    }
}
impl<T, const ROWS: usize> MatrixSxD<T, ROWS> {
    /// Constructs a matrix of a given shape with values sampled from a given distribution.
    /// ```
    /// use static_la::MatrixSxD;
    /// use rand::distributions::{Uniform,Standard};
    /// let a = MatrixSxD::<i32,2>::distribution(3, Uniform::from(0..10));
    /// let b = MatrixSxD::<i32,2>::distribution(3, Standard);
    /// ```
    pub fn distribution<DIST: Distribution<T>>(columns: usize, distribution: DIST) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            data: distribution
                .sample_iter(&mut rng)
                .take(ROWS * columns)
                .collect(),
            columns,
        }
    }
}
impl<T: std::fmt::Debug, const ROWS: usize, const COLUMNS: usize> MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    /// Constructs a matrix of a given shape with values sampled from a given distribution.
    /// ```
    /// use static_la::MatrixSxS;
    /// use rand::distributions::{Uniform,Standard};
    /// let a = MatrixSxS::<i32,2,3>::distribution(Uniform::from(0..10));
    /// let b = MatrixSxS::<i32,2,3>::distribution(Standard);
    /// ```
    pub fn distribution<DIST: Distribution<T>>(distribution: DIST) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            data: distribution
                .sample_iter(&mut rng)
                .take(ROWS * COLUMNS)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}
