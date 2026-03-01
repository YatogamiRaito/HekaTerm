use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use wezterm_surface::line::simd::{hash_line, lines_equal};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("wezterm_surface_simd");
    let line1 = vec![b'x'; 256];
    let line2 = vec![b'x'; 256];

    group.throughput(Throughput::Bytes(line1.len() as u64));

    group.bench_function("lines_equal_256", |b| {
        b.iter(|| black_box(lines_equal(black_box(&line1), black_box(&line2))))
    });

    group.bench_function("hash_line_256", |b| {
        b.iter(|| black_box(hash_line(black_box(&line1))))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
