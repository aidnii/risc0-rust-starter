
use methods::{MULTIPLY_ID, MULTIPLY_PATH, BlockHeaderNoHash, BlockHeaderNoNonce};
use risc0_zkvm_host::Prover;
use risc0_zkvm_serde::{from_slice, to_vec};
pub use blockdata::block::BlockHeader;

fn main() {
    
    // 1 - Lets list the variables of the BTC Block Header components
    let version: i32 = 684531712; // hexadecimal 28cd2000
    let prev_hash: &str = "00000000000000000002307b0ef0777962274de90b7e6c748d659e98402c2548";
    let merkle_root_hash: &str = "e2cd5b57a2ade6eea2c2fc31c9b92c30c4c82203234835d88342f9e503098c53";
    let time: u32 = 1658155428;
    let bits: u32 = 386508719; // hexadecimal 1709a7af
    let nonce: u32 = 746540630; // hexadecimal 2c7f4e56
    
    let header_no_hash = BlockHeaderNoHash(
        version,
        prev_hash,
        merkle_root_hash,
        time,
        bits,
        nonce
    );

    let header_no_nonce = BlockHeaderNoNonce(
        version,
        prev_hash,
        merkle_root_hash,
        time,
        bits,
        header_hash
    );

    // 2 - Make the prover load the '' method
    let mut prover = Prover::new(&std::fs::read(MULTIPLY_PATH).unwrap(), MULTIPLY_ID).unwrap();

    // 3 - Send the components to the guest, which is compiled for riscv32i
    prover.add_input(to_vec(&header_no_nonce).unwrap().as_slice()).unwrap();

    // 4  - Run the prover & generate a receipt
    let receipt = prover.run().unwrap();

    // 5 - Extract journal of receipt
    let valid_block_hash: BlockHeaderNoHash = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();

    // 6 - Print an assertion
    println!("The valid Bitcoin block header hash is {:?}, and I know the nonce to prove it!", valid_block_hash);

    // 7 - Send the receipt over to the mining pool network (skipped)

    // 8 - Verify the receipt!!
    receipt.verify(MULTIPLY_ID).unwrap();
    return valid_block_hash;
     
}
