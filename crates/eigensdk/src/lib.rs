#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

/* --------------------------------------- Core re-exports -------------------------------------- */

#[doc(inline)]
#[cfg(feature = "types")]
pub use newton_types as types;

#[doc(inline)]
#[cfg(feature = "utils")]
pub use newton_utils as utils;

#[doc(inline)]
#[cfg(feature = "crypto-bls")]
pub use newton_crypto_bls as crypto_bls;

#[doc(inline)]
#[cfg(feature = "crypto-bn254")]
pub use newton_crypto_bn254 as crypto_bn254;

#[doc(inline)]
#[cfg(feature = "signer")]
pub use newton_signer as signer;

#[doc(inline)]
#[cfg(feature = "metrics")]
pub use newton_metrics as metrics;

/* ------------------------------------- Client Re-exports ------------------------------------- */

#[doc(inline)]
#[cfg(feature = "client-avsregistry")]
pub use newton_client_avsregistry as client_avsregistry;

#[doc(inline)]
#[cfg(feature = "client-elcontracts")]
pub use newton_client_elcontracts as client_elcontracts;

#[doc(inline)]
#[cfg(feature = "client-eth")]
pub use newton_client_eth as client_eth;

#[doc(inline)]
#[cfg(feature = "client-fireblocks")]
pub use newton_client_fireblocks as client_fireblocks;

/* ------------------------------------- Services Re-exports ------------------------------------- */

#[doc(inline)]
#[cfg(feature = "services-avsregistry")]
pub use newton_services_avsregistry as services_avsregistry;

#[doc(inline)]
#[cfg(feature = "services-blsaggregation")]
pub use newton_services_blsaggregation as services_blsaggregation;

#[doc(inline)]
#[cfg(feature = "services-operatorsinfo")]
pub use newton_services_operatorsinfo as services_operatorsinfo;

/* ------------------------------------ Node API Re-export ------------------------------------ */

#[doc(inline)]
#[cfg(feature = "nodeapi")]
pub use newton_nodeapi as nodeapi;

/* ------------------------------------ Metrics Collectors Re-exports -------------------------- */

#[doc(inline)]
#[cfg(feature = "metrics-collectors-economic")]
pub use newton_metrics_collectors_economic as metrics_collectors_economic;

#[doc(inline)]
#[cfg(feature = "metrics-collectors-rpc-calls")]
pub use newton_metrics_collectors_rpc_calls as metrics_collectors_rpc_calls;

/* ------------------------------------ Common Utilities Re-exports -------------------------- */

#[doc(inline)]
#[cfg(feature = "common")]
pub use newton_common as common;
