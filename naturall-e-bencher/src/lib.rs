//#![feature(test)]


// extern crate test;
pub mod inner {
    use rand::distributions::uniform::SampleRange;
    use rand::Rng;
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use crate::inner;

    pub fn calculate_e_using_handrolled_loop(n: u64) -> f64 {
        inner::calculate_e_using_handrolled_loop_internal(n)
    }

    pub fn calculate_e_functionally(n: u64) -> f64 {
        inner::calculate_e_functionally_internal(n)
    }


    pub fn calculate_e_using_handrolled_loop_internal(n: u64) -> f64 {
        let mut cnt = 0_f64;
        let mut rng = rand::thread_rng();
        for _ in 1..n {
            let mut j = 0_f32;
            while j < 1.0_f32 {
                let r = rng.gen::<f32>();
                j += r;
                cnt += 1.0_f64;
            }
        }
        cnt / n as f64
    }

    pub fn calculate_e_functionally_internal(n: u64) -> f64 {
        let sum: u64 = (0..n)
            .into_par_iter()
            .map_init(rand::thread_rng, |rng, _| {
                let mut sum = 0f32;
                (1..)
                    .find(|_| {
                        sum += (0.0..1.0).sample_single(rng);
                        sum > 1.0
                    })
                    .expect("never fail")
            })
            .sum();
        sum as f64 / n as f64
    }
}

