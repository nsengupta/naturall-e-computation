
#![feature(test)]
extern crate test;

use rand::Rng;
use rand::distributions::uniform::SampleRange;
use rayon::prelude::*;
use naturall_e::inner;


const ITERATIONS: u64 = 10_000;

fn main() {
    println!("main called");
}

#[cfg(test)]
mod benchmarks {

    use test::Bencher;
    use naturall_e::inner::{calculate_e_functionally_internal, calculate_e_using_handrolled_loop};

    #[bench]
    fn bench_e_handrolled_while_loop_1K(b: &mut Bencher) {
        b.iter(|| {
            std::hint::black_box( calculate_e_using_handrolled_loop(1_000) );
        });
    }

    #[bench]
    fn bench_e_handrolled_while_loop_10K(b: &mut Bencher) {
        b.iter(|| {
            std::hint::black_box( calculate_e_using_handrolled_loop(10_000) );
        });
    }

    #[bench]
    fn bench_e_handrolled_while_loop_100K(b: &mut Bencher) {
        b.iter(|| {
            std::hint::black_box( calculate_e_using_handrolled_loop(100_000) );
        });
    }

    #[bench]
    fn bench_e_handrolled_while_loop_1000K(b: &mut Bencher) {
        b.iter(|| {
            std::hint::black_box( calculate_e_using_handrolled_loop(10_000_000) );
        });
    }

    #[bench]
    fn bench_e_functional_transformation_1K(b: &mut Bencher) {
        b.iter(|| {
            std::hint::black_box(calculate_e_functionally_internal(1000) );
        });
    }

    #[bench]
    fn bench_e_functional_transformation_10K(b: &mut Bencher) {
        b.iter(|| {
            std::hint::black_box(calculate_e_functionally_internal(10_000) );
        });
    }

    #[bench]
    fn bench_e_functional_transformation_100K(b: &mut Bencher) {
        b.iter(|| {
            std::hint::black_box(calculate_e_functionally_internal(100_000) );
        });
    }

    #[bench]
    fn bench_e_functional_transformation_1000K(b: &mut Bencher) {
        b.iter(|| {
            std::hint::black_box(calculate_e_functionally_internal(10_000_000) );
        });
    }
}
