use alloy::primitives::{keccak256, Address, Bytes, FixedBytes, U256};
use alloy::providers::Provider;
use alloy::signers::{local::PrivateKeySigner, Signer};
use async_trait::async_trait;
use eigen_common::{get_provider, get_signer};
use eigen_types::multichain::{KeyType, OperatorKey, OperatorSet};
use eigen_utils::convert_cross_chain_registry_operator_set_to_key_registrar_operator_set;
use eigen_utils::slashing::core::key_registrar::KeyRegistrar;
use eigen_utils::slashing::multichain::cross_chain_registry::CrossChainRegistry;
use eigen_utils::slashing::multichain::cross_chain_registry::CrossChainRegistry::OperatorSet as ContractOperatorSet;
use std::str::FromStr;
use thiserror::Error;
use tracing::{info, instrument};

#[derive(Debug, Error)]
pub enum KeyManagerError {
    #[error("key already registered")]
    KeyAlreadyRegistered,
    #[error("key not found")]
    KeyNotFound,
    #[error("invalid key format")]
    InvalidKeyFormat,
    #[error("key registration failed: {0}")]
    RegistrationFailed(String),
    #[error("key rotation failed: {0}")]
    RotationFailed(String),
    #[error("signing failed: {0}")]
    SigningFailed(String),
    #[error("contract error: {0}")]
    ContractError(String),
    #[error("unsupported key type")]
    UnsupportedKeyType,
}

impl From<alloy::contract::Error> for KeyManagerError {
    fn from(e: alloy::contract::Error) -> Self {
        KeyManagerError::ContractError(e.to_string())
    }
}

#[async_trait]
pub trait KeyManager: Send + Sync {
    async fn register_key(
        &self,
        operator: Address,
        operator_set: OperatorSet,
        key_type: KeyType,
        key_data: Bytes,
        signature: FixedBytes<65>,
        expiry: u64,
    ) -> Result<FixedBytes<32>, KeyManagerError>;

    async fn rotate_key(
        &self,
        operator: Address,
        operator_set: OperatorSet,
        new_key_data: Bytes,
        signature: FixedBytes<65>,
        expiry: u64,
    ) -> Result<FixedBytes<32>, KeyManagerError>;

    async fn get_operator_key(
        &self,
        operator: Address,
        operator_set: OperatorSet,
    ) -> Result<OperatorKey, KeyManagerError>;

    async fn verify_key_uniqueness(&self, key_data: Bytes) -> Result<bool, KeyManagerError>;
}

pub struct DefaultKeyManager {
    provider: String,
    signer: String,
    key_registrar_addr: Option<Address>,
    cross_chain_registry_addr: Option<Address>,
}

pub fn encode_ecdsa_key(pubkey: &[u8]) -> Result<Bytes, KeyManagerError> {
    if pubkey.len() != 33 && pubkey.len() != 65 {
        return Err(KeyManagerError::InvalidKeyFormat);
    }
    Ok(Bytes::from(pubkey.to_vec()))
}

pub fn decode_ecdsa_key(key_data: &Bytes) -> Result<Vec<u8>, KeyManagerError> {
    let bytes = key_data.to_vec();
    if bytes.len() != 33 && bytes.len() != 65 {
        return Err(KeyManagerError::InvalidKeyFormat);
    }
    Ok(bytes)
}

impl DefaultKeyManager {
    pub fn new(provider: String, signer: String) -> Self {
        Self {
            provider,
            signer,
            key_registrar_addr: None,
            cross_chain_registry_addr: None,
        }
    }

    pub fn with_key_registrar(mut self, addr: Address) -> Self {
        self.key_registrar_addr = Some(addr);
        self
    }

    pub fn with_cross_chain_registry(mut self, addr: Address) -> Self {
        self.cross_chain_registry_addr = Some(addr);
        self
    }

