use api::TransactionID;
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

#[derive(Debug, Clone)]
struct DoubleError;

#[path = "generated/p2p.rs"]
pub mod p2p;

#[path = "generated/block.rs"]
mod rpc_block;

#[path = "generated/nodewalletmanager.rs"]
mod rpc_node_wallet_manager;

#[path = "generated/nodeadmin.rs"]
mod rpc_node_admin;

#[path = "generated/nodeuser.rs"]
mod rpc_node_user;

pub mod rpc {
    pub mod block {
        pub use crate::rpc_block::*;
    }

    pub mod node_admin {
        pub use crate::rpc_node_admin::*;
    }

    pub mod node_wallet_manager {
        pub use crate::rpc_node_wallet_manager::*;
    }

    pub mod node_user {
        pub use crate::rpc_node_user::*;
    }
}

fn sha3(data: impl AsRef<[u8]>) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize().into()
}

impl api::SignedBlock {
    pub fn gen_id(block_data: &[u8]) -> Result<api::BlockID, std::io::Error> {
        Ok(sha3(block_data))
    }
}

pub fn concat_u8(first: &[u8], second: &[u8]) -> Vec<u8> {
    [first, second].concat()
}

impl api::Transaction {
    pub fn gen_id(
        parent_block_id: api::BlockID,
        index: u32,
    ) -> Result<TransactionID, std::io::Error> {
        Ok(sha3(concat_u8(&parent_block_id, &index.to_be_bytes())))
    }
}
