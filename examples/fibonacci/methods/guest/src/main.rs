// Copyright 2023 RISC Zero, Inc.
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

#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // Load the first number from the host
    let iter: u64 = env::read();

    let mut a = 0;
    let mut b = 1;

    let mut res = 0;
    for _ in 0..iter {
        res = a.add(b);
        b = res;
        a = b;
    }

    // Compute the product while being careful with integer overflow
    let product = res;
    env::commit(&product);
}
