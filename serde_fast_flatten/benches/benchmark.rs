use criterion::{
    black_box, criterion_group, criterion_main,
    measurement::{Measurement, WallTime},
    Bencher, Criterion,
};
use serde::{de::DeserializeOwned, Serialize};

trait Benchmark<M: Measurement> {
    fn benchmark<T: Serialize + DeserializeOwned, F: Fn() -> T>(f: F) -> impl Fn(&mut Bencher<M>);
}

struct SerializeBenchmark;

impl<M: Measurement> Benchmark<M> for SerializeBenchmark {
    fn benchmark<T: Serialize + DeserializeOwned, F: Fn() -> T>(
        test_value: F,
    ) -> impl Fn(&mut Bencher<M>) {
        move |b| {
            let xs = (0..1000).map(|_| test_value()).collect::<Vec<_>>();
            b.iter(move || serde_json::to_string(black_box(&xs)).unwrap())
        }
    }
}

struct DeserializeBenchmark;

impl<M: Measurement> Benchmark<M> for DeserializeBenchmark {
    fn benchmark<T: Serialize + DeserializeOwned, F: Fn() -> T>(
        test_value: F,
    ) -> impl Fn(&mut Bencher<M>) {
        move |b| {
            let xs = (0..1000).map(|_| test_value()).collect::<Vec<_>>();
            let s = serde_json::to_string(&xs).unwrap();
            b.iter(move || serde_json::from_str::<Vec<T>>(&s).unwrap())
        }
    }
}

fn benchmark_group<B: Benchmark<M>, M: Measurement>(c: &mut Criterion<M>, id: &str) {
    let mut group = c.benchmark_group(id);
    group.bench_function(
        "serde_flatten",
        B::benchmark(serde_fast_flatten::test_structs::serde_flatten::test_value),
    );
    group.bench_function(
        "serde_unflattened",
        B::benchmark(serde_fast_flatten::test_structs::serde_unflattened::test_value),
    );
    group.bench_function(
        "fast_flatten",
        B::benchmark(serde_fast_flatten::test_structs::fast_flatten::test_value),
    );
    group.finish();
}

fn benchmark(c: &mut Criterion) {
    benchmark_group::<SerializeBenchmark, WallTime>(c, "serialize");
    benchmark_group::<DeserializeBenchmark, WallTime>(c, "deserialize");
}

criterion_group!(bench, benchmark);
criterion_main!(bench);
