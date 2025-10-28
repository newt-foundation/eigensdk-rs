///Module containing a contract's types and functions.
/**

```solidity
library BeaconChainProofs {
    struct BalanceContainerProof { bytes32 balanceContainerRoot; bytes proof; }
    struct BalanceProof { bytes32 pubkeyHash; bytes32 balanceRoot; bytes proof; }
    struct StateRootProof { bytes32 beaconStateRoot; bytes proof; }
    struct ValidatorProof { bytes32[] validatorFields; bytes proof; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BeaconChainProofs {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct BalanceContainerProof { bytes32 balanceContainerRoot; bytes proof; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BalanceContainerProof {
        #[allow(missing_docs)]
        pub balanceContainerRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub proof: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BalanceContainerProof> for UnderlyingRustTuple<'_> {
            fn from(value: BalanceContainerProof) -> Self {
                (value.balanceContainerRoot, value.proof)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BalanceContainerProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    balanceContainerRoot: tuple.0,
                    proof: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BalanceContainerProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BalanceContainerProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceContainerRoot),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.proof,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for BalanceContainerProof {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for BalanceContainerProof {
            const NAME: &'static str = "BalanceContainerProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BalanceContainerProof(bytes32 balanceContainerRoot,bytes proof)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.balanceContainerRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proof,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BalanceContainerProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.balanceContainerRoot,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proof,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.balanceContainerRoot,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proof,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct BalanceProof { bytes32 pubkeyHash; bytes32 balanceRoot; bytes proof; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BalanceProof {
        #[allow(missing_docs)]
        pub pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub balanceRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub proof: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BalanceProof> for UnderlyingRustTuple<'_> {
            fn from(value: BalanceProof) -> Self {
                (value.pubkeyHash, value.balanceRoot, value.proof)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BalanceProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    pubkeyHash: tuple.0,
                    balanceRoot: tuple.1,
                    proof: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BalanceProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BalanceProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.pubkeyHash),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceRoot),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.proof,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for BalanceProof {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for BalanceProof {
            const NAME: &'static str = "BalanceProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BalanceProof(bytes32 pubkeyHash,bytes32 balanceRoot,bytes proof)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.pubkeyHash)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.balanceRoot)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proof,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BalanceProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pubkeyHash,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.balanceRoot,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proof,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pubkeyHash,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.balanceRoot,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proof,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct StateRootProof { bytes32 beaconStateRoot; bytes proof; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StateRootProof {
        #[allow(missing_docs)]
        pub beaconStateRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub proof: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StateRootProof> for UnderlyingRustTuple<'_> {
            fn from(value: StateRootProof) -> Self {
                (value.beaconStateRoot, value.proof)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StateRootProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    beaconStateRoot: tuple.0,
                    proof: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StateRootProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StateRootProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconStateRoot),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.proof,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StateRootProof {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StateRootProof {
            const NAME: &'static str = "StateRootProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StateRootProof(bytes32 beaconStateRoot,bytes proof)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.beaconStateRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proof,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StateRootProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.beaconStateRoot,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proof,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.beaconStateRoot,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proof,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct ValidatorProof { bytes32[] validatorFields; bytes proof; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorProof {
        #[allow(missing_docs)]
        pub validatorFields:
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        #[allow(missing_docs)]
        pub proof: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ValidatorProof> for UnderlyingRustTuple<'_> {
            fn from(value: ValidatorProof) -> Self {
                (value.validatorFields, value.proof)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidatorProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    validatorFields: tuple.0,
                    proof: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ValidatorProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ValidatorProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorFields),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.proof,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ValidatorProof {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ValidatorProof {
            const NAME: &'static str = "ValidatorProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ValidatorProof(bytes32[] validatorFields,bytes proof)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.validatorFields,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proof,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ValidatorProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.validatorFields,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proof,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.validatorFields,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proof,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BeaconChainProofs`](self) contract instance.

    See the [wrapper's documentation](`BeaconChainProofsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BeaconChainProofsInstance<P, N> {
        BeaconChainProofsInstance::<P, N>::new(address, provider)
    }
    /**A [`BeaconChainProofs`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`BeaconChainProofs`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BeaconChainProofsInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for BeaconChainProofsInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BeaconChainProofsInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BeaconChainProofsInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`BeaconChainProofs`](self) contract instance.

        See the [wrapper's documentation](`BeaconChainProofsInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> BeaconChainProofsInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BeaconChainProofsInstance<P, N> {
            BeaconChainProofsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BeaconChainProofsInstance<P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BeaconChainProofsInstance<P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library IEigenPodTypes {
    type VALIDATOR_STATUS is uint8;
    struct Checkpoint { bytes32 beaconBlockRoot; uint24 proofsRemaining; uint64 podBalanceGwei; int64 balanceDeltasGwei; uint64 prevBeaconBalanceGwei; }
    struct ConsolidationRequest { bytes srcPubkey; bytes targetPubkey; }
    struct ValidatorInfo { uint64 validatorIndex; uint64 restakedBalanceGwei; uint64 lastCheckpointedAt; VALIDATOR_STATUS status; }
    struct WithdrawalRequest { bytes pubkey; uint64 amountGwei; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IEigenPodTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VALIDATOR_STATUS(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<VALIDATOR_STATUS> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl VALIDATOR_STATUS {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from_underlying(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into_underlying(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl From<u8> for VALIDATOR_STATUS {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<VALIDATOR_STATUS> for u8 {
            fn from(value: VALIDATOR_STATUS) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for VALIDATOR_STATUS {
            type RustType = u8;
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for VALIDATOR_STATUS {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct Checkpoint { bytes32 beaconBlockRoot; uint24 proofsRemaining; uint64 podBalanceGwei; int64 balanceDeltasGwei; uint64 prevBeaconBalanceGwei; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Checkpoint {
        #[allow(missing_docs)]
        pub beaconBlockRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub proofsRemaining: alloy::sol_types::private::primitives::aliases::U24,
        #[allow(missing_docs)]
        pub podBalanceGwei: u64,
        #[allow(missing_docs)]
        pub balanceDeltasGwei: i64,
        #[allow(missing_docs)]
        pub prevBeaconBalanceGwei: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<24>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Int<64>,
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U24,
            u64,
            i64,
            u64,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Checkpoint> for UnderlyingRustTuple<'_> {
            fn from(value: Checkpoint) -> Self {
                (
                    value.beaconBlockRoot,
                    value.proofsRemaining,
                    value.podBalanceGwei,
                    value.balanceDeltasGwei,
                    value.prevBeaconBalanceGwei,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Checkpoint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    beaconBlockRoot: tuple.0,
                    proofsRemaining: tuple.1,
                    podBalanceGwei: tuple.2,
                    balanceDeltasGwei: tuple.3,
                    prevBeaconBalanceGwei: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Checkpoint {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Checkpoint {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconBlockRoot),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.proofsRemaining),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.podBalanceGwei),
                    <alloy::sol_types::sol_data::Int<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceDeltasGwei),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.prevBeaconBalanceGwei),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Checkpoint {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Checkpoint {
            const NAME: &'static str = "Checkpoint";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Checkpoint(bytes32 beaconBlockRoot,uint24 proofsRemaining,uint64 podBalanceGwei,int64 balanceDeltasGwei,uint64 prevBeaconBalanceGwei)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.beaconBlockRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proofsRemaining,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.podBalanceGwei,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.balanceDeltasGwei,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.prevBeaconBalanceGwei,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Checkpoint {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.beaconBlockRoot,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proofsRemaining,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.podBalanceGwei,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.balanceDeltasGwei,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.prevBeaconBalanceGwei,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.beaconBlockRoot,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proofsRemaining,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.podBalanceGwei,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.balanceDeltasGwei,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.prevBeaconBalanceGwei,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct ConsolidationRequest { bytes srcPubkey; bytes targetPubkey; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ConsolidationRequest {
        #[allow(missing_docs)]
        pub srcPubkey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub targetPubkey: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ConsolidationRequest> for UnderlyingRustTuple<'_> {
            fn from(value: ConsolidationRequest) -> Self {
                (value.srcPubkey, value.targetPubkey)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ConsolidationRequest {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    srcPubkey: tuple.0,
                    targetPubkey: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ConsolidationRequest {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ConsolidationRequest {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.srcPubkey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.targetPubkey,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ConsolidationRequest {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ConsolidationRequest {
            const NAME: &'static str = "ConsolidationRequest";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ConsolidationRequest(bytes srcPubkey,bytes targetPubkey)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.srcPubkey,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.targetPubkey,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ConsolidationRequest {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.srcPubkey,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.targetPubkey,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.srcPubkey,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.targetPubkey,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct ValidatorInfo { uint64 validatorIndex; uint64 restakedBalanceGwei; uint64 lastCheckpointedAt; VALIDATOR_STATUS status; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorInfo {
        #[allow(missing_docs)]
        pub validatorIndex: u64,
        #[allow(missing_docs)]
        pub restakedBalanceGwei: u64,
        #[allow(missing_docs)]
        pub lastCheckpointedAt: u64,
        #[allow(missing_docs)]
        pub status: <VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            VALIDATOR_STATUS,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            u64,
            u64,
            <VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ValidatorInfo> for UnderlyingRustTuple<'_> {
            fn from(value: ValidatorInfo) -> Self {
                (
                    value.validatorIndex,
                    value.restakedBalanceGwei,
                    value.lastCheckpointedAt,
                    value.status,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidatorInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    validatorIndex: tuple.0,
                    restakedBalanceGwei: tuple.1,
                    lastCheckpointedAt: tuple.2,
                    status: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ValidatorInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ValidatorInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.restakedBalanceGwei,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.lastCheckpointedAt,
                    ),
                    <VALIDATOR_STATUS as alloy_sol_types::SolType>::tokenize(&self.status),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ValidatorInfo {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ValidatorInfo {
            const NAME: &'static str = "ValidatorInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ValidatorInfo(uint64 validatorIndex,uint64 restakedBalanceGwei,uint64 lastCheckpointedAt,uint8 status)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.validatorIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.restakedBalanceGwei,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.lastCheckpointedAt,
                        )
                        .0,
                    <VALIDATOR_STATUS as alloy_sol_types::SolType>::eip712_data_word(
                            &self.status,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ValidatorInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.validatorIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.restakedBalanceGwei,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.lastCheckpointedAt,
                    )
                    + <VALIDATOR_STATUS as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.status,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.validatorIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.restakedBalanceGwei,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.lastCheckpointedAt,
                    out,
                );
                <VALIDATOR_STATUS as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.status,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct WithdrawalRequest { bytes pubkey; uint64 amountGwei; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalRequest {
        #[allow(missing_docs)]
        pub pubkey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub amountGwei: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes, u64);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<WithdrawalRequest> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalRequest) -> Self {
                (value.pubkey, value.amountGwei)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawalRequest {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    pubkey: tuple.0,
                    amountGwei: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for WithdrawalRequest {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for WithdrawalRequest {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.pubkey,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.amountGwei,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_encode_packed_to(
                    &tuple, out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple =
                    <UnderlyingRustTuple<'_> as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::abi_packed_encoded_size(
                    &tuple,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for WithdrawalRequest {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> =
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for WithdrawalRequest {
            const NAME: &'static str = "WithdrawalRequest";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "WithdrawalRequest(bytes pubkey,uint64 amountGwei)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pubkey,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amountGwei)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for WithdrawalRequest {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pubkey,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountGwei,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pubkey,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountGwei,
                    out,
                );
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IEigenPodTypes`](self) contract instance.

    See the [wrapper's documentation](`IEigenPodTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IEigenPodTypesInstance<P, N> {
        IEigenPodTypesInstance::<P, N>::new(address, provider)
    }
    /**A [`IEigenPodTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IEigenPodTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IEigenPodTypesInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IEigenPodTypesInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IEigenPodTypesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IEigenPodTypesInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`IEigenPodTypes`](self) contract instance.

        See the [wrapper's documentation](`IEigenPodTypesInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> IEigenPodTypesInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IEigenPodTypesInstance<P, N> {
            IEigenPodTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IEigenPodTypesInstance<P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IEigenPodTypesInstance<P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library BeaconChainProofs {
    struct BalanceContainerProof {
        bytes32 balanceContainerRoot;
        bytes proof;
    }
    struct BalanceProof {
        bytes32 pubkeyHash;
        bytes32 balanceRoot;
        bytes proof;
    }
    struct StateRootProof {
        bytes32 beaconStateRoot;
        bytes proof;
    }
    struct ValidatorProof {
        bytes32[] validatorFields;
        bytes proof;
    }
}

library IEigenPodTypes {
    type VALIDATOR_STATUS is uint8;
    struct Checkpoint {
        bytes32 beaconBlockRoot;
        uint24 proofsRemaining;
        uint64 podBalanceGwei;
        int64 balanceDeltasGwei;
        uint64 prevBeaconBalanceGwei;
    }
    struct ConsolidationRequest {
        bytes srcPubkey;
        bytes targetPubkey;
    }
    struct ValidatorInfo {
        uint64 validatorIndex;
        uint64 restakedBalanceGwei;
        uint64 lastCheckpointedAt;
        VALIDATOR_STATUS status;
    }
    struct WithdrawalRequest {
        bytes pubkey;
        uint64 amountGwei;
    }
}

interface EigenPod {
    error BeaconTimestampBeforeLatestCheckpoint();
    error BeaconTimestampTooFarInPast();
    error CannotCheckpointTwiceInSingleBlock();
    error CheckpointAlreadyActive();
    error CredentialsAlreadyVerified();
    error CurrentlyPaused();
    error EmptyRoot();
    error FeeQueryFailed();
    error ForkTimestampZero();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InsufficientFunds();
    error InsufficientWithdrawableBalance();
    error InvalidEIP4788Response();
    error InvalidIndex();
    error InvalidProof();
    error InvalidProofLength();
    error InvalidPubKeyLength();
    error InvalidShortString();
    error InvalidValidatorFieldsLength();
    error LeavesNotPowerOfTwo();
    error MsgValueNot32ETH();
    error NoActiveCheckpoint();
    error NoBalanceToCheckpoint();
    error NotEnoughLeaves();
    error OnlyEigenPodManager();
    error OnlyEigenPodOwner();
    error OnlyEigenPodOwnerOrProofSubmitter();
    error PredeployFailed();
    error StringTooLong(string str);
    error TimestampOutOfRange();
    error ValidatorInactiveOnBeaconChain();
    error ValidatorIsExitingBeaconChain();
    error ValidatorNotActiveInPod();
    error ValidatorNotSlashedOnBeaconChain();
    error WithdrawalCredentialsNotForEigenPod();

    event CheckpointCreated(uint64 indexed checkpointTimestamp, bytes32 indexed beaconBlockRoot, uint256 validatorCount);
    event CheckpointFinalized(uint64 indexed checkpointTimestamp, int256 totalShareDeltaWei);
    event ConsolidationRequested(bytes32 indexed sourcePubkeyHash, bytes32 indexed targetPubkeyHash);
    event EigenPodStaked(bytes32 pubkeyHash);
    event ExitRequested(bytes32 indexed validatorPubkeyHash);
    event Initialized(uint8 version);
    event NonBeaconChainETHReceived(uint256 amountReceived);
    event ProofSubmitterUpdated(address prevProofSubmitter, address newProofSubmitter);
    event RestakedBeaconChainETHWithdrawn(address indexed recipient, uint256 amount);
    event SwitchToCompoundingRequested(bytes32 indexed validatorPubkeyHash);
    event ValidatorBalanceUpdated(bytes32 pubkeyHash, uint64 balanceTimestamp, uint64 newValidatorBalanceGwei);
    event ValidatorCheckpointed(uint64 indexed checkpointTimestamp, bytes32 indexed pubkeyHash);
    event ValidatorRestaked(bytes32 pubkeyHash);
    event ValidatorWithdrawn(uint64 indexed checkpointTimestamp, bytes32 indexed pubkeyHash);
    event WithdrawalRequested(bytes32 indexed validatorPubkeyHash, uint64 withdrawalAmountGwei);

    constructor(address _ethPOS, address _eigenPodManager, string _version);

    receive() external payable;

    function activeValidatorCount() external view returns (uint256);
    function checkpointBalanceExitedGwei(uint64) external view returns (uint64);
    function currentCheckpoint() external view returns (IEigenPodTypes.Checkpoint memory);
    function currentCheckpointTimestamp() external view returns (uint64);
    function eigenPodManager() external view returns (address);
    function ethPOS() external view returns (address);
    function getConsolidationRequestFee() external view returns (uint256);
    function getParentBlockRoot(uint64 timestamp) external view returns (bytes32);
    function getWithdrawalRequestFee() external view returns (uint256);
    function initialize(address _podOwner) external;
    function lastCheckpointTimestamp() external view returns (uint64);
    function podOwner() external view returns (address);
    function proofSubmitter() external view returns (address);
    function recoverTokens(address[] memory tokenList, uint256[] memory amountsToWithdraw, address recipient) external;
    function requestConsolidation(IEigenPodTypes.ConsolidationRequest[] memory requests) external payable;
    function requestWithdrawal(IEigenPodTypes.WithdrawalRequest[] memory requests) external payable;
    function setProofSubmitter(address newProofSubmitter) external;
    function stake(bytes memory pubkey, bytes memory signature, bytes32 depositDataRoot) external payable;
    function startCheckpoint(bool revertIfNoBalance) external;
    function validatorPubkeyHashToInfo(bytes32 validatorPubkeyHash) external view returns (IEigenPodTypes.ValidatorInfo memory);
    function validatorPubkeyToInfo(bytes memory validatorPubkey) external view returns (IEigenPodTypes.ValidatorInfo memory);
    function validatorStatus(bytes memory validatorPubkey) external view returns (IEigenPodTypes.VALIDATOR_STATUS);
    function validatorStatus(bytes32 pubkeyHash) external view returns (IEigenPodTypes.VALIDATOR_STATUS);
    function verifyCheckpointProofs(BeaconChainProofs.BalanceContainerProof memory balanceContainerProof, BeaconChainProofs.BalanceProof[] memory proofs) external;
    function verifyStaleBalance(uint64 beaconTimestamp, BeaconChainProofs.StateRootProof memory stateRootProof, BeaconChainProofs.ValidatorProof memory proof) external;
    function verifyWithdrawalCredentials(uint64 beaconTimestamp, BeaconChainProofs.StateRootProof memory stateRootProof, uint40[] memory validatorIndices, bytes[] memory validatorFieldsProofs, bytes32[][] memory validatorFields) external;
    function version() external view returns (string memory);
    function withdrawRestakedBeaconChainETH(address recipient, uint256 amountWei) external;
    function withdrawableRestakedExecutionLayerGwei() external view returns (uint64);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_ethPOS",
        "type": "address",
        "internalType": "contract IETHPOSDeposit"
      },
      {
        "name": "_eigenPodManager",
        "type": "address",
        "internalType": "contract IEigenPodManager"
      },
      {
        "name": "_version",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "activeValidatorCount",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkpointBalanceExitedGwei",
    "inputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "currentCheckpoint",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IEigenPodTypes.Checkpoint",
        "components": [
          {
            "name": "beaconBlockRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "proofsRemaining",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "podBalanceGwei",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "balanceDeltasGwei",
            "type": "int64",
            "internalType": "int64"
          },
          {
            "name": "prevBeaconBalanceGwei",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "currentCheckpointTimestamp",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eigenPodManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IEigenPodManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ethPOS",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IETHPOSDeposit"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getConsolidationRequestFee",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getParentBlockRoot",
    "inputs": [
      {
        "name": "timestamp",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getWithdrawalRequestFee",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_podOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "lastCheckpointTimestamp",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "podOwner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "proofSubmitter",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "recoverTokens",
    "inputs": [
      {
        "name": "tokenList",
        "type": "address[]",
        "internalType": "contract IERC20[]"
      },
      {
        "name": "amountsToWithdraw",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "requestConsolidation",
    "inputs": [
      {
        "name": "requests",
        "type": "tuple[]",
        "internalType": "struct IEigenPodTypes.ConsolidationRequest[]",
        "components": [
          {
            "name": "srcPubkey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "targetPubkey",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "requestWithdrawal",
    "inputs": [
      {
        "name": "requests",
        "type": "tuple[]",
        "internalType": "struct IEigenPodTypes.WithdrawalRequest[]",
        "components": [
          {
            "name": "pubkey",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "amountGwei",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "setProofSubmitter",
    "inputs": [
      {
        "name": "newProofSubmitter",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "stake",
    "inputs": [
      {
        "name": "pubkey",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "depositDataRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "startCheckpoint",
    "inputs": [
      {
        "name": "revertIfNoBalance",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "validatorPubkeyHashToInfo",
    "inputs": [
      {
        "name": "validatorPubkeyHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IEigenPodTypes.ValidatorInfo",
        "components": [
          {
            "name": "validatorIndex",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "restakedBalanceGwei",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "lastCheckpointedAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum IEigenPodTypes.VALIDATOR_STATUS"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validatorPubkeyToInfo",
    "inputs": [
      {
        "name": "validatorPubkey",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IEigenPodTypes.ValidatorInfo",
        "components": [
          {
            "name": "validatorIndex",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "restakedBalanceGwei",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "lastCheckpointedAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum IEigenPodTypes.VALIDATOR_STATUS"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validatorStatus",
    "inputs": [
      {
        "name": "validatorPubkey",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IEigenPodTypes.VALIDATOR_STATUS"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validatorStatus",
    "inputs": [
      {
        "name": "pubkeyHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IEigenPodTypes.VALIDATOR_STATUS"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "verifyCheckpointProofs",
    "inputs": [
      {
        "name": "balanceContainerProof",
        "type": "tuple",
        "internalType": "struct BeaconChainProofs.BalanceContainerProof",
        "components": [
          {
            "name": "balanceContainerRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "proof",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "proofs",
        "type": "tuple[]",
        "internalType": "struct BeaconChainProofs.BalanceProof[]",
        "components": [
          {
            "name": "pubkeyHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "balanceRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "proof",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifyStaleBalance",
    "inputs": [
      {
        "name": "beaconTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "stateRootProof",
        "type": "tuple",
        "internalType": "struct BeaconChainProofs.StateRootProof",
        "components": [
          {
            "name": "beaconStateRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "proof",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "proof",
        "type": "tuple",
        "internalType": "struct BeaconChainProofs.ValidatorProof",
        "components": [
          {
            "name": "validatorFields",
            "type": "bytes32[]",
            "internalType": "bytes32[]"
          },
          {
            "name": "proof",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifyWithdrawalCredentials",
    "inputs": [
      {
        "name": "beaconTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "stateRootProof",
        "type": "tuple",
        "internalType": "struct BeaconChainProofs.StateRootProof",
        "components": [
          {
            "name": "beaconStateRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "proof",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "validatorIndices",
        "type": "uint40[]",
        "internalType": "uint40[]"
      },
      {
        "name": "validatorFieldsProofs",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "validatorFields",
        "type": "bytes32[][]",
        "internalType": "bytes32[][]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "version",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "withdrawRestakedBeaconChainETH",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amountWei",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawableRestakedExecutionLayerGwei",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "CheckpointCreated",
    "inputs": [
      {
        "name": "checkpointTimestamp",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "beaconBlockRoot",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "validatorCount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CheckpointFinalized",
    "inputs": [
      {
        "name": "checkpointTimestamp",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "totalShareDeltaWei",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ConsolidationRequested",
    "inputs": [
      {
        "name": "sourcePubkeyHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "targetPubkeyHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EigenPodStaked",
    "inputs": [
      {
        "name": "pubkeyHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ExitRequested",
    "inputs": [
      {
        "name": "validatorPubkeyHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NonBeaconChainETHReceived",
    "inputs": [
      {
        "name": "amountReceived",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProofSubmitterUpdated",
    "inputs": [
      {
        "name": "prevProofSubmitter",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newProofSubmitter",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RestakedBeaconChainETHWithdrawn",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SwitchToCompoundingRequested",
    "inputs": [
      {
        "name": "validatorPubkeyHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ValidatorBalanceUpdated",
    "inputs": [
      {
        "name": "pubkeyHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "balanceTimestamp",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "newValidatorBalanceGwei",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ValidatorCheckpointed",
    "inputs": [
      {
        "name": "checkpointTimestamp",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "pubkeyHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ValidatorRestaked",
    "inputs": [
      {
        "name": "pubkeyHash",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ValidatorWithdrawn",
    "inputs": [
      {
        "name": "checkpointTimestamp",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "pubkeyHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "WithdrawalRequested",
    "inputs": [
      {
        "name": "validatorPubkeyHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "withdrawalAmountGwei",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "BeaconTimestampBeforeLatestCheckpoint",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BeaconTimestampTooFarInPast",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotCheckpointTwiceInSingleBlock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CheckpointAlreadyActive",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CredentialsAlreadyVerified",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyRoot",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeQueryFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ForkTimestampZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputAddressZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputArrayLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientFunds",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientWithdrawableBalance",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidEIP4788Response",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidIndex",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidProofLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPubKeyLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidShortString",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidValidatorFieldsLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LeavesNotPowerOfTwo",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MsgValueNot32ETH",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoActiveCheckpoint",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoBalanceToCheckpoint",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotEnoughLeaves",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyEigenPodManager",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyEigenPodOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyEigenPodOwnerOrProofSubmitter",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PredeployFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StringTooLong",
    "inputs": [
      {
        "name": "str",
        "type": "string",
        "internalType": "string"
      }
    ]
  },
  {
    "type": "error",
    "name": "TimestampOutOfRange",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ValidatorInactiveOnBeaconChain",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ValidatorIsExitingBeaconChain",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ValidatorNotActiveInPod",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ValidatorNotSlashedOnBeaconChain",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawalCredentialsNotForEigenPod",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod EigenPod {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e060405234801561000f575f5ffd5b5060405161495a38038061495a83398101604081905261002e91610188565b806100388161005f565b608052506001600160a01b0380841660a052821660c0526100576100a5565b5050506102b9565b5f5f829050601f81511115610092578260405163305a27a960e01b8152600401610089919061025e565b60405180910390fd5b805161009d82610293565b179392505050565b5f54610100900460ff161561010c5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608401610089565b5f5460ff9081161461015b575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b0381168114610171575f5ffd5b50565b634e487b7160e01b5f52604160045260245ffd5b5f5f5f6060848603121561019a575f5ffd5b83516101a58161015d565b60208501519093506101b68161015d565b60408501519092506001600160401b038111156101d1575f5ffd5b8401601f810186136101e1575f5ffd5b80516001600160401b038111156101fa576101fa610174565b604051601f8201601f19908116603f011681016001600160401b038111828210171561022857610228610174565b60405281815282820160200188101561023f575f5ffd5b8160208401602083015e5f602083830101528093505050509250925092565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b805160208083015191908110156102b3575f198160200360031b1b821691505b50919050565b60805160a05160c0516146156103455f395f8181610329015281816106d40152818161077c01528181610a2a01528181610deb01528181610fe5015281816110f0015281816115f1015281816116980152818161192101528181611c6f01528181611da301528181612279015261318301525f818161057a015261170101525f61105101526146155ff3fe6080604052600436106101bd575f3560e01c80636691954e116100f2578063b522538a11610092578063d06d558711610062578063d06d55871461063f578063dda3346c1461065e578063ee94d67c1461067d578063f074ba621461069c575f5ffd5b8063b522538a146105ce578063c44e30dc146105ed578063c490744214610601578063c4d66de814610620575f5ffd5b80637439841f116100cd5780637439841f1461053457806374cdd7981461056957806388676cad1461059c5780639b4e4634146105bb575f5ffd5b80636691954e146104d65780636c0d2d5a146104e95780636fcd0e5314610508575f5ffd5b806342ecff2a1161015d57806352396a591161013857806352396a591461043657806354fd4d501461046a578063587533571461048b57806358eaee79146104aa575f5ffd5b806342ecff2a146102f25780634665bcda1461031857806347d283721461034b575f5ffd5b80632340e8d3116101985780632340e8d31461027a5780633474aa161461028f5780633f5fa57a146102c05780633f65cf19146102d3575f5ffd5b8063039157d2146101fb5780630b18ff661461021c5780631e51553314610258575f5ffd5b366101f7576040513481527f6fdd3dbdb173299608c0aa9f368735857c8842b581f8389238bf05bd04b3bf499060200160405180910390a1005b5f5ffd5b348015610206575f5ffd5b5061021a610215366004613abc565b6106bb565b005b348015610227575f5ffd5b5060335461023b906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b348015610263575f5ffd5b5061026c6109f0565b60405190815260200161024f565b348015610285575f5ffd5b5061026c60395481565b34801561029a575f5ffd5b506034546001600160401b03165b6040516001600160401b03909116815260200161024f565b61021a6102ce366004613b77565b610a11565b3480156102de575f5ffd5b5061021a6102ed366004613bb5565b610d92565b3480156102fd575f5ffd5b50603a546102a890600160401b90046001600160401b031681565b348015610323575f5ffd5b5061023b7f000000000000000000000000000000000000000000000000000000000000000081565b348015610356575f5ffd5b506103db6040805160a0810182525f80825260208201819052918101829052606081018290526080810191909152506040805160a081018252603c548152603d5462ffffff811660208301526001600160401b0363010000008204811693830193909352600160581b810460070b6060830152600160981b9004909116608082015290565b60405161024f91905f60a0820190508251825262ffffff60208401511660208301526001600160401b036040840151166040830152606083015160070b60608301526001600160401b03608084015116608083015292915050565b348015610441575f5ffd5b506102a8610450366004613c8c565b603b6020525f90815260409020546001600160401b031681565b348015610475575f5ffd5b5061047e61104a565b60405161024f9190613cd5565b348015610496575f5ffd5b50603e5461023b906001600160a01b031681565b3480156104b5575f5ffd5b506104c96104c4366004613d24565b611075565b60405161024f9190613d8a565b61021a6104e4366004613b77565b6110d7565b3480156104f4575f5ffd5b5061026c610503366004613c8c565b6113df565b348015610513575f5ffd5b50610527610522366004613d98565b6114ed565b60405161024f9190613daf565b34801561053f575f5ffd5b506104c961054e366004613d98565b5f90815260366020526040902054600160c01b900460ff1690565b348015610574575f5ffd5b5061023b7f000000000000000000000000000000000000000000000000000000000000000081565b3480156105a7575f5ffd5b5061021a6105b6366004613e0f565b611598565b61021a6105c9366004613e2a565b61168d565b3480156105d9575f5ffd5b506105276105e8366004613d24565b61180b565b3480156105f8575f5ffd5b5061026c6118fa565b34801561060c575f5ffd5b5061021a61061b366004613ebf565b611916565b34801561062b575f5ffd5b5061021a61063a366004613ee9565b611a4d565b34801561064a575f5ffd5b5061021a610659366004613ee9565b611b97565b348015610669575f5ffd5b5061021a610678366004613fd4565b611c2b565b348015610688575f5ffd5b50603a546102a8906001600160401b031681565b3480156106a7575f5ffd5b5061021a6106b63660046140a6565b611d8a565b604051635ac86ab760e01b8152600660048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610721573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610745919061410d565b156107635760405163840a48d560e01b815260040160405180910390fd5b604051635ac86ab760e01b8152600860048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156107c9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107ed919061410d565b1561080b5760405163840a48d560e01b815260040160405180910390fd5b5f61084f6108198580614128565b808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061218692505050565b5f818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff1660028111156108bd576108bd613d56565b60028111156108ce576108ce613d56565b81525050905080604001516001600160401b0316876001600160401b03161161090a576040516337e07ffd60e01b815260040160405180910390fd5b60018160600151600281111561092257610922613d56565b146109405760405163d49e19a760e01b815260040160405180910390fd5b61098361094d8680614128565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152506121a892505050565b6109a05760405163161ce5ed60e31b815260040160405180910390fd5b6109b26109ac886113df565b876121d0565b6109de6109be88612275565b87356109ca8880614128565b6109d760208b018b61416d565b875161234c565b6109e75f612480565b50505050505050565b5f610a0c71bbddc7ce488642fb579f8b00f3a590007251612600565b905090565b604051635ac86ab760e01b8152600a60048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610a77573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a9b919061410d565b15610ab95760405163840a48d560e01b815260040160405180910390fd5b6033546001600160a01b0316331480610adc5750603e546001600160a01b031633145b610af95760405163427a777960e01b815260040160405180910390fd5b5f610b026118fa565b90505f610b0f84836141c3565b905080341015610b325760405163356680b760e01b815260040160405180910390fd5b5f610b3d82346141da565b90505f5b85811015610d795736878783818110610b5c57610b5c6141ed565b9050602002810190610b6e9190614201565b90505f610bb7610b7e838061416d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061268692505050565b905060015f82815260366020526040902054600160c01b900460ff166002811115610be457610be4613d56565b14610c025760405163d49e19a760e01b815260040160405180910390fd5b5f610c0d838061416d565b610c1d6040860160208701613c8c565b604051602001610c2f9392919061421f565b60405160208183030381529060405290505f710961ef480eb55e80d19ad83579a64c0070026001600160a01b03168883604051610c6c9190614257565b5f6040518083038185875af1925050503d805f8114610ca6576040519150601f19603f3d011682016040523d82523d5f602084013e610cab565b606091505b5050905080610ccd5760405163fc52d48360e01b815260040160405180910390fd5b610cdd6040850160208601613c8c565b6001600160401b03165f03610d1b5760405183907f60d8ca014d4765a2b8b389e25714cb1cef83b574222911a01d90c1bd69d2d320905f90a2610d69565b827f8b2737bb64ab2f2dc09552dfa1c250399e6a42c7ea9f0e1c658f5d65d708ec05610d4d6040870160208801613c8c565b6040516001600160401b03909116815260200160405180910390a25b505060019092019150610b419050565b508015610d8a57610d8a3382612717565b505050505050565b6033546001600160a01b0316331480610db55750603e546001600160a01b031633145b610dd25760405163427a777960e01b815260040160405180910390fd5b604051635ac86ab760e01b8152600260048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610e38573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e5c919061410d565b15610e7a5760405163840a48d560e01b815260040160405180910390fd5b8584148015610e8857508382145b610ea5576040516343714afd60e01b815260040160405180910390fd5b603a546001600160401b03600160401b9091048116908a1611610edb576040516337e07ffd60e01b815260040160405180910390fd5b603a546001600160401b03908116908a1611610f0a576040516335e7f6b760e01b815260040160405180910390fd5b610f1c610f168a6113df565b896121d0565b5f805b87811015610fb557610fa18b8b358b8b85818110610f3f57610f3f6141ed565b9050602002016020810190610f549190614262565b8a8a86818110610f6657610f666141ed565b9050602002810190610f78919061416d565b8a8a88818110610f8a57610f8a6141ed565b9050602002810190610f9c9190614128565b61282c565b610fab9083614286565b9150600101610f1f565b5060335460405163a1ca780b60e01b81526001600160a01b0391821660048201525f6024820152604481018390527f00000000000000000000000000000000000000000000000000000000000000009091169063a1ca780b906064015f604051808303815f87803b158015611028575f5ffd5b505af115801561103a573d5f5f3e3d5ffd5b5050505050505050505050505050565b6060610a0c7f0000000000000000000000000000000000000000000000000000000000000000612d2d565b5f5f6110b584848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061268692505050565b5f90815260366020526040902054600160c01b900460ff169150505b92915050565b604051635ac86ab760e01b8152600960048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa15801561113d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611161919061410d565b1561117f5760405163840a48d560e01b815260040160405180910390fd5b6033546001600160a01b03163314806111a25750603e546001600160a01b031633145b6111bf5760405163427a777960e01b815260040160405180910390fd5b5f6111c86109f0565b90505f6111d584836141c3565b9050803410156111f85760405163356680b760e01b815260040160405180910390fd5b5f61120382346141da565b90505f5b85811015610d795736878783818110611222576112226141ed565b90506020028101906112349190614201565b90505f611244610b7e838061416d565b90505f611257610b7e602085018561416d565b905060015f82815260366020526040902054600160c01b900460ff16600281111561128457611284613d56565b146112a25760405163d49e19a760e01b815260040160405180910390fd5b5f6112ad848061416d565b6112ba602087018761416d565b6040516020016112cd9493929190614299565b60405160208183030381529060405290505f71bbddc7ce488642fb579f8b00f3a5900072516001600160a01b0316898360405161130a9190614257565b5f6040518083038185875af1925050503d805f8114611344576040519150601f19603f3d011682016040523d82523d5f602084013e611349565b606091505b505090508061136b5760405163fc52d48360e01b815260040160405180910390fd5b8284036113a15760405184907fc97b965b92ae7fd20095fe8eb7b99f81f95f8c4adffb22a19116d8eb2846b016905f90a26113ce565b604051839085907f42f9c9db2ca443e9ec62f4588bd0c9b241065c02c2a8001ac164ae1282dc7b94905f90a35b505060019093019250611207915050565b5f6113ed611fff600c6141c3565b6114006001600160401b038416426141da565b1061141e57604051637944e66d60e11b815260040160405180910390fd5b604080516001600160401b03841660208201525f918291720f3df6d732807ef1319fb7b8bb8522d0beac02910160408051601f198184030181529082905261146591614257565b5f60405180830381855afa9150503d805f811461149d576040519150601f19603f3d011682016040523d82523d5f602084013e6114a2565b606091505b50915091508180156114b457505f8151115b6114d15760405163558ad0a360e01b815260040160405180910390fd5b808060200190518101906114e591906142b8565b949350505050565b611514604080516080810182525f8082526020820181905291810182905290606082015290565b5f82815260366020908152604091829020825160808101845281546001600160401b038082168352600160401b8204811694830194909452600160801b810490931693810193909352906060830190600160c01b900460ff16600281111561157e5761157e613d56565b600281111561158f5761158f613d56565b90525092915050565b6033546001600160a01b03163314806115bb5750603e546001600160a01b031633145b6115d85760405163427a777960e01b815260040160405180910390fd5b604051635ac86ab760e01b8152600660048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa15801561163e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611662919061410d565b156116805760405163840a48d560e01b815260040160405180910390fd5b61168982612480565b5050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146116d657604051633213a66160e21b815260040160405180910390fd5b346801bc16d674ec800000146116ff5760405163049696b360e31b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663228951186801bc16d674ec8000008787611742612d6a565b8888886040518863ffffffff1660e01b8152600401611766969594939291906142f7565b5f604051808303818588803b15801561177d575f5ffd5b505af115801561178f573d5f5f3e3d5ffd5b50505050507fa01003766d3cd97cf2ade5429690bf5d206be7fb01ef9d3a0089ecf67bc112196117f386868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061268692505050565b60405190815260200160405180910390a15050505050565b611832604080516080810182525f8082526020820181905291810182905290606082015290565b60365f61187385858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061268692505050565b815260208082019290925260409081015f20815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b81049094169281019290925290916060830190600160c01b900460ff1660028111156118df576118df613d56565b60028111156118f0576118f0613d56565b9052509392505050565b5f610a0c710961ef480eb55e80d19ad83579a64c007002612600565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461195f57604051633213a66160e21b815260040160405180910390fd5b5f61196e633b9aca0083614359565b9050611987633b9aca006001600160401b0383166141c3565b6034549092506001600160401b0390811690821611156119ba576040516302c6f54760e21b815260040160405180910390fd5b603480548291905f906119d79084906001600160401b031661436c565b92506101000a8154816001600160401b0302191690836001600160401b03160217905550826001600160a01b03167f8947fd2ce07ef9cc302c4e8f0461015615d91ce851564839e91cc804c2f49d8e83604051611a3691815260200190565b60405180910390a2611a488383612717565b505050565b5f54610100900460ff1615808015611a6b57505f54600160ff909116105b80611a845750303b158015611a8457505f5460ff166001145b611aec5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff191660011790558015611b0d575f805461ff0019166101001790555b6001600160a01b038216611b34576040516339b190bb60e11b815260040160405180910390fd5b603380546001600160a01b0319166001600160a01b0384161790558015611689575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b6033546001600160a01b03163314611bc25760405163719f370360e11b815260040160405180910390fd5b603e54604080516001600160a01b03928316815291831660208301527ffb8129080a19d34dceac04ba253fc50304dc86c729bd63cdca4a969ad19a5eac910160405180910390a1603e80546001600160a01b0319166001600160a01b0392909216919091179055565b6033546001600160a01b03163314611c565760405163719f370360e11b815260040160405180910390fd5b604051635ac86ab760e01b8152600560048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015611cbc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ce0919061410d565b15611cfe5760405163840a48d560e01b815260040160405180910390fd5b8251845114611d20576040516343714afd60e01b815260040160405180910390fd5b5f5b8451811015611d8357611d7b83858381518110611d4157611d416141ed565b6020026020010151878481518110611d5b57611d5b6141ed565b60200260200101516001600160a01b0316612d9a9092919063ffffffff16565b600101611d22565b5050505050565b604051635ac86ab760e01b8152600760048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015611df0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e14919061410d565b15611e325760405163840a48d560e01b815260040160405180910390fd5b603a54600160401b90046001600160401b03165f819003611e6657604051631a544f4960e01b815260040160405180910390fd5b6040805160a081018252603c548152603d5462ffffff811660208301526001600160401b0363010000008204811693830193909352600160581b810460070b6060830152600160981b90049091166080820152611ecd611ec583612275565b825188612dec565b5f805b8581101561212d5736878783818110611eeb57611eeb6141ed565b9050602002810190611efd919061438b565b80355f908152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff166002811115611f6d57611f6d613d56565b6002811115611f7e57611f7e613d56565b9052509050600181606001516002811115611f9b57611f9b613d56565b14611fa7575050612125565b856001600160401b031681604001516001600160401b031610611fcb575050612125565b5f8080611fdb848a8f3588612ead565b60208b0180519396509194509250611ff28261439f565b62ffffff1690525060808801805184919061200e9083906143bc565b6001600160401b031690525060608801805183919061202e9083906143db565b60070b90525061203e81886143bc565b85355f908152603660209081526040918290208751815492890151938901516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b0319909516919092161792909217928316821781556060880151939a50879390929091839160ff60c01b1990911668ffffffffffffffffff60801b1990911617600160c01b8360028111156120e2576120e2613d56565b021790555050604051863591506001600160401b038b16907fe4866335761a51dcaff766448ab0af6064291ee5dc94e68492bb9cd757c1e350905f90a350505050505b600101611ed0565b506001600160401b038084165f908152603b6020526040812080548493919291612159918591166143bc565b92506101000a8154816001600160401b0302191690836001600160401b031602179055506109e782612fc3565b5f815f81518110612199576121996141ed565b60200260200101519050919050565b5f816003815181106121bc576121bc6141ed565b60200260200101515f5f1b14159050919050565b6121dc600360206141c3565b6121e9602083018361416d565b905014612209576040516313717da960e21b815260040160405180910390fd5b612258612219602083018361416d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250869250508435905060036131e3565b611689576040516309bde33960e01b815260040160405180910390fd5b5f5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316632704351a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156122d3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122f7919061440a565b9050806001600160401b03165f03612322576040516341a02cc960e01b815260040160405180910390fd5b806001600160401b0316836001600160401b03161115612343576001612345565b5f5b9392505050565b6008841461236d5760405163200591bd60e01b815260040160405180910390fd5b5f61237788613218565b90508061238660286001614286565b6123909190614286565b61239b9060206141c3565b83146123ba576040516313717da960e21b815260040160405180910390fd5b5f6123f68787808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061324092505050565b90505f64ffffffffff841661240d60286001614286565b600b901b17905061245786868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508d92508691508590506131e3565b612474576040516309bde33960e01b815260040160405180910390fd5b50505050505050505050565b603a54600160401b90046001600160401b0316156124b05760405162be9bc360e81b815260040160405180910390fd5b603a546001600160401b034281169116036124de576040516367db5b8b60e01b815260040160405180910390fd5b6034545f906001600160401b03166124fa633b9aca0047614359565b612504919061436c565b905081801561251a57506001600160401b038116155b15612538576040516332dea95960e21b815260040160405180910390fd5b5f6040518060a0016040528061254d426113df565b815260395462ffffff1660208201526001600160401b0380851660408301525f60608301819052608090920191909152603a805442909216600160401b026fffffffffffffffff00000000000000001990921691909117905590506125b181612fc3565b805160208083015160405162ffffff90911681526001600160401b034216917f575796133bbed337e5b39aa49a30dc2556a91e0c6c2af4b7b886ae77ebef1076910160405180910390a3505050565b5f5f5f836001600160a01b03166040515f60405180830381855afa9150503d805f8114612648576040519150601f19603f3d011682016040523d82523d5f602084013e61264d565b606091505b5091509150818015612660575080516020145b61267d5760405163c90158af60e01b815260040160405180910390fd5b6114e581614425565b5f81516030146126a957604051634f88323960e11b815260040160405180910390fd5b6040516002906126bf9084905f90602001614448565b60408051601f19818403018152908290526126d991614257565b602060405180830381855afa1580156126f4573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906110d191906142b8565b804710156127675760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e63650000006044820152606401611ae3565b5f826001600160a01b0316826040515f6040518083038185875af1925050503d805f81146127b0576040519150601f19603f3d011682016040523d82523d5f602084013e6127b5565b606091505b5050905080611a485760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d617920686176652072657665727465640000000000006064820152608401611ae3565b5f5f6128698484808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061218692505050565b5f818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff1660028111156128d7576128d7613d56565b60028111156128e8576128e8613d56565b90525090505f8160600151600281111561290457612904613d56565b14612922576040516335e09e9d60e01b815260040160405180910390fd5b6001600160401b0380166129678686808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061350e92505050565b6001600160401b03160361298e57604051631958236d60e21b815260040160405180910390fd5b6001600160401b0380166129d38686808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061353292505050565b6001600160401b0316146129fa57604051632eade63760e01b815260040160405180910390fd5b612a02612d6a565b612a0b90614425565b612a468686808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061354992505050565b1480612a9b5750612a5561355d565b612a5e90614425565b612a998686808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061354992505050565b145b612ab857604051633772dd5360e11b815260040160405180910390fd5b5f612af48686808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061357992505050565b9050612b0d612b028c612275565b8b88888c8c8f61234c565b60398054905f612b1c8361446c565b9091555050603a545f90600160401b90046001600160401b031615612b5357603a54600160401b90046001600160401b0316612b60565b603a546001600160401b03165b6040805160808101825264ffffffffff8d1681526001600160401b03858116602083015283169181019190915290915060608101600190525f858152603660209081526040918290208351815492850151938501516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b031990951691909216179290921792831682178155606084015190929091839160ff60c01b1990911668ffffffffffffffffff60801b1990911617600160c01b836002811115612c3557612c35613d56565b021790555050603d8054849250601390612c60908490600160981b90046001600160401b03166143bc565b92506101000a8154816001600160401b0302191690836001600160401b031602179055507f101790c2993f6a4d962bd17c786126823ba1c4cf04ff4cccb2659d50fb20aee884604051612cb591815260200190565b60405180910390a1604080518581526001600160401b03838116602083015284168183015290517fcdae700d7241bc027168c53cf6f889763b0a2c88a65d77fc13a8a9fef0d8605f9181900360600190a1612d1d633b9aca006001600160401b0384166141c3565b9c9b505050505050505050505050565b60605f612d3983613590565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b604051606090612d8690600160f81b905f903090602001614484565b604051602081830303815290604052905090565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663a9059cbb60e01b179052611a489084906135b7565b5f612df684613218565b9050612e03816003614286565b612e0e9060206141c3565b612e1b602084018461416d565b905014612e3b576040516313717da960e21b815260040160405180910390fd5b6003811b600c17612e90612e52602085018561416d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508892505086359050846131e3565b611d83576040516309bde33960e01b815260040160405180910390fd5b5f5f5f866020015192505f612ec686895f01518761368a565b9050836001600160401b0316816001600160401b031614612f3a57612eeb84826144c2565b60408051873581526001600160401b038a8116602083015284168183015290519194507fcdae700d7241bc027168c53cf6f889763b0a2c88a65d77fc13a8a9fef0d8605f919081900360600190a15b6001600160401b0380821660208a0181905290881660408a01525f03612fb85760398054905f612f69836144f1565b909155505060026060890152612f7e83614506565b6040519092508535906001600160401b038916907f5ce0aa04ae51d52da6e680fbe0336d2e2432f7c3dc2d4f3193204c57b9072107905f90a35b509450945094915050565b8051603c556020810151603d805460408401516060850151608086015162ffffff9095166affffffffffffffffffffff19909316831763010000006001600160401b0393841602176fffffffffffffffffffffffffffffffff60581b1916600160581b9183169190910267ffffffffffffffff60981b191617600160981b9190941602929092179055156130545750565b60808101516034545f91613070916001600160401b03166143bc565b90505f8260600151836040015161308791906143db565b60408401516034805492935090915f906130ab9084906001600160401b03166143bc565b82546101009290920a6001600160401b03818102199093169183160217909155603a8054600160401b810483166001600160801b03199091161790555f91506130fb90633b9aca009085166141c3565b90505f613110633b9aca00600785900b61452b565b603a546040518281529192506001600160401b0316907f525408c201bc1576eb44116f6478f1c2a54775b19a043bcfdc708364f74f8e449060200160405180910390a260335460405163a1ca780b60e01b81526001600160a01b03918216600482015260248101849052604481018390527f00000000000000000000000000000000000000000000000000000000000000009091169063a1ca780b906064015f604051808303815f87803b1580156131c6575f5ffd5b505af11580156131d8573d5f5f3e3d5ffd5b505050505050505050565b5f83613202576040516329e7276760e11b815260040160405180910390fd5b8361320e868585613768565b1495945050505050565b5f8082600181111561322c5761322c613d56565b146132385760066110d1565b600592915050565b5f60018251116132635760405163f8ef036760e01b815260040160405180910390fd5b61326d825161385b565b61328a5760405163f6558f5160e01b815260040160405180910390fd5b5f600283516132999190614359565b90505f816001600160401b038111156132b4576132b4613f04565b6040519080825280602002602001820160405280156132dd578160200160208202803683370190505b5090505f5b828110156133d7576002856132f783836141c3565b81518110613307576133076141ed565b60200260200101518683600261331d91906141c3565b613328906001614286565b81518110613338576133386141ed565b602002602001015160405160200161335a929190918252602082015260400190565b60408051601f198184030181529082905261337491614257565b602060405180830381855afa15801561338f573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906133b291906142b8565b8282815181106133c4576133c46141ed565b60209081029190910101526001016132e2565b505b816001146134eb576133ec600283614359565b91505f5b828110156134e55760028261340583836141c3565b81518110613415576134156141ed565b60200260200101518383600261342b91906141c3565b613436906001614286565b81518110613446576134466141ed565b6020026020010151604051602001613468929190918252602082015260400190565b60408051601f198184030181529082905261348291614257565b602060405180830381855afa15801561349d573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906134c091906142b8565b8282815181106134d2576134d26141ed565b60209081029190910101526001016133f0565b506133d9565b805f815181106134fd576134fd6141ed565b602002602001015192505050919050565b5f6110d182600581518110613525576135256141ed565b602002602001015161387a565b5f6110d182600681518110613525576135256141ed565b5f81600181518110612199576121996141ed565b604051606090612d8690600160f91b905f903090602001614484565b5f6110d182600281518110613525576135256141ed565b5f60ff8216601f8111156110d157604051632cd44ac360e21b815260040160405180910390fd5b5f61360b826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166138e19092919063ffffffff16565b905080515f148061362b57508080602001905181019061362b919061410d565b611a485760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608401611ae3565b5f61369760266001614286565b6136a29060206141c3565b6136af604084018461416d565b9050146136cf576040516313717da960e21b815260040160405180910390fd5b5f6136db60048561455a565b64ffffffffff1690506137346136f4604085018561416d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525089925050506020860135846131e3565b613751576040516309bde33960e01b815260040160405180910390fd5b61375f8360200135856138ef565b95945050505050565b5f83515f141580156137855750602084516137839190614583565b155b6137a2576040516313717da960e21b815260040160405180910390fd5b604080516020808201909252848152905b85518111613832576137c6600285614583565b5f036137f85781515f528086015160205260208260405f60026107d05a03fa6137ed575f5ffd5b600284049350613820565b808601515f52815160205260208260405f60026107d05a03fa613819575f5ffd5b6002840493505b61382b602082614286565b90506137b3565b508215613852576040516363df817160e01b815260040160405180910390fd5b51949350505050565b5f81158015906110d157506138716001836141da565b82161592915050565b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161767ff0000000000000060889290921c919091161790565b60606114e584845f8561391b565b5f806138fc600484614596565b6139079060406145bf565b64ffffffffff1690506114e584821b61387a565b60608247101561397c5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b6064820152608401611ae3565b5f5f866001600160a01b031685876040516139979190614257565b5f6040518083038185875af1925050503d805f81146139d1576040519150601f19603f3d011682016040523d82523d5f602084013e6139d6565b606091505b50915091506139e7878383876139f2565b979650505050505050565b60608315613a605782515f03613a59576001600160a01b0385163b613a595760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401611ae3565b50816114e5565b6114e58383815115613a755781518083602001fd5b8060405162461bcd60e51b8152600401611ae39190613cd5565b6001600160401b0381168114613aa3575f5ffd5b50565b5f60408284031215613ab6575f5ffd5b50919050565b5f5f5f60608486031215613ace575f5ffd5b8335613ad981613a8f565b925060208401356001600160401b03811115613af3575f5ffd5b613aff86828701613aa6565b92505060408401356001600160401b03811115613b1a575f5ffd5b613b2686828701613aa6565b9150509250925092565b5f5f83601f840112613b40575f5ffd5b5081356001600160401b03811115613b56575f5ffd5b6020830191508360208260051b8501011115613b70575f5ffd5b9250929050565b5f5f60208385031215613b88575f5ffd5b82356001600160401b03811115613b9d575f5ffd5b613ba985828601613b30565b90969095509350505050565b5f5f5f5f5f5f5f5f60a0898b031215613bcc575f5ffd5b8835613bd781613a8f565b975060208901356001600160401b03811115613bf1575f5ffd5b613bfd8b828c01613aa6565b97505060408901356001600160401b03811115613c18575f5ffd5b613c248b828c01613b30565b90975095505060608901356001600160401b03811115613c42575f5ffd5b613c4e8b828c01613b30565b90955093505060808901356001600160401b03811115613c6c575f5ffd5b613c788b828c01613b30565b999c989b5096995094979396929594505050565b5f60208284031215613c9c575f5ffd5b813561234581613a8f565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f6123456020830184613ca7565b5f5f83601f840112613cf7575f5ffd5b5081356001600160401b03811115613d0d575f5ffd5b602083019150836020828501011115613b70575f5ffd5b5f5f60208385031215613d35575f5ffd5b82356001600160401b03811115613d4a575f5ffd5b613ba985828601613ce7565b634e487b7160e01b5f52602160045260245ffd5b60038110613d8657634e487b7160e01b5f52602160045260245ffd5b9052565b602081016110d18284613d6a565b5f60208284031215613da8575f5ffd5b5035919050565b5f6080820190506001600160401b0383511682526001600160401b0360208401511660208301526001600160401b0360408401511660408301526060830151613dfb6060840182613d6a565b5092915050565b8015158114613aa3575f5ffd5b5f60208284031215613e1f575f5ffd5b813561234581613e02565b5f5f5f5f5f60608688031215613e3e575f5ffd5b85356001600160401b03811115613e53575f5ffd5b613e5f88828901613ce7565b90965094505060208601356001600160401b03811115613e7d575f5ffd5b613e8988828901613ce7565b96999598509660400135949350505050565b6001600160a01b0381168114613aa3575f5ffd5b8035613eba81613e9b565b919050565b5f5f60408385031215613ed0575f5ffd5b8235613edb81613e9b565b946020939093013593505050565b5f60208284031215613ef9575f5ffd5b813561234581613e9b565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b0381118282101715613f4057613f40613f04565b604052919050565b5f6001600160401b03821115613f6057613f60613f04565b5060051b60200190565b5f82601f830112613f79575f5ffd5b8135613f8c613f8782613f48565b613f18565b8082825260208201915060208360051b860101925085831115613fad575f5ffd5b602085015b83811015613fca578035835260209283019201613fb2565b5095945050505050565b5f5f5f60608486031215613fe6575f5ffd5b83356001600160401b03811115613ffb575f5ffd5b8401601f8101861361400b575f5ffd5b8035614019613f8782613f48565b8082825260208201915060208360051b85010192508883111561403a575f5ffd5b6020840193505b8284101561406557833561405481613e9b565b825260209384019390910190614041565b955050505060208401356001600160401b03811115614082575f5ffd5b61408e86828701613f6a565b92505061409d60408501613eaf565b90509250925092565b5f5f5f604084860312156140b8575f5ffd5b83356001600160401b038111156140cd575f5ffd5b6140d986828701613aa6565b93505060208401356001600160401b038111156140f4575f5ffd5b61410086828701613b30565b9497909650939450505050565b5f6020828403121561411d575f5ffd5b815161234581613e02565b5f5f8335601e1984360301811261413d575f5ffd5b8301803591506001600160401b03821115614156575f5ffd5b6020019150600581901b3603821315613b70575f5ffd5b5f5f8335601e19843603018112614182575f5ffd5b8301803591506001600160401b0382111561419b575f5ffd5b602001915036819003821315613b70575f5ffd5b634e487b7160e01b5f52601160045260245ffd5b80820281158282048414176110d1576110d16141af565b818103818111156110d1576110d16141af565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e19833603018112614215575f5ffd5b9190910192915050565b8284823760c09190911b6001600160c01b0319169101908152600801919050565b5f81518060208401855e5f93019283525090919050565b5f6123458284614240565b5f60208284031215614272575f5ffd5b813564ffffffffff81168114612345575f5ffd5b808201808211156110d1576110d16141af565b838582375f8482015f8152838582375f93019283525090949350505050565b5f602082840312156142c8575f5ffd5b5051919050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b608081525f61430a60808301888a6142cf565b828103602084015261431c8188613ca7565b905082810360408401526143318186886142cf565b915050826060830152979650505050505050565b634e487b7160e01b5f52601260045260245ffd5b5f8261436757614367614345565b500490565b6001600160401b0382811682821603908111156110d1576110d16141af565b5f8235605e19833603018112614215575f5ffd5b5f62ffffff8216806143b3576143b36141af565b5f190192915050565b6001600160401b0381811683821601908111156110d1576110d16141af565b600781810b9083900b01677fffffffffffffff8113677fffffffffffffff19821217156110d1576110d16141af565b5f6020828403121561441a575f5ffd5b815161234581613a8f565b80516020808301519190811015613ab6575f1960209190910360031b1b16919050565b5f6144538285614240565b6001600160801b03199390931683525050601001919050565b5f6001820161447d5761447d6141af565b5060010190565b6001600160f81b03199390931683526001600160a81b031991909116600183015260601b6bffffffffffffffffffffffff1916600c82015260200190565b600782810b9082900b03677fffffffffffffff198112677fffffffffffffff821317156110d1576110d16141af565b5f816144ff576144ff6141af565b505f190190565b5f8160070b677fffffffffffffff198103614523576145236141af565b5f0392915050565b8082025f8212600160ff1b84141615614546576145466141af565b81810583148215176110d1576110d16141af565b5f64ffffffffff83168061457057614570614345565b8064ffffffffff84160491505092915050565b5f8261459157614591614345565b500690565b5f64ffffffffff8316806145ac576145ac614345565b8064ffffffffff84160691505092915050565b64ffffffffff8181168382160290811690818114613dfb57613dfb6141af56fea2646970667358221220905476a9bc91953440c84d770e9b89fddd5a8856c845405968955c8c2a0995e264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@QaIZ8\x03\x80aIZ\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\x88V[\x80a\08\x81a\0_V[`\x80RP`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\xA0R\x82\x16`\xC0Ra\0Wa\0\xA5V[PPPa\x02\xB9V[__\x82\x90P`\x1F\x81Q\x11\x15a\0\x92W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\0\x89\x91\x90a\x02^V[`@Q\x80\x91\x03\x90\xFD[\x80Qa\0\x9D\x82a\x02\x93V[\x17\x93\x92PPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x01\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01a\0\x89V[_T`\xFF\x90\x81\x16\x14a\x01[W_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01qW__\xFD[PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[___``\x84\x86\x03\x12\x15a\x01\x9AW__\xFD[\x83Qa\x01\xA5\x81a\x01]V[` \x85\x01Q\x90\x93Pa\x01\xB6\x81a\x01]V[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xD1W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x01\xE1W__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xFAWa\x01\xFAa\x01tV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02(Wa\x02(a\x01tV[`@R\x81\x81R\x82\x82\x01` \x01\x88\x10\x15a\x02?W__\xFD[\x81` \x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x02\xB3W_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\x80Q`\xA0Q`\xC0QaF\x15a\x03E_9_\x81\x81a\x03)\x01R\x81\x81a\x06\xD4\x01R\x81\x81a\x07|\x01R\x81\x81a\n*\x01R\x81\x81a\r\xEB\x01R\x81\x81a\x0F\xE5\x01R\x81\x81a\x10\xF0\x01R\x81\x81a\x15\xF1\x01R\x81\x81a\x16\x98\x01R\x81\x81a\x19!\x01R\x81\x81a\x1Co\x01R\x81\x81a\x1D\xA3\x01R\x81\x81a\"y\x01Ra1\x83\x01R_\x81\x81a\x05z\x01Ra\x17\x01\x01R_a\x10Q\x01RaF\x15_\xF3\xFE`\x80`@R`\x046\x10a\x01\xBDW_5`\xE0\x1C\x80cf\x91\x95N\x11a\0\xF2W\x80c\xB5\"S\x8A\x11a\0\x92W\x80c\xD0mU\x87\x11a\0bW\x80c\xD0mU\x87\x14a\x06?W\x80c\xDD\xA34l\x14a\x06^W\x80c\xEE\x94\xD6|\x14a\x06}W\x80c\xF0t\xBAb\x14a\x06\x9CW__\xFD[\x80c\xB5\"S\x8A\x14a\x05\xCEW\x80c\xC4N0\xDC\x14a\x05\xEDW\x80c\xC4\x90tB\x14a\x06\x01W\x80c\xC4\xD6m\xE8\x14a\x06 W__\xFD[\x80ct9\x84\x1F\x11a\0\xCDW\x80ct9\x84\x1F\x14a\x054W\x80ct\xCD\xD7\x98\x14a\x05iW\x80c\x88gl\xAD\x14a\x05\x9CW\x80c\x9BNF4\x14a\x05\xBBW__\xFD[\x80cf\x91\x95N\x14a\x04\xD6W\x80cl\r-Z\x14a\x04\xE9W\x80co\xCD\x0ES\x14a\x05\x08W__\xFD[\x80cB\xEC\xFF*\x11a\x01]W\x80cR9jY\x11a\x018W\x80cR9jY\x14a\x046W\x80cT\xFDMP\x14a\x04jW\x80cXu3W\x14a\x04\x8BW\x80cX\xEA\xEEy\x14a\x04\xAAW__\xFD[\x80cB\xEC\xFF*\x14a\x02\xF2W\x80cFe\xBC\xDA\x14a\x03\x18W\x80cG\xD2\x83r\x14a\x03KW__\xFD[\x80c#@\xE8\xD3\x11a\x01\x98W\x80c#@\xE8\xD3\x14a\x02zW\x80c4t\xAA\x16\x14a\x02\x8FW\x80c?_\xA5z\x14a\x02\xC0W\x80c?e\xCF\x19\x14a\x02\xD3W__\xFD[\x80c\x03\x91W\xD2\x14a\x01\xFBW\x80c\x0B\x18\xFFf\x14a\x02\x1CW\x80c\x1EQU3\x14a\x02XW__\xFD[6a\x01\xF7W`@Q4\x81R\x7Fo\xDD=\xBD\xB1s)\x96\x08\xC0\xAA\x9F6\x875\x85|\x88B\xB5\x81\xF88\x928\xBF\x05\xBD\x04\xB3\xBFI\x90` \x01`@Q\x80\x91\x03\x90\xA1\0[__\xFD[4\x80\x15a\x02\x06W__\xFD[Pa\x02\x1Aa\x02\x156`\x04a:\xBCV[a\x06\xBBV[\0[4\x80\x15a\x02'W__\xFD[P`3Ta\x02;\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02cW__\xFD[Pa\x02la\t\xF0V[`@Q\x90\x81R` \x01a\x02OV[4\x80\x15a\x02\x85W__\xFD[Pa\x02l`9T\x81V[4\x80\x15a\x02\x9AW__\xFD[P`4T`\x01`\x01`@\x1B\x03\x16[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02OV[a\x02\x1Aa\x02\xCE6`\x04a;wV[a\n\x11V[4\x80\x15a\x02\xDEW__\xFD[Pa\x02\x1Aa\x02\xED6`\x04a;\xB5V[a\r\x92V[4\x80\x15a\x02\xFDW__\xFD[P`:Ta\x02\xA8\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x03#W__\xFD[Pa\x02;\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03VW__\xFD[Pa\x03\xDB`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82R`<T\x81R`=Tb\xFF\xFF\xFF\x81\x16` \x83\x01R`\x01`\x01`@\x1B\x03c\x01\0\0\0\x82\x04\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`X\x1B\x81\x04`\x07\x0B``\x83\x01R`\x01`\x98\x1B\x90\x04\x90\x91\x16`\x80\x82\x01R\x90V[`@Qa\x02O\x91\x90_`\xA0\x82\x01\x90P\x82Q\x82Rb\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`\x01`\x01`@\x1B\x03`@\x84\x01Q\x16`@\x83\x01R``\x83\x01Q`\x07\x0B``\x83\x01R`\x01`\x01`@\x1B\x03`\x80\x84\x01Q\x16`\x80\x83\x01R\x92\x91PPV[4\x80\x15a\x04AW__\xFD[Pa\x02\xA8a\x04P6`\x04a<\x8CV[`;` R_\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x04uW__\xFD[Pa\x04~a\x10JV[`@Qa\x02O\x91\x90a<\xD5V[4\x80\x15a\x04\x96W__\xFD[P`>Ta\x02;\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\xB5W__\xFD[Pa\x04\xC9a\x04\xC46`\x04a=$V[a\x10uV[`@Qa\x02O\x91\x90a=\x8AV[a\x02\x1Aa\x04\xE46`\x04a;wV[a\x10\xD7V[4\x80\x15a\x04\xF4W__\xFD[Pa\x02la\x05\x036`\x04a<\x8CV[a\x13\xDFV[4\x80\x15a\x05\x13W__\xFD[Pa\x05'a\x05\"6`\x04a=\x98V[a\x14\xEDV[`@Qa\x02O\x91\x90a=\xAFV[4\x80\x15a\x05?W__\xFD[Pa\x04\xC9a\x05N6`\x04a=\x98V[_\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x90V[4\x80\x15a\x05tW__\xFD[Pa\x02;\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\xA7W__\xFD[Pa\x02\x1Aa\x05\xB66`\x04a>\x0FV[a\x15\x98V[a\x02\x1Aa\x05\xC96`\x04a>*V[a\x16\x8DV[4\x80\x15a\x05\xD9W__\xFD[Pa\x05'a\x05\xE86`\x04a=$V[a\x18\x0BV[4\x80\x15a\x05\xF8W__\xFD[Pa\x02la\x18\xFAV[4\x80\x15a\x06\x0CW__\xFD[Pa\x02\x1Aa\x06\x1B6`\x04a>\xBFV[a\x19\x16V[4\x80\x15a\x06+W__\xFD[Pa\x02\x1Aa\x06:6`\x04a>\xE9V[a\x1AMV[4\x80\x15a\x06JW__\xFD[Pa\x02\x1Aa\x06Y6`\x04a>\xE9V[a\x1B\x97V[4\x80\x15a\x06iW__\xFD[Pa\x02\x1Aa\x06x6`\x04a?\xD4V[a\x1C+V[4\x80\x15a\x06\x88W__\xFD[P`:Ta\x02\xA8\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x06\xA7W__\xFD[Pa\x02\x1Aa\x06\xB66`\x04a@\xA6V[a\x1D\x8AV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07E\x91\x90aA\rV[\x15a\x07cW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x08`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xED\x91\x90aA\rV[\x15a\x08\x0BW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08Oa\x08\x19\x85\x80aA(V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa!\x86\x92PPPV[_\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x08\xBDWa\x08\xBDa=VV[`\x02\x81\x11\x15a\x08\xCEWa\x08\xCEa=VV[\x81RPP\x90P\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x87`\x01`\x01`@\x1B\x03\x16\x11a\t\nW`@Qc7\xE0\x7F\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81``\x01Q`\x02\x81\x11\x15a\t\"Wa\t\"a=VV[\x14a\t@W`@Qc\xD4\x9E\x19\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\x83a\tM\x86\x80aA(V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa!\xA8\x92PPPV[a\t\xA0W`@Qc\x16\x1C\xE5\xED`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xB2a\t\xAC\x88a\x13\xDFV[\x87a!\xD0V[a\t\xDEa\t\xBE\x88a\"uV[\x875a\t\xCA\x88\x80aA(V[a\t\xD7` \x8B\x01\x8BaAmV[\x87Qa#LV[a\t\xE7_a$\x80V[PPPPPPPV[_a\n\x0Cq\xBB\xDD\xC7\xCEH\x86B\xFBW\x9F\x8B\0\xF3\xA5\x90\0rQa&\0V[\x90P\x90V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\n`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nwW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9B\x91\x90aA\rV[\x15a\n\xB9W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\n\xDCWP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\n\xF9W`@QcBzwy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0B\x02a\x18\xFAV[\x90P_a\x0B\x0F\x84\x83aA\xC3V[\x90P\x804\x10\x15a\x0B2W`@Qc5f\x80\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0B=\x824aA\xDAV[\x90P_[\x85\x81\x10\x15a\ryW6\x87\x87\x83\x81\x81\x10a\x0B\\Wa\x0B\\aA\xEDV[\x90P` \x02\x81\x01\x90a\x0Bn\x91\x90aB\x01V[\x90P_a\x0B\xB7a\x0B~\x83\x80aAmV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa&\x86\x92PPPV[\x90P`\x01_\x82\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0B\xE4Wa\x0B\xE4a=VV[\x14a\x0C\x02W`@Qc\xD4\x9E\x19\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\r\x83\x80aAmV[a\x0C\x1D`@\x86\x01` \x87\x01a<\x8CV[`@Q` \x01a\x0C/\x93\x92\x91\x90aB\x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P_q\ta\xEFH\x0E\xB5^\x80\xD1\x9A\xD85y\xA6L\0p\x02`\x01`\x01`\xA0\x1B\x03\x16\x88\x83`@Qa\x0Cl\x91\x90aBWV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x0C\xA6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0C\xABV[``\x91P[PP\x90P\x80a\x0C\xCDW`@Qc\xFCR\xD4\x83`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\xDD`@\x85\x01` \x86\x01a<\x8CV[`\x01`\x01`@\x1B\x03\x16_\x03a\r\x1BW`@Q\x83\x90\x7F`\xD8\xCA\x01MGe\xA2\xB8\xB3\x89\xE2W\x14\xCB\x1C\xEF\x83\xB5t\")\x11\xA0\x1D\x90\xC1\xBDi\xD2\xD3 \x90_\x90\xA2a\riV[\x82\x7F\x8B'7\xBBd\xAB/-\xC0\x95R\xDF\xA1\xC2P9\x9EjB\xC7\xEA\x9F\x0E\x1Ce\x8F]e\xD7\x08\xEC\x05a\rM`@\x87\x01` \x88\x01a<\x8CV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA2[PP`\x01\x90\x92\x01\x91Pa\x0BA\x90PV[P\x80\x15a\r\x8AWa\r\x8A3\x82a'\x17V[PPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\r\xB5WP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\r\xD2W`@QcBzwy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x02`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\\\x91\x90aA\rV[\x15a\x0EzW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x84\x14\x80\x15a\x0E\x88WP\x83\x82\x14[a\x0E\xA5W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`\x01`@\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x90\x8A\x16\x11a\x0E\xDBW`@Qc7\xE0\x7F\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x8A\x16\x11a\x0F\nW`@Qc5\xE7\xF6\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x1Ca\x0F\x16\x8Aa\x13\xDFV[\x89a!\xD0V[_\x80[\x87\x81\x10\x15a\x0F\xB5Wa\x0F\xA1\x8B\x8B5\x8B\x8B\x85\x81\x81\x10a\x0F?Wa\x0F?aA\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x0FT\x91\x90aBbV[\x8A\x8A\x86\x81\x81\x10a\x0FfWa\x0FfaA\xEDV[\x90P` \x02\x81\x01\x90a\x0Fx\x91\x90aAmV[\x8A\x8A\x88\x81\x81\x10a\x0F\x8AWa\x0F\x8AaA\xEDV[\x90P` \x02\x81\x01\x90a\x0F\x9C\x91\x90aA(V[a(,V[a\x0F\xAB\x90\x83aB\x86V[\x91P`\x01\x01a\x0F\x1FV[P`3T`@Qc\xA1\xCAx\x0B`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R_`$\x82\x01R`D\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA1\xCAx\x0B\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10(W__\xFD[PZ\xF1\x15\x80\x15a\x10:W=__>=_\xFD[PPPPPPPPPPPPPPV[``a\n\x0C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a--V[__a\x10\xB5\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa&\x86\x92PPPV[_\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x91PP[\x92\x91PPV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\t`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11a\x91\x90aA\rV[\x15a\x11\x7FW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x11\xA2WP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\x11\xBFW`@QcBzwy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x11\xC8a\t\xF0V[\x90P_a\x11\xD5\x84\x83aA\xC3V[\x90P\x804\x10\x15a\x11\xF8W`@Qc5f\x80\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x12\x03\x824aA\xDAV[\x90P_[\x85\x81\x10\x15a\ryW6\x87\x87\x83\x81\x81\x10a\x12\"Wa\x12\"aA\xEDV[\x90P` \x02\x81\x01\x90a\x124\x91\x90aB\x01V[\x90P_a\x12Da\x0B~\x83\x80aAmV[\x90P_a\x12Wa\x0B~` \x85\x01\x85aAmV[\x90P`\x01_\x82\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x12\x84Wa\x12\x84a=VV[\x14a\x12\xA2W`@Qc\xD4\x9E\x19\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x12\xAD\x84\x80aAmV[a\x12\xBA` \x87\x01\x87aAmV[`@Q` \x01a\x12\xCD\x94\x93\x92\x91\x90aB\x99V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P_q\xBB\xDD\xC7\xCEH\x86B\xFBW\x9F\x8B\0\xF3\xA5\x90\0rQ`\x01`\x01`\xA0\x1B\x03\x16\x89\x83`@Qa\x13\n\x91\x90aBWV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x13DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x13IV[``\x91P[PP\x90P\x80a\x13kW`@Qc\xFCR\xD4\x83`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x03a\x13\xA1W`@Q\x84\x90\x7F\xC9{\x96[\x92\xAE\x7F\xD2\0\x95\xFE\x8E\xB7\xB9\x9F\x81\xF9_\x8CJ\xDF\xFB\"\xA1\x91\x16\xD8\xEB(F\xB0\x16\x90_\x90\xA2a\x13\xCEV[`@Q\x83\x90\x85\x90\x7FB\xF9\xC9\xDB,\xA4C\xE9\xECb\xF4X\x8B\xD0\xC9\xB2A\x06\\\x02\xC2\xA8\0\x1A\xC1d\xAE\x12\x82\xDC{\x94\x90_\x90\xA3[PP`\x01\x90\x93\x01\x92Pa\x12\x07\x91PPV[_a\x13\xEDa\x1F\xFF`\x0CaA\xC3V[a\x14\0`\x01`\x01`@\x1B\x03\x84\x16BaA\xDAV[\x10a\x14\x1EW`@QcyD\xE6m`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`@\x1B\x03\x84\x16` \x82\x01R_\x91\x82\x91r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x14e\x91aBWV[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a\x14\x9DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x14\xA2V[``\x91P[P\x91P\x91P\x81\x80\x15a\x14\xB4WP_\x81Q\x11[a\x14\xD1W`@QcU\x8A\xD0\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x80` \x01\x90Q\x81\x01\x90a\x14\xE5\x91\x90aB\xB8V[\x94\x93PPPPV[a\x15\x14`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[_\x82\x81R`6` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x81\x04\x90\x93\x16\x93\x81\x01\x93\x90\x93R\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x15~Wa\x15~a=VV[`\x02\x81\x11\x15a\x15\x8FWa\x15\x8Fa=VV[\x90RP\x92\x91PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x15\xBBWP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\x15\xD8W`@QcBzwy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16>W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16b\x91\x90aA\rV[\x15a\x16\x80W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\x89\x82a$\x80V[PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x16\xD6W`@Qc2\x13\xA6a`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[4h\x01\xBC\x16\xD6t\xEC\x80\0\0\x14a\x16\xFFW`@Qc\x04\x96\x96\xB3`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\"\x89Q\x18h\x01\xBC\x16\xD6t\xEC\x80\0\0\x87\x87a\x17Ba-jV[\x88\x88\x88`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17f\x96\x95\x94\x93\x92\x91\x90aB\xF7V[_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x17}W__\xFD[PZ\xF1\x15\x80\x15a\x17\x8FW=__>=_\xFD[PPPPP\x7F\xA0\x10\x03vm<\xD9|\xF2\xAD\xE5B\x96\x90\xBF] k\xE7\xFB\x01\xEF\x9D:\0\x89\xEC\xF6{\xC1\x12\x19a\x17\xF3\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa&\x86\x92PPPV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1PPPPPV[a\x182`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`6_a\x18s\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa&\x86\x92PPPV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x90\x91``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x18\xDFWa\x18\xDFa=VV[`\x02\x81\x11\x15a\x18\xF0Wa\x18\xF0a=VV[\x90RP\x93\x92PPPV[_a\n\x0Cq\ta\xEFH\x0E\xB5^\x80\xD1\x9A\xD85y\xA6L\0p\x02a&\0V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x19_W`@Qc2\x13\xA6a`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x19nc;\x9A\xCA\0\x83aCYV[\x90Pa\x19\x87c;\x9A\xCA\0`\x01`\x01`@\x1B\x03\x83\x16aA\xC3V[`4T\x90\x92P`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x82\x16\x11\x15a\x19\xBAW`@Qc\x02\xC6\xF5G`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`4\x80T\x82\x91\x90_\x90a\x19\xD7\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aClV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x89G\xFD,\xE0~\xF9\xCC0,N\x8F\x04a\x01V\x15\xD9\x1C\xE8QVH9\xE9\x1C\xC8\x04\xC2\xF4\x9D\x8E\x83`@Qa\x1A6\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x1AH\x83\x83a'\x17V[PPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1AkWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1A\x84WP0;\x15\x80\x15a\x1A\x84WP_T`\xFF\x16`\x01\x14[a\x1A\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1B\rW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1B4W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x16\x89W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1B\xC2W`@Qcq\x9F7\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xFB\x81)\x08\n\x19\xD3M\xCE\xAC\x04\xBA%?\xC5\x03\x04\xDC\x86\xC7)\xBDc\xCD\xCAJ\x96\x9A\xD1\x9A^\xAC\x91\x01`@Q\x80\x91\x03\x90\xA1`>\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1CVW`@Qcq\x9F7\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x05`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xE0\x91\x90aA\rV[\x15a\x1C\xFEW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q\x84Q\x14a\x1D W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84Q\x81\x10\x15a\x1D\x83Wa\x1D{\x83\x85\x83\x81Q\x81\x10a\x1DAWa\x1DAaA\xEDV[` \x02` \x01\x01Q\x87\x84\x81Q\x81\x10a\x1D[Wa\x1D[aA\xEDV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16a-\x9A\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01\x01a\x1D\"V[PPPPPV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x07`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x14\x91\x90aA\rV[\x15a\x1E2W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16_\x81\x90\x03a\x1EfW`@Qc\x1ATOI`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R`<T\x81R`=Tb\xFF\xFF\xFF\x81\x16` \x83\x01R`\x01`\x01`@\x1B\x03c\x01\0\0\0\x82\x04\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`X\x1B\x81\x04`\x07\x0B``\x83\x01R`\x01`\x98\x1B\x90\x04\x90\x91\x16`\x80\x82\x01Ra\x1E\xCDa\x1E\xC5\x83a\"uV[\x82Q\x88a-\xECV[_\x80[\x85\x81\x10\x15a!-W6\x87\x87\x83\x81\x81\x10a\x1E\xEBWa\x1E\xEBaA\xEDV[\x90P` \x02\x81\x01\x90a\x1E\xFD\x91\x90aC\x8BV[\x805_\x90\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1FmWa\x1Fma=VV[`\x02\x81\x11\x15a\x1F~Wa\x1F~a=VV[\x90RP\x90P`\x01\x81``\x01Q`\x02\x81\x11\x15a\x1F\x9BWa\x1F\x9Ba=VV[\x14a\x1F\xA7WPPa!%V[\x85`\x01`\x01`@\x1B\x03\x16\x81`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10a\x1F\xCBWPPa!%V[_\x80\x80a\x1F\xDB\x84\x8A\x8F5\x88a.\xADV[` \x8B\x01\x80Q\x93\x96P\x91\x94P\x92Pa\x1F\xF2\x82aC\x9FV[b\xFF\xFF\xFF\x16\x90RP`\x80\x88\x01\x80Q\x84\x91\x90a \x0E\x90\x83\x90aC\xBCV[`\x01`\x01`@\x1B\x03\x16\x90RP``\x88\x01\x80Q\x83\x91\x90a .\x90\x83\x90aC\xDBV[`\x07\x0B\x90RPa >\x81\x88aC\xBCV[\x855_\x90\x81R`6` \x90\x81R`@\x91\x82\x90 \x87Q\x81T\x92\x89\x01Q\x93\x89\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x88\x01Q\x93\x9AP\x87\x93\x90\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x90\x91\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a \xE2Wa \xE2a=VV[\x02\x17\x90UPP`@Q\x865\x91P`\x01`\x01`@\x1B\x03\x8B\x16\x90\x7F\xE4\x86c5v\x1AQ\xDC\xAF\xF7fD\x8A\xB0\xAF`d)\x1E\xE5\xDC\x94\xE6\x84\x92\xBB\x9C\xD7W\xC1\xE3P\x90_\x90\xA3PPPPP[`\x01\x01a\x1E\xD0V[P`\x01`\x01`@\x1B\x03\x80\x84\x16_\x90\x81R`;` R`@\x81 \x80T\x84\x93\x91\x92\x91a!Y\x91\x85\x91\x16aC\xBCV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa\t\xE7\x82a/\xC3V[_\x81_\x81Q\x81\x10a!\x99Wa!\x99aA\xEDV[` \x02` \x01\x01Q\x90P\x91\x90PV[_\x81`\x03\x81Q\x81\x10a!\xBCWa!\xBCaA\xEDV[` \x02` \x01\x01Q__\x1B\x14\x15\x90P\x91\x90PV[a!\xDC`\x03` aA\xC3V[a!\xE9` \x83\x01\x83aAmV[\x90P\x14a\"\tW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\"Xa\"\x19` \x83\x01\x83aAmV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x86\x92PP\x845\x90P`\x03a1\xE3V[a\x16\x89W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c'\x045\x1A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xD3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xF7\x91\x90aD\nV[\x90P\x80`\x01`\x01`@\x1B\x03\x16_\x03a#\"W`@QcA\xA0,\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`@\x1B\x03\x16\x83`\x01`\x01`@\x1B\x03\x16\x11\x15a#CW`\x01a#EV[_[\x93\x92PPPV[`\x08\x84\x14a#mW`@Qc \x05\x91\xBD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a#w\x88a2\x18V[\x90P\x80a#\x86`(`\x01aB\x86V[a#\x90\x91\x90aB\x86V[a#\x9B\x90` aA\xC3V[\x83\x14a#\xBAW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a#\xF6\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa2@\x92PPPV[\x90P_d\xFF\xFF\xFF\xFF\xFF\x84\x16a$\r`(`\x01aB\x86V[`\x0B\x90\x1B\x17\x90Pa$W\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8D\x92P\x86\x91P\x85\x90Pa1\xE3V[a$tW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPV[`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15a$\xB0W`@Qb\xBE\x9B\xC3`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`\x01`@\x1B\x03B\x81\x16\x91\x16\x03a$\xDEW`@Qcg\xDB[\x8B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`4T_\x90`\x01`\x01`@\x1B\x03\x16a$\xFAc;\x9A\xCA\0GaCYV[a%\x04\x91\x90aClV[\x90P\x81\x80\x15a%\x1AWP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a%8W`@Qc2\xDE\xA9Y`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`\xA0\x01`@R\x80a%MBa\x13\xDFV[\x81R`9Tb\xFF\xFF\xFF\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x80\x85\x16`@\x83\x01R_``\x83\x01\x81\x90R`\x80\x90\x92\x01\x91\x90\x91R`:\x80TB\x90\x92\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90U\x90Pa%\xB1\x81a/\xC3V[\x80Q` \x80\x83\x01Q`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R`\x01`\x01`@\x1B\x03B\x16\x91\x7FWW\x96\x13;\xBE\xD37\xE5\xB3\x9A\xA4\x9A0\xDC%V\xA9\x1E\x0Cl*\xF4\xB7\xB8\x86\xAEw\xEB\xEF\x10v\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[___\x83`\x01`\x01`\xA0\x1B\x03\x16`@Q_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a&HW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a&MV[``\x91P[P\x91P\x91P\x81\x80\x15a&`WP\x80Q` \x14[a&}W`@Qc\xC9\x01X\xAF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xE5\x81aD%V[_\x81Q`0\x14a&\xA9W`@QcO\x8829`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x02\x90a&\xBF\x90\x84\x90_\x90` \x01aDHV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra&\xD9\x91aBWV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a&\xF4W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xD1\x91\x90aB\xB8V[\x80G\x10\x15a'gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x1A\xE3V[_\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a'\xB0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a'\xB5V[``\x91P[PP\x90P\x80a\x1AHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xE3V[__a(i\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa!\x86\x92PPPV[_\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a(\xD7Wa(\xD7a=VV[`\x02\x81\x11\x15a(\xE8Wa(\xE8a=VV[\x90RP\x90P_\x81``\x01Q`\x02\x81\x11\x15a)\x04Wa)\x04a=VV[\x14a)\"W`@Qc5\xE0\x9E\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x80\x16a)g\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa5\x0E\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x03a)\x8EW`@Qc\x19X#m`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x80\x16a)\xD3\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa52\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x14a)\xFAW`@Qc.\xAD\xE67`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a*\x02a-jV[a*\x0B\x90aD%V[a*F\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa5I\x92PPPV[\x14\x80a*\x9BWPa*Ua5]V[a*^\x90aD%V[a*\x99\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa5I\x92PPPV[\x14[a*\xB8W`@Qc7r\xDDS`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a*\xF4\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa5y\x92PPPV[\x90Pa+\ra+\x02\x8Ca\"uV[\x8B\x88\x88\x8C\x8C\x8Fa#LV[`9\x80T\x90_a+\x1C\x83aDlV[\x90\x91UPP`:T_\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15a+SW`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a+`V[`:T`\x01`\x01`@\x1B\x03\x16[`@\x80Q`\x80\x81\x01\x82Rd\xFF\xFF\xFF\xFF\xFF\x8D\x16\x81R`\x01`\x01`@\x1B\x03\x85\x81\x16` \x83\x01R\x83\x16\x91\x81\x01\x91\x90\x91R\x90\x91P``\x81\x01`\x01\x90R_\x85\x81R`6` \x90\x81R`@\x91\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x85\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x84\x01Q\x90\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x90\x91\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a,5Wa,5a=VV[\x02\x17\x90UPP`=\x80T\x84\x92P`\x13\x90a,`\x90\x84\x90`\x01`\x98\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aC\xBCV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x7F\x10\x17\x90\xC2\x99?jM\x96+\xD1|xa&\x82;\xA1\xC4\xCF\x04\xFFL\xCC\xB2e\x9DP\xFB \xAE\xE8\x84`@Qa,\xB5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x85\x81R`\x01`\x01`@\x1B\x03\x83\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q\x7F\xCD\xAEp\rrA\xBC\x02qh\xC5<\xF6\xF8\x89v;\n,\x88\xA6]w\xFC\x13\xA8\xA9\xFE\xF0\xD8`_\x91\x81\x90\x03``\x01\x90\xA1a-\x1Dc;\x9A\xCA\0`\x01`\x01`@\x1B\x03\x84\x16aA\xC3V[\x9C\x9BPPPPPPPPPPPPV[``_a-9\x83a5\x90V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`@Q``\x90a-\x86\x90`\x01`\xF8\x1B\x90_\x900\x90` \x01aD\x84V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x1AH\x90\x84\x90a5\xB7V[_a-\xF6\x84a2\x18V[\x90Pa.\x03\x81`\x03aB\x86V[a.\x0E\x90` aA\xC3V[a.\x1B` \x84\x01\x84aAmV[\x90P\x14a.;W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x81\x1B`\x0C\x17a.\x90a.R` \x85\x01\x85aAmV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92PP\x865\x90P\x84a1\xE3V[a\x1D\x83W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___\x86` \x01Q\x92P_a.\xC6\x86\x89_\x01Q\x87a6\x8AV[\x90P\x83`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14a/:Wa.\xEB\x84\x82aD\xC2V[`@\x80Q\x875\x81R`\x01`\x01`@\x1B\x03\x8A\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q\x91\x94P\x7F\xCD\xAEp\rrA\xBC\x02qh\xC5<\xF6\xF8\x89v;\n,\x88\xA6]w\xFC\x13\xA8\xA9\xFE\xF0\xD8`_\x91\x90\x81\x90\x03``\x01\x90\xA1[`\x01`\x01`@\x1B\x03\x80\x82\x16` \x8A\x01\x81\x90R\x90\x88\x16`@\x8A\x01R_\x03a/\xB8W`9\x80T\x90_a/i\x83aD\xF1V[\x90\x91UPP`\x02``\x89\x01Ra/~\x83aE\x06V[`@Q\x90\x92P\x855\x90`\x01`\x01`@\x1B\x03\x89\x16\x90\x7F\\\xE0\xAA\x04\xAEQ\xD5-\xA6\xE6\x80\xFB\xE03m.$2\xF7\xC3\xDC-O1\x93 LW\xB9\x07!\x07\x90_\x90\xA3[P\x94P\x94P\x94\x91PPV[\x80Q`<U` \x81\x01Q`=\x80T`@\x84\x01Q``\x85\x01Q`\x80\x86\x01Qb\xFF\xFF\xFF\x90\x95\x16j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x83\x17c\x01\0\0\0`\x01`\x01`@\x1B\x03\x93\x84\x16\x02\x17o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`X\x1B\x19\x16`\x01`X\x1B\x91\x83\x16\x91\x90\x91\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x98\x1B\x19\x16\x17`\x01`\x98\x1B\x91\x90\x94\x16\x02\x92\x90\x92\x17\x90U\x15a0TWPV[`\x80\x81\x01Q`4T_\x91a0p\x91`\x01`\x01`@\x1B\x03\x16aC\xBCV[\x90P_\x82``\x01Q\x83`@\x01Qa0\x87\x91\x90aC\xDBV[`@\x84\x01Q`4\x80T\x92\x93P\x90\x91_\x90a0\xAB\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aC\xBCV[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`:\x80T`\x01`@\x1B\x81\x04\x83\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17\x90U_\x91Pa0\xFB\x90c;\x9A\xCA\0\x90\x85\x16aA\xC3V[\x90P_a1\x10c;\x9A\xCA\0`\x07\x85\x90\x0BaE+V[`:T`@Q\x82\x81R\x91\x92P`\x01`\x01`@\x1B\x03\x16\x90\x7FRT\x08\xC2\x01\xBC\x15v\xEBD\x11odx\xF1\xC2\xA5Gu\xB1\x9A\x04;\xCF\xDCp\x83d\xF7O\x8ED\x90` \x01`@Q\x80\x91\x03\x90\xA2`3T`@Qc\xA1\xCAx\x0B`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA1\xCAx\x0B\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a1\xC6W__\xFD[PZ\xF1\x15\x80\x15a1\xD8W=__>=_\xFD[PPPPPPPPPV[_\x83a2\x02W`@Qc)\xE7'g`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a2\x0E\x86\x85\x85a7hV[\x14\x95\x94PPPPPV[_\x80\x82`\x01\x81\x11\x15a2,Wa2,a=VV[\x14a28W`\x06a\x10\xD1V[`\x05\x92\x91PPV[_`\x01\x82Q\x11a2cW`@Qc\xF8\xEF\x03g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a2m\x82Qa8[V[a2\x8AW`@Qc\xF6U\x8FQ`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\x02\x83Qa2\x99\x91\x90aCYV[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xB4Wa2\xB4a?\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a2\xDDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a3\xD7W`\x02\x85a2\xF7\x83\x83aA\xC3V[\x81Q\x81\x10a3\x07Wa3\x07aA\xEDV[` \x02` \x01\x01Q\x86\x83`\x02a3\x1D\x91\x90aA\xC3V[a3(\x90`\x01aB\x86V[\x81Q\x81\x10a38Wa38aA\xEDV[` \x02` \x01\x01Q`@Q` \x01a3Z\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra3t\x91aBWV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a3\x8FW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xB2\x91\x90aB\xB8V[\x82\x82\x81Q\x81\x10a3\xC4Wa3\xC4aA\xEDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a2\xE2V[P[\x81`\x01\x14a4\xEBWa3\xEC`\x02\x83aCYV[\x91P_[\x82\x81\x10\x15a4\xE5W`\x02\x82a4\x05\x83\x83aA\xC3V[\x81Q\x81\x10a4\x15Wa4\x15aA\xEDV[` \x02` \x01\x01Q\x83\x83`\x02a4+\x91\x90aA\xC3V[a46\x90`\x01aB\x86V[\x81Q\x81\x10a4FWa4FaA\xEDV[` \x02` \x01\x01Q`@Q` \x01a4h\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra4\x82\x91aBWV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a4\x9DW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xC0\x91\x90aB\xB8V[\x82\x82\x81Q\x81\x10a4\xD2Wa4\xD2aA\xEDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a3\xF0V[Pa3\xD9V[\x80_\x81Q\x81\x10a4\xFDWa4\xFDaA\xEDV[` \x02` \x01\x01Q\x92PPP\x91\x90PV[_a\x10\xD1\x82`\x05\x81Q\x81\x10a5%Wa5%aA\xEDV[` \x02` \x01\x01Qa8zV[_a\x10\xD1\x82`\x06\x81Q\x81\x10a5%Wa5%aA\xEDV[_\x81`\x01\x81Q\x81\x10a!\x99Wa!\x99aA\xEDV[`@Q``\x90a-\x86\x90`\x01`\xF9\x1B\x90_\x900\x90` \x01aD\x84V[_a\x10\xD1\x82`\x02\x81Q\x81\x10a5%Wa5%aA\xEDV[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x10\xD1W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a6\x0B\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a8\xE1\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q_\x14\x80a6+WP\x80\x80` \x01\x90Q\x81\x01\x90a6+\x91\x90aA\rV[a\x1AHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x1A\xE3V[_a6\x97`&`\x01aB\x86V[a6\xA2\x90` aA\xC3V[a6\xAF`@\x84\x01\x84aAmV[\x90P\x14a6\xCFW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a6\xDB`\x04\x85aEZV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa74a6\xF4`@\x85\x01\x85aAmV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x89\x92PPP` \x86\x015\x84a1\xE3V[a7QW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a7_\x83` \x015\x85a8\xEFV[\x95\x94PPPPPV[_\x83Q_\x14\x15\x80\x15a7\x85WP` \x84Qa7\x83\x91\x90aE\x83V[\x15[a7\xA2W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82\x01\x90\x92R\x84\x81R\x90[\x85Q\x81\x11a82Wa7\xC6`\x02\x85aE\x83V[_\x03a7\xF8W\x81Q_R\x80\x86\x01Q` R` \x82`@_`\x02a\x07\xD0Z\x03\xFAa7\xEDW__\xFD[`\x02\x84\x04\x93Pa8 V[\x80\x86\x01Q_R\x81Q` R` \x82`@_`\x02a\x07\xD0Z\x03\xFAa8\x19W__\xFD[`\x02\x84\x04\x93P[a8+` \x82aB\x86V[\x90Pa7\xB3V[P\x82\x15a8RW`@Qcc\xDF\x81q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Q\x94\x93PPPPV[_\x81\x15\x80\x15\x90a\x10\xD1WPa8q`\x01\x83aA\xDAV[\x82\x16\x15\x92\x91PPV[`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17g\xFF\0\0\0\0\0\0\0`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[``a\x14\xE5\x84\x84_\x85a9\x1BV[_\x80a8\xFC`\x04\x84aE\x96V[a9\x07\x90`@aE\xBFV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x14\xE5\x84\x82\x1Ba8zV[``\x82G\x10\x15a9|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x1A\xE3V[__\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa9\x97\x91\x90aBWV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a9\xD1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a9\xD6V[``\x91P[P\x91P\x91Pa9\xE7\x87\x83\x83\x87a9\xF2V[\x97\x96PPPPPPPV[``\x83\x15a:`W\x82Q_\x03a:YW`\x01`\x01`\xA0\x1B\x03\x85\x16;a:YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x1A\xE3V[P\x81a\x14\xE5V[a\x14\xE5\x83\x83\x81Q\x15a:uW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1A\xE3\x91\x90a<\xD5V[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a:\xA3W__\xFD[PV[_`@\x82\x84\x03\x12\x15a:\xB6W__\xFD[P\x91\x90PV[___``\x84\x86\x03\x12\x15a:\xCEW__\xFD[\x835a:\xD9\x81a:\x8FV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a:\xF3W__\xFD[a:\xFF\x86\x82\x87\x01a:\xA6V[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a;\x1AW__\xFD[a;&\x86\x82\x87\x01a:\xA6V[\x91PP\x92P\x92P\x92V[__\x83`\x1F\x84\x01\x12a;@W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a;VW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a;pW__\xFD[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a;\x88W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a;\x9DW__\xFD[a;\xA9\x85\x82\x86\x01a;0V[\x90\x96\x90\x95P\x93PPPPV[________`\xA0\x89\x8B\x03\x12\x15a;\xCCW__\xFD[\x885a;\xD7\x81a:\x8FV[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a;\xF1W__\xFD[a;\xFD\x8B\x82\x8C\x01a:\xA6V[\x97PP`@\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<\x18W__\xFD[a<$\x8B\x82\x8C\x01a;0V[\x90\x97P\x95PP``\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<BW__\xFD[a<N\x8B\x82\x8C\x01a;0V[\x90\x95P\x93PP`\x80\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<lW__\xFD[a<x\x8B\x82\x8C\x01a;0V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[_` \x82\x84\x03\x12\x15a<\x9CW__\xFD[\x815a#E\x81a:\x8FV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a#E` \x83\x01\x84a<\xA7V[__\x83`\x1F\x84\x01\x12a<\xF7W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a=\rW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a;pW__\xFD[__` \x83\x85\x03\x12\x15a=5W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a=JW__\xFD[a;\xA9\x85\x82\x86\x01a<\xE7V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10a=\x86WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[` \x81\x01a\x10\xD1\x82\x84a=jV[_` \x82\x84\x03\x12\x15a=\xA8W__\xFD[P5\x91\x90PV[_`\x80\x82\x01\x90P`\x01`\x01`@\x1B\x03\x83Q\x16\x82R`\x01`\x01`@\x1B\x03` \x84\x01Q\x16` \x83\x01R`\x01`\x01`@\x1B\x03`@\x84\x01Q\x16`@\x83\x01R``\x83\x01Qa=\xFB``\x84\x01\x82a=jV[P\x92\x91PPV[\x80\x15\x15\x81\x14a:\xA3W__\xFD[_` \x82\x84\x03\x12\x15a>\x1FW__\xFD[\x815a#E\x81a>\x02V[_____``\x86\x88\x03\x12\x15a>>W__\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15a>SW__\xFD[a>_\x88\x82\x89\x01a<\xE7V[\x90\x96P\x94PP` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a>}W__\xFD[a>\x89\x88\x82\x89\x01a<\xE7V[\x96\x99\x95\x98P\x96`@\x015\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a:\xA3W__\xFD[\x805a>\xBA\x81a>\x9BV[\x91\x90PV[__`@\x83\x85\x03\x12\x15a>\xD0W__\xFD[\x825a>\xDB\x81a>\x9BV[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a>\xF9W__\xFD[\x815a#E\x81a>\x9BV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?@Wa?@a?\x04V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a?`Wa?`a?\x04V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a?yW__\xFD[\x815a?\x8Ca?\x87\x82a?HV[a?\x18V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a?\xADW__\xFD[` \x85\x01[\x83\x81\x10\x15a?\xCAW\x805\x83R` \x92\x83\x01\x92\x01a?\xB2V[P\x95\x94PPPPPV[___``\x84\x86\x03\x12\x15a?\xE6W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xFBW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a@\x0BW__\xFD[\x805a@\x19a?\x87\x82a?HV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a@:W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a@eW\x835a@T\x81a>\x9BV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a@AV[\x95PPPP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a@\x82W__\xFD[a@\x8E\x86\x82\x87\x01a?jV[\x92PPa@\x9D`@\x85\x01a>\xAFV[\x90P\x92P\x92P\x92V[___`@\x84\x86\x03\x12\x15a@\xB8W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xCDW__\xFD[a@\xD9\x86\x82\x87\x01a:\xA6V[\x93PP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xF4W__\xFD[aA\0\x86\x82\x87\x01a;0V[\x94\x97\x90\x96P\x93\x94PPPPV[_` \x82\x84\x03\x12\x15aA\x1DW__\xFD[\x81Qa#E\x81a>\x02V[__\x835`\x1E\x19\x846\x03\x01\x81\x12aA=W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aAVW__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a;pW__\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12aA\x82W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aA\x9BW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a;pW__\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x10\xD1Wa\x10\xD1aA\xAFV[\x81\x81\x03\x81\x81\x11\x15a\x10\xD1Wa\x10\xD1aA\xAFV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12aB\x15W__\xFD[\x91\x90\x91\x01\x92\x91PPV[\x82\x84\x827`\xC0\x91\x90\x91\x1B`\x01`\x01`\xC0\x1B\x03\x19\x16\x91\x01\x90\x81R`\x08\x01\x91\x90PV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a#E\x82\x84aB@V[_` \x82\x84\x03\x12\x15aBrW__\xFD[\x815d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a#EW__\xFD[\x80\x82\x01\x80\x82\x11\x15a\x10\xD1Wa\x10\xD1aA\xAFV[\x83\x85\x827_\x84\x82\x01_\x81R\x83\x85\x827_\x93\x01\x92\x83RP\x90\x94\x93PPPPV[_` \x82\x84\x03\x12\x15aB\xC8W__\xFD[PQ\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x80\x81R_aC\n`\x80\x83\x01\x88\x8AaB\xCFV[\x82\x81\x03` \x84\x01RaC\x1C\x81\x88a<\xA7V[\x90P\x82\x81\x03`@\x84\x01RaC1\x81\x86\x88aB\xCFV[\x91PP\x82``\x83\x01R\x97\x96PPPPPPPV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aCgWaCgaCEV[P\x04\x90V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x10\xD1Wa\x10\xD1aA\xAFV[_\x825`^\x19\x836\x03\x01\x81\x12aB\x15W__\xFD[_b\xFF\xFF\xFF\x82\x16\x80aC\xB3WaC\xB3aA\xAFV[_\x19\x01\x92\x91PPV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x10\xD1Wa\x10\xD1aA\xAFV[`\x07\x81\x81\x0B\x90\x83\x90\x0B\x01g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x12\x17\x15a\x10\xD1Wa\x10\xD1aA\xAFV[_` \x82\x84\x03\x12\x15aD\x1AW__\xFD[\x81Qa#E\x81a:\x8FV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a:\xB6W_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[_aDS\x82\x85aB@V[`\x01`\x01`\x80\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x10\x01\x91\x90PV[_`\x01\x82\x01aD}WaD}aA\xAFV[P`\x01\x01\x90V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83R`\x01`\x01`\xA8\x1B\x03\x19\x91\x90\x91\x16`\x01\x83\x01R``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x0C\x82\x01R` \x01\x90V[`\x07\x82\x81\x0B\x90\x82\x90\x0B\x03g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x13\x17\x15a\x10\xD1Wa\x10\xD1aA\xAFV[_\x81aD\xFFWaD\xFFaA\xAFV[P_\x19\x01\x90V[_\x81`\x07\x0Bg\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03aE#WaE#aA\xAFV[_\x03\x92\x91PPV[\x80\x82\x02_\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15aEFWaEFaA\xAFV[\x81\x81\x05\x83\x14\x82\x15\x17a\x10\xD1Wa\x10\xD1aA\xAFV[_d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80aEpWaEpaCEV[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[_\x82aE\x91WaE\x91aCEV[P\x06\x90V[_d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80aE\xACWaE\xACaCEV[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x06\x91PP\x92\x91PPV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a=\xFBWa=\xFBaA\xAFV\xFE\xA2dipfsX\"\x12 \x90Tv\xA9\xBC\x91\x954@\xC8Mw\x0E\x9B\x89\xFD\xDDZ\x88V\xC8E@Yh\x95\\\x8C*\t\x95\xE2dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101bd575f3560e01c80636691954e116100f2578063b522538a11610092578063d06d558711610062578063d06d55871461063f578063dda3346c1461065e578063ee94d67c1461067d578063f074ba621461069c575f5ffd5b8063b522538a146105ce578063c44e30dc146105ed578063c490744214610601578063c4d66de814610620575f5ffd5b80637439841f116100cd5780637439841f1461053457806374cdd7981461056957806388676cad1461059c5780639b4e4634146105bb575f5ffd5b80636691954e146104d65780636c0d2d5a146104e95780636fcd0e5314610508575f5ffd5b806342ecff2a1161015d57806352396a591161013857806352396a591461043657806354fd4d501461046a578063587533571461048b57806358eaee79146104aa575f5ffd5b806342ecff2a146102f25780634665bcda1461031857806347d283721461034b575f5ffd5b80632340e8d3116101985780632340e8d31461027a5780633474aa161461028f5780633f5fa57a146102c05780633f65cf19146102d3575f5ffd5b8063039157d2146101fb5780630b18ff661461021c5780631e51553314610258575f5ffd5b366101f7576040513481527f6fdd3dbdb173299608c0aa9f368735857c8842b581f8389238bf05bd04b3bf499060200160405180910390a1005b5f5ffd5b348015610206575f5ffd5b5061021a610215366004613abc565b6106bb565b005b348015610227575f5ffd5b5060335461023b906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b348015610263575f5ffd5b5061026c6109f0565b60405190815260200161024f565b348015610285575f5ffd5b5061026c60395481565b34801561029a575f5ffd5b506034546001600160401b03165b6040516001600160401b03909116815260200161024f565b61021a6102ce366004613b77565b610a11565b3480156102de575f5ffd5b5061021a6102ed366004613bb5565b610d92565b3480156102fd575f5ffd5b50603a546102a890600160401b90046001600160401b031681565b348015610323575f5ffd5b5061023b7f000000000000000000000000000000000000000000000000000000000000000081565b348015610356575f5ffd5b506103db6040805160a0810182525f80825260208201819052918101829052606081018290526080810191909152506040805160a081018252603c548152603d5462ffffff811660208301526001600160401b0363010000008204811693830193909352600160581b810460070b6060830152600160981b9004909116608082015290565b60405161024f91905f60a0820190508251825262ffffff60208401511660208301526001600160401b036040840151166040830152606083015160070b60608301526001600160401b03608084015116608083015292915050565b348015610441575f5ffd5b506102a8610450366004613c8c565b603b6020525f90815260409020546001600160401b031681565b348015610475575f5ffd5b5061047e61104a565b60405161024f9190613cd5565b348015610496575f5ffd5b50603e5461023b906001600160a01b031681565b3480156104b5575f5ffd5b506104c96104c4366004613d24565b611075565b60405161024f9190613d8a565b61021a6104e4366004613b77565b6110d7565b3480156104f4575f5ffd5b5061026c610503366004613c8c565b6113df565b348015610513575f5ffd5b50610527610522366004613d98565b6114ed565b60405161024f9190613daf565b34801561053f575f5ffd5b506104c961054e366004613d98565b5f90815260366020526040902054600160c01b900460ff1690565b348015610574575f5ffd5b5061023b7f000000000000000000000000000000000000000000000000000000000000000081565b3480156105a7575f5ffd5b5061021a6105b6366004613e0f565b611598565b61021a6105c9366004613e2a565b61168d565b3480156105d9575f5ffd5b506105276105e8366004613d24565b61180b565b3480156105f8575f5ffd5b5061026c6118fa565b34801561060c575f5ffd5b5061021a61061b366004613ebf565b611916565b34801561062b575f5ffd5b5061021a61063a366004613ee9565b611a4d565b34801561064a575f5ffd5b5061021a610659366004613ee9565b611b97565b348015610669575f5ffd5b5061021a610678366004613fd4565b611c2b565b348015610688575f5ffd5b50603a546102a8906001600160401b031681565b3480156106a7575f5ffd5b5061021a6106b63660046140a6565b611d8a565b604051635ac86ab760e01b8152600660048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610721573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610745919061410d565b156107635760405163840a48d560e01b815260040160405180910390fd5b604051635ac86ab760e01b8152600860048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156107c9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107ed919061410d565b1561080b5760405163840a48d560e01b815260040160405180910390fd5b5f61084f6108198580614128565b808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061218692505050565b5f818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff1660028111156108bd576108bd613d56565b60028111156108ce576108ce613d56565b81525050905080604001516001600160401b0316876001600160401b03161161090a576040516337e07ffd60e01b815260040160405180910390fd5b60018160600151600281111561092257610922613d56565b146109405760405163d49e19a760e01b815260040160405180910390fd5b61098361094d8680614128565b808060200260200160405190810160405280939291908181526020018383602002808284375f920191909152506121a892505050565b6109a05760405163161ce5ed60e31b815260040160405180910390fd5b6109b26109ac886113df565b876121d0565b6109de6109be88612275565b87356109ca8880614128565b6109d760208b018b61416d565b875161234c565b6109e75f612480565b50505050505050565b5f610a0c71bbddc7ce488642fb579f8b00f3a590007251612600565b905090565b604051635ac86ab760e01b8152600a60048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610a77573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a9b919061410d565b15610ab95760405163840a48d560e01b815260040160405180910390fd5b6033546001600160a01b0316331480610adc5750603e546001600160a01b031633145b610af95760405163427a777960e01b815260040160405180910390fd5b5f610b026118fa565b90505f610b0f84836141c3565b905080341015610b325760405163356680b760e01b815260040160405180910390fd5b5f610b3d82346141da565b90505f5b85811015610d795736878783818110610b5c57610b5c6141ed565b9050602002810190610b6e9190614201565b90505f610bb7610b7e838061416d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061268692505050565b905060015f82815260366020526040902054600160c01b900460ff166002811115610be457610be4613d56565b14610c025760405163d49e19a760e01b815260040160405180910390fd5b5f610c0d838061416d565b610c1d6040860160208701613c8c565b604051602001610c2f9392919061421f565b60405160208183030381529060405290505f710961ef480eb55e80d19ad83579a64c0070026001600160a01b03168883604051610c6c9190614257565b5f6040518083038185875af1925050503d805f8114610ca6576040519150601f19603f3d011682016040523d82523d5f602084013e610cab565b606091505b5050905080610ccd5760405163fc52d48360e01b815260040160405180910390fd5b610cdd6040850160208601613c8c565b6001600160401b03165f03610d1b5760405183907f60d8ca014d4765a2b8b389e25714cb1cef83b574222911a01d90c1bd69d2d320905f90a2610d69565b827f8b2737bb64ab2f2dc09552dfa1c250399e6a42c7ea9f0e1c658f5d65d708ec05610d4d6040870160208801613c8c565b6040516001600160401b03909116815260200160405180910390a25b505060019092019150610b419050565b508015610d8a57610d8a3382612717565b505050505050565b6033546001600160a01b0316331480610db55750603e546001600160a01b031633145b610dd25760405163427a777960e01b815260040160405180910390fd5b604051635ac86ab760e01b8152600260048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610e38573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e5c919061410d565b15610e7a5760405163840a48d560e01b815260040160405180910390fd5b8584148015610e8857508382145b610ea5576040516343714afd60e01b815260040160405180910390fd5b603a546001600160401b03600160401b9091048116908a1611610edb576040516337e07ffd60e01b815260040160405180910390fd5b603a546001600160401b03908116908a1611610f0a576040516335e7f6b760e01b815260040160405180910390fd5b610f1c610f168a6113df565b896121d0565b5f805b87811015610fb557610fa18b8b358b8b85818110610f3f57610f3f6141ed565b9050602002016020810190610f549190614262565b8a8a86818110610f6657610f666141ed565b9050602002810190610f78919061416d565b8a8a88818110610f8a57610f8a6141ed565b9050602002810190610f9c9190614128565b61282c565b610fab9083614286565b9150600101610f1f565b5060335460405163a1ca780b60e01b81526001600160a01b0391821660048201525f6024820152604481018390527f00000000000000000000000000000000000000000000000000000000000000009091169063a1ca780b906064015f604051808303815f87803b158015611028575f5ffd5b505af115801561103a573d5f5f3e3d5ffd5b5050505050505050505050505050565b6060610a0c7f0000000000000000000000000000000000000000000000000000000000000000612d2d565b5f5f6110b584848080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061268692505050565b5f90815260366020526040902054600160c01b900460ff169150505b92915050565b604051635ac86ab760e01b8152600960048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa15801561113d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611161919061410d565b1561117f5760405163840a48d560e01b815260040160405180910390fd5b6033546001600160a01b03163314806111a25750603e546001600160a01b031633145b6111bf5760405163427a777960e01b815260040160405180910390fd5b5f6111c86109f0565b90505f6111d584836141c3565b9050803410156111f85760405163356680b760e01b815260040160405180910390fd5b5f61120382346141da565b90505f5b85811015610d795736878783818110611222576112226141ed565b90506020028101906112349190614201565b90505f611244610b7e838061416d565b90505f611257610b7e602085018561416d565b905060015f82815260366020526040902054600160c01b900460ff16600281111561128457611284613d56565b146112a25760405163d49e19a760e01b815260040160405180910390fd5b5f6112ad848061416d565b6112ba602087018761416d565b6040516020016112cd9493929190614299565b60405160208183030381529060405290505f71bbddc7ce488642fb579f8b00f3a5900072516001600160a01b0316898360405161130a9190614257565b5f6040518083038185875af1925050503d805f8114611344576040519150601f19603f3d011682016040523d82523d5f602084013e611349565b606091505b505090508061136b5760405163fc52d48360e01b815260040160405180910390fd5b8284036113a15760405184907fc97b965b92ae7fd20095fe8eb7b99f81f95f8c4adffb22a19116d8eb2846b016905f90a26113ce565b604051839085907f42f9c9db2ca443e9ec62f4588bd0c9b241065c02c2a8001ac164ae1282dc7b94905f90a35b505060019093019250611207915050565b5f6113ed611fff600c6141c3565b6114006001600160401b038416426141da565b1061141e57604051637944e66d60e11b815260040160405180910390fd5b604080516001600160401b03841660208201525f918291720f3df6d732807ef1319fb7b8bb8522d0beac02910160408051601f198184030181529082905261146591614257565b5f60405180830381855afa9150503d805f811461149d576040519150601f19603f3d011682016040523d82523d5f602084013e6114a2565b606091505b50915091508180156114b457505f8151115b6114d15760405163558ad0a360e01b815260040160405180910390fd5b808060200190518101906114e591906142b8565b949350505050565b611514604080516080810182525f8082526020820181905291810182905290606082015290565b5f82815260366020908152604091829020825160808101845281546001600160401b038082168352600160401b8204811694830194909452600160801b810490931693810193909352906060830190600160c01b900460ff16600281111561157e5761157e613d56565b600281111561158f5761158f613d56565b90525092915050565b6033546001600160a01b03163314806115bb5750603e546001600160a01b031633145b6115d85760405163427a777960e01b815260040160405180910390fd5b604051635ac86ab760e01b8152600660048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa15801561163e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611662919061410d565b156116805760405163840a48d560e01b815260040160405180910390fd5b61168982612480565b5050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146116d657604051633213a66160e21b815260040160405180910390fd5b346801bc16d674ec800000146116ff5760405163049696b360e31b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663228951186801bc16d674ec8000008787611742612d6a565b8888886040518863ffffffff1660e01b8152600401611766969594939291906142f7565b5f604051808303818588803b15801561177d575f5ffd5b505af115801561178f573d5f5f3e3d5ffd5b50505050507fa01003766d3cd97cf2ade5429690bf5d206be7fb01ef9d3a0089ecf67bc112196117f386868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061268692505050565b60405190815260200160405180910390a15050505050565b611832604080516080810182525f8082526020820181905291810182905290606082015290565b60365f61187385858080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525061268692505050565b815260208082019290925260409081015f20815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b81049094169281019290925290916060830190600160c01b900460ff1660028111156118df576118df613d56565b60028111156118f0576118f0613d56565b9052509392505050565b5f610a0c710961ef480eb55e80d19ad83579a64c007002612600565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461195f57604051633213a66160e21b815260040160405180910390fd5b5f61196e633b9aca0083614359565b9050611987633b9aca006001600160401b0383166141c3565b6034549092506001600160401b0390811690821611156119ba576040516302c6f54760e21b815260040160405180910390fd5b603480548291905f906119d79084906001600160401b031661436c565b92506101000a8154816001600160401b0302191690836001600160401b03160217905550826001600160a01b03167f8947fd2ce07ef9cc302c4e8f0461015615d91ce851564839e91cc804c2f49d8e83604051611a3691815260200190565b60405180910390a2611a488383612717565b505050565b5f54610100900460ff1615808015611a6b57505f54600160ff909116105b80611a845750303b158015611a8457505f5460ff166001145b611aec5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff191660011790558015611b0d575f805461ff0019166101001790555b6001600160a01b038216611b34576040516339b190bb60e11b815260040160405180910390fd5b603380546001600160a01b0319166001600160a01b0384161790558015611689575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b6033546001600160a01b03163314611bc25760405163719f370360e11b815260040160405180910390fd5b603e54604080516001600160a01b03928316815291831660208301527ffb8129080a19d34dceac04ba253fc50304dc86c729bd63cdca4a969ad19a5eac910160405180910390a1603e80546001600160a01b0319166001600160a01b0392909216919091179055565b6033546001600160a01b03163314611c565760405163719f370360e11b815260040160405180910390fd5b604051635ac86ab760e01b8152600560048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015611cbc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ce0919061410d565b15611cfe5760405163840a48d560e01b815260040160405180910390fd5b8251845114611d20576040516343714afd60e01b815260040160405180910390fd5b5f5b8451811015611d8357611d7b83858381518110611d4157611d416141ed565b6020026020010151878481518110611d5b57611d5b6141ed565b60200260200101516001600160a01b0316612d9a9092919063ffffffff16565b600101611d22565b5050505050565b604051635ac86ab760e01b8152600760048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015611df0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e14919061410d565b15611e325760405163840a48d560e01b815260040160405180910390fd5b603a54600160401b90046001600160401b03165f819003611e6657604051631a544f4960e01b815260040160405180910390fd5b6040805160a081018252603c548152603d5462ffffff811660208301526001600160401b0363010000008204811693830193909352600160581b810460070b6060830152600160981b90049091166080820152611ecd611ec583612275565b825188612dec565b5f805b8581101561212d5736878783818110611eeb57611eeb6141ed565b9050602002810190611efd919061438b565b80355f908152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff166002811115611f6d57611f6d613d56565b6002811115611f7e57611f7e613d56565b9052509050600181606001516002811115611f9b57611f9b613d56565b14611fa7575050612125565b856001600160401b031681604001516001600160401b031610611fcb575050612125565b5f8080611fdb848a8f3588612ead565b60208b0180519396509194509250611ff28261439f565b62ffffff1690525060808801805184919061200e9083906143bc565b6001600160401b031690525060608801805183919061202e9083906143db565b60070b90525061203e81886143bc565b85355f908152603660209081526040918290208751815492890151938901516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b0319909516919092161792909217928316821781556060880151939a50879390929091839160ff60c01b1990911668ffffffffffffffffff60801b1990911617600160c01b8360028111156120e2576120e2613d56565b021790555050604051863591506001600160401b038b16907fe4866335761a51dcaff766448ab0af6064291ee5dc94e68492bb9cd757c1e350905f90a350505050505b600101611ed0565b506001600160401b038084165f908152603b6020526040812080548493919291612159918591166143bc565b92506101000a8154816001600160401b0302191690836001600160401b031602179055506109e782612fc3565b5f815f81518110612199576121996141ed565b60200260200101519050919050565b5f816003815181106121bc576121bc6141ed565b60200260200101515f5f1b14159050919050565b6121dc600360206141c3565b6121e9602083018361416d565b905014612209576040516313717da960e21b815260040160405180910390fd5b612258612219602083018361416d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f92019190915250869250508435905060036131e3565b611689576040516309bde33960e01b815260040160405180910390fd5b5f5f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316632704351a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156122d3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906122f7919061440a565b9050806001600160401b03165f03612322576040516341a02cc960e01b815260040160405180910390fd5b806001600160401b0316836001600160401b03161115612343576001612345565b5f5b9392505050565b6008841461236d5760405163200591bd60e01b815260040160405180910390fd5b5f61237788613218565b90508061238660286001614286565b6123909190614286565b61239b9060206141c3565b83146123ba576040516313717da960e21b815260040160405180910390fd5b5f6123f68787808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061324092505050565b90505f64ffffffffff841661240d60286001614286565b600b901b17905061245786868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508d92508691508590506131e3565b612474576040516309bde33960e01b815260040160405180910390fd5b50505050505050505050565b603a54600160401b90046001600160401b0316156124b05760405162be9bc360e81b815260040160405180910390fd5b603a546001600160401b034281169116036124de576040516367db5b8b60e01b815260040160405180910390fd5b6034545f906001600160401b03166124fa633b9aca0047614359565b612504919061436c565b905081801561251a57506001600160401b038116155b15612538576040516332dea95960e21b815260040160405180910390fd5b5f6040518060a0016040528061254d426113df565b815260395462ffffff1660208201526001600160401b0380851660408301525f60608301819052608090920191909152603a805442909216600160401b026fffffffffffffffff00000000000000001990921691909117905590506125b181612fc3565b805160208083015160405162ffffff90911681526001600160401b034216917f575796133bbed337e5b39aa49a30dc2556a91e0c6c2af4b7b886ae77ebef1076910160405180910390a3505050565b5f5f5f836001600160a01b03166040515f60405180830381855afa9150503d805f8114612648576040519150601f19603f3d011682016040523d82523d5f602084013e61264d565b606091505b5091509150818015612660575080516020145b61267d5760405163c90158af60e01b815260040160405180910390fd5b6114e581614425565b5f81516030146126a957604051634f88323960e11b815260040160405180910390fd5b6040516002906126bf9084905f90602001614448565b60408051601f19818403018152908290526126d991614257565b602060405180830381855afa1580156126f4573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906110d191906142b8565b804710156127675760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e63650000006044820152606401611ae3565b5f826001600160a01b0316826040515f6040518083038185875af1925050503d805f81146127b0576040519150601f19603f3d011682016040523d82523d5f602084013e6127b5565b606091505b5050905080611a485760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d617920686176652072657665727465640000000000006064820152608401611ae3565b5f5f6128698484808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061218692505050565b5f818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff1660028111156128d7576128d7613d56565b60028111156128e8576128e8613d56565b90525090505f8160600151600281111561290457612904613d56565b14612922576040516335e09e9d60e01b815260040160405180910390fd5b6001600160401b0380166129678686808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061350e92505050565b6001600160401b03160361298e57604051631958236d60e21b815260040160405180910390fd5b6001600160401b0380166129d38686808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061353292505050565b6001600160401b0316146129fa57604051632eade63760e01b815260040160405180910390fd5b612a02612d6a565b612a0b90614425565b612a468686808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061354992505050565b1480612a9b5750612a5561355d565b612a5e90614425565b612a998686808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061354992505050565b145b612ab857604051633772dd5360e11b815260040160405180910390fd5b5f612af48686808060200260200160405190810160405280939291908181526020018383602002808284375f9201919091525061357992505050565b9050612b0d612b028c612275565b8b88888c8c8f61234c565b60398054905f612b1c8361446c565b9091555050603a545f90600160401b90046001600160401b031615612b5357603a54600160401b90046001600160401b0316612b60565b603a546001600160401b03165b6040805160808101825264ffffffffff8d1681526001600160401b03858116602083015283169181019190915290915060608101600190525f858152603660209081526040918290208351815492850151938501516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b031990951691909216179290921792831682178155606084015190929091839160ff60c01b1990911668ffffffffffffffffff60801b1990911617600160c01b836002811115612c3557612c35613d56565b021790555050603d8054849250601390612c60908490600160981b90046001600160401b03166143bc565b92506101000a8154816001600160401b0302191690836001600160401b031602179055507f101790c2993f6a4d962bd17c786126823ba1c4cf04ff4cccb2659d50fb20aee884604051612cb591815260200190565b60405180910390a1604080518581526001600160401b03838116602083015284168183015290517fcdae700d7241bc027168c53cf6f889763b0a2c88a65d77fc13a8a9fef0d8605f9181900360600190a1612d1d633b9aca006001600160401b0384166141c3565b9c9b505050505050505050505050565b60605f612d3983613590565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b604051606090612d8690600160f81b905f903090602001614484565b604051602081830303815290604052905090565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663a9059cbb60e01b179052611a489084906135b7565b5f612df684613218565b9050612e03816003614286565b612e0e9060206141c3565b612e1b602084018461416d565b905014612e3b576040516313717da960e21b815260040160405180910390fd5b6003811b600c17612e90612e52602085018561416d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508892505086359050846131e3565b611d83576040516309bde33960e01b815260040160405180910390fd5b5f5f5f866020015192505f612ec686895f01518761368a565b9050836001600160401b0316816001600160401b031614612f3a57612eeb84826144c2565b60408051873581526001600160401b038a8116602083015284168183015290519194507fcdae700d7241bc027168c53cf6f889763b0a2c88a65d77fc13a8a9fef0d8605f919081900360600190a15b6001600160401b0380821660208a0181905290881660408a01525f03612fb85760398054905f612f69836144f1565b909155505060026060890152612f7e83614506565b6040519092508535906001600160401b038916907f5ce0aa04ae51d52da6e680fbe0336d2e2432f7c3dc2d4f3193204c57b9072107905f90a35b509450945094915050565b8051603c556020810151603d805460408401516060850151608086015162ffffff9095166affffffffffffffffffffff19909316831763010000006001600160401b0393841602176fffffffffffffffffffffffffffffffff60581b1916600160581b9183169190910267ffffffffffffffff60981b191617600160981b9190941602929092179055156130545750565b60808101516034545f91613070916001600160401b03166143bc565b90505f8260600151836040015161308791906143db565b60408401516034805492935090915f906130ab9084906001600160401b03166143bc565b82546101009290920a6001600160401b03818102199093169183160217909155603a8054600160401b810483166001600160801b03199091161790555f91506130fb90633b9aca009085166141c3565b90505f613110633b9aca00600785900b61452b565b603a546040518281529192506001600160401b0316907f525408c201bc1576eb44116f6478f1c2a54775b19a043bcfdc708364f74f8e449060200160405180910390a260335460405163a1ca780b60e01b81526001600160a01b03918216600482015260248101849052604481018390527f00000000000000000000000000000000000000000000000000000000000000009091169063a1ca780b906064015f604051808303815f87803b1580156131c6575f5ffd5b505af11580156131d8573d5f5f3e3d5ffd5b505050505050505050565b5f83613202576040516329e7276760e11b815260040160405180910390fd5b8361320e868585613768565b1495945050505050565b5f8082600181111561322c5761322c613d56565b146132385760066110d1565b600592915050565b5f60018251116132635760405163f8ef036760e01b815260040160405180910390fd5b61326d825161385b565b61328a5760405163f6558f5160e01b815260040160405180910390fd5b5f600283516132999190614359565b90505f816001600160401b038111156132b4576132b4613f04565b6040519080825280602002602001820160405280156132dd578160200160208202803683370190505b5090505f5b828110156133d7576002856132f783836141c3565b81518110613307576133076141ed565b60200260200101518683600261331d91906141c3565b613328906001614286565b81518110613338576133386141ed565b602002602001015160405160200161335a929190918252602082015260400190565b60408051601f198184030181529082905261337491614257565b602060405180830381855afa15801561338f573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906133b291906142b8565b8282815181106133c4576133c46141ed565b60209081029190910101526001016132e2565b505b816001146134eb576133ec600283614359565b91505f5b828110156134e55760028261340583836141c3565b81518110613415576134156141ed565b60200260200101518383600261342b91906141c3565b613436906001614286565b81518110613446576134466141ed565b6020026020010151604051602001613468929190918252602082015260400190565b60408051601f198184030181529082905261348291614257565b602060405180830381855afa15801561349d573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906134c091906142b8565b8282815181106134d2576134d26141ed565b60209081029190910101526001016133f0565b506133d9565b805f815181106134fd576134fd6141ed565b602002602001015192505050919050565b5f6110d182600581518110613525576135256141ed565b602002602001015161387a565b5f6110d182600681518110613525576135256141ed565b5f81600181518110612199576121996141ed565b604051606090612d8690600160f91b905f903090602001614484565b5f6110d182600281518110613525576135256141ed565b5f60ff8216601f8111156110d157604051632cd44ac360e21b815260040160405180910390fd5b5f61360b826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166138e19092919063ffffffff16565b905080515f148061362b57508080602001905181019061362b919061410d565b611a485760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608401611ae3565b5f61369760266001614286565b6136a29060206141c3565b6136af604084018461416d565b9050146136cf576040516313717da960e21b815260040160405180910390fd5b5f6136db60048561455a565b64ffffffffff1690506137346136f4604085018561416d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525089925050506020860135846131e3565b613751576040516309bde33960e01b815260040160405180910390fd5b61375f8360200135856138ef565b95945050505050565b5f83515f141580156137855750602084516137839190614583565b155b6137a2576040516313717da960e21b815260040160405180910390fd5b604080516020808201909252848152905b85518111613832576137c6600285614583565b5f036137f85781515f528086015160205260208260405f60026107d05a03fa6137ed575f5ffd5b600284049350613820565b808601515f52815160205260208260405f60026107d05a03fa613819575f5ffd5b6002840493505b61382b602082614286565b90506137b3565b508215613852576040516363df817160e01b815260040160405180910390fd5b51949350505050565b5f81158015906110d157506138716001836141da565b82161592915050565b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161767ff0000000000000060889290921c919091161790565b60606114e584845f8561391b565b5f806138fc600484614596565b6139079060406145bf565b64ffffffffff1690506114e584821b61387a565b60608247101561397c5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b6064820152608401611ae3565b5f5f866001600160a01b031685876040516139979190614257565b5f6040518083038185875af1925050503d805f81146139d1576040519150601f19603f3d011682016040523d82523d5f602084013e6139d6565b606091505b50915091506139e7878383876139f2565b979650505050505050565b60608315613a605782515f03613a59576001600160a01b0385163b613a595760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401611ae3565b50816114e5565b6114e58383815115613a755781518083602001fd5b8060405162461bcd60e51b8152600401611ae39190613cd5565b6001600160401b0381168114613aa3575f5ffd5b50565b5f60408284031215613ab6575f5ffd5b50919050565b5f5f5f60608486031215613ace575f5ffd5b8335613ad981613a8f565b925060208401356001600160401b03811115613af3575f5ffd5b613aff86828701613aa6565b92505060408401356001600160401b03811115613b1a575f5ffd5b613b2686828701613aa6565b9150509250925092565b5f5f83601f840112613b40575f5ffd5b5081356001600160401b03811115613b56575f5ffd5b6020830191508360208260051b8501011115613b70575f5ffd5b9250929050565b5f5f60208385031215613b88575f5ffd5b82356001600160401b03811115613b9d575f5ffd5b613ba985828601613b30565b90969095509350505050565b5f5f5f5f5f5f5f5f60a0898b031215613bcc575f5ffd5b8835613bd781613a8f565b975060208901356001600160401b03811115613bf1575f5ffd5b613bfd8b828c01613aa6565b97505060408901356001600160401b03811115613c18575f5ffd5b613c248b828c01613b30565b90975095505060608901356001600160401b03811115613c42575f5ffd5b613c4e8b828c01613b30565b90955093505060808901356001600160401b03811115613c6c575f5ffd5b613c788b828c01613b30565b999c989b5096995094979396929594505050565b5f60208284031215613c9c575f5ffd5b813561234581613a8f565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f6123456020830184613ca7565b5f5f83601f840112613cf7575f5ffd5b5081356001600160401b03811115613d0d575f5ffd5b602083019150836020828501011115613b70575f5ffd5b5f5f60208385031215613d35575f5ffd5b82356001600160401b03811115613d4a575f5ffd5b613ba985828601613ce7565b634e487b7160e01b5f52602160045260245ffd5b60038110613d8657634e487b7160e01b5f52602160045260245ffd5b9052565b602081016110d18284613d6a565b5f60208284031215613da8575f5ffd5b5035919050565b5f6080820190506001600160401b0383511682526001600160401b0360208401511660208301526001600160401b0360408401511660408301526060830151613dfb6060840182613d6a565b5092915050565b8015158114613aa3575f5ffd5b5f60208284031215613e1f575f5ffd5b813561234581613e02565b5f5f5f5f5f60608688031215613e3e575f5ffd5b85356001600160401b03811115613e53575f5ffd5b613e5f88828901613ce7565b90965094505060208601356001600160401b03811115613e7d575f5ffd5b613e8988828901613ce7565b96999598509660400135949350505050565b6001600160a01b0381168114613aa3575f5ffd5b8035613eba81613e9b565b919050565b5f5f60408385031215613ed0575f5ffd5b8235613edb81613e9b565b946020939093013593505050565b5f60208284031215613ef9575f5ffd5b813561234581613e9b565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b0381118282101715613f4057613f40613f04565b604052919050565b5f6001600160401b03821115613f6057613f60613f04565b5060051b60200190565b5f82601f830112613f79575f5ffd5b8135613f8c613f8782613f48565b613f18565b8082825260208201915060208360051b860101925085831115613fad575f5ffd5b602085015b83811015613fca578035835260209283019201613fb2565b5095945050505050565b5f5f5f60608486031215613fe6575f5ffd5b83356001600160401b03811115613ffb575f5ffd5b8401601f8101861361400b575f5ffd5b8035614019613f8782613f48565b8082825260208201915060208360051b85010192508883111561403a575f5ffd5b6020840193505b8284101561406557833561405481613e9b565b825260209384019390910190614041565b955050505060208401356001600160401b03811115614082575f5ffd5b61408e86828701613f6a565b92505061409d60408501613eaf565b90509250925092565b5f5f5f604084860312156140b8575f5ffd5b83356001600160401b038111156140cd575f5ffd5b6140d986828701613aa6565b93505060208401356001600160401b038111156140f4575f5ffd5b61410086828701613b30565b9497909650939450505050565b5f6020828403121561411d575f5ffd5b815161234581613e02565b5f5f8335601e1984360301811261413d575f5ffd5b8301803591506001600160401b03821115614156575f5ffd5b6020019150600581901b3603821315613b70575f5ffd5b5f5f8335601e19843603018112614182575f5ffd5b8301803591506001600160401b0382111561419b575f5ffd5b602001915036819003821315613b70575f5ffd5b634e487b7160e01b5f52601160045260245ffd5b80820281158282048414176110d1576110d16141af565b818103818111156110d1576110d16141af565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e19833603018112614215575f5ffd5b9190910192915050565b8284823760c09190911b6001600160c01b0319169101908152600801919050565b5f81518060208401855e5f93019283525090919050565b5f6123458284614240565b5f60208284031215614272575f5ffd5b813564ffffffffff81168114612345575f5ffd5b808201808211156110d1576110d16141af565b838582375f8482015f8152838582375f93019283525090949350505050565b5f602082840312156142c8575f5ffd5b5051919050565b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b608081525f61430a60808301888a6142cf565b828103602084015261431c8188613ca7565b905082810360408401526143318186886142cf565b915050826060830152979650505050505050565b634e487b7160e01b5f52601260045260245ffd5b5f8261436757614367614345565b500490565b6001600160401b0382811682821603908111156110d1576110d16141af565b5f8235605e19833603018112614215575f5ffd5b5f62ffffff8216806143b3576143b36141af565b5f190192915050565b6001600160401b0381811683821601908111156110d1576110d16141af565b600781810b9083900b01677fffffffffffffff8113677fffffffffffffff19821217156110d1576110d16141af565b5f6020828403121561441a575f5ffd5b815161234581613a8f565b80516020808301519190811015613ab6575f1960209190910360031b1b16919050565b5f6144538285614240565b6001600160801b03199390931683525050601001919050565b5f6001820161447d5761447d6141af565b5060010190565b6001600160f81b03199390931683526001600160a81b031991909116600183015260601b6bffffffffffffffffffffffff1916600c82015260200190565b600782810b9082900b03677fffffffffffffff198112677fffffffffffffff821317156110d1576110d16141af565b5f816144ff576144ff6141af565b505f190190565b5f8160070b677fffffffffffffff198103614523576145236141af565b5f0392915050565b8082025f8212600160ff1b84141615614546576145466141af565b81810583148215176110d1576110d16141af565b5f64ffffffffff83168061457057614570614345565b8064ffffffffff84160491505092915050565b5f8261459157614591614345565b500690565b5f64ffffffffff8316806145ac576145ac614345565b8064ffffffffff84160691505092915050565b64ffffffffff8181168382160290811690818114613dfb57613dfb6141af56fea2646970667358221220905476a9bc91953440c84d770e9b89fddd5a8856c845405968955c8c2a0995e264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xBDW_5`\xE0\x1C\x80cf\x91\x95N\x11a\0\xF2W\x80c\xB5\"S\x8A\x11a\0\x92W\x80c\xD0mU\x87\x11a\0bW\x80c\xD0mU\x87\x14a\x06?W\x80c\xDD\xA34l\x14a\x06^W\x80c\xEE\x94\xD6|\x14a\x06}W\x80c\xF0t\xBAb\x14a\x06\x9CW__\xFD[\x80c\xB5\"S\x8A\x14a\x05\xCEW\x80c\xC4N0\xDC\x14a\x05\xEDW\x80c\xC4\x90tB\x14a\x06\x01W\x80c\xC4\xD6m\xE8\x14a\x06 W__\xFD[\x80ct9\x84\x1F\x11a\0\xCDW\x80ct9\x84\x1F\x14a\x054W\x80ct\xCD\xD7\x98\x14a\x05iW\x80c\x88gl\xAD\x14a\x05\x9CW\x80c\x9BNF4\x14a\x05\xBBW__\xFD[\x80cf\x91\x95N\x14a\x04\xD6W\x80cl\r-Z\x14a\x04\xE9W\x80co\xCD\x0ES\x14a\x05\x08W__\xFD[\x80cB\xEC\xFF*\x11a\x01]W\x80cR9jY\x11a\x018W\x80cR9jY\x14a\x046W\x80cT\xFDMP\x14a\x04jW\x80cXu3W\x14a\x04\x8BW\x80cX\xEA\xEEy\x14a\x04\xAAW__\xFD[\x80cB\xEC\xFF*\x14a\x02\xF2W\x80cFe\xBC\xDA\x14a\x03\x18W\x80cG\xD2\x83r\x14a\x03KW__\xFD[\x80c#@\xE8\xD3\x11a\x01\x98W\x80c#@\xE8\xD3\x14a\x02zW\x80c4t\xAA\x16\x14a\x02\x8FW\x80c?_\xA5z\x14a\x02\xC0W\x80c?e\xCF\x19\x14a\x02\xD3W__\xFD[\x80c\x03\x91W\xD2\x14a\x01\xFBW\x80c\x0B\x18\xFFf\x14a\x02\x1CW\x80c\x1EQU3\x14a\x02XW__\xFD[6a\x01\xF7W`@Q4\x81R\x7Fo\xDD=\xBD\xB1s)\x96\x08\xC0\xAA\x9F6\x875\x85|\x88B\xB5\x81\xF88\x928\xBF\x05\xBD\x04\xB3\xBFI\x90` \x01`@Q\x80\x91\x03\x90\xA1\0[__\xFD[4\x80\x15a\x02\x06W__\xFD[Pa\x02\x1Aa\x02\x156`\x04a:\xBCV[a\x06\xBBV[\0[4\x80\x15a\x02'W__\xFD[P`3Ta\x02;\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02cW__\xFD[Pa\x02la\t\xF0V[`@Q\x90\x81R` \x01a\x02OV[4\x80\x15a\x02\x85W__\xFD[Pa\x02l`9T\x81V[4\x80\x15a\x02\x9AW__\xFD[P`4T`\x01`\x01`@\x1B\x03\x16[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02OV[a\x02\x1Aa\x02\xCE6`\x04a;wV[a\n\x11V[4\x80\x15a\x02\xDEW__\xFD[Pa\x02\x1Aa\x02\xED6`\x04a;\xB5V[a\r\x92V[4\x80\x15a\x02\xFDW__\xFD[P`:Ta\x02\xA8\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x03#W__\xFD[Pa\x02;\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03VW__\xFD[Pa\x03\xDB`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82R`<T\x81R`=Tb\xFF\xFF\xFF\x81\x16` \x83\x01R`\x01`\x01`@\x1B\x03c\x01\0\0\0\x82\x04\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`X\x1B\x81\x04`\x07\x0B``\x83\x01R`\x01`\x98\x1B\x90\x04\x90\x91\x16`\x80\x82\x01R\x90V[`@Qa\x02O\x91\x90_`\xA0\x82\x01\x90P\x82Q\x82Rb\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`\x01`\x01`@\x1B\x03`@\x84\x01Q\x16`@\x83\x01R``\x83\x01Q`\x07\x0B``\x83\x01R`\x01`\x01`@\x1B\x03`\x80\x84\x01Q\x16`\x80\x83\x01R\x92\x91PPV[4\x80\x15a\x04AW__\xFD[Pa\x02\xA8a\x04P6`\x04a<\x8CV[`;` R_\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x04uW__\xFD[Pa\x04~a\x10JV[`@Qa\x02O\x91\x90a<\xD5V[4\x80\x15a\x04\x96W__\xFD[P`>Ta\x02;\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\xB5W__\xFD[Pa\x04\xC9a\x04\xC46`\x04a=$V[a\x10uV[`@Qa\x02O\x91\x90a=\x8AV[a\x02\x1Aa\x04\xE46`\x04a;wV[a\x10\xD7V[4\x80\x15a\x04\xF4W__\xFD[Pa\x02la\x05\x036`\x04a<\x8CV[a\x13\xDFV[4\x80\x15a\x05\x13W__\xFD[Pa\x05'a\x05\"6`\x04a=\x98V[a\x14\xEDV[`@Qa\x02O\x91\x90a=\xAFV[4\x80\x15a\x05?W__\xFD[Pa\x04\xC9a\x05N6`\x04a=\x98V[_\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x90V[4\x80\x15a\x05tW__\xFD[Pa\x02;\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\xA7W__\xFD[Pa\x02\x1Aa\x05\xB66`\x04a>\x0FV[a\x15\x98V[a\x02\x1Aa\x05\xC96`\x04a>*V[a\x16\x8DV[4\x80\x15a\x05\xD9W__\xFD[Pa\x05'a\x05\xE86`\x04a=$V[a\x18\x0BV[4\x80\x15a\x05\xF8W__\xFD[Pa\x02la\x18\xFAV[4\x80\x15a\x06\x0CW__\xFD[Pa\x02\x1Aa\x06\x1B6`\x04a>\xBFV[a\x19\x16V[4\x80\x15a\x06+W__\xFD[Pa\x02\x1Aa\x06:6`\x04a>\xE9V[a\x1AMV[4\x80\x15a\x06JW__\xFD[Pa\x02\x1Aa\x06Y6`\x04a>\xE9V[a\x1B\x97V[4\x80\x15a\x06iW__\xFD[Pa\x02\x1Aa\x06x6`\x04a?\xD4V[a\x1C+V[4\x80\x15a\x06\x88W__\xFD[P`:Ta\x02\xA8\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x06\xA7W__\xFD[Pa\x02\x1Aa\x06\xB66`\x04a@\xA6V[a\x1D\x8AV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07!W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07E\x91\x90aA\rV[\x15a\x07cW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x08`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xED\x91\x90aA\rV[\x15a\x08\x0BW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x08Oa\x08\x19\x85\x80aA(V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa!\x86\x92PPPV[_\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x08\xBDWa\x08\xBDa=VV[`\x02\x81\x11\x15a\x08\xCEWa\x08\xCEa=VV[\x81RPP\x90P\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x87`\x01`\x01`@\x1B\x03\x16\x11a\t\nW`@Qc7\xE0\x7F\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81``\x01Q`\x02\x81\x11\x15a\t\"Wa\t\"a=VV[\x14a\t@W`@Qc\xD4\x9E\x19\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\x83a\tM\x86\x80aA(V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa!\xA8\x92PPPV[a\t\xA0W`@Qc\x16\x1C\xE5\xED`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xB2a\t\xAC\x88a\x13\xDFV[\x87a!\xD0V[a\t\xDEa\t\xBE\x88a\"uV[\x875a\t\xCA\x88\x80aA(V[a\t\xD7` \x8B\x01\x8BaAmV[\x87Qa#LV[a\t\xE7_a$\x80V[PPPPPPPV[_a\n\x0Cq\xBB\xDD\xC7\xCEH\x86B\xFBW\x9F\x8B\0\xF3\xA5\x90\0rQa&\0V[\x90P\x90V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\n`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nwW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x9B\x91\x90aA\rV[\x15a\n\xB9W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\n\xDCWP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\n\xF9W`@QcBzwy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0B\x02a\x18\xFAV[\x90P_a\x0B\x0F\x84\x83aA\xC3V[\x90P\x804\x10\x15a\x0B2W`@Qc5f\x80\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0B=\x824aA\xDAV[\x90P_[\x85\x81\x10\x15a\ryW6\x87\x87\x83\x81\x81\x10a\x0B\\Wa\x0B\\aA\xEDV[\x90P` \x02\x81\x01\x90a\x0Bn\x91\x90aB\x01V[\x90P_a\x0B\xB7a\x0B~\x83\x80aAmV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa&\x86\x92PPPV[\x90P`\x01_\x82\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0B\xE4Wa\x0B\xE4a=VV[\x14a\x0C\x02W`@Qc\xD4\x9E\x19\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\r\x83\x80aAmV[a\x0C\x1D`@\x86\x01` \x87\x01a<\x8CV[`@Q` \x01a\x0C/\x93\x92\x91\x90aB\x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P_q\ta\xEFH\x0E\xB5^\x80\xD1\x9A\xD85y\xA6L\0p\x02`\x01`\x01`\xA0\x1B\x03\x16\x88\x83`@Qa\x0Cl\x91\x90aBWV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x0C\xA6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0C\xABV[``\x91P[PP\x90P\x80a\x0C\xCDW`@Qc\xFCR\xD4\x83`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\xDD`@\x85\x01` \x86\x01a<\x8CV[`\x01`\x01`@\x1B\x03\x16_\x03a\r\x1BW`@Q\x83\x90\x7F`\xD8\xCA\x01MGe\xA2\xB8\xB3\x89\xE2W\x14\xCB\x1C\xEF\x83\xB5t\")\x11\xA0\x1D\x90\xC1\xBDi\xD2\xD3 \x90_\x90\xA2a\riV[\x82\x7F\x8B'7\xBBd\xAB/-\xC0\x95R\xDF\xA1\xC2P9\x9EjB\xC7\xEA\x9F\x0E\x1Ce\x8F]e\xD7\x08\xEC\x05a\rM`@\x87\x01` \x88\x01a<\x8CV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xA2[PP`\x01\x90\x92\x01\x91Pa\x0BA\x90PV[P\x80\x15a\r\x8AWa\r\x8A3\x82a'\x17V[PPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\r\xB5WP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\r\xD2W`@QcBzwy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x02`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\\\x91\x90aA\rV[\x15a\x0EzW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x84\x14\x80\x15a\x0E\x88WP\x83\x82\x14[a\x0E\xA5W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`\x01`@\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x90\x8A\x16\x11a\x0E\xDBW`@Qc7\xE0\x7F\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x8A\x16\x11a\x0F\nW`@Qc5\xE7\xF6\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x1Ca\x0F\x16\x8Aa\x13\xDFV[\x89a!\xD0V[_\x80[\x87\x81\x10\x15a\x0F\xB5Wa\x0F\xA1\x8B\x8B5\x8B\x8B\x85\x81\x81\x10a\x0F?Wa\x0F?aA\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x0FT\x91\x90aBbV[\x8A\x8A\x86\x81\x81\x10a\x0FfWa\x0FfaA\xEDV[\x90P` \x02\x81\x01\x90a\x0Fx\x91\x90aAmV[\x8A\x8A\x88\x81\x81\x10a\x0F\x8AWa\x0F\x8AaA\xEDV[\x90P` \x02\x81\x01\x90a\x0F\x9C\x91\x90aA(V[a(,V[a\x0F\xAB\x90\x83aB\x86V[\x91P`\x01\x01a\x0F\x1FV[P`3T`@Qc\xA1\xCAx\x0B`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R_`$\x82\x01R`D\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA1\xCAx\x0B\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x10(W__\xFD[PZ\xF1\x15\x80\x15a\x10:W=__>=_\xFD[PPPPPPPPPPPPPPV[``a\n\x0C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a--V[__a\x10\xB5\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa&\x86\x92PPPV[_\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x91PP[\x92\x91PPV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\t`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11a\x91\x90aA\rV[\x15a\x11\x7FW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x11\xA2WP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\x11\xBFW`@QcBzwy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x11\xC8a\t\xF0V[\x90P_a\x11\xD5\x84\x83aA\xC3V[\x90P\x804\x10\x15a\x11\xF8W`@Qc5f\x80\xB7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x12\x03\x824aA\xDAV[\x90P_[\x85\x81\x10\x15a\ryW6\x87\x87\x83\x81\x81\x10a\x12\"Wa\x12\"aA\xEDV[\x90P` \x02\x81\x01\x90a\x124\x91\x90aB\x01V[\x90P_a\x12Da\x0B~\x83\x80aAmV[\x90P_a\x12Wa\x0B~` \x85\x01\x85aAmV[\x90P`\x01_\x82\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x12\x84Wa\x12\x84a=VV[\x14a\x12\xA2W`@Qc\xD4\x9E\x19\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x12\xAD\x84\x80aAmV[a\x12\xBA` \x87\x01\x87aAmV[`@Q` \x01a\x12\xCD\x94\x93\x92\x91\x90aB\x99V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P_q\xBB\xDD\xC7\xCEH\x86B\xFBW\x9F\x8B\0\xF3\xA5\x90\0rQ`\x01`\x01`\xA0\x1B\x03\x16\x89\x83`@Qa\x13\n\x91\x90aBWV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x13DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x13IV[``\x91P[PP\x90P\x80a\x13kW`@Qc\xFCR\xD4\x83`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x84\x03a\x13\xA1W`@Q\x84\x90\x7F\xC9{\x96[\x92\xAE\x7F\xD2\0\x95\xFE\x8E\xB7\xB9\x9F\x81\xF9_\x8CJ\xDF\xFB\"\xA1\x91\x16\xD8\xEB(F\xB0\x16\x90_\x90\xA2a\x13\xCEV[`@Q\x83\x90\x85\x90\x7FB\xF9\xC9\xDB,\xA4C\xE9\xECb\xF4X\x8B\xD0\xC9\xB2A\x06\\\x02\xC2\xA8\0\x1A\xC1d\xAE\x12\x82\xDC{\x94\x90_\x90\xA3[PP`\x01\x90\x93\x01\x92Pa\x12\x07\x91PPV[_a\x13\xEDa\x1F\xFF`\x0CaA\xC3V[a\x14\0`\x01`\x01`@\x1B\x03\x84\x16BaA\xDAV[\x10a\x14\x1EW`@QcyD\xE6m`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`@\x1B\x03\x84\x16` \x82\x01R_\x91\x82\x91r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x14e\x91aBWV[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a\x14\x9DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x14\xA2V[``\x91P[P\x91P\x91P\x81\x80\x15a\x14\xB4WP_\x81Q\x11[a\x14\xD1W`@QcU\x8A\xD0\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x80` \x01\x90Q\x81\x01\x90a\x14\xE5\x91\x90aB\xB8V[\x94\x93PPPPV[a\x15\x14`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[_\x82\x81R`6` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x81\x04\x90\x93\x16\x93\x81\x01\x93\x90\x93R\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x15~Wa\x15~a=VV[`\x02\x81\x11\x15a\x15\x8FWa\x15\x8Fa=VV[\x90RP\x92\x91PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x15\xBBWP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\x15\xD8W`@QcBzwy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16>W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16b\x91\x90aA\rV[\x15a\x16\x80W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\x89\x82a$\x80V[PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x16\xD6W`@Qc2\x13\xA6a`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[4h\x01\xBC\x16\xD6t\xEC\x80\0\0\x14a\x16\xFFW`@Qc\x04\x96\x96\xB3`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\"\x89Q\x18h\x01\xBC\x16\xD6t\xEC\x80\0\0\x87\x87a\x17Ba-jV[\x88\x88\x88`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17f\x96\x95\x94\x93\x92\x91\x90aB\xF7V[_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x17}W__\xFD[PZ\xF1\x15\x80\x15a\x17\x8FW=__>=_\xFD[PPPPP\x7F\xA0\x10\x03vm<\xD9|\xF2\xAD\xE5B\x96\x90\xBF] k\xE7\xFB\x01\xEF\x9D:\0\x89\xEC\xF6{\xC1\x12\x19a\x17\xF3\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa&\x86\x92PPPV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1PPPPPV[a\x182`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`6_a\x18s\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa&\x86\x92PPPV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x90\x91``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x18\xDFWa\x18\xDFa=VV[`\x02\x81\x11\x15a\x18\xF0Wa\x18\xF0a=VV[\x90RP\x93\x92PPPV[_a\n\x0Cq\ta\xEFH\x0E\xB5^\x80\xD1\x9A\xD85y\xA6L\0p\x02a&\0V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x19_W`@Qc2\x13\xA6a`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x19nc;\x9A\xCA\0\x83aCYV[\x90Pa\x19\x87c;\x9A\xCA\0`\x01`\x01`@\x1B\x03\x83\x16aA\xC3V[`4T\x90\x92P`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x82\x16\x11\x15a\x19\xBAW`@Qc\x02\xC6\xF5G`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`4\x80T\x82\x91\x90_\x90a\x19\xD7\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aClV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x89G\xFD,\xE0~\xF9\xCC0,N\x8F\x04a\x01V\x15\xD9\x1C\xE8QVH9\xE9\x1C\xC8\x04\xC2\xF4\x9D\x8E\x83`@Qa\x1A6\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x1AH\x83\x83a'\x17V[PPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1AkWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1A\x84WP0;\x15\x80\x15a\x1A\x84WP_T`\xFF\x16`\x01\x14[a\x1A\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1B\rW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1B4W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x16\x89W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1B\xC2W`@Qcq\x9F7\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xFB\x81)\x08\n\x19\xD3M\xCE\xAC\x04\xBA%?\xC5\x03\x04\xDC\x86\xC7)\xBDc\xCD\xCAJ\x96\x9A\xD1\x9A^\xAC\x91\x01`@Q\x80\x91\x03\x90\xA1`>\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1CVW`@Qcq\x9F7\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x05`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xBCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xE0\x91\x90aA\rV[\x15a\x1C\xFEW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q\x84Q\x14a\x1D W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84Q\x81\x10\x15a\x1D\x83Wa\x1D{\x83\x85\x83\x81Q\x81\x10a\x1DAWa\x1DAaA\xEDV[` \x02` \x01\x01Q\x87\x84\x81Q\x81\x10a\x1D[Wa\x1D[aA\xEDV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16a-\x9A\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01\x01a\x1D\"V[PPPPPV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x07`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\xF0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x14\x91\x90aA\rV[\x15a\x1E2W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16_\x81\x90\x03a\x1EfW`@Qc\x1ATOI`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R`<T\x81R`=Tb\xFF\xFF\xFF\x81\x16` \x83\x01R`\x01`\x01`@\x1B\x03c\x01\0\0\0\x82\x04\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`X\x1B\x81\x04`\x07\x0B``\x83\x01R`\x01`\x98\x1B\x90\x04\x90\x91\x16`\x80\x82\x01Ra\x1E\xCDa\x1E\xC5\x83a\"uV[\x82Q\x88a-\xECV[_\x80[\x85\x81\x10\x15a!-W6\x87\x87\x83\x81\x81\x10a\x1E\xEBWa\x1E\xEBaA\xEDV[\x90P` \x02\x81\x01\x90a\x1E\xFD\x91\x90aC\x8BV[\x805_\x90\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1FmWa\x1Fma=VV[`\x02\x81\x11\x15a\x1F~Wa\x1F~a=VV[\x90RP\x90P`\x01\x81``\x01Q`\x02\x81\x11\x15a\x1F\x9BWa\x1F\x9Ba=VV[\x14a\x1F\xA7WPPa!%V[\x85`\x01`\x01`@\x1B\x03\x16\x81`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10a\x1F\xCBWPPa!%V[_\x80\x80a\x1F\xDB\x84\x8A\x8F5\x88a.\xADV[` \x8B\x01\x80Q\x93\x96P\x91\x94P\x92Pa\x1F\xF2\x82aC\x9FV[b\xFF\xFF\xFF\x16\x90RP`\x80\x88\x01\x80Q\x84\x91\x90a \x0E\x90\x83\x90aC\xBCV[`\x01`\x01`@\x1B\x03\x16\x90RP``\x88\x01\x80Q\x83\x91\x90a .\x90\x83\x90aC\xDBV[`\x07\x0B\x90RPa >\x81\x88aC\xBCV[\x855_\x90\x81R`6` \x90\x81R`@\x91\x82\x90 \x87Q\x81T\x92\x89\x01Q\x93\x89\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x88\x01Q\x93\x9AP\x87\x93\x90\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x90\x91\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a \xE2Wa \xE2a=VV[\x02\x17\x90UPP`@Q\x865\x91P`\x01`\x01`@\x1B\x03\x8B\x16\x90\x7F\xE4\x86c5v\x1AQ\xDC\xAF\xF7fD\x8A\xB0\xAF`d)\x1E\xE5\xDC\x94\xE6\x84\x92\xBB\x9C\xD7W\xC1\xE3P\x90_\x90\xA3PPPPP[`\x01\x01a\x1E\xD0V[P`\x01`\x01`@\x1B\x03\x80\x84\x16_\x90\x81R`;` R`@\x81 \x80T\x84\x93\x91\x92\x91a!Y\x91\x85\x91\x16aC\xBCV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa\t\xE7\x82a/\xC3V[_\x81_\x81Q\x81\x10a!\x99Wa!\x99aA\xEDV[` \x02` \x01\x01Q\x90P\x91\x90PV[_\x81`\x03\x81Q\x81\x10a!\xBCWa!\xBCaA\xEDV[` \x02` \x01\x01Q__\x1B\x14\x15\x90P\x91\x90PV[a!\xDC`\x03` aA\xC3V[a!\xE9` \x83\x01\x83aAmV[\x90P\x14a\"\tW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\"Xa\"\x19` \x83\x01\x83aAmV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x86\x92PP\x845\x90P`\x03a1\xE3V[a\x16\x89W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c'\x045\x1A`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\xD3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\xF7\x91\x90aD\nV[\x90P\x80`\x01`\x01`@\x1B\x03\x16_\x03a#\"W`@QcA\xA0,\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`@\x1B\x03\x16\x83`\x01`\x01`@\x1B\x03\x16\x11\x15a#CW`\x01a#EV[_[\x93\x92PPPV[`\x08\x84\x14a#mW`@Qc \x05\x91\xBD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a#w\x88a2\x18V[\x90P\x80a#\x86`(`\x01aB\x86V[a#\x90\x91\x90aB\x86V[a#\x9B\x90` aA\xC3V[\x83\x14a#\xBAW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a#\xF6\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa2@\x92PPPV[\x90P_d\xFF\xFF\xFF\xFF\xFF\x84\x16a$\r`(`\x01aB\x86V[`\x0B\x90\x1B\x17\x90Pa$W\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x8D\x92P\x86\x91P\x85\x90Pa1\xE3V[a$tW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPV[`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15a$\xB0W`@Qb\xBE\x9B\xC3`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`\x01`@\x1B\x03B\x81\x16\x91\x16\x03a$\xDEW`@Qcg\xDB[\x8B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`4T_\x90`\x01`\x01`@\x1B\x03\x16a$\xFAc;\x9A\xCA\0GaCYV[a%\x04\x91\x90aClV[\x90P\x81\x80\x15a%\x1AWP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a%8W`@Qc2\xDE\xA9Y`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`\xA0\x01`@R\x80a%MBa\x13\xDFV[\x81R`9Tb\xFF\xFF\xFF\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x80\x85\x16`@\x83\x01R_``\x83\x01\x81\x90R`\x80\x90\x92\x01\x91\x90\x91R`:\x80TB\x90\x92\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90U\x90Pa%\xB1\x81a/\xC3V[\x80Q` \x80\x83\x01Q`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R`\x01`\x01`@\x1B\x03B\x16\x91\x7FWW\x96\x13;\xBE\xD37\xE5\xB3\x9A\xA4\x9A0\xDC%V\xA9\x1E\x0Cl*\xF4\xB7\xB8\x86\xAEw\xEB\xEF\x10v\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[___\x83`\x01`\x01`\xA0\x1B\x03\x16`@Q_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a&HW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a&MV[``\x91P[P\x91P\x91P\x81\x80\x15a&`WP\x80Q` \x14[a&}W`@Qc\xC9\x01X\xAF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14\xE5\x81aD%V[_\x81Q`0\x14a&\xA9W`@QcO\x8829`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x02\x90a&\xBF\x90\x84\x90_\x90` \x01aDHV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra&\xD9\x91aBWV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a&\xF4W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xD1\x91\x90aB\xB8V[\x80G\x10\x15a'gW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x1A\xE3V[_\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a'\xB0W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a'\xB5V[``\x91P[PP\x90P\x80a\x1AHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x1A\xE3V[__a(i\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa!\x86\x92PPPV[_\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a(\xD7Wa(\xD7a=VV[`\x02\x81\x11\x15a(\xE8Wa(\xE8a=VV[\x90RP\x90P_\x81``\x01Q`\x02\x81\x11\x15a)\x04Wa)\x04a=VV[\x14a)\"W`@Qc5\xE0\x9E\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x80\x16a)g\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa5\x0E\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x03a)\x8EW`@Qc\x19X#m`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x80\x16a)\xD3\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa52\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x14a)\xFAW`@Qc.\xAD\xE67`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a*\x02a-jV[a*\x0B\x90aD%V[a*F\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa5I\x92PPPV[\x14\x80a*\x9BWPa*Ua5]V[a*^\x90aD%V[a*\x99\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa5I\x92PPPV[\x14[a*\xB8W`@Qc7r\xDDS`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a*\xF4\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa5y\x92PPPV[\x90Pa+\ra+\x02\x8Ca\"uV[\x8B\x88\x88\x8C\x8C\x8Fa#LV[`9\x80T\x90_a+\x1C\x83aDlV[\x90\x91UPP`:T_\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15a+SW`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a+`V[`:T`\x01`\x01`@\x1B\x03\x16[`@\x80Q`\x80\x81\x01\x82Rd\xFF\xFF\xFF\xFF\xFF\x8D\x16\x81R`\x01`\x01`@\x1B\x03\x85\x81\x16` \x83\x01R\x83\x16\x91\x81\x01\x91\x90\x91R\x90\x91P``\x81\x01`\x01\x90R_\x85\x81R`6` \x90\x81R`@\x91\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x85\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x84\x01Q\x90\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x90\x91\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a,5Wa,5a=VV[\x02\x17\x90UPP`=\x80T\x84\x92P`\x13\x90a,`\x90\x84\x90`\x01`\x98\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aC\xBCV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x7F\x10\x17\x90\xC2\x99?jM\x96+\xD1|xa&\x82;\xA1\xC4\xCF\x04\xFFL\xCC\xB2e\x9DP\xFB \xAE\xE8\x84`@Qa,\xB5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1`@\x80Q\x85\x81R`\x01`\x01`@\x1B\x03\x83\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q\x7F\xCD\xAEp\rrA\xBC\x02qh\xC5<\xF6\xF8\x89v;\n,\x88\xA6]w\xFC\x13\xA8\xA9\xFE\xF0\xD8`_\x91\x81\x90\x03``\x01\x90\xA1a-\x1Dc;\x9A\xCA\0`\x01`\x01`@\x1B\x03\x84\x16aA\xC3V[\x9C\x9BPPPPPPPPPPPPV[``_a-9\x83a5\x90V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`@Q``\x90a-\x86\x90`\x01`\xF8\x1B\x90_\x900\x90` \x01aD\x84V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x1AH\x90\x84\x90a5\xB7V[_a-\xF6\x84a2\x18V[\x90Pa.\x03\x81`\x03aB\x86V[a.\x0E\x90` aA\xC3V[a.\x1B` \x84\x01\x84aAmV[\x90P\x14a.;W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x81\x1B`\x0C\x17a.\x90a.R` \x85\x01\x85aAmV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92PP\x865\x90P\x84a1\xE3V[a\x1D\x83W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___\x86` \x01Q\x92P_a.\xC6\x86\x89_\x01Q\x87a6\x8AV[\x90P\x83`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14a/:Wa.\xEB\x84\x82aD\xC2V[`@\x80Q\x875\x81R`\x01`\x01`@\x1B\x03\x8A\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q\x91\x94P\x7F\xCD\xAEp\rrA\xBC\x02qh\xC5<\xF6\xF8\x89v;\n,\x88\xA6]w\xFC\x13\xA8\xA9\xFE\xF0\xD8`_\x91\x90\x81\x90\x03``\x01\x90\xA1[`\x01`\x01`@\x1B\x03\x80\x82\x16` \x8A\x01\x81\x90R\x90\x88\x16`@\x8A\x01R_\x03a/\xB8W`9\x80T\x90_a/i\x83aD\xF1V[\x90\x91UPP`\x02``\x89\x01Ra/~\x83aE\x06V[`@Q\x90\x92P\x855\x90`\x01`\x01`@\x1B\x03\x89\x16\x90\x7F\\\xE0\xAA\x04\xAEQ\xD5-\xA6\xE6\x80\xFB\xE03m.$2\xF7\xC3\xDC-O1\x93 LW\xB9\x07!\x07\x90_\x90\xA3[P\x94P\x94P\x94\x91PPV[\x80Q`<U` \x81\x01Q`=\x80T`@\x84\x01Q``\x85\x01Q`\x80\x86\x01Qb\xFF\xFF\xFF\x90\x95\x16j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x83\x17c\x01\0\0\0`\x01`\x01`@\x1B\x03\x93\x84\x16\x02\x17o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`X\x1B\x19\x16`\x01`X\x1B\x91\x83\x16\x91\x90\x91\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x98\x1B\x19\x16\x17`\x01`\x98\x1B\x91\x90\x94\x16\x02\x92\x90\x92\x17\x90U\x15a0TWPV[`\x80\x81\x01Q`4T_\x91a0p\x91`\x01`\x01`@\x1B\x03\x16aC\xBCV[\x90P_\x82``\x01Q\x83`@\x01Qa0\x87\x91\x90aC\xDBV[`@\x84\x01Q`4\x80T\x92\x93P\x90\x91_\x90a0\xAB\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aC\xBCV[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`:\x80T`\x01`@\x1B\x81\x04\x83\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x91\x16\x17\x90U_\x91Pa0\xFB\x90c;\x9A\xCA\0\x90\x85\x16aA\xC3V[\x90P_a1\x10c;\x9A\xCA\0`\x07\x85\x90\x0BaE+V[`:T`@Q\x82\x81R\x91\x92P`\x01`\x01`@\x1B\x03\x16\x90\x7FRT\x08\xC2\x01\xBC\x15v\xEBD\x11odx\xF1\xC2\xA5Gu\xB1\x9A\x04;\xCF\xDCp\x83d\xF7O\x8ED\x90` \x01`@Q\x80\x91\x03\x90\xA2`3T`@Qc\xA1\xCAx\x0B`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`D\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xA1\xCAx\x0B\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a1\xC6W__\xFD[PZ\xF1\x15\x80\x15a1\xD8W=__>=_\xFD[PPPPPPPPPV[_\x83a2\x02W`@Qc)\xE7'g`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a2\x0E\x86\x85\x85a7hV[\x14\x95\x94PPPPPV[_\x80\x82`\x01\x81\x11\x15a2,Wa2,a=VV[\x14a28W`\x06a\x10\xD1V[`\x05\x92\x91PPV[_`\x01\x82Q\x11a2cW`@Qc\xF8\xEF\x03g`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a2m\x82Qa8[V[a2\x8AW`@Qc\xF6U\x8FQ`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\x02\x83Qa2\x99\x91\x90aCYV[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xB4Wa2\xB4a?\x04V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a2\xDDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a3\xD7W`\x02\x85a2\xF7\x83\x83aA\xC3V[\x81Q\x81\x10a3\x07Wa3\x07aA\xEDV[` \x02` \x01\x01Q\x86\x83`\x02a3\x1D\x91\x90aA\xC3V[a3(\x90`\x01aB\x86V[\x81Q\x81\x10a38Wa38aA\xEDV[` \x02` \x01\x01Q`@Q` \x01a3Z\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra3t\x91aBWV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a3\x8FW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\xB2\x91\x90aB\xB8V[\x82\x82\x81Q\x81\x10a3\xC4Wa3\xC4aA\xEDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a2\xE2V[P[\x81`\x01\x14a4\xEBWa3\xEC`\x02\x83aCYV[\x91P_[\x82\x81\x10\x15a4\xE5W`\x02\x82a4\x05\x83\x83aA\xC3V[\x81Q\x81\x10a4\x15Wa4\x15aA\xEDV[` \x02` \x01\x01Q\x83\x83`\x02a4+\x91\x90aA\xC3V[a46\x90`\x01aB\x86V[\x81Q\x81\x10a4FWa4FaA\xEDV[` \x02` \x01\x01Q`@Q` \x01a4h\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra4\x82\x91aBWV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a4\x9DW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xC0\x91\x90aB\xB8V[\x82\x82\x81Q\x81\x10a4\xD2Wa4\xD2aA\xEDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a3\xF0V[Pa3\xD9V[\x80_\x81Q\x81\x10a4\xFDWa4\xFDaA\xEDV[` \x02` \x01\x01Q\x92PPP\x91\x90PV[_a\x10\xD1\x82`\x05\x81Q\x81\x10a5%Wa5%aA\xEDV[` \x02` \x01\x01Qa8zV[_a\x10\xD1\x82`\x06\x81Q\x81\x10a5%Wa5%aA\xEDV[_\x81`\x01\x81Q\x81\x10a!\x99Wa!\x99aA\xEDV[`@Q``\x90a-\x86\x90`\x01`\xF9\x1B\x90_\x900\x90` \x01aD\x84V[_a\x10\xD1\x82`\x02\x81Q\x81\x10a5%Wa5%aA\xEDV[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x10\xD1W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a6\x0B\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a8\xE1\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q_\x14\x80a6+WP\x80\x80` \x01\x90Q\x81\x01\x90a6+\x91\x90aA\rV[a\x1AHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x1A\xE3V[_a6\x97`&`\x01aB\x86V[a6\xA2\x90` aA\xC3V[a6\xAF`@\x84\x01\x84aAmV[\x90P\x14a6\xCFW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a6\xDB`\x04\x85aEZV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa74a6\xF4`@\x85\x01\x85aAmV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x89\x92PPP` \x86\x015\x84a1\xE3V[a7QW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a7_\x83` \x015\x85a8\xEFV[\x95\x94PPPPPV[_\x83Q_\x14\x15\x80\x15a7\x85WP` \x84Qa7\x83\x91\x90aE\x83V[\x15[a7\xA2W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82\x01\x90\x92R\x84\x81R\x90[\x85Q\x81\x11a82Wa7\xC6`\x02\x85aE\x83V[_\x03a7\xF8W\x81Q_R\x80\x86\x01Q` R` \x82`@_`\x02a\x07\xD0Z\x03\xFAa7\xEDW__\xFD[`\x02\x84\x04\x93Pa8 V[\x80\x86\x01Q_R\x81Q` R` \x82`@_`\x02a\x07\xD0Z\x03\xFAa8\x19W__\xFD[`\x02\x84\x04\x93P[a8+` \x82aB\x86V[\x90Pa7\xB3V[P\x82\x15a8RW`@Qcc\xDF\x81q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[Q\x94\x93PPPPV[_\x81\x15\x80\x15\x90a\x10\xD1WPa8q`\x01\x83aA\xDAV[\x82\x16\x15\x92\x91PPV[`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17g\xFF\0\0\0\0\0\0\0`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[``a\x14\xE5\x84\x84_\x85a9\x1BV[_\x80a8\xFC`\x04\x84aE\x96V[a9\x07\x90`@aE\xBFV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x14\xE5\x84\x82\x1Ba8zV[``\x82G\x10\x15a9|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x1A\xE3V[__\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa9\x97\x91\x90aBWV[_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a9\xD1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a9\xD6V[``\x91P[P\x91P\x91Pa9\xE7\x87\x83\x83\x87a9\xF2V[\x97\x96PPPPPPPV[``\x83\x15a:`W\x82Q_\x03a:YW`\x01`\x01`\xA0\x1B\x03\x85\x16;a:YW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x1A\xE3V[P\x81a\x14\xE5V[a\x14\xE5\x83\x83\x81Q\x15a:uW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x1A\xE3\x91\x90a<\xD5V[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a:\xA3W__\xFD[PV[_`@\x82\x84\x03\x12\x15a:\xB6W__\xFD[P\x91\x90PV[___``\x84\x86\x03\x12\x15a:\xCEW__\xFD[\x835a:\xD9\x81a:\x8FV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a:\xF3W__\xFD[a:\xFF\x86\x82\x87\x01a:\xA6V[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a;\x1AW__\xFD[a;&\x86\x82\x87\x01a:\xA6V[\x91PP\x92P\x92P\x92V[__\x83`\x1F\x84\x01\x12a;@W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a;VW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a;pW__\xFD[\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a;\x88W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a;\x9DW__\xFD[a;\xA9\x85\x82\x86\x01a;0V[\x90\x96\x90\x95P\x93PPPPV[________`\xA0\x89\x8B\x03\x12\x15a;\xCCW__\xFD[\x885a;\xD7\x81a:\x8FV[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a;\xF1W__\xFD[a;\xFD\x8B\x82\x8C\x01a:\xA6V[\x97PP`@\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<\x18W__\xFD[a<$\x8B\x82\x8C\x01a;0V[\x90\x97P\x95PP``\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<BW__\xFD[a<N\x8B\x82\x8C\x01a;0V[\x90\x95P\x93PP`\x80\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<lW__\xFD[a<x\x8B\x82\x8C\x01a;0V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[_` \x82\x84\x03\x12\x15a<\x9CW__\xFD[\x815a#E\x81a:\x8FV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a#E` \x83\x01\x84a<\xA7V[__\x83`\x1F\x84\x01\x12a<\xF7W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a=\rW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a;pW__\xFD[__` \x83\x85\x03\x12\x15a=5W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a=JW__\xFD[a;\xA9\x85\x82\x86\x01a<\xE7V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10a=\x86WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[` \x81\x01a\x10\xD1\x82\x84a=jV[_` \x82\x84\x03\x12\x15a=\xA8W__\xFD[P5\x91\x90PV[_`\x80\x82\x01\x90P`\x01`\x01`@\x1B\x03\x83Q\x16\x82R`\x01`\x01`@\x1B\x03` \x84\x01Q\x16` \x83\x01R`\x01`\x01`@\x1B\x03`@\x84\x01Q\x16`@\x83\x01R``\x83\x01Qa=\xFB``\x84\x01\x82a=jV[P\x92\x91PPV[\x80\x15\x15\x81\x14a:\xA3W__\xFD[_` \x82\x84\x03\x12\x15a>\x1FW__\xFD[\x815a#E\x81a>\x02V[_____``\x86\x88\x03\x12\x15a>>W__\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15a>SW__\xFD[a>_\x88\x82\x89\x01a<\xE7V[\x90\x96P\x94PP` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a>}W__\xFD[a>\x89\x88\x82\x89\x01a<\xE7V[\x96\x99\x95\x98P\x96`@\x015\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a:\xA3W__\xFD[\x805a>\xBA\x81a>\x9BV[\x91\x90PV[__`@\x83\x85\x03\x12\x15a>\xD0W__\xFD[\x825a>\xDB\x81a>\x9BV[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15a>\xF9W__\xFD[\x815a#E\x81a>\x9BV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?@Wa?@a?\x04V[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a?`Wa?`a?\x04V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a?yW__\xFD[\x815a?\x8Ca?\x87\x82a?HV[a?\x18V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a?\xADW__\xFD[` \x85\x01[\x83\x81\x10\x15a?\xCAW\x805\x83R` \x92\x83\x01\x92\x01a?\xB2V[P\x95\x94PPPPPV[___``\x84\x86\x03\x12\x15a?\xE6W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a?\xFBW__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a@\x0BW__\xFD[\x805a@\x19a?\x87\x82a?HV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a@:W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a@eW\x835a@T\x81a>\x9BV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a@AV[\x95PPPP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a@\x82W__\xFD[a@\x8E\x86\x82\x87\x01a?jV[\x92PPa@\x9D`@\x85\x01a>\xAFV[\x90P\x92P\x92P\x92V[___`@\x84\x86\x03\x12\x15a@\xB8W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xCDW__\xFD[a@\xD9\x86\x82\x87\x01a:\xA6V[\x93PP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xF4W__\xFD[aA\0\x86\x82\x87\x01a;0V[\x94\x97\x90\x96P\x93\x94PPPPV[_` \x82\x84\x03\x12\x15aA\x1DW__\xFD[\x81Qa#E\x81a>\x02V[__\x835`\x1E\x19\x846\x03\x01\x81\x12aA=W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aAVW__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a;pW__\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12aA\x82W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aA\x9BW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a;pW__\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x10\xD1Wa\x10\xD1aA\xAFV[\x81\x81\x03\x81\x81\x11\x15a\x10\xD1Wa\x10\xD1aA\xAFV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12aB\x15W__\xFD[\x91\x90\x91\x01\x92\x91PPV[\x82\x84\x827`\xC0\x91\x90\x91\x1B`\x01`\x01`\xC0\x1B\x03\x19\x16\x91\x01\x90\x81R`\x08\x01\x91\x90PV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a#E\x82\x84aB@V[_` \x82\x84\x03\x12\x15aBrW__\xFD[\x815d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a#EW__\xFD[\x80\x82\x01\x80\x82\x11\x15a\x10\xD1Wa\x10\xD1aA\xAFV[\x83\x85\x827_\x84\x82\x01_\x81R\x83\x85\x827_\x93\x01\x92\x83RP\x90\x94\x93PPPPV[_` \x82\x84\x03\x12\x15aB\xC8W__\xFD[PQ\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x80\x81R_aC\n`\x80\x83\x01\x88\x8AaB\xCFV[\x82\x81\x03` \x84\x01RaC\x1C\x81\x88a<\xA7V[\x90P\x82\x81\x03`@\x84\x01RaC1\x81\x86\x88aB\xCFV[\x91PP\x82``\x83\x01R\x97\x96PPPPPPPV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aCgWaCgaCEV[P\x04\x90V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x10\xD1Wa\x10\xD1aA\xAFV[_\x825`^\x19\x836\x03\x01\x81\x12aB\x15W__\xFD[_b\xFF\xFF\xFF\x82\x16\x80aC\xB3WaC\xB3aA\xAFV[_\x19\x01\x92\x91PPV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x10\xD1Wa\x10\xD1aA\xAFV[`\x07\x81\x81\x0B\x90\x83\x90\x0B\x01g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x12\x17\x15a\x10\xD1Wa\x10\xD1aA\xAFV[_` \x82\x84\x03\x12\x15aD\x1AW__\xFD[\x81Qa#E\x81a:\x8FV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a:\xB6W_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[_aDS\x82\x85aB@V[`\x01`\x01`\x80\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x10\x01\x91\x90PV[_`\x01\x82\x01aD}WaD}aA\xAFV[P`\x01\x01\x90V[`\x01`\x01`\xF8\x1B\x03\x19\x93\x90\x93\x16\x83R`\x01`\x01`\xA8\x1B\x03\x19\x91\x90\x91\x16`\x01\x83\x01R``\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x0C\x82\x01R` \x01\x90V[`\x07\x82\x81\x0B\x90\x82\x90\x0B\x03g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x13\x17\x15a\x10\xD1Wa\x10\xD1aA\xAFV[_\x81aD\xFFWaD\xFFaA\xAFV[P_\x19\x01\x90V[_\x81`\x07\x0Bg\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03aE#WaE#aA\xAFV[_\x03\x92\x91PPV[\x80\x82\x02_\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15aEFWaEFaA\xAFV[\x81\x81\x05\x83\x14\x82\x15\x17a\x10\xD1Wa\x10\xD1aA\xAFV[_d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80aEpWaEpaCEV[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[_\x82aE\x91WaE\x91aCEV[P\x06\x90V[_d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80aE\xACWaE\xACaCEV[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x06\x91PP\x92\x91PPV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a=\xFBWa=\xFBaA\xAFV\xFE\xA2dipfsX\"\x12 \x90Tv\xA9\xBC\x91\x954@\xC8Mw\x0E\x9B\x89\xFD\xDDZ\x88V\xC8E@Yh\x95\\\x8C*\t\x95\xE2dsolcC\0\x08\x1B\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BeaconTimestampBeforeLatestCheckpoint()` and selector `0x35e7f6b7`.
    ```solidity
    error BeaconTimestampBeforeLatestCheckpoint();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BeaconTimestampBeforeLatestCheckpoint;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BeaconTimestampBeforeLatestCheckpoint> for UnderlyingRustTuple<'_> {
            fn from(value: BeaconTimestampBeforeLatestCheckpoint) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BeaconTimestampBeforeLatestCheckpoint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BeaconTimestampBeforeLatestCheckpoint {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BeaconTimestampBeforeLatestCheckpoint()";
            const SELECTOR: [u8; 4] = [53u8, 231u8, 246u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `BeaconTimestampTooFarInPast()` and selector `0x37e07ffd`.
    ```solidity
    error BeaconTimestampTooFarInPast();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BeaconTimestampTooFarInPast;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<BeaconTimestampTooFarInPast> for UnderlyingRustTuple<'_> {
            fn from(value: BeaconTimestampTooFarInPast) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BeaconTimestampTooFarInPast {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BeaconTimestampTooFarInPast {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BeaconTimestampTooFarInPast()";
            const SELECTOR: [u8; 4] = [55u8, 224u8, 127u8, 253u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `CannotCheckpointTwiceInSingleBlock()` and selector `0x67db5b8b`.
    ```solidity
    error CannotCheckpointTwiceInSingleBlock();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotCheckpointTwiceInSingleBlock;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CannotCheckpointTwiceInSingleBlock> for UnderlyingRustTuple<'_> {
            fn from(value: CannotCheckpointTwiceInSingleBlock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotCheckpointTwiceInSingleBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotCheckpointTwiceInSingleBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotCheckpointTwiceInSingleBlock()";
            const SELECTOR: [u8; 4] = [103u8, 219u8, 91u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `CheckpointAlreadyActive()` and selector `0xbe9bc300`.
    ```solidity
    error CheckpointAlreadyActive();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckpointAlreadyActive;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CheckpointAlreadyActive> for UnderlyingRustTuple<'_> {
            fn from(value: CheckpointAlreadyActive) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CheckpointAlreadyActive {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CheckpointAlreadyActive {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CheckpointAlreadyActive()";
            const SELECTOR: [u8; 4] = [190u8, 155u8, 195u8, 0u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `CredentialsAlreadyVerified()` and selector `0x35e09e9d`.
    ```solidity
    error CredentialsAlreadyVerified();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CredentialsAlreadyVerified;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CredentialsAlreadyVerified> for UnderlyingRustTuple<'_> {
            fn from(value: CredentialsAlreadyVerified) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CredentialsAlreadyVerified {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CredentialsAlreadyVerified {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CredentialsAlreadyVerified()";
            const SELECTOR: [u8; 4] = [53u8, 224u8, 158u8, 157u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `CurrentlyPaused()` and selector `0x840a48d5`.
    ```solidity
    error CurrentlyPaused();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CurrentlyPaused;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CurrentlyPaused> for UnderlyingRustTuple<'_> {
            fn from(value: CurrentlyPaused) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CurrentlyPaused {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CurrentlyPaused {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CurrentlyPaused()";
            const SELECTOR: [u8; 4] = [132u8, 10u8, 72u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `EmptyRoot()` and selector `0x53ce4ece`.
    ```solidity
    error EmptyRoot();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyRoot;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<EmptyRoot> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyRoot) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyRoot()";
            const SELECTOR: [u8; 4] = [83u8, 206u8, 78u8, 206u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `FeeQueryFailed()` and selector `0xc90158af`.
    ```solidity
    error FeeQueryFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FeeQueryFailed;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<FeeQueryFailed> for UnderlyingRustTuple<'_> {
            fn from(value: FeeQueryFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FeeQueryFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeQueryFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeQueryFailed()";
            const SELECTOR: [u8; 4] = [201u8, 1u8, 88u8, 175u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ForkTimestampZero()` and selector `0x41a02cc9`.
    ```solidity
    error ForkTimestampZero();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ForkTimestampZero;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ForkTimestampZero> for UnderlyingRustTuple<'_> {
            fn from(value: ForkTimestampZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ForkTimestampZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ForkTimestampZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ForkTimestampZero()";
            const SELECTOR: [u8; 4] = [65u8, 160u8, 44u8, 201u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputAddressZero()` and selector `0x73632176`.
    ```solidity
    error InputAddressZero();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputAddressZero;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputAddressZero> for UnderlyingRustTuple<'_> {
            fn from(value: InputAddressZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputAddressZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputAddressZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputAddressZero()";
            const SELECTOR: [u8; 4] = [115u8, 99u8, 33u8, 118u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InputArrayLengthMismatch()` and selector `0x43714afd`.
    ```solidity
    error InputArrayLengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputArrayLengthMismatch;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InputArrayLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: InputArrayLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputArrayLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputArrayLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputArrayLengthMismatch()";
            const SELECTOR: [u8; 4] = [67u8, 113u8, 74u8, 253u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InsufficientFunds()` and selector `0x356680b7`.
    ```solidity
    error InsufficientFunds();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientFunds;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientFunds> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientFunds) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientFunds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientFunds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientFunds()";
            const SELECTOR: [u8; 4] = [53u8, 102u8, 128u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InsufficientWithdrawableBalance()` and selector `0x0b1bd51c`.
    ```solidity
    error InsufficientWithdrawableBalance();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientWithdrawableBalance;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientWithdrawableBalance> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientWithdrawableBalance) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientWithdrawableBalance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientWithdrawableBalance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientWithdrawableBalance()";
            const SELECTOR: [u8; 4] = [11u8, 27u8, 213u8, 28u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidEIP4788Response()` and selector `0x558ad0a3`.
    ```solidity
    error InvalidEIP4788Response();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidEIP4788Response;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidEIP4788Response> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidEIP4788Response) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidEIP4788Response {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidEIP4788Response {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidEIP4788Response()";
            const SELECTOR: [u8; 4] = [85u8, 138u8, 208u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidIndex()` and selector `0x63df8171`.
    ```solidity
    error InvalidIndex();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidIndex;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidIndex> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidIndex()";
            const SELECTOR: [u8; 4] = [99u8, 223u8, 129u8, 113u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidProof()` and selector `0x09bde339`.
    ```solidity
    error InvalidProof();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidProof;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidProof> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidProof()";
            const SELECTOR: [u8; 4] = [9u8, 189u8, 227u8, 57u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidProofLength()` and selector `0x4dc5f6a4`.
    ```solidity
    error InvalidProofLength();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidProofLength;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidProofLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidProofLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidProofLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidProofLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidProofLength()";
            const SELECTOR: [u8; 4] = [77u8, 197u8, 246u8, 164u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidPubKeyLength()` and selector `0x9f106472`.
    ```solidity
    error InvalidPubKeyLength();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidPubKeyLength;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidPubKeyLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPubKeyLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPubKeyLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPubKeyLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPubKeyLength()";
            const SELECTOR: [u8; 4] = [159u8, 16u8, 100u8, 114u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidShortString()` and selector `0xb3512b0c`.
    ```solidity
    error InvalidShortString();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidShortString;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidShortString> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidShortString) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidShortString {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidShortString {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidShortString()";
            const SELECTOR: [u8; 4] = [179u8, 81u8, 43u8, 12u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `InvalidValidatorFieldsLength()` and selector `0x200591bd`.
    ```solidity
    error InvalidValidatorFieldsLength();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidValidatorFieldsLength;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidValidatorFieldsLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidValidatorFieldsLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidValidatorFieldsLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidValidatorFieldsLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidValidatorFieldsLength()";
            const SELECTOR: [u8; 4] = [32u8, 5u8, 145u8, 189u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `LeavesNotPowerOfTwo()` and selector `0xf6558f51`.
    ```solidity
    error LeavesNotPowerOfTwo();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LeavesNotPowerOfTwo;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<LeavesNotPowerOfTwo> for UnderlyingRustTuple<'_> {
            fn from(value: LeavesNotPowerOfTwo) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LeavesNotPowerOfTwo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LeavesNotPowerOfTwo {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LeavesNotPowerOfTwo()";
            const SELECTOR: [u8; 4] = [246u8, 85u8, 143u8, 81u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `MsgValueNot32ETH()` and selector `0x24b4b598`.
    ```solidity
    error MsgValueNot32ETH();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MsgValueNot32ETH;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MsgValueNot32ETH> for UnderlyingRustTuple<'_> {
            fn from(value: MsgValueNot32ETH) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MsgValueNot32ETH {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MsgValueNot32ETH {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MsgValueNot32ETH()";
            const SELECTOR: [u8; 4] = [36u8, 180u8, 181u8, 152u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NoActiveCheckpoint()` and selector `0x1a544f49`.
    ```solidity
    error NoActiveCheckpoint();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoActiveCheckpoint;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NoActiveCheckpoint> for UnderlyingRustTuple<'_> {
            fn from(value: NoActiveCheckpoint) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoActiveCheckpoint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoActiveCheckpoint {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoActiveCheckpoint()";
            const SELECTOR: [u8; 4] = [26u8, 84u8, 79u8, 73u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NoBalanceToCheckpoint()` and selector `0xcb7aa564`.
    ```solidity
    error NoBalanceToCheckpoint();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoBalanceToCheckpoint;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NoBalanceToCheckpoint> for UnderlyingRustTuple<'_> {
            fn from(value: NoBalanceToCheckpoint) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoBalanceToCheckpoint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoBalanceToCheckpoint {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoBalanceToCheckpoint()";
            const SELECTOR: [u8; 4] = [203u8, 122u8, 165u8, 100u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `NotEnoughLeaves()` and selector `0xf8ef0367`.
    ```solidity
    error NotEnoughLeaves();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotEnoughLeaves;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NotEnoughLeaves> for UnderlyingRustTuple<'_> {
            fn from(value: NotEnoughLeaves) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotEnoughLeaves {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotEnoughLeaves {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotEnoughLeaves()";
            const SELECTOR: [u8; 4] = [248u8, 239u8, 3u8, 103u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlyEigenPodManager()` and selector `0xc84e9984`.
    ```solidity
    error OnlyEigenPodManager();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyEigenPodManager;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyEigenPodManager> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyEigenPodManager) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyEigenPodManager {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyEigenPodManager {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyEigenPodManager()";
            const SELECTOR: [u8; 4] = [200u8, 78u8, 153u8, 132u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlyEigenPodOwner()` and selector `0xe33e6e06`.
    ```solidity
    error OnlyEigenPodOwner();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyEigenPodOwner;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyEigenPodOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyEigenPodOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyEigenPodOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyEigenPodOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyEigenPodOwner()";
            const SELECTOR: [u8; 4] = [227u8, 62u8, 110u8, 6u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `OnlyEigenPodOwnerOrProofSubmitter()` and selector `0x427a7779`.
    ```solidity
    error OnlyEigenPodOwnerOrProofSubmitter();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyEigenPodOwnerOrProofSubmitter;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OnlyEigenPodOwnerOrProofSubmitter> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyEigenPodOwnerOrProofSubmitter) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyEigenPodOwnerOrProofSubmitter {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyEigenPodOwnerOrProofSubmitter {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyEigenPodOwnerOrProofSubmitter()";
            const SELECTOR: [u8; 4] = [66u8, 122u8, 119u8, 121u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `PredeployFailed()` and selector `0xfc52d483`.
    ```solidity
    error PredeployFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PredeployFailed;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PredeployFailed> for UnderlyingRustTuple<'_> {
            fn from(value: PredeployFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PredeployFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PredeployFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PredeployFailed()";
            const SELECTOR: [u8; 4] = [252u8, 82u8, 212u8, 131u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `StringTooLong(string)` and selector `0x305a27a9`.
    ```solidity
    error StringTooLong(string str);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StringTooLong {
        #[allow(missing_docs)]
        pub str: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StringTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: StringTooLong) -> Self {
                (value.str,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StringTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { str: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StringTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StringTooLong(string)";
            const SELECTOR: [u8; 4] = [48u8, 90u8, 39u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.str,
                    ),
                )
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `TimestampOutOfRange()` and selector `0xf289ccda`.
    ```solidity
    error TimestampOutOfRange();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimestampOutOfRange;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TimestampOutOfRange> for UnderlyingRustTuple<'_> {
            fn from(value: TimestampOutOfRange) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TimestampOutOfRange {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimestampOutOfRange {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimestampOutOfRange()";
            const SELECTOR: [u8; 4] = [242u8, 137u8, 204u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ValidatorInactiveOnBeaconChain()` and selector `0x65608db4`.
    ```solidity
    error ValidatorInactiveOnBeaconChain();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorInactiveOnBeaconChain;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ValidatorInactiveOnBeaconChain> for UnderlyingRustTuple<'_> {
            fn from(value: ValidatorInactiveOnBeaconChain) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidatorInactiveOnBeaconChain {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ValidatorInactiveOnBeaconChain {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ValidatorInactiveOnBeaconChain()";
            const SELECTOR: [u8; 4] = [101u8, 96u8, 141u8, 180u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ValidatorIsExitingBeaconChain()` and selector `0x2eade637`.
    ```solidity
    error ValidatorIsExitingBeaconChain();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorIsExitingBeaconChain;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ValidatorIsExitingBeaconChain> for UnderlyingRustTuple<'_> {
            fn from(value: ValidatorIsExitingBeaconChain) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidatorIsExitingBeaconChain {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ValidatorIsExitingBeaconChain {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ValidatorIsExitingBeaconChain()";
            const SELECTOR: [u8; 4] = [46u8, 173u8, 230u8, 55u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ValidatorNotActiveInPod()` and selector `0xd49e19a7`.
    ```solidity
    error ValidatorNotActiveInPod();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorNotActiveInPod;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ValidatorNotActiveInPod> for UnderlyingRustTuple<'_> {
            fn from(value: ValidatorNotActiveInPod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidatorNotActiveInPod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ValidatorNotActiveInPod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ValidatorNotActiveInPod()";
            const SELECTOR: [u8; 4] = [212u8, 158u8, 25u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `ValidatorNotSlashedOnBeaconChain()` and selector `0xb0e72f68`.
    ```solidity
    error ValidatorNotSlashedOnBeaconChain();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorNotSlashedOnBeaconChain;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ValidatorNotSlashedOnBeaconChain> for UnderlyingRustTuple<'_> {
            fn from(value: ValidatorNotSlashedOnBeaconChain) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidatorNotSlashedOnBeaconChain {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ValidatorNotSlashedOnBeaconChain {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ValidatorNotSlashedOnBeaconChain()";
            const SELECTOR: [u8; 4] = [176u8, 231u8, 47u8, 104u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Custom error with signature `WithdrawalCredentialsNotForEigenPod()` and selector `0x6ee5baa6`.
    ```solidity
    error WithdrawalCredentialsNotForEigenPod();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalCredentialsNotForEigenPod;
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<WithdrawalCredentialsNotForEigenPod> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalCredentialsNotForEigenPod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawalCredentialsNotForEigenPod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawalCredentialsNotForEigenPod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawalCredentialsNotForEigenPod()";
            const SELECTOR: [u8; 4] = [110u8, 229u8, 186u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_raw_validate(data: &[u8]) -> alloy_sol_types::Result<Self> {
                <Self::Parameters<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Self::new)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CheckpointCreated(uint64,bytes32,uint256)` and selector `0x575796133bbed337e5b39aa49a30dc2556a91e0c6c2af4b7b886ae77ebef1076`.
    ```solidity
    event CheckpointCreated(uint64 indexed checkpointTimestamp, bytes32 indexed beaconBlockRoot, uint256 validatorCount);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CheckpointCreated {
        #[allow(missing_docs)]
        pub checkpointTimestamp: u64,
        #[allow(missing_docs)]
        pub beaconBlockRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub validatorCount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for CheckpointCreated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "CheckpointCreated(uint64,bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    87u8, 87u8, 150u8, 19u8, 59u8, 190u8, 211u8, 55u8, 229u8, 179u8, 154u8, 164u8,
                    154u8, 48u8, 220u8, 37u8, 86u8, 169u8, 30u8, 12u8, 108u8, 42u8, 244u8, 183u8,
                    184u8, 134u8, 174u8, 119u8, 235u8, 239u8, 16u8, 118u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    checkpointTimestamp: topics.1,
                    beaconBlockRoot: topics.2,
                    validatorCount: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorCount,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.checkpointTimestamp.clone(),
                    self.beaconBlockRoot.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.checkpointTimestamp,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.beaconBlockRoot);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CheckpointCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CheckpointCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CheckpointCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `CheckpointFinalized(uint64,int256)` and selector `0x525408c201bc1576eb44116f6478f1c2a54775b19a043bcfdc708364f74f8e44`.
    ```solidity
    event CheckpointFinalized(uint64 indexed checkpointTimestamp, int256 totalShareDeltaWei);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CheckpointFinalized {
        #[allow(missing_docs)]
        pub checkpointTimestamp: u64,
        #[allow(missing_docs)]
        pub totalShareDeltaWei: alloy::sol_types::private::primitives::aliases::I256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for CheckpointFinalized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "CheckpointFinalized(uint64,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    82u8, 84u8, 8u8, 194u8, 1u8, 188u8, 21u8, 118u8, 235u8, 68u8, 17u8, 111u8,
                    100u8, 120u8, 241u8, 194u8, 165u8, 71u8, 117u8, 177u8, 154u8, 4u8, 59u8, 207u8,
                    220u8, 112u8, 131u8, 100u8, 247u8, 79u8, 142u8, 68u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    checkpointTimestamp: topics.1,
                    totalShareDeltaWei: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Int<256> as alloy_sol_types::SolType>::tokenize(
                        &self.totalShareDeltaWei,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.checkpointTimestamp.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.checkpointTimestamp,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CheckpointFinalized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CheckpointFinalized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CheckpointFinalized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ConsolidationRequested(bytes32,bytes32)` and selector `0x42f9c9db2ca443e9ec62f4588bd0c9b241065c02c2a8001ac164ae1282dc7b94`.
    ```solidity
    event ConsolidationRequested(bytes32 indexed sourcePubkeyHash, bytes32 indexed targetPubkeyHash);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ConsolidationRequested {
        #[allow(missing_docs)]
        pub sourcePubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub targetPubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ConsolidationRequested {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ConsolidationRequested(bytes32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    66u8, 249u8, 201u8, 219u8, 44u8, 164u8, 67u8, 233u8, 236u8, 98u8, 244u8, 88u8,
                    139u8, 208u8, 201u8, 178u8, 65u8, 6u8, 92u8, 2u8, 194u8, 168u8, 0u8, 26u8,
                    193u8, 100u8, 174u8, 18u8, 130u8, 220u8, 123u8, 148u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    sourcePubkeyHash: topics.1,
                    targetPubkeyHash: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.sourcePubkeyHash.clone(),
                    self.targetPubkeyHash.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.sourcePubkeyHash);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.targetPubkeyHash);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ConsolidationRequested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ConsolidationRequested> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ConsolidationRequested) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `EigenPodStaked(bytes32)` and selector `0xa01003766d3cd97cf2ade5429690bf5d206be7fb01ef9d3a0089ecf67bc11219`.
    ```solidity
    event EigenPodStaked(bytes32 pubkeyHash);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EigenPodStaked {
        #[allow(missing_docs)]
        pub pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for EigenPodStaked {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EigenPodStaked(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    160u8, 16u8, 3u8, 118u8, 109u8, 60u8, 217u8, 124u8, 242u8, 173u8, 229u8, 66u8,
                    150u8, 144u8, 191u8, 93u8, 32u8, 107u8, 231u8, 251u8, 1u8, 239u8, 157u8, 58u8,
                    0u8, 137u8, 236u8, 246u8, 123u8, 193u8, 18u8, 25u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { pubkeyHash: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.pubkeyHash),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for EigenPodStaked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EigenPodStaked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EigenPodStaked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ExitRequested(bytes32)` and selector `0x60d8ca014d4765a2b8b389e25714cb1cef83b574222911a01d90c1bd69d2d320`.
    ```solidity
    event ExitRequested(bytes32 indexed validatorPubkeyHash);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ExitRequested {
        #[allow(missing_docs)]
        pub validatorPubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ExitRequested {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ExitRequested(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    96u8, 216u8, 202u8, 1u8, 77u8, 71u8, 101u8, 162u8, 184u8, 179u8, 137u8, 226u8,
                    87u8, 20u8, 203u8, 28u8, 239u8, 131u8, 181u8, 116u8, 34u8, 41u8, 17u8, 160u8,
                    29u8, 144u8, 193u8, 189u8, 105u8, 210u8, 211u8, 32u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    validatorPubkeyHash: topics.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.validatorPubkeyHash.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.validatorPubkeyHash,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ExitRequested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ExitRequested> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ExitRequested) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
    ```solidity
    event Initialized(uint8 version);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8,
                    56u8, 82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8,
                    96u8, 206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.version,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `NonBeaconChainETHReceived(uint256)` and selector `0x6fdd3dbdb173299608c0aa9f368735857c8842b581f8389238bf05bd04b3bf49`.
    ```solidity
    event NonBeaconChainETHReceived(uint256 amountReceived);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NonBeaconChainETHReceived {
        #[allow(missing_docs)]
        pub amountReceived: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for NonBeaconChainETHReceived {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "NonBeaconChainETHReceived(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    111u8, 221u8, 61u8, 189u8, 177u8, 115u8, 41u8, 150u8, 8u8, 192u8, 170u8, 159u8,
                    54u8, 135u8, 53u8, 133u8, 124u8, 136u8, 66u8, 181u8, 129u8, 248u8, 56u8, 146u8,
                    56u8, 191u8, 5u8, 189u8, 4u8, 179u8, 191u8, 73u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    amountReceived: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amountReceived,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NonBeaconChainETHReceived {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NonBeaconChainETHReceived> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NonBeaconChainETHReceived) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ProofSubmitterUpdated(address,address)` and selector `0xfb8129080a19d34dceac04ba253fc50304dc86c729bd63cdca4a969ad19a5eac`.
    ```solidity
    event ProofSubmitterUpdated(address prevProofSubmitter, address newProofSubmitter);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProofSubmitterUpdated {
        #[allow(missing_docs)]
        pub prevProofSubmitter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newProofSubmitter: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProofSubmitterUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProofSubmitterUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    251u8, 129u8, 41u8, 8u8, 10u8, 25u8, 211u8, 77u8, 206u8, 172u8, 4u8, 186u8,
                    37u8, 63u8, 197u8, 3u8, 4u8, 220u8, 134u8, 199u8, 41u8, 189u8, 99u8, 205u8,
                    202u8, 74u8, 150u8, 154u8, 209u8, 154u8, 94u8, 172u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevProofSubmitter: data.0,
                    newProofSubmitter: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.prevProofSubmitter,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newProofSubmitter,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProofSubmitterUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProofSubmitterUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProofSubmitterUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `RestakedBeaconChainETHWithdrawn(address,uint256)` and selector `0x8947fd2ce07ef9cc302c4e8f0461015615d91ce851564839e91cc804c2f49d8e`.
    ```solidity
    event RestakedBeaconChainETHWithdrawn(address indexed recipient, uint256 amount);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RestakedBeaconChainETHWithdrawn {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RestakedBeaconChainETHWithdrawn {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RestakedBeaconChainETHWithdrawn(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    137u8, 71u8, 253u8, 44u8, 224u8, 126u8, 249u8, 204u8, 48u8, 44u8, 78u8, 143u8,
                    4u8, 97u8, 1u8, 86u8, 21u8, 217u8, 28u8, 232u8, 81u8, 86u8, 72u8, 57u8, 233u8,
                    28u8, 200u8, 4u8, 194u8, 244u8, 157u8, 142u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    recipient: topics.1,
                    amount: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.recipient.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.recipient,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RestakedBeaconChainETHWithdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RestakedBeaconChainETHWithdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RestakedBeaconChainETHWithdrawn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `SwitchToCompoundingRequested(bytes32)` and selector `0xc97b965b92ae7fd20095fe8eb7b99f81f95f8c4adffb22a19116d8eb2846b016`.
    ```solidity
    event SwitchToCompoundingRequested(bytes32 indexed validatorPubkeyHash);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SwitchToCompoundingRequested {
        #[allow(missing_docs)]
        pub validatorPubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for SwitchToCompoundingRequested {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "SwitchToCompoundingRequested(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    201u8, 123u8, 150u8, 91u8, 146u8, 174u8, 127u8, 210u8, 0u8, 149u8, 254u8,
                    142u8, 183u8, 185u8, 159u8, 129u8, 249u8, 95u8, 140u8, 74u8, 223u8, 251u8,
                    34u8, 161u8, 145u8, 22u8, 216u8, 235u8, 40u8, 70u8, 176u8, 22u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    validatorPubkeyHash: topics.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.validatorPubkeyHash.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.validatorPubkeyHash,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SwitchToCompoundingRequested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SwitchToCompoundingRequested> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SwitchToCompoundingRequested) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ValidatorBalanceUpdated(bytes32,uint64,uint64)` and selector `0xcdae700d7241bc027168c53cf6f889763b0a2c88a65d77fc13a8a9fef0d8605f`.
    ```solidity
    event ValidatorBalanceUpdated(bytes32 pubkeyHash, uint64 balanceTimestamp, uint64 newValidatorBalanceGwei);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ValidatorBalanceUpdated {
        #[allow(missing_docs)]
        pub pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub balanceTimestamp: u64,
        #[allow(missing_docs)]
        pub newValidatorBalanceGwei: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ValidatorBalanceUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ValidatorBalanceUpdated(bytes32,uint64,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    205u8, 174u8, 112u8, 13u8, 114u8, 65u8, 188u8, 2u8, 113u8, 104u8, 197u8, 60u8,
                    246u8, 248u8, 137u8, 118u8, 59u8, 10u8, 44u8, 136u8, 166u8, 93u8, 119u8, 252u8,
                    19u8, 168u8, 169u8, 254u8, 240u8, 216u8, 96u8, 95u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    pubkeyHash: data.0,
                    balanceTimestamp: data.1,
                    newValidatorBalanceGwei: data.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.pubkeyHash),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newValidatorBalanceGwei,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ValidatorBalanceUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ValidatorBalanceUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ValidatorBalanceUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ValidatorCheckpointed(uint64,bytes32)` and selector `0xe4866335761a51dcaff766448ab0af6064291ee5dc94e68492bb9cd757c1e350`.
    ```solidity
    event ValidatorCheckpointed(uint64 indexed checkpointTimestamp, bytes32 indexed pubkeyHash);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ValidatorCheckpointed {
        #[allow(missing_docs)]
        pub checkpointTimestamp: u64,
        #[allow(missing_docs)]
        pub pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ValidatorCheckpointed {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ValidatorCheckpointed(uint64,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    228u8, 134u8, 99u8, 53u8, 118u8, 26u8, 81u8, 220u8, 175u8, 247u8, 102u8, 68u8,
                    138u8, 176u8, 175u8, 96u8, 100u8, 41u8, 30u8, 229u8, 220u8, 148u8, 230u8,
                    132u8, 146u8, 187u8, 156u8, 215u8, 87u8, 193u8, 227u8, 80u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    checkpointTimestamp: topics.1,
                    pubkeyHash: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.checkpointTimestamp.clone(),
                    self.pubkeyHash.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.checkpointTimestamp,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.pubkeyHash);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ValidatorCheckpointed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ValidatorCheckpointed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ValidatorCheckpointed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ValidatorRestaked(bytes32)` and selector `0x101790c2993f6a4d962bd17c786126823ba1c4cf04ff4cccb2659d50fb20aee8`.
    ```solidity
    event ValidatorRestaked(bytes32 pubkeyHash);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ValidatorRestaked {
        #[allow(missing_docs)]
        pub pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ValidatorRestaked {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ValidatorRestaked(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    16u8, 23u8, 144u8, 194u8, 153u8, 63u8, 106u8, 77u8, 150u8, 43u8, 209u8, 124u8,
                    120u8, 97u8, 38u8, 130u8, 59u8, 161u8, 196u8, 207u8, 4u8, 255u8, 76u8, 204u8,
                    178u8, 101u8, 157u8, 80u8, 251u8, 32u8, 174u8, 232u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { pubkeyHash: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.pubkeyHash),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ValidatorRestaked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ValidatorRestaked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ValidatorRestaked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ValidatorWithdrawn(uint64,bytes32)` and selector `0x5ce0aa04ae51d52da6e680fbe0336d2e2432f7c3dc2d4f3193204c57b9072107`.
    ```solidity
    event ValidatorWithdrawn(uint64 indexed checkpointTimestamp, bytes32 indexed pubkeyHash);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ValidatorWithdrawn {
        #[allow(missing_docs)]
        pub checkpointTimestamp: u64,
        #[allow(missing_docs)]
        pub pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ValidatorWithdrawn {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "ValidatorWithdrawn(uint64,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    92u8, 224u8, 170u8, 4u8, 174u8, 81u8, 213u8, 45u8, 166u8, 230u8, 128u8, 251u8,
                    224u8, 51u8, 109u8, 46u8, 36u8, 50u8, 247u8, 195u8, 220u8, 45u8, 79u8, 49u8,
                    147u8, 32u8, 76u8, 87u8, 185u8, 7u8, 33u8, 7u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    checkpointTimestamp: topics.1,
                    pubkeyHash: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.checkpointTimestamp.clone(),
                    self.pubkeyHash.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.checkpointTimestamp,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.pubkeyHash);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ValidatorWithdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ValidatorWithdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ValidatorWithdrawn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `WithdrawalRequested(bytes32,uint64)` and selector `0x8b2737bb64ab2f2dc09552dfa1c250399e6a42c7ea9f0e1c658f5d65d708ec05`.
    ```solidity
    event WithdrawalRequested(bytes32 indexed validatorPubkeyHash, uint64 withdrawalAmountGwei);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WithdrawalRequested {
        #[allow(missing_docs)]
        pub validatorPubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub withdrawalAmountGwei: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for WithdrawalRequested {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "WithdrawalRequested(bytes32,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 39u8, 55u8, 187u8, 100u8, 171u8, 47u8, 45u8, 192u8, 149u8, 82u8, 223u8,
                    161u8, 194u8, 80u8, 57u8, 158u8, 106u8, 66u8, 199u8, 234u8, 159u8, 14u8, 28u8,
                    101u8, 143u8, 93u8, 101u8, 215u8, 8u8, 236u8, 5u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    validatorPubkeyHash: topics.1,
                    withdrawalAmountGwei: data.0,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(alloy_sol_types::Error::invalid_event_signature_hash(
                        Self::SIGNATURE,
                        topics.0,
                        Self::SIGNATURE_HASH,
                    ));
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawalAmountGwei,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.validatorPubkeyHash.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.validatorPubkeyHash,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for WithdrawalRequested {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WithdrawalRequested> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &WithdrawalRequested) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _ethPOS, address _eigenPodManager, string _version);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _ethPOS: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _eigenPodManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _version: alloy::sol_types::private::String,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::String,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._ethPOS, value._eigenPodManager, value._version)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _ethPOS: tuple.0,
                        _eigenPodManager: tuple.1,
                        _version: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._ethPOS,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._eigenPodManager,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._version,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `activeValidatorCount()` and selector `0x2340e8d3`.
    ```solidity
    function activeValidatorCount() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct activeValidatorCountCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`activeValidatorCount()`](activeValidatorCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct activeValidatorCountReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<activeValidatorCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: activeValidatorCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for activeValidatorCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<activeValidatorCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: activeValidatorCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for activeValidatorCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for activeValidatorCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "activeValidatorCount()";
            const SELECTOR: [u8; 4] = [35u8, 64u8, 232u8, 211u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: activeValidatorCountReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: activeValidatorCountReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `checkpointBalanceExitedGwei(uint64)` and selector `0x52396a59`.
    ```solidity
    function checkpointBalanceExitedGwei(uint64) external view returns (uint64);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointBalanceExitedGweiCall(pub u64);
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`checkpointBalanceExitedGwei(uint64)`](checkpointBalanceExitedGweiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointBalanceExitedGweiReturn {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkpointBalanceExitedGweiCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkpointBalanceExitedGweiCall) -> Self {
                    (value.0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkpointBalanceExitedGweiCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self(tuple.0)
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkpointBalanceExitedGweiReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkpointBalanceExitedGweiReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkpointBalanceExitedGweiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkpointBalanceExitedGweiCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkpointBalanceExitedGwei(uint64)";
            const SELECTOR: [u8; 4] = [82u8, 57u8, 106u8, 89u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.0,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: checkpointBalanceExitedGweiReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: checkpointBalanceExitedGweiReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `currentCheckpoint()` and selector `0x47d28372`.
    ```solidity
    function currentCheckpoint() external view returns (IEigenPodTypes.Checkpoint memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentCheckpointCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`currentCheckpoint()`](currentCheckpointCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentCheckpointReturn {
        #[allow(missing_docs)]
        pub _0: <IEigenPodTypes::Checkpoint as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<currentCheckpointCall> for UnderlyingRustTuple<'_> {
                fn from(value: currentCheckpointCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentCheckpointCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPodTypes::Checkpoint,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IEigenPodTypes::Checkpoint as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<currentCheckpointReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currentCheckpointReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentCheckpointReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentCheckpointCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IEigenPodTypes::Checkpoint as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IEigenPodTypes::Checkpoint,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentCheckpoint()";
            const SELECTOR: [u8; 4] = [71u8, 210u8, 131u8, 114u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<IEigenPodTypes::Checkpoint as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: currentCheckpointReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: currentCheckpointReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `currentCheckpointTimestamp()` and selector `0x42ecff2a`.
    ```solidity
    function currentCheckpointTimestamp() external view returns (uint64);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentCheckpointTimestampCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`currentCheckpointTimestamp()`](currentCheckpointTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentCheckpointTimestampReturn {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<currentCheckpointTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: currentCheckpointTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentCheckpointTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<currentCheckpointTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currentCheckpointTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentCheckpointTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentCheckpointTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentCheckpointTimestamp()";
            const SELECTOR: [u8; 4] = [66u8, 236u8, 255u8, 42u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: currentCheckpointTimestampReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: currentCheckpointTimestampReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `eigenPodManager()` and selector `0x4665bcda`.
    ```solidity
    function eigenPodManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`eigenPodManager()`](eigenPodManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eigenPodManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenPodManagerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eigenPodManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenPodManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenPodManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenPodManager()";
            const SELECTOR: [u8; 4] = [70u8, 101u8, 188u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: eigenPodManagerReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: eigenPodManagerReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ethPOS()` and selector `0x74cdd798`.
    ```solidity
    function ethPOS() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ethPOSCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ethPOS()`](ethPOSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ethPOSReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ethPOSCall> for UnderlyingRustTuple<'_> {
                fn from(value: ethPOSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ethPOSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ethPOSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ethPOSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ethPOSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ethPOSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ethPOS()";
            const SELECTOR: [u8; 4] = [116u8, 205u8, 215u8, 152u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: ethPOSReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: ethPOSReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getConsolidationRequestFee()` and selector `0x1e515533`.
    ```solidity
    function getConsolidationRequestFee() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getConsolidationRequestFeeCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getConsolidationRequestFee()`](getConsolidationRequestFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getConsolidationRequestFeeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getConsolidationRequestFeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getConsolidationRequestFeeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getConsolidationRequestFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getConsolidationRequestFeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getConsolidationRequestFeeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getConsolidationRequestFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getConsolidationRequestFeeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getConsolidationRequestFee()";
            const SELECTOR: [u8; 4] = [30u8, 81u8, 85u8, 51u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getConsolidationRequestFeeReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: getConsolidationRequestFeeReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getParentBlockRoot(uint64)` and selector `0x6c0d2d5a`.
    ```solidity
    function getParentBlockRoot(uint64 timestamp) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getParentBlockRootCall {
        #[allow(missing_docs)]
        pub timestamp: u64,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getParentBlockRoot(uint64)`](getParentBlockRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getParentBlockRootReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getParentBlockRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getParentBlockRootCall) -> Self {
                    (value.timestamp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getParentBlockRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { timestamp: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getParentBlockRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getParentBlockRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getParentBlockRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getParentBlockRootCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getParentBlockRoot(uint64)";
            const SELECTOR: [u8; 4] = [108u8, 13u8, 45u8, 90u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.timestamp,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getParentBlockRootReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: getParentBlockRootReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getWithdrawalRequestFee()` and selector `0xc44e30dc`.
    ```solidity
    function getWithdrawalRequestFee() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalRequestFeeCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getWithdrawalRequestFee()`](getWithdrawalRequestFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalRequestFeeReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getWithdrawalRequestFeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalRequestFeeCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getWithdrawalRequestFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U256,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getWithdrawalRequestFeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalRequestFeeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getWithdrawalRequestFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalRequestFeeCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalRequestFee()";
            const SELECTOR: [u8; 4] = [196u8, 78u8, 48u8, 220u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getWithdrawalRequestFeeReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: getWithdrawalRequestFeeReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initialize(address)` and selector `0xc4d66de8`.
    ```solidity
    function initialize(address _podOwner) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub _podOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._podOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _podOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl initializeReturn {
            fn _tokenize(&self) -> <initializeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address)";
            const SELECTOR: [u8; 4] = [196u8, 214u8, 109u8, 232u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._podOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                initializeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `lastCheckpointTimestamp()` and selector `0xee94d67c`.
    ```solidity
    function lastCheckpointTimestamp() external view returns (uint64);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastCheckpointTimestampCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`lastCheckpointTimestamp()`](lastCheckpointTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastCheckpointTimestampReturn {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lastCheckpointTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: lastCheckpointTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastCheckpointTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lastCheckpointTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lastCheckpointTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastCheckpointTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastCheckpointTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastCheckpointTimestamp()";
            const SELECTOR: [u8; 4] = [238u8, 148u8, 214u8, 124u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: lastCheckpointTimestampReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: lastCheckpointTimestampReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `podOwner()` and selector `0x0b18ff66`.
    ```solidity
    function podOwner() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct podOwnerCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`podOwner()`](podOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct podOwnerReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<podOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: podOwnerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for podOwnerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<podOwnerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: podOwnerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for podOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for podOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "podOwner()";
            const SELECTOR: [u8; 4] = [11u8, 24u8, 255u8, 102u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: podOwnerReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: podOwnerReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `proofSubmitter()` and selector `0x58753357`.
    ```solidity
    function proofSubmitter() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofSubmitterCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`proofSubmitter()`](proofSubmitterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofSubmitterReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proofSubmitterCall> for UnderlyingRustTuple<'_> {
                fn from(value: proofSubmitterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proofSubmitterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proofSubmitterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proofSubmitterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proofSubmitterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proofSubmitterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proofSubmitter()";
            const SELECTOR: [u8; 4] = [88u8, 117u8, 51u8, 87u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: proofSubmitterReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: proofSubmitterReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `recoverTokens(address[],uint256[],address)` and selector `0xdda3346c`.
    ```solidity
    function recoverTokens(address[] memory tokenList, uint256[] memory amountsToWithdraw, address recipient) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct recoverTokensCall {
        #[allow(missing_docs)]
        pub tokenList: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub amountsToWithdraw:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`recoverTokens(address[],uint256[],address)`](recoverTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct recoverTokensReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<recoverTokensCall> for UnderlyingRustTuple<'_> {
                fn from(value: recoverTokensCall) -> Self {
                    (value.tokenList, value.amountsToWithdraw, value.recipient)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for recoverTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenList: tuple.0,
                        amountsToWithdraw: tuple.1,
                        recipient: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<recoverTokensReturn> for UnderlyingRustTuple<'_> {
                fn from(value: recoverTokensReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for recoverTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl recoverTokensReturn {
            fn _tokenize(
                &self,
            ) -> <recoverTokensCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for recoverTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = recoverTokensReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "recoverTokens(address[],uint256[],address)";
            const SELECTOR: [u8; 4] = [221u8, 163u8, 52u8, 108u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenList),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountsToWithdraw),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                recoverTokensReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `requestConsolidation((bytes,bytes)[])` and selector `0x6691954e`.
    ```solidity
    function requestConsolidation(IEigenPodTypes.ConsolidationRequest[] memory requests) external payable;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestConsolidationCall {
        #[allow(missing_docs)]
        pub requests: alloy::sol_types::private::Vec<
            <IEigenPodTypes::ConsolidationRequest as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`requestConsolidation((bytes,bytes)[])`](requestConsolidationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestConsolidationReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IEigenPodTypes::ConsolidationRequest>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IEigenPodTypes::ConsolidationRequest as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<requestConsolidationCall> for UnderlyingRustTuple<'_> {
                fn from(value: requestConsolidationCall) -> Self {
                    (value.requests,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for requestConsolidationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requests: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<requestConsolidationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: requestConsolidationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for requestConsolidationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl requestConsolidationReturn {
            fn _tokenize(
                &self,
            ) -> <requestConsolidationCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestConsolidationCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IEigenPodTypes::ConsolidationRequest>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = requestConsolidationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestConsolidation((bytes,bytes)[])";
            const SELECTOR: [u8; 4] = [102u8, 145u8, 149u8, 78u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IEigenPodTypes::ConsolidationRequest,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.requests
                ),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                requestConsolidationReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `requestWithdrawal((bytes,uint64)[])` and selector `0x3f5fa57a`.
    ```solidity
    function requestWithdrawal(IEigenPodTypes.WithdrawalRequest[] memory requests) external payable;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestWithdrawalCall {
        #[allow(missing_docs)]
        pub requests: alloy::sol_types::private::Vec<
            <IEigenPodTypes::WithdrawalRequest as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`requestWithdrawal((bytes,uint64)[])`](requestWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct requestWithdrawalReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IEigenPodTypes::WithdrawalRequest>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IEigenPodTypes::WithdrawalRequest as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<requestWithdrawalCall> for UnderlyingRustTuple<'_> {
                fn from(value: requestWithdrawalCall) -> Self {
                    (value.requests,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for requestWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { requests: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<requestWithdrawalReturn> for UnderlyingRustTuple<'_> {
                fn from(value: requestWithdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for requestWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl requestWithdrawalReturn {
            fn _tokenize(
                &self,
            ) -> <requestWithdrawalCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for requestWithdrawalCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IEigenPodTypes::WithdrawalRequest>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = requestWithdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "requestWithdrawal((bytes,uint64)[])";
            const SELECTOR: [u8; 4] = [63u8, 95u8, 165u8, 122u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IEigenPodTypes::WithdrawalRequest,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.requests
                ),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                requestWithdrawalReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `setProofSubmitter(address)` and selector `0xd06d5587`.
    ```solidity
    function setProofSubmitter(address newProofSubmitter) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setProofSubmitterCall {
        #[allow(missing_docs)]
        pub newProofSubmitter: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setProofSubmitter(address)`](setProofSubmitterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setProofSubmitterReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setProofSubmitterCall> for UnderlyingRustTuple<'_> {
                fn from(value: setProofSubmitterCall) -> Self {
                    (value.newProofSubmitter,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setProofSubmitterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newProofSubmitter: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setProofSubmitterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setProofSubmitterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setProofSubmitterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setProofSubmitterReturn {
            fn _tokenize(
                &self,
            ) -> <setProofSubmitterCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setProofSubmitterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setProofSubmitterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setProofSubmitter(address)";
            const SELECTOR: [u8; 4] = [208u8, 109u8, 85u8, 135u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newProofSubmitter,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setProofSubmitterReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `stake(bytes,bytes,bytes32)` and selector `0x9b4e4634`.
    ```solidity
    function stake(bytes memory pubkey, bytes memory signature, bytes32 depositDataRoot) external payable;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeCall {
        #[allow(missing_docs)]
        pub pubkey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub depositDataRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`stake(bytes,bytes,bytes32)`](stakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeCall) -> Self {
                    (value.pubkey, value.signature, value.depositDataRoot)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pubkey: tuple.0,
                        signature: tuple.1,
                        depositDataRoot: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl stakeReturn {
            fn _tokenize(&self) -> <stakeCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stake(bytes,bytes,bytes32)";
            const SELECTOR: [u8; 4] = [155u8, 78u8, 70u8, 52u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.pubkey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.depositDataRoot),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                stakeReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `startCheckpoint(bool)` and selector `0x88676cad`.
    ```solidity
    function startCheckpoint(bool revertIfNoBalance) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startCheckpointCall {
        #[allow(missing_docs)]
        pub revertIfNoBalance: bool,
    }
    ///Container type for the return parameters of the [`startCheckpoint(bool)`](startCheckpointCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startCheckpointReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<startCheckpointCall> for UnderlyingRustTuple<'_> {
                fn from(value: startCheckpointCall) -> Self {
                    (value.revertIfNoBalance,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startCheckpointCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        revertIfNoBalance: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<startCheckpointReturn> for UnderlyingRustTuple<'_> {
                fn from(value: startCheckpointReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startCheckpointReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl startCheckpointReturn {
            fn _tokenize(
                &self,
            ) -> <startCheckpointCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startCheckpointCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bool,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = startCheckpointReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startCheckpoint(bool)";
            const SELECTOR: [u8; 4] = [136u8, 103u8, 108u8, 173u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.revertIfNoBalance,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                startCheckpointReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `validatorPubkeyHashToInfo(bytes32)` and selector `0x6fcd0e53`.
    ```solidity
    function validatorPubkeyHashToInfo(bytes32 validatorPubkeyHash) external view returns (IEigenPodTypes.ValidatorInfo memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorPubkeyHashToInfoCall {
        #[allow(missing_docs)]
        pub validatorPubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    ///Container type for the return parameters of the [`validatorPubkeyHashToInfo(bytes32)`](validatorPubkeyHashToInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorPubkeyHashToInfoReturn {
        #[allow(missing_docs)]
        pub _0: <IEigenPodTypes::ValidatorInfo as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorPubkeyHashToInfoCall> for UnderlyingRustTuple<'_> {
                fn from(value: validatorPubkeyHashToInfoCall) -> Self {
                    (value.validatorPubkeyHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorPubkeyHashToInfoCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        validatorPubkeyHash: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPodTypes::ValidatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IEigenPodTypes::ValidatorInfo as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorPubkeyHashToInfoReturn> for UnderlyingRustTuple<'_> {
                fn from(value: validatorPubkeyHashToInfoReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorPubkeyHashToInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorPubkeyHashToInfoCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IEigenPodTypes::ValidatorInfo as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IEigenPodTypes::ValidatorInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatorPubkeyHashToInfo(bytes32)";
            const SELECTOR: [u8; 4] = [111u8, 205u8, 14u8, 83u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorPubkeyHash),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<IEigenPodTypes::ValidatorInfo as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: validatorPubkeyHashToInfoReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: validatorPubkeyHashToInfoReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `validatorPubkeyToInfo(bytes)` and selector `0xb522538a`.
    ```solidity
    function validatorPubkeyToInfo(bytes memory validatorPubkey) external view returns (IEigenPodTypes.ValidatorInfo memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorPubkeyToInfoCall {
        #[allow(missing_docs)]
        pub validatorPubkey: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    ///Container type for the return parameters of the [`validatorPubkeyToInfo(bytes)`](validatorPubkeyToInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorPubkeyToInfoReturn {
        #[allow(missing_docs)]
        pub _0: <IEigenPodTypes::ValidatorInfo as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorPubkeyToInfoCall> for UnderlyingRustTuple<'_> {
                fn from(value: validatorPubkeyToInfoCall) -> Self {
                    (value.validatorPubkey,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorPubkeyToInfoCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        validatorPubkey: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPodTypes::ValidatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IEigenPodTypes::ValidatorInfo as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorPubkeyToInfoReturn> for UnderlyingRustTuple<'_> {
                fn from(value: validatorPubkeyToInfoReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorPubkeyToInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorPubkeyToInfoCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IEigenPodTypes::ValidatorInfo as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IEigenPodTypes::ValidatorInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatorPubkeyToInfo(bytes)";
            const SELECTOR: [u8; 4] = [181u8, 34u8, 83u8, 138u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.validatorPubkey,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<IEigenPodTypes::ValidatorInfo as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: validatorPubkeyToInfoReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: validatorPubkeyToInfoReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `validatorStatus(bytes)` and selector `0x58eaee79`.
    ```solidity
    function validatorStatus(bytes memory validatorPubkey) external view returns (IEigenPodTypes.VALIDATOR_STATUS);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorStatus_0Call {
        #[allow(missing_docs)]
        pub validatorPubkey: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`validatorStatus(bytes)`](validatorStatus_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorStatus_0Return {
        #[allow(missing_docs)]
        pub _0: <IEigenPodTypes::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorStatus_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: validatorStatus_0Call) -> Self {
                    (value.validatorPubkey,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorStatus_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        validatorPubkey: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPodTypes::VALIDATOR_STATUS,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IEigenPodTypes::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorStatus_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: validatorStatus_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorStatus_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorStatus_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IEigenPodTypes::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IEigenPodTypes::VALIDATOR_STATUS,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatorStatus(bytes)";
            const SELECTOR: [u8; 4] = [88u8, 234u8, 238u8, 121u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.validatorPubkey,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<IEigenPodTypes::VALIDATOR_STATUS as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: validatorStatus_0Return = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: validatorStatus_0Return = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `validatorStatus(bytes32)` and selector `0x7439841f`.
    ```solidity
    function validatorStatus(bytes32 pubkeyHash) external view returns (IEigenPodTypes.VALIDATOR_STATUS);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorStatus_1Call {
        #[allow(missing_docs)]
        pub pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`validatorStatus(bytes32)`](validatorStatus_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorStatus_1Return {
        #[allow(missing_docs)]
        pub _0: <IEigenPodTypes::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorStatus_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: validatorStatus_1Call) -> Self {
                    (value.pubkeyHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorStatus_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pubkeyHash: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPodTypes::VALIDATOR_STATUS,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IEigenPodTypes::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorStatus_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: validatorStatus_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorStatus_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorStatus_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IEigenPodTypes::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IEigenPodTypes::VALIDATOR_STATUS,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatorStatus(bytes32)";
            const SELECTOR: [u8; 4] = [116u8, 57u8, 132u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.pubkeyHash),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<IEigenPodTypes::VALIDATOR_STATUS as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: validatorStatus_1Return = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: validatorStatus_1Return = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `verifyCheckpointProofs((bytes32,bytes),(bytes32,bytes32,bytes)[])` and selector `0xf074ba62`.
    ```solidity
    function verifyCheckpointProofs(BeaconChainProofs.BalanceContainerProof memory balanceContainerProof, BeaconChainProofs.BalanceProof[] memory proofs) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCheckpointProofsCall {
        #[allow(missing_docs)]
        pub balanceContainerProof:
            <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub proofs: alloy::sol_types::private::Vec<
            <BeaconChainProofs::BalanceProof as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`verifyCheckpointProofs((bytes32,bytes),(bytes32,bytes32,bytes)[])`](verifyCheckpointProofsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCheckpointProofsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                BeaconChainProofs::BalanceContainerProof,
                alloy::sol_types::sol_data::Array<BeaconChainProofs::BalanceProof>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <BeaconChainProofs::BalanceProof as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyCheckpointProofsCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCheckpointProofsCall) -> Self {
                    (value.balanceContainerProof, value.proofs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCheckpointProofsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        balanceContainerProof: tuple.0,
                        proofs: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyCheckpointProofsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCheckpointProofsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCheckpointProofsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl verifyCheckpointProofsReturn {
            fn _tokenize(
                &self,
            ) -> <verifyCheckpointProofsCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyCheckpointProofsCall {
            type Parameters<'a> = (
                BeaconChainProofs::BalanceContainerProof,
                alloy::sol_types::sol_data::Array<BeaconChainProofs::BalanceProof>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyCheckpointProofsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "verifyCheckpointProofs((bytes32,bytes),(bytes32,bytes32,bytes)[])";
            const SELECTOR: [u8; 4] = [240u8, 116u8, 186u8, 98u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <BeaconChainProofs::BalanceContainerProof as alloy_sol_types::SolType>::tokenize(
                        &self.balanceContainerProof,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BeaconChainProofs::BalanceProof,
                    > as alloy_sol_types::SolType>::tokenize(&self.proofs),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                verifyCheckpointProofsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `verifyStaleBalance(uint64,(bytes32,bytes),(bytes32[],bytes))` and selector `0x039157d2`.
    ```solidity
    function verifyStaleBalance(uint64 beaconTimestamp, BeaconChainProofs.StateRootProof memory stateRootProof, BeaconChainProofs.ValidatorProof memory proof) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyStaleBalanceCall {
        #[allow(missing_docs)]
        pub beaconTimestamp: u64,
        #[allow(missing_docs)]
        pub stateRootProof:
            <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub proof: <BeaconChainProofs::ValidatorProof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`verifyStaleBalance(uint64,(bytes32,bytes),(bytes32[],bytes))`](verifyStaleBalanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyStaleBalanceReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                BeaconChainProofs::StateRootProof,
                BeaconChainProofs::ValidatorProof,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
                <BeaconChainProofs::ValidatorProof as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyStaleBalanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyStaleBalanceCall) -> Self {
                    (value.beaconTimestamp, value.stateRootProof, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyStaleBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        beaconTimestamp: tuple.0,
                        stateRootProof: tuple.1,
                        proof: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyStaleBalanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyStaleBalanceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyStaleBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl verifyStaleBalanceReturn {
            fn _tokenize(
                &self,
            ) -> <verifyStaleBalanceCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyStaleBalanceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                BeaconChainProofs::StateRootProof,
                BeaconChainProofs::ValidatorProof,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyStaleBalanceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "verifyStaleBalance(uint64,(bytes32,bytes),(bytes32[],bytes))";
            const SELECTOR: [u8; 4] = [3u8, 145u8, 87u8, 210u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.beaconTimestamp,
                    ),
                    <BeaconChainProofs::StateRootProof as alloy_sol_types::SolType>::tokenize(
                        &self.stateRootProof,
                    ),
                    <BeaconChainProofs::ValidatorProof as alloy_sol_types::SolType>::tokenize(
                        &self.proof,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                verifyStaleBalanceReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `verifyWithdrawalCredentials(uint64,(bytes32,bytes),uint40[],bytes[],bytes32[][])` and selector `0x3f65cf19`.
    ```solidity
    function verifyWithdrawalCredentials(uint64 beaconTimestamp, BeaconChainProofs.StateRootProof memory stateRootProof, uint40[] memory validatorIndices, bytes[] memory validatorFieldsProofs, bytes32[][] memory validatorFields) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyWithdrawalCredentialsCall {
        #[allow(missing_docs)]
        pub beaconTimestamp: u64,
        #[allow(missing_docs)]
        pub stateRootProof:
            <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub validatorIndices:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
        #[allow(missing_docs)]
        pub validatorFieldsProofs: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub validatorFields: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        >,
    }
    ///Container type for the return parameters of the [`verifyWithdrawalCredentials(uint64,(bytes32,bytes),uint40[],bytes[],bytes32[][])`](verifyWithdrawalCredentialsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyWithdrawalCredentialsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                BeaconChainProofs::StateRootProof,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyWithdrawalCredentialsCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentialsCall) -> Self {
                    (
                        value.beaconTimestamp,
                        value.stateRootProof,
                        value.validatorIndices,
                        value.validatorFieldsProofs,
                        value.validatorFields,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyWithdrawalCredentialsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        beaconTimestamp: tuple.0,
                        stateRootProof: tuple.1,
                        validatorIndices: tuple.2,
                        validatorFieldsProofs: tuple.3,
                        validatorFields: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyWithdrawalCredentialsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentialsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyWithdrawalCredentialsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl verifyWithdrawalCredentialsReturn {
            fn _tokenize(
                &self,
            ) -> <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyWithdrawalCredentialsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                BeaconChainProofs::StateRootProof,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                >,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyWithdrawalCredentialsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "verifyWithdrawalCredentials(uint64,(bytes32,bytes),uint40[],bytes[],bytes32[][])";
            const SELECTOR: [u8; 4] = [63u8, 101u8, 207u8, 25u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconTimestamp),
                    <BeaconChainProofs::StateRootProof as alloy_sol_types::SolType>::tokenize(
                        &self.stateRootProof,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<40>,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.validatorFieldsProofs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::FixedBytes<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorFields),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                verifyWithdrawalCredentialsReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `version()` and selector `0x54fd4d50`.
    ```solidity
    function version() external view returns (string memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`version()`](versionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct versionReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<versionCall> for UnderlyingRustTuple<'_> {
                fn from(value: versionCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for versionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<versionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: versionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for versionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for versionCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::String;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "version()";
            const SELECTOR: [u8; 4] = [84u8, 253u8, 77u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: versionReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: versionReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `withdrawRestakedBeaconChainETH(address,uint256)` and selector `0xc4907442`.
    ```solidity
    function withdrawRestakedBeaconChainETH(address recipient, uint256 amountWei) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawRestakedBeaconChainETHCall {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amountWei: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdrawRestakedBeaconChainETH(address,uint256)`](withdrawRestakedBeaconChainETHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawRestakedBeaconChainETHReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawRestakedBeaconChainETHCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawRestakedBeaconChainETHCall) -> Self {
                    (value.recipient, value.amountWei)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawRestakedBeaconChainETHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        recipient: tuple.0,
                        amountWei: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawRestakedBeaconChainETHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawRestakedBeaconChainETHReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawRestakedBeaconChainETHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl withdrawRestakedBeaconChainETHReturn {
            fn _tokenize(
                &self,
            ) -> <withdrawRestakedBeaconChainETHCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawRestakedBeaconChainETHCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawRestakedBeaconChainETHReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawRestakedBeaconChainETH(address,uint256)";
            const SELECTOR: [u8; 4] = [196u8, 144u8, 116u8, 66u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amountWei,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                withdrawRestakedBeaconChainETHReturn::_tokenize(ret)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data)
                    .map(Into::into)
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(Into::into)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `withdrawableRestakedExecutionLayerGwei()` and selector `0x3474aa16`.
    ```solidity
    function withdrawableRestakedExecutionLayerGwei() external view returns (uint64);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawableRestakedExecutionLayerGweiCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`withdrawableRestakedExecutionLayerGwei()`](withdrawableRestakedExecutionLayerGweiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawableRestakedExecutionLayerGweiReturn {
        #[allow(missing_docs)]
        pub _0: u64,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawableRestakedExecutionLayerGweiCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawableRestakedExecutionLayerGweiCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawableRestakedExecutionLayerGweiCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawableRestakedExecutionLayerGweiReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: withdrawableRestakedExecutionLayerGweiReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for withdrawableRestakedExecutionLayerGweiReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawableRestakedExecutionLayerGweiCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u64;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawableRestakedExecutionLayerGwei()";
            const SELECTOR: [u8; 4] = [52u8, 116u8, 170u8, 22u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: withdrawableRestakedExecutionLayerGweiReturn = r.into();
                        r._0
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: withdrawableRestakedExecutionLayerGweiReturn = r.into();
                    r._0
                })
            }
        }
    };
    ///Container for all the [`EigenPod`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum EigenPodCalls {
        #[allow(missing_docs)]
        activeValidatorCount(activeValidatorCountCall),
        #[allow(missing_docs)]
        checkpointBalanceExitedGwei(checkpointBalanceExitedGweiCall),
        #[allow(missing_docs)]
        currentCheckpoint(currentCheckpointCall),
        #[allow(missing_docs)]
        currentCheckpointTimestamp(currentCheckpointTimestampCall),
        #[allow(missing_docs)]
        eigenPodManager(eigenPodManagerCall),
        #[allow(missing_docs)]
        ethPOS(ethPOSCall),
        #[allow(missing_docs)]
        getConsolidationRequestFee(getConsolidationRequestFeeCall),
        #[allow(missing_docs)]
        getParentBlockRoot(getParentBlockRootCall),
        #[allow(missing_docs)]
        getWithdrawalRequestFee(getWithdrawalRequestFeeCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        lastCheckpointTimestamp(lastCheckpointTimestampCall),
        #[allow(missing_docs)]
        podOwner(podOwnerCall),
        #[allow(missing_docs)]
        proofSubmitter(proofSubmitterCall),
        #[allow(missing_docs)]
        recoverTokens(recoverTokensCall),
        #[allow(missing_docs)]
        requestConsolidation(requestConsolidationCall),
        #[allow(missing_docs)]
        requestWithdrawal(requestWithdrawalCall),
        #[allow(missing_docs)]
        setProofSubmitter(setProofSubmitterCall),
        #[allow(missing_docs)]
        stake(stakeCall),
        #[allow(missing_docs)]
        startCheckpoint(startCheckpointCall),
        #[allow(missing_docs)]
        validatorPubkeyHashToInfo(validatorPubkeyHashToInfoCall),
        #[allow(missing_docs)]
        validatorPubkeyToInfo(validatorPubkeyToInfoCall),
        #[allow(missing_docs)]
        validatorStatus_0(validatorStatus_0Call),
        #[allow(missing_docs)]
        validatorStatus_1(validatorStatus_1Call),
        #[allow(missing_docs)]
        verifyCheckpointProofs(verifyCheckpointProofsCall),
        #[allow(missing_docs)]
        verifyStaleBalance(verifyStaleBalanceCall),
        #[allow(missing_docs)]
        verifyWithdrawalCredentials(verifyWithdrawalCredentialsCall),
        #[allow(missing_docs)]
        version(versionCall),
        #[allow(missing_docs)]
        withdrawRestakedBeaconChainETH(withdrawRestakedBeaconChainETHCall),
        #[allow(missing_docs)]
        withdrawableRestakedExecutionLayerGwei(withdrawableRestakedExecutionLayerGweiCall),
    }
    #[automatically_derived]
    impl EigenPodCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 145u8, 87u8, 210u8],
            [11u8, 24u8, 255u8, 102u8],
            [30u8, 81u8, 85u8, 51u8],
            [35u8, 64u8, 232u8, 211u8],
            [52u8, 116u8, 170u8, 22u8],
            [63u8, 95u8, 165u8, 122u8],
            [63u8, 101u8, 207u8, 25u8],
            [66u8, 236u8, 255u8, 42u8],
            [70u8, 101u8, 188u8, 218u8],
            [71u8, 210u8, 131u8, 114u8],
            [82u8, 57u8, 106u8, 89u8],
            [84u8, 253u8, 77u8, 80u8],
            [88u8, 117u8, 51u8, 87u8],
            [88u8, 234u8, 238u8, 121u8],
            [102u8, 145u8, 149u8, 78u8],
            [108u8, 13u8, 45u8, 90u8],
            [111u8, 205u8, 14u8, 83u8],
            [116u8, 57u8, 132u8, 31u8],
            [116u8, 205u8, 215u8, 152u8],
            [136u8, 103u8, 108u8, 173u8],
            [155u8, 78u8, 70u8, 52u8],
            [181u8, 34u8, 83u8, 138u8],
            [196u8, 78u8, 48u8, 220u8],
            [196u8, 144u8, 116u8, 66u8],
            [196u8, 214u8, 109u8, 232u8],
            [208u8, 109u8, 85u8, 135u8],
            [221u8, 163u8, 52u8, 108u8],
            [238u8, 148u8, 214u8, 124u8],
            [240u8, 116u8, 186u8, 98u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EigenPodCalls {
        const NAME: &'static str = "EigenPodCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::activeValidatorCount(_) => {
                    <activeValidatorCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkpointBalanceExitedGwei(_) => {
                    <checkpointBalanceExitedGweiCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentCheckpoint(_) => {
                    <currentCheckpointCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentCheckpointTimestamp(_) => {
                    <currentCheckpointTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenPodManager(_) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ethPOS(_) => <ethPOSCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getConsolidationRequestFee(_) => {
                    <getConsolidationRequestFeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getParentBlockRoot(_) => {
                    <getParentBlockRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawalRequestFee(_) => {
                    <getWithdrawalRequestFeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastCheckpointTimestamp(_) => {
                    <lastCheckpointTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::podOwner(_) => <podOwnerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proofSubmitter(_) => {
                    <proofSubmitterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::recoverTokens(_) => {
                    <recoverTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requestConsolidation(_) => {
                    <requestConsolidationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::requestWithdrawal(_) => {
                    <requestWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setProofSubmitter(_) => {
                    <setProofSubmitterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stake(_) => <stakeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::startCheckpoint(_) => {
                    <startCheckpointCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatorPubkeyHashToInfo(_) => {
                    <validatorPubkeyHashToInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatorPubkeyToInfo(_) => {
                    <validatorPubkeyToInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatorStatus_0(_) => {
                    <validatorStatus_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatorStatus_1(_) => {
                    <validatorStatus_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyCheckpointProofs(_) => {
                    <verifyCheckpointProofsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyStaleBalance(_) => {
                    <verifyStaleBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyWithdrawalCredentials(_) => {
                    <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::version(_) => <versionCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::withdrawRestakedBeaconChainETH(_) => {
                    <withdrawRestakedBeaconChainETHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawableRestakedExecutionLayerGwei(_) => {
                    <withdrawableRestakedExecutionLayerGweiCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(selector: [u8; 4], data: &[u8]) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<EigenPodCalls>] = &[
                {
                    fn verifyStaleBalance(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::verifyStaleBalance)
                    }
                    verifyStaleBalance
                },
                {
                    fn podOwner(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <podOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::podOwner)
                    }
                    podOwner
                },
                {
                    fn getConsolidationRequestFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <getConsolidationRequestFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodCalls::getConsolidationRequestFee)
                    }
                    getConsolidationRequestFee
                },
                {
                    fn activeValidatorCount(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <activeValidatorCountCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::activeValidatorCount)
                    }
                    activeValidatorCount
                },
                {
                    fn withdrawableRestakedExecutionLayerGwei(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <withdrawableRestakedExecutionLayerGweiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodCalls::withdrawableRestakedExecutionLayerGwei)
                    }
                    withdrawableRestakedExecutionLayerGwei
                },
                {
                    fn requestWithdrawal(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <requestWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::requestWithdrawal)
                    }
                    requestWithdrawal
                },
                {
                    fn verifyWithdrawalCredentials(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodCalls::verifyWithdrawalCredentials)
                    }
                    verifyWithdrawalCredentials
                },
                {
                    fn currentCheckpointTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <currentCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodCalls::currentCheckpointTimestamp)
                    }
                    currentCheckpointTimestamp
                },
                {
                    fn eigenPodManager(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::eigenPodManager)
                    }
                    eigenPodManager
                },
                {
                    fn currentCheckpoint(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <currentCheckpointCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::currentCheckpoint)
                    }
                    currentCheckpoint
                },
                {
                    fn checkpointBalanceExitedGwei(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <checkpointBalanceExitedGweiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodCalls::checkpointBalanceExitedGwei)
                    }
                    checkpointBalanceExitedGwei
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::version)
                    }
                    version
                },
                {
                    fn proofSubmitter(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <proofSubmitterCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::proofSubmitter)
                    }
                    proofSubmitter
                },
                {
                    fn validatorStatus_0(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <validatorStatus_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::validatorStatus_0)
                    }
                    validatorStatus_0
                },
                {
                    fn requestConsolidation(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <requestConsolidationCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::requestConsolidation)
                    }
                    requestConsolidation
                },
                {
                    fn getParentBlockRoot(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <getParentBlockRootCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::getParentBlockRoot)
                    }
                    getParentBlockRoot
                },
                {
                    fn validatorPubkeyHashToInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <validatorPubkeyHashToInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(EigenPodCalls::validatorPubkeyHashToInfo)
                    }
                    validatorPubkeyHashToInfo
                },
                {
                    fn validatorStatus_1(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <validatorStatus_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::validatorStatus_1)
                    }
                    validatorStatus_1
                },
                {
                    fn ethPOS(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <ethPOSCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::ethPOS)
                    }
                    ethPOS
                },
                {
                    fn startCheckpoint(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <startCheckpointCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::startCheckpoint)
                    }
                    startCheckpoint
                },
                {
                    fn stake(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <stakeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::stake)
                    }
                    stake
                },
                {
                    fn validatorPubkeyToInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <validatorPubkeyToInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(EigenPodCalls::validatorPubkeyToInfo)
                    }
                    validatorPubkeyToInfo
                },
                {
                    fn getWithdrawalRequestFee(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <getWithdrawalRequestFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(EigenPodCalls::getWithdrawalRequestFee)
                    }
                    getWithdrawalRequestFee
                },
                {
                    fn withdrawRestakedBeaconChainETH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <withdrawRestakedBeaconChainETHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodCalls::withdrawRestakedBeaconChainETH)
                    }
                    withdrawRestakedBeaconChainETH
                },
                {
                    fn initialize(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::initialize)
                    }
                    initialize
                },
                {
                    fn setProofSubmitter(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <setProofSubmitterCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::setProofSubmitter)
                    }
                    setProofSubmitter
                },
                {
                    fn recoverTokens(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                        <recoverTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(EigenPodCalls::recoverTokens)
                    }
                    recoverTokens
                },
                {
                    fn lastCheckpointTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <lastCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(EigenPodCalls::lastCheckpointTimestamp)
                    }
                    lastCheckpointTimestamp
                },
                {
                    fn verifyCheckpointProofs(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <verifyCheckpointProofsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(EigenPodCalls::verifyCheckpointProofs)
                    }
                    verifyCheckpointProofs
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<EigenPodCalls>] =
                &[
                    {
                        fn verifyStaleBalance(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::verifyStaleBalance)
                        }
                        verifyStaleBalance
                    },
                    {
                        fn podOwner(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                            <podOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::podOwner)
                        }
                        podOwner
                    },
                    {
                        fn getConsolidationRequestFee(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <getConsolidationRequestFeeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::getConsolidationRequestFee)
                        }
                        getConsolidationRequestFee
                    },
                    {
                        fn activeValidatorCount(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <activeValidatorCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::activeValidatorCount)
                        }
                        activeValidatorCount
                    },
                    {
                        fn withdrawableRestakedExecutionLayerGwei(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <withdrawableRestakedExecutionLayerGweiCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::withdrawableRestakedExecutionLayerGwei)
                        }
                        withdrawableRestakedExecutionLayerGwei
                    },
                    {
                        fn requestWithdrawal(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <requestWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::requestWithdrawal)
                        }
                        requestWithdrawal
                    },
                    {
                        fn verifyWithdrawalCredentials(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::verifyWithdrawalCredentials)
                        }
                        verifyWithdrawalCredentials
                    },
                    {
                        fn currentCheckpointTimestamp(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <currentCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::currentCheckpointTimestamp)
                        }
                        currentCheckpointTimestamp
                    },
                    {
                        fn eigenPodManager(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                            <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::eigenPodManager)
                        }
                        eigenPodManager
                    },
                    {
                        fn currentCheckpoint(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <currentCheckpointCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::currentCheckpoint)
                        }
                        currentCheckpoint
                    },
                    {
                        fn checkpointBalanceExitedGwei(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <checkpointBalanceExitedGweiCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::checkpointBalanceExitedGwei)
                        }
                        checkpointBalanceExitedGwei
                    },
                    {
                        fn version(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                            <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                                .map(EigenPodCalls::version)
                        }
                        version
                    },
                    {
                        fn proofSubmitter(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                            <proofSubmitterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::proofSubmitter)
                        }
                        proofSubmitter
                    },
                    {
                        fn validatorStatus_0(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <validatorStatus_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::validatorStatus_0)
                        }
                        validatorStatus_0
                    },
                    {
                        fn requestConsolidation(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <requestConsolidationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::requestConsolidation)
                        }
                        requestConsolidation
                    },
                    {
                        fn getParentBlockRoot(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <getParentBlockRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::getParentBlockRoot)
                        }
                        getParentBlockRoot
                    },
                    {
                        fn validatorPubkeyHashToInfo(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <validatorPubkeyHashToInfoCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::validatorPubkeyHashToInfo)
                        }
                        validatorPubkeyHashToInfo
                    },
                    {
                        fn validatorStatus_1(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <validatorStatus_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::validatorStatus_1)
                        }
                        validatorStatus_1
                    },
                    {
                        fn ethPOS(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                            <ethPOSCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                                .map(EigenPodCalls::ethPOS)
                        }
                        ethPOS
                    },
                    {
                        fn startCheckpoint(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                            <startCheckpointCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::startCheckpoint)
                        }
                        startCheckpoint
                    },
                    {
                        fn stake(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                            <stakeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                                .map(EigenPodCalls::stake)
                        }
                        stake
                    },
                    {
                        fn validatorPubkeyToInfo(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <validatorPubkeyToInfoCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::validatorPubkeyToInfo)
                        }
                        validatorPubkeyToInfo
                    },
                    {
                        fn getWithdrawalRequestFee(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <getWithdrawalRequestFeeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::getWithdrawalRequestFee)
                        }
                        getWithdrawalRequestFee
                    },
                    {
                        fn withdrawRestakedBeaconChainETH(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <withdrawRestakedBeaconChainETHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::withdrawRestakedBeaconChainETH)
                        }
                        withdrawRestakedBeaconChainETH
                    },
                    {
                        fn initialize(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                            <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::initialize)
                        }
                        initialize
                    },
                    {
                        fn setProofSubmitter(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <setProofSubmitterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::setProofSubmitter)
                        }
                        setProofSubmitter
                    },
                    {
                        fn recoverTokens(data: &[u8]) -> alloy_sol_types::Result<EigenPodCalls> {
                            <recoverTokensCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::recoverTokens)
                        }
                        recoverTokens
                    },
                    {
                        fn lastCheckpointTimestamp(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <lastCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::lastCheckpointTimestamp)
                        }
                        lastCheckpointTimestamp
                    },
                    {
                        fn verifyCheckpointProofs(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<EigenPodCalls> {
                            <verifyCheckpointProofsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodCalls::verifyCheckpointProofs)
                        }
                        verifyCheckpointProofs
                    },
                ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::activeValidatorCount(inner) => {
                    <activeValidatorCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkpointBalanceExitedGwei(inner) => {
                    <checkpointBalanceExitedGweiCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentCheckpoint(inner) => {
                    <currentCheckpointCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentCheckpointTimestamp(inner) => {
                    <currentCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ethPOS(inner) => {
                    <ethPOSCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getConsolidationRequestFee(inner) => {
                    <getConsolidationRequestFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getParentBlockRoot(inner) => {
                    <getParentBlockRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawalRequestFee(inner) => {
                    <getWithdrawalRequestFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lastCheckpointTimestamp(inner) => {
                    <lastCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::podOwner(inner) => {
                    <podOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proofSubmitter(inner) => {
                    <proofSubmitterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::recoverTokens(inner) => {
                    <recoverTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requestConsolidation(inner) => {
                    <requestConsolidationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::requestWithdrawal(inner) => {
                    <requestWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setProofSubmitter(inner) => {
                    <setProofSubmitterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stake(inner) => {
                    <stakeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::startCheckpoint(inner) => {
                    <startCheckpointCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatorPubkeyHashToInfo(inner) => {
                    <validatorPubkeyHashToInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatorPubkeyToInfo(inner) => {
                    <validatorPubkeyToInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatorStatus_0(inner) => {
                    <validatorStatus_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatorStatus_1(inner) => {
                    <validatorStatus_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyCheckpointProofs(inner) => {
                    <verifyCheckpointProofsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyStaleBalance(inner) => {
                    <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyWithdrawalCredentials(inner) => {
                    <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::withdrawRestakedBeaconChainETH(inner) => {
                    <withdrawRestakedBeaconChainETHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawableRestakedExecutionLayerGwei(inner) => {
                    <withdrawableRestakedExecutionLayerGweiCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::activeValidatorCount(inner) => {
                    <activeValidatorCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkpointBalanceExitedGwei(inner) => {
                    <checkpointBalanceExitedGweiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentCheckpoint(inner) => {
                    <currentCheckpointCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentCheckpointTimestamp(inner) => {
                    <currentCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ethPOS(inner) => {
                    <ethPOSCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getConsolidationRequestFee(inner) => {
                    <getConsolidationRequestFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getParentBlockRoot(inner) => {
                    <getParentBlockRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawalRequestFee(inner) => {
                    <getWithdrawalRequestFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastCheckpointTimestamp(inner) => {
                    <lastCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::podOwner(inner) => {
                    <podOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::proofSubmitter(inner) => {
                    <proofSubmitterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::recoverTokens(inner) => {
                    <recoverTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::requestConsolidation(inner) => {
                    <requestConsolidationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::requestWithdrawal(inner) => {
                    <requestWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setProofSubmitter(inner) => {
                    <setProofSubmitterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stake(inner) => {
                    <stakeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::startCheckpoint(inner) => {
                    <startCheckpointCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatorPubkeyHashToInfo(inner) => {
                    <validatorPubkeyHashToInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatorPubkeyToInfo(inner) => {
                    <validatorPubkeyToInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatorStatus_0(inner) => {
                    <validatorStatus_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatorStatus_1(inner) => {
                    <validatorStatus_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyCheckpointProofs(inner) => {
                    <verifyCheckpointProofsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyStaleBalance(inner) => {
                    <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyWithdrawalCredentials(inner) => {
                    <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::withdrawRestakedBeaconChainETH(inner) => {
                    <withdrawRestakedBeaconChainETHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawableRestakedExecutionLayerGwei(inner) => {
                    <withdrawableRestakedExecutionLayerGweiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`EigenPod`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum EigenPodErrors {
        #[allow(missing_docs)]
        BeaconTimestampBeforeLatestCheckpoint(BeaconTimestampBeforeLatestCheckpoint),
        #[allow(missing_docs)]
        BeaconTimestampTooFarInPast(BeaconTimestampTooFarInPast),
        #[allow(missing_docs)]
        CannotCheckpointTwiceInSingleBlock(CannotCheckpointTwiceInSingleBlock),
        #[allow(missing_docs)]
        CheckpointAlreadyActive(CheckpointAlreadyActive),
        #[allow(missing_docs)]
        CredentialsAlreadyVerified(CredentialsAlreadyVerified),
        #[allow(missing_docs)]
        CurrentlyPaused(CurrentlyPaused),
        #[allow(missing_docs)]
        EmptyRoot(EmptyRoot),
        #[allow(missing_docs)]
        FeeQueryFailed(FeeQueryFailed),
        #[allow(missing_docs)]
        ForkTimestampZero(ForkTimestampZero),
        #[allow(missing_docs)]
        InputAddressZero(InputAddressZero),
        #[allow(missing_docs)]
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        #[allow(missing_docs)]
        InsufficientFunds(InsufficientFunds),
        #[allow(missing_docs)]
        InsufficientWithdrawableBalance(InsufficientWithdrawableBalance),
        #[allow(missing_docs)]
        InvalidEIP4788Response(InvalidEIP4788Response),
        #[allow(missing_docs)]
        InvalidIndex(InvalidIndex),
        #[allow(missing_docs)]
        InvalidProof(InvalidProof),
        #[allow(missing_docs)]
        InvalidProofLength(InvalidProofLength),
        #[allow(missing_docs)]
        InvalidPubKeyLength(InvalidPubKeyLength),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        InvalidValidatorFieldsLength(InvalidValidatorFieldsLength),
        #[allow(missing_docs)]
        LeavesNotPowerOfTwo(LeavesNotPowerOfTwo),
        #[allow(missing_docs)]
        MsgValueNot32ETH(MsgValueNot32ETH),
        #[allow(missing_docs)]
        NoActiveCheckpoint(NoActiveCheckpoint),
        #[allow(missing_docs)]
        NoBalanceToCheckpoint(NoBalanceToCheckpoint),
        #[allow(missing_docs)]
        NotEnoughLeaves(NotEnoughLeaves),
        #[allow(missing_docs)]
        OnlyEigenPodManager(OnlyEigenPodManager),
        #[allow(missing_docs)]
        OnlyEigenPodOwner(OnlyEigenPodOwner),
        #[allow(missing_docs)]
        OnlyEigenPodOwnerOrProofSubmitter(OnlyEigenPodOwnerOrProofSubmitter),
        #[allow(missing_docs)]
        PredeployFailed(PredeployFailed),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
        #[allow(missing_docs)]
        TimestampOutOfRange(TimestampOutOfRange),
        #[allow(missing_docs)]
        ValidatorInactiveOnBeaconChain(ValidatorInactiveOnBeaconChain),
        #[allow(missing_docs)]
        ValidatorIsExitingBeaconChain(ValidatorIsExitingBeaconChain),
        #[allow(missing_docs)]
        ValidatorNotActiveInPod(ValidatorNotActiveInPod),
        #[allow(missing_docs)]
        ValidatorNotSlashedOnBeaconChain(ValidatorNotSlashedOnBeaconChain),
        #[allow(missing_docs)]
        WithdrawalCredentialsNotForEigenPod(WithdrawalCredentialsNotForEigenPod),
    }
    #[automatically_derived]
    impl EigenPodErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [9u8, 189u8, 227u8, 57u8],
            [11u8, 27u8, 213u8, 28u8],
            [26u8, 84u8, 79u8, 73u8],
            [32u8, 5u8, 145u8, 189u8],
            [36u8, 180u8, 181u8, 152u8],
            [46u8, 173u8, 230u8, 55u8],
            [48u8, 90u8, 39u8, 169u8],
            [53u8, 102u8, 128u8, 183u8],
            [53u8, 224u8, 158u8, 157u8],
            [53u8, 231u8, 246u8, 183u8],
            [55u8, 224u8, 127u8, 253u8],
            [65u8, 160u8, 44u8, 201u8],
            [66u8, 122u8, 119u8, 121u8],
            [67u8, 113u8, 74u8, 253u8],
            [77u8, 197u8, 246u8, 164u8],
            [83u8, 206u8, 78u8, 206u8],
            [85u8, 138u8, 208u8, 163u8],
            [99u8, 223u8, 129u8, 113u8],
            [101u8, 96u8, 141u8, 180u8],
            [103u8, 219u8, 91u8, 139u8],
            [110u8, 229u8, 186u8, 166u8],
            [115u8, 99u8, 33u8, 118u8],
            [132u8, 10u8, 72u8, 213u8],
            [159u8, 16u8, 100u8, 114u8],
            [176u8, 231u8, 47u8, 104u8],
            [179u8, 81u8, 43u8, 12u8],
            [190u8, 155u8, 195u8, 0u8],
            [200u8, 78u8, 153u8, 132u8],
            [201u8, 1u8, 88u8, 175u8],
            [203u8, 122u8, 165u8, 100u8],
            [212u8, 158u8, 25u8, 167u8],
            [227u8, 62u8, 110u8, 6u8],
            [242u8, 137u8, 204u8, 218u8],
            [246u8, 85u8, 143u8, 81u8],
            [248u8, 239u8, 3u8, 103u8],
            [252u8, 82u8, 212u8, 131u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EigenPodErrors {
        const NAME: &'static str = "EigenPodErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 36usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BeaconTimestampBeforeLatestCheckpoint(_) => {
                    <BeaconTimestampBeforeLatestCheckpoint as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BeaconTimestampTooFarInPast(_) => {
                    <BeaconTimestampTooFarInPast as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotCheckpointTwiceInSingleBlock(_) => {
                    <CannotCheckpointTwiceInSingleBlock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CheckpointAlreadyActive(_) => {
                    <CheckpointAlreadyActive as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CredentialsAlreadyVerified(_) => {
                    <CredentialsAlreadyVerified as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyRoot(_) => <EmptyRoot as alloy_sol_types::SolError>::SELECTOR,
                Self::FeeQueryFailed(_) => <FeeQueryFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ForkTimestampZero(_) => {
                    <ForkTimestampZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthMismatch(_) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientFunds(_) => {
                    <InsufficientFunds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientWithdrawableBalance(_) => {
                    <InsufficientWithdrawableBalance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidEIP4788Response(_) => {
                    <InvalidEIP4788Response as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidIndex(_) => <InvalidIndex as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidProof(_) => <InvalidProof as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidProofLength(_) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPubKeyLength(_) => {
                    <InvalidPubKeyLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidValidatorFieldsLength(_) => {
                    <InvalidValidatorFieldsLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LeavesNotPowerOfTwo(_) => {
                    <LeavesNotPowerOfTwo as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MsgValueNot32ETH(_) => {
                    <MsgValueNot32ETH as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoActiveCheckpoint(_) => {
                    <NoActiveCheckpoint as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoBalanceToCheckpoint(_) => {
                    <NoBalanceToCheckpoint as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotEnoughLeaves(_) => {
                    <NotEnoughLeaves as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyEigenPodManager(_) => {
                    <OnlyEigenPodManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyEigenPodOwner(_) => {
                    <OnlyEigenPodOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyEigenPodOwnerOrProofSubmitter(_) => {
                    <OnlyEigenPodOwnerOrProofSubmitter as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PredeployFailed(_) => {
                    <PredeployFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringTooLong(_) => <StringTooLong as alloy_sol_types::SolError>::SELECTOR,
                Self::TimestampOutOfRange(_) => {
                    <TimestampOutOfRange as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ValidatorInactiveOnBeaconChain(_) => {
                    <ValidatorInactiveOnBeaconChain as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ValidatorIsExitingBeaconChain(_) => {
                    <ValidatorIsExitingBeaconChain as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ValidatorNotActiveInPod(_) => {
                    <ValidatorNotActiveInPod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ValidatorNotSlashedOnBeaconChain(_) => {
                    <ValidatorNotSlashedOnBeaconChain as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawalCredentialsNotForEigenPod(_) => {
                    <WithdrawalCredentialsNotForEigenPod as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(selector: [u8; 4], data: &[u8]) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<EigenPodErrors>] = &[
                {
                    fn InvalidProof(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidProof as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::InvalidProof)
                    }
                    InvalidProof
                },
                {
                    fn InsufficientWithdrawableBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InsufficientWithdrawableBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodErrors::InsufficientWithdrawableBalance)
                    }
                    InsufficientWithdrawableBalance
                },
                {
                    fn NoActiveCheckpoint(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <NoActiveCheckpoint as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::NoActiveCheckpoint)
                    }
                    NoActiveCheckpoint
                },
                {
                    fn InvalidValidatorFieldsLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidValidatorFieldsLength as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(EigenPodErrors::InvalidValidatorFieldsLength)
                    }
                    InvalidValidatorFieldsLength
                },
                {
                    fn MsgValueNot32ETH(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <MsgValueNot32ETH as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::MsgValueNot32ETH)
                    }
                    MsgValueNot32ETH
                },
                {
                    fn ValidatorIsExitingBeaconChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ValidatorIsExitingBeaconChain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodErrors::ValidatorIsExitingBeaconChain)
                    }
                    ValidatorIsExitingBeaconChain
                },
                {
                    fn StringTooLong(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn InsufficientFunds(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InsufficientFunds as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::InsufficientFunds)
                    }
                    InsufficientFunds
                },
                {
                    fn CredentialsAlreadyVerified(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <CredentialsAlreadyVerified as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(EigenPodErrors::CredentialsAlreadyVerified)
                    }
                    CredentialsAlreadyVerified
                },
                {
                    fn BeaconTimestampBeforeLatestCheckpoint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <BeaconTimestampBeforeLatestCheckpoint as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodErrors::BeaconTimestampBeforeLatestCheckpoint)
                    }
                    BeaconTimestampBeforeLatestCheckpoint
                },
                {
                    fn BeaconTimestampTooFarInPast(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <BeaconTimestampTooFarInPast as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(EigenPodErrors::BeaconTimestampTooFarInPast)
                    }
                    BeaconTimestampTooFarInPast
                },
                {
                    fn ForkTimestampZero(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ForkTimestampZero as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::ForkTimestampZero)
                    }
                    ForkTimestampZero
                },
                {
                    fn OnlyEigenPodOwnerOrProofSubmitter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <OnlyEigenPodOwnerOrProofSubmitter as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodErrors::OnlyEigenPodOwnerOrProofSubmitter)
                    }
                    OnlyEigenPodOwnerOrProofSubmitter
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(EigenPodErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn InvalidProofLength(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidProofLength as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::InvalidProofLength)
                    }
                    InvalidProofLength
                },
                {
                    fn EmptyRoot(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <EmptyRoot as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::EmptyRoot)
                    }
                    EmptyRoot
                },
                {
                    fn InvalidEIP4788Response(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidEIP4788Response as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::InvalidEIP4788Response)
                    }
                    InvalidEIP4788Response
                },
                {
                    fn InvalidIndex(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidIndex as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::InvalidIndex)
                    }
                    InvalidIndex
                },
                {
                    fn ValidatorInactiveOnBeaconChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ValidatorInactiveOnBeaconChain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodErrors::ValidatorInactiveOnBeaconChain)
                    }
                    ValidatorInactiveOnBeaconChain
                },
                {
                    fn CannotCheckpointTwiceInSingleBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <CannotCheckpointTwiceInSingleBlock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodErrors::CannotCheckpointTwiceInSingleBlock)
                    }
                    CannotCheckpointTwiceInSingleBlock
                },
                {
                    fn WithdrawalCredentialsNotForEigenPod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <WithdrawalCredentialsNotForEigenPod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodErrors::WithdrawalCredentialsNotForEigenPod)
                    }
                    WithdrawalCredentialsNotForEigenPod
                },
                {
                    fn InputAddressZero(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn CurrentlyPaused(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidPubKeyLength(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidPubKeyLength as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::InvalidPubKeyLength)
                    }
                    InvalidPubKeyLength
                },
                {
                    fn ValidatorNotSlashedOnBeaconChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ValidatorNotSlashedOnBeaconChain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(EigenPodErrors::ValidatorNotSlashedOnBeaconChain)
                    }
                    ValidatorNotSlashedOnBeaconChain
                },
                {
                    fn InvalidShortString(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn CheckpointAlreadyActive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <CheckpointAlreadyActive as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::CheckpointAlreadyActive)
                    }
                    CheckpointAlreadyActive
                },
                {
                    fn OnlyEigenPodManager(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::OnlyEigenPodManager)
                    }
                    OnlyEigenPodManager
                },
                {
                    fn FeeQueryFailed(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <FeeQueryFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::FeeQueryFailed)
                    }
                    FeeQueryFailed
                },
                {
                    fn NoBalanceToCheckpoint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <NoBalanceToCheckpoint as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::NoBalanceToCheckpoint)
                    }
                    NoBalanceToCheckpoint
                },
                {
                    fn ValidatorNotActiveInPod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ValidatorNotActiveInPod as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::ValidatorNotActiveInPod)
                    }
                    ValidatorNotActiveInPod
                },
                {
                    fn OnlyEigenPodOwner(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <OnlyEigenPodOwner as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::OnlyEigenPodOwner)
                    }
                    OnlyEigenPodOwner
                },
                {
                    fn TimestampOutOfRange(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <TimestampOutOfRange as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::TimestampOutOfRange)
                    }
                    TimestampOutOfRange
                },
                {
                    fn LeavesNotPowerOfTwo(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <LeavesNotPowerOfTwo as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::LeavesNotPowerOfTwo)
                    }
                    LeavesNotPowerOfTwo
                },
                {
                    fn NotEnoughLeaves(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <NotEnoughLeaves as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::NotEnoughLeaves)
                    }
                    NotEnoughLeaves
                },
                {
                    fn PredeployFailed(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <PredeployFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(EigenPodErrors::PredeployFailed)
                    }
                    PredeployFailed
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_SHIMS[idx](data)
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw_validate(
            selector: [u8; 4],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_VALIDATE_SHIMS: &[fn(
                &[u8],
            )
                -> alloy_sol_types::Result<EigenPodErrors>] = &[
                {
                    fn InvalidProof(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidProof as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(EigenPodErrors::InvalidProof)
                    }
                    InvalidProof
                },
                {
                    fn InsufficientWithdrawableBalance(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InsufficientWithdrawableBalance as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::InsufficientWithdrawableBalance)
                    }
                    InsufficientWithdrawableBalance
                },
                {
                    fn NoActiveCheckpoint(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <NoActiveCheckpoint as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::NoActiveCheckpoint)
                    }
                    NoActiveCheckpoint
                },
                {
                    fn InvalidValidatorFieldsLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidValidatorFieldsLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::InvalidValidatorFieldsLength)
                    }
                    InvalidValidatorFieldsLength
                },
                {
                    fn MsgValueNot32ETH(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <MsgValueNot32ETH as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::MsgValueNot32ETH)
                    }
                    MsgValueNot32ETH
                },
                {
                    fn ValidatorIsExitingBeaconChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ValidatorIsExitingBeaconChain as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::ValidatorIsExitingBeaconChain)
                    }
                    ValidatorIsExitingBeaconChain
                },
                {
                    fn StringTooLong(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(EigenPodErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn InsufficientFunds(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InsufficientFunds as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::InsufficientFunds)
                    }
                    InsufficientFunds
                },
                {
                    fn CredentialsAlreadyVerified(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <CredentialsAlreadyVerified as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::CredentialsAlreadyVerified)
                    }
                    CredentialsAlreadyVerified
                },
                {
                    fn BeaconTimestampBeforeLatestCheckpoint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <BeaconTimestampBeforeLatestCheckpoint as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::BeaconTimestampBeforeLatestCheckpoint)
                    }
                    BeaconTimestampBeforeLatestCheckpoint
                },
                {
                    fn BeaconTimestampTooFarInPast(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <BeaconTimestampTooFarInPast as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::BeaconTimestampTooFarInPast)
                    }
                    BeaconTimestampTooFarInPast
                },
                {
                    fn ForkTimestampZero(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ForkTimestampZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::ForkTimestampZero)
                    }
                    ForkTimestampZero
                },
                {
                    fn OnlyEigenPodOwnerOrProofSubmitter(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <OnlyEigenPodOwnerOrProofSubmitter as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::OnlyEigenPodOwnerOrProofSubmitter)
                    }
                    OnlyEigenPodOwnerOrProofSubmitter
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn InvalidProofLength(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidProofLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::InvalidProofLength)
                    }
                    InvalidProofLength
                },
                {
                    fn EmptyRoot(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <EmptyRoot as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(EigenPodErrors::EmptyRoot)
                    }
                    EmptyRoot
                },
                {
                    fn InvalidEIP4788Response(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidEIP4788Response as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::InvalidEIP4788Response)
                    }
                    InvalidEIP4788Response
                },
                {
                    fn InvalidIndex(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(EigenPodErrors::InvalidIndex)
                    }
                    InvalidIndex
                },
                {
                    fn ValidatorInactiveOnBeaconChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ValidatorInactiveOnBeaconChain as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::ValidatorInactiveOnBeaconChain)
                    }
                    ValidatorInactiveOnBeaconChain
                },
                {
                    fn CannotCheckpointTwiceInSingleBlock(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <CannotCheckpointTwiceInSingleBlock as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::CannotCheckpointTwiceInSingleBlock)
                    }
                    CannotCheckpointTwiceInSingleBlock
                },
                {
                    fn WithdrawalCredentialsNotForEigenPod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <WithdrawalCredentialsNotForEigenPod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::WithdrawalCredentialsNotForEigenPod)
                    }
                    WithdrawalCredentialsNotForEigenPod
                },
                {
                    fn InputAddressZero(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn CurrentlyPaused(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidPubKeyLength(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidPubKeyLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::InvalidPubKeyLength)
                    }
                    InvalidPubKeyLength
                },
                {
                    fn ValidatorNotSlashedOnBeaconChain(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ValidatorNotSlashedOnBeaconChain as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::ValidatorNotSlashedOnBeaconChain)
                    }
                    ValidatorNotSlashedOnBeaconChain
                },
                {
                    fn InvalidShortString(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn CheckpointAlreadyActive(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <CheckpointAlreadyActive as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::CheckpointAlreadyActive)
                    }
                    CheckpointAlreadyActive
                },
                {
                    fn OnlyEigenPodManager(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::OnlyEigenPodManager)
                    }
                    OnlyEigenPodManager
                },
                {
                    fn FeeQueryFailed(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <FeeQueryFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(EigenPodErrors::FeeQueryFailed)
                    }
                    FeeQueryFailed
                },
                {
                    fn NoBalanceToCheckpoint(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <NoBalanceToCheckpoint as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::NoBalanceToCheckpoint)
                    }
                    NoBalanceToCheckpoint
                },
                {
                    fn ValidatorNotActiveInPod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ValidatorNotActiveInPod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(EigenPodErrors::ValidatorNotActiveInPod)
                    }
                    ValidatorNotActiveInPod
                },
                {
                    fn OnlyEigenPodOwner(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <OnlyEigenPodOwner as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::OnlyEigenPodOwner)
                    }
                    OnlyEigenPodOwner
                },
                {
                    fn TimestampOutOfRange(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <TimestampOutOfRange as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::TimestampOutOfRange)
                    }
                    TimestampOutOfRange
                },
                {
                    fn LeavesNotPowerOfTwo(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <LeavesNotPowerOfTwo as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::LeavesNotPowerOfTwo)
                    }
                    LeavesNotPowerOfTwo
                },
                {
                    fn NotEnoughLeaves(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <NotEnoughLeaves as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::NotEnoughLeaves)
                    }
                    NotEnoughLeaves
                },
                {
                    fn PredeployFailed(data: &[u8]) -> alloy_sol_types::Result<EigenPodErrors> {
                        <PredeployFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(EigenPodErrors::PredeployFailed)
                    }
                    PredeployFailed
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(alloy_sol_types::Error::unknown_selector(
                    <Self as alloy_sol_types::SolInterface>::NAME,
                    selector,
                ));
            };
            DECODE_VALIDATE_SHIMS[idx](data)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::BeaconTimestampBeforeLatestCheckpoint(inner) => {
                    <BeaconTimestampBeforeLatestCheckpoint as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BeaconTimestampTooFarInPast(inner) => {
                    <BeaconTimestampTooFarInPast as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotCheckpointTwiceInSingleBlock(inner) => {
                    <CannotCheckpointTwiceInSingleBlock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CheckpointAlreadyActive(inner) => {
                    <CheckpointAlreadyActive as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CredentialsAlreadyVerified(inner) => {
                    <CredentialsAlreadyVerified as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyRoot(inner) => {
                    <EmptyRoot as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FeeQueryFailed(inner) => {
                    <FeeQueryFailed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ForkTimestampZero(inner) => {
                    <ForkTimestampZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientFunds(inner) => {
                    <InsufficientFunds as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientWithdrawableBalance(inner) => {
                    <InsufficientWithdrawableBalance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidEIP4788Response(inner) => {
                    <InvalidEIP4788Response as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidIndex(inner) => {
                    <InvalidIndex as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidProof(inner) => {
                    <InvalidProof as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidProofLength(inner) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidPubKeyLength(inner) => {
                    <InvalidPubKeyLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidValidatorFieldsLength(inner) => {
                    <InvalidValidatorFieldsLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LeavesNotPowerOfTwo(inner) => {
                    <LeavesNotPowerOfTwo as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MsgValueNot32ETH(inner) => {
                    <MsgValueNot32ETH as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoActiveCheckpoint(inner) => {
                    <NoActiveCheckpoint as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoBalanceToCheckpoint(inner) => {
                    <NoBalanceToCheckpoint as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotEnoughLeaves(inner) => {
                    <NotEnoughLeaves as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyEigenPodManager(inner) => {
                    <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyEigenPodOwner(inner) => {
                    <OnlyEigenPodOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyEigenPodOwnerOrProofSubmitter(inner) => {
                    <OnlyEigenPodOwnerOrProofSubmitter as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PredeployFailed(inner) => {
                    <PredeployFailed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::TimestampOutOfRange(inner) => {
                    <TimestampOutOfRange as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ValidatorInactiveOnBeaconChain(inner) => {
                    <ValidatorInactiveOnBeaconChain as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ValidatorIsExitingBeaconChain(inner) => {
                    <ValidatorIsExitingBeaconChain as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ValidatorNotActiveInPod(inner) => {
                    <ValidatorNotActiveInPod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ValidatorNotSlashedOnBeaconChain(inner) => {
                    <ValidatorNotSlashedOnBeaconChain as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawalCredentialsNotForEigenPod(inner) => {
                    <WithdrawalCredentialsNotForEigenPod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BeaconTimestampBeforeLatestCheckpoint(inner) => {
                    <BeaconTimestampBeforeLatestCheckpoint as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BeaconTimestampTooFarInPast(inner) => {
                    <BeaconTimestampTooFarInPast as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotCheckpointTwiceInSingleBlock(inner) => {
                    <CannotCheckpointTwiceInSingleBlock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CheckpointAlreadyActive(inner) => {
                    <CheckpointAlreadyActive as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CredentialsAlreadyVerified(inner) => {
                    <CredentialsAlreadyVerified as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyRoot(inner) => {
                    <EmptyRoot as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::FeeQueryFailed(inner) => {
                    <FeeQueryFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ForkTimestampZero(inner) => {
                    <ForkTimestampZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientFunds(inner) => {
                    <InsufficientFunds as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientWithdrawableBalance(inner) => {
                    <InsufficientWithdrawableBalance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidEIP4788Response(inner) => {
                    <InvalidEIP4788Response as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidIndex(inner) => {
                    <InvalidIndex as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidProof(inner) => {
                    <InvalidProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidProofLength(inner) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidPubKeyLength(inner) => {
                    <InvalidPubKeyLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidValidatorFieldsLength(inner) => {
                    <InvalidValidatorFieldsLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LeavesNotPowerOfTwo(inner) => {
                    <LeavesNotPowerOfTwo as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MsgValueNot32ETH(inner) => {
                    <MsgValueNot32ETH as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoActiveCheckpoint(inner) => {
                    <NoActiveCheckpoint as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoBalanceToCheckpoint(inner) => {
                    <NoBalanceToCheckpoint as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotEnoughLeaves(inner) => {
                    <NotEnoughLeaves as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyEigenPodManager(inner) => {
                    <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyEigenPodOwner(inner) => {
                    <OnlyEigenPodOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyEigenPodOwnerOrProofSubmitter(inner) => {
                    <OnlyEigenPodOwnerOrProofSubmitter as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PredeployFailed(inner) => {
                    <PredeployFailed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimestampOutOfRange(inner) => {
                    <TimestampOutOfRange as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ValidatorInactiveOnBeaconChain(inner) => {
                    <ValidatorInactiveOnBeaconChain as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ValidatorIsExitingBeaconChain(inner) => {
                    <ValidatorIsExitingBeaconChain as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ValidatorNotActiveInPod(inner) => {
                    <ValidatorNotActiveInPod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ValidatorNotSlashedOnBeaconChain(inner) => {
                    <ValidatorNotSlashedOnBeaconChain as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithdrawalCredentialsNotForEigenPod(inner) => {
                    <WithdrawalCredentialsNotForEigenPod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`EigenPod`](self) events.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum EigenPodEvents {
        #[allow(missing_docs)]
        CheckpointCreated(CheckpointCreated),
        #[allow(missing_docs)]
        CheckpointFinalized(CheckpointFinalized),
        #[allow(missing_docs)]
        ConsolidationRequested(ConsolidationRequested),
        #[allow(missing_docs)]
        EigenPodStaked(EigenPodStaked),
        #[allow(missing_docs)]
        ExitRequested(ExitRequested),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        NonBeaconChainETHReceived(NonBeaconChainETHReceived),
        #[allow(missing_docs)]
        ProofSubmitterUpdated(ProofSubmitterUpdated),
        #[allow(missing_docs)]
        RestakedBeaconChainETHWithdrawn(RestakedBeaconChainETHWithdrawn),
        #[allow(missing_docs)]
        SwitchToCompoundingRequested(SwitchToCompoundingRequested),
        #[allow(missing_docs)]
        ValidatorBalanceUpdated(ValidatorBalanceUpdated),
        #[allow(missing_docs)]
        ValidatorCheckpointed(ValidatorCheckpointed),
        #[allow(missing_docs)]
        ValidatorRestaked(ValidatorRestaked),
        #[allow(missing_docs)]
        ValidatorWithdrawn(ValidatorWithdrawn),
        #[allow(missing_docs)]
        WithdrawalRequested(WithdrawalRequested),
    }
    #[automatically_derived]
    impl EigenPodEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                16u8, 23u8, 144u8, 194u8, 153u8, 63u8, 106u8, 77u8, 150u8, 43u8, 209u8, 124u8,
                120u8, 97u8, 38u8, 130u8, 59u8, 161u8, 196u8, 207u8, 4u8, 255u8, 76u8, 204u8,
                178u8, 101u8, 157u8, 80u8, 251u8, 32u8, 174u8, 232u8,
            ],
            [
                66u8, 249u8, 201u8, 219u8, 44u8, 164u8, 67u8, 233u8, 236u8, 98u8, 244u8, 88u8,
                139u8, 208u8, 201u8, 178u8, 65u8, 6u8, 92u8, 2u8, 194u8, 168u8, 0u8, 26u8, 193u8,
                100u8, 174u8, 18u8, 130u8, 220u8, 123u8, 148u8,
            ],
            [
                82u8, 84u8, 8u8, 194u8, 1u8, 188u8, 21u8, 118u8, 235u8, 68u8, 17u8, 111u8, 100u8,
                120u8, 241u8, 194u8, 165u8, 71u8, 117u8, 177u8, 154u8, 4u8, 59u8, 207u8, 220u8,
                112u8, 131u8, 100u8, 247u8, 79u8, 142u8, 68u8,
            ],
            [
                87u8, 87u8, 150u8, 19u8, 59u8, 190u8, 211u8, 55u8, 229u8, 179u8, 154u8, 164u8,
                154u8, 48u8, 220u8, 37u8, 86u8, 169u8, 30u8, 12u8, 108u8, 42u8, 244u8, 183u8,
                184u8, 134u8, 174u8, 119u8, 235u8, 239u8, 16u8, 118u8,
            ],
            [
                92u8, 224u8, 170u8, 4u8, 174u8, 81u8, 213u8, 45u8, 166u8, 230u8, 128u8, 251u8,
                224u8, 51u8, 109u8, 46u8, 36u8, 50u8, 247u8, 195u8, 220u8, 45u8, 79u8, 49u8, 147u8,
                32u8, 76u8, 87u8, 185u8, 7u8, 33u8, 7u8,
            ],
            [
                96u8, 216u8, 202u8, 1u8, 77u8, 71u8, 101u8, 162u8, 184u8, 179u8, 137u8, 226u8,
                87u8, 20u8, 203u8, 28u8, 239u8, 131u8, 181u8, 116u8, 34u8, 41u8, 17u8, 160u8, 29u8,
                144u8, 193u8, 189u8, 105u8, 210u8, 211u8, 32u8,
            ],
            [
                111u8, 221u8, 61u8, 189u8, 177u8, 115u8, 41u8, 150u8, 8u8, 192u8, 170u8, 159u8,
                54u8, 135u8, 53u8, 133u8, 124u8, 136u8, 66u8, 181u8, 129u8, 248u8, 56u8, 146u8,
                56u8, 191u8, 5u8, 189u8, 4u8, 179u8, 191u8, 73u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                137u8, 71u8, 253u8, 44u8, 224u8, 126u8, 249u8, 204u8, 48u8, 44u8, 78u8, 143u8, 4u8,
                97u8, 1u8, 86u8, 21u8, 217u8, 28u8, 232u8, 81u8, 86u8, 72u8, 57u8, 233u8, 28u8,
                200u8, 4u8, 194u8, 244u8, 157u8, 142u8,
            ],
            [
                139u8, 39u8, 55u8, 187u8, 100u8, 171u8, 47u8, 45u8, 192u8, 149u8, 82u8, 223u8,
                161u8, 194u8, 80u8, 57u8, 158u8, 106u8, 66u8, 199u8, 234u8, 159u8, 14u8, 28u8,
                101u8, 143u8, 93u8, 101u8, 215u8, 8u8, 236u8, 5u8,
            ],
            [
                160u8, 16u8, 3u8, 118u8, 109u8, 60u8, 217u8, 124u8, 242u8, 173u8, 229u8, 66u8,
                150u8, 144u8, 191u8, 93u8, 32u8, 107u8, 231u8, 251u8, 1u8, 239u8, 157u8, 58u8, 0u8,
                137u8, 236u8, 246u8, 123u8, 193u8, 18u8, 25u8,
            ],
            [
                201u8, 123u8, 150u8, 91u8, 146u8, 174u8, 127u8, 210u8, 0u8, 149u8, 254u8, 142u8,
                183u8, 185u8, 159u8, 129u8, 249u8, 95u8, 140u8, 74u8, 223u8, 251u8, 34u8, 161u8,
                145u8, 22u8, 216u8, 235u8, 40u8, 70u8, 176u8, 22u8,
            ],
            [
                205u8, 174u8, 112u8, 13u8, 114u8, 65u8, 188u8, 2u8, 113u8, 104u8, 197u8, 60u8,
                246u8, 248u8, 137u8, 118u8, 59u8, 10u8, 44u8, 136u8, 166u8, 93u8, 119u8, 252u8,
                19u8, 168u8, 169u8, 254u8, 240u8, 216u8, 96u8, 95u8,
            ],
            [
                228u8, 134u8, 99u8, 53u8, 118u8, 26u8, 81u8, 220u8, 175u8, 247u8, 102u8, 68u8,
                138u8, 176u8, 175u8, 96u8, 100u8, 41u8, 30u8, 229u8, 220u8, 148u8, 230u8, 132u8,
                146u8, 187u8, 156u8, 215u8, 87u8, 193u8, 227u8, 80u8,
            ],
            [
                251u8, 129u8, 41u8, 8u8, 10u8, 25u8, 211u8, 77u8, 206u8, 172u8, 4u8, 186u8, 37u8,
                63u8, 197u8, 3u8, 4u8, 220u8, 134u8, 199u8, 41u8, 189u8, 99u8, 205u8, 202u8, 74u8,
                150u8, 154u8, 209u8, 154u8, 94u8, 172u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for EigenPodEvents {
        const NAME: &'static str = "EigenPodEvents";
        const COUNT: usize = 15usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<CheckpointCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CheckpointCreated as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::CheckpointCreated)
                }
                Some(<CheckpointFinalized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CheckpointFinalized as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::CheckpointFinalized)
                }
                Some(<ConsolidationRequested as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ConsolidationRequested as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::ConsolidationRequested)
                }
                Some(<EigenPodStaked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EigenPodStaked as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::EigenPodStaked)
                }
                Some(<ExitRequested as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ExitRequested as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::ExitRequested)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Initialized)
                }
                Some(<NonBeaconChainETHReceived as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NonBeaconChainETHReceived as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::NonBeaconChainETHReceived)
                }
                Some(<ProofSubmitterUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ProofSubmitterUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::ProofSubmitterUpdated)
                }
                Some(
                    <RestakedBeaconChainETHWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RestakedBeaconChainETHWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::RestakedBeaconChainETHWithdrawn)
                }
                Some(
                    <SwitchToCompoundingRequested as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <SwitchToCompoundingRequested as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data,
                )
                .map(Self::SwitchToCompoundingRequested),
                Some(<ValidatorBalanceUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ValidatorBalanceUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::ValidatorBalanceUpdated)
                }
                Some(<ValidatorCheckpointed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ValidatorCheckpointed as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::ValidatorCheckpointed)
                }
                Some(<ValidatorRestaked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ValidatorRestaked as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::ValidatorRestaked)
                }
                Some(<ValidatorWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ValidatorWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::ValidatorWithdrawn)
                }
                Some(<WithdrawalRequested as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <WithdrawalRequested as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::WithdrawalRequested)
                }
                _ => alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                    name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                    log: alloy_sol_types::private::Box::new(
                        alloy_sol_types::private::LogData::new_unchecked(
                            topics.to_vec(),
                            data.to_vec().into(),
                        ),
                    ),
                }),
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for EigenPodEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::CheckpointCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CheckpointFinalized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ConsolidationRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EigenPodStaked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ExitRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NonBeaconChainETHReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProofSubmitterUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RestakedBeaconChainETHWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SwitchToCompoundingRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidatorBalanceUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidatorCheckpointed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidatorRestaked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidatorWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::WithdrawalRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::CheckpointCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CheckpointFinalized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ConsolidationRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EigenPodStaked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ExitRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NonBeaconChainETHReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProofSubmitterUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RestakedBeaconChainETHWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SwitchToCompoundingRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidatorBalanceUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidatorCheckpointed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidatorRestaked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidatorWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WithdrawalRequested(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`EigenPod`](self) contract instance.

    See the [wrapper's documentation](`EigenPodInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EigenPodInstance<P, N> {
        EigenPodInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
        provider: P,
        _ethPOS: alloy::sol_types::private::Address,
        _eigenPodManager: alloy::sol_types::private::Address,
        _version: alloy::sol_types::private::String,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<EigenPodInstance<P, N>>> {
        EigenPodInstance::<P, N>::deploy(provider, _ethPOS, _eigenPodManager, _version)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
    and constructor arguments, if any.

    This is a simple wrapper around creating a `RawCallBuilder` with the data set to
    the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _ethPOS: alloy::sol_types::private::Address,
        _eigenPodManager: alloy::sol_types::private::Address,
        _version: alloy::sol_types::private::String,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        EigenPodInstance::<P, N>::deploy_builder(provider, _ethPOS, _eigenPodManager, _version)
    }
    /**A [`EigenPod`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`EigenPod`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EigenPodInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for EigenPodInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EigenPodInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        EigenPodInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`EigenPod`](self) contract instance.

        See the [wrapper's documentation](`EigenPodInstance`) for more details.*/
        #[inline]
        pub const fn new(address: alloy_sol_types::private::Address, provider: P) -> Self {
            Self {
                address,
                provider,
                _network: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

        Returns a new instance of the contract, if the deployment was successful.

        For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _ethPOS: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::Result<EigenPodInstance<P, N>> {
            let call_builder = Self::deploy_builder(provider, _ethPOS, _eigenPodManager, _version);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _ethPOS: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _ethPOS,
                        _eigenPodManager,
                        _version,
                    })[..],
                ]
                .concat()
                .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<P: ::core::clone::Clone, N> EigenPodInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EigenPodInstance<P, N> {
            EigenPodInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        EigenPodInstance<P, N>
    {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<&P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`activeValidatorCount`] function.
        pub fn activeValidatorCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, activeValidatorCountCall, N> {
            self.call_builder(&activeValidatorCountCall)
        }
        ///Creates a new call builder for the [`checkpointBalanceExitedGwei`] function.
        pub fn checkpointBalanceExitedGwei(
            &self,
            _0: u64,
        ) -> alloy_contract::SolCallBuilder<&P, checkpointBalanceExitedGweiCall, N> {
            self.call_builder(&checkpointBalanceExitedGweiCall(_0))
        }
        ///Creates a new call builder for the [`currentCheckpoint`] function.
        pub fn currentCheckpoint(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, currentCheckpointCall, N> {
            self.call_builder(&currentCheckpointCall)
        }
        ///Creates a new call builder for the [`currentCheckpointTimestamp`] function.
        pub fn currentCheckpointTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, currentCheckpointTimestampCall, N> {
            self.call_builder(&currentCheckpointTimestampCall)
        }
        ///Creates a new call builder for the [`eigenPodManager`] function.
        pub fn eigenPodManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, eigenPodManagerCall, N> {
            self.call_builder(&eigenPodManagerCall)
        }
        ///Creates a new call builder for the [`ethPOS`] function.
        pub fn ethPOS(&self) -> alloy_contract::SolCallBuilder<&P, ethPOSCall, N> {
            self.call_builder(&ethPOSCall)
        }
        ///Creates a new call builder for the [`getConsolidationRequestFee`] function.
        pub fn getConsolidationRequestFee(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getConsolidationRequestFeeCall, N> {
            self.call_builder(&getConsolidationRequestFeeCall)
        }
        ///Creates a new call builder for the [`getParentBlockRoot`] function.
        pub fn getParentBlockRoot(
            &self,
            timestamp: u64,
        ) -> alloy_contract::SolCallBuilder<&P, getParentBlockRootCall, N> {
            self.call_builder(&getParentBlockRootCall { timestamp })
        }
        ///Creates a new call builder for the [`getWithdrawalRequestFee`] function.
        pub fn getWithdrawalRequestFee(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getWithdrawalRequestFeeCall, N> {
            self.call_builder(&getWithdrawalRequestFeeCall)
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _podOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall { _podOwner })
        }
        ///Creates a new call builder for the [`lastCheckpointTimestamp`] function.
        pub fn lastCheckpointTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, lastCheckpointTimestampCall, N> {
            self.call_builder(&lastCheckpointTimestampCall)
        }
        ///Creates a new call builder for the [`podOwner`] function.
        pub fn podOwner(&self) -> alloy_contract::SolCallBuilder<&P, podOwnerCall, N> {
            self.call_builder(&podOwnerCall)
        }
        ///Creates a new call builder for the [`proofSubmitter`] function.
        pub fn proofSubmitter(&self) -> alloy_contract::SolCallBuilder<&P, proofSubmitterCall, N> {
            self.call_builder(&proofSubmitterCall)
        }
        ///Creates a new call builder for the [`recoverTokens`] function.
        pub fn recoverTokens(
            &self,
            tokenList: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            amountsToWithdraw: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, recoverTokensCall, N> {
            self.call_builder(&recoverTokensCall {
                tokenList,
                amountsToWithdraw,
                recipient,
            })
        }
        ///Creates a new call builder for the [`requestConsolidation`] function.
        pub fn requestConsolidation(
            &self,
            requests: alloy::sol_types::private::Vec<
                <IEigenPodTypes::ConsolidationRequest as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, requestConsolidationCall, N> {
            self.call_builder(&requestConsolidationCall { requests })
        }
        ///Creates a new call builder for the [`requestWithdrawal`] function.
        pub fn requestWithdrawal(
            &self,
            requests: alloy::sol_types::private::Vec<
                <IEigenPodTypes::WithdrawalRequest as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, requestWithdrawalCall, N> {
            self.call_builder(&requestWithdrawalCall { requests })
        }
        ///Creates a new call builder for the [`setProofSubmitter`] function.
        pub fn setProofSubmitter(
            &self,
            newProofSubmitter: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setProofSubmitterCall, N> {
            self.call_builder(&setProofSubmitterCall { newProofSubmitter })
        }
        ///Creates a new call builder for the [`stake`] function.
        pub fn stake(
            &self,
            pubkey: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
            depositDataRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, stakeCall, N> {
            self.call_builder(&stakeCall {
                pubkey,
                signature,
                depositDataRoot,
            })
        }
        ///Creates a new call builder for the [`startCheckpoint`] function.
        pub fn startCheckpoint(
            &self,
            revertIfNoBalance: bool,
        ) -> alloy_contract::SolCallBuilder<&P, startCheckpointCall, N> {
            self.call_builder(&startCheckpointCall { revertIfNoBalance })
        }
        ///Creates a new call builder for the [`validatorPubkeyHashToInfo`] function.
        pub fn validatorPubkeyHashToInfo(
            &self,
            validatorPubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, validatorPubkeyHashToInfoCall, N> {
            self.call_builder(&validatorPubkeyHashToInfoCall {
                validatorPubkeyHash,
            })
        }
        ///Creates a new call builder for the [`validatorPubkeyToInfo`] function.
        pub fn validatorPubkeyToInfo(
            &self,
            validatorPubkey: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, validatorPubkeyToInfoCall, N> {
            self.call_builder(&validatorPubkeyToInfoCall { validatorPubkey })
        }
        ///Creates a new call builder for the [`validatorStatus_0`] function.
        pub fn validatorStatus_0(
            &self,
            validatorPubkey: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, validatorStatus_0Call, N> {
            self.call_builder(&validatorStatus_0Call { validatorPubkey })
        }
        ///Creates a new call builder for the [`validatorStatus_1`] function.
        pub fn validatorStatus_1(
            &self,
            pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, validatorStatus_1Call, N> {
            self.call_builder(&validatorStatus_1Call { pubkeyHash })
        }
        ///Creates a new call builder for the [`verifyCheckpointProofs`] function.
        pub fn verifyCheckpointProofs(
            &self,
            balanceContainerProof: <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
            proofs: alloy::sol_types::private::Vec<
                <BeaconChainProofs::BalanceProof as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, verifyCheckpointProofsCall, N> {
            self.call_builder(&verifyCheckpointProofsCall {
                balanceContainerProof,
                proofs,
            })
        }
        ///Creates a new call builder for the [`verifyStaleBalance`] function.
        pub fn verifyStaleBalance(
            &self,
            beaconTimestamp: u64,
            stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
            proof: <BeaconChainProofs::ValidatorProof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, verifyStaleBalanceCall, N> {
            self.call_builder(&verifyStaleBalanceCall {
                beaconTimestamp,
                stateRootProof,
                proof,
            })
        }
        ///Creates a new call builder for the [`verifyWithdrawalCredentials`] function.
        pub fn verifyWithdrawalCredentials(
            &self,
            beaconTimestamp: u64,
            stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
            validatorIndices: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
            validatorFieldsProofs: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            validatorFields: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, verifyWithdrawalCredentialsCall, N> {
            self.call_builder(&verifyWithdrawalCredentialsCall {
                beaconTimestamp,
                stateRootProof,
                validatorIndices,
                validatorFieldsProofs,
                validatorFields,
            })
        }
        ///Creates a new call builder for the [`version`] function.
        pub fn version(&self) -> alloy_contract::SolCallBuilder<&P, versionCall, N> {
            self.call_builder(&versionCall)
        }
        ///Creates a new call builder for the [`withdrawRestakedBeaconChainETH`] function.
        pub fn withdrawRestakedBeaconChainETH(
            &self,
            recipient: alloy::sol_types::private::Address,
            amountWei: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, withdrawRestakedBeaconChainETHCall, N> {
            self.call_builder(&withdrawRestakedBeaconChainETHCall {
                recipient,
                amountWei,
            })
        }
        ///Creates a new call builder for the [`withdrawableRestakedExecutionLayerGwei`] function.
        pub fn withdrawableRestakedExecutionLayerGwei(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, withdrawableRestakedExecutionLayerGweiCall, N>
        {
            self.call_builder(&withdrawableRestakedExecutionLayerGweiCall)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        EigenPodInstance<P, N>
    {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<&P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`CheckpointCreated`] event.
        pub fn CheckpointCreated_filter(&self) -> alloy_contract::Event<&P, CheckpointCreated, N> {
            self.event_filter::<CheckpointCreated>()
        }
        ///Creates a new event filter for the [`CheckpointFinalized`] event.
        pub fn CheckpointFinalized_filter(
            &self,
        ) -> alloy_contract::Event<&P, CheckpointFinalized, N> {
            self.event_filter::<CheckpointFinalized>()
        }
        ///Creates a new event filter for the [`ConsolidationRequested`] event.
        pub fn ConsolidationRequested_filter(
            &self,
        ) -> alloy_contract::Event<&P, ConsolidationRequested, N> {
            self.event_filter::<ConsolidationRequested>()
        }
        ///Creates a new event filter for the [`EigenPodStaked`] event.
        pub fn EigenPodStaked_filter(&self) -> alloy_contract::Event<&P, EigenPodStaked, N> {
            self.event_filter::<EigenPodStaked>()
        }
        ///Creates a new event filter for the [`ExitRequested`] event.
        pub fn ExitRequested_filter(&self) -> alloy_contract::Event<&P, ExitRequested, N> {
            self.event_filter::<ExitRequested>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`NonBeaconChainETHReceived`] event.
        pub fn NonBeaconChainETHReceived_filter(
            &self,
        ) -> alloy_contract::Event<&P, NonBeaconChainETHReceived, N> {
            self.event_filter::<NonBeaconChainETHReceived>()
        }
        ///Creates a new event filter for the [`ProofSubmitterUpdated`] event.
        pub fn ProofSubmitterUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, ProofSubmitterUpdated, N> {
            self.event_filter::<ProofSubmitterUpdated>()
        }
        ///Creates a new event filter for the [`RestakedBeaconChainETHWithdrawn`] event.
        pub fn RestakedBeaconChainETHWithdrawn_filter(
            &self,
        ) -> alloy_contract::Event<&P, RestakedBeaconChainETHWithdrawn, N> {
            self.event_filter::<RestakedBeaconChainETHWithdrawn>()
        }
        ///Creates a new event filter for the [`SwitchToCompoundingRequested`] event.
        pub fn SwitchToCompoundingRequested_filter(
            &self,
        ) -> alloy_contract::Event<&P, SwitchToCompoundingRequested, N> {
            self.event_filter::<SwitchToCompoundingRequested>()
        }
        ///Creates a new event filter for the [`ValidatorBalanceUpdated`] event.
        pub fn ValidatorBalanceUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, ValidatorBalanceUpdated, N> {
            self.event_filter::<ValidatorBalanceUpdated>()
        }
        ///Creates a new event filter for the [`ValidatorCheckpointed`] event.
        pub fn ValidatorCheckpointed_filter(
            &self,
        ) -> alloy_contract::Event<&P, ValidatorCheckpointed, N> {
            self.event_filter::<ValidatorCheckpointed>()
        }
        ///Creates a new event filter for the [`ValidatorRestaked`] event.
        pub fn ValidatorRestaked_filter(&self) -> alloy_contract::Event<&P, ValidatorRestaked, N> {
            self.event_filter::<ValidatorRestaked>()
        }
        ///Creates a new event filter for the [`ValidatorWithdrawn`] event.
        pub fn ValidatorWithdrawn_filter(
            &self,
        ) -> alloy_contract::Event<&P, ValidatorWithdrawn, N> {
            self.event_filter::<ValidatorWithdrawn>()
        }
        ///Creates a new event filter for the [`WithdrawalRequested`] event.
        pub fn WithdrawalRequested_filter(
            &self,
        ) -> alloy_contract::Event<&P, WithdrawalRequested, N> {
            self.event_filter::<WithdrawalRequested>()
        }
    }
}
