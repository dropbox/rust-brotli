# Changelog

## 9.1.1

Improves quality-5/6 and quality-11 compression throughput with `fearless_simd`, and adds finer
`hotpath` attribution for the match finders and Zopfli shortest-path work. The public API is
unchanged; the MSRV remains 1.89.0.

### SIMD tagged match finders

- Quality 5 and 6 now select Brotli's H58/H68 tagged hash-table layout deterministically from the
  input size and window. A compact byte tag rejects 16 or 32 candidates at once with
  `fearless_simd::u8x16` or `u8x32`, so positions are loaded only for candidates whose tags match.
  Large inputs with sufficient window use H68's five-byte hash; smaller inputs use H58.
- This also avoids the old small-window H40 selection. H40 was not implemented and silently fell
  back to H6 with the wrong default table dimensions, causing quality 6 to scan 256 candidates
  instead of its intended 32.
- On `testdata/random_then_unicode` at q6, Apple Silicon release builds with LTO improved from
  561.6 ms to 493.4 ms over 20 runs: **12.1% less wall time, or 1.14x throughput**. On
  `testdata/alice29.txt` with `-w15`, H58 improved from 339.7 ms to 244.9 ms: **27.9% less wall time,
  or 1.39x throughput**.

### SIMD recent-match filtering

- The q11 H10 match finder previously screened as many as 63 recent positions with two scalar byte
  comparisons per candidate. It now compares the first two bytes of 32 candidates at once with
  `fearless_simd::u8x32`, then visits matching lanes nearest-first so Brotli's candidate ordering
  and tie-breaking remain unchanged.
- Distance one retains a scalar fast path. This avoids SIMD setup when repetitive input immediately
  produces a long match, while varied input benefits from the wide rejection filter.
- On `testdata/random_then_unicode` at q11, Apple Silicon release builds with LTO improved from
  132.5 ms to 128.1 ms over 30 runs: **3.3% less wall time, or approximately 3.4% more throughput**.
  The highly repetitive corpus remained effectively unchanged, within measurement variance; q10
  is also unchanged because it does not have enough recent candidates for a 32-lane batch.

### Profiling attribution

- Added feature-gated `hotpath::measure` boundaries around the H5/H6 scalar matcher, H58/H68 tagged
  matcher and its SIMD tag filter, plus `FindAllMatchesH10Simd`,
  `StoreAndFindMatchesH10Simd`, `UpdateNodesSimd`, `ShortestPathPositionsSimd` and
  `ComputeShortestPathFromNodes`. A default build compiles all instrumentation out.
- On the same q11 input, the detailed report attributes 43.05 ms across 272,660 calls to the HQ
  match finder, including 18.88 ms in its binary-tree walk, and 83.45 ms across 545,320 calls to
  Zopfli node updates. These are inclusive instrumented totals, not end-to-end benchmark results.
- On a q6 code-shaped sample, the tagged matcher accounts for 115.60 ms across 1,962,100 calls, or
  57.53% of the instrumented run, while its SIMD tag filter accounts for 12.98 ms, or 6.46%.
  Backward-reference generation is 86.93% in total, showing that matching-candidate validation is
  now much more expensive than producing the tag mask.
- Most instrumentation remains metablock-granular. The new HQ measurements deliberately run per
  position to expose inclusive totals and call counts, so uninstrumented release builds should be
  used for throughput comparisons.

### Verification

Quality-11 output was diffed against 9.1.0 on six varied, textual and repetitive corpora and every
stream was byte-identical. Quality-6 H58 output was byte-identical on the main benchmark corpus;
H58, H68 and the corrected small-window streams pass decompression round trips. The full test suite
passes, as do default, `hotpath` and `no-default-features` builds.

## 9.1.0

Adds a scoped multi-threaded entry point, `enc::threading::CompressMultiScoped`, which compresses
a **borrowed** `&[u8]` inside a thread scope the caller supplies — `std::thread::scope`,
`rayon::in_place_scope`, or anything else that joins its tasks before returning. **Compressed
output is unchanged**: for the same input, chunk count and parameters it emits the same bytes as
`CompressMulti`, and it shares that function's chunking, hasher construction and concatenation. The
release is purely additive — no existing item changed shape — and the MSRV stays at 1.89.0.

