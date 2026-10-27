//! Log lines pushed into a capped sink.

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use substrate_run_app::LogSink;

/// A realistic slice of `cargo build` stderr — mostly progress, some diagnostics.
const STDERR_LINES: &[&str] = &[
    "   Compiling serde v1.0.228",
    "   Compiling proc-macro2 v1.0.107",
    "warning: unused variable: `x`",
    "  --> src/lib.rs:12:9",
    "   |",
    "12 |     let x = 5;",
    "   |         ^ help: if this is intentional, prefix it with an underscore: `_x`",
    "error[E0599]: no method named `pause` found for struct `Foo` in the current scope",
    "    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.51s",
];

fn log_sink(c: &mut Criterion) {
    c.bench_function("log_sink/push_20k_capped", |b| {
        b.iter_batched(
            LogSink::new,
            |sink| {
                for i in 0..20_000 {
                    sink.stderr(STDERR_LINES[i % STDERR_LINES.len()]);
                }
                sink.drain().len()
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, log_sink);
criterion_main!(benches);
