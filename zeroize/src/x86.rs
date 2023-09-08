//! [`Zeroize`] impls for x86 SIMD registers

use crate::{atomic_fence, volatile_write, Zeroize};

#[cfg(target_arch = "x86")]
use core::arch::x86::{__m128, __m128d, __m128i, __m256, __m256d, __m256i};

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{__m128, __m128d, __m128i, __m256, __m256d, __m256i};

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
}

impl_zeroize_for_simd_register!(__m128, __m128d, __m128i, __m256, __m256d, __m256i);
