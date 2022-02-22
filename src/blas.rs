use crate::*;
fn t(x: bool) -> cblas::Transpose {
    match x {
        true => cblas::Transpose::Ordinary,
        false => cblas::Transpose::None,
    }
}
/// A generic matrix.
pub trait Matrix<T> {
    fn rows(&self) -> usize;
    fn columns(&self) -> usize;
    fn data(&self) -> &[T];
    fn data_mut(&mut self) -> &mut [T];
}
impl<T> Matrix<T> for MatrixDxD<T> {
    fn rows(&self) -> usize {
        self.rows
    }
    fn columns(&self) -> usize {
        self.columns
    }
    fn data(&self) -> &[T] {
        &self.data
    }
    fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}
impl<T, const ROWS: usize> Matrix<T> for MatrixSxD<T, ROWS> {
    fn rows(&self) -> usize {
        ROWS
    }
    fn columns(&self) -> usize {
        self.columns
    }
    fn data(&self) -> &[T] {
        &self.data
    }
    fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}
impl<T, const COLUMNS: usize> Matrix<T> for MatrixDxS<T, COLUMNS> {
    fn rows(&self) -> usize {
        self.rows
    }
    fn columns(&self) -> usize {
        COLUMNS
    }
    fn data(&self) -> &[T] {
        &self.data
    }
    fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}
impl<T, const ROWS: usize, const COLUMNS: usize> Matrix<T> for MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
{
    fn rows(&self) -> usize {
        ROWS
    }
    fn columns(&self) -> usize {
        COLUMNS
    }
    fn data(&self) -> &[T] {
        &self.data
    }
    fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

// Level 1
// --------------------------------------------------
/// [saxpy](http://www.netlib.org/lapack/explore-html/d8/daf/saxpy_8f.html) BLAS operation.
pub fn saxpy<X: Matrix<f32>, Y: Matrix<f32>>(alpha: f32, x: &X, y: &mut Y) {
    assert_eq!(x.data().len(), y.data().len(), "Non-matching lengths");
    unsafe {
        cblas::saxpy(x.data().len() as i32, alpha, x.data(), 0, y.data_mut(), 0);
    }
}
/// [daxpy](http://www.netlib.org/lapack/explore-html/d9/dcd/daxpy_8f.html) BLAS operation.
pub fn daxpy<X: Matrix<f64>, Y: Matrix<f64>>(alpha: f64, x: &X, y: &mut Y) {
    assert_eq!(x.data().len(), y.data().len(), "Non-matching lengths");
    unsafe {
        cblas::daxpy(x.data().len() as i32, alpha, x.data(), 0, y.data_mut(), 0);
    }
}
/// [sdot](http://www.netlib.org/lapack/explore-html/d0/d16/sdot_8f.html) BLAS operation.
pub fn sdot<X: Matrix<f32>, Y: Matrix<f32>>(x: &X, y: &Y) -> f32 {
    assert_eq!(x.data().len(), y.data().len(), "Non-matching lengths");
    unsafe { cblas::sdot(x.data().len() as i32, x.data(), 0, y.data(), 0) }
}
/// [ddot](http://www.netlib.org/lapack/explore-html/d5/df6/ddot_8f.html) BLAS operation.
pub fn ddot<X: Matrix<f64>, Y: Matrix<f64>>(x: &X, y: &Y) -> f64 {
    assert_eq!(x.data().len(), y.data().len(), "Non-matching lengths");
    unsafe { cblas::ddot(x.data().len() as i32, x.data(), 0, y.data(), 0) }
}
/// [snrm2](http://www.netlib.org/lapack/explore-html/df/d28/group__single__blas__level1_gad179c1611098b5881f147d39afb009b8.html) BLAS operation.
pub fn snrm2<X: Matrix<f32>>(x: &X) -> f32 {
    unsafe { cblas::snrm2(x.data().len() as i32, x.data(), 0) }
}
/// [dnrm2](http://www.netlib.org/lapack/explore-html/df/d28/group__single__blas__level1_gab5393665c8f0e7d5de9bd1dd2ff0d9d0.html) BLAS operation.
pub fn dnrm2<X: Matrix<f64>>(x: &X) -> f64 {
    unsafe { cblas::dnrm2(x.data().len() as i32, x.data(), 0) }
}
/// [sasum](http://www.netlib.org/lapack/explore-html/df/d1f/sasum_8f.html) BLAS operation.
pub fn sasum<X: Matrix<f32>>(x: &X) -> f32 {
    unsafe { cblas::sasum(x.data().len() as i32, x.data(), 0) }
}
/// [dasum](http://www.netlib.org/lapack/explore-html/de/d05/dasum_8f.html) BLAS operation.
pub fn dasum<X: Matrix<f64>>(x: &X) -> f64 {
    unsafe { cblas::dasum(x.data().len() as i32, x.data(), 0) }
}
/// [isamax](http://www.netlib.org/lapack/explore-html/d6/d44/isamax_8f.html) BLAS operation.
pub fn isamax<X: Matrix<f32>>(x: &X) -> usize {
    unsafe { cblas::isamax(x.data().len() as i32, x.data(), 0) as usize }
}
/// [idamax](http://www.netlib.org/lapack/explore-html/dd/de0/idamax_8f.html) BLAS operation.
pub fn idamax<X: Matrix<f64>>(x: &X) -> usize {
    unsafe { cblas::idamax(x.data().len() as i32, x.data(), 0) as usize }
}
// Level 2
// --------------------------------------------------
/// [sgemm](http://www.netlib.org/lapack/explore-html/d4/de2/sgemm_8f.html) BLAS operation.
pub fn sgemm<A: Matrix<f32>, B: Matrix<f32>, C: Matrix<f32>>(
    transpose_a: bool,
    transpose_b: bool,
    alpha: f32,
    a: &A,
    b: &B,
    beta: f32,
    c: &mut C,
) {
    assert_eq!(a.columns(), b.rows(), "Non-matching columns to rows");
    let (m, n, k) = (a.rows() as i32, b.columns() as i32, a.columns() as i32);
    unsafe {
        cblas::sgemm(
            cblas::Layout::RowMajor,
            t(transpose_a),
            t(transpose_b),
            m,
            n,
            k,
            alpha,
            a.data(),
            k,
            b.data(),
            n,
            beta,
            c.data_mut(),
            n,
        );
    }
}
/// [dgemm](http://www.netlib.org/lapack/explore-html/d7/d2b/dgemm_8f.html) BLAS operation.
pub fn dgemm<A: Matrix<f64>, B: Matrix<f64>, C: Matrix<f64>>(
    transpose_a: bool,
    transpose_b: bool,
    alpha: f64,
    a: &A,
    b: &B,
    beta: f64,
    c: &mut C,
) {
    assert_eq!(a.columns(), b.rows(), "Non-matching columns to rows");
    let (m, n, k) = (a.rows() as i32, b.columns() as i32, a.columns() as i32);
    unsafe {
        cblas::dgemm(
            cblas::Layout::RowMajor,
            t(transpose_a),
            t(transpose_b),
            m,
            n,
            k,
            alpha,
            a.data(),
            k,
            b.data(),
            n,
            beta,
            c.data_mut(),
            n,
        );
    }
}
