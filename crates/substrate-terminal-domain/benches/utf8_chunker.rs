//! Terminal output as it comes off a pty: decoded to text as it streams.

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use substrate_terminal_domain::Utf8Chunker;

fn pty_output(c: &mut Criterion) {
    // Mostly ASCII with ANSI escapes and some multi-byte text, split at
    // arbitrary 4 KiB read boundaries.
    let line = "\x1b[32m✓\x1b[0m test dir::tests::list — ok (世界) 🦀\r\n";
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
        });
    });
    group.bench_function("keystroke_echo", |b| {
        b.iter(|| {
            let mut chunker = Utf8Chunker::new();
            let mut total = 0;
            for chunk in stream[..4096].chunks(3) {
                total += chunker.push(black_box(chunk)).len();
            }
            total
        });
    });
    group.finish();
}

criterion_group!(benches, pty_output);
criterion_main!(benches);
