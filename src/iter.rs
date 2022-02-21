use crate::*;
use std::convert::TryFrom;

/// &[T] -> [&T]
fn ref_arr<T, const N: usize>(x: &[T; N]) -> [&T; N] {
    // TODO: Which is faster this
    #[allow(deref_nullptr)]
    let mut ref_arr: [&T; N] = unsafe { [&*std::ptr::null(); N] };
    for i in 0..N {
        ref_arr[i] = &x[i];
    }
    ref_arr
    // Or this:
    // let ref_arr: [&T;COLUMNS] = r.iter().collect::<Vec<_>>().try_into().unwrap();
}

// TODO: Add mutable iterators over columns and rows (e.g. `RowVectorD<&'a mut T>` and `ColumnVectorD<&'a mut T>`).
impl<'a, T> MatrixDxD<T> {
    /// An iterator over all elements.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        self.data.iter()
    }
    /// A mutable iterator over all elements.
    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.data.iter_mut()
    }
    /// An iterator over rows.
    pub fn rows_iter(&'a self) -> impl Iterator<Item = RowVectorD<&'a T>> {
        self.data.chunks_exact(self.columns).map(|r| {
            // &[T] -> Vec<&T>
            let ref_vec = r.iter().collect::<Vec<_>>();
            RowVectorD::try_from([ref_vec]).unwrap()
        })
    }
    /// An iterator over columns.
    pub fn columns_iter(&'a self) -> impl Iterator<Item = ColumnVectorD<&'a T>> {
        (0..self.columns).map(move |c| {
            let ref_vec = self
                .data
                .iter()
                .skip(c)
                .step_by(self.columns)
                .map(|v| [v])
                .collect::<Vec<_>>();
            ColumnVectorD::from(ref_vec)
        })
    }
}
impl<'a, T: std::fmt::Debug, const ROWS: usize> MatrixSxD<T, ROWS>
where
    [(); ROWS * 1]:,
{
    /// An iterator over all elements.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        self.data.iter()
    }
    /// A mutable iterator over all elements.
    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.data.iter_mut()
    }
    /// An iterator over rows.
    pub fn rows_iter(&'a self) -> impl Iterator<Item = RowVectorD<&'a T>> {
        self.data.chunks_exact(self.columns).map(|r| {
            // &[T] -> Vec<&T>
            let ref_vec = r.iter().collect::<Vec<_>>();
            RowVectorD::try_from([ref_vec]).unwrap()
        })
    }
    /// An iterator over columns.
    pub fn columns_iter(&'a self) -> impl Iterator<Item = ColumnVectorS<&'a T, ROWS>> {
        (0..self.columns).map(move |c| {
            #[allow(deref_nullptr)]
            let mut ref_arr: [[&T; 1]; ROWS] = unsafe { [[&*std::ptr::null(); 1]; ROWS] };
            for (i, v) in self.data.iter().skip(c).step_by(self.columns).enumerate() {
                ref_arr[i] = [v];
            }
            ColumnVectorS::from(ref_arr)
        })
    }
}
impl<'a, T: std::fmt::Debug, const COLUMNS: usize> MatrixDxS<T, COLUMNS>
where
    [(); 1 * COLUMNS]:,
{
    /// An iterator over all elements.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        self.data.iter()
    }
    /// A mutable iterator over all elements.
    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.data.iter_mut()
    }
    /// An iterator over rows.
    pub fn rows_iter(&'a self) -> impl Iterator<Item = RowVectorS<&'a T, COLUMNS>> {
        self.data
            .array_chunks::<COLUMNS>()
            .map(|r| RowVectorS::from([ref_arr(r)]))
    }
    /// An iterator over columns.
    pub fn columns_iter(&'a self) -> impl Iterator<Item = ColumnVectorD<&'a T>> {
        (0..COLUMNS).map(move |c| {
            let ref_vec = self
                .data
                .iter()
                .skip(c)
                .step_by(COLUMNS)
                .map(|v| [v])
                .collect::<Vec<_>>();
            ColumnVectorD::from(ref_vec)
        })
    }
}
impl<'a, T: std::fmt::Debug, const ROWS: usize, const COLUMNS: usize> MatrixSxS<T, ROWS, COLUMNS>
where
    [(); ROWS * COLUMNS]:,
    [(); 1 * COLUMNS]:,
    [(); ROWS * 1]:,
{
    /// An iterator over all elements.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        self.data.iter()
    }
    /// A mutable iterator over all elements.
    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.data.iter_mut()
    }
    /// An iterator over rows.
    pub fn rows_iter(&'a self) -> impl Iterator<Item = RowVectorS<&'a T, COLUMNS>> {
        self.data
            .array_chunks::<COLUMNS>()
            .map(|r| RowVectorS::from([ref_arr(r)]))
    }
    /// An iterator over columns.
    pub fn columns_iter(&'a self) -> impl Iterator<Item = ColumnVectorS<&'a T, ROWS>> {
        (0..COLUMNS).map(move |c| {
            #[allow(deref_nullptr)]
            let mut ref_arr: [[&T; 1]; ROWS] = unsafe { [[&*std::ptr::null(); 1]; ROWS] };
            for (i, v) in self.data.iter().skip(c).step_by(COLUMNS).enumerate() {
                ref_arr[i] = [v];
            }
            ColumnVectorS::from(ref_arr)
        })
    }
}
#[cfg(test)]
mod tests {
    use crate::*;
    use std::convert::TryFrom;
    // MatrixDxD
    // --------------------------------------------------
    #[test]
    fn dxd_rows() {
        let a = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let mut iter = a.rows_iter();
        assert_eq!(
            Some(RowVectorD::try_from([vec![&7, &8, &9]]).unwrap()),
            iter.next()
        );
        assert_eq!(
            Some(RowVectorD::try_from([vec![&10, &11, &12]]).unwrap()),
            iter.next()
        );
        assert_eq!(None, iter.next());
    }
    #[test]
    fn dxd_cols() {
        let a = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let mut iter = a.columns_iter();
        assert_eq!(Some(ColumnVectorD::from(vec![[&7], [&10]])), iter.next());
        assert_eq!(Some(ColumnVectorD::from(vec![[&8], [&11]])), iter.next());
        assert_eq!(Some(ColumnVectorD::from(vec![[&9], [&12]])), iter.next());
        assert_eq!(None, iter.next());
    }
    // MatrixDxS
    // --------------------------------------------------
    #[test]
    fn dxs_rows() {
        let a = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let mut iter = a.rows_iter();
        assert_eq!(Some(RowVectorS::from([[&7, &8, &9]])), iter.next());
        assert_eq!(Some(RowVectorS::from([[&10, &11, &12]])), iter.next());
        assert_eq!(None, iter.next());
    }
    #[test]
    fn dxs_cols() {
        let a = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let mut iter = a.columns_iter();
        assert_eq!(Some(ColumnVectorD::from(vec![[&7], [&10]])), iter.next());
        assert_eq!(Some(ColumnVectorD::from(vec![[&8], [&11]])), iter.next());
        assert_eq!(Some(ColumnVectorD::from(vec![[&9], [&12]])), iter.next());
        assert_eq!(None, iter.next());
    }
    // MatrixSxD
    // --------------------------------------------------
    #[test]
    fn sxd_rows() {
        let a = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let mut iter = a.rows_iter();
        assert_eq!(
            Some(RowVectorD::try_from([vec![&7, &8, &9]]).unwrap()),
            iter.next()
        );
        assert_eq!(
            Some(RowVectorD::try_from([vec![&10, &11, &12]]).unwrap()),
            iter.next()
        );
        assert_eq!(None, iter.next());
    }
    #[test]
    fn sxd_cols() {
        let a = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let mut iter = a.columns_iter();
        assert_eq!(Some(ColumnVectorS::from([[&7], [&10]])), iter.next());
        assert_eq!(Some(ColumnVectorS::from([[&8], [&11]])), iter.next());
        assert_eq!(Some(ColumnVectorS::from([[&9], [&12]])), iter.next());
        assert_eq!(None, iter.next());
    }
    // MatrixSxS
    // --------------------------------------------------
    #[test]
    fn sxs_rows() {
        let a = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let mut iter = a.rows_iter();
        assert_eq!(Some(RowVectorS::from([[&7, &8, &9]])), iter.next());
        assert_eq!(Some(RowVectorS::from([[&10, &11, &12]])), iter.next());
        assert_eq!(None, iter.next());
    }
    #[test]
    fn sxs_cols() {
        let a = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let mut iter = a.columns_iter();
        assert_eq!(Some(ColumnVectorS::from([[&7], [&10]])), iter.next());
        assert_eq!(Some(ColumnVectorS::from([[&8], [&11]])), iter.next());
        assert_eq!(Some(ColumnVectorS::from([[&9], [&12]])), iter.next());
        assert_eq!(None, iter.next());
    }
}