### `CompressMultiScoped`

```rust
pub fn CompressMultiScoped<Alloc: BrotliAlloc + Send + 'static, Scope: ThreadScope>(
    params: &BrotliEncoderParams,
    input: &[u8],
    output: &mut [u8],
    alloc_per_thread: &mut [Option<Alloc>],
    thread_scope: &Scope,
) -> Result<usize, BrotliEncoderThreadError>
```

Because the scope guarantees the workers finish before it returns, most of what `CompressMulti`
carries to make the input outlive its threads is unnecessary:

- **The input is borrowed, not owned.** `CompressMulti` moves the buffer into an `Owned<SliceW>`,
  publishes it as `Arc<RwLock<..>>` and has every worker take a read lock to reach it;
  `CompressMultiSlice` additionally allocates and memcpys a private copy of the whole input. The
  scoped path hands each worker a plain `&[u8]`. The caller's buffer needs no `SliceWrapper`
  wrapper and no `Send + Sync + 'static` bound, and there is no `Arc`, no lock and no copy.
- **Results land in slots instead of being joined.** Each worker owns one disjoint
  `&mut Option<CompressionThreadResult<Alloc>>` and writes into it; nothing blocks until the scope
  closes, after which the chunks are concatenated in order with no further synchronization.
- **Allocators are `&mut [Option<Alloc>]`.** One per chunk, taken on spawn and put back when the
  results are drained, so `SendAlloc`, `Joinable` and the `BatchSpawnableLite` spawner all drop out
  of the signature. A slot still `None` on return means that worker never completed, which is
  reported as `OtherThreadPanic`.

Scheduling is deliberately identical to `CompressMulti`: chunk 0 is spawned before the shared
hasher is built so it overlaps that work, chunks 1..n-2 are spawned as their hasher clones become
ready, and the last chunk is compressed on the calling thread.

### Bringing your own scope: `ThreadScope`, `ScopeBody`, `ScopedSpawner`

Three traits (`std` only) bridge to a scoped-thread API without this crate depending on one.
`StdThreadScope` implements them over `std::thread::scope`; the `ThreadScope` docs carry
copy-pasteable rayon implementations for the calling crate, since orphan rules keep them from
living here.

- Everything is generic — no `Box<dyn FnOnce>` per task and no `dyn` spawner, so tasks are
  monomorphized and passed straight to the underlying `spawn`.
- That is why `ScopeBody` exists rather than a closure: `ScopedSpawner::spawn` is generic, so the
  spawner is not object-safe, and the body has to be generic over it. A scope implementation only
  learns its spawner type *inside* e.g. `std::thread::scope`, which quantifies over a lifetime no
  outer signature can name — a GAT cannot express it, but a rank-2 `fn run<Spawner>(self, ..)` can.
- **Both rayon entry points are supported.** `rayon::scope` requires its body to be `Send` and its
  return value to be `Send`, and an implementation of `ThreadScope::scope` cannot add `where`
  clauses of its own — so those bounds live on the trait, as `ScopeBody: Send` with
  `type Output: Send`. `rayon::in_place_scope` needs neither, and is the better default when the
  calling thread is yours to use: the body compresses the last chunk itself, so running it in place
  keeps that work on the caller instead of handing it to a pool worker while the caller blocks.
  `rayon::scope` is the right choice when the calling thread should do no encode work at all.
  Output is identical either way.
- Sound by construction rather than by contract: a task is bounded by the `'env` it borrows, so no
  safe implementation can hand it to a non-scoped thread. Leaking one is safe and merely leaves the
  slot empty, which surfaces as an error.

### Verification and measurements

Three tests were added. The scoped path's output is asserted byte-identical to `CompressMulti`
across 1–4 chunks × `favor_cpu_efficiency` on/off × two scope implementations (`StdThreadScope` and
an inline one that runs tasks on the calling thread, which also checks the traits against a
non-`std::thread` backend); a round-trip decodes back to the input; and the insufficient-output-space
path is checked to still return every allocator. The suite is now 141 tests.

