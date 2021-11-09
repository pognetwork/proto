use api::TransactionID;
use bytes::{BufMut, Bytes, BytesMut};
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
    pub fn serialize_for_id(&self) -> Result<Vec<u8>, std::io::Error> {
        let data = &self.data.clone().ok_or(std::io::ErrorKind::InvalidInput)?;
        let mut res = BytesMut::new();

        if data.version != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "unsupported block version",
            ));
        }

        // block version v0 orders the block by protobuf index
        res.put_slice(&self.public_key);
        res.put_u8(data.version as u8);
        res.put_u8(data.signature_type as u8);
        res.put_u64(data.balance);
        res.put_u64(data.height);
        res.put_slice(data.previous());

        data.transactions
            .iter()
            .filter_map(|tx| tx.serialize_for_id().ok())
            .for_each(|tx| res.put(tx));

        Ok(res.to_vec())
    }

    pub fn get_id(&self) -> Result<api::BlockID, std::io::Error> {
        Ok(sha3(self.serialize_for_id()?))
    }
}

impl api::Transaction {
    pub fn serialize_for_id(&self) -> Result<Bytes, std::io::Error> {
        let data = self.data.clone().ok_or(std::io::ErrorKind::InvalidInput)?;
        let mut res = BytesMut::new();

        match data {
            api::transaction::Data::TxCollect(data) => res.put_slice(&data.transaction_id),
            api::transaction::Data::TxDelegate(data) => res.put_slice(&data.representative),
            api::transaction::Data::TxOpen(data) => res.put_u8(data.r#type as u8),
            api::transaction::Data::TxSend(data) => {
                res.put_u64(data.amount);
                res.put_slice(&data.data);
                res.put_slice(&data.receiver);
            }
        }

        Ok(res.into())
    }

    pub fn get_id(&self, parent_block_id: api::BlockID) -> Result<TransactionID, std::io::Error> {
        let transaction_hash = sha3(self.serialize_for_id()?);
        Ok(sha3([parent_block_id, transaction_hash].concat()))
    }
}
