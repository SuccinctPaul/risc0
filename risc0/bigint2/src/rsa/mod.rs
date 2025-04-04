// Copyright 2025 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(test)]
mod tests;

use include_bytes_aligned::include_bytes_aligned;

use crate::{ffi::sys_bigint2_3, WORD_SIZE};

pub const RSA_4096_WIDTH_WORDS: usize = 4096 / (WORD_SIZE * 8);

const BLOB: &[u8] = include_bytes_aligned!(4, "modpow_65537.blob");

type RsaArray = [u32; RSA_4096_WIDTH_WORDS];

pub fn modpow_65537(base: &RsaArray, modulus: &RsaArray, result: &mut RsaArray) {
    unsafe {
        sys_bigint2_3(
            BLOB.as_ptr(),
            base.as_ptr(),
            modulus.as_ptr(),
            result.as_mut_ptr(),
        );
    }
    // An honest host will always return a result less than the modulus.
    // A dishonest prover could return a result greater than the modulus that differs by a multiple
    // of the modulus, e.g. they could return `4` (instead of `1`) as the answer to `1^65537 % 3`,
    // since `4 - 1 = 3`.
    // Therefore, we check that we are in the honest case.
    assert!(crate::is_less(result, modulus));
}
