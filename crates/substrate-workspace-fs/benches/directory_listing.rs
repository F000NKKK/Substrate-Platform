//! Directory levels listed for a file tree.
#![expect(
    clippy::unwrap_used,
    reason = "a benchmark that cannot build or list its fixture has nothing to measure"
)]

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use substrate_workspace_fs::dir::list;

fn directory_listing(c: &mut Criterion) {
    let dir = std::env::temp_dir().join(format!("substrate-bench-dir-{}", std::process::id()));
    std::fs::remove_dir_all(&dir).ok();
    std::fs::create_dir_all(&dir).unwrap();
    for i in 0..2_000 {
        if i % 10 == 0 {
            std::fs::create_dir(dir.join(format!("Folder_{i:04}"))).unwrap();
        } else {
            std::fs::File::create(dir.join(format!("Source_File_{i:04}.rs"))).unwrap();
        }
    }
    c.bench_function("list/2000_entries", |b| {
        b.iter(|| list(black_box(&dir)).unwrap().len());
    });
    std::fs::remove_dir_all(&dir).unwrap();
}

criterion_group!(benches, directory_listing);
criterion_main!(benches);
