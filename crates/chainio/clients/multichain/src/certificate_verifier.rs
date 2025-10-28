use crate::error::MultichainError;
use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use alloy::providers::Provider;
use async_trait::async_trait;
use eigen_common::get_provider;
use eigen_utils::slashing::multichain::bn254_certificate_verifier::BN254CertificateVerifier;
use eigen_utils::slashing::multichain::cross_chain_registry::CrossChainRegistry::OperatorSet as ContractOperatorSet;
use eigen_utils::slashing::multichain::ecdsa_certificate_verifier::ECDSACertificateVerifier;
use tracing::instrument;

#[derive(Debug, Clone)]
pub enum CertificateVerifierReader {
    BN254(BN254CertificateVerifierReader),
    ECDSA(ECDSACertificateVerifierReader),
}

#[derive(Debug, Clone)]
pub struct BN254CertificateVerifierReader {
    verifier_addr: Address,
    provider: String,
}

impl BN254CertificateVerifierReader {
    pub fn new(verifier_addr: Address, provider: String) -> Self {
        Self {
            verifier_addr,
            provider,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ECDSACertificateVerifierReader {
    verifier_addr: Address,
    provider: String,
}

impl ECDSACertificateVerifierReader {
    pub fn new(verifier_addr: Address, provider: String) -> Self {
        Self {
            verifier_addr,
            provider,
        }
    }
}

#[async_trait]
pub trait CertificateVerifierReaderTrait {
    async fn verify_certificate(
        &self,
        avs: Address,
        operator_set_id: u32,
        certificate: Bytes,
    ) -> Result<Vec<U256>, MultichainError>;
}

#[async_trait]
impl CertificateVerifierReaderTrait for BN254CertificateVerifierReader {
    #[instrument(skip(self, certificate))]
    async fn verify_certificate(
        &self,
        avs: Address,
        operator_set_id: u32,
        certificate: Bytes,
    ) -> Result<Vec<U256>, MultichainError> {
        let provider = get_provider(&self.provider);
        let contract = BN254CertificateVerifier::new(self.verifier_addr, &provider);

        let operator_set = ContractOperatorSet {
            avs,
            id: operator_set_id,
        };

        use alloy::sol_types::SolType;
        use eigen_utils::slashing::multichain::bn254_certificate_verifier::IBN254CertificateVerifierTypes::BN254Certificate as CertType;
        use eigen_utils::slashing::multichain::bn254_certificate_verifier::BN254CertificateVerifier::OperatorSet as BN254OperatorSet;

        let cert = <CertType as SolType>::abi_decode(&certificate).map_err(|e| {
            MultichainError::CertificateVerificationFailed(format!(
                "certificate decode failed: {}",
                e
            ))
        })?;

        let bn254_operator_set = BN254OperatorSet {
            avs: operator_set.avs,
            id: operator_set.id,
        };

        let result = contract
            .verifyCertificate(bn254_operator_set, cert)
            .call()
            .await
            .map_err(|e| {
                MultichainError::CertificateVerificationFailed(format!(
                    "verification call failed: {}",
                    e
                ))
            })?;

        Ok(result)
    }
}

#[async_trait]
impl CertificateVerifierReaderTrait for ECDSACertificateVerifierReader {
    #[instrument(skip(self, certificate))]
    async fn verify_certificate(
        &self,
        avs: Address,
        operator_set_id: u32,
        certificate: Bytes,
    ) -> Result<Vec<U256>, MultichainError> {
        let provider = get_provider(&self.provider);
        let contract = ECDSACertificateVerifier::new(self.verifier_addr, &provider);

        let operator_set = ContractOperatorSet {
            avs,
            id: operator_set_id,
        };

        use alloy::sol_types::SolType;
        use eigen_utils::slashing::multichain::ecdsa_certificate_verifier::IECDSACertificateVerifierTypes::ECDSACertificate as CertType;
        use eigen_utils::slashing::multichain::ecdsa_certificate_verifier::ECDSACertificateVerifier::OperatorSet as ECDSAOperatorSet;

        let cert = <CertType as SolType>::abi_decode(&certificate).map_err(|e| {
            MultichainError::CertificateVerificationFailed(format!(
                "certificate decode failed: {}",
                e
            ))
        })?;

        let ecdsa_operator_set = ECDSAOperatorSet {
            avs: operator_set.avs,
            id: operator_set.id,
        };

        let result = contract
            .verifyCertificate(ecdsa_operator_set, cert)
            .call()
            .await
            .map_err(|e| {
                MultichainError::CertificateVerificationFailed(format!(
                    "verification call failed: {}",
                    e
                ))
            })?;

        Ok(result)
    }
}

#[async_trait]
impl CertificateVerifierReaderTrait for CertificateVerifierReader {
    async fn verify_certificate(
        &self,
        avs: Address,
        operator_set_id: u32,
        certificate: Bytes,
    ) -> Result<Vec<U256>, MultichainError> {
        match self {
            CertificateVerifierReader::BN254(reader) => {
                reader
                    .verify_certificate(avs, operator_set_id, certificate)
                    .await
            }
            CertificateVerifierReader::ECDSA(reader) => {
                reader
                    .verify_certificate(avs, operator_set_id, certificate)
                    .await
            }
        }
    }
}
