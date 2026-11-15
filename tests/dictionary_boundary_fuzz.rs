extern crate brotli;

#[path = "support/dictionary_boundary_harness.rs"]
mod harness;

// Reproducible seed sweep for CI; scale locally with:
// BROTLI_BOUNDARY_FUZZ_CASES=2000 cargo test --release --test dictionary_boundary_fuzz -- --nocapture
#[test]
fn seeded_boundary_fuzz() {
    let n = std::env::var("BROTLI_BOUNDARY_FUZZ_CASES")
        .ok().and_then(|s| s.parse::<u64>().ok()).unwrap_or(24);
    for seed in 0..n {
        let dict_len = [2usize, 3, 4, 8, 16, 31, 63, 127, 1024][(seed % 9) as usize];
        let data_len = [128usize, 256, 4096, 8192][((seed / 9) % 4) as usize];
        let quality = (seed % 12) as u32;
        let catable = seed % 2 == 0;
        let shape = ((seed / 3) % 6) as u8;
        let (dictionary, input) = harness::corpus(seed + 1, dict_len, data_len, shape);
        harness::check_dict(&dictionary, &input, quality, catable).unwrap_or_else(|e| {
            panic!("seed={} dict={} len={} shape={} q={} catable={}: {}",
                seed, dict_len, data_len, shape, quality, catable, e)
        });
    }
}

#[test]
fn seeded_multithread_fuzz() {
    let n = std::env::var("BROTLI_BOUNDARY_MULTI_FUZZ_CASES")
        .ok().and_then(|s| s.parse::<u64>().ok()).unwrap_or(4);
    for seed in 0..n {
        let quality = [2i32, 5, 6, 9, 11][(seed % 5) as usize];
        let threads = [1usize, 2, 4][((seed / 5) % 3) as usize];
        let length = 65536 + ((seed as usize % 4) * 8192);
        harness::check_multi_pattern(seed + 50, length, (seed % 6) as u8, quality, threads)
            .unwrap_or_else(|e| panic!("multi seed={} threads={} q={}: {}", seed, threads, quality, e));
    }
}
