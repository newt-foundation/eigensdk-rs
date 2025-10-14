use alloy::primitives::{Address, Bytes, FixedBytes};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperatorSet {
    pub avs: Address,
    pub id: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    ECDSA,
    BN254,
}

#[derive(Debug, Clone)]
pub struct OperatorKey {
    pub operator: Address,
    pub operator_set: OperatorSet,
    pub key_type: KeyType,
    pub key_data: Bytes,
}

#[derive(Debug, Clone)]
pub struct Certificate {
    pub avs: Address,
    pub operator_set_id: u32,
    pub certificate_data: Bytes,
}

#[derive(Debug, Clone)]
pub struct OperatorTableEntry {
    pub operator: Address,
    pub stake: u128,
    pub key_data: Bytes,
}

#[derive(Debug, Clone)]
pub struct OperatorTable {
    pub avs: Address,
    pub operator_set_id: u32,
    pub block_number: u64,
    pub operators: Vec<OperatorTableEntry>,
    pub merkle_root: FixedBytes<32>,
}

#[derive(Debug, Clone)]
pub struct OperatorSetConfig {
    pub owner: Address,
    pub max_staleness_period: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChainType {
    Source,
    Destination,
}

#[derive(Debug, Clone)]
pub struct ChainConfig {
    pub chain_id: u64,
    pub chain_type: ChainType,
    pub cross_chain_registry_addr: Address,
    pub certificate_verifier_addr: Option<Address>,
    pub operator_table_updater_addr: Option<Address>,
}
