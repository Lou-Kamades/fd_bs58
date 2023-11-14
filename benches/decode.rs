use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_decode_32(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_32");
    let string = "2gPihUTjt3FJqf1VpidgrY5cZ6PuyMccGVwQHRfjMPZG";

    group.bench_with_input("decode_bs58", string, |b, str| {
        b.iter(|| bs58::decode(black_box(str)).into_vec())
    });
    group.bench_with_input("decode_bs58_noalloc", string, |b, str| {
        let mut output = [0; 32];
        b.iter(|| bs58::decode(black_box(str)).into(&mut output).unwrap());
    });
    group.bench_with_input("decode_fd", string, |b, str| {
        b.iter(|| fd_bs58::decode_32(black_box(str)))
    });
    group.finish();
}

fn bench_decode_64(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_64");
    let string =
        "11cgTH4D5e8S3snD444WbbGrkepjTvWMj2jkmCGJtgn3H7qrPb1BnwapxpbGdRtHQh9t9Wbn9t6ZDGHzWpL4df";

    group.bench_with_input("decode_bs58", string, |b, str| {
        b.iter(|| bs58::decode(black_box(str)).into_vec())
    });
    group.bench_with_input("decode_bs58_noalloc", string, |b, str| {
        let mut output = [0; 64];
        b.iter(|| bs58::decode(black_box(str)).into(&mut output).unwrap());
    });
    group.bench_with_input("decode_fd", string, |b, str| {
        b.iter(|| fd_bs58::decode_64(black_box(str)))
    });
    group.finish();
}

criterion_group!(benches, bench_decode_32, bench_decode_64);
criterion_main!(benches);
