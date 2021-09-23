use api::TransactionID;
use prost::Message;
use sha3::Digest;
use sha3::Sha3_256;

#[path = "generated/api.rs"]
mod _api;

pub mod api {
    pub use crate::_api::*;
    pub type AccountID = [u8; 24];
    pub type BlockID = [u8; 32];
    pub type TransactionID = [u8; 32];
}

#[path = "generated/rpc.rs"]
pub mod rpc;

fn sha3(data: impl AsRef<[u8]>) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize().into()
}

impl api::Block {
    pub fn get_id(&self) -> api::BlockID {
        sha3(&self.encode_to_vec())
    }
}

impl api::Transaction {
    pub fn get_id(&self, parent_block_id: api::BlockID) -> TransactionID {
        let transaction_hash = sha3(&self.encode_to_vec());
        sha3([parent_block_id, transaction_hash].concat())
    }
}
