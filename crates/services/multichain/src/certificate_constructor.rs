use alloy::primitives::{keccak256, U256};
use alloy::sol_types::SolValue;
use ark_ec::AffineRepr;
use ark_ff::PrimeField;
use eigen_services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigen_utils::slashing::multichain::operator_table_updater::IBN254CertificateVerifierTypes::{
    BN254Certificate, BN254OperatorInfoWitness,
};
use eigen_utils::slashing::multichain::operator_table_updater::IOperatorTableCalculatorTypes::BN254OperatorInfo as TaskManagerOperatorInfo;
use eigen_utils::slashing::multichain::bn254_certificate_verifier::IOperatorTableCalculatorTypes::BN254OperatorInfo;
use eigen_utils::slashing::multichain::operator_table_updater::BN254::{G1Point, G2Point};
use rs_merkle::{algorithms::Sha256, Hasher, MerkleTree};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CertificateConstructorError {
    #[error("invalid g1 point: {0}")]
    InvalidG1Point(String),
    #[error("invalid g2 point: {0}")]
    InvalidG2Point(String),
    #[error("witness construction failed: {0}")]
    WitnessConstructionFailed(String),
    #[error("operator table incomplete: {0}")]
    OperatorTableIncomplete(String),
    #[error("contract call failed: {0}")]
    ContractError(String),
}

// custom keccak256 hasher for rs_merkle to match solidity behavior
#[derive(Clone)]
struct Keccak256Algorithm;

impl Hasher for Keccak256Algorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        keccak256(data).into()
    }
}

pub fn construct_bn254_certificate<T: SolValue>(
    response: &BlsAggregationServiceResponse,
    reference_timestamp: u32,
    task_response: &T,
) -> Result<BN254Certificate, CertificateConstructorError> {
    let sig_g1 = response.signers_agg_sig_g1.g1_point().g1();
    let sig_x = U256::from_limbs(sig_g1.x().unwrap().into_bigint().0);
    let sig_y = U256::from_limbs(sig_g1.y().unwrap().into_bigint().0);
    let signature = G1Point { X: sig_x, Y: sig_y };

    let apk_g2 = response.signers_apk_g2.g2();
    let apk_x = apk_g2.x().unwrap();
    let apk_y = apk_g2.y().unwrap();
    let apk = G2Point {
        X: [
            U256::from_limbs(apk_x.c0.into_bigint().0),
            U256::from_limbs(apk_x.c1.into_bigint().0),
        ],
        Y: [
            U256::from_limbs(apk_y.c0.into_bigint().0),
            U256::from_limbs(apk_y.c1.into_bigint().0),
        ],
    };

    let non_signer_witnesses = construct_non_signer_witnesses(response)?;

    // compute message hash from task response instead of using response.task_response_digest
    // because in multichain mode, response.task_response_digest is the full certificate digest
    // but the certificate struct expects just the plain message hash
    let message_hash = alloy::primitives::keccak256(task_response.abi_encode());

    Ok(BN254Certificate {
        referenceTimestamp: reference_timestamp,
        messageHash: message_hash,
        signature,
        apk,
        nonSignerWitnesses: non_signer_witnesses,
    })
}

fn construct_non_signer_witnesses(
    response: &BlsAggregationServiceResponse,
) -> Result<Vec<BN254OperatorInfoWitness>, CertificateConstructorError> {
    let mut witnesses = Vec::with_capacity(response.non_signers_pub_keys_g1.len());

    for (idx, non_signer_pubkey) in response.non_signers_pub_keys_g1.iter().enumerate() {
        let g1_affine = non_signer_pubkey.g1();
        let x = U256::from_limbs(g1_affine.x().unwrap().into_bigint().0);
        let y = U256::from_limbs(g1_affine.y().unwrap().into_bigint().0);
        let pubkey = G1Point { X: x, Y: y };

        let operator_index = response
            .non_signer_stake_indices
            .get(idx)
            .and_then(|indices| indices.first())
            .copied()
            .ok_or_else(|| {
                CertificateConstructorError::WitnessConstructionFailed(format!(
                    "missing stake index for non-signer {}",
                    idx
                ))
            })?;

        let weights = if let Some(stake_indices) = response.non_signer_stake_indices.get(idx) {
            stake_indices.iter().map(|&idx| U256::from(idx)).collect()
        } else {
            vec![]
        };

        let operator_info = TaskManagerOperatorInfo { pubkey, weights };

        witnesses.push(BN254OperatorInfoWitness {
            operatorIndex: operator_index,
            operatorInfoProof: alloy::primitives::Bytes::new(),
            operatorInfo: operator_info,
        });
    }

    Ok(witnesses)
}

