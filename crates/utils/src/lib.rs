//! This module exports generated bindings.
pub mod common;
pub mod rewardsv2;
pub mod slashing;

use crate::slashing::core::allocation_manager::AllocationManager::OperatorSet;
use crate::slashing::core::i_rewards_coordinator::IRewardsCoordinator::OperatorSet as RewardsOperatorSet;
use crate::slashing::core::key_registrar::KeyRegistrar::OperatorSet as KeyRegistrarOperatorSet;
use crate::slashing::middleware::registry_coordinator::IStakeRegistryTypes::StrategyParams as RegistryCoordiinatorStrategyParams;
use crate::slashing::middleware::stake_registry::IStakeRegistryTypes::StrategyParams;
use crate::slashing::multichain::cross_chain_registry::CrossChainRegistry::OperatorSet as CrossChainRegistryOperatorSet;

pub fn convert_allocation_operator_set_to_rewards_operator_set(
    operator_set: OperatorSet,
) -> RewardsOperatorSet {
    RewardsOperatorSet {
        avs: operator_set.avs,
        id: operator_set.id,
    }
}

pub fn convert_cross_chain_registry_operator_set_to_key_registrar_operator_set(
    operator_set: CrossChainRegistryOperatorSet,
) -> KeyRegistrarOperatorSet {
    KeyRegistrarOperatorSet {
        avs: operator_set.avs,
        id: operator_set.id,
    }
}

pub fn convert_key_registrar_operator_set_to_cross_chain_registry_operator_set(
    operator_set: KeyRegistrarOperatorSet,
) -> CrossChainRegistryOperatorSet {
    CrossChainRegistryOperatorSet {
        avs: operator_set.avs,
        id: operator_set.id,
    }
}

pub fn convert_stake_registry_strategy_params_to_registry_coordinator_strategy_params(
    params: StrategyParams,
) -> RegistryCoordiinatorStrategyParams {
    RegistryCoordiinatorStrategyParams {
        strategy: params.strategy,
        multiplier: params.multiplier,
    }
}
