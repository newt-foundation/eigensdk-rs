///Module containing a contract's types and functions.
/**

```solidity
library ICrossChainRegistryTypes {
    struct OperatorSetConfig { address owner; uint32 maxStalenessPeriod; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ICrossChainRegistryTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct OperatorSetConfig { address owner; uint32 maxStalenessPeriod; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSetConfig {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub maxStalenessPeriod: u32,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
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
        impl ::core::convert::From<OperatorSetConfig> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSetConfig) -> Self {
                (value.owner, value.maxStalenessPeriod)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSetConfig {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    owner: tuple.0,
                    maxStalenessPeriod: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorSetConfig {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorSetConfig {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.maxStalenessPeriod,
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
        impl alloy_sol_types::SolType for OperatorSetConfig {
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
        impl alloy_sol_types::SolStruct for OperatorSetConfig {
            const NAME: &'static str = "OperatorSetConfig";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorSetConfig(address owner,uint32 maxStalenessPeriod)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.owner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.maxStalenessPeriod,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorSetConfig {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.owner,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.maxStalenessPeriod,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.owner,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.maxStalenessPeriod,
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
    /**Creates a new wrapper around an on-chain [`ICrossChainRegistryTypes`](self) contract instance.

    See the [wrapper's documentation](`ICrossChainRegistryTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ICrossChainRegistryTypesInstance<P, N> {
        ICrossChainRegistryTypesInstance::<P, N>::new(address, provider)
    }
    /**A [`ICrossChainRegistryTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ICrossChainRegistryTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ICrossChainRegistryTypesInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ICrossChainRegistryTypesInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ICrossChainRegistryTypesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        ICrossChainRegistryTypesInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`ICrossChainRegistryTypes`](self) contract instance.

        See the [wrapper's documentation](`ICrossChainRegistryTypesInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> ICrossChainRegistryTypesInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ICrossChainRegistryTypesInstance<P, N> {
            ICrossChainRegistryTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        ICrossChainRegistryTypesInstance<P, N>
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
        ICrossChainRegistryTypesInstance<P, N>
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
library IECDSACertificateVerifierTypes {
    struct ECDSACertificate { uint32 referenceTimestamp; bytes32 messageHash; bytes sig; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IECDSACertificateVerifierTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct ECDSACertificate { uint32 referenceTimestamp; bytes32 messageHash; bytes sig; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSACertificate {
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub messageHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub sig: alloy::sol_types::private::Bytes,
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
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
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
        impl ::core::convert::From<ECDSACertificate> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSACertificate) -> Self {
                (value.referenceTimestamp, value.messageHash, value.sig)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSACertificate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    referenceTimestamp: tuple.0,
                    messageHash: tuple.1,
                    sig: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ECDSACertificate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ECDSACertificate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTimestamp),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.messageHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.sig,
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
        impl alloy_sol_types::SolType for ECDSACertificate {
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
        impl alloy_sol_types::SolStruct for ECDSACertificate {
            const NAME: &'static str = "ECDSACertificate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ECDSACertificate(uint32 referenceTimestamp,bytes32 messageHash,bytes sig)",
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
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.referenceTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.messageHash)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sig,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ECDSACertificate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.referenceTimestamp,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.messageHash,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sig,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.referenceTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.messageHash,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sig,
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
    /**Creates a new wrapper around an on-chain [`IECDSACertificateVerifierTypes`](self) contract instance.

    See the [wrapper's documentation](`IECDSACertificateVerifierTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IECDSACertificateVerifierTypesInstance<P, N> {
        IECDSACertificateVerifierTypesInstance::<P, N>::new(address, provider)
    }
    /**A [`IECDSACertificateVerifierTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IECDSACertificateVerifierTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IECDSACertificateVerifierTypesInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IECDSACertificateVerifierTypesInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IECDSACertificateVerifierTypesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IECDSACertificateVerifierTypesInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`IECDSACertificateVerifierTypes`](self) contract instance.

        See the [wrapper's documentation](`IECDSACertificateVerifierTypesInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> IECDSACertificateVerifierTypesInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IECDSACertificateVerifierTypesInstance<P, N> {
            IECDSACertificateVerifierTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IECDSACertificateVerifierTypesInstance<P, N>
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
        IECDSACertificateVerifierTypesInstance<P, N>
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
library IOperatorTableCalculatorTypes {
    struct ECDSAOperatorInfo { address pubkey; uint256[] weights; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IOperatorTableCalculatorTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct ECDSAOperatorInfo { address pubkey; uint256[] weights; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSAOperatorInfo {
        #[allow(missing_docs)]
        pub pubkey: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub weights:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
        impl ::core::convert::From<ECDSAOperatorInfo> for UnderlyingRustTuple<'_> {
            fn from(value: ECDSAOperatorInfo) -> Self {
                (value.pubkey, value.weights)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSAOperatorInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    pubkey: tuple.0,
                    weights: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ECDSAOperatorInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ECDSAOperatorInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.pubkey,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.weights),
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
        impl alloy_sol_types::SolType for ECDSAOperatorInfo {
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
        impl alloy_sol_types::SolStruct for ECDSAOperatorInfo {
            const NAME: &'static str = "ECDSAOperatorInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ECDSAOperatorInfo(address pubkey,uint256[] weights)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.pubkey,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.weights)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ECDSAOperatorInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pubkey,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.weights,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pubkey,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.weights,
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
    /**Creates a new wrapper around an on-chain [`IOperatorTableCalculatorTypes`](self) contract instance.

    See the [wrapper's documentation](`IOperatorTableCalculatorTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IOperatorTableCalculatorTypesInstance<P, N> {
        IOperatorTableCalculatorTypesInstance::<P, N>::new(address, provider)
    }
    /**A [`IOperatorTableCalculatorTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IOperatorTableCalculatorTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IOperatorTableCalculatorTypesInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IOperatorTableCalculatorTypesInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IOperatorTableCalculatorTypesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IOperatorTableCalculatorTypesInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`IOperatorTableCalculatorTypes`](self) contract instance.

        See the [wrapper's documentation](`IOperatorTableCalculatorTypesInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> IOperatorTableCalculatorTypesInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IOperatorTableCalculatorTypesInstance<P, N> {
            IOperatorTableCalculatorTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IOperatorTableCalculatorTypesInstance<P, N>
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
        IOperatorTableCalculatorTypesInstance<P, N>
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
library ICrossChainRegistryTypes {
    struct OperatorSetConfig {
        address owner;
        uint32 maxStalenessPeriod;
    }
}

library IECDSACertificateVerifierTypes {
    struct ECDSACertificate {
        uint32 referenceTimestamp;
        bytes32 messageHash;
        bytes sig;
    }
}

library IOperatorTableCalculatorTypes {
    struct ECDSAOperatorInfo {
        address pubkey;
        uint256[] weights;
    }
}

interface ECDSACertificateVerifier {
    struct OperatorSet {
        address avs;
        uint32 id;
    }

    error ArrayLengthMismatch();
    error CertificateStale();
    error IndexOutOfBounds();
    error InvalidShortString();
    error InvalidSignature();
    error InvalidSignatureLength();
    error OnlyTableUpdater();
    error OperatorCountZero();
    error ReferenceTimestampDoesNotExist();
    error RootDisabled();
    error SignatureExpired();
    error SignersNotOrdered();
    error StringTooLong(string str);
    error TableUpdateStale();
    error VerificationFailed();

    event Initialized(uint8 version);
    event MaxStalenessPeriodUpdated(OperatorSet operatorSet, uint32 maxStalenessPeriod);
    event OperatorSetOwnerUpdated(OperatorSet operatorSet, address owner);
    event TableUpdated(OperatorSet operatorSet, uint32 referenceTimestamp, IOperatorTableCalculatorTypes.ECDSAOperatorInfo[] operatorInfos);

    constructor(address _operatorTableUpdater, string _version);

    function calculateCertificateDigest(uint32 referenceTimestamp, bytes32 messageHash) external view returns (bytes32);
    function calculateCertificateDigestBytes(uint32 referenceTimestamp, bytes32 messageHash) external view returns (bytes memory);
    function domainSeparator() external view returns (bytes32);
    function getOperatorCount(OperatorSet memory operatorSet, uint32 referenceTimestamp) external view returns (uint256);
    function getOperatorInfo(OperatorSet memory operatorSet, uint32 referenceTimestamp, uint256 operatorIndex) external view returns (IOperatorTableCalculatorTypes.ECDSAOperatorInfo memory);
    function getOperatorInfos(OperatorSet memory operatorSet, uint32 referenceTimestamp) external view returns (IOperatorTableCalculatorTypes.ECDSAOperatorInfo[] memory);
    function getOperatorSetOwner(OperatorSet memory operatorSet) external view returns (address);
    function getTotalStakeWeights(OperatorSet memory operatorSet, uint32 referenceTimestamp) external view returns (uint256[] memory);
    function isReferenceTimestampSet(OperatorSet memory operatorSet, uint32 referenceTimestamp) external view returns (bool);
    function latestReferenceTimestamp(OperatorSet memory operatorSet) external view returns (uint32);
    function maxOperatorTableStaleness(OperatorSet memory operatorSet) external view returns (uint32);
    function operatorTableUpdater() external view returns (address);
    function updateOperatorTable(OperatorSet memory operatorSet, uint32 referenceTimestamp, IOperatorTableCalculatorTypes.ECDSAOperatorInfo[] memory operatorInfos, ICrossChainRegistryTypes.OperatorSetConfig memory operatorSetConfig) external;
    function verifyCertificate(OperatorSet memory operatorSet, IECDSACertificateVerifierTypes.ECDSACertificate memory cert) external view returns (uint256[] memory, address[] memory);
    function verifyCertificateNominal(OperatorSet memory operatorSet, IECDSACertificateVerifierTypes.ECDSACertificate memory cert, uint256[] memory totalStakeNominalThresholds) external view returns (bool, address[] memory);
    function verifyCertificateProportion(OperatorSet memory operatorSet, IECDSACertificateVerifierTypes.ECDSACertificate memory cert, uint16[] memory totalStakeProportionThresholds) external view returns (bool, address[] memory);
    function version() external view returns (string memory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_operatorTableUpdater",
        "type": "address",
        "internalType": "contract IOperatorTableUpdater"
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
    "type": "function",
    "name": "calculateCertificateDigest",
    "inputs": [
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "messageHash",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "calculateCertificateDigestBytes",
    "inputs": [
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "messageHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "domainSeparator",
    "inputs": [],
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
    "name": "getOperatorCount",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
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
    "name": "getOperatorInfo",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "operatorIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IOperatorTableCalculatorTypes.ECDSAOperatorInfo",
        "components": [
          {
            "name": "pubkey",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "weights",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorInfos",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IOperatorTableCalculatorTypes.ECDSAOperatorInfo[]",
        "components": [
          {
            "name": "pubkey",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "weights",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorSetOwner",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
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
    "name": "getTotalStakeWeights",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isReferenceTimestampSet",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "latestReferenceTimestamp",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "maxOperatorTableStaleness",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorTableUpdater",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IOperatorTableUpdater"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "updateOperatorTable",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "operatorInfos",
        "type": "tuple[]",
        "internalType": "struct IOperatorTableCalculatorTypes.ECDSAOperatorInfo[]",
        "components": [
          {
            "name": "pubkey",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "weights",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      },
      {
        "name": "operatorSetConfig",
        "type": "tuple",
        "internalType": "struct ICrossChainRegistryTypes.OperatorSetConfig",
        "components": [
          {
            "name": "owner",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "maxStalenessPeriod",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifyCertificate",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "cert",
        "type": "tuple",
        "internalType": "struct IECDSACertificateVerifierTypes.ECDSACertificate",
        "components": [
          {
            "name": "referenceTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "messageHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "sig",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "verifyCertificateNominal",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "cert",
        "type": "tuple",
        "internalType": "struct IECDSACertificateVerifierTypes.ECDSACertificate",
        "components": [
          {
            "name": "referenceTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "messageHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "sig",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "totalStakeNominalThresholds",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "verifyCertificateProportion",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "cert",
        "type": "tuple",
        "internalType": "struct IECDSACertificateVerifierTypes.ECDSACertificate",
        "components": [
          {
            "name": "referenceTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "messageHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "sig",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "totalStakeProportionThresholds",
        "type": "uint16[]",
        "internalType": "uint16[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
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
    "name": "MaxStalenessPeriodUpdated",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "maxStalenessPeriod",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSetOwnerUpdated",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "owner",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TableUpdated",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "operatorInfos",
        "type": "tuple[]",
        "indexed": false,
        "internalType": "struct IOperatorTableCalculatorTypes.ECDSAOperatorInfo[]",
        "components": [
          {
            "name": "pubkey",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "weights",
            "type": "uint256[]",
            "internalType": "uint256[]"
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "ArrayLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CertificateStale",
    "inputs": []
  },
  {
    "type": "error",
    "name": "IndexOutOfBounds",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidShortString",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignatureLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyTableUpdater",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorCountZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ReferenceTimestampDoesNotExist",
    "inputs": []
  },
  {
    "type": "error",
    "name": "RootDisabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SignatureExpired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SignersNotOrdered",
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
    "name": "TableUpdateStale",
    "inputs": []
  },
  {
    "type": "error",
    "name": "VerificationFailed",
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
pub mod ECDSACertificateVerifier {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c060405234801561000f575f5ffd5b5060405161231938038061231983398101604081905261002e9161016d565b6001600160a01b03821660805280806100468161005b565b60a0525061005490506100a1565b5050610297565b5f5f829050601f8151111561008e578260405163305a27a960e01b8152600401610085919061023c565b60405180910390fd5b805161009982610271565b179392505050565b5f54610100900460ff16156101085760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608401610085565b5f5460ff90811614610157575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b634e487b7160e01b5f52604160045260245ffd5b5f5f6040838503121561017e575f5ffd5b82516001600160a01b0381168114610194575f5ffd5b60208401519092506001600160401b038111156101af575f5ffd5b8301601f810185136101bf575f5ffd5b80516001600160401b038111156101d8576101d8610159565b604051601f8201601f19908116603f011681016001600160401b038111828210171561020657610206610159565b60405281815282820160200187101561021d575f5ffd5b8160208401602083015e5f602083830101528093505050509250929050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80516020808301519190811015610291575f198160200360031b1b821691505b50919050565b60805160a05161204c6102cd5f395f8181610342015261104601525f81816101ad015281816103760152610ee4015261204c5ff3fe608060405234801561000f575f5ffd5b5060043610610106575f3560e01c80637c85ac4c1161009e578063be86e0b21161006e578063be86e0b21461026e578063c0da24201461028f578063cd83a72b146102a2578063e49613fc146102c5578063f698da25146102e5575f5ffd5b80637c85ac4c146101fa5780637d1d1f5b1461021a57806380c7d3f31461023a578063848189201461025b575f5ffd5b80635ddb9b5b116100d95780635ddb9b5b1461016d5780636141879e1461019557806368d6e081146101a8578063702ca531146101e7575f5ffd5b8063184674341461010a57806323c2a3cb1461013057806354fd4d501461014357806356d482f514610158575b5f5ffd5b61011d61011836600461161c565b6102ed565b6040519081526020015b60405180910390f35b61011d61013e3660046116fc565b610308565b61014b61033b565b604051610127919061175c565b61016b6101663660046117cb565b61036b565b005b61018061017b36600461183d565b610582565b60405163ffffffff9091168152602001610127565b6101806101a336600461183d565b6105a8565b6101cf7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610127565b61014b6101f536600461161c565b6105ce565b61020d6102083660046116fc565b610665565b60405161012791906118ac565b61022d61022836600461190f565b6107c3565b6040516101279190611964565b61024d610248366004611986565b610986565b604051610127929190611a0a565b6101cf61026936600461183d565b6109a4565b61028161027c366004611a37565b6109cd565b604051610127929190611b1d565b61028161029d366004611b3f565b610a6d565b6102b56102b03660046116fc565b610b6f565b6040519015158152602001610127565b6102d86102d3366004611bb5565b610ba5565b6040516101279190611bf0565b61011d610c9e565b5f6102f883836105ce565b8051906020012090505b92915050565b5f5f61031384610d5e565b5f90815260046020908152604080832063ffffffff8716845290915290205491505092915050565b60606103667f0000000000000000000000000000000000000000000000000000000000000000610dc1565b905090565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146103b45760405163030c1b6b60e11b815260040160405180910390fd5b5f6103cc6103c73688900388018861183d565b610d5e565b5f8181526003602052604090205490915063ffffffff9081169086161161040657604051632f20889f60e01b815260040160405180910390fd5b5f81815260046020908152604080832063ffffffff8916845290915281208490555b8381101561048e5784848281811061044257610442611c02565b90506020028101906104549190611c16565b5f83815260056020908152604080832063ffffffff8b168452825280832085845290915290206104848282611c5f565b5050600101610428565b505f818152600360209081526040909120805463ffffffff191663ffffffff88161790556104be90830183611d62565b5f8281526001602090815260409182902080546001600160a01b0319166001600160a01b0394909416939093179092556104fd91908401908401611d7d565b5f828152600260209081526040808320805463ffffffff191663ffffffff958616179055600682528083209389168352929052819020805460ff19166001179055517f4f588da9ec57976194a79b5594f8f8782923d93013df2b9ed12fe125805011ef90610572908890889088908890611d96565b60405180910390a1505050505050565b5f5f61058d83610d5e565b5f9081526003602052604090205463ffffffff169392505050565b5f5f6105b383610d5e565b5f9081526002602052604090205463ffffffff169392505050565b604080517fda346acb3ce99e7c5132bf8cafb159ad8085970ebfdba78007ef0fe163063d1460208083019190915263ffffffff85168284015260608083018590528351808403820181526080909301909352815191012061062d610c9e565b60405161190160f01b602082015260228101919091526042810182905260620160405160208183030381529060405291505092915050565b60605f61067184610d5e565b5f81815260046020908152604080832063ffffffff8089168552925282205492935082166001600160401b038111156106ac576106ac611644565b6040519080825280602002602001820160405280156106f157816020015b604080518082019091525f8152606060208201528152602001906001900390816106ca5790505b5090505f5b8263ffffffff168110156107b9575f84815260056020908152604080832063ffffffff8a16845282528083208484528252918290208251808401845281546001600160a01b031681526001820180548551818602810186019096528086529194929385810193929083018282801561078b57602002820191905f5260205f20905b815481526020019060010190808311610777575b5050505050815250508282815181106107a6576107a6611c02565b60209081029190910101526001016106f6565b5095945050505050565b60605f6107d86103c73686900386018661183d565b5f81815260046020908152604080832063ffffffff881684529091529020549091508061081857604051631029081560e21b815260040160405180910390fd5b5f82815260056020908152604080832063ffffffff88168452825280832083805290915281206001015490816001600160401b0381111561085b5761085b611644565b604051908082528060200260200182016040528015610884578160200160208202803683370190505b5090505f5b8381101561097b575f85815260056020908152604080832063ffffffff8b16845282528083208484528252808320600101805482518185028101850190935280835291929091908301828280156108fd57602002820191905f5260205f20905b8154815260200190600101908083116108e9575b509394505f93505050505b81518110801561091757508481105b156109715781818151811061092e5761092e611c02565b602002602001015184828151811061094857610948611c02565b6020026020010181815161095c9190611ec9565b9052508061096981611edc565b915050610908565b5050600101610889565b509695505050505050565b6060805f5f6109958686610dfe565b909450925050505b9250929050565b5f5f6109af83610d5e565b5f908152600160205260409020546001600160a01b03169392505050565b5f60605f5f6109dc8787610dfe565b915091508451825114610a025760405163512509d360e11b815260040160405180910390fd5b5f5b8251811015610a5c57858181518110610a1f57610a1f611c02565b6020026020010151838281518110610a3957610a39611c02565b60200260200101511015610a5457505f93509150610a659050565b600101610a04565b50600193509150505b935093915050565b5f60605f5f610a7c8888610dfe565b90925090505f610a938961022860208b018b611d7d565b83519091508614610ab75760405163512509d360e11b815260040160405180910390fd5b5f5b8351811015610b5b575f612710898984818110610ad857610ad8611c02565b9050602002016020810190610aed9190611ef4565b61ffff16848481518110610b0357610b03611c02565b6020026020010151610b159190611c48565b610b1f9190611f29565b905080858381518110610b3457610b34611c02565b60200260200101511015610b52575f84965096505050505050610b66565b50600101610ab9565b506001945090925050505b94509492505050565b5f5f610b7a84610d5e565b5f90815260066020908152604080832063ffffffff8716845290915290205460ff1691505092915050565b604080518082019091525f8152606060208201525f610bc385610d5e565b5f81815260046020908152604080832063ffffffff891684529091529020549091508310610c0457604051634e23d03560e01b815260040160405180910390fd5b5f81815260056020908152604080832063ffffffff8816845282528083208684528252918290208251808401845281546001600160a01b0316815260018201805485518186028101860190965280865291949293858101939290830182828015610c8b57602002820191905f5260205f20905b815481526020019060010190808311610c77575b5050505050815250509150509392505050565b60408051808201909152600a81526922b4b3b2b72630bcb2b960b11b6020909101525f7f91ab3d17e3a50a9d89e63fd30b92be7f5336b03b287bb946787a83a9d62a27667f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea610d0b61103e565b8051602091820120604051610d43949392309101938452602084019290925260408301526001600160a01b0316606082015260800190565b60405160208183030381529060405280519060200120905090565b5f815f0151826020015163ffffffff16604051602001610da992919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b60405160208183030381529060405261030290611f3c565b60605f610dcd836110b3565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b6060805f610e146103c73687900387018761183d565b5f8181526002602052604090205490915063ffffffff161580610e6a57505f8181526002602090815260409091205463ffffffff1690610e5690860186611d7d565b610e609190611f5f565b63ffffffff164211155b610e875760405163640fcd6b60e11b815260040160405180910390fd5b5f81815260066020908152604082209190610ea490870187611d7d565b63ffffffff16815260208101919091526040015f205460ff16610eda57604051630cad17b760e31b815260040160405180910390fd5b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000166364e1df84610f166020870187611d7d565b6040516001600160e01b031960e084901b16815263ffffffff919091166004820152602401602060405180830381865afa158015610f56573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f7a9190611f7b565b610f9757604051631b14174b60e01b815260040160405180910390fd5b5f610fb2610fa86020870187611d7d565b86602001356102ed565b90505f610fff82610fc66040890189611f9a565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152506110da92505050565b90505f6110138861022860208a018a611d7d565b5190505f61102f8561102860208b018b611d7d565b85856112e1565b99929850919650505050505050565b60605f61106a7f0000000000000000000000000000000000000000000000000000000000000000610dc1565b9050805f8151811061107e5761107e611c02565b016020908101516040516001600160f81b03199091169181019190915260210160405160208183030381529060405291505090565b5f60ff8216601f81111561030257604051632cd44ac360e21b815260040160405180910390fd5b60605f82511180156110f75750604182516110f59190611fdc565b155b61111457604051634be6321b60e01b815260040160405180910390fd5b5f604183516111239190611f29565b9050806001600160401b0381111561113d5761113d611644565b604051908082528060200260200182016040528015611166578160200160208202803683370190505b5091505f5b818110156112d957604080516041808252608082019092525f916020820181803683370190505090505f5b60418110156112015785816111ac856041611c48565b6111b69190611ec9565b815181106111c6576111c6611c02565b602001015160f81c60f81b8282815181106111e3576111e3611c02565b60200101906001600160f81b03191690815f1a905350600101611196565b505f5f61120e888461150b565b90925090505f81600481111561122657611226611fef565b1461124457604051638baa579f60e01b815260040160405180910390fd5b831580611285575085611258600186612003565b8151811061126857611268611c02565b60200260200101516001600160a01b0316826001600160a01b0316115b6112a257604051630b550c5760e41b815260040160405180910390fd5b818685815181106112b5576112b5611c02565b6001600160a01b03929092166020928302919091019091015250505060010161116b565b505092915050565b5f84815260046020908152604080832063ffffffff87168452909152902054606090826001600160401b0381111561131b5761131b611644565b604051908082528060200260200182016040528015611344578160200160208202803683370190505b5091505f5b8451811015611501575f85828151811061136557611365611c02565b602002602001015190505f5f905061139860405180604001604052805f6001600160a01b03168152602001606081525090565b5f5b85811015611461575f8b815260056020908152604080832063ffffffff8e16845282528083208484528252918290208251808401845281546001600160a01b031681526001820180548551818602810186019096528086529194929385810193929083018282801561142957602002820191905f5260205f20905b815481526020019060010190808311611415575b5050505050815250509150836001600160a01b0316825f01516001600160a01b0316036114595760019250611461565b60010161139a565b50816114805760405163439cc0cd60e01b815260040160405180910390fd5b60208101515f5b81518110801561149657508881105b156114f0578181815181106114ad576114ad611c02565b60200260200101518882815181106114c7576114c7611c02565b602002602001018181516114db9190611ec9565b905250806114e881611edc565b915050611487565b505060019093019250611349915050565b5050949350505050565b5f5f825160410361153f576020830151604084015160608501515f1a6115338782858561154a565b9450945050505061099d565b505f9050600261099d565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083111561157f57505f90506003610b66565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa1580156115d0573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b0381166115f8575f60019250925050610b66565b965f9650945050505050565b803563ffffffff81168114611617575f5ffd5b919050565b5f5f6040838503121561162d575f5ffd5b61163683611604565b946020939093013593505050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b038111828210171561168057611680611644565b604052919050565b6001600160a01b038116811461169c575f5ffd5b50565b5f604082840312156116af575f5ffd5b604080519081016001600160401b03811182821017156116d1576116d1611644565b60405290508082356116e281611688565b81526116f060208401611604565b60208201525092915050565b5f5f6060838503121561170d575f5ffd5b611717848461169f565b915061172560408401611604565b90509250929050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f61176e602083018461172e565b9392505050565b5f60408284031215611785575f5ffd5b50919050565b5f5f83601f84011261179b575f5ffd5b5081356001600160401b038111156117b1575f5ffd5b6020830191508360208260051b850101111561099d575f5ffd5b5f5f5f5f5f60c086880312156117df575f5ffd5b6117e98787611775565b94506117f760408701611604565b935060608601356001600160401b03811115611811575f5ffd5b61181d8882890161178b565b909450925061183190508760808801611775565b90509295509295909350565b5f6040828403121561184d575f5ffd5b61176e838361169f565b80516001600160a01b03168252602080820151604082850181905281519085018190525f929190910190829060608601905b808310156107b95783518252602082019150602084019350600183019250611889565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561190357603f198786030184526118ee858351611857565b945060209384019391909101906001016118d2565b50929695505050505050565b5f5f60608385031215611920575f5ffd5b6117178484611775565b5f8151808452602084019350602083015f5b8281101561195a57815186526020958601959091019060010161193c565b5093949350505050565b602081525f61176e602083018461192a565b5f60608284031215611785575f5ffd5b5f5f60608385031215611997575f5ffd5b6119a18484611775565b915060408301356001600160401b038111156119bb575f5ffd5b6119c785828601611976565b9150509250929050565b5f8151808452602084019350602083015f5b8281101561195a5781516001600160a01b03168652602095860195909101906001016119e3565b604081525f611a1c604083018561192a565b8281036020840152611a2e81856119d1565b95945050505050565b5f5f5f60808486031215611a49575f5ffd5b611a538585611775565b925060408401356001600160401b03811115611a6d575f5ffd5b611a7986828701611976565b92505060608401356001600160401b03811115611a94575f5ffd5b8401601f81018613611aa4575f5ffd5b80356001600160401b03811115611abd57611abd611644565b8060051b611acd60208201611658565b91825260208184018101929081019089841115611ae8575f5ffd5b6020850194505b83851015611b0e57843580835260209586019590935090910190611aef565b80955050505050509250925092565b8215158152604060208201525f611b3760408301846119d1565b949350505050565b5f5f5f5f60808587031215611b52575f5ffd5b611b5c8686611775565b935060408501356001600160401b03811115611b76575f5ffd5b611b8287828801611976565b93505060608501356001600160401b03811115611b9d575f5ffd5b611ba98782880161178b565b95989497509550505050565b5f5f5f60808486031215611bc7575f5ffd5b611bd1858561169f565b9250611bdf60408501611604565b929592945050506060919091013590565b602081525f61176e6020830184611857565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e19833603018112611c2a575f5ffd5b9190910192915050565b634e487b7160e01b5f52601160045260245ffd5b808202811582820484141761030257610302611c34565b8135611c6a81611688565b81546001600160a01b0319166001600160a01b0391909116178155602082013536839003601e19018112611c9c575f5ffd5b820180356001600160401b03811115611cb3575f5ffd5b6020820191508060051b3603821315611cca575f5ffd5b600183016001600160401b03821115611ce557611ce5611644565b68010000000000000000821115611cfe57611cfe611644565b805482825580831015611d33575f828152602090208381019082015b80821015611d30575f8255600182019150611d1a565b50505b505f90815260208120905b82811015611d5a57833582820155602090930192600101611d3e565b505050505050565b5f60208284031215611d72575f5ffd5b813561176e81611688565b5f60208284031215611d8d575f5ffd5b61176e82611604565b5f608082018635611da681611688565b6001600160a01b0316835263ffffffff611dc260208901611604565b16602084015263ffffffff861660408401526080606084015283905260a0600584901b83018101908301855f603e1936839003015b87821015611eba57868503609f190184528235818112611e15575f5ffd5b89018035611e2281611688565b6001600160a01b03168652602081013536829003601e19018112611e44575f5ffd5b016020810190356001600160401b03811115611e5e575f5ffd5b8060051b803603831315611e70575f5ffd5b60406020890181905288018290526001600160fb1b03821115611e91575f5ffd5b808360608a01376060818901019750505050602083019250602084019350600182019150611df7565b50929998505050505050505050565b8082018082111561030257610302611c34565b5f60018201611eed57611eed611c34565b5060010190565b5f60208284031215611f04575f5ffd5b813561ffff8116811461176e575f5ffd5b634e487b7160e01b5f52601260045260245ffd5b5f82611f3757611f37611f15565b500490565b80516020808301519190811015611785575f1960209190910360031b1b16919050565b63ffffffff818116838216019081111561030257610302611c34565b5f60208284031215611f8b575f5ffd5b8151801515811461176e575f5ffd5b5f5f8335601e19843603018112611faf575f5ffd5b8301803591506001600160401b03821115611fc8575f5ffd5b60200191503681900382131561099d575f5ffd5b5f82611fea57611fea611f15565b500690565b634e487b7160e01b5f52602160045260245ffd5b8181038181111561030257610302611c3456fea26469706673582212208d0a878a042e9b91809f8767929e9c01d8a7b59a7c046bfd148b8ebb747253bb64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa#\x198\x03\x80a#\x19\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01mV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\x80R\x80\x80a\0F\x81a\0[V[`\xA0RPa\0T\x90Pa\0\xA1V[PPa\x02\x97V[__\x82\x90P`\x1F\x81Q\x11\x15a\0\x8EW\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\0\x85\x91\x90a\x02<V[`@Q\x80\x91\x03\x90\xFD[\x80Qa\0\x99\x82a\x02qV[\x17\x93\x92PPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x01\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01a\0\x85V[_T`\xFF\x90\x81\x16\x14a\x01WW_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__`@\x83\x85\x03\x12\x15a\x01~W__\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x94W__\xFD[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xAFW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x01\xBFW__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xD8Wa\x01\xD8a\x01YV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02\x06Wa\x02\x06a\x01YV[`@R\x81\x81R\x82\x82\x01` \x01\x87\x10\x15a\x02\x1DW__\xFD[\x81` \x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x02\x91W_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\x80Q`\xA0Qa La\x02\xCD_9_\x81\x81a\x03B\x01Ra\x10F\x01R_\x81\x81a\x01\xAD\x01R\x81\x81a\x03v\x01Ra\x0E\xE4\x01Ra L_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x06W_5`\xE0\x1C\x80c|\x85\xACL\x11a\0\x9EW\x80c\xBE\x86\xE0\xB2\x11a\0nW\x80c\xBE\x86\xE0\xB2\x14a\x02nW\x80c\xC0\xDA$ \x14a\x02\x8FW\x80c\xCD\x83\xA7+\x14a\x02\xA2W\x80c\xE4\x96\x13\xFC\x14a\x02\xC5W\x80c\xF6\x98\xDA%\x14a\x02\xE5W__\xFD[\x80c|\x85\xACL\x14a\x01\xFAW\x80c}\x1D\x1F[\x14a\x02\x1AW\x80c\x80\xC7\xD3\xF3\x14a\x02:W\x80c\x84\x81\x89 \x14a\x02[W__\xFD[\x80c]\xDB\x9B[\x11a\0\xD9W\x80c]\xDB\x9B[\x14a\x01mW\x80caA\x87\x9E\x14a\x01\x95W\x80ch\xD6\xE0\x81\x14a\x01\xA8W\x80cp,\xA51\x14a\x01\xE7W__\xFD[\x80c\x18Ft4\x14a\x01\nW\x80c#\xC2\xA3\xCB\x14a\x010W\x80cT\xFDMP\x14a\x01CW\x80cV\xD4\x82\xF5\x14a\x01XW[__\xFD[a\x01\x1Da\x01\x186`\x04a\x16\x1CV[a\x02\xEDV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x1Da\x01>6`\x04a\x16\xFCV[a\x03\x08V[a\x01Ka\x03;V[`@Qa\x01'\x91\x90a\x17\\V[a\x01ka\x01f6`\x04a\x17\xCBV[a\x03kV[\0[a\x01\x80a\x01{6`\x04a\x18=V[a\x05\x82V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01'V[a\x01\x80a\x01\xA36`\x04a\x18=V[a\x05\xA8V[a\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01'V[a\x01Ka\x01\xF56`\x04a\x16\x1CV[a\x05\xCEV[a\x02\ra\x02\x086`\x04a\x16\xFCV[a\x06eV[`@Qa\x01'\x91\x90a\x18\xACV[a\x02-a\x02(6`\x04a\x19\x0FV[a\x07\xC3V[`@Qa\x01'\x91\x90a\x19dV[a\x02Ma\x02H6`\x04a\x19\x86V[a\t\x86V[`@Qa\x01'\x92\x91\x90a\x1A\nV[a\x01\xCFa\x02i6`\x04a\x18=V[a\t\xA4V[a\x02\x81a\x02|6`\x04a\x1A7V[a\t\xCDV[`@Qa\x01'\x92\x91\x90a\x1B\x1DV[a\x02\x81a\x02\x9D6`\x04a\x1B?V[a\nmV[a\x02\xB5a\x02\xB06`\x04a\x16\xFCV[a\x0BoV[`@Q\x90\x15\x15\x81R` \x01a\x01'V[a\x02\xD8a\x02\xD36`\x04a\x1B\xB5V[a\x0B\xA5V[`@Qa\x01'\x91\x90a\x1B\xF0V[a\x01\x1Da\x0C\x9EV[_a\x02\xF8\x83\x83a\x05\xCEV[\x80Q\x90` \x01 \x90P[\x92\x91PPV[__a\x03\x13\x84a\r^V[_\x90\x81R`\x04` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 T\x91PP\x92\x91PPV[``a\x03f\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\xC1V[\x90P\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03\xB4W`@Qc\x03\x0C\x1Bk`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x03\xCCa\x03\xC76\x88\x90\x03\x88\x01\x88a\x18=V[a\r^V[_\x81\x81R`\x03` R`@\x90 T\x90\x91Pc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x86\x16\x11a\x04\x06W`@Qc/ \x88\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x04` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x90\x91R\x81 \x84\x90U[\x83\x81\x10\x15a\x04\x8EW\x84\x84\x82\x81\x81\x10a\x04BWa\x04Ba\x1C\x02V[\x90P` \x02\x81\x01\x90a\x04T\x91\x90a\x1C\x16V[_\x83\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x8B\x16\x84R\x82R\x80\x83 \x85\x84R\x90\x91R\x90 a\x04\x84\x82\x82a\x1C_V[PP`\x01\x01a\x04(V[P_\x81\x81R`\x03` \x90\x81R`@\x90\x91 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x88\x16\x17\x90Ua\x04\xBE\x90\x83\x01\x83a\x1DbV[_\x82\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x93\x90\x93\x17\x90\x92Ua\x04\xFD\x91\x90\x84\x01\x90\x84\x01a\x1D}V[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x95\x86\x16\x17\x90U`\x06\x82R\x80\x83 \x93\x89\x16\x83R\x92\x90R\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7FOX\x8D\xA9\xECW\x97a\x94\xA7\x9BU\x94\xF8\xF8x)#\xD90\x13\xDF+\x9E\xD1/\xE1%\x80P\x11\xEF\x90a\x05r\x90\x88\x90\x88\x90\x88\x90\x88\x90a\x1D\x96V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[__a\x05\x8D\x83a\r^V[_\x90\x81R`\x03` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x93\x92PPPV[__a\x05\xB3\x83a\r^V[_\x90\x81R`\x02` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x93\x92PPPV[`@\x80Q\x7F\xDA4j\xCB<\xE9\x9E|Q2\xBF\x8C\xAF\xB1Y\xAD\x80\x85\x97\x0E\xBF\xDB\xA7\x80\x07\xEF\x0F\xE1c\x06=\x14` \x80\x83\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x85\x16\x82\x84\x01R``\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x82\x01\x81R`\x80\x90\x93\x01\x90\x93R\x81Q\x91\x01 a\x06-a\x0C\x9EV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x82\x90R`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x92\x91PPV[``_a\x06q\x84a\r^V[_\x81\x81R`\x04` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x80\x89\x16\x85R\x92R\x82 T\x92\x93P\x82\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xACWa\x06\xACa\x16DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xF1W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\xCAW\x90P[P\x90P_[\x82c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x07\xB9W_\x84\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x8A\x16\x84R\x82R\x80\x83 \x84\x84R\x82R\x91\x82\x90 \x82Q\x80\x84\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01\x80T\x85Q\x81\x86\x02\x81\x01\x86\x01\x90\x96R\x80\x86R\x91\x94\x92\x93\x85\x81\x01\x93\x92\x90\x83\x01\x82\x82\x80\x15a\x07\x8BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x07wW[PPPPP\x81RPP\x82\x82\x81Q\x81\x10a\x07\xA6Wa\x07\xA6a\x1C\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x06\xF6V[P\x95\x94PPPPPV[``_a\x07\xD8a\x03\xC76\x86\x90\x03\x86\x01\x86a\x18=V[_\x81\x81R`\x04` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x88\x16\x84R\x90\x91R\x90 T\x90\x91P\x80a\x08\x18W`@Qc\x10)\x08\x15`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x88\x16\x84R\x82R\x80\x83 \x83\x80R\x90\x91R\x81 `\x01\x01T\x90\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08[Wa\x08[a\x16DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\x84W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\t{W_\x85\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x8B\x16\x84R\x82R\x80\x83 \x84\x84R\x82R\x80\x83 `\x01\x01\x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\x08\xFDW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xE9W[P\x93\x94P_\x93PPPP[\x81Q\x81\x10\x80\x15a\t\x17WP\x84\x81\x10[\x15a\tqW\x81\x81\x81Q\x81\x10a\t.Wa\t.a\x1C\x02V[` \x02` \x01\x01Q\x84\x82\x81Q\x81\x10a\tHWa\tHa\x1C\x02V[` \x02` \x01\x01\x81\x81Qa\t\\\x91\x90a\x1E\xC9V[\x90RP\x80a\ti\x81a\x1E\xDCV[\x91PPa\t\x08V[PP`\x01\x01a\x08\x89V[P\x96\x95PPPPPPV[``\x80__a\t\x95\x86\x86a\r\xFEV[\x90\x94P\x92PPP[\x92P\x92\x90PV[__a\t\xAF\x83a\r^V[_\x90\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x93\x92PPPV[_``__a\t\xDC\x87\x87a\r\xFEV[\x91P\x91P\x84Q\x82Q\x14a\n\x02W`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82Q\x81\x10\x15a\n\\W\x85\x81\x81Q\x81\x10a\n\x1FWa\n\x1Fa\x1C\x02V[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a\n9Wa\n9a\x1C\x02V[` \x02` \x01\x01Q\x10\x15a\nTWP_\x93P\x91Pa\ne\x90PV[`\x01\x01a\n\x04V[P`\x01\x93P\x91PP[\x93P\x93\x91PPV[_``__a\n|\x88\x88a\r\xFEV[\x90\x92P\x90P_a\n\x93\x89a\x02(` \x8B\x01\x8Ba\x1D}V[\x83Q\x90\x91P\x86\x14a\n\xB7W`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83Q\x81\x10\x15a\x0B[W_a'\x10\x89\x89\x84\x81\x81\x10a\n\xD8Wa\n\xD8a\x1C\x02V[\x90P` \x02\x01` \x81\x01\x90a\n\xED\x91\x90a\x1E\xF4V[a\xFF\xFF\x16\x84\x84\x81Q\x81\x10a\x0B\x03Wa\x0B\x03a\x1C\x02V[` \x02` \x01\x01Qa\x0B\x15\x91\x90a\x1CHV[a\x0B\x1F\x91\x90a\x1F)V[\x90P\x80\x85\x83\x81Q\x81\x10a\x0B4Wa\x0B4a\x1C\x02V[` \x02` \x01\x01Q\x10\x15a\x0BRW_\x84\x96P\x96PPPPPPa\x0BfV[P`\x01\x01a\n\xB9V[P`\x01\x94P\x90\x92PPP[\x94P\x94\x92PPPV[__a\x0Bz\x84a\r^V[_\x90\x81R`\x06` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16\x91PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01R_a\x0B\xC3\x85a\r^V[_\x81\x81R`\x04` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x90\x91R\x90 T\x90\x91P\x83\x10a\x0C\x04W`@QcN#\xD05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x88\x16\x84R\x82R\x80\x83 \x86\x84R\x82R\x91\x82\x90 \x82Q\x80\x84\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01\x80T\x85Q\x81\x86\x02\x81\x01\x86\x01\x90\x96R\x80\x86R\x91\x94\x92\x93\x85\x81\x01\x93\x92\x90\x83\x01\x82\x82\x80\x15a\x0C\x8BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0CwW[PPPPP\x81RPP\x91PP\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x90\x91\x01R_\x7F\x91\xAB=\x17\xE3\xA5\n\x9D\x89\xE6?\xD3\x0B\x92\xBE\x7FS6\xB0;({\xB9Fxz\x83\xA9\xD6*'f\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEAa\r\x0Ba\x10>V[\x80Q` \x91\x82\x01 `@Qa\rC\x94\x93\x920\x91\x01\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a\r\xA9\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x03\x02\x90a\x1F<V[``_a\r\xCD\x83a\x10\xB3V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[``\x80_a\x0E\x14a\x03\xC76\x87\x90\x03\x87\x01\x87a\x18=V[_\x81\x81R`\x02` R`@\x90 T\x90\x91Pc\xFF\xFF\xFF\xFF\x16\x15\x80a\x0EjWP_\x81\x81R`\x02` \x90\x81R`@\x90\x91 Tc\xFF\xFF\xFF\xFF\x16\x90a\x0EV\x90\x86\x01\x86a\x1D}V[a\x0E`\x91\x90a\x1F_V[c\xFF\xFF\xFF\xFF\x16B\x11\x15[a\x0E\x87W`@Qcd\x0F\xCDk`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x06` \x90\x81R`@\x82 \x91\x90a\x0E\xA4\x90\x87\x01\x87a\x1D}V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16a\x0E\xDAW`@Qc\x0C\xAD\x17\xB7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16cd\xE1\xDF\x84a\x0F\x16` \x87\x01\x87a\x1D}V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FVW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fz\x91\x90a\x1F{V[a\x0F\x97W`@Qc\x1B\x14\x17K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0F\xB2a\x0F\xA8` \x87\x01\x87a\x1D}V[\x86` \x015a\x02\xEDV[\x90P_a\x0F\xFF\x82a\x0F\xC6`@\x89\x01\x89a\x1F\x9AV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x10\xDA\x92PPPV[\x90P_a\x10\x13\x88a\x02(` \x8A\x01\x8Aa\x1D}V[Q\x90P_a\x10/\x85a\x10(` \x8B\x01\x8Ba\x1D}V[\x85\x85a\x12\xE1V[\x99\x92\x98P\x91\x96PPPPPPPV[``_a\x10j\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\xC1V[\x90P\x80_\x81Q\x81\x10a\x10~Wa\x10~a\x1C\x02V[\x01` \x90\x81\x01Q`@Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x91\x81\x01\x91\x90\x91R`!\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x90V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x03\x02W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x82Q\x11\x80\x15a\x10\xF7WP`A\x82Qa\x10\xF5\x91\x90a\x1F\xDCV[\x15[a\x11\x14W`@QcK\xE62\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`A\x83Qa\x11#\x91\x90a\x1F)V[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11=Wa\x11=a\x16DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11fW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P_[\x81\x81\x10\x15a\x12\xD9W`@\x80Q`A\x80\x82R`\x80\x82\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P_[`A\x81\x10\x15a\x12\x01W\x85\x81a\x11\xAC\x85`Aa\x1CHV[a\x11\xB6\x91\x90a\x1E\xC9V[\x81Q\x81\x10a\x11\xC6Wa\x11\xC6a\x1C\x02V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x11\xE3Wa\x11\xE3a\x1C\x02V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x11\x96V[P__a\x12\x0E\x88\x84a\x15\x0BV[\x90\x92P\x90P_\x81`\x04\x81\x11\x15a\x12&Wa\x12&a\x1F\xEFV[\x14a\x12DW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x15\x80a\x12\x85WP\x85a\x12X`\x01\x86a \x03V[\x81Q\x81\x10a\x12hWa\x12ha\x1C\x02V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x11[a\x12\xA2W`@Qc\x0BU\x0CW`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x86\x85\x81Q\x81\x10a\x12\xB5Wa\x12\xB5a\x1C\x02V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPPP`\x01\x01a\x11kV[PP\x92\x91PPV[_\x84\x81R`\x04` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 T``\x90\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\x1BWa\x13\x1Ba\x16DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P_[\x84Q\x81\x10\x15a\x15\x01W_\x85\x82\x81Q\x81\x10a\x13eWa\x13ea\x1C\x02V[` \x02` \x01\x01Q\x90P__\x90Pa\x13\x98`@Q\x80`@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81RP\x90V[_[\x85\x81\x10\x15a\x14aW_\x8B\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x8E\x16\x84R\x82R\x80\x83 \x84\x84R\x82R\x91\x82\x90 \x82Q\x80\x84\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01\x80T\x85Q\x81\x86\x02\x81\x01\x86\x01\x90\x96R\x80\x86R\x91\x94\x92\x93\x85\x81\x01\x93\x92\x90\x83\x01\x82\x82\x80\x15a\x14)W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x14\x15W[PPPPP\x81RPP\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x14YW`\x01\x92Pa\x14aV[`\x01\x01a\x13\x9AV[P\x81a\x14\x80W`@QcC\x9C\xC0\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q_[\x81Q\x81\x10\x80\x15a\x14\x96WP\x88\x81\x10[\x15a\x14\xF0W\x81\x81\x81Q\x81\x10a\x14\xADWa\x14\xADa\x1C\x02V[` \x02` \x01\x01Q\x88\x82\x81Q\x81\x10a\x14\xC7Wa\x14\xC7a\x1C\x02V[` \x02` \x01\x01\x81\x81Qa\x14\xDB\x91\x90a\x1E\xC9V[\x90RP\x80a\x14\xE8\x81a\x1E\xDCV[\x91PPa\x14\x87V[PP`\x01\x90\x93\x01\x92Pa\x13I\x91PPV[PP\x94\x93PPPPV[__\x82Q`A\x03a\x15?W` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa\x153\x87\x82\x85\x85a\x15JV[\x94P\x94PPPPa\t\x9DV[P_\x90P`\x02a\t\x9DV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x15\x7FWP_\x90P`\x03a\x0BfV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x15\xD0W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x15\xF8W_`\x01\x92P\x92PPa\x0BfV[\x96_\x96P\x94PPPPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x16\x17W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x16-W__\xFD[a\x166\x83a\x16\x04V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x16\x80Wa\x16\x80a\x16DV[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\x9CW__\xFD[PV[_`@\x82\x84\x03\x12\x15a\x16\xAFW__\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x16\xD1Wa\x16\xD1a\x16DV[`@R\x90P\x80\x825a\x16\xE2\x81a\x16\x88V[\x81Ra\x16\xF0` \x84\x01a\x16\x04V[` \x82\x01RP\x92\x91PPV[__``\x83\x85\x03\x12\x15a\x17\rW__\xFD[a\x17\x17\x84\x84a\x16\x9FV[\x91Pa\x17%`@\x84\x01a\x16\x04V[\x90P\x92P\x92\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x17n` \x83\x01\x84a\x17.V[\x93\x92PPPV[_`@\x82\x84\x03\x12\x15a\x17\x85W__\xFD[P\x91\x90PV[__\x83`\x1F\x84\x01\x12a\x17\x9BW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xB1W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\t\x9DW__\xFD[_____`\xC0\x86\x88\x03\x12\x15a\x17\xDFW__\xFD[a\x17\xE9\x87\x87a\x17uV[\x94Pa\x17\xF7`@\x87\x01a\x16\x04V[\x93P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x11W__\xFD[a\x18\x1D\x88\x82\x89\x01a\x17\x8BV[\x90\x94P\x92Pa\x181\x90P\x87`\x80\x88\x01a\x17uV[\x90P\x92\x95P\x92\x95\x90\x93PV[_`@\x82\x84\x03\x12\x15a\x18MW__\xFD[a\x17n\x83\x83a\x16\x9FV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x80\x82\x01Q`@\x82\x85\x01\x81\x90R\x81Q\x90\x85\x01\x81\x90R_\x92\x91\x90\x91\x01\x90\x82\x90``\x86\x01\x90[\x80\x83\x10\x15a\x07\xB9W\x83Q\x82R` \x82\x01\x91P` \x84\x01\x93P`\x01\x83\x01\x92Pa\x18\x89V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x19\x03W`?\x19\x87\x86\x03\x01\x84Ra\x18\xEE\x85\x83Qa\x18WV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x18\xD2V[P\x92\x96\x95PPPPPPV[__``\x83\x85\x03\x12\x15a\x19 W__\xFD[a\x17\x17\x84\x84a\x17uV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x19ZW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x19<V[P\x93\x94\x93PPPPV[` \x81R_a\x17n` \x83\x01\x84a\x19*V[_``\x82\x84\x03\x12\x15a\x17\x85W__\xFD[__``\x83\x85\x03\x12\x15a\x19\x97W__\xFD[a\x19\xA1\x84\x84a\x17uV[\x91P`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xBBW__\xFD[a\x19\xC7\x85\x82\x86\x01a\x19vV[\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x19ZW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x19\xE3V[`@\x81R_a\x1A\x1C`@\x83\x01\x85a\x19*V[\x82\x81\x03` \x84\x01Ra\x1A.\x81\x85a\x19\xD1V[\x95\x94PPPPPV[___`\x80\x84\x86\x03\x12\x15a\x1AIW__\xFD[a\x1AS\x85\x85a\x17uV[\x92P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1AmW__\xFD[a\x1Ay\x86\x82\x87\x01a\x19vV[\x92PP``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\x94W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x1A\xA4W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xBDWa\x1A\xBDa\x16DV[\x80`\x05\x1Ba\x1A\xCD` \x82\x01a\x16XV[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x89\x84\x11\x15a\x1A\xE8W__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15a\x1B\x0EW\x845\x80\x83R` \x95\x86\x01\x95\x90\x93P\x90\x91\x01\x90a\x1A\xEFV[\x80\x95PPPPPP\x92P\x92P\x92V[\x82\x15\x15\x81R`@` \x82\x01R_a\x1B7`@\x83\x01\x84a\x19\xD1V[\x94\x93PPPPV[____`\x80\x85\x87\x03\x12\x15a\x1BRW__\xFD[a\x1B\\\x86\x86a\x17uV[\x93P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1BvW__\xFD[a\x1B\x82\x87\x82\x88\x01a\x19vV[\x93PP``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\x9DW__\xFD[a\x1B\xA9\x87\x82\x88\x01a\x17\x8BV[\x95\x98\x94\x97P\x95PPPPV[___`\x80\x84\x86\x03\x12\x15a\x1B\xC7W__\xFD[a\x1B\xD1\x85\x85a\x16\x9FV[\x92Pa\x1B\xDF`@\x85\x01a\x16\x04V[\x92\x95\x92\x94PPP``\x91\x90\x91\x015\x90V[` \x81R_a\x17n` \x83\x01\x84a\x18WV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12a\x1C*W__\xFD[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x02Wa\x03\x02a\x1C4V[\x815a\x1Cj\x81a\x16\x88V[\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x81U` \x82\x0156\x83\x90\x03`\x1E\x19\x01\x81\x12a\x1C\x9CW__\xFD[\x82\x01\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xB3W__\xFD[` \x82\x01\x91P\x80`\x05\x1B6\x03\x82\x13\x15a\x1C\xCAW__\xFD[`\x01\x83\x01`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1C\xE5Wa\x1C\xE5a\x16DV[h\x01\0\0\0\0\0\0\0\0\x82\x11\x15a\x1C\xFEWa\x1C\xFEa\x16DV[\x80T\x82\x82U\x80\x83\x10\x15a\x1D3W_\x82\x81R` \x90 \x83\x81\x01\x90\x82\x01[\x80\x82\x10\x15a\x1D0W_\x82U`\x01\x82\x01\x91Pa\x1D\x1AV[PP[P_\x90\x81R` \x81 \x90[\x82\x81\x10\x15a\x1DZW\x835\x82\x82\x01U` \x90\x93\x01\x92`\x01\x01a\x1D>V[PPPPPPV[_` \x82\x84\x03\x12\x15a\x1DrW__\xFD[\x815a\x17n\x81a\x16\x88V[_` \x82\x84\x03\x12\x15a\x1D\x8DW__\xFD[a\x17n\x82a\x16\x04V[_`\x80\x82\x01\x865a\x1D\xA6\x81a\x16\x88V[`\x01`\x01`\xA0\x1B\x03\x16\x83Rc\xFF\xFF\xFF\xFFa\x1D\xC2` \x89\x01a\x16\x04V[\x16` \x84\x01Rc\xFF\xFF\xFF\xFF\x86\x16`@\x84\x01R`\x80``\x84\x01R\x83\x90R`\xA0`\x05\x84\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x85_`>\x196\x83\x90\x03\x01[\x87\x82\x10\x15a\x1E\xBAW\x86\x85\x03`\x9F\x19\x01\x84R\x825\x81\x81\x12a\x1E\x15W__\xFD[\x89\x01\x805a\x1E\"\x81a\x16\x88V[`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x81\x0156\x82\x90\x03`\x1E\x19\x01\x81\x12a\x1EDW__\xFD[\x01` \x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E^W__\xFD[\x80`\x05\x1B\x806\x03\x83\x13\x15a\x1EpW__\xFD[`@` \x89\x01\x81\x90R\x88\x01\x82\x90R`\x01`\x01`\xFB\x1B\x03\x82\x11\x15a\x1E\x91W__\xFD[\x80\x83``\x8A\x017``\x81\x89\x01\x01\x97PPPP` \x83\x01\x92P` \x84\x01\x93P`\x01\x82\x01\x91Pa\x1D\xF7V[P\x92\x99\x98PPPPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x03\x02Wa\x03\x02a\x1C4V[_`\x01\x82\x01a\x1E\xEDWa\x1E\xEDa\x1C4V[P`\x01\x01\x90V[_` \x82\x84\x03\x12\x15a\x1F\x04W__\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a\x17nW__\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a\x1F7Wa\x1F7a\x1F\x15V[P\x04\x90V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x17\x85W_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x03\x02Wa\x03\x02a\x1C4V[_` \x82\x84\x03\x12\x15a\x1F\x8BW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x17nW__\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1F\xAFW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1F\xC8W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\t\x9DW__\xFD[_\x82a\x1F\xEAWa\x1F\xEAa\x1F\x15V[P\x06\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\x02Wa\x03\x02a\x1C4V\xFE\xA2dipfsX\"\x12 \x8D\n\x87\x8A\x04.\x9B\x91\x80\x9F\x87g\x92\x9E\x9C\x01\xD8\xA7\xB5\x9A|\x04k\xFD\x14\x8B\x8E\xBBtrS\xBBdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610106575f3560e01c80637c85ac4c1161009e578063be86e0b21161006e578063be86e0b21461026e578063c0da24201461028f578063cd83a72b146102a2578063e49613fc146102c5578063f698da25146102e5575f5ffd5b80637c85ac4c146101fa5780637d1d1f5b1461021a57806380c7d3f31461023a578063848189201461025b575f5ffd5b80635ddb9b5b116100d95780635ddb9b5b1461016d5780636141879e1461019557806368d6e081146101a8578063702ca531146101e7575f5ffd5b8063184674341461010a57806323c2a3cb1461013057806354fd4d501461014357806356d482f514610158575b5f5ffd5b61011d61011836600461161c565b6102ed565b6040519081526020015b60405180910390f35b61011d61013e3660046116fc565b610308565b61014b61033b565b604051610127919061175c565b61016b6101663660046117cb565b61036b565b005b61018061017b36600461183d565b610582565b60405163ffffffff9091168152602001610127565b6101806101a336600461183d565b6105a8565b6101cf7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610127565b61014b6101f536600461161c565b6105ce565b61020d6102083660046116fc565b610665565b60405161012791906118ac565b61022d61022836600461190f565b6107c3565b6040516101279190611964565b61024d610248366004611986565b610986565b604051610127929190611a0a565b6101cf61026936600461183d565b6109a4565b61028161027c366004611a37565b6109cd565b604051610127929190611b1d565b61028161029d366004611b3f565b610a6d565b6102b56102b03660046116fc565b610b6f565b6040519015158152602001610127565b6102d86102d3366004611bb5565b610ba5565b6040516101279190611bf0565b61011d610c9e565b5f6102f883836105ce565b8051906020012090505b92915050565b5f5f61031384610d5e565b5f90815260046020908152604080832063ffffffff8716845290915290205491505092915050565b60606103667f0000000000000000000000000000000000000000000000000000000000000000610dc1565b905090565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146103b45760405163030c1b6b60e11b815260040160405180910390fd5b5f6103cc6103c73688900388018861183d565b610d5e565b5f8181526003602052604090205490915063ffffffff9081169086161161040657604051632f20889f60e01b815260040160405180910390fd5b5f81815260046020908152604080832063ffffffff8916845290915281208490555b8381101561048e5784848281811061044257610442611c02565b90506020028101906104549190611c16565b5f83815260056020908152604080832063ffffffff8b168452825280832085845290915290206104848282611c5f565b5050600101610428565b505f818152600360209081526040909120805463ffffffff191663ffffffff88161790556104be90830183611d62565b5f8281526001602090815260409182902080546001600160a01b0319166001600160a01b0394909416939093179092556104fd91908401908401611d7d565b5f828152600260209081526040808320805463ffffffff191663ffffffff958616179055600682528083209389168352929052819020805460ff19166001179055517f4f588da9ec57976194a79b5594f8f8782923d93013df2b9ed12fe125805011ef90610572908890889088908890611d96565b60405180910390a1505050505050565b5f5f61058d83610d5e565b5f9081526003602052604090205463ffffffff169392505050565b5f5f6105b383610d5e565b5f9081526002602052604090205463ffffffff169392505050565b604080517fda346acb3ce99e7c5132bf8cafb159ad8085970ebfdba78007ef0fe163063d1460208083019190915263ffffffff85168284015260608083018590528351808403820181526080909301909352815191012061062d610c9e565b60405161190160f01b602082015260228101919091526042810182905260620160405160208183030381529060405291505092915050565b60605f61067184610d5e565b5f81815260046020908152604080832063ffffffff8089168552925282205492935082166001600160401b038111156106ac576106ac611644565b6040519080825280602002602001820160405280156106f157816020015b604080518082019091525f8152606060208201528152602001906001900390816106ca5790505b5090505f5b8263ffffffff168110156107b9575f84815260056020908152604080832063ffffffff8a16845282528083208484528252918290208251808401845281546001600160a01b031681526001820180548551818602810186019096528086529194929385810193929083018282801561078b57602002820191905f5260205f20905b815481526020019060010190808311610777575b5050505050815250508282815181106107a6576107a6611c02565b60209081029190910101526001016106f6565b5095945050505050565b60605f6107d86103c73686900386018661183d565b5f81815260046020908152604080832063ffffffff881684529091529020549091508061081857604051631029081560e21b815260040160405180910390fd5b5f82815260056020908152604080832063ffffffff88168452825280832083805290915281206001015490816001600160401b0381111561085b5761085b611644565b604051908082528060200260200182016040528015610884578160200160208202803683370190505b5090505f5b8381101561097b575f85815260056020908152604080832063ffffffff8b16845282528083208484528252808320600101805482518185028101850190935280835291929091908301828280156108fd57602002820191905f5260205f20905b8154815260200190600101908083116108e9575b509394505f93505050505b81518110801561091757508481105b156109715781818151811061092e5761092e611c02565b602002602001015184828151811061094857610948611c02565b6020026020010181815161095c9190611ec9565b9052508061096981611edc565b915050610908565b5050600101610889565b509695505050505050565b6060805f5f6109958686610dfe565b909450925050505b9250929050565b5f5f6109af83610d5e565b5f908152600160205260409020546001600160a01b03169392505050565b5f60605f5f6109dc8787610dfe565b915091508451825114610a025760405163512509d360e11b815260040160405180910390fd5b5f5b8251811015610a5c57858181518110610a1f57610a1f611c02565b6020026020010151838281518110610a3957610a39611c02565b60200260200101511015610a5457505f93509150610a659050565b600101610a04565b50600193509150505b935093915050565b5f60605f5f610a7c8888610dfe565b90925090505f610a938961022860208b018b611d7d565b83519091508614610ab75760405163512509d360e11b815260040160405180910390fd5b5f5b8351811015610b5b575f612710898984818110610ad857610ad8611c02565b9050602002016020810190610aed9190611ef4565b61ffff16848481518110610b0357610b03611c02565b6020026020010151610b159190611c48565b610b1f9190611f29565b905080858381518110610b3457610b34611c02565b60200260200101511015610b52575f84965096505050505050610b66565b50600101610ab9565b506001945090925050505b94509492505050565b5f5f610b7a84610d5e565b5f90815260066020908152604080832063ffffffff8716845290915290205460ff1691505092915050565b604080518082019091525f8152606060208201525f610bc385610d5e565b5f81815260046020908152604080832063ffffffff891684529091529020549091508310610c0457604051634e23d03560e01b815260040160405180910390fd5b5f81815260056020908152604080832063ffffffff8816845282528083208684528252918290208251808401845281546001600160a01b0316815260018201805485518186028101860190965280865291949293858101939290830182828015610c8b57602002820191905f5260205f20905b815481526020019060010190808311610c77575b5050505050815250509150509392505050565b60408051808201909152600a81526922b4b3b2b72630bcb2b960b11b6020909101525f7f91ab3d17e3a50a9d89e63fd30b92be7f5336b03b287bb946787a83a9d62a27667f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea610d0b61103e565b8051602091820120604051610d43949392309101938452602084019290925260408301526001600160a01b0316606082015260800190565b60405160208183030381529060405280519060200120905090565b5f815f0151826020015163ffffffff16604051602001610da992919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b60405160208183030381529060405261030290611f3c565b60605f610dcd836110b3565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b6060805f610e146103c73687900387018761183d565b5f8181526002602052604090205490915063ffffffff161580610e6a57505f8181526002602090815260409091205463ffffffff1690610e5690860186611d7d565b610e609190611f5f565b63ffffffff164211155b610e875760405163640fcd6b60e11b815260040160405180910390fd5b5f81815260066020908152604082209190610ea490870187611d7d565b63ffffffff16815260208101919091526040015f205460ff16610eda57604051630cad17b760e31b815260040160405180910390fd5b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000166364e1df84610f166020870187611d7d565b6040516001600160e01b031960e084901b16815263ffffffff919091166004820152602401602060405180830381865afa158015610f56573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f7a9190611f7b565b610f9757604051631b14174b60e01b815260040160405180910390fd5b5f610fb2610fa86020870187611d7d565b86602001356102ed565b90505f610fff82610fc66040890189611f9a565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152506110da92505050565b90505f6110138861022860208a018a611d7d565b5190505f61102f8561102860208b018b611d7d565b85856112e1565b99929850919650505050505050565b60605f61106a7f0000000000000000000000000000000000000000000000000000000000000000610dc1565b9050805f8151811061107e5761107e611c02565b016020908101516040516001600160f81b03199091169181019190915260210160405160208183030381529060405291505090565b5f60ff8216601f81111561030257604051632cd44ac360e21b815260040160405180910390fd5b60605f82511180156110f75750604182516110f59190611fdc565b155b61111457604051634be6321b60e01b815260040160405180910390fd5b5f604183516111239190611f29565b9050806001600160401b0381111561113d5761113d611644565b604051908082528060200260200182016040528015611166578160200160208202803683370190505b5091505f5b818110156112d957604080516041808252608082019092525f916020820181803683370190505090505f5b60418110156112015785816111ac856041611c48565b6111b69190611ec9565b815181106111c6576111c6611c02565b602001015160f81c60f81b8282815181106111e3576111e3611c02565b60200101906001600160f81b03191690815f1a905350600101611196565b505f5f61120e888461150b565b90925090505f81600481111561122657611226611fef565b1461124457604051638baa579f60e01b815260040160405180910390fd5b831580611285575085611258600186612003565b8151811061126857611268611c02565b60200260200101516001600160a01b0316826001600160a01b0316115b6112a257604051630b550c5760e41b815260040160405180910390fd5b818685815181106112b5576112b5611c02565b6001600160a01b03929092166020928302919091019091015250505060010161116b565b505092915050565b5f84815260046020908152604080832063ffffffff87168452909152902054606090826001600160401b0381111561131b5761131b611644565b604051908082528060200260200182016040528015611344578160200160208202803683370190505b5091505f5b8451811015611501575f85828151811061136557611365611c02565b602002602001015190505f5f905061139860405180604001604052805f6001600160a01b03168152602001606081525090565b5f5b85811015611461575f8b815260056020908152604080832063ffffffff8e16845282528083208484528252918290208251808401845281546001600160a01b031681526001820180548551818602810186019096528086529194929385810193929083018282801561142957602002820191905f5260205f20905b815481526020019060010190808311611415575b5050505050815250509150836001600160a01b0316825f01516001600160a01b0316036114595760019250611461565b60010161139a565b50816114805760405163439cc0cd60e01b815260040160405180910390fd5b60208101515f5b81518110801561149657508881105b156114f0578181815181106114ad576114ad611c02565b60200260200101518882815181106114c7576114c7611c02565b602002602001018181516114db9190611ec9565b905250806114e881611edc565b915050611487565b505060019093019250611349915050565b5050949350505050565b5f5f825160410361153f576020830151604084015160608501515f1a6115338782858561154a565b9450945050505061099d565b505f9050600261099d565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a083111561157f57505f90506003610b66565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa1580156115d0573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b0381166115f8575f60019250925050610b66565b965f9650945050505050565b803563ffffffff81168114611617575f5ffd5b919050565b5f5f6040838503121561162d575f5ffd5b61163683611604565b946020939093013593505050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b038111828210171561168057611680611644565b604052919050565b6001600160a01b038116811461169c575f5ffd5b50565b5f604082840312156116af575f5ffd5b604080519081016001600160401b03811182821017156116d1576116d1611644565b60405290508082356116e281611688565b81526116f060208401611604565b60208201525092915050565b5f5f6060838503121561170d575f5ffd5b611717848461169f565b915061172560408401611604565b90509250929050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f61176e602083018461172e565b9392505050565b5f60408284031215611785575f5ffd5b50919050565b5f5f83601f84011261179b575f5ffd5b5081356001600160401b038111156117b1575f5ffd5b6020830191508360208260051b850101111561099d575f5ffd5b5f5f5f5f5f60c086880312156117df575f5ffd5b6117e98787611775565b94506117f760408701611604565b935060608601356001600160401b03811115611811575f5ffd5b61181d8882890161178b565b909450925061183190508760808801611775565b90509295509295909350565b5f6040828403121561184d575f5ffd5b61176e838361169f565b80516001600160a01b03168252602080820151604082850181905281519085018190525f929190910190829060608601905b808310156107b95783518252602082019150602084019350600183019250611889565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561190357603f198786030184526118ee858351611857565b945060209384019391909101906001016118d2565b50929695505050505050565b5f5f60608385031215611920575f5ffd5b6117178484611775565b5f8151808452602084019350602083015f5b8281101561195a57815186526020958601959091019060010161193c565b5093949350505050565b602081525f61176e602083018461192a565b5f60608284031215611785575f5ffd5b5f5f60608385031215611997575f5ffd5b6119a18484611775565b915060408301356001600160401b038111156119bb575f5ffd5b6119c785828601611976565b9150509250929050565b5f8151808452602084019350602083015f5b8281101561195a5781516001600160a01b03168652602095860195909101906001016119e3565b604081525f611a1c604083018561192a565b8281036020840152611a2e81856119d1565b95945050505050565b5f5f5f60808486031215611a49575f5ffd5b611a538585611775565b925060408401356001600160401b03811115611a6d575f5ffd5b611a7986828701611976565b92505060608401356001600160401b03811115611a94575f5ffd5b8401601f81018613611aa4575f5ffd5b80356001600160401b03811115611abd57611abd611644565b8060051b611acd60208201611658565b91825260208184018101929081019089841115611ae8575f5ffd5b6020850194505b83851015611b0e57843580835260209586019590935090910190611aef565b80955050505050509250925092565b8215158152604060208201525f611b3760408301846119d1565b949350505050565b5f5f5f5f60808587031215611b52575f5ffd5b611b5c8686611775565b935060408501356001600160401b03811115611b76575f5ffd5b611b8287828801611976565b93505060608501356001600160401b03811115611b9d575f5ffd5b611ba98782880161178b565b95989497509550505050565b5f5f5f60808486031215611bc7575f5ffd5b611bd1858561169f565b9250611bdf60408501611604565b929592945050506060919091013590565b602081525f61176e6020830184611857565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e19833603018112611c2a575f5ffd5b9190910192915050565b634e487b7160e01b5f52601160045260245ffd5b808202811582820484141761030257610302611c34565b8135611c6a81611688565b81546001600160a01b0319166001600160a01b0391909116178155602082013536839003601e19018112611c9c575f5ffd5b820180356001600160401b03811115611cb3575f5ffd5b6020820191508060051b3603821315611cca575f5ffd5b600183016001600160401b03821115611ce557611ce5611644565b68010000000000000000821115611cfe57611cfe611644565b805482825580831015611d33575f828152602090208381019082015b80821015611d30575f8255600182019150611d1a565b50505b505f90815260208120905b82811015611d5a57833582820155602090930192600101611d3e565b505050505050565b5f60208284031215611d72575f5ffd5b813561176e81611688565b5f60208284031215611d8d575f5ffd5b61176e82611604565b5f608082018635611da681611688565b6001600160a01b0316835263ffffffff611dc260208901611604565b16602084015263ffffffff861660408401526080606084015283905260a0600584901b83018101908301855f603e1936839003015b87821015611eba57868503609f190184528235818112611e15575f5ffd5b89018035611e2281611688565b6001600160a01b03168652602081013536829003601e19018112611e44575f5ffd5b016020810190356001600160401b03811115611e5e575f5ffd5b8060051b803603831315611e70575f5ffd5b60406020890181905288018290526001600160fb1b03821115611e91575f5ffd5b808360608a01376060818901019750505050602083019250602084019350600182019150611df7565b50929998505050505050505050565b8082018082111561030257610302611c34565b5f60018201611eed57611eed611c34565b5060010190565b5f60208284031215611f04575f5ffd5b813561ffff8116811461176e575f5ffd5b634e487b7160e01b5f52601260045260245ffd5b5f82611f3757611f37611f15565b500490565b80516020808301519190811015611785575f1960209190910360031b1b16919050565b63ffffffff818116838216019081111561030257610302611c34565b5f60208284031215611f8b575f5ffd5b8151801515811461176e575f5ffd5b5f5f8335601e19843603018112611faf575f5ffd5b8301803591506001600160401b03821115611fc8575f5ffd5b60200191503681900382131561099d575f5ffd5b5f82611fea57611fea611f15565b500690565b634e487b7160e01b5f52602160045260245ffd5b8181038181111561030257610302611c3456fea26469706673582212208d0a878a042e9b91809f8767929e9c01d8a7b59a7c046bfd148b8ebb747253bb64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\x06W_5`\xE0\x1C\x80c|\x85\xACL\x11a\0\x9EW\x80c\xBE\x86\xE0\xB2\x11a\0nW\x80c\xBE\x86\xE0\xB2\x14a\x02nW\x80c\xC0\xDA$ \x14a\x02\x8FW\x80c\xCD\x83\xA7+\x14a\x02\xA2W\x80c\xE4\x96\x13\xFC\x14a\x02\xC5W\x80c\xF6\x98\xDA%\x14a\x02\xE5W__\xFD[\x80c|\x85\xACL\x14a\x01\xFAW\x80c}\x1D\x1F[\x14a\x02\x1AW\x80c\x80\xC7\xD3\xF3\x14a\x02:W\x80c\x84\x81\x89 \x14a\x02[W__\xFD[\x80c]\xDB\x9B[\x11a\0\xD9W\x80c]\xDB\x9B[\x14a\x01mW\x80caA\x87\x9E\x14a\x01\x95W\x80ch\xD6\xE0\x81\x14a\x01\xA8W\x80cp,\xA51\x14a\x01\xE7W__\xFD[\x80c\x18Ft4\x14a\x01\nW\x80c#\xC2\xA3\xCB\x14a\x010W\x80cT\xFDMP\x14a\x01CW\x80cV\xD4\x82\xF5\x14a\x01XW[__\xFD[a\x01\x1Da\x01\x186`\x04a\x16\x1CV[a\x02\xEDV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x1Da\x01>6`\x04a\x16\xFCV[a\x03\x08V[a\x01Ka\x03;V[`@Qa\x01'\x91\x90a\x17\\V[a\x01ka\x01f6`\x04a\x17\xCBV[a\x03kV[\0[a\x01\x80a\x01{6`\x04a\x18=V[a\x05\x82V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01'V[a\x01\x80a\x01\xA36`\x04a\x18=V[a\x05\xA8V[a\x01\xCF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01'V[a\x01Ka\x01\xF56`\x04a\x16\x1CV[a\x05\xCEV[a\x02\ra\x02\x086`\x04a\x16\xFCV[a\x06eV[`@Qa\x01'\x91\x90a\x18\xACV[a\x02-a\x02(6`\x04a\x19\x0FV[a\x07\xC3V[`@Qa\x01'\x91\x90a\x19dV[a\x02Ma\x02H6`\x04a\x19\x86V[a\t\x86V[`@Qa\x01'\x92\x91\x90a\x1A\nV[a\x01\xCFa\x02i6`\x04a\x18=V[a\t\xA4V[a\x02\x81a\x02|6`\x04a\x1A7V[a\t\xCDV[`@Qa\x01'\x92\x91\x90a\x1B\x1DV[a\x02\x81a\x02\x9D6`\x04a\x1B?V[a\nmV[a\x02\xB5a\x02\xB06`\x04a\x16\xFCV[a\x0BoV[`@Q\x90\x15\x15\x81R` \x01a\x01'V[a\x02\xD8a\x02\xD36`\x04a\x1B\xB5V[a\x0B\xA5V[`@Qa\x01'\x91\x90a\x1B\xF0V[a\x01\x1Da\x0C\x9EV[_a\x02\xF8\x83\x83a\x05\xCEV[\x80Q\x90` \x01 \x90P[\x92\x91PPV[__a\x03\x13\x84a\r^V[_\x90\x81R`\x04` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 T\x91PP\x92\x91PPV[``a\x03f\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\xC1V[\x90P\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03\xB4W`@Qc\x03\x0C\x1Bk`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x03\xCCa\x03\xC76\x88\x90\x03\x88\x01\x88a\x18=V[a\r^V[_\x81\x81R`\x03` R`@\x90 T\x90\x91Pc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x86\x16\x11a\x04\x06W`@Qc/ \x88\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x04` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x90\x91R\x81 \x84\x90U[\x83\x81\x10\x15a\x04\x8EW\x84\x84\x82\x81\x81\x10a\x04BWa\x04Ba\x1C\x02V[\x90P` \x02\x81\x01\x90a\x04T\x91\x90a\x1C\x16V[_\x83\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x8B\x16\x84R\x82R\x80\x83 \x85\x84R\x90\x91R\x90 a\x04\x84\x82\x82a\x1C_V[PP`\x01\x01a\x04(V[P_\x81\x81R`\x03` \x90\x81R`@\x90\x91 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x88\x16\x17\x90Ua\x04\xBE\x90\x83\x01\x83a\x1DbV[_\x82\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x93\x90\x93\x17\x90\x92Ua\x04\xFD\x91\x90\x84\x01\x90\x84\x01a\x1D}V[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x95\x86\x16\x17\x90U`\x06\x82R\x80\x83 \x93\x89\x16\x83R\x92\x90R\x81\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UQ\x7FOX\x8D\xA9\xECW\x97a\x94\xA7\x9BU\x94\xF8\xF8x)#\xD90\x13\xDF+\x9E\xD1/\xE1%\x80P\x11\xEF\x90a\x05r\x90\x88\x90\x88\x90\x88\x90\x88\x90a\x1D\x96V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[__a\x05\x8D\x83a\r^V[_\x90\x81R`\x03` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x93\x92PPPV[__a\x05\xB3\x83a\r^V[_\x90\x81R`\x02` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x93\x92PPPV[`@\x80Q\x7F\xDA4j\xCB<\xE9\x9E|Q2\xBF\x8C\xAF\xB1Y\xAD\x80\x85\x97\x0E\xBF\xDB\xA7\x80\x07\xEF\x0F\xE1c\x06=\x14` \x80\x83\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x85\x16\x82\x84\x01R``\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x82\x01\x81R`\x80\x90\x93\x01\x90\x93R\x81Q\x91\x01 a\x06-a\x0C\x9EV[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x82\x90R`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x92\x91PPV[``_a\x06q\x84a\r^V[_\x81\x81R`\x04` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x80\x89\x16\x85R\x92R\x82 T\x92\x93P\x82\x16`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xACWa\x06\xACa\x16DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xF1W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x06\xCAW\x90P[P\x90P_[\x82c\xFF\xFF\xFF\xFF\x16\x81\x10\x15a\x07\xB9W_\x84\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x8A\x16\x84R\x82R\x80\x83 \x84\x84R\x82R\x91\x82\x90 \x82Q\x80\x84\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01\x80T\x85Q\x81\x86\x02\x81\x01\x86\x01\x90\x96R\x80\x86R\x91\x94\x92\x93\x85\x81\x01\x93\x92\x90\x83\x01\x82\x82\x80\x15a\x07\x8BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x07wW[PPPPP\x81RPP\x82\x82\x81Q\x81\x10a\x07\xA6Wa\x07\xA6a\x1C\x02V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x06\xF6V[P\x95\x94PPPPPV[``_a\x07\xD8a\x03\xC76\x86\x90\x03\x86\x01\x86a\x18=V[_\x81\x81R`\x04` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x88\x16\x84R\x90\x91R\x90 T\x90\x91P\x80a\x08\x18W`@Qc\x10)\x08\x15`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x88\x16\x84R\x82R\x80\x83 \x83\x80R\x90\x91R\x81 `\x01\x01T\x90\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08[Wa\x08[a\x16DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\x84W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\t{W_\x85\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x8B\x16\x84R\x82R\x80\x83 \x84\x84R\x82R\x80\x83 `\x01\x01\x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R\x91\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\x08\xFDW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xE9W[P\x93\x94P_\x93PPPP[\x81Q\x81\x10\x80\x15a\t\x17WP\x84\x81\x10[\x15a\tqW\x81\x81\x81Q\x81\x10a\t.Wa\t.a\x1C\x02V[` \x02` \x01\x01Q\x84\x82\x81Q\x81\x10a\tHWa\tHa\x1C\x02V[` \x02` \x01\x01\x81\x81Qa\t\\\x91\x90a\x1E\xC9V[\x90RP\x80a\ti\x81a\x1E\xDCV[\x91PPa\t\x08V[PP`\x01\x01a\x08\x89V[P\x96\x95PPPPPPV[``\x80__a\t\x95\x86\x86a\r\xFEV[\x90\x94P\x92PPP[\x92P\x92\x90PV[__a\t\xAF\x83a\r^V[_\x90\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x93\x92PPPV[_``__a\t\xDC\x87\x87a\r\xFEV[\x91P\x91P\x84Q\x82Q\x14a\n\x02W`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82Q\x81\x10\x15a\n\\W\x85\x81\x81Q\x81\x10a\n\x1FWa\n\x1Fa\x1C\x02V[` \x02` \x01\x01Q\x83\x82\x81Q\x81\x10a\n9Wa\n9a\x1C\x02V[` \x02` \x01\x01Q\x10\x15a\nTWP_\x93P\x91Pa\ne\x90PV[`\x01\x01a\n\x04V[P`\x01\x93P\x91PP[\x93P\x93\x91PPV[_``__a\n|\x88\x88a\r\xFEV[\x90\x92P\x90P_a\n\x93\x89a\x02(` \x8B\x01\x8Ba\x1D}V[\x83Q\x90\x91P\x86\x14a\n\xB7W`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x83Q\x81\x10\x15a\x0B[W_a'\x10\x89\x89\x84\x81\x81\x10a\n\xD8Wa\n\xD8a\x1C\x02V[\x90P` \x02\x01` \x81\x01\x90a\n\xED\x91\x90a\x1E\xF4V[a\xFF\xFF\x16\x84\x84\x81Q\x81\x10a\x0B\x03Wa\x0B\x03a\x1C\x02V[` \x02` \x01\x01Qa\x0B\x15\x91\x90a\x1CHV[a\x0B\x1F\x91\x90a\x1F)V[\x90P\x80\x85\x83\x81Q\x81\x10a\x0B4Wa\x0B4a\x1C\x02V[` \x02` \x01\x01Q\x10\x15a\x0BRW_\x84\x96P\x96PPPPPPa\x0BfV[P`\x01\x01a\n\xB9V[P`\x01\x94P\x90\x92PPP[\x94P\x94\x92PPPV[__a\x0Bz\x84a\r^V[_\x90\x81R`\x06` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 T`\xFF\x16\x91PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x81R``` \x82\x01R_a\x0B\xC3\x85a\r^V[_\x81\x81R`\x04` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x90\x91R\x90 T\x90\x91P\x83\x10a\x0C\x04W`@QcN#\xD05`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x88\x16\x84R\x82R\x80\x83 \x86\x84R\x82R\x91\x82\x90 \x82Q\x80\x84\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01\x80T\x85Q\x81\x86\x02\x81\x01\x86\x01\x90\x96R\x80\x86R\x91\x94\x92\x93\x85\x81\x01\x93\x92\x90\x83\x01\x82\x82\x80\x15a\x0C\x8BW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x0CwW[PPPPP\x81RPP\x91PP\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x90\x91\x01R_\x7F\x91\xAB=\x17\xE3\xA5\n\x9D\x89\xE6?\xD3\x0B\x92\xBE\x7FS6\xB0;({\xB9Fxz\x83\xA9\xD6*'f\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEAa\r\x0Ba\x10>V[\x80Q` \x91\x82\x01 `@Qa\rC\x94\x93\x920\x91\x01\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a\r\xA9\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x03\x02\x90a\x1F<V[``_a\r\xCD\x83a\x10\xB3V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[``\x80_a\x0E\x14a\x03\xC76\x87\x90\x03\x87\x01\x87a\x18=V[_\x81\x81R`\x02` R`@\x90 T\x90\x91Pc\xFF\xFF\xFF\xFF\x16\x15\x80a\x0EjWP_\x81\x81R`\x02` \x90\x81R`@\x90\x91 Tc\xFF\xFF\xFF\xFF\x16\x90a\x0EV\x90\x86\x01\x86a\x1D}V[a\x0E`\x91\x90a\x1F_V[c\xFF\xFF\xFF\xFF\x16B\x11\x15[a\x0E\x87W`@Qcd\x0F\xCDk`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x06` \x90\x81R`@\x82 \x91\x90a\x0E\xA4\x90\x87\x01\x87a\x1D}V[c\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16a\x0E\xDAW`@Qc\x0C\xAD\x17\xB7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16cd\xE1\xDF\x84a\x0F\x16` \x87\x01\x87a\x1D}V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FVW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Fz\x91\x90a\x1F{V[a\x0F\x97W`@Qc\x1B\x14\x17K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0F\xB2a\x0F\xA8` \x87\x01\x87a\x1D}V[\x86` \x015a\x02\xEDV[\x90P_a\x0F\xFF\x82a\x0F\xC6`@\x89\x01\x89a\x1F\x9AV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x10\xDA\x92PPPV[\x90P_a\x10\x13\x88a\x02(` \x8A\x01\x8Aa\x1D}V[Q\x90P_a\x10/\x85a\x10(` \x8B\x01\x8Ba\x1D}V[\x85\x85a\x12\xE1V[\x99\x92\x98P\x91\x96PPPPPPPV[``_a\x10j\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\xC1V[\x90P\x80_\x81Q\x81\x10a\x10~Wa\x10~a\x1C\x02V[\x01` \x90\x81\x01Q`@Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x91\x81\x01\x91\x90\x91R`!\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x90V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x03\x02W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``_\x82Q\x11\x80\x15a\x10\xF7WP`A\x82Qa\x10\xF5\x91\x90a\x1F\xDCV[\x15[a\x11\x14W`@QcK\xE62\x1B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`A\x83Qa\x11#\x91\x90a\x1F)V[\x90P\x80`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11=Wa\x11=a\x16DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11fW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P_[\x81\x81\x10\x15a\x12\xD9W`@\x80Q`A\x80\x82R`\x80\x82\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P_[`A\x81\x10\x15a\x12\x01W\x85\x81a\x11\xAC\x85`Aa\x1CHV[a\x11\xB6\x91\x90a\x1E\xC9V[\x81Q\x81\x10a\x11\xC6Wa\x11\xC6a\x1C\x02V[` \x01\x01Q`\xF8\x1C`\xF8\x1B\x82\x82\x81Q\x81\x10a\x11\xE3Wa\x11\xE3a\x1C\x02V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x01\x01a\x11\x96V[P__a\x12\x0E\x88\x84a\x15\x0BV[\x90\x92P\x90P_\x81`\x04\x81\x11\x15a\x12&Wa\x12&a\x1F\xEFV[\x14a\x12DW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x15\x80a\x12\x85WP\x85a\x12X`\x01\x86a \x03V[\x81Q\x81\x10a\x12hWa\x12ha\x1C\x02V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x11[a\x12\xA2W`@Qc\x0BU\x0CW`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x86\x85\x81Q\x81\x10a\x12\xB5Wa\x12\xB5a\x1C\x02V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPPP`\x01\x01a\x11kV[PP\x92\x91PPV[_\x84\x81R`\x04` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x87\x16\x84R\x90\x91R\x90 T``\x90\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\x1BWa\x13\x1Ba\x16DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P_[\x84Q\x81\x10\x15a\x15\x01W_\x85\x82\x81Q\x81\x10a\x13eWa\x13ea\x1C\x02V[` \x02` \x01\x01Q\x90P__\x90Pa\x13\x98`@Q\x80`@\x01`@R\x80_`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81RP\x90V[_[\x85\x81\x10\x15a\x14aW_\x8B\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x8E\x16\x84R\x82R\x80\x83 \x84\x84R\x82R\x91\x82\x90 \x82Q\x80\x84\x01\x84R\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x82\x01\x80T\x85Q\x81\x86\x02\x81\x01\x86\x01\x90\x96R\x80\x86R\x91\x94\x92\x93\x85\x81\x01\x93\x92\x90\x83\x01\x82\x82\x80\x15a\x14)W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x14\x15W[PPPPP\x81RPP\x91P\x83`\x01`\x01`\xA0\x1B\x03\x16\x82_\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x03a\x14YW`\x01\x92Pa\x14aV[`\x01\x01a\x13\x9AV[P\x81a\x14\x80W`@QcC\x9C\xC0\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q_[\x81Q\x81\x10\x80\x15a\x14\x96WP\x88\x81\x10[\x15a\x14\xF0W\x81\x81\x81Q\x81\x10a\x14\xADWa\x14\xADa\x1C\x02V[` \x02` \x01\x01Q\x88\x82\x81Q\x81\x10a\x14\xC7Wa\x14\xC7a\x1C\x02V[` \x02` \x01\x01\x81\x81Qa\x14\xDB\x91\x90a\x1E\xC9V[\x90RP\x80a\x14\xE8\x81a\x1E\xDCV[\x91PPa\x14\x87V[PP`\x01\x90\x93\x01\x92Pa\x13I\x91PPV[PP\x94\x93PPPPV[__\x82Q`A\x03a\x15?W` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa\x153\x87\x82\x85\x85a\x15JV[\x94P\x94PPPPa\t\x9DV[P_\x90P`\x02a\t\x9DV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\x15\x7FWP_\x90P`\x03a\x0BfV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x15\xD0W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x15\xF8W_`\x01\x92P\x92PPa\x0BfV[\x96_\x96P\x94PPPPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x16\x17W__\xFD[\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x16-W__\xFD[a\x166\x83a\x16\x04V[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x16\x80Wa\x16\x80a\x16DV[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x16\x9CW__\xFD[PV[_`@\x82\x84\x03\x12\x15a\x16\xAFW__\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x16\xD1Wa\x16\xD1a\x16DV[`@R\x90P\x80\x825a\x16\xE2\x81a\x16\x88V[\x81Ra\x16\xF0` \x84\x01a\x16\x04V[` \x82\x01RP\x92\x91PPV[__``\x83\x85\x03\x12\x15a\x17\rW__\xFD[a\x17\x17\x84\x84a\x16\x9FV[\x91Pa\x17%`@\x84\x01a\x16\x04V[\x90P\x92P\x92\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x17n` \x83\x01\x84a\x17.V[\x93\x92PPPV[_`@\x82\x84\x03\x12\x15a\x17\x85W__\xFD[P\x91\x90PV[__\x83`\x1F\x84\x01\x12a\x17\x9BW__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xB1W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\t\x9DW__\xFD[_____`\xC0\x86\x88\x03\x12\x15a\x17\xDFW__\xFD[a\x17\xE9\x87\x87a\x17uV[\x94Pa\x17\xF7`@\x87\x01a\x16\x04V[\x93P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x11W__\xFD[a\x18\x1D\x88\x82\x89\x01a\x17\x8BV[\x90\x94P\x92Pa\x181\x90P\x87`\x80\x88\x01a\x17uV[\x90P\x92\x95P\x92\x95\x90\x93PV[_`@\x82\x84\x03\x12\x15a\x18MW__\xFD[a\x17n\x83\x83a\x16\x9FV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x80\x82\x01Q`@\x82\x85\x01\x81\x90R\x81Q\x90\x85\x01\x81\x90R_\x92\x91\x90\x91\x01\x90\x82\x90``\x86\x01\x90[\x80\x83\x10\x15a\x07\xB9W\x83Q\x82R` \x82\x01\x91P` \x84\x01\x93P`\x01\x83\x01\x92Pa\x18\x89V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x19\x03W`?\x19\x87\x86\x03\x01\x84Ra\x18\xEE\x85\x83Qa\x18WV[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x18\xD2V[P\x92\x96\x95PPPPPPV[__``\x83\x85\x03\x12\x15a\x19 W__\xFD[a\x17\x17\x84\x84a\x17uV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x19ZW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x19<V[P\x93\x94\x93PPPPV[` \x81R_a\x17n` \x83\x01\x84a\x19*V[_``\x82\x84\x03\x12\x15a\x17\x85W__\xFD[__``\x83\x85\x03\x12\x15a\x19\x97W__\xFD[a\x19\xA1\x84\x84a\x17uV[\x91P`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xBBW__\xFD[a\x19\xC7\x85\x82\x86\x01a\x19vV[\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x19ZW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x19\xE3V[`@\x81R_a\x1A\x1C`@\x83\x01\x85a\x19*V[\x82\x81\x03` \x84\x01Ra\x1A.\x81\x85a\x19\xD1V[\x95\x94PPPPPV[___`\x80\x84\x86\x03\x12\x15a\x1AIW__\xFD[a\x1AS\x85\x85a\x17uV[\x92P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1AmW__\xFD[a\x1Ay\x86\x82\x87\x01a\x19vV[\x92PP``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\x94W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x1A\xA4W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xBDWa\x1A\xBDa\x16DV[\x80`\x05\x1Ba\x1A\xCD` \x82\x01a\x16XV[\x91\x82R` \x81\x84\x01\x81\x01\x92\x90\x81\x01\x90\x89\x84\x11\x15a\x1A\xE8W__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15a\x1B\x0EW\x845\x80\x83R` \x95\x86\x01\x95\x90\x93P\x90\x91\x01\x90a\x1A\xEFV[\x80\x95PPPPPP\x92P\x92P\x92V[\x82\x15\x15\x81R`@` \x82\x01R_a\x1B7`@\x83\x01\x84a\x19\xD1V[\x94\x93PPPPV[____`\x80\x85\x87\x03\x12\x15a\x1BRW__\xFD[a\x1B\\\x86\x86a\x17uV[\x93P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1BvW__\xFD[a\x1B\x82\x87\x82\x88\x01a\x19vV[\x93PP``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\x9DW__\xFD[a\x1B\xA9\x87\x82\x88\x01a\x17\x8BV[\x95\x98\x94\x97P\x95PPPPV[___`\x80\x84\x86\x03\x12\x15a\x1B\xC7W__\xFD[a\x1B\xD1\x85\x85a\x16\x9FV[\x92Pa\x1B\xDF`@\x85\x01a\x16\x04V[\x92\x95\x92\x94PPP``\x91\x90\x91\x015\x90V[` \x81R_a\x17n` \x83\x01\x84a\x18WV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12a\x1C*W__\xFD[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x02Wa\x03\x02a\x1C4V[\x815a\x1Cj\x81a\x16\x88V[\x81T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x17\x81U` \x82\x0156\x83\x90\x03`\x1E\x19\x01\x81\x12a\x1C\x9CW__\xFD[\x82\x01\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xB3W__\xFD[` \x82\x01\x91P\x80`\x05\x1B6\x03\x82\x13\x15a\x1C\xCAW__\xFD[`\x01\x83\x01`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1C\xE5Wa\x1C\xE5a\x16DV[h\x01\0\0\0\0\0\0\0\0\x82\x11\x15a\x1C\xFEWa\x1C\xFEa\x16DV[\x80T\x82\x82U\x80\x83\x10\x15a\x1D3W_\x82\x81R` \x90 \x83\x81\x01\x90\x82\x01[\x80\x82\x10\x15a\x1D0W_\x82U`\x01\x82\x01\x91Pa\x1D\x1AV[PP[P_\x90\x81R` \x81 \x90[\x82\x81\x10\x15a\x1DZW\x835\x82\x82\x01U` \x90\x93\x01\x92`\x01\x01a\x1D>V[PPPPPPV[_` \x82\x84\x03\x12\x15a\x1DrW__\xFD[\x815a\x17n\x81a\x16\x88V[_` \x82\x84\x03\x12\x15a\x1D\x8DW__\xFD[a\x17n\x82a\x16\x04V[_`\x80\x82\x01\x865a\x1D\xA6\x81a\x16\x88V[`\x01`\x01`\xA0\x1B\x03\x16\x83Rc\xFF\xFF\xFF\xFFa\x1D\xC2` \x89\x01a\x16\x04V[\x16` \x84\x01Rc\xFF\xFF\xFF\xFF\x86\x16`@\x84\x01R`\x80``\x84\x01R\x83\x90R`\xA0`\x05\x84\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x85_`>\x196\x83\x90\x03\x01[\x87\x82\x10\x15a\x1E\xBAW\x86\x85\x03`\x9F\x19\x01\x84R\x825\x81\x81\x12a\x1E\x15W__\xFD[\x89\x01\x805a\x1E\"\x81a\x16\x88V[`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x81\x0156\x82\x90\x03`\x1E\x19\x01\x81\x12a\x1EDW__\xFD[\x01` \x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E^W__\xFD[\x80`\x05\x1B\x806\x03\x83\x13\x15a\x1EpW__\xFD[`@` \x89\x01\x81\x90R\x88\x01\x82\x90R`\x01`\x01`\xFB\x1B\x03\x82\x11\x15a\x1E\x91W__\xFD[\x80\x83``\x8A\x017``\x81\x89\x01\x01\x97PPPP` \x83\x01\x92P` \x84\x01\x93P`\x01\x82\x01\x91Pa\x1D\xF7V[P\x92\x99\x98PPPPPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x03\x02Wa\x03\x02a\x1C4V[_`\x01\x82\x01a\x1E\xEDWa\x1E\xEDa\x1C4V[P`\x01\x01\x90V[_` \x82\x84\x03\x12\x15a\x1F\x04W__\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a\x17nW__\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a\x1F7Wa\x1F7a\x1F\x15V[P\x04\x90V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x17\x85W_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x03\x02Wa\x03\x02a\x1C4V[_` \x82\x84\x03\x12\x15a\x1F\x8BW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x17nW__\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1F\xAFW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1F\xC8W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\t\x9DW__\xFD[_\x82a\x1F\xEAWa\x1F\xEAa\x1F\x15V[P\x06\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\x02Wa\x03\x02a\x1C4V\xFE\xA2dipfsX\"\x12 \x8D\n\x87\x8A\x04.\x9B\x91\x80\x9F\x87g\x92\x9E\x9C\x01\xD8\xA7\xB5\x9A|\x04k\xFD\x14\x8B\x8E\xBBtrS\xBBdsolcC\0\x08\x1B\x003",
    );
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct OperatorSet { address avs; uint32 id; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSet {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub id: u32,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
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
        impl ::core::convert::From<OperatorSet> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSet) -> Self {
                (value.avs, value.id)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    avs: tuple.0,
                    id: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorSet {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorSet {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.id,
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
        impl alloy_sol_types::SolType for OperatorSet {
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
        impl alloy_sol_types::SolStruct for OperatorSet {
            const NAME: &'static str = "OperatorSet";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("OperatorSet(address avs,uint32 id)")
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.avs,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.id)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorSet {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.avs,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.id)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.avs,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.id, out);
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
    /**Custom error with signature `ArrayLengthMismatch()` and selector `0xa24a13a6`.
    ```solidity
    error ArrayLengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ArrayLengthMismatch;
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
        impl ::core::convert::From<ArrayLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: ArrayLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ArrayLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ArrayLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ArrayLengthMismatch()";
            const SELECTOR: [u8; 4] = [162u8, 74u8, 19u8, 166u8];
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
    /**Custom error with signature `CertificateStale()` and selector `0xc81f9ad6`.
    ```solidity
    error CertificateStale();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CertificateStale;
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
        impl ::core::convert::From<CertificateStale> for UnderlyingRustTuple<'_> {
            fn from(value: CertificateStale) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CertificateStale {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CertificateStale {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CertificateStale()";
            const SELECTOR: [u8; 4] = [200u8, 31u8, 154u8, 214u8];
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
    /**Custom error with signature `IndexOutOfBounds()` and selector `0x4e23d035`.
    ```solidity
    error IndexOutOfBounds();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IndexOutOfBounds;
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
        impl ::core::convert::From<IndexOutOfBounds> for UnderlyingRustTuple<'_> {
            fn from(value: IndexOutOfBounds) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IndexOutOfBounds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IndexOutOfBounds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IndexOutOfBounds()";
            const SELECTOR: [u8; 4] = [78u8, 35u8, 208u8, 53u8];
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
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
    ```solidity
    error InvalidSignature();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature;
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
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
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
    /**Custom error with signature `InvalidSignatureLength()` and selector `0x4be6321b`.
    ```solidity
    error InvalidSignatureLength();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignatureLength;
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
        impl ::core::convert::From<InvalidSignatureLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignatureLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignatureLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignatureLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignatureLength()";
            const SELECTOR: [u8; 4] = [75u8, 230u8, 50u8, 27u8];
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
    /**Custom error with signature `OnlyTableUpdater()` and selector `0x061836d6`.
    ```solidity
    error OnlyTableUpdater();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyTableUpdater;
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
        impl ::core::convert::From<OnlyTableUpdater> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyTableUpdater) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyTableUpdater {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyTableUpdater {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyTableUpdater()";
            const SELECTOR: [u8; 4] = [6u8, 24u8, 54u8, 214u8];
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
    /**Custom error with signature `OperatorCountZero()` and selector `0x40a42054`.
    ```solidity
    error OperatorCountZero();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorCountZero;
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
        impl ::core::convert::From<OperatorCountZero> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorCountZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorCountZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorCountZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorCountZero()";
            const SELECTOR: [u8; 4] = [64u8, 164u8, 32u8, 84u8];
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
    /**Custom error with signature `ReferenceTimestampDoesNotExist()` and selector `0x6568bdb8`.
    ```solidity
    error ReferenceTimestampDoesNotExist();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ReferenceTimestampDoesNotExist;
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
        impl ::core::convert::From<ReferenceTimestampDoesNotExist> for UnderlyingRustTuple<'_> {
            fn from(value: ReferenceTimestampDoesNotExist) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ReferenceTimestampDoesNotExist {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ReferenceTimestampDoesNotExist {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ReferenceTimestampDoesNotExist()";
            const SELECTOR: [u8; 4] = [101u8, 104u8, 189u8, 184u8];
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
    /**Custom error with signature `RootDisabled()` and selector `0x1b14174b`.
    ```solidity
    error RootDisabled();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RootDisabled;
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
        impl ::core::convert::From<RootDisabled> for UnderlyingRustTuple<'_> {
            fn from(value: RootDisabled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RootDisabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RootDisabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RootDisabled()";
            const SELECTOR: [u8; 4] = [27u8, 20u8, 23u8, 75u8];
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
    /**Custom error with signature `SignatureExpired()` and selector `0x0819bdcd`.
    ```solidity
    error SignatureExpired();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureExpired;
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
        impl ::core::convert::From<SignatureExpired> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureExpired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SignatureExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SignatureExpired()";
            const SELECTOR: [u8; 4] = [8u8, 25u8, 189u8, 205u8];
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
    /**Custom error with signature `SignersNotOrdered()` and selector `0xb550c570`.
    ```solidity
    error SignersNotOrdered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignersNotOrdered;
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
        impl ::core::convert::From<SignersNotOrdered> for UnderlyingRustTuple<'_> {
            fn from(value: SignersNotOrdered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignersNotOrdered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SignersNotOrdered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SignersNotOrdered()";
            const SELECTOR: [u8; 4] = [181u8, 80u8, 197u8, 112u8];
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
    /**Custom error with signature `TableUpdateStale()` and selector `0x2f20889f`.
    ```solidity
    error TableUpdateStale();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TableUpdateStale;
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
        impl ::core::convert::From<TableUpdateStale> for UnderlyingRustTuple<'_> {
            fn from(value: TableUpdateStale) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TableUpdateStale {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TableUpdateStale {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TableUpdateStale()";
            const SELECTOR: [u8; 4] = [47u8, 32u8, 136u8, 159u8];
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
    /**Custom error with signature `VerificationFailed()` and selector `0x439cc0cd`.
    ```solidity
    error VerificationFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VerificationFailed;
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
        impl ::core::convert::From<VerificationFailed> for UnderlyingRustTuple<'_> {
            fn from(value: VerificationFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for VerificationFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for VerificationFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "VerificationFailed()";
            const SELECTOR: [u8; 4] = [67u8, 156u8, 192u8, 205u8];
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
    /**Event with signature `MaxStalenessPeriodUpdated((address,uint32),uint32)` and selector `0x28539469fbbc8a5482e60966bf9376f7b9d25b2f0a65a9976f6baa3f0e3788da`.
    ```solidity
    event MaxStalenessPeriodUpdated(OperatorSet operatorSet, uint32 maxStalenessPeriod);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MaxStalenessPeriodUpdated {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub maxStalenessPeriod: u32,
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
        impl alloy_sol_types::SolEvent for MaxStalenessPeriodUpdated {
            type DataTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<32>);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MaxStalenessPeriodUpdated((address,uint32),uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    40u8, 83u8, 148u8, 105u8, 251u8, 188u8, 138u8, 84u8, 130u8, 230u8, 9u8, 102u8,
                    191u8, 147u8, 118u8, 247u8, 185u8, 210u8, 91u8, 47u8, 10u8, 101u8, 169u8,
                    151u8, 111u8, 107u8, 170u8, 63u8, 14u8, 55u8, 136u8, 218u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorSet: data.0,
                    maxStalenessPeriod: data.1,
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.maxStalenessPeriod,
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
        impl alloy_sol_types::private::IntoLogData for MaxStalenessPeriodUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MaxStalenessPeriodUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MaxStalenessPeriodUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OperatorSetOwnerUpdated((address,uint32),address)` and selector `0x806dc367095c0baf953d7144b7c4376261675ee0b4e0da2761e43673051c7375`.
    ```solidity
    event OperatorSetOwnerUpdated(OperatorSet operatorSet, address owner);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSetOwnerUpdated {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OperatorSetOwnerUpdated {
            type DataTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorSetOwnerUpdated((address,uint32),address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    128u8, 109u8, 195u8, 103u8, 9u8, 92u8, 11u8, 175u8, 149u8, 61u8, 113u8, 68u8,
                    183u8, 196u8, 55u8, 98u8, 97u8, 103u8, 94u8, 224u8, 180u8, 224u8, 218u8, 39u8,
                    97u8, 228u8, 54u8, 115u8, 5u8, 28u8, 115u8, 117u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorSet: data.0,
                    owner: data.1,
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
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
        impl alloy_sol_types::private::IntoLogData for OperatorSetOwnerUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSetOwnerUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSetOwnerUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TableUpdated((address,uint32),uint32,(address,uint256[])[])` and selector `0x4f588da9ec57976194a79b5594f8f8782923d93013df2b9ed12fe125805011ef`.
    ```solidity
    event TableUpdated(OperatorSet operatorSet, uint32 referenceTimestamp, IOperatorTableCalculatorTypes.ECDSAOperatorInfo[] operatorInfos);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TableUpdated {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub operatorInfos: alloy::sol_types::private::Vec<
            <IOperatorTableCalculatorTypes::ECDSAOperatorInfo as alloy::sol_types::SolType>::RustType,
        >,
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
        impl alloy_sol_types::SolEvent for TableUpdated {
            type DataTuple<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<IOperatorTableCalculatorTypes::ECDSAOperatorInfo>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "TableUpdated((address,uint32),uint32,(address,uint256[])[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    79u8, 88u8, 141u8, 169u8, 236u8, 87u8, 151u8, 97u8, 148u8, 167u8, 155u8, 85u8,
                    148u8, 248u8, 248u8, 120u8, 41u8, 35u8, 217u8, 48u8, 19u8, 223u8, 43u8, 158u8,
                    209u8, 47u8, 225u8, 37u8, 128u8, 80u8, 17u8, 239u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorSet: data.0,
                    referenceTimestamp: data.1,
                    operatorInfos: data.2,
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceTimestamp,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        IOperatorTableCalculatorTypes::ECDSAOperatorInfo,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.operatorInfos
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
        impl alloy_sol_types::private::IntoLogData for TableUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TableUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TableUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _operatorTableUpdater, string _version);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _operatorTableUpdater: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _version: alloy::sol_types::private::String,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
                    (value._operatorTableUpdater, value._version)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operatorTableUpdater: tuple.0,
                        _version: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
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
                        &self._operatorTableUpdater,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._version,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `calculateCertificateDigest(uint32,bytes32)` and selector `0x18467434`.
    ```solidity
    function calculateCertificateDigest(uint32 referenceTimestamp, bytes32 messageHash) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateCertificateDigestCall {
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub messageHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`calculateCertificateDigest(uint32,bytes32)`](calculateCertificateDigestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateCertificateDigestReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::FixedBytes<32>);
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
            impl ::core::convert::From<calculateCertificateDigestCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateCertificateDigestCall) -> Self {
                    (value.referenceTimestamp, value.messageHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateCertificateDigestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        referenceTimestamp: tuple.0,
                        messageHash: tuple.1,
                    }
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
            impl ::core::convert::From<calculateCertificateDigestReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calculateCertificateDigestReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateCertificateDigestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateCertificateDigestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateCertificateDigest(uint32,bytes32)";
            const SELECTOR: [u8; 4] = [24u8, 70u8, 116u8, 52u8];
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTimestamp),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.messageHash),
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
                        let r: calculateCertificateDigestReturn = r.into();
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
                    let r: calculateCertificateDigestReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `calculateCertificateDigestBytes(uint32,bytes32)` and selector `0x702ca531`.
    ```solidity
    function calculateCertificateDigestBytes(uint32 referenceTimestamp, bytes32 messageHash) external view returns (bytes memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateCertificateDigestBytesCall {
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub messageHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`calculateCertificateDigestBytes(uint32,bytes32)`](calculateCertificateDigestBytesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateCertificateDigestBytesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::FixedBytes<32>);
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
            impl ::core::convert::From<calculateCertificateDigestBytesCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateCertificateDigestBytesCall) -> Self {
                    (value.referenceTimestamp, value.messageHash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateCertificateDigestBytesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        referenceTimestamp: tuple.0,
                        messageHash: tuple.1,
                    }
                }
            }
        }
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
            impl ::core::convert::From<calculateCertificateDigestBytesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calculateCertificateDigestBytesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateCertificateDigestBytesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateCertificateDigestBytesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Bytes;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateCertificateDigestBytes(uint32,bytes32)";
            const SELECTOR: [u8; 4] = [112u8, 44u8, 165u8, 49u8];
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTimestamp),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.messageHash),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: calculateCertificateDigestBytesReturn = r.into();
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
                    let r: calculateCertificateDigestBytesReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `domainSeparator()` and selector `0xf698da25`.
    ```solidity
    function domainSeparator() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`domainSeparator()`](domainSeparatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorReturn {
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
            impl ::core::convert::From<domainSeparatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for domainSeparatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<domainSeparatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for domainSeparatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for domainSeparatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "domainSeparator()";
            const SELECTOR: [u8; 4] = [246u8, 152u8, 218u8, 37u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: domainSeparatorReturn = r.into();
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
                    let r: domainSeparatorReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperatorCount((address,uint32),uint32)` and selector `0x23c2a3cb`.
    ```solidity
    function getOperatorCount(OperatorSet memory operatorSet, uint32 referenceTimestamp) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorCountCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorCount((address,uint32),uint32)`](getOperatorCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorCountReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<32>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<OperatorSet as alloy::sol_types::SolType>::RustType, u32);
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
            impl ::core::convert::From<getOperatorCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorCountCall) -> Self {
                    (value.operatorSet, value.referenceTimestamp)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        referenceTimestamp: tuple.1,
                    }
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
            impl ::core::convert::From<getOperatorCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorCountCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<32>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorCount((address,uint32),uint32)";
            const SELECTOR: [u8; 4] = [35u8, 194u8, 163u8, 203u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceTimestamp,
                    ),
                )
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
                        let r: getOperatorCountReturn = r.into();
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
                    let r: getOperatorCountReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperatorInfo((address,uint32),uint32,uint256)` and selector `0xe49613fc`.
    ```solidity
    function getOperatorInfo(OperatorSet memory operatorSet, uint32 referenceTimestamp, uint256 operatorIndex) external view returns (IOperatorTableCalculatorTypes.ECDSAOperatorInfo memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorInfoCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub operatorIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorInfo((address,uint32),uint32,uint256)`](getOperatorInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorInfoReturn {
        #[allow(missing_docs)]
        pub _0: <IOperatorTableCalculatorTypes::ECDSAOperatorInfo as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                u32,
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
            impl ::core::convert::From<getOperatorInfoCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorInfoCall) -> Self {
                    (
                        value.operatorSet,
                        value.referenceTimestamp,
                        value.operatorIndex,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorInfoCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        referenceTimestamp: tuple.1,
                        operatorIndex: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IOperatorTableCalculatorTypes::ECDSAOperatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IOperatorTableCalculatorTypes::ECDSAOperatorInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorInfoReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorInfoReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorInfoCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IOperatorTableCalculatorTypes::ECDSAOperatorInfo as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IOperatorTableCalculatorTypes::ECDSAOperatorInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorInfo((address,uint32),uint32,uint256)";
            const SELECTOR: [u8; 4] = [228u8, 150u8, 19u8, 252u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceTimestamp,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.operatorIndex,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <IOperatorTableCalculatorTypes::ECDSAOperatorInfo as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getOperatorInfoReturn = r.into();
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
                    let r: getOperatorInfoReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperatorInfos((address,uint32),uint32)` and selector `0x7c85ac4c`.
    ```solidity
    function getOperatorInfos(OperatorSet memory operatorSet, uint32 referenceTimestamp) external view returns (IOperatorTableCalculatorTypes.ECDSAOperatorInfo[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorInfosCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorInfos((address,uint32),uint32)`](getOperatorInfosCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorInfosReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            <IOperatorTableCalculatorTypes::ECDSAOperatorInfo as alloy::sol_types::SolType>::RustType,
        >,
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
            type UnderlyingSolTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<32>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<OperatorSet as alloy::sol_types::SolType>::RustType, u32);
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
            impl ::core::convert::From<getOperatorInfosCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorInfosCall) -> Self {
                    (value.operatorSet, value.referenceTimestamp)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorInfosCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        referenceTimestamp: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IOperatorTableCalculatorTypes::ECDSAOperatorInfo>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IOperatorTableCalculatorTypes::ECDSAOperatorInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorInfosReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorInfosReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorInfosReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorInfosCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<32>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <IOperatorTableCalculatorTypes::ECDSAOperatorInfo as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<IOperatorTableCalculatorTypes::ECDSAOperatorInfo>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorInfos((address,uint32),uint32)";
            const SELECTOR: [u8; 4] = [124u8, 133u8, 172u8, 76u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceTimestamp,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IOperatorTableCalculatorTypes::ECDSAOperatorInfo,
                > as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getOperatorInfosReturn = r.into();
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
                    let r: getOperatorInfosReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperatorSetOwner((address,uint32))` and selector `0x84818920`.
    ```solidity
    function getOperatorSetOwner(OperatorSet memory operatorSet) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetOwnerCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorSetOwner((address,uint32))`](getOperatorSetOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetOwnerReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (<OperatorSet as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getOperatorSetOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetOwnerCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetOwnerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                    }
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
            impl ::core::convert::From<getOperatorSetOwnerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetOwnerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSetOwnerCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSetOwner((address,uint32))";
            const SELECTOR: [u8; 4] = [132u8, 129u8, 137u8, 32u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(
                    &self.operatorSet,
                ),)
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
                        let r: getOperatorSetOwnerReturn = r.into();
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
                    let r: getOperatorSetOwnerReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getTotalStakeWeights((address,uint32),uint32)` and selector `0x7d1d1f5b`.
    ```solidity
    function getTotalStakeWeights(OperatorSet memory operatorSet, uint32 referenceTimestamp) external view returns (uint256[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeWeightsCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getTotalStakeWeights((address,uint32),uint32)`](getTotalStakeWeightsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeWeightsReturn {
        #[allow(missing_docs)]
        pub _0:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            type UnderlyingSolTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<32>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<OperatorSet as alloy::sol_types::SolType>::RustType, u32);
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
            impl ::core::convert::From<getTotalStakeWeightsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeWeightsCall) -> Self {
                    (value.operatorSet, value.referenceTimestamp)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeWeightsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        referenceTimestamp: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getTotalStakeWeightsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeWeightsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeWeightsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeWeightsCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<32>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalStakeWeights((address,uint32),uint32)";
            const SELECTOR: [u8; 4] = [125u8, 29u8, 31u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceTimestamp,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getTotalStakeWeightsReturn = r.into();
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
                    let r: getTotalStakeWeightsReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isReferenceTimestampSet((address,uint32),uint32)` and selector `0xcd83a72b`.
    ```solidity
    function isReferenceTimestampSet(OperatorSet memory operatorSet, uint32 referenceTimestamp) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isReferenceTimestampSetCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isReferenceTimestampSet((address,uint32),uint32)`](isReferenceTimestampSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isReferenceTimestampSetReturn {
        #[allow(missing_docs)]
        pub _0: bool,
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
            type UnderlyingSolTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<32>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<OperatorSet as alloy::sol_types::SolType>::RustType, u32);
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
            impl ::core::convert::From<isReferenceTimestampSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: isReferenceTimestampSetCall) -> Self {
                    (value.operatorSet, value.referenceTimestamp)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isReferenceTimestampSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        referenceTimestamp: tuple.1,
                    }
                }
            }
        }
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
            impl ::core::convert::From<isReferenceTimestampSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isReferenceTimestampSetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isReferenceTimestampSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isReferenceTimestampSetCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<32>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isReferenceTimestampSet((address,uint32),uint32)";
            const SELECTOR: [u8; 4] = [205u8, 131u8, 167u8, 43u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceTimestamp,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: isReferenceTimestampSetReturn = r.into();
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
                    let r: isReferenceTimestampSetReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `latestReferenceTimestamp((address,uint32))` and selector `0x5ddb9b5b`.
    ```solidity
    function latestReferenceTimestamp(OperatorSet memory operatorSet) external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestReferenceTimestampCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`latestReferenceTimestamp((address,uint32))`](latestReferenceTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestReferenceTimestampReturn {
        #[allow(missing_docs)]
        pub _0: u32,
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
            type UnderlyingSolTuple<'a> = (OperatorSet,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (<OperatorSet as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<latestReferenceTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: latestReferenceTimestampCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestReferenceTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<latestReferenceTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: latestReferenceTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestReferenceTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestReferenceTimestampCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestReferenceTimestamp((address,uint32))";
            const SELECTOR: [u8; 4] = [93u8, 219u8, 155u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(
                    &self.operatorSet,
                ),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: latestReferenceTimestampReturn = r.into();
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
                    let r: latestReferenceTimestampReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `maxOperatorTableStaleness((address,uint32))` and selector `0x6141879e`.
    ```solidity
    function maxOperatorTableStaleness(OperatorSet memory operatorSet) external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxOperatorTableStalenessCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`maxOperatorTableStaleness((address,uint32))`](maxOperatorTableStalenessCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct maxOperatorTableStalenessReturn {
        #[allow(missing_docs)]
        pub _0: u32,
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
            type UnderlyingSolTuple<'a> = (OperatorSet,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (<OperatorSet as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<maxOperatorTableStalenessCall> for UnderlyingRustTuple<'_> {
                fn from(value: maxOperatorTableStalenessCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for maxOperatorTableStalenessCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<maxOperatorTableStalenessReturn> for UnderlyingRustTuple<'_> {
                fn from(value: maxOperatorTableStalenessReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for maxOperatorTableStalenessReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for maxOperatorTableStalenessCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "maxOperatorTableStaleness((address,uint32))";
            const SELECTOR: [u8; 4] = [97u8, 65u8, 135u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(
                    &self.operatorSet,
                ),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: maxOperatorTableStalenessReturn = r.into();
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
                    let r: maxOperatorTableStalenessReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `operatorTableUpdater()` and selector `0x68d6e081`.
    ```solidity
    function operatorTableUpdater() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorTableUpdaterCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`operatorTableUpdater()`](operatorTableUpdaterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorTableUpdaterReturn {
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
            impl ::core::convert::From<operatorTableUpdaterCall> for UnderlyingRustTuple<'_> {
                fn from(value: operatorTableUpdaterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorTableUpdaterCall {
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
            impl ::core::convert::From<operatorTableUpdaterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: operatorTableUpdaterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for operatorTableUpdaterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorTableUpdaterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorTableUpdater()";
            const SELECTOR: [u8; 4] = [104u8, 214u8, 224u8, 129u8];
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
                        let r: operatorTableUpdaterReturn = r.into();
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
                    let r: operatorTableUpdaterReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `updateOperatorTable((address,uint32),uint32,(address,uint256[])[],(address,uint32))` and selector `0x56d482f5`.
    ```solidity
    function updateOperatorTable(OperatorSet memory operatorSet, uint32 referenceTimestamp, IOperatorTableCalculatorTypes.ECDSAOperatorInfo[] memory operatorInfos, ICrossChainRegistryTypes.OperatorSetConfig memory operatorSetConfig) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorTableCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub operatorInfos: alloy::sol_types::private::Vec<
            <IOperatorTableCalculatorTypes::ECDSAOperatorInfo as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub operatorSetConfig: <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`updateOperatorTable((address,uint32),uint32,(address,uint256[])[],(address,uint32))`](updateOperatorTableCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorTableReturn {}
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
                OperatorSet,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<IOperatorTableCalculatorTypes::ECDSAOperatorInfo>,
                ICrossChainRegistryTypes::OperatorSetConfig,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                u32,
                alloy::sol_types::private::Vec<
                    <IOperatorTableCalculatorTypes::ECDSAOperatorInfo as alloy::sol_types::SolType>::RustType,
                >,
                <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<updateOperatorTableCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorTableCall) -> Self {
                    (
                        value.operatorSet,
                        value.referenceTimestamp,
                        value.operatorInfos,
                        value.operatorSetConfig,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorTableCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        referenceTimestamp: tuple.1,
                        operatorInfos: tuple.2,
                        operatorSetConfig: tuple.3,
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
            impl ::core::convert::From<updateOperatorTableReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorTableReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorTableReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateOperatorTableReturn {
            fn _tokenize(
                &self,
            ) -> <updateOperatorTableCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorTableCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<IOperatorTableCalculatorTypes::ECDSAOperatorInfo>,
                ICrossChainRegistryTypes::OperatorSetConfig,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorTableReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorTable((address,uint32),uint32,(address,uint256[])[],(address,uint32))";
            const SELECTOR: [u8; 4] = [86u8, 212u8, 130u8, 245u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTimestamp),
                    <alloy::sol_types::sol_data::Array<
                        IOperatorTableCalculatorTypes::ECDSAOperatorInfo,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorInfos),
                    <ICrossChainRegistryTypes::OperatorSetConfig as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetConfig,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateOperatorTableReturn::_tokenize(ret)
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
    /**Function with signature `verifyCertificate((address,uint32),(uint32,bytes32,bytes))` and selector `0x80c7d3f3`.
    ```solidity
    function verifyCertificate(OperatorSet memory operatorSet, IECDSACertificateVerifierTypes.ECDSACertificate memory cert) external view returns (uint256[] memory, address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCertificateCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub cert: <IECDSACertificateVerifierTypes::ECDSACertificate as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`verifyCertificate((address,uint32),(uint32,bytes32,bytes))`](verifyCertificateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCertificateReturn {
        #[allow(missing_docs)]
        pub _0:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            type UnderlyingSolTuple<'a> = (
                OperatorSet,
                IECDSACertificateVerifierTypes::ECDSACertificate,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                <IECDSACertificateVerifierTypes::ECDSACertificate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<verifyCertificateCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCertificateCall) -> Self {
                    (value.operatorSet, value.cert)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCertificateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        cert: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<verifyCertificateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCertificateReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCertificateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        impl verifyCertificateReturn {
            fn _tokenize(
                &self,
            ) -> <verifyCertificateCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyCertificateCall {
            type Parameters<'a> = (
                OperatorSet,
                IECDSACertificateVerifierTypes::ECDSACertificate,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyCertificateReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "verifyCertificate((address,uint32),(uint32,bytes32,bytes))";
            const SELECTOR: [u8; 4] = [128u8, 199u8, 211u8, 243u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <IECDSACertificateVerifierTypes::ECDSACertificate as alloy_sol_types::SolType>::tokenize(
                        &self.cert,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                verifyCertificateReturn::_tokenize(ret)
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
    /**Function with signature `verifyCertificateNominal((address,uint32),(uint32,bytes32,bytes),uint256[])` and selector `0xbe86e0b2`.
    ```solidity
    function verifyCertificateNominal(OperatorSet memory operatorSet, IECDSACertificateVerifierTypes.ECDSACertificate memory cert, uint256[] memory totalStakeNominalThresholds) external view returns (bool, address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCertificateNominalCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub cert: <IECDSACertificateVerifierTypes::ECDSACertificate as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub totalStakeNominalThresholds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`verifyCertificateNominal((address,uint32),(uint32,bytes32,bytes),uint256[])`](verifyCertificateNominalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCertificateNominalReturn {
        #[allow(missing_docs)]
        pub _0: bool,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            type UnderlyingSolTuple<'a> = (
                OperatorSet,
                IECDSACertificateVerifierTypes::ECDSACertificate,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                <IECDSACertificateVerifierTypes::ECDSACertificate as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<verifyCertificateNominalCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCertificateNominalCall) -> Self {
                    (
                        value.operatorSet,
                        value.cert,
                        value.totalStakeNominalThresholds,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCertificateNominalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        cert: tuple.1,
                        totalStakeNominalThresholds: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                bool,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<verifyCertificateNominalReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCertificateNominalReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCertificateNominalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        impl verifyCertificateNominalReturn {
            fn _tokenize(
                &self,
            ) -> <verifyCertificateNominalCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyCertificateNominalCall {
            type Parameters<'a> = (
                OperatorSet,
                IECDSACertificateVerifierTypes::ECDSACertificate,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyCertificateNominalReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "verifyCertificateNominal((address,uint32),(uint32,bytes32,bytes),uint256[])";
            const SELECTOR: [u8; 4] = [190u8, 134u8, 224u8, 178u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <IECDSACertificateVerifierTypes::ECDSACertificate as alloy_sol_types::SolType>::tokenize(
                        &self.cert,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.totalStakeNominalThresholds,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                verifyCertificateNominalReturn::_tokenize(ret)
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
    /**Function with signature `verifyCertificateProportion((address,uint32),(uint32,bytes32,bytes),uint16[])` and selector `0xc0da2420`.
    ```solidity
    function verifyCertificateProportion(OperatorSet memory operatorSet, IECDSACertificateVerifierTypes.ECDSACertificate memory cert, uint16[] memory totalStakeProportionThresholds) external view returns (bool, address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCertificateProportionCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub cert: <IECDSACertificateVerifierTypes::ECDSACertificate as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub totalStakeProportionThresholds: alloy::sol_types::private::Vec<u16>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`verifyCertificateProportion((address,uint32),(uint32,bytes32,bytes),uint16[])`](verifyCertificateProportionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCertificateProportionReturn {
        #[allow(missing_docs)]
        pub _0: bool,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            type UnderlyingSolTuple<'a> = (
                OperatorSet,
                IECDSACertificateVerifierTypes::ECDSACertificate,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<16>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                <IECDSACertificateVerifierTypes::ECDSACertificate as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<u16>,
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
            impl ::core::convert::From<verifyCertificateProportionCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCertificateProportionCall) -> Self {
                    (
                        value.operatorSet,
                        value.cert,
                        value.totalStakeProportionThresholds,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCertificateProportionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        cert: tuple.1,
                        totalStakeProportionThresholds: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                bool,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<verifyCertificateProportionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCertificateProportionReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCertificateProportionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        impl verifyCertificateProportionReturn {
            fn _tokenize(
                &self,
            ) -> <verifyCertificateProportionCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyCertificateProportionCall {
            type Parameters<'a> = (
                OperatorSet,
                IECDSACertificateVerifierTypes::ECDSACertificate,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<16>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyCertificateProportionReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "verifyCertificateProportion((address,uint32),(uint32,bytes32,bytes),uint16[])";
            const SELECTOR: [u8; 4] = [192u8, 218u8, 36u8, 32u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <IECDSACertificateVerifierTypes::ECDSACertificate as alloy_sol_types::SolType>::tokenize(
                        &self.cert,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<16>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.totalStakeProportionThresholds,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                verifyCertificateProportionReturn::_tokenize(ret)
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
    ///Container for all the [`ECDSACertificateVerifier`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum ECDSACertificateVerifierCalls {
        #[allow(missing_docs)]
        calculateCertificateDigest(calculateCertificateDigestCall),
        #[allow(missing_docs)]
        calculateCertificateDigestBytes(calculateCertificateDigestBytesCall),
        #[allow(missing_docs)]
        domainSeparator(domainSeparatorCall),
        #[allow(missing_docs)]
        getOperatorCount(getOperatorCountCall),
        #[allow(missing_docs)]
        getOperatorInfo(getOperatorInfoCall),
        #[allow(missing_docs)]
        getOperatorInfos(getOperatorInfosCall),
        #[allow(missing_docs)]
        getOperatorSetOwner(getOperatorSetOwnerCall),
        #[allow(missing_docs)]
        getTotalStakeWeights(getTotalStakeWeightsCall),
        #[allow(missing_docs)]
        isReferenceTimestampSet(isReferenceTimestampSetCall),
        #[allow(missing_docs)]
        latestReferenceTimestamp(latestReferenceTimestampCall),
        #[allow(missing_docs)]
        maxOperatorTableStaleness(maxOperatorTableStalenessCall),
        #[allow(missing_docs)]
        operatorTableUpdater(operatorTableUpdaterCall),
        #[allow(missing_docs)]
        updateOperatorTable(updateOperatorTableCall),
        #[allow(missing_docs)]
        verifyCertificate(verifyCertificateCall),
        #[allow(missing_docs)]
        verifyCertificateNominal(verifyCertificateNominalCall),
        #[allow(missing_docs)]
        verifyCertificateProportion(verifyCertificateProportionCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    #[automatically_derived]
    impl ECDSACertificateVerifierCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [24u8, 70u8, 116u8, 52u8],
            [35u8, 194u8, 163u8, 203u8],
            [84u8, 253u8, 77u8, 80u8],
            [86u8, 212u8, 130u8, 245u8],
            [93u8, 219u8, 155u8, 91u8],
            [97u8, 65u8, 135u8, 158u8],
            [104u8, 214u8, 224u8, 129u8],
            [112u8, 44u8, 165u8, 49u8],
            [124u8, 133u8, 172u8, 76u8],
            [125u8, 29u8, 31u8, 91u8],
            [128u8, 199u8, 211u8, 243u8],
            [132u8, 129u8, 137u8, 32u8],
            [190u8, 134u8, 224u8, 178u8],
            [192u8, 218u8, 36u8, 32u8],
            [205u8, 131u8, 167u8, 43u8],
            [228u8, 150u8, 19u8, 252u8],
            [246u8, 152u8, 218u8, 37u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ECDSACertificateVerifierCalls {
        const NAME: &'static str = "ECDSACertificateVerifierCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 17usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::calculateCertificateDigest(_) => {
                    <calculateCertificateDigestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateCertificateDigestBytes(_) => {
                    <calculateCertificateDigestBytesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::domainSeparator(_) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorCount(_) => {
                    <getOperatorCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorInfo(_) => {
                    <getOperatorInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorInfos(_) => {
                    <getOperatorInfosCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSetOwner(_) => {
                    <getOperatorSetOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeWeights(_) => {
                    <getTotalStakeWeightsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isReferenceTimestampSet(_) => {
                    <isReferenceTimestampSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestReferenceTimestamp(_) => {
                    <latestReferenceTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::maxOperatorTableStaleness(_) => {
                    <maxOperatorTableStalenessCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorTableUpdater(_) => {
                    <operatorTableUpdaterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorTable(_) => {
                    <updateOperatorTableCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyCertificate(_) => {
                    <verifyCertificateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyCertificateNominal(_) => {
                    <verifyCertificateNominalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyCertificateProportion(_) => {
                    <verifyCertificateProportionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::version(_) => <versionCall as alloy_sol_types::SolCall>::SELECTOR,
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
            static DECODE_SHIMS: &[fn(
                &[u8],
            )
                -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>] = &[
                {
                    fn calculateCertificateDigest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <calculateCertificateDigestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ECDSACertificateVerifierCalls::calculateCertificateDigest,
                            )
                    }
                    calculateCertificateDigest
                },
                {
                    fn getOperatorCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <getOperatorCountCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierCalls::getOperatorCount)
                    }
                    getOperatorCount
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierCalls::version)
                    }
                    version
                },
                {
                    fn updateOperatorTable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierCalls::updateOperatorTable)
                    }
                    updateOperatorTable
                },
                {
                    fn latestReferenceTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <latestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(ECDSACertificateVerifierCalls::latestReferenceTimestamp)
                    }
                    latestReferenceTimestamp
                },
                {
                    fn maxOperatorTableStaleness(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <maxOperatorTableStalenessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(ECDSACertificateVerifierCalls::maxOperatorTableStaleness)
                    }
                    maxOperatorTableStaleness
                },
                {
                    fn operatorTableUpdater(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <operatorTableUpdaterCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierCalls::operatorTableUpdater)
                    }
                    operatorTableUpdater
                },
                {
                    fn calculateCertificateDigestBytes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <calculateCertificateDigestBytesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ECDSACertificateVerifierCalls::calculateCertificateDigestBytes,
                            )
                    }
                    calculateCertificateDigestBytes
                },
                {
                    fn getOperatorInfos(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <getOperatorInfosCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierCalls::getOperatorInfos)
                    }
                    getOperatorInfos
                },
                {
                    fn getTotalStakeWeights(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <getTotalStakeWeightsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierCalls::getTotalStakeWeights)
                    }
                    getTotalStakeWeights
                },
                {
                    fn verifyCertificate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <verifyCertificateCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierCalls::verifyCertificate)
                    }
                    verifyCertificate
                },
                {
                    fn getOperatorSetOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <getOperatorSetOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierCalls::getOperatorSetOwner)
                    }
                    getOperatorSetOwner
                },
                {
                    fn verifyCertificateNominal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <verifyCertificateNominalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(ECDSACertificateVerifierCalls::verifyCertificateNominal)
                    }
                    verifyCertificateNominal
                },
                {
                    fn verifyCertificateProportion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <verifyCertificateProportionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ECDSACertificateVerifierCalls::verifyCertificateProportion,
                            )
                    }
                    verifyCertificateProportion
                },
                {
                    fn isReferenceTimestampSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <isReferenceTimestampSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(ECDSACertificateVerifierCalls::isReferenceTimestampSet)
                    }
                    isReferenceTimestampSet
                },
                {
                    fn getOperatorInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <getOperatorInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierCalls::getOperatorInfo)
                    }
                    getOperatorInfo
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierCalls::domainSeparator)
                    }
                    domainSeparator
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
            ) -> alloy_sol_types::Result<
                ECDSACertificateVerifierCalls,
            >] = &[
                {
                    fn calculateCertificateDigest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <calculateCertificateDigestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ECDSACertificateVerifierCalls::calculateCertificateDigest,
                            )
                    }
                    calculateCertificateDigest
                },
                {
                    fn getOperatorCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <getOperatorCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierCalls::getOperatorCount)
                    }
                    getOperatorCount
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(ECDSACertificateVerifierCalls::version)
                    }
                    version
                },
                {
                    fn updateOperatorTable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ECDSACertificateVerifierCalls::updateOperatorTable)
                    }
                    updateOperatorTable
                },
                {
                    fn latestReferenceTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <latestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ECDSACertificateVerifierCalls::latestReferenceTimestamp)
                    }
                    latestReferenceTimestamp
                },
                {
                    fn maxOperatorTableStaleness(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <maxOperatorTableStalenessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ECDSACertificateVerifierCalls::maxOperatorTableStaleness,
                            )
                    }
                    maxOperatorTableStaleness
                },
                {
                    fn operatorTableUpdater(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <operatorTableUpdaterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ECDSACertificateVerifierCalls::operatorTableUpdater)
                    }
                    operatorTableUpdater
                },
                {
                    fn calculateCertificateDigestBytes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <calculateCertificateDigestBytesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ECDSACertificateVerifierCalls::calculateCertificateDigestBytes,
                            )
                    }
                    calculateCertificateDigestBytes
                },
                {
                    fn getOperatorInfos(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <getOperatorInfosCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierCalls::getOperatorInfos)
                    }
                    getOperatorInfos
                },
                {
                    fn getTotalStakeWeights(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <getTotalStakeWeightsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ECDSACertificateVerifierCalls::getTotalStakeWeights)
                    }
                    getTotalStakeWeights
                },
                {
                    fn verifyCertificate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <verifyCertificateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ECDSACertificateVerifierCalls::verifyCertificate)
                    }
                    verifyCertificate
                },
                {
                    fn getOperatorSetOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <getOperatorSetOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ECDSACertificateVerifierCalls::getOperatorSetOwner)
                    }
                    getOperatorSetOwner
                },
                {
                    fn verifyCertificateNominal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <verifyCertificateNominalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ECDSACertificateVerifierCalls::verifyCertificateNominal)
                    }
                    verifyCertificateNominal
                },
                {
                    fn verifyCertificateProportion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <verifyCertificateProportionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ECDSACertificateVerifierCalls::verifyCertificateProportion,
                            )
                    }
                    verifyCertificateProportion
                },
                {
                    fn isReferenceTimestampSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <isReferenceTimestampSetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ECDSACertificateVerifierCalls::isReferenceTimestampSet)
                    }
                    isReferenceTimestampSet
                },
                {
                    fn getOperatorInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <getOperatorInfoCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierCalls::getOperatorInfo)
                    }
                    getOperatorInfo
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierCalls>
                    {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierCalls::domainSeparator)
                    }
                    domainSeparator
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
                Self::calculateCertificateDigest(inner) => {
                    <calculateCertificateDigestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateCertificateDigestBytes(inner) => {
                    <calculateCertificateDigestBytesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorCount(inner) => {
                    <getOperatorCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorInfo(inner) => {
                    <getOperatorInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorInfos(inner) => {
                    <getOperatorInfosCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSetOwner(inner) => {
                    <getOperatorSetOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeWeights(inner) => {
                    <getTotalStakeWeightsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isReferenceTimestampSet(inner) => {
                    <isReferenceTimestampSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::latestReferenceTimestamp(inner) => {
                    <latestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::maxOperatorTableStaleness(inner) => {
                    <maxOperatorTableStalenessCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorTableUpdater(inner) => {
                    <operatorTableUpdaterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorTable(inner) => {
                    <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyCertificate(inner) => {
                    <verifyCertificateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyCertificateNominal(inner) => {
                    <verifyCertificateNominalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyCertificateProportion(inner) => {
                    <verifyCertificateProportionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::calculateCertificateDigest(inner) => {
                    <calculateCertificateDigestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateCertificateDigestBytes(inner) => {
                    <calculateCertificateDigestBytesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorCount(inner) => {
                    <getOperatorCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorInfo(inner) => {
                    <getOperatorInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorInfos(inner) => {
                    <getOperatorInfosCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorSetOwner(inner) => {
                    <getOperatorSetOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeWeights(inner) => {
                    <getTotalStakeWeightsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isReferenceTimestampSet(inner) => {
                    <isReferenceTimestampSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::latestReferenceTimestamp(inner) => {
                    <latestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::maxOperatorTableStaleness(inner) => {
                    <maxOperatorTableStalenessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorTableUpdater(inner) => {
                    <operatorTableUpdaterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorTable(inner) => {
                    <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyCertificate(inner) => {
                    <verifyCertificateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyCertificateNominal(inner) => {
                    <verifyCertificateNominalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyCertificateProportion(inner) => {
                    <verifyCertificateProportionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`ECDSACertificateVerifier`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum ECDSACertificateVerifierErrors {
        #[allow(missing_docs)]
        ArrayLengthMismatch(ArrayLengthMismatch),
        #[allow(missing_docs)]
        CertificateStale(CertificateStale),
        #[allow(missing_docs)]
        IndexOutOfBounds(IndexOutOfBounds),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        InvalidSignature(InvalidSignature),
        #[allow(missing_docs)]
        InvalidSignatureLength(InvalidSignatureLength),
        #[allow(missing_docs)]
        OnlyTableUpdater(OnlyTableUpdater),
        #[allow(missing_docs)]
        OperatorCountZero(OperatorCountZero),
        #[allow(missing_docs)]
        ReferenceTimestampDoesNotExist(ReferenceTimestampDoesNotExist),
        #[allow(missing_docs)]
        RootDisabled(RootDisabled),
        #[allow(missing_docs)]
        SignatureExpired(SignatureExpired),
        #[allow(missing_docs)]
        SignersNotOrdered(SignersNotOrdered),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
        #[allow(missing_docs)]
        TableUpdateStale(TableUpdateStale),
        #[allow(missing_docs)]
        VerificationFailed(VerificationFailed),
    }
    #[automatically_derived]
    impl ECDSACertificateVerifierErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 24u8, 54u8, 214u8],
            [8u8, 25u8, 189u8, 205u8],
            [27u8, 20u8, 23u8, 75u8],
            [47u8, 32u8, 136u8, 159u8],
            [48u8, 90u8, 39u8, 169u8],
            [64u8, 164u8, 32u8, 84u8],
            [67u8, 156u8, 192u8, 205u8],
            [75u8, 230u8, 50u8, 27u8],
            [78u8, 35u8, 208u8, 53u8],
            [101u8, 104u8, 189u8, 184u8],
            [139u8, 170u8, 87u8, 159u8],
            [162u8, 74u8, 19u8, 166u8],
            [179u8, 81u8, 43u8, 12u8],
            [181u8, 80u8, 197u8, 112u8],
            [200u8, 31u8, 154u8, 214u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ECDSACertificateVerifierErrors {
        const NAME: &'static str = "ECDSACertificateVerifierErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 15usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ArrayLengthMismatch(_) => {
                    <ArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CertificateStale(_) => {
                    <CertificateStale as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IndexOutOfBounds(_) => {
                    <IndexOutOfBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignatureLength(_) => {
                    <InvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyTableUpdater(_) => {
                    <OnlyTableUpdater as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorCountZero(_) => {
                    <OperatorCountZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReferenceTimestampDoesNotExist(_) => {
                    <ReferenceTimestampDoesNotExist as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RootDisabled(_) => <RootDisabled as alloy_sol_types::SolError>::SELECTOR,
                Self::SignatureExpired(_) => {
                    <SignatureExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SignersNotOrdered(_) => {
                    <SignersNotOrdered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringTooLong(_) => <StringTooLong as alloy_sol_types::SolError>::SELECTOR,
                Self::TableUpdateStale(_) => {
                    <TableUpdateStale as alloy_sol_types::SolError>::SELECTOR
                }
                Self::VerificationFailed(_) => {
                    <VerificationFailed as alloy_sol_types::SolError>::SELECTOR
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
            static DECODE_SHIMS: &[fn(
                &[u8],
            ) -> alloy_sol_types::Result<
                ECDSACertificateVerifierErrors,
            >] = &[
                {
                    fn OnlyTableUpdater(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <OnlyTableUpdater as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::OnlyTableUpdater)
                    }
                    OnlyTableUpdater
                },
                {
                    fn SignatureExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn RootDisabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <RootDisabled as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::RootDisabled)
                    }
                    RootDisabled
                },
                {
                    fn TableUpdateStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <TableUpdateStale as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::TableUpdateStale)
                    }
                    TableUpdateStale
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn OperatorCountZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <OperatorCountZero as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::OperatorCountZero)
                    }
                    OperatorCountZero
                },
                {
                    fn VerificationFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <VerificationFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::VerificationFailed)
                    }
                    VerificationFailed
                },
                {
                    fn InvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <InvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::InvalidSignatureLength)
                    }
                    InvalidSignatureLength
                },
                {
                    fn IndexOutOfBounds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <IndexOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::IndexOutOfBounds)
                    }
                    IndexOutOfBounds
                },
                {
                    fn ReferenceTimestampDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <ReferenceTimestampDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ECDSACertificateVerifierErrors::ReferenceTimestampDoesNotExist,
                            )
                    }
                    ReferenceTimestampDoesNotExist
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn ArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::ArrayLengthMismatch)
                    }
                    ArrayLengthMismatch
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn SignersNotOrdered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <SignersNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::SignersNotOrdered)
                    }
                    SignersNotOrdered
                },
                {
                    fn CertificateStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <CertificateStale as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ECDSACertificateVerifierErrors::CertificateStale)
                    }
                    CertificateStale
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
            ) -> alloy_sol_types::Result<
                ECDSACertificateVerifierErrors,
            >] = &[
                {
                    fn OnlyTableUpdater(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <OnlyTableUpdater as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierErrors::OnlyTableUpdater)
                    }
                    OnlyTableUpdater
                },
                {
                    fn SignatureExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn RootDisabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <RootDisabled as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(ECDSACertificateVerifierErrors::RootDisabled)
                    }
                    RootDisabled
                },
                {
                    fn TableUpdateStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <TableUpdateStale as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierErrors::TableUpdateStale)
                    }
                    TableUpdateStale
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(ECDSACertificateVerifierErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn OperatorCountZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <OperatorCountZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierErrors::OperatorCountZero)
                    }
                    OperatorCountZero
                },
                {
                    fn VerificationFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <VerificationFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierErrors::VerificationFailed)
                    }
                    VerificationFailed
                },
                {
                    fn InvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <InvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ECDSACertificateVerifierErrors::InvalidSignatureLength)
                    }
                    InvalidSignatureLength
                },
                {
                    fn IndexOutOfBounds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <IndexOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierErrors::IndexOutOfBounds)
                    }
                    IndexOutOfBounds
                },
                {
                    fn ReferenceTimestampDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <ReferenceTimestampDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ECDSACertificateVerifierErrors::ReferenceTimestampDoesNotExist,
                            )
                    }
                    ReferenceTimestampDoesNotExist
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn ArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierErrors::ArrayLengthMismatch)
                    }
                    ArrayLengthMismatch
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn SignersNotOrdered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <SignersNotOrdered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierErrors::SignersNotOrdered)
                    }
                    SignersNotOrdered
                },
                {
                    fn CertificateStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ECDSACertificateVerifierErrors>
                    {
                        <CertificateStale as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ECDSACertificateVerifierErrors::CertificateStale)
                    }
                    CertificateStale
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
                Self::ArrayLengthMismatch(inner) => {
                    <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::CertificateStale(inner) => {
                    <CertificateStale as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::IndexOutOfBounds(inner) => {
                    <IndexOutOfBounds as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidSignatureLength(inner) => {
                    <InvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyTableUpdater(inner) => {
                    <OnlyTableUpdater as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorCountZero(inner) => {
                    <OperatorCountZero as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ReferenceTimestampDoesNotExist(inner) => {
                    <ReferenceTimestampDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RootDisabled(inner) => {
                    <RootDisabled as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SignersNotOrdered(inner) => {
                    <SignersNotOrdered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::TableUpdateStale(inner) => {
                    <TableUpdateStale as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::VerificationFailed(inner) => {
                    <VerificationFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ArrayLengthMismatch(inner) => {
                    <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::CertificateStale(inner) => {
                    <CertificateStale as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::IndexOutOfBounds(inner) => {
                    <IndexOutOfBounds as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidSignatureLength(inner) => {
                    <InvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OnlyTableUpdater(inner) => {
                    <OnlyTableUpdater as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorCountZero(inner) => {
                    <OperatorCountZero as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ReferenceTimestampDoesNotExist(inner) => {
                    <ReferenceTimestampDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::RootDisabled(inner) => {
                    <RootDisabled as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::SignersNotOrdered(inner) => {
                    <SignersNotOrdered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::TableUpdateStale(inner) => {
                    <TableUpdateStale as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::VerificationFailed(inner) => {
                    <VerificationFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`ECDSACertificateVerifier`](self) events.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum ECDSACertificateVerifierEvents {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        MaxStalenessPeriodUpdated(MaxStalenessPeriodUpdated),
        #[allow(missing_docs)]
        OperatorSetOwnerUpdated(OperatorSetOwnerUpdated),
        #[allow(missing_docs)]
        TableUpdated(TableUpdated),
    }
    #[automatically_derived]
    impl ECDSACertificateVerifierEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                40u8, 83u8, 148u8, 105u8, 251u8, 188u8, 138u8, 84u8, 130u8, 230u8, 9u8, 102u8,
                191u8, 147u8, 118u8, 247u8, 185u8, 210u8, 91u8, 47u8, 10u8, 101u8, 169u8, 151u8,
                111u8, 107u8, 170u8, 63u8, 14u8, 55u8, 136u8, 218u8,
            ],
            [
                79u8, 88u8, 141u8, 169u8, 236u8, 87u8, 151u8, 97u8, 148u8, 167u8, 155u8, 85u8,
                148u8, 248u8, 248u8, 120u8, 41u8, 35u8, 217u8, 48u8, 19u8, 223u8, 43u8, 158u8,
                209u8, 47u8, 225u8, 37u8, 128u8, 80u8, 17u8, 239u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                128u8, 109u8, 195u8, 103u8, 9u8, 92u8, 11u8, 175u8, 149u8, 61u8, 113u8, 68u8,
                183u8, 196u8, 55u8, 98u8, 97u8, 103u8, 94u8, 224u8, 180u8, 224u8, 218u8, 39u8,
                97u8, 228u8, 54u8, 115u8, 5u8, 28u8, 115u8, 117u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ECDSACertificateVerifierEvents {
        const NAME: &'static str = "ECDSACertificateVerifierEvents";
        const COUNT: usize = 4usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Initialized)
                }
                Some(<MaxStalenessPeriodUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MaxStalenessPeriodUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::MaxStalenessPeriodUpdated)
                }
                Some(<OperatorSetOwnerUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSetOwnerUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::OperatorSetOwnerUpdated)
                }
                Some(<TableUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TableUpdated as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::TableUpdated)
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
    impl alloy_sol_types::private::IntoLogData for ECDSACertificateVerifierEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MaxStalenessPeriodUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetOwnerUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TableUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MaxStalenessPeriodUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetOwnerUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TableUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ECDSACertificateVerifier`](self) contract instance.

    See the [wrapper's documentation](`ECDSACertificateVerifierInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ECDSACertificateVerifierInstance<P, N> {
        ECDSACertificateVerifierInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
        provider: P,
        _operatorTableUpdater: alloy::sol_types::private::Address,
        _version: alloy::sol_types::private::String,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ECDSACertificateVerifierInstance<P, N>>,
    > {
        ECDSACertificateVerifierInstance::<P, N>::deploy(provider, _operatorTableUpdater, _version)
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
        _operatorTableUpdater: alloy::sol_types::private::Address,
        _version: alloy::sol_types::private::String,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        ECDSACertificateVerifierInstance::<P, N>::deploy_builder(
            provider,
            _operatorTableUpdater,
            _version,
        )
    }
    /**A [`ECDSACertificateVerifier`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ECDSACertificateVerifier`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ECDSACertificateVerifierInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ECDSACertificateVerifierInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ECDSACertificateVerifierInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        ECDSACertificateVerifierInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`ECDSACertificateVerifier`](self) contract instance.

        See the [wrapper's documentation](`ECDSACertificateVerifierInstance`) for more details.*/
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
            _operatorTableUpdater: alloy::sol_types::private::Address,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::Result<ECDSACertificateVerifierInstance<P, N>> {
            let call_builder = Self::deploy_builder(provider, _operatorTableUpdater, _version);
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
            _operatorTableUpdater: alloy::sol_types::private::Address,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _operatorTableUpdater,
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
    impl<P: ::core::clone::Clone, N> ECDSACertificateVerifierInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ECDSACertificateVerifierInstance<P, N> {
            ECDSACertificateVerifierInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        ECDSACertificateVerifierInstance<P, N>
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
        ///Creates a new call builder for the [`calculateCertificateDigest`] function.
        pub fn calculateCertificateDigest(
            &self,
            referenceTimestamp: u32,
            messageHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, calculateCertificateDigestCall, N> {
            self.call_builder(&calculateCertificateDigestCall {
                referenceTimestamp,
                messageHash,
            })
        }
        ///Creates a new call builder for the [`calculateCertificateDigestBytes`] function.
        pub fn calculateCertificateDigestBytes(
            &self,
            referenceTimestamp: u32,
            messageHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, calculateCertificateDigestBytesCall, N> {
            self.call_builder(&calculateCertificateDigestBytesCall {
                referenceTimestamp,
                messageHash,
            })
        }
        ///Creates a new call builder for the [`domainSeparator`] function.
        pub fn domainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, domainSeparatorCall, N> {
            self.call_builder(&domainSeparatorCall)
        }
        ///Creates a new call builder for the [`getOperatorCount`] function.
        pub fn getOperatorCount(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            referenceTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorCountCall, N> {
            self.call_builder(&getOperatorCountCall {
                operatorSet,
                referenceTimestamp,
            })
        }
        ///Creates a new call builder for the [`getOperatorInfo`] function.
        pub fn getOperatorInfo(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            referenceTimestamp: u32,
            operatorIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorInfoCall, N> {
            self.call_builder(&getOperatorInfoCall {
                operatorSet,
                referenceTimestamp,
                operatorIndex,
            })
        }
        ///Creates a new call builder for the [`getOperatorInfos`] function.
        pub fn getOperatorInfos(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            referenceTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorInfosCall, N> {
            self.call_builder(&getOperatorInfosCall {
                operatorSet,
                referenceTimestamp,
            })
        }
        ///Creates a new call builder for the [`getOperatorSetOwner`] function.
        pub fn getOperatorSetOwner(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorSetOwnerCall, N> {
            self.call_builder(&getOperatorSetOwnerCall { operatorSet })
        }
        ///Creates a new call builder for the [`getTotalStakeWeights`] function.
        pub fn getTotalStakeWeights(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            referenceTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getTotalStakeWeightsCall, N> {
            self.call_builder(&getTotalStakeWeightsCall {
                operatorSet,
                referenceTimestamp,
            })
        }
        ///Creates a new call builder for the [`isReferenceTimestampSet`] function.
        pub fn isReferenceTimestampSet(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            referenceTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<&P, isReferenceTimestampSetCall, N> {
            self.call_builder(&isReferenceTimestampSetCall {
                operatorSet,
                referenceTimestamp,
            })
        }
        ///Creates a new call builder for the [`latestReferenceTimestamp`] function.
        pub fn latestReferenceTimestamp(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, latestReferenceTimestampCall, N> {
            self.call_builder(&latestReferenceTimestampCall { operatorSet })
        }
        ///Creates a new call builder for the [`maxOperatorTableStaleness`] function.
        pub fn maxOperatorTableStaleness(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, maxOperatorTableStalenessCall, N> {
            self.call_builder(&maxOperatorTableStalenessCall { operatorSet })
        }
        ///Creates a new call builder for the [`operatorTableUpdater`] function.
        pub fn operatorTableUpdater(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, operatorTableUpdaterCall, N> {
            self.call_builder(&operatorTableUpdaterCall)
        }
        ///Creates a new call builder for the [`updateOperatorTable`] function.
        pub fn updateOperatorTable(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            referenceTimestamp: u32,
            operatorInfos: alloy::sol_types::private::Vec<
                <IOperatorTableCalculatorTypes::ECDSAOperatorInfo as alloy::sol_types::SolType>::RustType,
            >,
            operatorSetConfig: <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, updateOperatorTableCall, N> {
            self.call_builder(&updateOperatorTableCall {
                operatorSet,
                referenceTimestamp,
                operatorInfos,
                operatorSetConfig,
            })
        }
        ///Creates a new call builder for the [`verifyCertificate`] function.
        pub fn verifyCertificate(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            cert: <IECDSACertificateVerifierTypes::ECDSACertificate as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, verifyCertificateCall, N> {
            self.call_builder(&verifyCertificateCall { operatorSet, cert })
        }
        ///Creates a new call builder for the [`verifyCertificateNominal`] function.
        pub fn verifyCertificateNominal(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            cert: <IECDSACertificateVerifierTypes::ECDSACertificate as alloy::sol_types::SolType>::RustType,
            totalStakeNominalThresholds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, verifyCertificateNominalCall, N> {
            self.call_builder(&verifyCertificateNominalCall {
                operatorSet,
                cert,
                totalStakeNominalThresholds,
            })
        }
        ///Creates a new call builder for the [`verifyCertificateProportion`] function.
        pub fn verifyCertificateProportion(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            cert: <IECDSACertificateVerifierTypes::ECDSACertificate as alloy::sol_types::SolType>::RustType,
            totalStakeProportionThresholds: alloy::sol_types::private::Vec<u16>,
        ) -> alloy_contract::SolCallBuilder<&P, verifyCertificateProportionCall, N> {
            self.call_builder(&verifyCertificateProportionCall {
                operatorSet,
                cert,
                totalStakeProportionThresholds,
            })
        }
        ///Creates a new call builder for the [`version`] function.
        pub fn version(&self) -> alloy_contract::SolCallBuilder<&P, versionCall, N> {
            self.call_builder(&versionCall)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        ECDSACertificateVerifierInstance<P, N>
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
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`MaxStalenessPeriodUpdated`] event.
        pub fn MaxStalenessPeriodUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, MaxStalenessPeriodUpdated, N> {
            self.event_filter::<MaxStalenessPeriodUpdated>()
        }
        ///Creates a new event filter for the [`OperatorSetOwnerUpdated`] event.
        pub fn OperatorSetOwnerUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, OperatorSetOwnerUpdated, N> {
            self.event_filter::<OperatorSetOwnerUpdated>()
        }
        ///Creates a new event filter for the [`TableUpdated`] event.
        pub fn TableUpdated_filter(&self) -> alloy_contract::Event<&P, TableUpdated, N> {
            self.event_filter::<TableUpdated>()
        }
    }
}