pub async fn construct_bn254_certificate_with_proofs<T: SolValue, P, N>(
    response: &BlsAggregationServiceResponse,
    reference_timestamp: u32,
    task_response: &T,
    certificate_verifier: eigen_utils::slashing::multichain::bn254_certificate_verifier::BN254CertificateVerifier::BN254CertificateVerifierInstance<P, N>,
    operator_set: eigen_utils::slashing::multichain::bn254_certificate_verifier::BN254CertificateVerifier::OperatorSet,
) -> Result<BN254Certificate, CertificateConstructorError>
where
    P: alloy::providers::Provider<N> + Clone,
    N: alloy::providers::Network,
{
    let sig_g1 = response.signers_agg_sig_g1.g1_point().g1();
    let sig_x = U256::from_limbs(sig_g1.x().unwrap().into_bigint().0);
    let sig_y = U256::from_limbs(sig_g1.y().unwrap().into_bigint().0);
    let signature = G1Point { X: sig_x, Y: sig_y };

    let apk_g2 = response.signers_apk_g2.g2();
    let apk_x = apk_g2.x().unwrap();
    let apk_y = apk_g2.y().unwrap();
    let apk = G2Point {
        X: [
            U256::from_limbs(apk_x.c0.into_bigint().0),
            U256::from_limbs(apk_x.c1.into_bigint().0),
        ],
        Y: [
            U256::from_limbs(apk_y.c0.into_bigint().0),
            U256::from_limbs(apk_y.c1.into_bigint().0),
        ],
    };

    let message_hash = keccak256(task_response.abi_encode());

    if response.non_signers_pub_keys_g1.is_empty() {
        return Ok(BN254Certificate {
            referenceTimestamp: reference_timestamp,
            messageHash: message_hash,
            signature,
            apk,
            nonSignerWitnesses: vec![],
        });
    }

    let operator_set_info = certificate_verifier
        .getOperatorSetInfo(operator_set.clone(), reference_timestamp)
        .call()
        .await
        .map_err(|e| CertificateConstructorError::ContractError(format!("failed to get operator set info: {}", e)))?;

    let num_operators = operator_set_info.numOperators.to::<usize>();

    // fetch all cached operators to reconstruct table
    let operator_futures = (0..num_operators).map(|operator_index| {
        let cert_verifier = certificate_verifier.clone();
        let op_set = operator_set.clone();
        async move {
            let operator_info = cert_verifier
                .getNonsignerOperatorInfo(op_set, reference_timestamp, U256::from(operator_index))
                .call()
                .await
                .map_err(|e| CertificateConstructorError::ContractError(format!("failed to get operator info: {}", e)))?;

            if operator_info.pubkey.X == U256::ZERO && operator_info.pubkey.Y == U256::ZERO {
                return Err(CertificateConstructorError::OperatorTableIncomplete(
                    format!("operator {} not cached", operator_index)
                ));
            }

            Ok(operator_info)
        }
    });

    let operator_table = futures::future::try_join_all(operator_futures).await?;

    // calculate merkle leaves: keccak256(abi.encode(operator_info))
    let leaves: Vec<[u8; 32]> = operator_table
        .iter()
        .map(|op_info| keccak256(op_info.abi_encode()).into())
        .collect();

    let merkle_tree = MerkleTree::<Keccak256Algorithm>::from_leaves(&leaves);

    let non_signer_witnesses = response
        .non_signers_pub_keys_g1
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            let operator_index = response
                .non_signer_stake_indices
                .get(idx)
                .and_then(|indices| indices.first())
                .copied()
                .ok_or_else(|| {
                    CertificateConstructorError::WitnessConstructionFailed(
                        format!("missing stake index for non-signer {}", idx)
                    )
                })?;

            let verifier_operator_info = operator_table.get(operator_index as usize).ok_or_else(|| {
                CertificateConstructorError::WitnessConstructionFailed(
                    format!("operator index {} out of bounds", operator_index)
                )
            })?;

            // convert from verifier's BN254OperatorInfo to task manager's BN254OperatorInfo
            let operator_info = TaskManagerOperatorInfo {
                pubkey: G1Point {
                    X: verifier_operator_info.pubkey.X,
                    Y: verifier_operator_info.pubkey.Y,
                },
                weights: verifier_operator_info.weights.clone(),
            };

            let proof = merkle_tree.proof(&[operator_index as usize]);
            let proof_bytes = proof.proof_hashes().iter().flat_map(|hash| hash.iter().copied()).collect::<Vec<u8>>();

            Ok(BN254OperatorInfoWitness {
                operatorIndex: operator_index,
                operatorInfoProof: alloy::primitives::Bytes::from(proof_bytes),
                operatorInfo: operator_info,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(BN254Certificate {
        referenceTimestamp: reference_timestamp,
        messageHash: message_hash,
        signature,
        apk,
        nonSignerWitnesses: non_signer_witnesses,
    })
}
