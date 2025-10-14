use crate::error::MultichainError;
use alloy::primitives::{Address, Bytes, FixedBytes};
use alloy::providers::Provider;
use async_trait::async_trait;
use eigen_common::{get_provider, get_signer};
use eigen_utils::slashing::multichain::operator_table_updater::OperatorTableUpdater;
use tracing::instrument;

#[derive(Debug, Clone)]
pub struct OperatorTableUpdaterReader {
    updater_addr: Address,
    provider: String,
}

impl OperatorTableUpdaterReader {
    pub fn new(updater_addr: Address, provider: String) -> Self {
        Self {
            updater_addr,
            provider,
        }
    }
}

#[async_trait]
pub trait OperatorTableUpdaterReaderTrait {
    async fn get_current_global_table_root(&self) -> Result<FixedBytes<32>, MultichainError>;

    async fn get_global_table_root_by_timestamp(
        &self,
        timestamp: u32,
    ) -> Result<FixedBytes<32>, MultichainError>;
}

#[async_trait]
impl OperatorTableUpdaterReaderTrait for OperatorTableUpdaterReader {
    #[instrument(skip(self))]
    async fn get_current_global_table_root(&self) -> Result<FixedBytes<32>, MultichainError> {
        let provider = get_provider(&self.provider);
        let contract = OperatorTableUpdater::new(self.updater_addr, &provider);

        contract
            .getCurrentGlobalTableRoot()
            .call()
            .await
            .map_err(Into::into)
    }

    #[instrument(skip(self))]
    async fn get_global_table_root_by_timestamp(
        &self,
        timestamp: u32,
    ) -> Result<FixedBytes<32>, MultichainError> {
        let provider = get_provider(&self.provider);
        let contract = OperatorTableUpdater::new(self.updater_addr, &provider);

        contract
            .getGlobalTableRootByTimestamp(timestamp)
            .call()
            .await
            .map_err(Into::into)
    }
}

#[derive(Debug)]
pub struct OperatorTableUpdaterWriter {
    updater_addr: Address,
    provider: String,
    signer: String,
}

impl OperatorTableUpdaterWriter {
    pub fn new(updater_addr: Address, provider: String, signer: String) -> Self {
        Self {
            updater_addr,
            provider,
            signer,
        }
    }

    pub fn set_signer(&mut self, signer: String) {
        self.signer = signer;
    }

    #[instrument(skip(self, proof, operator_table_bytes))]
    pub async fn update_operator_table(
        &self,
        reference_timestamp: u32,
        global_table_root: FixedBytes<32>,
        operator_set_index: u32,
        proof: Bytes,
        operator_table_bytes: Bytes,
    ) -> Result<FixedBytes<32>, MultichainError> {
        let provider = get_signer(&self.signer, &self.provider);
        let contract = OperatorTableUpdater::new(self.updater_addr, provider);

        let tx = contract
            .updateOperatorTable(
                reference_timestamp,
                global_table_root,
                operator_set_index,
                proof,
                operator_table_bytes,
            )
            .send()
            .await
            .map_err(|e| {
                MultichainError::OperatorTableUpdateFailed(format!("send failed: {}", e))
            })?;

        let receipt = tx.get_receipt().await.map_err(|e| {
            MultichainError::OperatorTableUpdateFailed(format!("receipt failed: {}", e))
        })?;

        Ok(receipt.transaction_hash)
    }
}
