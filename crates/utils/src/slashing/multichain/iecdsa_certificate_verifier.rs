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

interface IECDSACertificateVerifier {
    struct OperatorSet {
        address avs;
        uint32 id;
    }

    error ArrayLengthMismatch();
    error CertificateStale();
    error IndexOutOfBounds();
    error InvalidSignatureLength();
    error OnlyTableUpdater();
    error OperatorCountZero();
    error ReferenceTimestampDoesNotExist();
    error RootDisabled();
    error SignersNotOrdered();
    error TableUpdateStale();
    error VerificationFailed();

    event MaxStalenessPeriodUpdated(OperatorSet operatorSet, uint32 maxStalenessPeriod);
    event OperatorSetOwnerUpdated(OperatorSet operatorSet, address owner);
    event TableUpdated(OperatorSet operatorSet, uint32 referenceTimestamp, IOperatorTableCalculatorTypes.ECDSAOperatorInfo[] operatorInfos);

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
    function updateOperatorTable(OperatorSet memory operatorSet, uint32 referenceTimestamp, IOperatorTableCalculatorTypes.ECDSAOperatorInfo[] memory operatorInfos, ICrossChainRegistryTypes.OperatorSetConfig memory operatorSetConfig) external;
    function verifyCertificate(OperatorSet memory operatorSet, IECDSACertificateVerifierTypes.ECDSACertificate memory cert) external view returns (uint256[] memory totalSignedStakeWeights, address[] memory signers);
    function verifyCertificateNominal(OperatorSet memory operatorSet, IECDSACertificateVerifierTypes.ECDSACertificate memory cert, uint256[] memory totalStakeNominalThresholds) external view returns (bool, address[] memory signers);
    function verifyCertificateProportion(OperatorSet memory operatorSet, IECDSACertificateVerifierTypes.ECDSACertificate memory cert, uint16[] memory totalStakeProportionThresholds) external view returns (bool, address[] memory signers);
}
```

...which was generated by the following JSON ABI:
```json
[
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
        "name": "totalSignedStakeWeights",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "signers",
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
        "name": "signers",
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
        "name": "signers",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
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
    "name": "SignersNotOrdered",
    "inputs": []
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
pub mod IECDSACertificateVerifier {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"",
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
    function verifyCertificate(OperatorSet memory operatorSet, IECDSACertificateVerifierTypes.ECDSACertificate memory cert) external view returns (uint256[] memory totalSignedStakeWeights, address[] memory signers);
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
        pub totalSignedStakeWeights:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        #[allow(missing_docs)]
        pub signers: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
                    (value.totalSignedStakeWeights, value.signers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCertificateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        totalSignedStakeWeights: tuple.0,
                        signers: tuple.1,
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.totalSignedStakeWeights,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.signers),
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
    function verifyCertificateNominal(OperatorSet memory operatorSet, IECDSACertificateVerifierTypes.ECDSACertificate memory cert, uint256[] memory totalStakeNominalThresholds) external view returns (bool, address[] memory signers);
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
        pub signers: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
                    (value._0, value.signers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCertificateNominalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        signers: tuple.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.signers),
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
    function verifyCertificateProportion(OperatorSet memory operatorSet, IECDSACertificateVerifierTypes.ECDSACertificate memory cert, uint16[] memory totalStakeProportionThresholds) external view returns (bool, address[] memory signers);
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
        pub signers: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
                    (value._0, value.signers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCertificateProportionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        signers: tuple.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.signers),
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
    ///Container for all the [`IECDSACertificateVerifier`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum IECDSACertificateVerifierCalls {
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
        updateOperatorTable(updateOperatorTableCall),
        #[allow(missing_docs)]
        verifyCertificate(verifyCertificateCall),
        #[allow(missing_docs)]
        verifyCertificateNominal(verifyCertificateNominalCall),
        #[allow(missing_docs)]
        verifyCertificateProportion(verifyCertificateProportionCall),
    }
    #[automatically_derived]
    impl IECDSACertificateVerifierCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [24u8, 70u8, 116u8, 52u8],
            [35u8, 194u8, 163u8, 203u8],
            [86u8, 212u8, 130u8, 245u8],
            [93u8, 219u8, 155u8, 91u8],
            [97u8, 65u8, 135u8, 158u8],
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
    impl alloy_sol_types::SolInterface for IECDSACertificateVerifierCalls {
        const NAME: &'static str = "IECDSACertificateVerifierCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 15usize;
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
                IECDSACertificateVerifierCalls,
            >] = &[
                {
                    fn calculateCertificateDigest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <calculateCertificateDigestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                IECDSACertificateVerifierCalls::calculateCertificateDigest,
                            )
                    }
                    calculateCertificateDigest
                },
                {
                    fn getOperatorCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <getOperatorCountCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierCalls::getOperatorCount)
                    }
                    getOperatorCount
                },
                {
                    fn updateOperatorTable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierCalls::updateOperatorTable)
                    }
                    updateOperatorTable
                },
                {
                    fn latestReferenceTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <latestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(IECDSACertificateVerifierCalls::latestReferenceTimestamp)
                    }
                    latestReferenceTimestamp
                },
                {
                    fn maxOperatorTableStaleness(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <maxOperatorTableStalenessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(IECDSACertificateVerifierCalls::maxOperatorTableStaleness)
                    }
                    maxOperatorTableStaleness
                },
                {
                    fn calculateCertificateDigestBytes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <calculateCertificateDigestBytesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                IECDSACertificateVerifierCalls::calculateCertificateDigestBytes,
                            )
                    }
                    calculateCertificateDigestBytes
                },
                {
                    fn getOperatorInfos(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <getOperatorInfosCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierCalls::getOperatorInfos)
                    }
                    getOperatorInfos
                },
                {
                    fn getTotalStakeWeights(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <getTotalStakeWeightsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierCalls::getTotalStakeWeights)
                    }
                    getTotalStakeWeights
                },
                {
                    fn verifyCertificate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <verifyCertificateCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierCalls::verifyCertificate)
                    }
                    verifyCertificate
                },
                {
                    fn getOperatorSetOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <getOperatorSetOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierCalls::getOperatorSetOwner)
                    }
                    getOperatorSetOwner
                },
                {
                    fn verifyCertificateNominal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <verifyCertificateNominalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(IECDSACertificateVerifierCalls::verifyCertificateNominal)
                    }
                    verifyCertificateNominal
                },
                {
                    fn verifyCertificateProportion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <verifyCertificateProportionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                IECDSACertificateVerifierCalls::verifyCertificateProportion,
                            )
                    }
                    verifyCertificateProportion
                },
                {
                    fn isReferenceTimestampSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <isReferenceTimestampSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(IECDSACertificateVerifierCalls::isReferenceTimestampSet)
                    }
                    isReferenceTimestampSet
                },
                {
                    fn getOperatorInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <getOperatorInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierCalls::getOperatorInfo)
                    }
                    getOperatorInfo
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierCalls::domainSeparator)
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
                IECDSACertificateVerifierCalls,
            >] = &[
                {
                    fn calculateCertificateDigest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <calculateCertificateDigestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IECDSACertificateVerifierCalls::calculateCertificateDigest,
                            )
                    }
                    calculateCertificateDigest
                },
                {
                    fn getOperatorCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <getOperatorCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IECDSACertificateVerifierCalls::getOperatorCount)
                    }
                    getOperatorCount
                },
                {
                    fn updateOperatorTable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IECDSACertificateVerifierCalls::updateOperatorTable)
                    }
                    updateOperatorTable
                },
                {
                    fn latestReferenceTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <latestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IECDSACertificateVerifierCalls::latestReferenceTimestamp,
                            )
                    }
                    latestReferenceTimestamp
                },
                {
                    fn maxOperatorTableStaleness(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <maxOperatorTableStalenessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IECDSACertificateVerifierCalls::maxOperatorTableStaleness,
                            )
                    }
                    maxOperatorTableStaleness
                },
                {
                    fn calculateCertificateDigestBytes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <calculateCertificateDigestBytesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IECDSACertificateVerifierCalls::calculateCertificateDigestBytes,
                            )
                    }
                    calculateCertificateDigestBytes
                },
                {
                    fn getOperatorInfos(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <getOperatorInfosCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IECDSACertificateVerifierCalls::getOperatorInfos)
                    }
                    getOperatorInfos
                },
                {
                    fn getTotalStakeWeights(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <getTotalStakeWeightsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IECDSACertificateVerifierCalls::getTotalStakeWeights)
                    }
                    getTotalStakeWeights
                },
                {
                    fn verifyCertificate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <verifyCertificateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IECDSACertificateVerifierCalls::verifyCertificate)
                    }
                    verifyCertificate
                },
                {
                    fn getOperatorSetOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <getOperatorSetOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IECDSACertificateVerifierCalls::getOperatorSetOwner)
                    }
                    getOperatorSetOwner
                },
                {
                    fn verifyCertificateNominal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <verifyCertificateNominalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IECDSACertificateVerifierCalls::verifyCertificateNominal,
                            )
                    }
                    verifyCertificateNominal
                },
                {
                    fn verifyCertificateProportion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <verifyCertificateProportionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IECDSACertificateVerifierCalls::verifyCertificateProportion,
                            )
                    }
                    verifyCertificateProportion
                },
                {
                    fn isReferenceTimestampSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <isReferenceTimestampSetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IECDSACertificateVerifierCalls::isReferenceTimestampSet)
                    }
                    isReferenceTimestampSet
                },
                {
                    fn getOperatorInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <getOperatorInfoCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IECDSACertificateVerifierCalls::getOperatorInfo)
                    }
                    getOperatorInfo
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierCalls>
                    {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IECDSACertificateVerifierCalls::domainSeparator)
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
            }
        }
    }
    ///Container for all the [`IECDSACertificateVerifier`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum IECDSACertificateVerifierErrors {
        #[allow(missing_docs)]
        ArrayLengthMismatch(ArrayLengthMismatch),
        #[allow(missing_docs)]
        CertificateStale(CertificateStale),
        #[allow(missing_docs)]
        IndexOutOfBounds(IndexOutOfBounds),
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
        SignersNotOrdered(SignersNotOrdered),
        #[allow(missing_docs)]
        TableUpdateStale(TableUpdateStale),
        #[allow(missing_docs)]
        VerificationFailed(VerificationFailed),
    }
    #[automatically_derived]
    impl IECDSACertificateVerifierErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 24u8, 54u8, 214u8],
            [27u8, 20u8, 23u8, 75u8],
            [47u8, 32u8, 136u8, 159u8],
            [64u8, 164u8, 32u8, 84u8],
            [67u8, 156u8, 192u8, 205u8],
            [75u8, 230u8, 50u8, 27u8],
            [78u8, 35u8, 208u8, 53u8],
            [101u8, 104u8, 189u8, 184u8],
            [162u8, 74u8, 19u8, 166u8],
            [181u8, 80u8, 197u8, 112u8],
            [200u8, 31u8, 154u8, 214u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IECDSACertificateVerifierErrors {
        const NAME: &'static str = "IECDSACertificateVerifierErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 11usize;
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
                Self::SignersNotOrdered(_) => {
                    <SignersNotOrdered as alloy_sol_types::SolError>::SELECTOR
                }
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
                IECDSACertificateVerifierErrors,
            >] = &[
                {
                    fn OnlyTableUpdater(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <OnlyTableUpdater as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierErrors::OnlyTableUpdater)
                    }
                    OnlyTableUpdater
                },
                {
                    fn RootDisabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <RootDisabled as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierErrors::RootDisabled)
                    }
                    RootDisabled
                },
                {
                    fn TableUpdateStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <TableUpdateStale as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierErrors::TableUpdateStale)
                    }
                    TableUpdateStale
                },
                {
                    fn OperatorCountZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <OperatorCountZero as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierErrors::OperatorCountZero)
                    }
                    OperatorCountZero
                },
                {
                    fn VerificationFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <VerificationFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierErrors::VerificationFailed)
                    }
                    VerificationFailed
                },
                {
                    fn InvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <InvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierErrors::InvalidSignatureLength)
                    }
                    InvalidSignatureLength
                },
                {
                    fn IndexOutOfBounds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <IndexOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierErrors::IndexOutOfBounds)
                    }
                    IndexOutOfBounds
                },
                {
                    fn ReferenceTimestampDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <ReferenceTimestampDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                IECDSACertificateVerifierErrors::ReferenceTimestampDoesNotExist,
                            )
                    }
                    ReferenceTimestampDoesNotExist
                },
                {
                    fn ArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierErrors::ArrayLengthMismatch)
                    }
                    ArrayLengthMismatch
                },
                {
                    fn SignersNotOrdered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <SignersNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierErrors::SignersNotOrdered)
                    }
                    SignersNotOrdered
                },
                {
                    fn CertificateStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <CertificateStale as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IECDSACertificateVerifierErrors::CertificateStale)
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
                IECDSACertificateVerifierErrors,
            >] = &[
                {
                    fn OnlyTableUpdater(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <OnlyTableUpdater as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IECDSACertificateVerifierErrors::OnlyTableUpdater)
                    }
                    OnlyTableUpdater
                },
                {
                    fn RootDisabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <RootDisabled as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(IECDSACertificateVerifierErrors::RootDisabled)
                    }
                    RootDisabled
                },
                {
                    fn TableUpdateStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <TableUpdateStale as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IECDSACertificateVerifierErrors::TableUpdateStale)
                    }
                    TableUpdateStale
                },
                {
                    fn OperatorCountZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <OperatorCountZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IECDSACertificateVerifierErrors::OperatorCountZero)
                    }
                    OperatorCountZero
                },
                {
                    fn VerificationFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <VerificationFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IECDSACertificateVerifierErrors::VerificationFailed)
                    }
                    VerificationFailed
                },
                {
                    fn InvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <InvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IECDSACertificateVerifierErrors::InvalidSignatureLength)
                    }
                    InvalidSignatureLength
                },
                {
                    fn IndexOutOfBounds(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <IndexOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IECDSACertificateVerifierErrors::IndexOutOfBounds)
                    }
                    IndexOutOfBounds
                },
                {
                    fn ReferenceTimestampDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <ReferenceTimestampDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                IECDSACertificateVerifierErrors::ReferenceTimestampDoesNotExist,
                            )
                    }
                    ReferenceTimestampDoesNotExist
                },
                {
                    fn ArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IECDSACertificateVerifierErrors::ArrayLengthMismatch)
                    }
                    ArrayLengthMismatch
                },
                {
                    fn SignersNotOrdered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <SignersNotOrdered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IECDSACertificateVerifierErrors::SignersNotOrdered)
                    }
                    SignersNotOrdered
                },
                {
                    fn CertificateStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IECDSACertificateVerifierErrors>
                    {
                        <CertificateStale as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IECDSACertificateVerifierErrors::CertificateStale)
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
                Self::SignersNotOrdered(inner) => {
                    <SignersNotOrdered as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::SignersNotOrdered(inner) => {
                    <SignersNotOrdered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
    ///Container for all the [`IECDSACertificateVerifier`](self) events.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum IECDSACertificateVerifierEvents {
        #[allow(missing_docs)]
        MaxStalenessPeriodUpdated(MaxStalenessPeriodUpdated),
        #[allow(missing_docs)]
        OperatorSetOwnerUpdated(OperatorSetOwnerUpdated),
        #[allow(missing_docs)]
        TableUpdated(TableUpdated),
    }
    #[automatically_derived]
    impl IECDSACertificateVerifierEvents {
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
                128u8, 109u8, 195u8, 103u8, 9u8, 92u8, 11u8, 175u8, 149u8, 61u8, 113u8, 68u8,
                183u8, 196u8, 55u8, 98u8, 97u8, 103u8, 94u8, 224u8, 180u8, 224u8, 218u8, 39u8,
                97u8, 228u8, 54u8, 115u8, 5u8, 28u8, 115u8, 117u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IECDSACertificateVerifierEvents {
        const NAME: &'static str = "IECDSACertificateVerifierEvents";
        const COUNT: usize = 3usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
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
    impl alloy_sol_types::private::IntoLogData for IECDSACertificateVerifierEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
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
    /**Creates a new wrapper around an on-chain [`IECDSACertificateVerifier`](self) contract instance.

    See the [wrapper's documentation](`IECDSACertificateVerifierInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IECDSACertificateVerifierInstance<P, N> {
        IECDSACertificateVerifierInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<IECDSACertificateVerifierInstance<P, N>>,
    > {
        IECDSACertificateVerifierInstance::<P, N>::deploy(provider)
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
    ) -> alloy_contract::RawCallBuilder<P, N> {
        IECDSACertificateVerifierInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`IECDSACertificateVerifier`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IECDSACertificateVerifier`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IECDSACertificateVerifierInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IECDSACertificateVerifierInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IECDSACertificateVerifierInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IECDSACertificateVerifierInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`IECDSACertificateVerifier`](self) contract instance.

        See the [wrapper's documentation](`IECDSACertificateVerifierInstance`) for more details.*/
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
        ) -> alloy_contract::Result<IECDSACertificateVerifierInstance<P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
        and constructor arguments, if any.

        This is a simple wrapper around creating a `RawCallBuilder` with the data set to
        the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<P: ::core::clone::Clone, N> IECDSACertificateVerifierInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IECDSACertificateVerifierInstance<P, N> {
            IECDSACertificateVerifierInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IECDSACertificateVerifierInstance<P, N>
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
    }
    /// Event filters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IECDSACertificateVerifierInstance<P, N>
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