    async fn get_key_registrar_addr(&self) -> Result<Address, KeyManagerError> {
        if let Some(addr) = self.key_registrar_addr {
            return Ok(addr);
        }

        let registry_addr = self.cross_chain_registry_addr.ok_or_else(|| {
            KeyManagerError::RegistrationFailed(
                "neither key_registrar_addr nor cross_chain_registry_addr provided".into(),
            )
        })?;

        let provider = get_provider(&self.provider);
        let registry = CrossChainRegistry::new(registry_addr, &provider);

        registry.keyRegistrar().call().await.map_err(Into::into)
    }

    fn compute_eip712_domain_separator(
        &self,
        key_registrar_addr: Address,
        chain_id: u64,
    ) -> FixedBytes<32> {
        let domain_type_hash = keccak256(
            "EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)",
        );
        let name_hash = keccak256("KeyRegistrar");
        let version_hash = keccak256("1");

        let encoded = alloy::sol_types::SolValue::abi_encode_packed(&(
            domain_type_hash,
            name_hash,
            version_hash,
            U256::from(chain_id),
            key_registrar_addr,
        ));

        keccak256(encoded)
    }

    fn compute_ecdsa_registration_message_hash(
        &self,
        operator: Address,
        operator_set: &OperatorSet,
        key_data: &Bytes,
    ) -> FixedBytes<32> {
        let type_hash = keccak256(
            "ECDSAKeyRegistration(address operator,address avs,uint32 operatorSetId,bytes keyData)",
        );

        let encoded = alloy::sol_types::SolValue::abi_encode_packed(&(
            type_hash,
            operator,
            operator_set.avs,
            operator_set.id,
            keccak256(key_data),
        ));

        keccak256(encoded)
    }

    fn compute_bn254_registration_message_hash(
        &self,
        operator: Address,
        operator_set: &OperatorSet,
        key_data: &Bytes,
    ) -> FixedBytes<32> {
        let type_hash = keccak256(
            "BN254KeyRegistration(address operator,address avs,uint32 operatorSetId,bytes keyData)",
        );

        let encoded = alloy::sol_types::SolValue::abi_encode_packed(&(
            type_hash,
            operator,
            operator_set.avs,
            operator_set.id,
            keccak256(key_data),
        ));

        keccak256(encoded)
    }

    async fn sign_typed_data(
        &self,
        domain_separator: FixedBytes<32>,
        struct_hash: FixedBytes<32>,
    ) -> Result<FixedBytes<65>, KeyManagerError> {
        let digest = keccak256(
            [
                &[0x19, 0x01],
                domain_separator.as_slice(),
                struct_hash.as_slice(),
            ]
            .concat(),
        );

        let signer = PrivateKeySigner::from_str(&self.signer)
            .map_err(|e| KeyManagerError::SigningFailed(format!("invalid signer key: {}", e)))?;

        let signature = signer
            .sign_hash(&digest)
            .await
            .map_err(|e| KeyManagerError::SigningFailed(e.to_string()))?;

        let mut sig_bytes = [0u8; 65];
        sig_bytes[..32].copy_from_slice(&signature.r().to_be_bytes::<32>());
        sig_bytes[32..64].copy_from_slice(&signature.s().to_be_bytes::<32>());
        sig_bytes[64] = if signature.v() { 28 } else { 27 };

        Ok(FixedBytes::from(sig_bytes))
    }
}

