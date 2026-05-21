#!/bin/bash
set -euo pipefail

echo "Step 1: Updating workspace Cargo.toml version and repository..."
sed -i.bak 's/version = "2.0.0"/version = "0.4.7"/g' Cargo.toml
sed -i.bak 's|repository = "https://github.com/Layr-Labs/eigensdk-rs"|repository = "https://github.com/newt-foundation/eigensdk-rs"|' Cargo.toml

echo "Step 2: Updating workspace.dependencies keys in root Cargo.toml..."
# Update all workspace dependency declarations
sed -i.bak 's/^eigensdk = { version/newton-eigensdk = { version/' Cargo.toml
sed -i.bak 's/^eigen-cli = { version/newton-eigen-cli = { version/' Cargo.toml
sed -i.bak 's/^eigen-common = { version/newton-common = { version/' Cargo.toml
sed -i.bak 's/^eigen-client-avsregistry = { version/newton-client-avsregistry = { version/' Cargo.toml
sed -i.bak 's/^eigen-client-elcontracts = { version/newton-client-elcontracts = { version/' Cargo.toml
sed -i.bak 's/^eigen-client-eth = { version/newton-client-eth = { version/' Cargo.toml
sed -i.bak 's/^eigen-client-fireblocks = { version/newton-client-fireblocks = { version/' Cargo.toml
sed -i.bak 's/^eigen-client-multichain = { version/newton-client-multichain = { version/' Cargo.toml
sed -i.bak 's/^eigen-crypto-bls = { version/newton-crypto-bls = { version/' Cargo.toml
sed -i.bak 's/^eigen-crypto-bn254 = { version/newton-crypto-bn254 = { version/' Cargo.toml
sed -i.bak 's/^eigen-crypto-ecdsa = { version/newton-crypto-ecdsa = { version/' Cargo.toml
sed -i.bak 's/^eigen-metrics = { version/newton-metrics = { version/' Cargo.toml
sed -i.bak 's/^eigen-metrics-collectors-economic = { version/newton-metrics-collectors-economic = { version/' Cargo.toml
sed -i.bak 's/^eigen-metrics-collectors-rpc-calls = { version/newton-metrics-collectors-rpc-calls = { version/' Cargo.toml
sed -i.bak 's/^eigen-services-avsregistry = { version/newton-services-avsregistry = { version/' Cargo.toml
sed -i.bak 's/^eigen-services-blsaggregation = { version/newton-services-blsaggregation = { version/' Cargo.toml
sed -i.bak 's/^eigen-services-multichain = { version/newton-services-multichain = { version/' Cargo.toml
sed -i.bak 's/^eigen-services-operatorsinfo = { version/newton-services-operatorsinfo = { version/' Cargo.toml
sed -i.bak 's/^eigen-signer = { version/newton-signer = { version/' Cargo.toml
sed -i.bak 's/^eigen-types = { version/newton-types = { version/' Cargo.toml
sed -i.bak 's/^eigen-utils = { version/newton-utils = { version/' Cargo.toml
sed -i.bak 's/^eigen-nodeapi = { version/newton-nodeapi = { version/' Cargo.toml
sed -i.bak 's/^eigen-testing-utils = { version/newton-testing-utils = { version/' Cargo.toml

