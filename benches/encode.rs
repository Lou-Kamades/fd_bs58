use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_encode_32(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_32");
    let bytes = &[
        24, 243, 6, 223, 230, 153, 210, 8, 92, 137, 123, 67, 164, 197, 79, 196, 125, 43, 183, 85,
        103, 91, 232, 167, 73, 131, 104, 131, 0, 101, 214, 231,
    ];
    let string = "2gPihUTjt3FJqf1VpidgrY5cZ6PuyMccGVwQHRfjMPZG";

    group.bench_with_input("encode_bs58", bytes, |b, byt| {
        b.iter(|| bs58::encode(black_box(byt)).into_string())
    });
    group.bench_with_input("encode_bs58_noalloc", bytes, |b, byt| {
        let mut output = String::with_capacity(string.len());
        b.iter(|| bs58::encode(black_box(byt)).into(&mut output));
    });
    group.bench_with_input("encode_fd", bytes, |b, byt| {
        b.iter(|| fd_bs58::encode_32(black_box(*byt)))
    });
    group.finish();
}

fn bench_encode_64(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode_64");
    let bytes = &[
        0, 0, 10, 85, 198, 191, 71, 18, 5, 54, 6, 255, 181, 32, 227, 150, 208, 3, 157, 135, 222,
        67, 50, 23, 237, 51, 240, 123, 34, 148, 111, 84, 98, 162, 236, 133, 31, 93, 185, 142, 108,
        41, 191, 1, 138, 6, 192, 0, 46, 93, 25, 65, 243, 223, 225, 225, 85, 55, 82, 251, 109, 132,
        165, 2,
    ];
    let string =
        "11cgTH4D5e8S3snD444WbbGrkepjTvWMj2jkmCGJtgn3H7qrPb1BnwapxpbGdRtHQh9t9Wbn9t6ZDGHzWpL4df";

    group.bench_with_input("encode_bs58", bytes, |b, byt| {
        b.iter(|| bs58::encode(black_box(byt)).into_string())
    });
    group.bench_with_input("encode_bs58_noalloc", bytes, |b, byt| {
        let mut output = String::with_capacity(string.len());
        b.iter(|| bs58::encode(black_box(byt)).into(&mut output));
    });
    group.bench_with_input("encode_fd", bytes, |b, byt| {
        b.iter(|| fd_bs58::encode_64(black_box(*byt)))
    });
    group.finish();
}

criterion_group!(benches, bench_encode_32, bench_encode_64);
criterion_main!(benches);
