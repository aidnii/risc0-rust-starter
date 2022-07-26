#![no_main]
#![no_std]

use methods::{BlockHeaderNoHash, BlockHeaderNoNonce};
use risc0_zkvm_guest::{env, sha};
pub use blockdata::block::BlockHeader;
use serde::{Deserialize, Serialize};
risc0_zkvm_guest::entry!(main);


// Calculating the hash by excluding the block hash and nonce

pub fn main() {
    let mut header_no_hash: BlockHeaderNoHash = env::read();

    let header_hash: &'static str = &sha::digest(&header_no_hash).to_hex();
    let header_hash = &(sha::digest(&header_no_hash).to_hex()[..]);
    //let hash_string = Digest::from_slice(&hash);

    let mut header_no_nonce = BlockHeaderNoNonce {
        version: header_no_hash.version,
        prev_hash: header_no_hash.prev_hash,
        merkle_root_hash: header_no_hash.merkle_root_hash,
        time: header_no_hash.time,
        bits: header_no_hash.bits,
        header_hash
    };

    env::commit(&header_no_nonce);
}