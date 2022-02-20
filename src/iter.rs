use crate::*;
use std::convert::{TryFrom, TryInto};
// TODO: Mutable iterators over columns.
impl<'a, T> MatrixDxD<T> {
    /// An iterator over all elements.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        self.data.iter().map(|c| c.iter()).flatten()
    }
    /// A mutable iterator over all elements.
    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.data.iter_mut().map(|c| c.iter_mut()).flatten()
    }
    /// An iterator over rows.
    pub fn rows_iter(&'a self) -> impl Iterator<Item = RowVectorD<&'a T>> {
        self.data.iter().map(|r| {
            let ref_vec = r.iter().collect::<Vec<_>>();
            RowVectorD::try_from([ref_vec]).unwrap()
        })
    }
    /// An iterator over mutable rows.
    pub fn rows_iter_mut(&'a mut self) -> impl Iterator<Item = RowVectorD<&'a mut T>> {
        self.data.iter_mut().map(|r| {
            let ref_vec = r.iter_mut().collect::<Vec<_>>();
            RowVectorD::try_from([ref_vec]).unwrap()
        })
    }
    /// An iterator over columns.
    pub fn columns_iter(&'a self) -> impl Iterator<Item = ColumnVectorD<&'a T>> {
        (0..self.data[0].len()).map(move |c| {
            let ref_vec: Vec<[&T; 1]> = self.data.iter().map(|r| [&r[c]]).collect::<Vec<_>>();
            ColumnVectorD::from(ref_vec)
        })
        // move |c| self.data.iter().map(|r| &r[c]).collect::<Vec<_>>())
    }
}
impl<'a, T, const ROWS: usize> MatrixSxD<T, ROWS> {
    /// An iterator over all elements.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        self.data.iter().map(|c| c.iter()).flatten()
    }
    /// A mutable iterator over all elements.
    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.data.iter_mut().map(|c| c.iter_mut()).flatten()
    }
    /// An iterator over rows.
    pub fn rows_iter(&'a self) -> impl Iterator<Item = RowVectorD<&'a T>> {
        self.data.iter().map(|r| {
            let ref_vec = r.iter().collect::<Vec<_>>();
            RowVectorD::try_from([ref_vec]).unwrap()
        })
    }
    /// An iterator over mutable rows.
    pub fn rows_iter_mut(&'a mut self) -> impl Iterator<Item = RowVectorD<&'a mut T>> {
        self.data.iter_mut().map(|r| {
            let ref_vec = r.iter_mut().collect::<Vec<_>>();
            RowVectorD::try_from([ref_vec]).unwrap()
        })
    }
    /// An iterator over columns.
    pub fn columns_iter(&'a self) -> impl Iterator<Item = ColumnVectorS<&'a T, ROWS>> {
        (0..self.data[0].len()).map(move |c| {
            #[allow(deref_nullptr)]
            let mut ref_vec: [[&T; 1]; ROWS] = unsafe { [[&*std::ptr::null(); 1]; ROWS] };
            for i in 0..ROWS {
                ref_vec[i][0] = &self.data[i][c];
            }
            ColumnVectorS::from(ref_vec)
        })
    }
}
impl<'a, T: std::fmt::Debug, const COLUMNS: usize> MatrixDxS<T, COLUMNS> {
    /// An iterator over all elements.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        self.data.iter().map(|c| c.iter()).flatten()
    }
    /// A mutable iterator over all elements.
    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.data.iter_mut().map(|c| c.iter_mut()).flatten()
    }
    /// An iterator over rows.
    pub fn rows_iter(&'a self) -> impl Iterator<Item = RowVectorS<&'a T, COLUMNS>> {
        self.data.iter().map(|r| {
            #[allow(deref_nullptr)]
            let mut ref_vec: [[&T; COLUMNS]; 1] = unsafe { [[&*std::ptr::null(); COLUMNS]; 1] };
            for i in 0..COLUMNS {
                ref_vec[0][i] = &r[i];
            }
            RowVectorS::from(ref_vec)
        })
    }
    /// An iterator over mutable rows.
    pub fn rows_iter_mut(&'a mut self) -> impl Iterator<Item = RowVectorS<&'a mut T, COLUMNS>> {
        self.data.iter_mut().map(|r| {
            // TODO Do something like:
            // let mut ref_vec: [[&mut T; COLUMNS];1] = unsafe { [[&mut*std::ptr::null_mut();COLUMNS];1] };
            let mut ref_vec: Vec<&mut T> = Vec::with_capacity(COLUMNS);
            for i in 0..COLUMNS {
                // We use a mut pointer to circumvent the borrow checker here, since we know we are
                //  only borrowing each value in `r` mutably once.
                unsafe {
                    let mut_ptr: *mut T = &mut r[i];
                    ref_vec.push(&mut *mut_ptr);
                }
            }
            let ref_arr: [[&mut T; COLUMNS]; 1] = [ref_vec.try_into().unwrap()];
            RowVectorS::from(ref_arr)
        })
    }
    /// An iterator over columns.
    pub fn columns_iter(&'a self) -> impl Iterator<Item = ColumnVectorD<&'a T>> {
        (0..self.data[0].len()).map(move |c| {
            let ref_vec: Vec<[&T; 1]> = self.data.iter().map(|r| [&r[c]]).collect::<Vec<_>>();
            ColumnVectorD::from(ref_vec)
        })
        // move |c| self.data.iter().map(|r| &r[c]).collect::<Vec<_>>())
    }
}
impl<'a, T: std::fmt::Debug, const ROWS: usize, const COLUMNS: usize> MatrixSxS<T, ROWS, COLUMNS> {
    /// An iterator over all elements.
    pub fn iter(&'a self) -> impl Iterator<Item = &'a T> {
        self.data.iter().map(|c| c.iter()).flatten()
    }
    /// A mutable iterator over all elements.
    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.data.iter_mut().map(|c| c.iter_mut()).flatten()
    }
    /// An iterator over rows.
    pub fn rows_iter(&'a self) -> impl Iterator<Item = RowVectorS<&'a T, COLUMNS>> {
        self.data.iter().map(|r| {
            #[allow(deref_nullptr)]
            let mut ref_vec: [[&T; COLUMNS]; 1] = unsafe { [[&*std::ptr::null(); COLUMNS]; 1] };
            for i in 0..COLUMNS {
                ref_vec[0][i] = &r[i];
            }
            RowVectorS::from(ref_vec)
        })
    }
    /// An iterator over mutable rows.
    pub fn rows_iter_mut(&'a mut self) -> impl Iterator<Item = RowVectorS<&'a mut T, COLUMNS>> {
        self.data.iter_mut().map(|r| {
            // TODO Do something like:
            // let mut ref_vec: [[&mut T; COLUMNS];1] = unsafe { [[&mut*std::ptr::null_mut();COLUMNS];1] };
            let mut ref_vec: Vec<&mut T> = Vec::with_capacity(COLUMNS);
            for i in 0..COLUMNS {
                // We use a mut pointer to circumvent the borrow checker here, since we know we are
                //  only borrowing each value in `r` mutably once.
                unsafe {
                    let mut_ptr: *mut T = &mut r[i];
                    ref_vec.push(&mut *mut_ptr);
                }
            }
            let ref_arr: [[&mut T; COLUMNS]; 1] = [ref_vec.try_into().unwrap()];
            RowVectorS::from(ref_arr)
        })
    }
    /// An iterator over columns.
    pub fn columns_iter(&'a self) -> impl Iterator<Item = ColumnVectorS<&'a T, ROWS>> {
        (0..self.data[0].len()).map(move |c| {
            #[allow(deref_nullptr)]
            let mut ref_vec: [[&T; 1]; ROWS] = unsafe { [[&*std::ptr::null(); 1]; ROWS] };
            for i in 0..ROWS {
                ref_vec[i][0] = &self.data[i][c];
            }
            ColumnVectorS::from(ref_vec)
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
    fn dxd_rows_mut() {
        let mut a = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let mut iter = a.rows_iter_mut();
        assert_eq!(
            Some(RowVectorD::try_from([vec![&mut 7, &mut 8, &mut 9]]).unwrap()),
            iter.next()
        );
        assert_eq!(
            Some(RowVectorD::try_from([vec![&mut 10, &mut 11, &mut 12]]).unwrap()),
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
    fn dxs_rows_mut() {
        let mut a = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
        let mut iter = a.rows_iter_mut();
        assert_eq!(
            Some(RowVectorS::from([[&mut 7, &mut 8, &mut 9]])),
            iter.next()
        );
        assert_eq!(
            Some(RowVectorS::from([[&mut 10, &mut 11, &mut 12]])),
            iter.next()
        );
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
    fn sxd_rows_mut() {
        let mut a = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
        let mut iter = a.rows_iter_mut();
        assert_eq!(
            Some(RowVectorD::try_from([vec![&mut 7, &mut 8, &mut 9]]).unwrap()),
            iter.next()
        );
        assert_eq!(
            Some(RowVectorD::try_from([vec![&mut 10, &mut 11, &mut 12]]).unwrap()),
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
    fn sxs_rows_mut() {
        let mut a = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
        let mut iter = a.rows_iter_mut();
        assert_eq!(
            Some(RowVectorS::from([[&mut 7, &mut 8, &mut 9]])),
            iter.next()
        );
        assert_eq!(
            Some(RowVectorS::from([[&mut 10, &mut 11, &mut 12]])),
            iter.next()
        );
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