echo "Step 3: Updating package names in all Cargo.toml files..."
find . -name Cargo.toml -type f | grep -v target | while read -r cargo_file; do
    # Update package name fields
    sed -i.bak 's/^name = "eigensdk"/name = "newton-eigensdk"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-cli"/name = "newton-eigen-cli"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-common"/name = "newton-common"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-client-avsregistry"/name = "newton-client-avsregistry"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-client-elcontracts"/name = "newton-client-elcontracts"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-client-eth"/name = "newton-client-eth"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-client-fireblocks"/name = "newton-client-fireblocks"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-client-multichain"/name = "newton-client-multichain"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-crypto-bls"/name = "newton-crypto-bls"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-crypto-bn254"/name = "newton-crypto-bn254"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-crypto-ecdsa"/name = "newton-crypto-ecdsa"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-metrics"/name = "newton-metrics"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-metrics-collectors-economic"/name = "newton-metrics-collectors-economic"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-metrics-collectors-rpc-calls"/name = "newton-metrics-collectors-rpc-calls"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-services-avsregistry"/name = "newton-services-avsregistry"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-services-blsaggregation"/name = "newton-services-blsaggregation"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-services-multichain"/name = "newton-services-multichain"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-services-operatorsinfo"/name = "newton-services-operatorsinfo"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-signer"/name = "newton-signer"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-types"/name = "newton-types"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-utils"/name = "newton-utils"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-nodeapi"/name = "newton-nodeapi"/' "$cargo_file"
    sed -i.bak 's/^name = "eigen-testing-utils"/name = "newton-testing-utils"/' "$cargo_file"

    # Update dependency declarations (workspace = true)
    sed -i.bak 's/^eigensdk = { workspace/newton-eigensdk = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-cli = { workspace/newton-eigen-cli = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-common = { workspace/newton-common = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-client-avsregistry = { workspace/newton-client-avsregistry = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-client-elcontracts = { workspace/newton-client-elcontracts = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-client-eth = { workspace/newton-client-eth = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-client-fireblocks = { workspace/newton-client-fireblocks = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-client-multichain = { workspace/newton-client-multichain = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-crypto-bls = { workspace/newton-crypto-bls = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-crypto-bn254 = { workspace/newton-crypto-bn254 = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-crypto-ecdsa = { workspace/newton-crypto-ecdsa = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-metrics = { workspace/newton-metrics = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-metrics-collectors-economic = { workspace/newton-metrics-collectors-economic = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-metrics-collectors-rpc-calls = { workspace/newton-metrics-collectors-rpc-calls = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-services-avsregistry = { workspace/newton-services-avsregistry = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-services-blsaggregation = { workspace/newton-services-blsaggregation = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-services-multichain = { workspace/newton-services-multichain = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-services-operatorsinfo = { workspace/newton-services-operatorsinfo = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-signer = { workspace/newton-signer = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-types = { workspace/newton-types = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-utils = { workspace/newton-utils = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-nodeapi = { workspace/newton-nodeapi = { workspace/' "$cargo_file"
    sed -i.bak 's/^eigen-testing-utils = { workspace/newton-testing-utils = { workspace/' "$cargo_file"
done

echo "Step 4: Updating use statements in Rust source files..."
find . -name "*.rs" -type f | grep -v target | while read -r rs_file; do
    sed -i.bak 's/use eigen_common::/use newton_common::/g' "$rs_file"
    sed -i.bak 's/use eigen_client_avsregistry::/use newton_client_avsregistry::/g' "$rs_file"
    sed -i.bak 's/use eigen_client_elcontracts::/use newton_client_elcontracts::/g' "$rs_file"
    sed -i.bak 's/use eigen_client_eth::/use newton_client_eth::/g' "$rs_file"
    sed -i.bak 's/use eigen_client_fireblocks::/use newton_client_fireblocks::/g' "$rs_file"
    sed -i.bak 's/use eigen_client_multichain::/use newton_client_multichain::/g' "$rs_file"
    sed -i.bak 's/use eigen_crypto_bls::/use newton_crypto_bls::/g' "$rs_file"
    sed -i.bak 's/use eigen_crypto_bn254::/use newton_crypto_bn254::/g' "$rs_file"
    sed -i.bak 's/use eigen_crypto_ecdsa::/use newton_crypto_ecdsa::/g' "$rs_file"
    sed -i.bak 's/use eigen_metrics::/use newton_metrics::/g' "$rs_file"
    sed -i.bak 's/use eigen_metrics_collectors_economic::/use newton_metrics_collectors_economic::/g' "$rs_file"
    sed -i.bak 's/use eigen_metrics_collectors_rpc_calls::/use newton_metrics_collectors_rpc_calls::/g' "$rs_file"
    sed -i.bak 's/use eigen_services_avsregistry::/use newton_services_avsregistry::/g' "$rs_file"
    sed -i.bak 's/use eigen_services_blsaggregation::/use newton_services_blsaggregation::/g' "$rs_file"
    sed -i.bak 's/use eigen_services_multichain::/use newton_services_multichain::/g' "$rs_file"
    sed -i.bak 's/use eigen_services_operatorsinfo::/use newton_services_operatorsinfo::/g' "$rs_file"
    sed -i.bak 's/use eigen_signer::/use newton_signer::/g' "$rs_file"
    sed -i.bak 's/use eigen_types::/use newton_types::/g' "$rs_file"
    sed -i.bak 's/use eigen_utils::/use newton_utils::/g' "$rs_file"
    sed -i.bak 's/use eigen_nodeapi::/use newton_nodeapi::/g' "$rs_file"
    sed -i.bak 's/use eigen_testing_utils::/use newton_testing_utils::/g' "$rs_file"
    sed -i.bak 's/extern crate eigen_/extern crate newton_/g' "$rs_file"
done

echo "Step 5: Cleaning up backup files..."
find . -name "*.bak" -type f -delete

echo ""
echo "✓ Rename complete!"
echo ""
echo "Verification:"
echo "-------------"
echo "Package names (first 10):"
grep 'name = "newton' $(find . -name Cargo.toml -type f | grep -v target) 2>/dev/null | head -10
echo ""
echo "Use statements (first 5):"
grep -r 'use newton_' --include='*.rs' crates/ 2>/dev/null | head -5
