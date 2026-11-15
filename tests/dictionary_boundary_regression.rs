extern crate brotli;

#[path = "support/dictionary_boundary_harness.rs"]
mod harness;

#[test]
fn issue_280_h5_qualities_5_and_6_catable() {
    let (dictionary, input) = harness::corpus(50, 16, 4096, 0);
    for quality in 5..=6 {
        harness::check_dict(&dictionary, &input, quality, true)
            .unwrap_or_else(|e| panic!("q={} issue#280 catable: {}", quality, e));
    }
}

#[test]
fn issue_280_higher_qualities_7_through_9() {
    let (dictionary, input) = harness::corpus(50, 16, 4096, 0);
    for quality in 7..=9 {
        harness::check_dict(&dictionary, &input, quality, false)
            .unwrap_or_else(|e| panic!("q={} issue#280: {}", quality, e));
    }
}

#[test]
fn boundary_dictionary_final_byte_matches_input() {
    for period in [1usize, 2, 3, 5, 17].iter().cloned() {
        let (dictionary, input) = harness::boundary_run(16, 4096, period);
        for quality in 2..=11 {
            harness::check_dict(&dictionary, &input, quality, true)
                .unwrap_or_else(|e| panic!("q={} period={}: {}", quality, period, e));
        }
    }
}

#[test]
fn advanced_bucket_and_cached_distance_regressions() {
    // q5-q8 are AdvHasher configurations; include both cache and bucket
    // search opportunities with a deterministic low-entropy corpus.
    for seed in 1..=64 {
        let (dictionary, input) = harness::corpus(seed, 16, 4096, 0);
        for quality in 5..=8 {
            harness::check_dict(&dictionary, &input, quality, true)
                .unwrap_or_else(|e| panic!("q={} seed={}: {}", quality, seed, e));
        }
    }
}

#[test]
fn basic_hasher_boundary_regressions() {
    // q2 and q3 use the BasicHasher. Unlike the issue#280 short fixture,
    // varied structured input can exercise their dictionary-boundary paths.
    for seed in 1..=48 {
        let (dictionary, input) = harness::corpus(seed, 31, 8192, (seed % 6) as u8);
        for quality in 2..=4 {
            harness::check_dict(&dictionary, &input, quality, true)
                .unwrap_or_else(|e| panic!("q={} seed={}: {}", quality, seed, e));
        }
    }
}

#[test]
fn h9_boundary_regressions() {
    for seed in 1..=64 {
        let (dictionary, input) = harness::corpus(seed, 16, 4096, 0);
        harness::check_dict(&dictionary, &input, 9, true)
            .unwrap_or_else(|e| panic!("q9 seed={}: {}", seed, e));
    }
}

#[test]
fn h10_short_distance_cross_boundary_regressions() {
    // Exercises the independent issue#278 path, which previously did not
    // clamp its short-distance candidate at all.
    for dict_len in [8usize, 16, 31, 127].iter().cloned() {
        for period in [1usize, 2, 3].iter().cloned() {
            let (dictionary, input) = harness::boundary_run(dict_len, 4096, period);
            for quality in 10..=11 {
                harness::check_dict(&dictionary, &input, quality, false)
                    .unwrap_or_else(|e| panic!("q={} dict={} period={}: {}", quality, dict_len, period, e));
            }
        }
    }
}

#[test]
fn waterfall_q5_catable_two_threads_four_mib() {
    // Dropbox blockstore defaults: quality 5, catable, appendable, two
    // compression threads; second thread uses first half as its dictionary.
    let size = 4 * 1024 * 1024;
    harness::check_multi_pattern(50, size, 0, 5, 2)
        .unwrap_or_else(|e| panic!("Waterfall 4MiB two-thread: {}", e));
}

#[test]
fn multithreaded_one_two_four_thread_roundtrips() {
    for threads in [1usize, 2, 4].iter().cloned() {
        for quality in [2i32, 5, 9, 11].iter().cloned() {
            harness::check_multi_pattern(17, 1 << 16, 2, quality, threads)
                .unwrap_or_else(|e| panic!("threads={} q={}: {}", threads, quality, e));
        }
    }
}
