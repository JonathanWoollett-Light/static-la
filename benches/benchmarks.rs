use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use static_la::*;
use std::convert::{TryFrom, TryInto};

const LARGE: usize = 100;
const RANGE: std::ops::Range<f32> = 1f32..100f32;

fn add_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("addition");
    group.warm_up_time(std::time::Duration::from_millis(100));
    group.measurement_time(std::time::Duration::from_millis(500));
    // Small
    // --------------------------------------------------
    let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
    // TODO: Can I avoid measuring the clone time here?
    group.bench_function("MatrixDxD std::ops::add (s)", |bench| {
        bench.iter(|| a.clone() + b.clone())
    });

    let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
    let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
    group.bench_function("MatrixDxS std::ops::add (s)", |bench| {
        bench.iter(|| a.clone() + b.clone())
    });

    let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
    group.bench_function("MatrixSxD std::ops::add (s)", |bench| {
        bench.iter(|| a.clone() + b.clone())
    });

    let a = MatrixSxS::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
    let b = MatrixSxS::<i32, 2, 3>::from([[7, 8, 9], [10, 11, 12]]);
    group.bench_function("MatrixSxS std::ops::add (s)", |bench| {
        bench.iter(|| a.clone() + b.clone())
    });

    // Large
    // --------------------------------------------------
    let mut rng = rand::thread_rng();

    let a = MatrixDxD::try_from(vec![
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
    ])
    .unwrap();
    let b = MatrixDxD::try_from(vec![
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
    ])
    .unwrap();
    group.bench_function("MatrixDxD std::ops::add (l)", |bench| {
        bench.iter(|| a.clone() + b.clone())
    });

    let a = MatrixDxS::<_, LARGE>::from(vec![
        (0..LARGE)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        (0..LARGE)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    ]);
    let b = MatrixDxS::<_, LARGE>::from(vec![
        (0..LARGE)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        (0..LARGE)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    ]);
    group.bench_function("MatrixDxS std::ops::add (l)", |bench| {
        bench.iter(|| a.clone() + b.clone())
    });

    let a = MatrixSxD::try_from([
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
    ])
    .unwrap();
    let b = MatrixSxD::try_from([
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
    ])
    .unwrap();
    group.bench_function("MatrixSxD std::ops::add (l)", |bench| {
        bench.iter(|| a.clone() + b.clone())
    });

    let a = MatrixSxS::<f32, LARGE, LARGE>::from({
        let c: [[f32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [f32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[f32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    let b = MatrixSxS::<f32, LARGE, LARGE>::from({
        let c: [[f32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [f32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[f32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    group.bench_function("MatrixSxS std::ops::add (l)", |bench| {
        bench.iter(|| a.clone() + b.clone())
    });
}

fn sub_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("subtraction");
    group.warm_up_time(std::time::Duration::from_millis(100));
    group.measurement_time(std::time::Duration::from_millis(500));
    // Small
    // --------------------------------------------------
    let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
    // TODO: Can I avoid measuring the clone time here?
    group.bench_function("MatrixDxD std::ops::sub (s)", |bench| {
        bench.iter(|| a.clone() - b.clone())
    });

    let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
    let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
    group.bench_function("MatrixDxS std::ops::sub (s)", |bench| {
        bench.iter(|| a.clone() - b.clone())
    });

    let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
    group.bench_function("MatrixSxD std::ops::sub (s)", |bench| {
        bench.iter(|| a.clone() - b.clone())
    });

    let a = MatrixSxS::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
    let b = MatrixSxS::<i32, 2, 3>::from([[7, 8, 9], [10, 11, 12]]);
    group.bench_function("MatrixSxS std::ops::sub (s)", |bench| {
        bench.iter(|| a.clone() - b.clone())
    });

    // Large
    // --------------------------------------------------
    let mut rng = rand::thread_rng();

    let a = MatrixDxD::try_from(vec![
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
    ])
    .unwrap();
    let b = MatrixDxD::try_from(vec![
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
    ])
    .unwrap();
    group.bench_function("MatrixDxD std::ops::sub (l)", |bench| {
        bench.iter(|| a.clone() - b.clone())
    });

    let a = MatrixDxS::<_, LARGE>::from(vec![
        (0..LARGE)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        (0..LARGE)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    ]);
    let b = MatrixDxS::<_, LARGE>::from(vec![
        (0..LARGE)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        (0..LARGE)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    ]);
    group.bench_function("MatrixDxS std::ops::sub (l)", |bench| {
        bench.iter(|| a.clone() - b.clone())
    });

    let a = MatrixSxD::try_from([
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
    ])
    .unwrap();
    let b = MatrixSxD::try_from([
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
    ])
    .unwrap();
    group.bench_function("MatrixSxD std::ops::sub (l)", |bench| {
        bench.iter(|| a.clone() - b.clone())
    });

    let a = MatrixSxS::<f32, LARGE, LARGE>::from({
        let c: [[f32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [f32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[f32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    let b = MatrixSxS::<f32, LARGE, LARGE>::from({
        let c: [[f32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [f32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[f32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    group.bench_function("MatrixSxS std::ops::sub (l)", |bench| {
        bench.iter(|| a.clone() - b.clone())
    });
}

fn div_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("division");
    group.warm_up_time(std::time::Duration::from_millis(100));
    group.measurement_time(std::time::Duration::from_millis(500));
    // Small
    // --------------------------------------------------
    let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
    // TODO: Can I avoid measuring the clone time here?
    group.bench_function("MatrixDxD std::ops::div (s)", |bench| {
        bench.iter(|| a.clone() / b.clone())
    });

    let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
    let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
    group.bench_function("MatrixDxS std::ops::div (s)", |bench| {
        bench.iter(|| a.clone() / b.clone())
    });

    let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
    group.bench_function("MatrixSxD std::ops::div (s)", |bench| {
        bench.iter(|| a.clone() / b.clone())
    });

    let a = MatrixSxS::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
    let b = MatrixSxS::<i32, 2, 3>::from([[7, 8, 9], [10, 11, 12]]);
    group.bench_function("MatrixSxS std::ops::div (s)", |bench| {
        bench.iter(|| a.clone() / b.clone())
    });

    // Large
    // --------------------------------------------------
    let mut rng = rand::thread_rng();

    let a = MatrixDxD::try_from(vec![
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
    ])
    .unwrap();
    let b = MatrixDxD::try_from(vec![
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
    ])
    .unwrap();
    group.bench_function("MatrixDxD std::ops::div (l)", |bench| {
        bench.iter(|| a.clone() / b.clone())
    });

    let a = MatrixDxS::<_, LARGE>::from(vec![
        (0..LARGE)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        (0..LARGE)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    ]);
    let b = MatrixDxS::<_, LARGE>::from(vec![
        (0..LARGE)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        (0..LARGE)
            .map(|_| rng.gen_range(RANGE))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    ]);
    group.bench_function("MatrixDxS std::ops::div (l)", |bench| {
        bench.iter(|| a.clone() / b.clone())
    });

    let a = MatrixSxD::try_from([
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
    ])
    .unwrap();
    let b = MatrixSxD::try_from([
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
        (0..LARGE).map(|_| rng.gen_range(RANGE)).collect(),
    ])
    .unwrap();
    group.bench_function("MatrixSxD std::ops::div (l)", |bench| {
        bench.iter(|| a.clone() / b.clone())
    });

    let a = MatrixSxS::<f32, LARGE, LARGE>::from({
        let c: [[f32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [f32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[f32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    let b = MatrixSxS::<f32, LARGE, LARGE>::from({
        let c: [[f32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [f32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[f32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    group.bench_function("MatrixSxS std::ops::div (l)", |bench| {
        bench.iter(|| a.clone() / b.clone())
    });
}

fn mul_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("multiplication");
    // group.warm_up_time(std::time::Duration::from_millis(100));
    // group.measurement_time(std::time::Duration::from_millis(500));
    // Small
    // --------------------------------------------------
    let a = MatrixDxD::try_from(vec![vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    let b = MatrixDxD::try_from(vec![vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
    // TODO: Can I avoid measuring the clone time here?
    group.bench_function("MatrixDxD std::ops::mul (s)", |bench| {
        bench.iter(|| a.clone() * b.clone())
    });

    let a = MatrixDxS::from(vec![[1, 2, 3], [4, 5, 6]]);
    let b = MatrixDxS::from(vec![[7, 8, 9], [10, 11, 12]]);
    group.bench_function("MatrixDxS std::ops::mul (s)", |bench| {
        bench.iter(|| a.clone() * b.clone())
    });

    let a = MatrixSxD::try_from([vec![1, 2, 3], vec![4, 5, 6]]).unwrap();
    let b = MatrixSxD::try_from([vec![7, 8, 9], vec![10, 11, 12]]).unwrap();
    group.bench_function("MatrixSxD std::ops::mul (s)", |bench| {
        bench.iter(|| a.clone() * b.clone())
    });

    let a = MatrixSxS::<i32, 2, 3>::from([[1, 2, 3], [4, 5, 6]]);
    let b = MatrixSxS::<i32, 2, 3>::from([[7, 8, 9], [10, 11, 12]]);
    group.bench_function("MatrixSxS std::ops::mul (s)", |bench| {
        bench.iter(|| a.clone() * b.clone())
    });

    // Large
    // --------------------------------------------------
    let mut rng = rand::thread_rng();

    let a = MatrixDxD::try_from(
        (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let b = MatrixDxD::try_from(
        (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
    .unwrap();
    group.bench_function("MatrixDxD std::ops::mul (l)", |bench| {
        bench.iter(|| a.clone() * b.clone())
    });

    let a = MatrixDxS::<_, LARGE>::from({
        let c: Vec<[f32; LARGE]> = (0..LARGE)
            .map(|_| {
                (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[f32; LARGE]>>();
        c
    });
    let b = MatrixDxS::<_, LARGE>::from({
        let c: Vec<[f32; LARGE]> = (0..LARGE)
            .map(|_| {
                (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[f32; LARGE]>>();
        c
    });
    group.bench_function("MatrixDxS std::ops::mul (l)", |bench| {
        bench.iter(|| a.clone() * b.clone())
    });

    let a = MatrixSxD::try_from({
        let c: [Vec<f32>; LARGE] = (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        c
    })
    .unwrap();
    let b = MatrixSxD::try_from({
        let c: [Vec<f32>; LARGE] = (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        c
    })
    .unwrap();
    group.bench_function("MatrixSxD std::ops::mul (l)", |bench| {
        bench.iter(|| a.clone() * b.clone())
    });

    // TODO Clean up this construct here and in the other benchmarks.
    let a = MatrixSxS::<f32, LARGE, LARGE>::from({
        let c: [[f32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [f32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[f32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    let b = MatrixSxS::<f32, LARGE, LARGE>::from({
        let c: [[f32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [f32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[f32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    group.bench_function("MatrixSxS std::ops::mul (l)", |bench| {
        bench.iter(|| a.clone() * b.clone())
    });
}

// This causes compilers error.
fn _matmul_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix multiplication");
    // group.warm_up_time(std::time::Duration::from_millis(100));
    // group.measurement_time(std::time::Duration::from_millis(500));

    // Small
    // --------------------------------------------------
    let a = MatrixDxD::try_from(vec![vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
    let b = MatrixDxD::try_from(vec![vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
    group.bench_function("MatrixDxD matmul (s)", |bench| bench.iter(|| a.matmul(&b)));

    let a = MatrixDxS::from(vec![[1, 3, 5], [2, 4, 6]]);
    let b = MatrixDxS::from(vec![[7, 10], [8, 11], [9, 12]]);
    group.bench_function("MatrixDxS matmul (s)", |bench| bench.iter(|| a.matmul(&b)));

    let a = MatrixSxD::try_from([vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
    let b = MatrixSxD::try_from([vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
    group.bench_function("MatrixSxD matmul (s)", |bench| bench.iter(|| a.matmul(&b)));

    let a = MatrixSxS::<i32, 2, 3>::from([[1, 3, 5], [2, 4, 6]]);
    let b = MatrixSxS::<i32, 3, 2>::from([[7, 10], [8, 11], [9, 12]]);
    group.bench_function("MatrixSxS matmul (s)", |bench| bench.iter(|| a.matmul(&b)));

    // Large
    // --------------------------------------------------
    let mut rng = rand::thread_rng();

    let a = MatrixDxD::try_from(
        (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let b = MatrixDxD::try_from(
        (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
    .unwrap();
    group.bench_function("MatrixDxD matmul (l)", |bench| bench.iter(|| a.matmul(&b)));

    let a = MatrixDxS::<_, LARGE>::from({
        let c: Vec<[f32; LARGE]> = (0..LARGE)
            .map(|_| {
                (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[f32; LARGE]>>();
        c
    });
    let b = MatrixDxS::<_, LARGE>::from({
        let c: Vec<[f32; LARGE]> = (0..LARGE)
            .map(|_| {
                (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[f32; LARGE]>>();
        c
    });
    group.bench_function("MatrixDxS matmul (l)", |bench| bench.iter(|| a.matmul(&b)));

    let a = MatrixSxD::try_from({
        let c: [Vec<f32>; LARGE] = (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        c
    })
    .unwrap();
    let b = MatrixSxD::try_from({
        let c: [Vec<f32>; LARGE] = (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        c
    })
    .unwrap();
    group.bench_function("MatrixSxD matmul (l)", |bench| bench.iter(|| a.matmul(&b)));

    let a = MatrixSxS::<f32, LARGE, LARGE>::from({
        let c: [[f32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [f32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[f32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    let b = MatrixSxS::<f32, LARGE, LARGE>::from({
        let c: [[f32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [f32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[f32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    group.bench_function("MatrixSxS matmul (l)", |bench| bench.iter(|| a.matmul(&b)));
}

fn sum_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum");
    // group.warm_up_time(std::time::Duration::from_millis(100));
    // group.measurement_time(std::time::Duration::from_millis(500));

    // Small
    // --------------------------------------------------
    let a = MatrixDxD::try_from(vec![vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
    group.bench_function("MatrixDxD sum (s)", |bench| bench.iter(|| a.sum()));
    group.bench_function("MatrixDxD row_sum (s)", |bench| bench.iter(|| a.row_sum()));
    group.bench_function("MatrixDxD column_sum (s)", |bench| {
        bench.iter(|| a.column_sum())
    });

    let a = MatrixDxS::from(vec![[1, 3, 5], [2, 4, 6]]);
    group.bench_function("MatrixDxS sum (s)", |bench| bench.iter(|| a.sum()));
    group.bench_function("MatrixDxS row_sum (s)", |bench| bench.iter(|| a.row_sum()));
    group.bench_function("MatrixDxS column_sum (s)", |bench| {
        bench.iter(|| a.column_sum())
    });

    let a = MatrixSxD::try_from([vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
    group.bench_function("MatrixSxD sum (s)", |bench| bench.iter(|| a.sum()));
    group.bench_function("MatrixSxD row_sum (s)", |bench| bench.iter(|| a.row_sum()));
    group.bench_function("MatrixSxD column_sum (s)", |bench| {
        bench.iter(|| a.column_sum())
    });

    let a = MatrixSxS::<i32, 2, 3>::from([[1, 3, 5], [2, 4, 6]]);
    group.bench_function("MatrixSxS sum (s)", |bench| bench.iter(|| a.sum()));
    group.bench_function("MatrixSxS row_sum (s)", |bench| bench.iter(|| a.row_sum()));
    group.bench_function("MatrixSxS column_sum (s)", |bench| {
        bench.iter(|| a.column_sum())
    });

    // Large
    // --------------------------------------------------
    let mut rng = rand::thread_rng();

    let a = MatrixDxD::try_from(
        (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
    .unwrap();
    group.bench_function("MatrixDxD sum (l)", |bench| bench.iter(|| a.sum()));
    group.bench_function("MatrixDxD row_sum (l)", |bench| bench.iter(|| a.row_sum()));
    group.bench_function("MatrixDxD column_sum (l)", |bench| {
        bench.iter(|| a.column_sum())
    });

    let a = MatrixDxS::<_, LARGE>::from({
        let c: Vec<[f32; LARGE]> = (0..LARGE)
            .map(|_| {
                (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[f32; LARGE]>>();
        c
    });
    group.bench_function("MatrixDxS sum (l)", |bench| bench.iter(|| a.sum()));
    group.bench_function("MatrixDxS row_sum (l)", |bench| bench.iter(|| a.row_sum()));
    group.bench_function("MatrixDxS column_sum (l)", |bench| {
        bench.iter(|| a.column_sum())
    });

    let a = MatrixSxD::try_from({
        let c: [Vec<f32>; LARGE] = (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        c
    })
    .unwrap();
    group.bench_function("MatrixSxD sum (l)", |bench| bench.iter(|| a.sum()));
    group.bench_function("MatrixSxD row_sum (l)", |bench| bench.iter(|| a.row_sum()));
    group.bench_function("MatrixSxD column_sum (l)", |bench| {
        bench.iter(|| a.column_sum())
    });

    let a = MatrixSxS::<f32, LARGE, LARGE>::from({
        let c: [[f32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [f32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[f32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    group.bench_function("MatrixSxS sum (l)", |bench| bench.iter(|| a.sum()));
    group.bench_function("MatrixSxS row_sum (l)", |bench| bench.iter(|| a.row_sum()));
    group.bench_function("MatrixSxS column_sum (l)", |bench| {
        bench.iter(|| a.column_sum())
    });
}

use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

fn static_la_rand(rows: usize, columns: usize) -> MatrixDxD<f32> {
    let mut rng = rand::thread_rng();
    MatrixDxD::try_from(
        (0..rows)
            .map(|_| {
                (0..columns)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
    .unwrap()
}
fn ndarray_rand(rows: usize, columns: usize) -> ndarray::Array2<f32> {
    ndarray::Array::random((rows, columns), Uniform::new(RANGE.start, RANGE.end))
}
fn nalgebra_rand(rows: usize, columns: usize) -> nalgebra::DMatrix<f32> {
    let mut rng = rand::thread_rng();
    nalgebra::base::DMatrix::from_fn(rows, columns, |_, _| rng.gen_range(RANGE))
}

fn dynamic_add_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("std::ops::Add");
    group.warm_up_time(std::time::Duration::from_millis(100));
    group.measurement_time(std::time::Duration::from_millis(500));
    for i in 10..100 {
        group.bench_with_input(BenchmarkId::new("ndarray", i), &i, |b, &i| {
            let x = ndarray_rand(i, i);
            let y = ndarray_rand(i, i);
            b.iter(|| x.clone() + y.clone())
        });
        group.bench_with_input(BenchmarkId::new("nalgebra", i), &i, |b, &i| {
            let x = nalgebra_rand(i, i);
            let y = nalgebra_rand(i, i);
            b.iter(|| x.clone() + y.clone())
        });
        group.bench_with_input(BenchmarkId::new("static-la", i), &i, |b, &i| {
            let x = static_la_rand(i, i);
            let y = static_la_rand(i, i);
            b.iter(|| x.clone() + y.clone())
        });
    }
}
fn dynamic_sub_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("std::ops::Sub");
    group.warm_up_time(std::time::Duration::from_millis(100));
    group.measurement_time(std::time::Duration::from_millis(500));
    for i in 10..100 {
        group.bench_with_input(BenchmarkId::new("ndarray", i), &i, |b, &i| {
            let x = ndarray_rand(i, i);
            let y = ndarray_rand(i, i);
            b.iter(|| x.clone() - y.clone())
        });
        group.bench_with_input(BenchmarkId::new("nalgebra", i), &i, |b, &i| {
            let x = nalgebra_rand(i, i);
            let y = nalgebra_rand(i, i);
            b.iter(|| x.clone() - y.clone())
        });
        group.bench_with_input(BenchmarkId::new("static-la", i), &i, |b, &i| {
            let x = static_la_rand(i, i);
            let y = static_la_rand(i, i);
            b.iter(|| x.clone() - y.clone())
        });
    }
}
fn dynamic_div_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("std::ops::Div");
    group.warm_up_time(std::time::Duration::from_millis(100));
    group.measurement_time(std::time::Duration::from_millis(500));
    for i in 10..100 {
        group.bench_with_input(BenchmarkId::new("ndarray", i), &i, |b, &i| {
            let x = ndarray_rand(i, i);
            let y = ndarray_rand(i, i);
            b.iter(|| x.clone() / y.clone())
        });
        group.bench_with_input(BenchmarkId::new("nalgebra", i), &i, |b, &i| {
            let x = nalgebra_rand(i, i);
            let y = nalgebra_rand(i, i);
            b.iter(|| x.component_div(&y))
        });
        group.bench_with_input(BenchmarkId::new("static-la", i), &i, |b, &i| {
            let x = static_la_rand(i, i);
            let y = static_la_rand(i, i);
            b.iter(|| x.clone() / y.clone())
        });
    }
}
fn dynamic_mul_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("std::ops::Mul");
    group.warm_up_time(std::time::Duration::from_millis(100));
    group.measurement_time(std::time::Duration::from_millis(500));
    for i in 10..100 {
        group.bench_with_input(BenchmarkId::new("ndarray", i), &i, |b, &i| {
            let x = ndarray_rand(i, i);
            let y = ndarray_rand(i, i);
            b.iter(|| x.clone() * y.clone())
        });
        // TODO W
        group.bench_with_input(BenchmarkId::new("nalgebra", i), &i, |b, &i| {
            let x = nalgebra_rand(i, i);
            let y = nalgebra_rand(i, i);
            b.iter(|| x.component_mul(&y))
        });
        group.bench_with_input(BenchmarkId::new("static-la", i), &i, |b, &i| {
            let x = static_la_rand(i, i);
            let y = static_la_rand(i, i);
            b.iter(|| x.clone() * y.clone())
        });
    }
}
fn dynamic_matmul_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matrix multiplication");
    group.warm_up_time(std::time::Duration::from_millis(100));
    group.measurement_time(std::time::Duration::from_millis(500));
    for i in 10..100 {
        group.bench_with_input(BenchmarkId::new("ndarray", i), &i, |b, &i| {
            let x = ndarray_rand(i, i);
            let y = ndarray_rand(i, i);
            b.iter(|| x.dot(&y))
        });
        group.bench_with_input(BenchmarkId::new("nalgebra", i), &i, |b, &i| {
            let x = nalgebra_rand(i, i);
            let y = nalgebra_rand(i, i);
            b.iter(|| x.clone() * y.clone())
        });
        group.bench_with_input(BenchmarkId::new("static-la", i), &i, |b, &i| {
            let x = static_la_rand(i, i);
            let y = static_la_rand(i, i);
            b.iter(|| x.matmul(&y))
        });
    }
}
// fn simple_matmul_benchmark(c: &mut Criterion) {
//     let a = static_la_rand(50, 50);
//     let b = static_la_rand(50, 50);
//     c.bench_function("Simple matmul", |bench| {
//         bench.iter(|| a.clone().matmul(b.clone()))
//     });
// }
// criterion_group!(benches,simple_matmul_benchmark);
// criterion_group!(benches, dynamic_matmul_comparison);
criterion_group!(
    benches,
    add_benchmark,
    sub_benchmark,
    div_benchmark,
    mul_benchmark,
    // matmul_benchmark,
    sum_benchmark,
    dynamic_add_comparison,
    dynamic_sub_comparison,
    dynamic_mul_comparison,
    dynamic_div_comparison,
    dynamic_matmul_comparison
);
criterion_main!(benches);
