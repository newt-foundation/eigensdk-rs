use alloy::primitives::U256;
use ark_ec::AffineRepr;
use ark_ff::PrimeField;
use eigen_services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigen_utils::slashing::multichain::operator_table_updater::IBN254CertificateVerifierTypes::{
    BN254Certificate, BN254OperatorInfoWitness,
};
use eigen_utils::slashing::multichain::operator_table_updater::IOperatorTableCalculatorTypes::BN254OperatorInfo;
use eigen_utils::slashing::multichain::operator_table_updater::BN254::{G1Point, G2Point};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CertificateConstructorError {
    #[error("invalid g1 point: {0}")]
    InvalidG1Point(String),
    #[error("invalid g2 point: {0}")]
    InvalidG2Point(String),
    #[error("witness construction failed: {0}")]
    WitnessConstructionFailed(String),
}

pub fn construct_bn254_certificate(
    response: &BlsAggregationServiceResponse,
    reference_timestamp: u32,
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

    Ok(BN254Certificate {
        referenceTimestamp: reference_timestamp,
        messageHash: response.task_response_digest,
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

        let operator_info = BN254OperatorInfo { pubkey, weights };

        witnesses.push(BN254OperatorInfoWitness {
            operatorIndex: operator_index,
            operatorInfoProof: alloy::primitives::Bytes::new(),
            operatorInfo: operator_info,
        });
    }

    Ok(witnesses)
}
