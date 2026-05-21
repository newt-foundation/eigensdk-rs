use crate::error::MultichainError;
use alloy::primitives::Address;
use alloy::providers::Provider;
use async_trait::async_trait;
use newton_common::get_provider;
use newton_types::multichain::OperatorSetConfig;
use newton_utils::convert_cross_chain_registry_operator_set_to_key_registrar_operator_set;
use newton_utils::slashing::core::key_registrar::KeyRegistrar;
use newton_utils::slashing::multichain::cross_chain_registry::CrossChainRegistry;
use newton_utils::slashing::multichain::cross_chain_registry::CrossChainRegistry::OperatorSet as ContractOperatorSet;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct CrossChainRegistryReader {
    cross_chain_registry_addr: Address,
    key_registrar_addr: Address,
    provider: String,
}

impl CrossChainRegistryReader {
    pub fn new(
        cross_chain_registry_addr: Address,
        key_registrar_addr: Address,
        provider: String,
    ) -> Self {
        Self {
            cross_chain_registry_addr,
            key_registrar_addr,
            provider,
        }
    }
}

#[async_trait]
pub trait CrossChainRegistryReaderTrait {
    async fn get_operator_set_config(
        &self,
        avs: Address,
        operator_set_id: u32,
    ) -> Result<OperatorSetConfig, MultichainError>;

    async fn is_operator_registered(
        &self,
        avs: Address,
        operator_set_id: u32,
        operator: Address,
    ) -> Result<bool, MultichainError>;
}

#[async_trait]
impl CrossChainRegistryReaderTrait for CrossChainRegistryReader {
    #[instrument(skip(self))]
    async fn get_operator_set_config(
        &self,
        avs: Address,
        operator_set_id: u32,
    ) -> Result<OperatorSetConfig, MultichainError> {
        let provider = get_provider(&self.provider);
        let contract = CrossChainRegistry::new(self.cross_chain_registry_addr, &provider);

        let operator_set = ContractOperatorSet {
            avs,
            id: operator_set_id,
        };
        let config = contract.getOperatorSetConfig(operator_set).call().await?;

        Ok(OperatorSetConfig {
            owner: config.owner,
            max_staleness_period: config.maxStalenessPeriod,
        })
    }

    #[instrument(skip(self))]
    async fn is_operator_registered(
        &self,
        avs: Address,
        operator_set_id: u32,
        operator: Address,
    ) -> Result<bool, MultichainError> {
        let provider = get_provider(&self.provider);
        let key_registrar = KeyRegistrar::new(self.key_registrar_addr, &provider);

        let operator_set = convert_cross_chain_registry_operator_set_to_key_registrar_operator_set(
            ContractOperatorSet {
                avs,
                id: operator_set_id,
            },
        );

        key_registrar
            .isRegistered(operator_set, operator)
            .call()
            .await
            .map_err(Into::into)
    }
}

#[derive(Debug)]
pub struct CrossChainRegistryWriter {
    cross_chain_registry_addr: Address,
    provider: String,
    signer: String,
}

impl CrossChainRegistryWriter {
    pub fn new(cross_chain_registry_addr: Address, provider: String, signer: String) -> Self {
        Self {
            cross_chain_registry_addr,
            provider,
            signer,
        }
    }

    pub fn set_signer(&mut self, signer: String) {
        self.signer = signer;
    }
}