Against rayon 1.12 out of tree, the two implementations from the `ThreadScope` docs were compiled
verbatim and both — `in_place_scope` and `scope` — match `StdThreadScope` byte for byte at 1–4
chunks.

Wall clock on a 3.4 MB varied corpus (English text, a mixed unicode/binary blob, a shared library
and two synthetic files), 4 chunks, `favor_cpu_efficiency`, Apple M5 Pro, rustc 1.97.1, release
profile with LTO: **no measurable difference** from either `CompressMulti` or `CompressMultiSlice`
at q5, q9 or q11. Comparing medians of 7 alternating runs, the deltas against `CompressMulti` are
−0.3%, +0.8% and −0.2%; against `CompressMultiSlice` they swing between −4.6% and +1.8% across
four repeats of the same measurement, averaging about 1% in favour of the scoped path — roughly
what skipping one 3.4 MB allocation and copy is worth. This is the expected result: the `Arc`, the
lock and the join are per chunk, not per byte. The reason to reach for this entry point is what
the caller no longer has to own or copy and the ability to run on a pool it already has, not
encode time.

## 9.0.1

Maintenance release: a dependency bump and a slice-copy cleanup, neither of which changes what the
encoder emits. **Compressed output is unchanged** — every stream is byte-identical to 9.0.0 at
every quality level, and the MSRV stays at 1.89.0.

### `fearless_simd` 0.6 → 0.7

- Raised the requirement to `~0.7`. The upgrade is source-only: 0.7 moves the arithmetic and
  bitwise operator bounds that used to sit on `SimdInt` and `SimdFloat` up onto `SimdBase`, so the
  two traits no longer need importing where the code only does arithmetic on lanes. That drops one
  or two names from the `fearless_simd` import in `bit_cost.rs`, `block_splitter.rs`,
  `prior_eval.rs`, `static_dict.rs` and `vectorization.rs`, and touches nothing else.
- No vectorized path was rewritten. 0.7's new surface was reviewed for anything this crate could
  adopt and nothing applies — `rotate_elements_left`, the most promising addition, is an alias for
  the `slide` this crate already uses.

### `clone_from_slice` → `copy_from_slice`

- Replaced 93 of the 95 `clone_from_slice` calls under `src/` with `copy_from_slice`. This is an
  intent change rather than an optimization: core specializes `clone_from_slice` through
  `CloneFromSpec` at monomorphization, so for `Copy` elements both already lowered to the same
  `memcpy`. What changes is that the call site now says so. Release `__text` shrank by 408 bytes
  (754,496 → 754,088) and encode wall clock is unchanged — at q5, q9, q10 and q11 on a 4.9 MB
  varied corpus, over 20 runs each, every delta lands inside one standard deviation.
- Two call sites keep `clone_from_slice` because their element type is only `Clone`:
  `enc::compress_fragment_two_pass::memcpy`, a public generic whose bound cannot be tightened
  without a breaking change, and the histogram-array growth in `enc::block_splitter`, where
  `HistogramLiteral` / `HistogramCommand` / `HistogramDistance` are deliberately not `Copy` — they
  carry 256- to 704-entry arrays behind hand-written `Clone` impls.

### Verification

Encoder output was diffed against 9.0.0 over 504 (corpus, quality, window) combinations — 14
`testdata` inputs × qualities 0–11 × `-w16`, `-w22` and `-w24` — with zero mismatches. The
multi-threaded (`-j2`, `-j4`) and `catbrotli` concatenation paths produce identical bytes too, and
round-trips decode across builds in both directions. The 138-test suite passes, and the default,
`no-default-features`, `hotpath`, wide-feature and `c/` FFI builds are all clean.

## 9.0.0 — first release as `simd-brotli`

