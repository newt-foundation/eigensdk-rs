use alloy::transports::{RpcError, TransportErrorKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MultichainError {
    #[error("alloy contract error")]
    AlloyContractError,
    #[error("invalid key: {0}")]
    InvalidKey(String),
    #[error("key registration failed: {0}")]
    KeyRegistrationFailed(String),
    #[error("certificate verification failed: {0}")]
    CertificateVerificationFailed(String),
    #[error("operator table update failed: {0}")]
    OperatorTableUpdateFailed(String),
    #[error("cross chain registry error: {0}")]
    CrossChainRegistryError(String),
    #[error("invalid signature: {0}")]
    InvalidSignature(String),
    #[error("invalid private key")]
    InvalidPrivateKey,
}

impl From<RpcError<TransportErrorKind>> for MultichainError {
    fn from(_: RpcError<TransportErrorKind>) -> Self {
        MultichainError::AlloyContractError
    }
}

impl From<alloy::contract::Error> for MultichainError {
    fn from(_: alloy::contract::Error) -> Self {
        MultichainError::AlloyContractError
    }
}
