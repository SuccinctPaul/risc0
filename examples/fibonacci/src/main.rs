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

#[doc = include_str!("../README.md")]
use fibonacci_methods::FIB_ELF;
use fibonacci_methods::FIB_ID;

use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use ark_std::{start_timer, end_timer};

// This is a Hello World demo for the RISC Zero zkVM.
// By running the demo, Alice can produce a receipt that proves that she knows
// some numbers a and b, such that a*b == 391.
// The factors a and b are kept secret.

// Compute the product a*b inside the zkVM
pub fn execute(iter: u64) -> (Receipt, u64) {
    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        .write(&iter)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let start = start_timer!(||"prove_elf");
    let receipt = prover.prove_elf(env, FIB_ELF).unwrap();
    end_timer!(start);
    // Extract journal of receipt (i.e. output c, where c = a * b)
    let c: u64 = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    // Report the product
    println!("I know the factors of {}, and I can prove it!", c);

    (receipt, c)
}

fn main() {
    let start = start_timer!(||"fibonacci_riscv");

    // Pick two numbers
    let (receipt, _) = execute(100);

    // Here is where one would send 'receipt' over the network...

    // Verify receipt, panic if it's wrong
    receipt.verify(FIB_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
    end_timer!(start);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expect_fib(iter: u64) -> u64 {
        let mut a = 0;
        let mut b = 1;

        let mut res = 0;
        for _ in 0..iter {
            res = a + b;
            a = b;
            b = res;
        }
        res
    }

    #[test]
    fn test_fib() {
        const N: u64 = 100;
        let (_, actual) = execute(N);
        assert_eq!(
            actual,
            expect_fib(N),
            "We expect the zkVM output to be the fib of the inputs"
        )
    }

    #[test]
    fn test_expect_fib() {
        const N: u64 = 9;
        let result = expect_fib(N);
        println!("{:?}",result);
        assert_eq!(
            result, 55,
            "We expect the zkVM output to be the product of the inputs"
        )
    }
}
