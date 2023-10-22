//! [`Zeroize`] impls for x86 SIMD registers
//!
//! `AVX512` registers may be zeroized with the `avx512` feature (MSRV is 1.73).

use crate::{atomic_fence, volatile_write, Zeroize};

#[cfg(target_arch = "x86")]
#[allow(clippy::wildcard_imports)]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
#[allow(clippy::wildcard_imports)]
use core::arch::x86_64::*;

macro_rules! impl_zeroize_for_simd_register {
    ($($type:ty),* $(,)?) => {
        $(
            #[cfg_attr(docsrs, doc(cfg(any(target_arch = "x86", target_arch = "x86_64"))))]
            impl Zeroize for $type {
                #[inline]
                fn zeroize(&mut self) {
                    volatile_write(self, unsafe { core::mem::zeroed() });
                    atomic_fence();
                }
            }
        )*
    };

    ($feature:expr, $($type:ty),*) => {
        $(
            #[cfg_attr(docsrs, doc(cfg(all(any(target_arch = "x86", target_arch = "x86_64"), feature = $feature))))]
            impl Zeroize for $type {
                #[inline]
                fn zeroize(&mut self) {
                    volatile_write(self, unsafe { core::mem::zeroed() });
                    atomic_fence();
                }
            }
        )*
    };
}

impl_zeroize_for_simd_register!(__m128, __m128d, __m128i, __m256, __m256d, __m256i);

#[cfg(feature = "avx512")]
#[cfg(target_feature = "avx512")]
impl_zeroize_for_simd_register!("avx512", __m512, __m512d, __m512i);
