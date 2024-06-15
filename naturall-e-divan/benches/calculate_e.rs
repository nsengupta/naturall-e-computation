
const ITERATIONS: u64 = 10_000_00;

fn main() {
    divan::main();
}

mod calculation {
    use divan::Bencher;
    use rand::distributions::uniform::SampleRange;
    use rand::Rng;
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use naturall_e::inner::{calculate_e_functionally_internal, calculate_e_using_handrolled_loop_internal};
    use crate::ITERATIONS;


    #[divan::bench]
    pub fn calculate_e_using_handrolled_loop_1K(bencher: Bencher) -> () {
        bencher.bench_local(|| {
            calculate_e_using_handrolled_loop_internal(1_000)
        })
    }

    #[divan::bench]
    pub fn calculate_e_using_handrolled_loop_10K(bencher: Bencher) -> () {
        bencher.bench_local(|| {
            calculate_e_using_handrolled_loop_internal(10_000)
        })
    }

    #[divan::bench]
    pub fn calculate_e_using_handrolled_loop_100K(bencher: Bencher) -> () {
        bencher.bench_local(|| {
            calculate_e_using_handrolled_loop_internal(100_000)
        })
    }

    #[divan::bench]
    pub fn calculate_e_using_handrolled_loop_1000K(bencher: Bencher) -> () {
        bencher.bench_local(|| {
            calculate_e_using_handrolled_loop_internal(10_000_000)
        })
    }

    #[divan::bench]
    pub fn calculate_e_functionally_1K(bencher: Bencher) -> () {
        bencher.bench_local(|| {
            calculate_e_functionally_internal(1_000)
        })
    }

    #[divan::bench]
    pub fn calculate_e_functionally_10K(bencher: Bencher) -> () {
        bencher.bench_local(|| {
            calculate_e_functionally_internal(10_000)
        })
    }

    #[divan::bench]
    pub fn calculate_e_functionally_100K(bencher: Bencher) -> () {
        bencher.bench_local(|| {
            calculate_e_functionally_internal(100_000)
        })
    }

    #[divan::bench]
    pub fn calculate_e_functionally_1000K(bencher: Bencher) -> () {
        bencher.bench_local(|| {
            calculate_e_functionally_internal(10_000_000)
        })
    }
}