#[async_trait]
impl KeyManager for DefaultKeyManager {
    #[instrument(skip(self, key_data, _signature))]
    async fn register_key(
        &self,
        operator: Address,
        operator_set: OperatorSet,
        key_type: KeyType,
        key_data: Bytes,
        _signature: FixedBytes<65>,
        _expiry: u64,
    ) -> Result<FixedBytes<32>, KeyManagerError> {
        info!(
            operator = %operator,
            avs = %operator_set.avs,
            operator_set_id = operator_set.id,
            key_type = ?key_type,
            "registering operator key"
        );

        match key_type {
            KeyType::ECDSA => {
                decode_ecdsa_key(&key_data)?;

                let key_registrar_addr = self.get_key_registrar_addr().await?;
                let domain_separator = self.compute_eip712_domain_separator(key_registrar_addr, 1);
                let message_hash = self.compute_ecdsa_registration_message_hash(
                    operator,
                    &operator_set,
                    &key_data,
                );
                let signature = self.sign_typed_data(domain_separator, message_hash).await?;

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
                    .map_err(|e| {
                        KeyManagerError::RegistrationFailed(format!("send failed: {}", e))
                    })?;

                let receipt = tx.get_receipt().await.map_err(|e| {
                    KeyManagerError::RegistrationFailed(format!("receipt failed: {}", e))
                })?;

                Ok(receipt.transaction_hash)
            }
            KeyType::BN254 => {
                let key_registrar_addr = self.get_key_registrar_addr().await?;
                let domain_separator = self.compute_eip712_domain_separator(key_registrar_addr, 1);
                let message_hash = self.compute_bn254_registration_message_hash(
                    operator,
                    &operator_set,
                    &key_data,
                );
                let signature = self.sign_typed_data(domain_separator, message_hash).await?;

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
                    .map_err(|e| {
                        KeyManagerError::RegistrationFailed(format!("send failed: {}", e))
                    })?;

                let receipt = tx.get_receipt().await.map_err(|e| {
                    KeyManagerError::RegistrationFailed(format!("receipt failed: {}", e))
                })?;

                Ok(receipt.transaction_hash)
            }
        }
    }

    #[instrument(skip(self, new_key_data, _signature))]
    async fn rotate_key(
        &self,
        operator: Address,
        operator_set: OperatorSet,
        new_key_data: Bytes,
        _signature: FixedBytes<65>,
        _expiry: u64,
    ) -> Result<FixedBytes<32>, KeyManagerError> {
        info!(
            operator = %operator,
            avs = %operator_set.avs,
            operator_set_id = operator_set.id,
            "rotating operator key"
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

        let deregister_tx = key_registrar
            .deregisterKey(operator, contract_operator_set.clone())
            .send()
            .await
            .map_err(|e| {
                KeyManagerError::RotationFailed(format!("deregister send failed: {}", e))
            })?;

        deregister_tx.get_receipt().await.map_err(|e| {
            KeyManagerError::RotationFailed(format!("deregister receipt failed: {}", e))
        })?;

        let domain_separator = self.compute_eip712_domain_separator(key_registrar_addr, 1);
        let message_hash =
            self.compute_ecdsa_registration_message_hash(operator, &operator_set, &new_key_data);
        let signature = self.sign_typed_data(domain_separator, message_hash).await?;
        let encoded_signature = signature.to_vec().into();

        let register_tx = key_registrar
            .registerKey(
                operator,
                contract_operator_set,
                new_key_data,
                encoded_signature,
            )
            .send()
            .await
            .map_err(|e| KeyManagerError::RotationFailed(format!("register send failed: {}", e)))?;

        let receipt = register_tx.get_receipt().await.map_err(|e| {
            KeyManagerError::RotationFailed(format!("register receipt failed: {}", e))
        })?;

        Ok(receipt.transaction_hash)
    }

    #[instrument(skip(self))]
    async fn get_operator_key(
        &self,
        operator: Address,
        operator_set: OperatorSet,
    ) -> Result<OperatorKey, KeyManagerError> {
        info!(
            operator = %operator,
            avs = %operator_set.avs,
            operator_set_id = operator_set.id,
            "retrieving operator key"
        );

        let key_registrar_addr = self.get_key_registrar_addr().await?;
        let provider = get_provider(&self.provider);
        let key_registrar = KeyRegistrar::new(key_registrar_addr, &provider);

        let contract_operator_set =
            convert_cross_chain_registry_operator_set_to_key_registrar_operator_set(
                ContractOperatorSet {
                    avs: operator_set.avs,
                    id: operator_set.id,
                },
            );

        let result = key_registrar
            .getECDSAKey(contract_operator_set, operator)
            .call()
            .await?;

        Ok(OperatorKey {
            operator,
            operator_set: operator_set.clone(),
            key_type: KeyType::ECDSA,
            key_data: result.into(),
        })
    }

    #[instrument(skip(self, _key_data))]
    async fn verify_key_uniqueness(&self, _key_data: Bytes) -> Result<bool, KeyManagerError> {
        info!("verifying key uniqueness");
        todo!("implement key uniqueness verification across operator sets")
    }
}
