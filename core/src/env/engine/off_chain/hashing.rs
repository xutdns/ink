// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

//! Implementations of supported cryptographic hash functions.

/// Conduct the BLAKE2 256-bit hash and place the result into `output`.
pub fn blake2_256(input: &[u8], output: &mut [u8; 32]) {
    output.copy_from_slice(blake2_rfc::blake2b::blake2b(32, &[], input).as_bytes());
}

/// Conduct the BLAKE2 128-bit hash and place the result into `output`.
pub fn blake2_128(input: &[u8], output: &mut [u8; 16]) {
    output.copy_from_slice(blake2_rfc::blake2b::blake2b(16, &[], input).as_bytes());
}

/// Conduct the KECCAK 256-bit hash and place the result into `output`.
pub fn keccak_256(input: &[u8], output: &mut [u8; 32]) {
    use ::tiny_keccak::{
        Hasher,
        Keccak,
    };
    let mut keccak = Keccak::v256();
    keccak.update(input);
    keccak.finalize(output)
}

/// Conduct the SHA2 256-bit hash and place the result into `output`.
pub fn sha2_256(input: &[u8], output: &mut [u8; 32]) {
    use ::sha2::{
        Digest,
        Sha256,
    };
    let mut hasher = Sha256::new();
    hasher.input(input);
    output.copy_from_slice(&hasher.result());
}
