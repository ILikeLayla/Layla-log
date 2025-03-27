#![feature(test)]

extern crate test;
use criterion::{criterion_group, criterion_main, Criterion};
#[cfg(not(feature = "async"))]
use std::time::Duration;
#[cfg(feature = "async")]
use tokio::time::Duration;

#[cfg(feature = "async")]
mod bench {
    use criterion::Criterion;
    use layla_log::{clean_log, info, init, Setting};

    fn rt() -> tokio::runtime::Runtime {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .build()
            .unwrap()
    }

    pub fn write_a_lot(c: &mut Criterion) {
        let rt = rt();

        c.bench_function("write_a_lot", |b| {
            b.iter(|| {
                let task = || async {
                    init(Setting::default()).await;
                    clean_log().await;
                    let mut handles = Vec::new();
                    for _ in 0..10_000 {
                        handles.push(tokio::spawn(async { info().await }))
                    }
                    for handle in handles {
                        handle.await.unwrap();
                    }
                };

                rt.block_on(task());
            })
        });
    }

    async fn info() {
        info!("Hello, world!");
    }
}

#[cfg(not(feature = "async"))]
mod bench {
    use criterion::Criterion;
    use layla_log::{clean_log, info, init, Setting};

    pub fn write_a_lot(c: &mut Criterion) {
        c.bench_function("write_a_lot", |b| {
            b.iter(|| {
                init(Setting::default());
                clean_log();
                let mut handles = Vec::new();
                for _ in 0..10_000 {
                    handles.push(std::thread::spawn(|| { info!("Hello, world!"); }))
                }
                for handle in handles {
                    handle.join().unwrap();
                }
            })
        });
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(40));
    targets = bench::write_a_lot
);
criterion_main!(benches);
