use crate::error::MultichainError;
use alloy::primitives::{Address, Bytes, FixedBytes};
use alloy::providers::Provider;
use async_trait::async_trait;
use eigen_common::{get_provider, get_signer};
use eigen_types::multichain::{KeyType, OperatorSet};
use eigen_utils::convert_cross_chain_registry_operator_set_to_key_registrar_operator_set;
use eigen_utils::slashing::core::key_registrar::KeyRegistrar;
use eigen_utils::slashing::multichain::cross_chain_registry::CrossChainRegistry;
use eigen_utils::slashing::multichain::cross_chain_registry::CrossChainRegistry::OperatorSet as ContractOperatorSet;
use tracing::{info, instrument};

#[async_trait]
pub trait MultichainOperatorRegistration {
    async fn register_operator_with_multichain(
        &self,
        operator: Address,
        operator_set: OperatorSet,
        key_type: KeyType,
        key_data: Bytes,
        signature: FixedBytes<65>,
        expiry: u64,
    ) -> Result<FixedBytes<32>, MultichainError>;

    async fn deregister_operator_from_multichain(
        &self,
        operator: Address,
        operator_set: OperatorSet,
    ) -> Result<FixedBytes<32>, MultichainError>;
}

pub struct MultichainOperatorRegistrar {
    provider: String,
    signer: String,
    cross_chain_registry_addr: Address,
    key_registrar_addr: Option<Address>,
}

impl MultichainOperatorRegistrar {
    pub fn new(provider: String, signer: String, cross_chain_registry_addr: Address) -> Self {
        Self {
            provider,
            signer,
            cross_chain_registry_addr,
            key_registrar_addr: None,
        }
    }

    pub fn with_key_registrar(mut self, key_registrar_addr: Address) -> Self {
        self.key_registrar_addr = Some(key_registrar_addr);
        self
    }

    pub fn set_signer(&mut self, signer: String) {
        self.signer = signer;
    }

    async fn get_key_registrar_addr(&self) -> Result<Address, MultichainError> {
        if let Some(addr) = self.key_registrar_addr {
            return Ok(addr);
        }

        let provider = get_provider(&self.provider);
        let registry = CrossChainRegistry::new(self.cross_chain_registry_addr, &provider);

        registry.keyRegistrar().call().await.map_err(Into::into)
    }
}

#[async_trait]
impl MultichainOperatorRegistration for MultichainOperatorRegistrar {
    #[instrument(skip(self, key_data, signature))]
    async fn register_operator_with_multichain(
        &self,
        operator: Address,
        operator_set: OperatorSet,
        key_type: KeyType,
        key_data: Bytes,
        signature: FixedBytes<65>,
        expiry: u64,
    ) -> Result<FixedBytes<32>, MultichainError> {
        info!(
            operator = %operator,
            avs = %operator_set.avs,
            operator_set_id = operator_set.id,
            key_type = ?key_type,
            "registering operator with multichain"
        );

        let key_registrar_addr = self.get_key_registrar_addr().await?;
        let provider = get_signer(&self.signer, &self.provider);
        let key_registrar = KeyRegistrar::new(key_registrar_addr, provider);

        let contract_operator_set =
            convert_cross_chain_registry_operator_set_to_key_registrar_operator_set(
                ContractOperatorSet {
                    avs: operator_set.avs,
                    id: operator_set.id,
                },
            );

        let encoded_signature = signature.to_vec().into();

        let tx = key_registrar
            .registerKey(operator, contract_operator_set, key_data, encoded_signature)
            .send()
            .await
            .map_err(|e| MultichainError::KeyRegistrationFailed(format!("send failed: {}", e)))?;

        let receipt = tx.get_receipt().await.map_err(|e| {
            MultichainError::KeyRegistrationFailed(format!("receipt failed: {}", e))
        })?;

        Ok(receipt.transaction_hash)
    }

    #[instrument(skip(self))]
    async fn deregister_operator_from_multichain(
        &self,
        operator: Address,
        operator_set: OperatorSet,
    ) -> Result<FixedBytes<32>, MultichainError> {
        info!(
            operator = %operator,
            avs = %operator_set.avs,
            operator_set_id = operator_set.id,
            "deregistering operator from multichain"
        );

        let key_registrar_addr = self.get_key_registrar_addr().await?;
        let provider = get_signer(&self.signer, &self.provider);
        let key_registrar = KeyRegistrar::new(key_registrar_addr, provider);

        let contract_operator_set =
            convert_cross_chain_registry_operator_set_to_key_registrar_operator_set(
                ContractOperatorSet {
                    avs: operator_set.avs,
                    id: operator_set.id,
                },
            );

        let tx = key_registrar
            .deregisterKey(operator, contract_operator_set)
            .send()
            .await
            .map_err(|e| {
                MultichainError::KeyRegistrationFailed(format!("deregister send failed: {}", e))
            })?;

        let receipt = tx.get_receipt().await.map_err(|e| {
            MultichainError::KeyRegistrationFailed(format!("deregister receipt failed: {}", e))
        })?;

        Ok(receipt.transaction_hash)
    }
}