Forked from [`brotli`](https://crates.io/crates/brotli) 8.0.4
(`dropbox/rust-brotli` at `9651aa3`). The encoder's hot paths are now vectorized with
[`fearless_simd`](https://crates.io/crates/fearless_simd), which runtime-dispatches to the
widest instruction set the running CPU supports, on stable Rust and without `unsafe`.

**Compressed output is unchanged.** Every optimization below is bit-identical to the scalar
code it replaces: streams produced by this crate match upstream `brotli` byte for byte at every
quality level, and remain bitwise identical to the C brotli engine at levels 0–9.

### Verification and measurements

Encoder output was diffed against the 8.0.4 fork base over 60 (corpus, quality) pairs — four
corpora (`alice29.txt`, `asyoulik.txt`, `random_org_10k.bin`, `monkey`) × qualities 0–11
including 9.5, 9.5x and 9.5y, all at `-w22` — and every pair is byte-identical. The 100-test
suite passes unchanged.

Wall clock on a 3.1 MB varied corpus (English text, a word list and three shared libraries),
Apple M5 Pro, rustc 1.97.1, release profile with LTO, best of three runs:

| quality | 8.0.4 base | this fork | change |
| ------- | ---------- | --------- | ------ |
| q9      | 0.18 s     | 0.16 s    | −11%   |
| q10     | 0.82 s     | 0.70 s    | −15%   |
| q11     | 1.93 s     | 1.77 s    | −8%    |

These are NEON numbers from one machine and one corpus; the win depends on the instruction set
the CPU offers and on how much of the input reaches the slow paths. Treat them as an indication
of scale, not a guarantee.

### Packaging

- Renamed the package to `simd-brotli`; the library target is `simd_brotli`, so this crate and
  upstream `brotli` can sit in the same dependency graph. Migration is a rename of the import
  (`use brotli::…` → `use simd_brotli::…`); the API surface is otherwise untouched.
- The `brotli` and `catbrotli` binaries keep their names.
- Added Mikhail Panfilov to the author list, alongside the upstream authors. Licensing is
  unchanged: BSD-3-Clause AND MIT, with both upstream license files shipped.

### SIMD backend: `packed_simd`/`portable_simd` → `fearless_simd`

- **Removed the `simd` cargo feature and the nightly requirement it carried.** Vectorization is
  now unconditional and builds on stable: the `#![feature(portable_simd)]` gate is gone, as is
  the `core::simd` dependency behind it.
- **Removed `src/enc/compat.rs`** (392 lines of hand-written scalar fallbacks for
  `Compat16x16` / `CompatF8` / `Compat32x8`). There is no longer a scalar shim to keep in sync
  with a vector path — one implementation serves both.
- `s16`, `v8` and `s8` are now `Mem16x16`, `Mem256f` and `Mem256i` from
  `enc::vectorization`. These are plain `Copy + Default` arrays so they can live in the
  encoder's allocator-backed slices; arithmetic happens on `fearless_simd` registers loaded via
  `to_simd` / stored via `from_simd`.
- **Runtime dispatch.** `enc::vectorization::detect_level()` picks the instruction set (AVX2,
  NEON, …) on `std` and wasm builds, caching the answer in a `OnceLock` — the probe is a dozen
  feature tests, and clustering calls into it once per histogram. `no_std` builds fall back to
  the level the crate was compiled for, so the `no-stdlib` configuration still works.
- Because detection is not free, the `dispatch!` regions are hoisted to the outermost loop that
  can own them: a whole Zopfli block, a whole hasher store range, a whole match-finder walk.
  Inner functions take an already-detected `S: Simd` instead of probing per call.
- Bumped MSRV to **1.89.0** and the edition to **2024**, both required by `fearless_simd`.
- `no_std` support is preserved through `fearless_simd`'s `libm` feature; the crate's `std`
  feature forwards to `fearless_simd/std`.

### Newly vectorized encoder paths

Upstream vectorized two cost loops. This fork extends wide-lane work to the stages that
profiling showed dominate compression time:

- **H10 binary-tree match finder** (`backward_references/hash_to_binary_tree.rs`) —
  `StoreAndFindMatchesH10`, `Store`, `StoreRange` and `BulkStoreRange` gained SIMD variants.
  The tree walk measures a match length per node, up to 64 per position, so detection is
  hoisted to the range walk rather than paid per comparison.
- **Zopfli shortest path** (`backward_references/hq.rs`) — `UpdateNodes`,
  `StitchToPreviousBlockH10`, `FindAllMatchesH10`, the per-position sweep of
  `BrotliZopfliComputeShortestPath` and `ZopfliIterate` all run on a per-block detected
  instruction set, so one probe covers the match finder, the node update and the hasher store
  for the whole block.
- **Static-dictionary match length** (`static_dict.rs`) — `FindMatchLengthWithLimit` now
  resolves short matches (the overwhelming majority) with a narrow scalar probe and never looks
  at the CPU feature set; only once a match passes the wide-compare threshold does it detect an
  instruction set and switch to 32-byte compares. `FindMatchLengthWithLimitSimd` is available
  for callers that already hold one.
- **Block splitter** (`block_splitter.rs`) — `FindBlocks`' per-histogram cost scan keeps a
  running per-lane winner and reduces across lanes once per row instead of branching per
  histogram. Ties resolve to the lowest histogram id, exactly as the scalar scan did.
- **Population cost** (`bit_cost.rs`) — the symbol-cost walk tests eight buckets per compare
  and only falls back to per-bucket work for populated ones. Histograms are mostly empty (a
  distance alphabet has 544 buckets and a metablock rarely touches a tenth of them), so the
  empty runs are what the wide compare is for. Empty runs still land in `depth_histo` exactly
  as a bucket-at-a-time scan would leave them.
- **Prior evaluation** (`prior_eval.rs`) — the per-literal cost update takes a level detected
  once when the evaluator is built.

### Hot-path optimizations

- **Histogram clustering no longer materializes histogram sums.** `BrotliPopulationCostOfSum`
  costs the histogram that adding `b` into `a` *would* produce, walking the two bucket arrays
  lane-wise instead of cloning one and running a separate add pass over it. The float
  accumulation sees the same values in the same order, so the result is bit-identical. Applied
  in `BrotliHistogramBitCostDistance` and in the clustering pair queue
  (`BrotliCompareAndPushToQueue`), which is where nearly all population-cost calls come from —
  and on large varied input, histogram clustering is roughly half of a quality-10 compression.
- The population-cost walk was refactored behind a `Buckets` trait (`Own` / `Sum`) so the
  single-histogram and summed-histogram forms share one implementation and cannot drift apart.

### Profiling support

- Added an optional [`hotpath`](https://crates.io/crates/hotpath) instrumentation layer over
  the encoder pipeline, behind three features: `hotpath` (wall clock), `hotpath-cpu` (CPU
  time) and `hotpath-alloc` (allocation counts and bytes). Every call site is a `cfg_attr`, so
  a default build neither links `hotpath` nor pays any runtime cost.
- Instrumented stages: `encode_data`, `copy_input_to_ring_buffer`, `WriteMetaBlockInternal`,
  `ChooseContextMap`, `DecideOverLiteralContextModeling`, `compress_stream_fast`, the three
  `store_meta_block*` writers, `LogMetaBlock`, `BrotliCreateBackwardReferences` and the Zopfli
  entry points, `BrotliBuildMetaBlock` / `Greedy` / `BrotliOptimizeHistograms`,
  `BrotliSplitBlock` and its internals, the `cluster.rs` histogram-clustering functions,
  `BrotliEstimateBitCostsForLiterals`, and the two `compress_fragment` fast paths.
- Instrumentation sits at metablock granularity, not per byte, so overhead stays under
  measurement noise (q9/q10/q11 on a 3.9 MB corpus timed within 1% of an uninstrumented build).
- See [Profiling the encoder](README.md#profiling-the-encoder) for usage.

### Modernization

- Migrated the crate to **edition 2024**, including the FFI layer (`src/ffi/`), the threading
  and worker-pool modules, and the binaries. `unsafe` blocks inside `unsafe fn` are now
  explicit, as edition 2024 requires.
- Removed the now-dead `extern crate` / SIMD `use` statements the old feature gate needed.

### Upstream

For changes inherited from `brotli` 8.0.4 and earlier, see the version history in
[README.md](README.md#whats-new-in-804).
