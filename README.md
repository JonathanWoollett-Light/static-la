# Static L.A. (Linear Algebra)

An extremely minimal super static type safe implementation of matrix types.

While [`ndarray`](https://docs.rs/ndarray/latest/ndarray/) offers no compile time type checking
 on dimensionality and [`nalgebra`](https://docs.rs/nalgebra/latest/nalgebra/) offers some
 finnicky checking, this offers the maximum possible checking.

When performing the addition of a `MatrixDxS` (a matrix with a known number of columns at
 compile time) and a `MatrixSxD` (a matrix with a known number of rows at compile time) you
 get a `MatrixSxS` (a matrix with a known number of rows and columns at compile time) since
 now both the number of rows and columns are known at compile time. This then allows this
 infomation to propagate through your program providing excellent compile time checking.

That being said... I made this in a weekend, there is a tiny amount of functionality and its ~4x slower than [`ndarray`](https://docs.rs/ndarray/latest/ndarray/) and [`nalgebra`](https://docs.rs/nalgebra/latest/nalgebra/).

An example of how types will propagate through a program:
```rust
use static_la::*;
// MatrixSxS<i32,2,3>
let a = MatrixSxS::from([[1,2,3],[4,5,6]]);
// MatrixDxS<i32,3>
let b = MatrixDxS::from(vec![[2,2,2],[3,3,3]]);
// MatrixSxS<i32,2,3>
let c = (a.clone() + b.clone()) - a.clone();
// MatrixDxS<i32,3>
let d = c.add_rows(b);
// MatrixSxS<i32,4,3>
let e = MatrixSxS::from([[1,2,3],[4,5,6],[7,8,9],[10,11,12]]);
// MatrixSxS<i32,4,6>
let f = d.add_columns(e);
```

In this example the only operations which cannot be fully checked at compile time are:
1. `a.clone() + b.clone()`
2. `d.add_columns(e)`