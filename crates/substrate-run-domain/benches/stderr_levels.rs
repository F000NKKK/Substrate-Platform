//! Build-tool stderr classified line by line.

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use substrate_run_domain::Level;

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

fn stderr_levels(c: &mut Criterion) {
    let lines: Vec<String> = STDERR_LINES
        .iter()
        .cycle()
        .take(10_000)
        .map(|l| l.to_string())
        .collect();
    c.bench_function("level_of_stderr_line/10k", |b| {
        b.iter(|| {
            lines
                .iter()
                .filter(|l| Level::of_stderr_line(black_box(l)) != Level::Info)
                .count()
        })
    });
}

criterion_group!(benches, stderr_levels);
criterion_main!(benches);
