
use criterion::{criterion_main};
use crate::benchmarks::benches;

const MX_TASKS: usize = 4; // Assuming, there are 4 cpus.

criterion_main!(benches);
mod benchmarks {
    use std::time::Duration;
    use criterion::{BenchmarkId, Criterion, criterion_group};
    use naturall_e_including_asynch::inner::{calculate_e_parallely_internal, calculate_e_using_handrolled_loop_internal, calculate_e_asynchronously_internal};
    use crate::MX_TASKS;

    fn bench_calculation_of_epsilon(c: &mut Criterion) {

        // For benchmarking the calculation using tokio tasks, a Tokio runtime is required.
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("benchmark-thread")
            .worker_threads(MX_TASKS)
            .build()
            .expect("Creating Tokio runtime");

        let mut group = c.benchmark_group("calculate_e");
        for i in [1_000u64, 10_000u64, 100_000u64, 10_000_000u64].iter() {
            group.measurement_time(Duration::from_secs(7)).bench_with_input(
               BenchmarkId::new("handrolled_loop", i),
               i,
               |b, i| b.iter(|| calculate_e_using_handrolled_loop_internal(*i))
            );
            group.measurement_time(Duration::from_secs(7)).bench_with_input(
                BenchmarkId::new("functional", i),
                i,
               |b, i| b.iter(|| calculate_e_parallely_internal(*i))
            );

            group.bench_with_input(BenchmarkId::new("async_task", i), &i, |b, &s| {
                // Insert a call to `to_async` to convert the bencher to async mode.
                // The timing loops are the same as with the normal bencher.
                b.to_async(&runtime).iter(|| calculate_e_asynchronously_internal(*s));
            });

        }
        group.finish();
    }

    criterion_group!(
           benches,
           bench_calculation_of_epsilon
    );

}
