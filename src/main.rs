#![allow(dead_code)]
#![allow(unused_variables)]
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
        todo!()
    }
}

struct KadaneSolver;

impl MaximumSubArraySolver for KadaneSolver {
    fn max_sum(&self, arr: &[i32]) -> i32 {
        todo!()
    }
}

fn gen_data(size: usize, seed: u64) -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..size).map(|_| rng.random_range(-100..100)).collect()
}

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
    let data = gen_data(8000, 7);
    println!("{:?}", benchmark_solver(&CubicSolver, &data));
}
