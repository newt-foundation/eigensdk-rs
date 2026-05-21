use alloy::primitives::{Address, Bytes, FixedBytes};
use async_trait::async_trait;
use newton_types::multichain::{Certificate, OperatorTable};
use thiserror::Error;
use tracing::{info, instrument};

#[derive(Debug, Error)]
pub enum CertificateVerifierError {
    #[error("invalid certificate")]
    InvalidCertificate,
    #[error("certificate expired")]
    CertificateExpired,
    #[error("insufficient stake")]
    InsufficientStake,
    #[error("verification failed: {0}")]
    VerificationFailed(String),
    #[error("merkle proof invalid")]
    InvalidMerkleProof,
}

#[async_trait]
pub trait CertificateVerifier: Send + Sync {
    async fn verify_certificate(
        &self,
        certificate: &Certificate,
    ) -> Result<bool, CertificateVerifierError>;

    async fn verify_operator_in_table(
        &self,
        operator: Address,
        operator_table: &OperatorTable,
        merkle_proof: Vec<FixedBytes<32>>,
    ) -> Result<bool, CertificateVerifierError>;

    async fn get_verified_operator_table(
        &self,
        avs: Address,
        operator_set_id: u32,
    ) -> Result<OperatorTable, CertificateVerifierError>;
}

pub struct DefaultCertificateVerifier {
    provider: String,
}

impl DefaultCertificateVerifier {
    pub fn new(provider: String) -> Self {
        Self { provider }
    }
}

#[async_trait]
impl CertificateVerifier for DefaultCertificateVerifier {
    #[instrument(skip(self, certificate))]
    async fn verify_certificate(
        &self,
        certificate: &Certificate,
    ) -> Result<bool, CertificateVerifierError> {
        info!(
            avs = %certificate.avs,
            operator_set_id = certificate.operator_set_id,
            "verifying certificate"
        );

        Ok(true)
    }

    #[instrument(skip(self, operator_table, merkle_proof))]
    async fn verify_operator_in_table(
        &self,
        operator: Address,
        operator_table: &OperatorTable,
        merkle_proof: Vec<FixedBytes<32>>,
    ) -> Result<bool, CertificateVerifierError> {
        info!(
            operator = %operator,
            avs = %operator_table.avs,
            operator_set_id = operator_table.operator_set_id,
            "verifying operator in table"
        );

        Ok(true)
    }

    #[instrument(skip(self))]
    async fn get_verified_operator_table(
        &self,
        avs: Address,
        operator_set_id: u32,
    ) -> Result<OperatorTable, CertificateVerifierError> {
        info!(
            avs = %avs,
            operator_set_id = operator_set_id,
            "retrieving verified operator table"
        );

        Ok(OperatorTable {
            avs,
            operator_set_id,
            block_number: 0,
            operators: vec![],
            merkle_root: FixedBytes::<32>::ZERO,
        })
    }
}
