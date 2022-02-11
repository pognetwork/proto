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

impl api::signed_block::BlockData {
    pub fn unique_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut res = BytesMut::new();

        res.put_u8(self.version as u8);
        res.put_u8(self.signature_type as u8);
        res.put_u64(self.balance);
        res.put_u64(self.height);
        res.put_slice(&self.previous);

        self.transactions
            .iter()
            .filter_map(|tx| tx.unique_bytes().ok())
            .for_each(|tx| res.put(tx));

        Ok(res.to_vec())
    }
}

impl api::SignedBlock {
    pub fn unique_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let data = &self.data.clone().ok_or(std::io::ErrorKind::InvalidInput)?;
        let mut res = BytesMut::new();

        if data.version != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "unsupported block version",
            ));
        }

        res.put_slice(&data.unique_bytes()?);

        // block version v0 orders the block by protobuf index
        res.put_slice(&self.public_key);

        Ok(res.to_vec())
    }

    pub fn get_id(&self) -> Result<api::BlockID, std::io::Error> {
        Ok(sha3(self.unique_bytes()?))
    }
}

impl api::Transaction {
    pub fn unique_bytes(&self) -> Result<Bytes, std::io::Error> {
        let data = self.data.clone().ok_or(std::io::ErrorKind::InvalidInput)?;
        let mut res = BytesMut::new();

        match data {
            api::transaction::Data::TxClaim(data) => res.put_slice(&data.send_transaction_id),
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
        let transaction_hash = sha3(self.unique_bytes()?);
        Ok(sha3([parent_block_id, transaction_hash].concat()))
    }
}
