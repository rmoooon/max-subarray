use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use rand::{RngExt, SeedableRng, rngs::StdRng};

trait MaximumSubArraySolver {
    fn max_sum(&self, arr: &[i32]) -> i32;
}

struct CubicSolver;

impl MaximumSubArraySolver for CubicSolver {
    fn max_sum(&self, arr: &[i32]) -> i32 {
        let mut best = i32::MIN;
        let size = arr.len();
        for i in 1..size {
            for j in i..size {
                let mut sum = 0;

                for k in &arr[i..j] {
                    sum += k;
                }

                if sum > best {
                    best = sum;
                }
            }
        }

        best
    }
}

struct QuadraticSolver;

impl MaximumSubArraySolver for QuadraticSolver {
    fn max_sum(&self, arr: &[i32]) -> i32 {
        let mut best = i32::MIN;
        let size = arr.len();
        for i in 1..size {
            let mut sum = 0;
            for j in &arr[i..size] {
                sum += j;
                if sum > best {
                    best = sum;
                }
            }
        }

        best
    }
}

struct KadaneSolver;

impl MaximumSubArraySolver for KadaneSolver {
    fn max_sum(&self, arr: &[i32]) -> i32 {
        let mut local = arr[1];
        let mut global = arr[1];
        for &i in arr {
            local = i.max(local + i);
            global = global.max(local);
        }

        global
    }
}

fn gen_data(size: usize, seed: u64) -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..size).map(|_| rng.random_range(-100..100)).collect()
}

#[allow(dead_code)]
#[derive(Debug)]
struct SolverBenchmark {
    duration: Duration,
    result: i32,
}

fn benchmark_solver<T: MaximumSubArraySolver>(solver: &T, arr: &[i32]) -> SolverBenchmark {
    let start = Instant::now();
    let result = black_box(solver.max_sum(arr));
    let duration = start.elapsed();
    SolverBenchmark { duration, result }
}

fn main() {
    const ARR_SIZES: [usize; 4] = [1000, 2000, 4000, 8000];
    const ARR_NUM: u64 = 100;

    for size in ARR_SIZES {
        let mut cubic_total = Duration::ZERO;
        let mut quadratic_total = Duration::ZERO;
        let mut kadane_total = Duration::ZERO;

        for seed in 1..=ARR_NUM {
            let data = gen_data(size, seed);
            cubic_total += benchmark_solver(&CubicSolver, &data).duration;
            quadratic_total += benchmark_solver(&QuadraticSolver, &data).duration;
            kadane_total += benchmark_solver(&KadaneSolver, &data).duration;
        }

        let cubic_avg = cubic_total / ARR_NUM as u32;
        let quadratic_avg = quadratic_total / ARR_NUM as u32;
        let kadane_avg = kadane_total / ARR_NUM as u32;

        println!("Size: {size}");
        println!("\tCubic: {cubic_avg:?}");
        println!("\tQuadratic: {quadratic_avg:?}");
        println!("\tKadane: {kadane_avg:?}");
    }
}
