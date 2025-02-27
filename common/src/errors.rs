use ethers_core::types::H256;
use ic_cdk::api::call::RejectionCode;
use thiserror::Error;

use crate::types::BlockTag;

#[derive(Debug, Error)]
#[error("block not available: {block}")]
pub struct BlockNotFoundError {
    block: BlockTag,
}

impl BlockNotFoundError {
    pub fn new(block: BlockTag) -> Self {
        Self { block }
    }
}

#[derive(Debug, Error)]
#[error("slot not found: {slot:?}")]
pub struct SlotNotFoundError {
    slot: H256,
}

impl SlotNotFoundError {
    pub fn new(slot: H256) -> Self {
        Self { slot }
    }
}

#[derive(Debug, Error)]
#[error("rpc error on method: {method}, message: {error}")]
pub struct RpcError<E: ToString> {
    method: String,
    error: E,
}

impl<E: ToString> RpcError<E> {
    pub fn new(method: &str, err: E) -> Self {
        Self {
            method: method.to_string(),
            error: err,
        }
    }
}

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("canister call error: rejection code: {0:?}, message: {1}")]
    CanisterCall(RejectionCode, String),
}

impl From<(RejectionCode, String)> for HttpError {
    fn from(value: (RejectionCode, String)) -> Self {
        HttpError::CanisterCall(value.0, value.1)
    }
}
