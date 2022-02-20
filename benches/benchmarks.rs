use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use static_la::*;
use std::convert::{TryFrom, TryInto};

const LARGE: usize = 100;
const RANGE: std::ops::Range<i32> = 1..100;

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

    let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
    let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
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

    let a = MatrixSxS::<i32, LARGE, LARGE>::from({
        let c: [[i32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [i32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[i32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    let b = MatrixSxS::<i32, LARGE, LARGE>::from({
        let c: [[i32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [i32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[i32; LARGE]>>()
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

    let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
    let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
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

    let a = MatrixSxS::<i32, LARGE, LARGE>::from({
        let c: [[i32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [i32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[i32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    let b = MatrixSxS::<i32, LARGE, LARGE>::from({
        let c: [[i32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [i32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[i32; LARGE]>>()
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

    let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
    let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
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

    let a = MatrixSxS::<i32, LARGE, LARGE>::from({
        let c: [[i32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [i32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[i32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    let b = MatrixSxS::<i32, LARGE, LARGE>::from({
        let c: [[i32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [i32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[i32; LARGE]>>()
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

    let a = MatrixSxS::from([[1, 2, 3], [4, 5, 6]]);
    let b = MatrixSxS::from([[7, 8, 9], [10, 11, 12]]);
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
        let c: Vec<[i32; LARGE]> = (0..LARGE)
            .map(|_| {
                (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[i32; LARGE]>>();
        c
    });
    let b = MatrixDxS::<_, LARGE>::from({
        let c: Vec<[i32; LARGE]> = (0..LARGE)
            .map(|_| {
                (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[i32; LARGE]>>();
        c
    });
    group.bench_function("MatrixDxS std::ops::mul (l)", |bench| {
        bench.iter(|| a.clone() * b.clone())
    });

    let a = MatrixSxD::try_from({
        let c: [Vec<i32>; LARGE] = (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        c
    })
    .unwrap();
    let b = MatrixSxD::try_from({
        let c: [Vec<i32>; LARGE] = (0..LARGE)
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
    let a = MatrixSxS::<i32, LARGE, LARGE>::from({
        let c: [[i32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [i32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[i32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    let b = MatrixSxS::<i32, LARGE, LARGE>::from({
        let c: [[i32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [i32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[i32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    group.bench_function("MatrixSxS std::ops::mul (l)", |bench| {
        bench.iter(|| a.clone() * b.clone())
    });
}

fn matmul_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix multiplication");
    // group.warm_up_time(std::time::Duration::from_millis(100));
    // group.measurement_time(std::time::Duration::from_millis(500));

    // Small
    // --------------------------------------------------
    let a = MatrixDxD::try_from(vec![vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
    let b = MatrixDxD::try_from(vec![vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
    group.bench_function("MatrixDxD matmul (s)", |bench| {
        bench.iter(|| a.clone().matmul(b.clone()))
    });

    let a = MatrixDxS::from(vec![[1, 3, 5], [2, 4, 6]]);
    let b = MatrixDxS::from(vec![[7, 10], [8, 11], [9, 12]]);
    group.bench_function("MatrixDxS matmul (s)", |bench| {
        bench.iter(|| a.clone().matmul(b.clone()))
    });

    let a = MatrixSxD::try_from([vec![1, 3, 5], vec![2, 4, 6]]).unwrap();
    let b = MatrixSxD::try_from([vec![7, 10], vec![8, 11], vec![9, 12]]).unwrap();
    group.bench_function("MatrixSxD matmul (s)", |bench| {
        bench.iter(|| a.clone().matmul(b.clone()))
    });

    let a = MatrixSxS::from([[1, 3, 5], [2, 4, 6]]);
    let b = MatrixSxS::from([[7, 10], [8, 11], [9, 12]]);
    group.bench_function("MatrixSxS matmul (s)", |bench| {
        bench.iter(|| a.clone().matmul(b.clone()))
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
    group.bench_function("MatrixDxD matmul (l)", |bench| {
        bench.iter(|| a.clone().matmul(b.clone()))
    });

    let a = MatrixDxS::<_, LARGE>::from({
        let c: Vec<[i32; LARGE]> = (0..LARGE)
            .map(|_| {
                (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[i32; LARGE]>>();
        c
    });
    let b = MatrixDxS::<_, LARGE>::from({
        let c: Vec<[i32; LARGE]> = (0..LARGE)
            .map(|_| {
                (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<[i32; LARGE]>>();
        c
    });
    group.bench_function("MatrixDxS matmul (l)", |bench| {
        bench.iter(|| a.clone().matmul(b.clone()))
    });

    let a = MatrixSxD::try_from({
        let c: [Vec<i32>; LARGE] = (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        c
    })
    .unwrap();
    let b = MatrixSxD::try_from({
        let c: [Vec<i32>; LARGE] = (0..LARGE)
            .map(|_| (0..LARGE).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        c
    })
    .unwrap();
    group.bench_function("MatrixSxD matmul (l)", |bench| {
        bench.iter(|| a.clone().matmul(b.clone()))
    });

    let a = MatrixSxS::<i32, LARGE, LARGE>::from({
        let c: [[i32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [i32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[i32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    let b = MatrixSxS::<i32, LARGE, LARGE>::from({
        let c: [[i32; LARGE]; LARGE] = (0..LARGE)
            .map(|_| {
                let a: [i32; LARGE] = (0..LARGE)
                    .map(|_| rng.gen_range(RANGE))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                a
            })
            .collect::<Vec<[i32; LARGE]>>()
            .try_into()
            .unwrap();
        c
    });
    group.bench_function("MatrixSxS matmul (l)", |bench| {
        bench.iter(|| a.clone().matmul(b.clone()))
    });
}

use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

fn dynamic_add_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("std::ops::Add");
    group.warm_up_time(std::time::Duration::from_millis(50));
    group.measurement_time(std::time::Duration::from_millis(200));
    let mut rng = rand::thread_rng();
    for i in 10..100 {
        group.bench_with_input(BenchmarkId::new("ndarray", i), &i, |b, &i| {
            let x = ndarray::Array::random((i, i), Uniform::new(RANGE.start, RANGE.end));
            let y = ndarray::Array::random((i, i), Uniform::new(RANGE.start, RANGE.end));
            b.iter(|| x.clone() + y.clone())
        });
        group.bench_with_input(BenchmarkId::new("nalgebra", i), &i, |b, &i| {
            let x = nalgebra::base::DMatrix::from_fn(i, i, |_, _| rng.gen_range(RANGE));
            let y = nalgebra::base::DMatrix::from_fn(i, i, |_, _| rng.gen_range(RANGE));
            b.iter(|| x.clone() + y.clone())
        });
        group.bench_with_input(BenchmarkId::new("static-la", i), &i, |b, &i| {
            let x = MatrixDxD::<i32>::try_from(
                (0..i)
                    .map(|_| (0..i).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            let y = MatrixDxD::<i32>::try_from(
                (0..i)
                    .map(|_| (0..i).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            b.iter(|| x.clone() + y.clone())
        });
    }
}
fn dynamic_sub_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("std::ops::Sub");
    group.warm_up_time(std::time::Duration::from_millis(50));
    group.measurement_time(std::time::Duration::from_millis(200));
    let mut rng = rand::thread_rng();
    for i in 10..100 {
        group.bench_with_input(BenchmarkId::new("ndarray", i), &i, |b, &i| {
            let x = ndarray::Array::random((i, i), Uniform::new(RANGE.start, RANGE.end));
            let y = ndarray::Array::random((i, i), Uniform::new(RANGE.start, RANGE.end));
            b.iter(|| x.clone() - y.clone())
        });
        group.bench_with_input(BenchmarkId::new("nalgebra", i), &i, |b, &i| {
            let x = nalgebra::base::DMatrix::from_fn(i, i, |_, _| rng.gen_range(RANGE));
            let y = nalgebra::base::DMatrix::from_fn(i, i, |_, _| rng.gen_range(RANGE));
            b.iter(|| x.clone() - y.clone())
        });
        group.bench_with_input(BenchmarkId::new("static-la", i), &i, |b, &i| {
            let x = MatrixDxD::<i32>::try_from(
                (0..i)
                    .map(|_| (0..i).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            let y = MatrixDxD::<i32>::try_from(
                (0..i)
                    .map(|_| (0..i).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            b.iter(|| x.clone() - y.clone())
        });
    }
}
fn dynamic_div_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("std::ops::Div");
    group.warm_up_time(std::time::Duration::from_millis(50));
    group.measurement_time(std::time::Duration::from_millis(200));
    let mut rng = rand::thread_rng();
    for i in 10..100 {
        group.bench_with_input(BenchmarkId::new("ndarray", i), &i, |b, &i| {
            let x = ndarray::Array::random((i, i), Uniform::new(RANGE.start, RANGE.end));
            let y = ndarray::Array::random((i, i), Uniform::new(RANGE.start, RANGE.end));
            b.iter(|| x.clone() / y.clone())
        });
        // group.bench_with_input(BenchmarkId::new("nalgebra", i), &i, |b, &i| {
        //     let x = nalgebra::base::DMatrix::from_fn(i, i, |_, _| rng.gen_range(RANGE));
        //     let y = nalgebra::base::DMatrix::from_fn(i, i, |_, _| rng.gen_range(RANGE));
        //     b.iter(|| x.clone() / y.clone())
        // });
        group.bench_with_input(BenchmarkId::new("static-la", i), &i, |b, &i| {
            let x = MatrixDxD::<i32>::try_from(
                (0..i)
                    .map(|_| (0..i).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            let y = MatrixDxD::<i32>::try_from(
                (0..i)
                    .map(|_| (0..i).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            b.iter(|| x.clone() / y.clone())
        });
    }
}
fn dynamic_mul_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("std::ops::Mul");
    group.warm_up_time(std::time::Duration::from_millis(50));
    group.measurement_time(std::time::Duration::from_millis(200));
    let mut rng = rand::thread_rng();
    for i in 10..100 {
        group.bench_with_input(BenchmarkId::new("ndarray", i), &i, |b, &i| {
            let x = ndarray::Array::random((i, i), Uniform::new(RANGE.start, RANGE.end));
            let y = ndarray::Array::random((i, i), Uniform::new(RANGE.start, RANGE.end));
            b.iter(|| x.clone() * y.clone())
        });
        group.bench_with_input(BenchmarkId::new("nalgebra", i), &i, |b, &i| {
            let x = nalgebra::base::DMatrix::from_fn(i, i, |_, _| rng.gen_range(RANGE));
            let y = nalgebra::base::DMatrix::from_fn(i, i, |_, _| rng.gen_range(RANGE));
            b.iter(|| x.clone() * y.clone())
        });
        group.bench_with_input(BenchmarkId::new("static-la", i), &i, |b, &i| {
            let x = MatrixDxD::<i32>::try_from(
                (0..i)
                    .map(|_| (0..i).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            let y = MatrixDxD::<i32>::try_from(
                (0..i)
                    .map(|_| (0..i).map(|_| rng.gen_range(RANGE)).collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            b.iter(|| x.clone() * y.clone())
        });
    }
}

criterion_group!(
    benches,
    add_benchmark,
    sub_benchmark,
    div_benchmark,
    mul_benchmark,
    matmul_benchmark,
    dynamic_add_comparison,
    dynamic_sub_comparison,
    dynamic_mul_comparison,
    dynamic_div_comparison
);
criterion_main!(benches);
