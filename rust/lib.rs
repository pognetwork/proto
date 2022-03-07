use std::convert::TryFrom;

use api::TransactionID;
pub use prost::Message;
pub use prost::{DecodeError, EncodeError};

#[path = "generated/api.rs"]
mod _api;
mod util;

pub mod api {
    pub use crate::_api::*;
    pub type AccountID = [u8; 24];
    pub type BlockID = [u8; 32];
    pub type TransactionID = [u8; 32];

    /// SignedBlock is the preferred way to deal with blocks. From/Into Conversions are provided for `Block`/`RawBlock` for convenience.
    #[derive(Debug, Clone, PartialEq)]
    pub struct SignedBlock {
        pub header: BlockHeader,
        pub data: BlockData,
        pub data_raw: Vec<u8>,
    }
}

fn data_err(err: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, err)
}

impl From<api::SignedBlock> for api::Block {
    fn from(block: api::SignedBlock) -> Self {
        let block_id = block.get_id().to_vec();
        api::Block {
            data: Some(block.data),
            block_id,
            header: Some(block.header),
        }
    }
}

impl From<api::SignedBlock> for api::RawBlock {
    fn from(block: api::SignedBlock) -> Self {
        api::RawBlock {
            data: block.data_raw,
            header: Some(block.header),
        }
    }
}

impl TryFrom<api::RawBlock> for api::SignedBlock {
    type Error = std::io::Error;

    fn try_from(raw_block: api::RawBlock) -> Result<Self, Self::Error> {
        if raw_block.data.is_empty() {
            return Err(data_err("block data cannot be empty"));
        }

        let data = api::BlockData::decode(&*raw_block.data)?;
        let header = raw_block
            .header
            .ok_or_else(|| data_err("missing block header"))?;

        Ok(Self {
            data_raw: raw_block.data,
            data,
            header,
        })
    }
}

impl api::SignedBlock {
    pub fn get_id(&self) -> api::BlockID {
        util::sha3(&self.data_raw)
    }

    pub fn new(header: api::BlockHeader, data: api::BlockData) -> Self {
        let data_raw = data.encode_to_vec();
        Self {
            data,
            header,
            data_raw,
        }
    }
}

impl api::Transaction {
    pub fn get_id(
        parent_block_id: api::BlockID,
        index: u32,
    ) -> Result<TransactionID, std::io::Error> {
        match parent_block_id.is_empty() {
            true => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "parent_block_id cannot be empty",
            )),
            false => Ok(util::sha3(util::concat_u8(
                &parent_block_id,
                &index.to_be_bytes(),
            ))),
        }
    }
}

#[derive(Debug, Clone)]
struct DoubleError;

#[path = "generated/p2p.rs"]
pub mod p2p;

#[path = "generated/lattice.rs"]
mod rpc_lattice;

#[path = "generated/nodewalletmanager.rs"]
mod rpc_node_wallet_manager;

#[path = "generated/nodeadmin.rs"]
mod rpc_node_admin;

#[path = "generated/nodeuser.rs"]
mod rpc_node_user;

pub mod rpc {
    pub mod lattice {
        pub use crate::rpc_lattice::*;
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
