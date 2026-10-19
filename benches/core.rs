//! Hot paths a host drives at output/IPC rate: PTY byte streams decoded to
//! text, build-tool stderr classified line by line, log lines pushed into a
//! capped sink, and directory levels listed for a file tree.

use std::hint::black_box;

use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use substrate_platform::fs::dir::list_dir;
use substrate_platform::run::{Level, LogSink};
use substrate_platform::terminal::Utf8Chunker;

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

fn pty_output(c: &mut Criterion) {
    // Terminal output as it comes off a pty: mostly ASCII with ANSI escapes
    // and some multi-byte text, split at arbitrary 4 KiB read boundaries.
    let line = "\x1b[32m✓\x1b[0m test dir::tests::list_dir — ok (世界) 🦀\r\n";
    let stream = line.repeat(64 * 1024 / line.len()).into_bytes();
    let mut group = c.benchmark_group("utf8_chunker");
    group.bench_function("4k_reads", |b| {
        b.iter(|| {
            let mut chunker = Utf8Chunker::new();
            let mut total = 0;
            for chunk in stream.chunks(4096) {
                total += chunker.push(black_box(chunk)).len();
            }
            total + chunker.finish().len()
        })
    });
    group.bench_function("keystroke_echo", |b| {
        b.iter(|| {
            let mut chunker = Utf8Chunker::new();
            let mut total = 0;
            for chunk in stream[..4096].chunks(3) {
                total += chunker.push(black_box(chunk)).len();
            }
            total
        })
    });
    group.finish();
}

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

fn directory_listing(c: &mut Criterion) {
    let dir = std::env::temp_dir().join(format!("substrate-bench-dir-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for i in 0..2_000 {
        if i % 10 == 0 {
            std::fs::create_dir(dir.join(format!("Folder_{i:04}"))).unwrap();
        } else {
            std::fs::File::create(dir.join(format!("Source_File_{i:04}.rs"))).unwrap();
        }
    }
    c.bench_function("list_dir/2000_entries", |b| {
        b.iter(|| list_dir(black_box(&dir)).unwrap().len())
    });
    std::fs::remove_dir_all(&dir).unwrap();
}

criterion_group!(
    benches,
    pty_output,
    stderr_levels,
    log_sink,
    directory_listing
);
criterion_main!(benches);
