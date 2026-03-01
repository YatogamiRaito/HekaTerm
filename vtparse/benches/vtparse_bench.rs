use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use vtparse::find_first_escape;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("vtparse_simd");

    // Benchmark a kilobyte of data without any escape characters
    let data_no_esc = vec![b'A'; 1024];
    group.throughput(Throughput::Bytes(data_no_esc.len() as u64));
    group.bench_function("find_first_escape_1k_none", |b| {
        b.iter(|| black_box(find_first_escape(black_box(&data_no_esc))))
    });

    // Benchmark a kilobyte of data with an escape character at the end
    let mut data_with_esc = vec![b'B'; 1024];
    data_with_esc[1023] = 0x1b;
    group.throughput(Throughput::Bytes(data_with_esc.len() as u64));
    group.bench_function("find_first_escape_1k_end", |b| {
        b.iter(|| black_box(find_first_escape(black_box(&data_with_esc))))
    });

    // Benchmark short text (common case for single cell updates or short strings)
    let short_data = b"hello\x1bworld!";
    group.throughput(Throughput::Bytes(short_data.len() as u64));
    group.bench_function("find_first_escape_short", |b| {
        b.iter(|| black_box(find_first_escape(black_box(short_data))))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
