//! Fixed-width vector storage for the encoder.
//!
//! The types here are plain arrays: `Default` + `Copy`, so they can live in the
//! encoder's allocator-backed slices and be handed to `Allocator<T>`. They carry no
//! arithmetic of their own. Math is done on [`fearless_simd`] vectors instead: inside a
//! `dispatch!` region, [`Mem256f::to_simd`] (and friends) loads a register and
//! [`Mem256f::from_simd`] stores it back.

use core::ops::{Index, IndexMut};
use core::slice::SliceIndex;

use fearless_simd::{Level, Simd, SimdBase, SimdInto, f32x8, i16x16, i32x8, u32x8};

/// The instruction set the vectorized encoder paths run on.
///
/// Detected at runtime where the platform allows it (`std` builds, wasm), otherwise the
/// best level this crate was compiled for. The `std` answer is cached: probing costs a
/// dozen feature tests, and callers such as [`crate::enc::bit_cost::BrotliPopulationCost`]
/// dispatch once per histogram, deep inside the clustering loops.
#[cfg(feature = "std")]
#[inline]
pub fn detect_level() -> Level {
    static LEVEL: std::sync::LazyLock<Level> = std::sync::LazyLock::new(|| Level::try_detect().unwrap_or_else(Level::baseline));
    *LEVEL
}

/// See the `std` variant above; without `std` there is nothing to cache, as
/// `try_detect` cannot probe the CPU and always resolves to the compiled-for level.
#[cfg(not(feature = "std"))]
#[inline]
pub fn detect_level() -> Level {
    Level::try_detect().unwrap_or_else(Level::baseline)
}

/// The smallest lane of `v`, folded in `log2(8)` steps.
#[inline(always)]
pub fn min_lane_f32x8<S: Simd>(v: f32x8<S>) -> f32 {
    let v = v.min(v.slide::<4>(v));
    let v = v.min(v.slide::<2>(v));
    let v = v.min(v.slide::<1>(v));
    v[0]
}

/// The smallest lane of `v`, folded in `log2(8)` steps.
#[inline(always)]
pub fn min_lane_u32x8<S: Simd>(v: u32x8<S>) -> u32 {
    let v = v.min(v.slide::<4>(v));
    let v = v.min(v.slide::<2>(v));
    let v = v.min(v.slide::<1>(v));
    v[0]
}

macro_rules! define_vector {
    ($(#[$attr:meta])* $name:ident, $elem:ty, $lanes:literal, $simd:ident) => {
        $(#[$attr])*
        #[derive(Default, Copy, Clone, Debug)]
        pub struct $name([$elem; $lanes]);

        impl $name {
            /// Load the lanes into a SIMD register.
            #[inline(always)]
            pub fn to_simd<S: Simd>(self, simd: S) -> $simd<S> {
                self.0.simd_into(simd)
            }

            /// Store a SIMD register back into plain memory.
            #[inline(always)]
            pub fn from_simd<S: Simd>(value: $simd<S>) -> Self {
                Self(value.into())
            }
        }

        impl From<[$elem; $lanes]> for $name {
            #[inline(always)]
            fn from(value: [$elem; $lanes]) -> Self {
                Self(value)
            }
        }

        impl<I: SliceIndex<[$elem]>> Index<I> for $name {
            type Output = I::Output;

            #[inline(always)]
            fn index(&self, index: I) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<I: SliceIndex<[$elem]>> IndexMut<I> for $name {
            #[inline(always)]
            fn index_mut(&mut self, index: I) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
    };
}

define_vector!(Mem256f, f32, 8, f32x8);
define_vector!(Mem256i, i32, 8, i32x8);
define_vector!(Mem16x16, i16, 16, i16x16);
define_vector!(
    /// A 16-bucket probability distribution.
    ///
    /// Same shape as [`Mem16x16`], but deliberately a separate type: `BrotliAlloc`
    /// requires `Allocator<PDF>` and `Allocator<s16>` as distinct bounds, so the two
    /// cannot be aliases of each other. Re-exported as [`crate::enc::pdf::PDF`].
    PDF,
    i16,
    16,
    i16x16
);

pub type v256 = Mem256f;
pub type v256i = Mem256i;
