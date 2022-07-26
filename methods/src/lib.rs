//include!(concat!(env!("OUT_DIR"), "/methods.rs"));

// Struct for the Block Header without hash
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]     

pub struct BlockHeaderNoHash {
    pub version: i32,
    pub prev_hash: &'static str,
    pub merkle_root_hash: &'static str,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32
}

// Struct for the Block Header without NONCE
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]      

pub struct BlockHeaderNoNonce {
    pub version: i32,
    pub prev_hash: &'static str,
    pub merkle_root_hash: &'static str,
    pub time: u32,
    pub bits: u32,
    pub hash_string: &'static str,
}