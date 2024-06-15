
const ITERATIONS: u64 = 10_000_00;

pub mod inner {
    use rand::distributions::uniform::SampleRange;
    use rand::Rng;
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use tokio::task::{JoinSet};
    use crate::inner;
    fn main() {
        println!("main (inner) called!");
    }

    pub fn calculate_epsilon(n: u64) -> () {
        println!("{}", calculate_e_parallely(n))
    }

    pub fn calculate_e_using_handrolled_loop(n: u64) -> f64 {
        inner::calculate_e_using_handrolled_loop_internal(n)
    }

    pub fn calculate_e_parallely(n: u64) -> f64 {
        inner::calculate_e_parallely_internal(n)
    }

    // Code credit: Ethan Barry (LinkedIn: https://www.linkedin.com/in/ethanbarry/)
    pub fn calculate_e_using_handrolled_loop_internal(n: u64) -> f64 {
        let mut cnt = 0_f64;
        let mut rng = rand::thread_rng();
        for i in 0..n {
            let mut j = 0_f32;
            while j < 1.0_f32 {
                let r = rng.gen::<f32>();
                j += r;
                cnt += 1.0_f64;
            }
        }
        cnt / n as f64
    }

    // Code credit: Sergei Blinov (LinkedIn: https://www.linkedin.com/in/awnion/)
    pub fn calculate_e_parallely_internal(n: u64) -> f64 {
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

    pub async fn calculate_e_asynchronously_internal(n: u64) -> f64 {
        let each_itaration_chunk = n / 4;

        let mut final_sum = 0.0f64;

        let mut s: JoinSet<f64> = JoinSet::new();

        for i in 0..4 {
            let this_chunk = each_itaration_chunk.clone();
            s.spawn (async move {
                //println!("New task launched, with chunk {}", this_chunk);
                let mut rng = rand::thread_rng();
                let mut count = 0.0f64;
                for j  in 0..this_chunk {
                    let mut inner_sum: f32 = 0.0;
                    while inner_sum < 1.0f32 {
                        inner_sum += (0.0..1.0).sample_single(&mut rng);
                        count += 1.0;
                    }
                }
                ;
                count as f64
            });
        }

        while let Some(result) = s.join_next().await {
                match result {
                    Ok(v) => final_sum += v,
                    Err(_) => {}
                }
        };

        final_sum / n as f64
    }
}
