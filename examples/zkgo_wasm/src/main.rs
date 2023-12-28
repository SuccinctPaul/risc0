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


use clap::{Arg, Command};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::fs;
use zkgo_wasm_methods::{ZKGO_WASM_INTERP_ELF, ZKGO_WASM_INTERP_ID};

use ark_std::{start_timer, end_timer};

// fn wat2wasm(wat: &str) -> Result<Vec<u8>, wat::Error> {
//     wat::parse_str(wat)
// }

fn run_guest(iters: i32) -> i32 {
    let matches = Command::new("myprog")
        .arg(Arg::with_name("wasm").short('w').long("wasm").help("wasm_file").required(false).takes_value(true))
        .get_matches();

    let wasm_file = matches.get_one::<String>("wasm");

    let wasm = {
        let wasm_file = wasm_file.unwrap();
        let wasm_binary = fs::read(wasm_file).unwrap();
        println!("load file from {}", wasm_file);
        wasm_binary
    };

    let env = ExecutorEnv::builder()
        .write(&wasm)
        .unwrap()
        .write(&iters)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let start = start_timer!(||"prove_elf");
    let receipt = prover.prove_elf(env, ZKGO_WASM_INTERP_ELF).unwrap();
    end_timer!(start);

    receipt.verify(ZKGO_WASM_INTERP_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );
    let result: i32 = receipt.journal.decode().unwrap();

    result
}

fn main() {
    let fib_iters: i32 = 100;
    let start = start_timer!(||"fibonacci_zkgo_wasm");
    let _ = run_guest(fib_iters);
    end_timer!(start);
}

#[cfg(test)]
mod tests {
    fn fibonacci(n: i32) -> i32 {
        let (mut a, mut b) = (0, 1);
        for _ in 0..n {
            let c = a;
            a = b;
            b += c;
        }
        a
    }

    #[test]
    fn wasm_fib() {
        let fib_iters: i32 = 10;
        let result = super::run_guest(fib_iters);
        assert_eq!(
            result,
            fibonacci(fib_iters),
            "We expect the zkVM output to be the product of the inputs"
        )
    }
}
