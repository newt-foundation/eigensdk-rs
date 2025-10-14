///Module containing a contract's types and functions.
/**

```solidity
library BN254 {
    struct G1Point { uint256 X; uint256 Y; }
    struct G2Point { uint256[2] X; uint256[2] Y; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BN254 {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct G1Point { uint256 X; uint256 Y; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G1Point {
        #[allow(missing_docs)]
        pub X: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub Y: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<G1Point> for UnderlyingRustTuple<'_> {
            fn from(value: G1Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G1Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    X: tuple.0,
                    Y: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G1Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.X,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.Y,
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
        impl alloy_sol_types::SolType for G1Point {
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
        impl alloy_sol_types::SolStruct for G1Point {
            const NAME: &'static str = "G1Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G1Point(uint256 X,uint256 Y)")
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
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G1Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.X)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.Y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.Y, out);
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
    struct G2Point { uint256[2] X; uint256[2] Y; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G2Point {
        #[allow(missing_docs)]
        pub X: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        #[allow(missing_docs)]
        pub Y: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
            alloy::sol_types::sol_data::FixedArray<alloy::sol_types::sol_data::Uint<256>, 2usize>,
            alloy::sol_types::sol_data::FixedArray<alloy::sol_types::sol_data::Uint<256>, 2usize>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
        impl ::core::convert::From<G2Point> for UnderlyingRustTuple<'_> {
            fn from(value: G2Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G2Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    X: tuple.0,
                    Y: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G2Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G2Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.X),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.Y),
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
        impl alloy_sol_types::SolType for G2Point {
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
        impl alloy_sol_types::SolStruct for G2Point {
            const NAME: &'static str = "G2Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G2Point(uint256[2] X,uint256[2] Y)")
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
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                    .0,
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                    .0,
                ]
                .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G2Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.X
                    )
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.Y
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.X, out
                );
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.Y, out
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
    /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

    See the [wrapper's documentation](`BN254Instance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BN254Instance<P, N> {
        BN254Instance::<P, N>::new(address, provider)
    }
    /**A [`BN254`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`BN254`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BN254Instance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for BN254Instance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BN254Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BN254Instance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

        See the [wrapper's documentation](`BN254Instance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> BN254Instance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BN254Instance<P, N> {
            BN254Instance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BN254Instance<P, N>
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
        BN254Instance<P, N>
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
library IBN254CertificateVerifierTypes {
    struct BN254Certificate { uint32 referenceTimestamp; bytes32 messageHash; BN254.G1Point signature; BN254.G2Point apk; BN254OperatorInfoWitness[] nonSignerWitnesses; }
    struct BN254OperatorInfoWitness { uint32 operatorIndex; bytes operatorInfoProof; IOperatorTableCalculatorTypes.BN254OperatorInfo operatorInfo; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IBN254CertificateVerifierTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct BN254Certificate { uint32 referenceTimestamp; bytes32 messageHash; BN254.G1Point signature; BN254.G2Point apk; BN254OperatorInfoWitness[] nonSignerWitnesses; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BN254Certificate {
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub messageHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub signature: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub apk: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub nonSignerWitnesses: alloy::sol_types::private::Vec<
            <BN254OperatorInfoWitness as alloy::sol_types::SolType>::RustType,
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            BN254::G1Point,
            BN254::G2Point,
            alloy::sol_types::sol_data::Array<BN254OperatorInfoWitness>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
            alloy::sol_types::private::FixedBytes<32>,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<
                <BN254OperatorInfoWitness as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<BN254Certificate> for UnderlyingRustTuple<'_> {
            fn from(value: BN254Certificate) -> Self {
                (
                    value.referenceTimestamp,
                    value.messageHash,
                    value.signature,
                    value.apk,
                    value.nonSignerWitnesses,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BN254Certificate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    referenceTimestamp: tuple.0,
                    messageHash: tuple.1,
                    signature: tuple.2,
                    apk: tuple.3,
                    nonSignerWitnesses: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BN254Certificate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BN254Certificate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTimestamp),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.messageHash),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apk),
                    <alloy::sol_types::sol_data::Array<
                        BN254OperatorInfoWitness,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerWitnesses),
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
        impl alloy_sol_types::SolType for BN254Certificate {
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
        impl alloy_sol_types::SolStruct for BN254Certificate {
            const NAME: &'static str = "BN254Certificate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BN254Certificate(uint32 referenceTimestamp,bytes32 messageHash,BN254.G1Point signature,BN254.G2Point apk,BN254OperatorInfoWitness[] nonSignerWitnesses)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(3);
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G2Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G2Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(
                    <BN254OperatorInfoWitness as alloy_sol_types::SolStruct>::eip712_root_type(),
                );
                components.extend(
                    <BN254OperatorInfoWitness as alloy_sol_types::SolStruct>::eip712_components(),
                );
                components
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
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <BN254::G2Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.apk,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        BN254OperatorInfoWitness,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerWitnesses,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BN254Certificate {
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
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <BN254::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.apk,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        BN254OperatorInfoWitness,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerWitnesses,
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
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <BN254::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.apk, out,
                );
                <alloy::sol_types::sol_data::Array<
                    BN254OperatorInfoWitness,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerWitnesses,
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
    struct BN254OperatorInfoWitness { uint32 operatorIndex; bytes operatorInfoProof; IOperatorTableCalculatorTypes.BN254OperatorInfo operatorInfo; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BN254OperatorInfoWitness {
        #[allow(missing_docs)]
        pub operatorIndex: u32,
        #[allow(missing_docs)]
        pub operatorInfoProof: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub operatorInfo: <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::Bytes,
            IOperatorTableCalculatorTypes::BN254OperatorInfo,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
            alloy::sol_types::private::Bytes,
            <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<BN254OperatorInfoWitness> for UnderlyingRustTuple<'_> {
            fn from(value: BN254OperatorInfoWitness) -> Self {
                (
                    value.operatorIndex,
                    value.operatorInfoProof,
                    value.operatorInfo,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BN254OperatorInfoWitness {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorIndex: tuple.0,
                    operatorInfoProof: tuple.1,
                    operatorInfo: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BN254OperatorInfoWitness {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BN254OperatorInfoWitness {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIndex),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.operatorInfoProof,
                    ),
                    <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy_sol_types::SolType>::tokenize(
                        &self.operatorInfo,
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
        impl alloy_sol_types::SolType for BN254OperatorInfoWitness {
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
        impl alloy_sol_types::SolStruct for BN254OperatorInfoWitness {
            const NAME: &'static str = "BN254OperatorInfoWitness";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BN254OperatorInfoWitness(uint32 operatorIndex,bytes operatorInfoProof,IOperatorTableCalculatorTypes.BN254OperatorInfo operatorInfo)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorIndex)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorInfoProof,
                        )
                        .0,
                    <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorInfo,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BN254OperatorInfoWitness {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorIndex,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorInfoProof,
                    )
                    + <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorInfo,
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
                    &rust.operatorIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorInfoProof,
                    out,
                );
                <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorInfo,
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
    /**Creates a new wrapper around an on-chain [`IBN254CertificateVerifierTypes`](self) contract instance.

    See the [wrapper's documentation](`IBN254CertificateVerifierTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IBN254CertificateVerifierTypesInstance<P, N> {
        IBN254CertificateVerifierTypesInstance::<P, N>::new(address, provider)
    }
    /**A [`IBN254CertificateVerifierTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IBN254CertificateVerifierTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IBN254CertificateVerifierTypesInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IBN254CertificateVerifierTypesInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBN254CertificateVerifierTypesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IBN254CertificateVerifierTypesInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`IBN254CertificateVerifierTypes`](self) contract instance.

        See the [wrapper's documentation](`IBN254CertificateVerifierTypesInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> IBN254CertificateVerifierTypesInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IBN254CertificateVerifierTypesInstance<P, N> {
            IBN254CertificateVerifierTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IBN254CertificateVerifierTypesInstance<P, N>
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
        IBN254CertificateVerifierTypesInstance<P, N>
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
library IOperatorTableCalculatorTypes {
    struct BN254OperatorInfo { BN254.G1Point pubkey; uint256[] weights; }
    struct BN254OperatorSetInfo { bytes32 operatorInfoTreeRoot; uint256 numOperators; BN254.G1Point aggregatePubkey; uint256[] totalWeights; }
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
    struct BN254OperatorInfo { BN254.G1Point pubkey; uint256[] weights; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BN254OperatorInfo {
        #[allow(missing_docs)]
        pub pubkey: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            BN254::G1Point,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<BN254OperatorInfo> for UnderlyingRustTuple<'_> {
            fn from(value: BN254OperatorInfo) -> Self {
                (value.pubkey, value.weights)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BN254OperatorInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    pubkey: tuple.0,
                    weights: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BN254OperatorInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BN254OperatorInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.pubkey),
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
        impl alloy_sol_types::SolType for BN254OperatorInfo {
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
        impl alloy_sol_types::SolStruct for BN254OperatorInfo {
            const NAME: &'static str = "BN254OperatorInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BN254OperatorInfo(BN254.G1Point pubkey,uint256[] weights)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(
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
        impl alloy_sol_types::EventTopic for BN254OperatorInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
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
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
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
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**```solidity
    struct BN254OperatorSetInfo { bytes32 operatorInfoTreeRoot; uint256 numOperators; BN254.G1Point aggregatePubkey; uint256[] totalWeights; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BN254OperatorSetInfo {
        #[allow(missing_docs)]
        pub operatorInfoTreeRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub numOperators: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub aggregatePubkey: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub totalWeights:
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
            BN254::G1Point,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<BN254OperatorSetInfo> for UnderlyingRustTuple<'_> {
            fn from(value: BN254OperatorSetInfo) -> Self {
                (
                    value.operatorInfoTreeRoot,
                    value.numOperators,
                    value.aggregatePubkey,
                    value.totalWeights,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BN254OperatorSetInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorInfoTreeRoot: tuple.0,
                    numOperators: tuple.1,
                    aggregatePubkey: tuple.2,
                    totalWeights: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BN254OperatorSetInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BN254OperatorSetInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorInfoTreeRoot),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.numOperators),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(
                        &self.aggregatePubkey,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalWeights),
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
        impl alloy_sol_types::SolType for BN254OperatorSetInfo {
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
        impl alloy_sol_types::SolStruct for BN254OperatorSetInfo {
            const NAME: &'static str = "BN254OperatorSetInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BN254OperatorSetInfo(bytes32 operatorInfoTreeRoot,uint256 numOperators,BN254.G1Point aggregatePubkey,uint256[] totalWeights)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorInfoTreeRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.numOperators)
                        .0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.aggregatePubkey,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.totalWeights)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BN254OperatorSetInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorInfoTreeRoot,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.numOperators,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.aggregatePubkey,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalWeights,
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
                    &rust.operatorInfoTreeRoot,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.numOperators,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.aggregatePubkey,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalWeights,
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
library BN254 {
    struct G1Point {
        uint256 X;
        uint256 Y;
    }
    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }
}

library IBN254CertificateVerifierTypes {
    struct BN254Certificate {
        uint32 referenceTimestamp;
        bytes32 messageHash;
        BN254.G1Point signature;
        BN254.G2Point apk;
        BN254OperatorInfoWitness[] nonSignerWitnesses;
    }
    struct BN254OperatorInfoWitness {
        uint32 operatorIndex;
        bytes operatorInfoProof;
        IOperatorTableCalculatorTypes.BN254OperatorInfo operatorInfo;
    }
}

library ICrossChainRegistryTypes {
    struct OperatorSetConfig {
        address owner;
        uint32 maxStalenessPeriod;
    }
}

library IOperatorTableCalculatorTypes {
    struct BN254OperatorInfo {
        BN254.G1Point pubkey;
        uint256[] weights;
    }
    struct BN254OperatorSetInfo {
        bytes32 operatorInfoTreeRoot;
        uint256 numOperators;
        BN254.G1Point aggregatePubkey;
        uint256[] totalWeights;
    }
}

interface BN254CertificateVerifier {
    struct OperatorSet {
        address avs;
        uint32 id;
    }

    error ArrayLengthMismatch();
    error CertificateStale();
    error ECAddFailed();
    error ECMulFailed();
    error ECPairingFailed();
    error ExpModFailed();
    error InvalidOperatorIndex();
    error InvalidProofLength();
    error InvalidShortString();
    error OnlyTableUpdater();
    error ReferenceTimestampDoesNotExist();
    error RootDisabled();
    error StringTooLong(string str);
    error TableUpdateStale();
    error VerificationFailed();

    event Initialized(uint8 version);
    event MaxStalenessPeriodUpdated(OperatorSet operatorSet, uint32 maxStalenessPeriod);
    event OperatorSetOwnerUpdated(OperatorSet operatorSet, address owner);
    event TableUpdated(OperatorSet operatorSet, uint32 referenceTimestamp, IOperatorTableCalculatorTypes.BN254OperatorSetInfo operatorSetInfo);

    constructor(address _operatorTableUpdater, string _version);

    function getNonsignerOperatorInfo(OperatorSet memory operatorSet, uint32 referenceTimestamp, uint256 operatorIndex) external view returns (IOperatorTableCalculatorTypes.BN254OperatorInfo memory);
    function getOperatorSetInfo(OperatorSet memory operatorSet, uint32 referenceTimestamp) external view returns (IOperatorTableCalculatorTypes.BN254OperatorSetInfo memory);
    function getOperatorSetOwner(OperatorSet memory operatorSet) external view returns (address);
    function isNonsignerCached(OperatorSet memory operatorSet, uint32 referenceTimestamp, uint256 operatorIndex) external view returns (bool);
    function latestReferenceTimestamp(OperatorSet memory operatorSet) external view returns (uint32);
    function maxOperatorTableStaleness(OperatorSet memory operatorSet) external view returns (uint32);
    function operatorTableUpdater() external view returns (address);
    function trySignatureVerification(bytes32 msgHash, BN254.G1Point memory aggPubkey, BN254.G2Point memory apkG2, BN254.G1Point memory signature) external view returns (bool pairingSuccessful, bool signatureValid);
    function updateOperatorTable(OperatorSet memory operatorSet, uint32 referenceTimestamp, IOperatorTableCalculatorTypes.BN254OperatorSetInfo memory operatorSetInfo, ICrossChainRegistryTypes.OperatorSetConfig memory operatorSetConfig) external;
    function verifyCertificate(OperatorSet memory operatorSet, IBN254CertificateVerifierTypes.BN254Certificate memory cert) external returns (uint256[] memory signedStakes);
    function verifyCertificateNominal(OperatorSet memory operatorSet, IBN254CertificateVerifierTypes.BN254Certificate memory cert, uint256[] memory totalStakeNominalThresholds) external returns (bool);
    function verifyCertificateProportion(OperatorSet memory operatorSet, IBN254CertificateVerifierTypes.BN254Certificate memory cert, uint16[] memory totalStakeProportionThresholds) external returns (bool);
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
    "name": "getNonsignerOperatorInfo",
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
        "internalType": "struct IOperatorTableCalculatorTypes.BN254OperatorInfo",
        "components": [
          {
            "name": "pubkey",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
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
    "name": "getOperatorSetInfo",
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
        "type": "tuple",
        "internalType": "struct IOperatorTableCalculatorTypes.BN254OperatorSetInfo",
        "components": [
          {
            "name": "operatorInfoTreeRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "numOperators",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "aggregatePubkey",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "totalWeights",
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
    "name": "isNonsignerCached",
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
    "name": "trySignatureVerification",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "aggPubkey",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "apkG2",
        "type": "tuple",
        "internalType": "struct BN254.G2Point",
        "components": [
          {
            "name": "X",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          },
          {
            "name": "Y",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          }
        ]
      },
      {
        "name": "signature",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "pairingSuccessful",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "signatureValid",
        "type": "bool",
        "internalType": "bool"
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
        "name": "operatorSetInfo",
        "type": "tuple",
        "internalType": "struct IOperatorTableCalculatorTypes.BN254OperatorSetInfo",
        "components": [
          {
            "name": "operatorInfoTreeRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "numOperators",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "aggregatePubkey",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "totalWeights",
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
        "internalType": "struct IBN254CertificateVerifierTypes.BN254Certificate",
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
            "name": "signature",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "apk",
            "type": "tuple",
            "internalType": "struct BN254.G2Point",
            "components": [
              {
                "name": "X",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "Y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "nonSignerWitnesses",
            "type": "tuple[]",
            "internalType": "struct IBN254CertificateVerifierTypes.BN254OperatorInfoWitness[]",
            "components": [
              {
                "name": "operatorIndex",
                "type": "uint32",
                "internalType": "uint32"
              },
              {
                "name": "operatorInfoProof",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "operatorInfo",
                "type": "tuple",
                "internalType": "struct IOperatorTableCalculatorTypes.BN254OperatorInfo",
                "components": [
                  {
                    "name": "pubkey",
                    "type": "tuple",
                    "internalType": "struct BN254.G1Point",
                    "components": [
                      {
                        "name": "X",
                        "type": "uint256",
                        "internalType": "uint256"
                      },
                      {
                        "name": "Y",
                        "type": "uint256",
                        "internalType": "uint256"
                      }
                    ]
                  },
                  {
                    "name": "weights",
                    "type": "uint256[]",
                    "internalType": "uint256[]"
                  }
                ]
              }
            ]
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "signedStakes",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "nonpayable"
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
        "internalType": "struct IBN254CertificateVerifierTypes.BN254Certificate",
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
            "name": "signature",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "apk",
            "type": "tuple",
            "internalType": "struct BN254.G2Point",
            "components": [
              {
                "name": "X",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "Y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "nonSignerWitnesses",
            "type": "tuple[]",
            "internalType": "struct IBN254CertificateVerifierTypes.BN254OperatorInfoWitness[]",
            "components": [
              {
                "name": "operatorIndex",
                "type": "uint32",
                "internalType": "uint32"
              },
              {
                "name": "operatorInfoProof",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "operatorInfo",
                "type": "tuple",
                "internalType": "struct IOperatorTableCalculatorTypes.BN254OperatorInfo",
                "components": [
                  {
                    "name": "pubkey",
                    "type": "tuple",
                    "internalType": "struct BN254.G1Point",
                    "components": [
                      {
                        "name": "X",
                        "type": "uint256",
                        "internalType": "uint256"
                      },
                      {
                        "name": "Y",
                        "type": "uint256",
                        "internalType": "uint256"
                      }
                    ]
                  },
                  {
                    "name": "weights",
                    "type": "uint256[]",
                    "internalType": "uint256[]"
                  }
                ]
              }
            ]
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
      }
    ],
    "stateMutability": "nonpayable"
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
        "internalType": "struct IBN254CertificateVerifierTypes.BN254Certificate",
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
            "name": "signature",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "apk",
            "type": "tuple",
            "internalType": "struct BN254.G2Point",
            "components": [
              {
                "name": "X",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "Y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "nonSignerWitnesses",
            "type": "tuple[]",
            "internalType": "struct IBN254CertificateVerifierTypes.BN254OperatorInfoWitness[]",
            "components": [
              {
                "name": "operatorIndex",
                "type": "uint32",
                "internalType": "uint32"
              },
              {
                "name": "operatorInfoProof",
                "type": "bytes",
                "internalType": "bytes"
              },
              {
                "name": "operatorInfo",
                "type": "tuple",
                "internalType": "struct IOperatorTableCalculatorTypes.BN254OperatorInfo",
                "components": [
                  {
                    "name": "pubkey",
                    "type": "tuple",
                    "internalType": "struct BN254.G1Point",
                    "components": [
                      {
                        "name": "X",
                        "type": "uint256",
                        "internalType": "uint256"
                      },
                      {
                        "name": "Y",
                        "type": "uint256",
                        "internalType": "uint256"
                      }
                    ]
                  },
                  {
                    "name": "weights",
                    "type": "uint256[]",
                    "internalType": "uint256[]"
                  }
                ]
              }
            ]
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
      }
    ],
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
        "name": "operatorSetInfo",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IOperatorTableCalculatorTypes.BN254OperatorSetInfo",
        "components": [
          {
            "name": "operatorInfoTreeRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "numOperators",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "aggregatePubkey",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "totalWeights",
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
    "name": "ECAddFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECMulFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECPairingFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpModFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidOperatorIndex",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidProofLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidShortString",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyTableUpdater",
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
pub mod BN254CertificateVerifier {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c060405234801561000f575f5ffd5b50604051612a06380380612a0683398101604081905261002e9161016a565b6001600160a01b0382166080528061004581610058565b60a0525061005161009e565b5050610294565b5f5f829050601f8151111561008b578260405163305a27a960e01b81526004016100829190610239565b60405180910390fd5b80516100968261026e565b179392505050565b5f54610100900460ff16156101055760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608401610082565b5f5460ff90811614610154575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b634e487b7160e01b5f52604160045260245ffd5b5f5f6040838503121561017b575f5ffd5b82516001600160a01b0381168114610191575f5ffd5b60208401519092506001600160401b038111156101ac575f5ffd5b8301601f810185136101bc575f5ffd5b80516001600160401b038111156101d5576101d5610156565b604051601f8201601f19908116603f011681016001600160401b038111828210171561020357610203610156565b60405281815282820160200187101561021a575f5ffd5b8160208401602083015e5f602083830101528093505050509250929050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b8051602080830151919081101561028e575f198160200360031b1b821691505b50919050565b60805160a0516127436102c35f395f6104e401525f81816101de0152818161062f0152610d0201526127435ff3fe608060405234801561000f575f5ffd5b50600436106100cb575f3560e01c80635ddb9b5b1161008857806368d6e0811161006357806368d6e081146101d95780638481892014610218578063dd2ae1b91461022b578063eb39e68f1461023e575f5ffd5b80635ddb9b5b146101895780636141879e146101b15780636738c40b146101c4575f5ffd5b8063017d7974146100cf578063080b7150146100f75780631a18746c1461011757806326af6a3c1461014157806354fd4d50146101615780635be8727414610176575b5f5ffd5b6100e26100dd366004612103565b61025e565b60405190151581526020015b60405180910390f35b61010a6101053660046121de565b6103ef565b6040516100ee9190612229565b61012a610125366004612260565b610404565b6040805192151583529015156020830152016100ee565b61015461014f3660046122ae565b610425565b6040516100ee9190612323565b6101696104dd565b6040516100ee9190612358565b6100e26101843660046122ae565b61050d565b61019c61019736600461238d565b6105d8565b60405163ffffffff90911681526020016100ee565b61019c6101bf36600461238d565b6105fe565b6101d76101d23660046123bd565b610624565b005b6102007f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100ee565b61020061022636600461238d565b6107f1565b6100e2610239366004612476565b61081a565b61025161024c3660046124e9565b6108ad565b6040516100ee9190612560565b5f5f61026a8585610972565b90505f61027686610b32565b5f8181526004602081815260408084208a5163ffffffff16855282528084208151608081018352815481526001820154818501528251808401845260028301548152600383015481860152818401529381018054835181860281018601909452808452969750949593949093606086019383018282801561031457602002820191905f5260205f20905b815481526020019060010190808311610300575b50505050508152505090505f8160600151905085518451146103495760405163512509d360e11b815260040160405180910390fd5b5f5b84518110156103de575f61271088838151811061036a5761036a612572565b602002602001015161ffff1684848151811061038857610388612572565b602002602001015161039a919061259a565b6103a491906125c5565b9050808683815181106103b9576103b9612572565b602002602001015110156103d5575f96505050505050506103e8565b5060010161034b565b5060019450505050505b9392505050565b60606103fb8383610972565b90505b92915050565b5f5f61041886848787600162061a80610b95565b9150915094509492505050565b61042d611a9c565b5f61043785610b32565b5f81815260056020908152604080832063ffffffff8916845282528083208784528252918290208251608081018452815481850190815260018301546060830152815260028201805485518186028101860190965280865295965090949193858401939092908301828280156104ca57602002820191905f5260205f20905b8154815260200190600101908083116104b6575b5050505050815250509150509392505050565b60606105087f0000000000000000000000000000000000000000000000000000000000000000610c5d565b905090565b5f5f61051885610b32565b5f81815260056020908152604080832063ffffffff891684528252808320878452825280832081516080810183528154818401908152600183015460608301528152600282018054845181870281018701909552808552969750949590949193858101939291908301828280156105ac57602002820191905f5260205f20905b815481526020019060010190808311610598575b50505091909252505081515191925050158015906105ce575080516020015115155b9695505050505050565b5f5f6105e383610b32565b5f9081526003602052604090205463ffffffff169392505050565b5f5f61060983610b32565b5f9081526002602052604090205463ffffffff169392505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461066d5760405163030c1b6b60e11b815260040160405180910390fd5b5f6106856106803687900387018761238d565b610b32565b5f8181526003602052604090205490915063ffffffff908116908516116106bf57604051632f20889f60e01b815260040160405180910390fd5b5f81815260046020818152604080842063ffffffff8916855282529283902086518155818701516001820155928601518051600285015581015160038401556060860151805187949361071793908501920190611ac6565b5050505f818152600360209081526040909120805463ffffffff191663ffffffff8716179055610749908301836125d8565b5f8281526001602090815260409182902080546001600160a01b0319166001600160a01b039490941693909317909255610788919084019084016125f1565b5f8281526002602052604090819020805463ffffffff191663ffffffff9390931692909217909155517f93e6bea1c9b5dce4a5c07b00261e956df2a4a253d9ab6ca070ca2037d72ada9e906107e29087908790879061260a565b60405180910390a15050505050565b5f5f6107fc83610b32565b5f908152600160205260409020546001600160a01b03169392505050565b5f5f6108268585610972565b9050825181511461084a5760405163512509d360e11b815260040160405180910390fd5b5f5b81518110156108a15783818151811061086757610867612572565b602002602001015182828151811061088157610881612572565b60200260200101511015610899575f925050506103e8565b60010161084c565b50600195945050505050565b6108b5611b0f565b5f6108bf84610b32565b5f81815260046020818152604080842063ffffffff891685528252928390208351608081018552815481526001820154818401528451808601865260028301548152600383015481850152818601529281018054855181850281018501909652808652959650929490936060860193909290919083018282801561096057602002820191905f5260205f20905b81548152602001906001019080831161094c575b50505050508152505091505092915050565b606061097c611b41565b61098584610b32565b80825283516109949190610c9a565b80515f908152600460208181526040808420875163ffffffff1685528252928390208351608081018552815481526001820154818401528451808601865260028301548152600383015481850152818601529281018054855181850281018501909652808652939491936060860193830182828015610a3057602002820191905f5260205f20905b815481526020019060010190808311610a1c575b505050919092525050506020820181905251610a5f57604051630cad17b760e31b815260040160405180910390fd5b806020015160600151516001600160401b03811115610a8057610a80611c6f565b604051908082528060200260200182016040528015610aa9578160200160208202803683370190505b5060408201525f5b81602001516060015151811015610b0d578160200151606001518181518110610adc57610adc612572565b602002602001015182604001518281518110610afa57610afa612572565b6020908102919091010152600101610ab1565b50610b188184610d95565b6060820152610b278184610ebf565b604001519392505050565b5f815f0151826020015163ffffffff16604051602001610b7d92919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b6040516020818303038152906040526103fe90612656565b5f5f5f610ba189610f2d565b90505f610bb08a89898c610fb7565b90505f610bc7610bc08a8461106b565b8b906110db565b90505f610c09610c0284610bfc6040805180820182525f80825260209182015281518083019092526001825260029082015290565b9061106b565b85906110db565b90508715610c2e57610c2582610c1d61114f565b838c8b61120f565b96509450610c4e565b610c4182610c3a61114f565b838c611423565b95508515610c4e57600194505b50505050965096945050505050565b60605f610c698361165a565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b5f8281526002602052604090205463ffffffff16801580610cca5750610cc08183612679565b63ffffffff164211155b610ce75760405163640fcd6b60e11b815260040160405180910390fd5b60405163193877e160e21b815263ffffffff831660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906364e1df8490602401602060405180830381865afa158015610d4f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d739190612695565b610d9057604051631b14174b60e01b815260040160405180910390fd5b505050565b6040805180820182525f808252602091820181905282518084019093528083529082018190525b826080015151811015610eb8575f83608001518281518110610de057610de0612572565b60200260200101519050846020015160200151815f015163ffffffff1610610e1b576040516301fa53c760e11b815260040160405180910390fd5b845184515f91610e2b9184611681565b8051909150610e3b9085906110db565b93505f5b816020015151811015610ead57866040015151811015610ea55781602001518181518110610e6f57610e6f612572565b602002602001015187604001518281518110610e8d57610e8d612572565b60200260200101818151610ea191906126b4565b9052505b600101610e3f565b505050600101610dbc565b5092915050565b5f610edf610ed084606001516117f8565b602085015160400151906110db565b90505f5f610efb84602001518486606001518760400151610404565b91509150818015610f095750805b610f265760405163439cc0cd60e01b815260040160405180910390fd5b5050505050565b604080518082019091525f80825260208201525f8080610f5a5f5160206126ee5f395f51905f52866126c7565b90505b610f668161188e565b90935091505f5160206126ee5f395f51905f528283098303610f9e576040805180820190915290815260208101919091529392505050565b5f5160206126ee5f395f51905f52600182089050610f5d565b8251602080850151845180519083015186840151805190850151875188870151604080519889018e90528801989098526060870195909552608086019390935260a085019190915260c084015260e08301526101008201526101208101919091525f907f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f00000019061014001604051602081830303815290604052805190602001205f1c61106291906126c7565b95945050505050565b604080518082019091525f8082526020820152611086611b86565b835181526020808501519082015260408082018490525f908360608460076107d05a03fa905080806110b457fe5b50806110d357604051632319df1960e11b815260040160405180910390fd5b505092915050565b604080518082019091525f80825260208201526110f6611ba4565b835181526020808501518183015283516040808401919091529084015160608301525f908360808460066107d05a03fa9050808061113057fe5b50806110d35760405163d4b68fd760e01b815260040160405180910390fd5b611157611bc2565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b6040805180820182528681526020808201869052825180840190935286835282018490525f91829190611240611be2565b5f5b60028110156113f7575f61125782600661259a565b905084826002811061126b5761126b612572565b6020020151518361127c835f6126da565b600c811061128c5761128c612572565b60200201528482600281106112a3576112a3612572565b602002015160200151838260016112ba91906126da565b600c81106112ca576112ca612572565b60200201528382600281106112e1576112e1612572565b60200201515151836112f48360026126da565b600c811061130457611304612572565b602002015283826002811061131b5761131b612572565b60200201515160016020020151836113348360036126da565b600c811061134457611344612572565b602002015283826002811061135b5761135b612572565b6020020151602001515f6002811061137557611375612572565b6020020151836113868360046126da565b600c811061139657611396612572565b60200201528382600281106113ad576113ad612572565b6020020151602001516001600281106113c8576113c8612572565b6020020151836113d98360056126da565b600c81106113e9576113e9612572565b602002015250600101611242565b50611400611c01565b5f6020826101808560088cfa9151919c9115159b50909950505050505050505050565b6040805180820182528581526020808201859052825180840190935285835282018390525f91611451611be2565b5f5b6002811015611608575f61146882600661259a565b905084826002811061147c5761147c612572565b6020020151518361148d835f6126da565b600c811061149d5761149d612572565b60200201528482600281106114b4576114b4612572565b602002015160200151838260016114cb91906126da565b600c81106114db576114db612572565b60200201528382600281106114f2576114f2612572565b60200201515151836115058360026126da565b600c811061151557611515612572565b602002015283826002811061152c5761152c612572565b60200201515160016020020151836115458360036126da565b600c811061155557611555612572565b602002015283826002811061156c5761156c612572565b6020020151602001515f6002811061158657611586612572565b6020020151836115978360046126da565b600c81106115a7576115a7612572565b60200201528382600281106115be576115be612572565b6020020151602001516001600281106115d9576115d9612572565b6020020151836115ea8360056126da565b600c81106115fa576115fa612572565b602002015250600101611453565b50611611611c01565b5f6020826101808560086107d05a03fa9050808061162b57fe5b508061164a576040516324ccc79360e21b815260040160405180910390fd5b5051151598975050505050505050565b5f60ff8216601f8111156103fe57604051632cd44ac360e21b815260040160405180910390fd5b611689611a9c565b5f84815260056020908152604080832063ffffffff808816855290835281842086519091168452825280832081516080810183528154818401908152600183015460608301528152600282018054845181870281018701909552808552919492938584019390929083018282801561171e57602002820191905f5260205f20905b81548152602001906001019080831161170a575b5050509190925250508151519192505f911515905080611742575081516020015115155b9050806117eb575f6117628787875f01518860400151896020015161190a565b9050806117825760405163439cc0cd60e01b815260040160405180910390fd5b6040808601515f8981526005602090815283822063ffffffff808c1684529082528483208a51909116835281529290208151805182558301516001820155828201518051929391926117da9260028501920190611ac6565b5090505084604001519350506117ef565b8192505b50509392505050565b604080518082019091525f8082526020820152815115801561181c57506020820151155b15611839575050604080518082019091525f808252602082015290565b6040518060400160405280835f015181526020015f5160206126ee5f395f51905f52846020015161186a91906126c7565b611881905f5160206126ee5f395f51905f526126b4565b905292915050565b919050565b5f80805f5160206126ee5f395f51905f5260035f5160206126ee5f395f51905f52865f5160206126ee5f395f51905f52888909090890505f6118fe827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f525f5160206126ee5f395f51905f52611975565b91959194509092505050565b5f5f8360405160200161191d9190612323565b60408051601f1981840301815291815281516020928301205f8a81526004845282812063ffffffff808c1683529452919091205490925090611969908590839085908a8116906119ee16565b98975050505050505050565b5f5f61197f611c01565b611987611c1f565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa925082806119c457fe5b50826119e35760405163d51edae360e01b815260040160405180910390fd5b505195945050505050565b5f836119fb868585611a05565b1495945050505050565b5f60208451611a1491906126c7565b15611a32576040516313717da960e21b815260040160405180910390fd5b8260205b85518111611a9357611a496002856126c7565b5f03611a6a57815f528086015160205260405f209150600284049350611a81565b808601515f528160205260405f2091506002840493505b611a8c6020826126da565b9050611a36565b50949350505050565b604080516080810182525f91810182815260608201929092529081905b8152602001606081525090565b828054828255905f5260205f20908101928215611aff579160200282015b82811115611aff578251825591602001919060010190611ae4565b50611b0b929150611c3d565b5090565b60405180608001604052805f81526020015f8152602001611ab960405180604001604052805f81526020015f81525090565b60405180608001604052805f8152602001611b5a611b0f565b815260200160608152602001611b8160405180604001604052805f81526020015f81525090565b905290565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280611bd5611c51565b8152602001611b81611c51565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b5b80821115611b0b575f8155600101611c3e565b60405180604001604052806002906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b0381118282101715611ca557611ca5611c6f565b60405290565b60405160a081016001600160401b0381118282101715611ca557611ca5611c6f565b604051606081016001600160401b0381118282101715611ca557611ca5611c6f565b604051608081016001600160401b0381118282101715611ca557611ca5611c6f565b604051601f8201601f191681016001600160401b0381118282101715611d3957611d39611c6f565b604052919050565b80356001600160a01b0381168114611889575f5ffd5b803563ffffffff81168114611889575f5ffd5b5f60408284031215611d7a575f5ffd5b611d82611c83565b9050611d8d82611d41565b8152611d9b60208301611d57565b602082015292915050565b5f60408284031215611db6575f5ffd5b611dbe611c83565b823581526020928301359281019290925250919050565b5f82601f830112611de4575f5ffd5b611dec611c83565b806040840185811115611dfd575f5ffd5b845b81811015611e17578035845260209384019301611dff565b509095945050505050565b5f60808284031215611e32575f5ffd5b611e3a611c83565b9050611e468383611dd5565b8152611d9b8360408401611dd5565b5f6001600160401b03821115611e6d57611e6d611c6f565b5060051b60200190565b5f82601f830112611e86575f5ffd5b8135611e99611e9482611e55565b611d11565b8082825260208201915060208360051b860101925085831115611eba575f5ffd5b602085015b83811015611ed7578035835260209283019201611ebf565b5095945050505050565b5f60608284031215611ef1575f5ffd5b611ef9611c83565b9050611f058383611da6565b815260408201356001600160401b03811115611f1f575f5ffd5b611f2b84828501611e77565b60208301525092915050565b5f6101208284031215611f48575f5ffd5b611f50611cab565b9050611f5b82611d57565b815260208281013590820152611f748360408401611da6565b6040820152611f868360808401611e22565b60608201526101008201356001600160401b03811115611fa4575f5ffd5b8201601f81018413611fb4575f5ffd5b8035611fc2611e9482611e55565b8082825260208201915060208360051b850101925086831115611fe3575f5ffd5b602084015b838110156120f35780356001600160401b03811115612005575f5ffd5b85016060818a03601f1901121561201a575f5ffd5b612022611ccd565b61202e60208301611d57565b815260408201356001600160401b03811115612048575f5ffd5b82016020810190603f018b1361205c575f5ffd5b80356001600160401b0381111561207557612075611c6f565b612088601f8201601f1916602001611d11565b8181528c602083850101111561209c575f5ffd5b816020840160208301375f6020838301015280602085015250505060608201356001600160401b038111156120cf575f5ffd5b6120de8b602083860101611ee1565b60408301525084525060209283019201611fe8565b5060808501525091949350505050565b5f5f5f60808486031215612115575f5ffd5b61211f8585611d6a565b925060408401356001600160401b03811115612139575f5ffd5b61214586828701611f37565b92505060608401356001600160401b03811115612160575f5ffd5b8401601f81018613612170575f5ffd5b803561217e611e9482611e55565b8082825260208201915060208360051b85010192508883111561219f575f5ffd5b6020840193505b828410156121d057833561ffff811681146121bf575f5ffd5b8252602093840193909101906121a6565b809450505050509250925092565b5f5f606083850312156121ef575f5ffd5b6121f98484611d6a565b915060408301356001600160401b03811115612213575f5ffd5b61221f85828601611f37565b9150509250929050565b602080825282518282018190525f918401906040840190835b81811015611e17578351835260209384019390920191600101612242565b5f5f5f5f6101208587031215612274575f5ffd5b843593506122858660208701611da6565b92506122948660608701611e22565b91506122a38660e08701611da6565b905092959194509250565b5f5f5f608084860312156122c0575f5ffd5b6122ca8585611d6a565b92506122d860408501611d57565b929592945050506060919091013590565b5f8151808452602084019350602083015f5b828110156123195781518652602095860195909101906001016122fb565b5093949350505050565b60208082528251805183830152015160408201525f602083015160608084015261235060808401826122e9565b949350505050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b5f6040828403121561239d575f5ffd5b6103fb8383611d6a565b5f604082840312156123b7575f5ffd5b50919050565b5f5f5f5f60c085870312156123d0575f5ffd5b6123da86866123a7565b93506123e860408601611d57565b925060608501356001600160401b03811115612402575f5ffd5b850160a08188031215612413575f5ffd5b61241b611cef565b81358152602080830135908201526124368860408401611da6565b604082015260808201356001600160401b03811115612453575f5ffd5b61245f89828501611e77565b60608301525092506122a3905086608087016123a7565b5f5f5f60808486031215612488575f5ffd5b6124928585611d6a565b925060408401356001600160401b038111156124ac575f5ffd5b6124b886828701611f37565b92505060608401356001600160401b038111156124d3575f5ffd5b6124df86828701611e77565b9150509250925092565b5f5f606083850312156124fa575f5ffd5b6125048484611d6a565b915061251260408401611d57565b90509250929050565b80518252602081015160208301525f6040820151612546604085018280518252602090810151910152565b50606082015160a0608085015261235060a08501826122e9565b602081525f6103fb602083018461251b565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b80820281158282048414176103fe576103fe612586565b634e487b7160e01b5f52601260045260245ffd5b5f826125d3576125d36125b1565b500490565b5f602082840312156125e8575f5ffd5b6103fb82611d41565b5f60208284031215612601575f5ffd5b6103fb82611d57565b6001600160a01b0361261b85611d41565b16815263ffffffff61262f60208601611d57565b16602082015263ffffffff83166040820152608060608201525f611062608083018461251b565b805160208083015191908110156123b7575f1960209190910360031b1b16919050565b63ffffffff81811683821601908111156103fe576103fe612586565b5f602082840312156126a5575f5ffd5b815180151581146103e8575f5ffd5b818103818111156103fe576103fe612586565b5f826126d5576126d56125b1565b500690565b808201808211156103fe576103fe61258656fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220a865aa49005873bcdb12a4b8ff2641cdafed180bd82251bab1af28d752526c6064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa*\x068\x03\x80a*\x06\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01jV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\x80R\x80a\0E\x81a\0XV[`\xA0RPa\0Qa\0\x9EV[PPa\x02\x94V[__\x82\x90P`\x1F\x81Q\x11\x15a\0\x8BW\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\0\x82\x91\x90a\x029V[`@Q\x80\x91\x03\x90\xFD[\x80Qa\0\x96\x82a\x02nV[\x17\x93\x92PPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x01\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01a\0\x82V[_T`\xFF\x90\x81\x16\x14a\x01TW_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__`@\x83\x85\x03\x12\x15a\x01{W__\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x91W__\xFD[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xACW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x01\xBCW__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xD5Wa\x01\xD5a\x01VV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02\x03Wa\x02\x03a\x01VV[`@R\x81\x81R\x82\x82\x01` \x01\x87\x10\x15a\x02\x1AW__\xFD[\x81` \x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x02\x8EW_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\x80Q`\xA0Qa'Ca\x02\xC3_9_a\x04\xE4\x01R_\x81\x81a\x01\xDE\x01R\x81\x81a\x06/\x01Ra\r\x02\x01Ra'C_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xCBW_5`\xE0\x1C\x80c]\xDB\x9B[\x11a\0\x88W\x80ch\xD6\xE0\x81\x11a\0cW\x80ch\xD6\xE0\x81\x14a\x01\xD9W\x80c\x84\x81\x89 \x14a\x02\x18W\x80c\xDD*\xE1\xB9\x14a\x02+W\x80c\xEB9\xE6\x8F\x14a\x02>W__\xFD[\x80c]\xDB\x9B[\x14a\x01\x89W\x80caA\x87\x9E\x14a\x01\xB1W\x80cg8\xC4\x0B\x14a\x01\xC4W__\xFD[\x80c\x01}yt\x14a\0\xCFW\x80c\x08\x0BqP\x14a\0\xF7W\x80c\x1A\x18tl\x14a\x01\x17W\x80c&\xAFj<\x14a\x01AW\x80cT\xFDMP\x14a\x01aW\x80c[\xE8rt\x14a\x01vW[__\xFD[a\0\xE2a\0\xDD6`\x04a!\x03V[a\x02^V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\na\x01\x056`\x04a!\xDEV[a\x03\xEFV[`@Qa\0\xEE\x91\x90a\")V[a\x01*a\x01%6`\x04a\"`V[a\x04\x04V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\0\xEEV[a\x01Ta\x01O6`\x04a\"\xAEV[a\x04%V[`@Qa\0\xEE\x91\x90a##V[a\x01ia\x04\xDDV[`@Qa\0\xEE\x91\x90a#XV[a\0\xE2a\x01\x846`\x04a\"\xAEV[a\x05\rV[a\x01\x9Ca\x01\x976`\x04a#\x8DV[a\x05\xD8V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xEEV[a\x01\x9Ca\x01\xBF6`\x04a#\x8DV[a\x05\xFEV[a\x01\xD7a\x01\xD26`\x04a#\xBDV[a\x06$V[\0[a\x02\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xEEV[a\x02\0a\x02&6`\x04a#\x8DV[a\x07\xF1V[a\0\xE2a\x0296`\x04a$vV[a\x08\x1AV[a\x02Qa\x02L6`\x04a$\xE9V[a\x08\xADV[`@Qa\0\xEE\x91\x90a%`V[__a\x02j\x85\x85a\trV[\x90P_a\x02v\x86a\x0B2V[_\x81\x81R`\x04` \x81\x81R`@\x80\x84 \x8AQc\xFF\xFF\xFF\xFF\x16\x85R\x82R\x80\x84 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x81\x85\x01R\x82Q\x80\x84\x01\x84R`\x02\x83\x01T\x81R`\x03\x83\x01T\x81\x86\x01R\x81\x84\x01R\x93\x81\x01\x80T\x83Q\x81\x86\x02\x81\x01\x86\x01\x90\x94R\x80\x84R\x96\x97P\x94\x95\x93\x94\x90\x93``\x86\x01\x93\x83\x01\x82\x82\x80\x15a\x03\x14W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x03\0W[PPPPP\x81RPP\x90P_\x81``\x01Q\x90P\x85Q\x84Q\x14a\x03IW`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84Q\x81\x10\x15a\x03\xDEW_a'\x10\x88\x83\x81Q\x81\x10a\x03jWa\x03ja%rV[` \x02` \x01\x01Qa\xFF\xFF\x16\x84\x84\x81Q\x81\x10a\x03\x88Wa\x03\x88a%rV[` \x02` \x01\x01Qa\x03\x9A\x91\x90a%\x9AV[a\x03\xA4\x91\x90a%\xC5V[\x90P\x80\x86\x83\x81Q\x81\x10a\x03\xB9Wa\x03\xB9a%rV[` \x02` \x01\x01Q\x10\x15a\x03\xD5W_\x96PPPPPPPa\x03\xE8V[P`\x01\x01a\x03KV[P`\x01\x94PPPPP[\x93\x92PPPV[``a\x03\xFB\x83\x83a\trV[\x90P[\x92\x91PPV[__a\x04\x18\x86\x84\x87\x87`\x01b\x06\x1A\x80a\x0B\x95V[\x91P\x91P\x94P\x94\x92PPPV[a\x04-a\x1A\x9CV[_a\x047\x85a\x0B2V[_\x81\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x82R\x80\x83 \x87\x84R\x82R\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T\x81\x85\x01\x90\x81R`\x01\x83\x01T``\x83\x01R\x81R`\x02\x82\x01\x80T\x85Q\x81\x86\x02\x81\x01\x86\x01\x90\x96R\x80\x86R\x95\x96P\x90\x94\x91\x93\x85\x84\x01\x93\x90\x92\x90\x83\x01\x82\x82\x80\x15a\x04\xCAW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x04\xB6W[PPPPP\x81RPP\x91PP\x93\x92PPPV[``a\x05\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0C]V[\x90P\x90V[__a\x05\x18\x85a\x0B2V[_\x81\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x82R\x80\x83 \x87\x84R\x82R\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T\x81\x84\x01\x90\x81R`\x01\x83\x01T``\x83\x01R\x81R`\x02\x82\x01\x80T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x96\x97P\x94\x95\x90\x94\x91\x93\x85\x81\x01\x93\x92\x91\x90\x83\x01\x82\x82\x80\x15a\x05\xACW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05\x98W[PPP\x91\x90\x92RPP\x81QQ\x91\x92PP\x15\x80\x15\x90a\x05\xCEWP\x80Q` \x01Q\x15\x15[\x96\x95PPPPPPV[__a\x05\xE3\x83a\x0B2V[_\x90\x81R`\x03` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x93\x92PPPV[__a\x06\t\x83a\x0B2V[_\x90\x81R`\x02` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06mW`@Qc\x03\x0C\x1Bk`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x06\x85a\x06\x806\x87\x90\x03\x87\x01\x87a#\x8DV[a\x0B2V[_\x81\x81R`\x03` R`@\x90 T\x90\x91Pc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x85\x16\x11a\x06\xBFW`@Qc/ \x88\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x04` \x81\x81R`@\x80\x84 c\xFF\xFF\xFF\xFF\x89\x16\x85R\x82R\x92\x83\x90 \x86Q\x81U\x81\x87\x01Q`\x01\x82\x01U\x92\x86\x01Q\x80Q`\x02\x85\x01U\x81\x01Q`\x03\x84\x01U``\x86\x01Q\x80Q\x87\x94\x93a\x07\x17\x93\x90\x85\x01\x92\x01\x90a\x1A\xC6V[PPP_\x81\x81R`\x03` \x90\x81R`@\x90\x91 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x87\x16\x17\x90Ua\x07I\x90\x83\x01\x83a%\xD8V[_\x82\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x93\x90\x93\x17\x90\x92Ua\x07\x88\x91\x90\x84\x01\x90\x84\x01a%\xF1V[_\x82\x81R`\x02` R`@\x90\x81\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91UQ\x7F\x93\xE6\xBE\xA1\xC9\xB5\xDC\xE4\xA5\xC0{\0&\x1E\x95m\xF2\xA4\xA2S\xD9\xABl\xA0p\xCA 7\xD7*\xDA\x9E\x90a\x07\xE2\x90\x87\x90\x87\x90\x87\x90a&\nV[`@Q\x80\x91\x03\x90\xA1PPPPPV[__a\x07\xFC\x83a\x0B2V[_\x90\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x93\x92PPPV[__a\x08&\x85\x85a\trV[\x90P\x82Q\x81Q\x14a\x08JW`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x81Q\x81\x10\x15a\x08\xA1W\x83\x81\x81Q\x81\x10a\x08gWa\x08ga%rV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x08\x81Wa\x08\x81a%rV[` \x02` \x01\x01Q\x10\x15a\x08\x99W_\x92PPPa\x03\xE8V[`\x01\x01a\x08LV[P`\x01\x95\x94PPPPPV[a\x08\xB5a\x1B\x0FV[_a\x08\xBF\x84a\x0B2V[_\x81\x81R`\x04` \x81\x81R`@\x80\x84 c\xFF\xFF\xFF\xFF\x89\x16\x85R\x82R\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x81\x84\x01R\x84Q\x80\x86\x01\x86R`\x02\x83\x01T\x81R`\x03\x83\x01T\x81\x85\x01R\x81\x86\x01R\x92\x81\x01\x80T\x85Q\x81\x85\x02\x81\x01\x85\x01\x90\x96R\x80\x86R\x95\x96P\x92\x94\x90\x93``\x86\x01\x93\x90\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\t`W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\tLW[PPPPP\x81RPP\x91PP\x92\x91PPV[``a\t|a\x1BAV[a\t\x85\x84a\x0B2V[\x80\x82R\x83Qa\t\x94\x91\x90a\x0C\x9AV[\x80Q_\x90\x81R`\x04` \x81\x81R`@\x80\x84 \x87Qc\xFF\xFF\xFF\xFF\x16\x85R\x82R\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x81\x84\x01R\x84Q\x80\x86\x01\x86R`\x02\x83\x01T\x81R`\x03\x83\x01T\x81\x85\x01R\x81\x86\x01R\x92\x81\x01\x80T\x85Q\x81\x85\x02\x81\x01\x85\x01\x90\x96R\x80\x86R\x93\x94\x91\x93``\x86\x01\x93\x83\x01\x82\x82\x80\x15a\n0W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\n\x1CW[PPP\x91\x90\x92RPPP` \x82\x01\x81\x90RQa\n_W`@Qc\x0C\xAD\x17\xB7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80` \x01Q``\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x80Wa\n\x80a\x1CoV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xA9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`@\x82\x01R_[\x81` \x01Q``\x01QQ\x81\x10\x15a\x0B\rW\x81` \x01Q``\x01Q\x81\x81Q\x81\x10a\n\xDCWa\n\xDCa%rV[` \x02` \x01\x01Q\x82`@\x01Q\x82\x81Q\x81\x10a\n\xFAWa\n\xFAa%rV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\n\xB1V[Pa\x0B\x18\x81\x84a\r\x95V[``\x82\x01Ra\x0B'\x81\x84a\x0E\xBFV[`@\x01Q\x93\x92PPPV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a\x0B}\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x03\xFE\x90a&VV[___a\x0B\xA1\x89a\x0F-V[\x90P_a\x0B\xB0\x8A\x89\x89\x8Ca\x0F\xB7V[\x90P_a\x0B\xC7a\x0B\xC0\x8A\x84a\x10kV[\x8B\x90a\x10\xDBV[\x90P_a\x0C\ta\x0C\x02\x84a\x0B\xFC`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\x10kV[\x85\x90a\x10\xDBV[\x90P\x87\x15a\x0C.Wa\x0C%\x82a\x0C\x1Da\x11OV[\x83\x8C\x8Ba\x12\x0FV[\x96P\x94Pa\x0CNV[a\x0CA\x82a\x0C:a\x11OV[\x83\x8Ca\x14#V[\x95P\x85\x15a\x0CNW`\x01\x94P[PPPP\x96P\x96\x94PPPPPV[``_a\x0Ci\x83a\x16ZV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[_\x82\x81R`\x02` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x80\x15\x80a\x0C\xCAWPa\x0C\xC0\x81\x83a&yV[c\xFF\xFF\xFF\xFF\x16B\x11\x15[a\x0C\xE7W`@Qcd\x0F\xCDk`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x198w\xE1`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cd\xE1\xDF\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rOW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rs\x91\x90a&\x95V[a\r\x90W`@Qc\x1B\x14\x17K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01\x81\x90R\x82Q\x80\x84\x01\x90\x93R\x80\x83R\x90\x82\x01\x81\x90R[\x82`\x80\x01QQ\x81\x10\x15a\x0E\xB8W_\x83`\x80\x01Q\x82\x81Q\x81\x10a\r\xE0Wa\r\xE0a%rV[` \x02` \x01\x01Q\x90P\x84` \x01Q` \x01Q\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x10a\x0E\x1BW`@Qc\x01\xFAS\xC7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Q\x84Q_\x91a\x0E+\x91\x84a\x16\x81V[\x80Q\x90\x91Pa\x0E;\x90\x85\x90a\x10\xDBV[\x93P_[\x81` \x01QQ\x81\x10\x15a\x0E\xADW\x86`@\x01QQ\x81\x10\x15a\x0E\xA5W\x81` \x01Q\x81\x81Q\x81\x10a\x0EoWa\x0Eoa%rV[` \x02` \x01\x01Q\x87`@\x01Q\x82\x81Q\x81\x10a\x0E\x8DWa\x0E\x8Da%rV[` \x02` \x01\x01\x81\x81Qa\x0E\xA1\x91\x90a&\xB4V[\x90RP[`\x01\x01a\x0E?V[PPP`\x01\x01a\r\xBCV[P\x92\x91PPV[_a\x0E\xDFa\x0E\xD0\x84``\x01Qa\x17\xF8V[` \x85\x01Q`@\x01Q\x90a\x10\xDBV[\x90P__a\x0E\xFB\x84` \x01Q\x84\x86``\x01Q\x87`@\x01Qa\x04\x04V[\x91P\x91P\x81\x80\x15a\x0F\tWP\x80[a\x0F&W`@QcC\x9C\xC0\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80\x80a\x0FZ_Q` a&\xEE_9_Q\x90_R\x86a&\xC7V[\x90P[a\x0Ff\x81a\x18\x8EV[\x90\x93P\x91P_Q` a&\xEE_9_Q\x90_R\x82\x83\t\x83\x03a\x0F\x9EW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[_Q` a&\xEE_9_Q\x90_R`\x01\x82\x08\x90Pa\x0F]V[\x82Q` \x80\x85\x01Q\x84Q\x80Q\x90\x83\x01Q\x86\x84\x01Q\x80Q\x90\x85\x01Q\x87Q\x88\x87\x01Q`@\x80Q\x98\x89\x01\x8E\x90R\x88\x01\x98\x90\x98R``\x87\x01\x95\x90\x95R`\x80\x86\x01\x93\x90\x93R`\xA0\x85\x01\x91\x90\x91R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x81\x01\x91\x90\x91R_\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90a\x01@\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\x10b\x91\x90a&\xC7V[\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x10\x86a\x1B\x86V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R_\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x10\xB4W\xFE[P\x80a\x10\xD3W`@Qc#\x19\xDF\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x10\xF6a\x1B\xA4V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R_\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x110W\xFE[P\x80a\x10\xD3W`@Qc\xD4\xB6\x8F\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11Wa\x1B\xC2V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R_\x91\x82\x91\x90a\x12@a\x1B\xE2V[_[`\x02\x81\x10\x15a\x13\xF7W_a\x12W\x82`\x06a%\x9AV[\x90P\x84\x82`\x02\x81\x10a\x12kWa\x12ka%rV[` \x02\x01QQ\x83a\x12|\x83_a&\xDAV[`\x0C\x81\x10a\x12\x8CWa\x12\x8Ca%rV[` \x02\x01R\x84\x82`\x02\x81\x10a\x12\xA3Wa\x12\xA3a%rV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x12\xBA\x91\x90a&\xDAV[`\x0C\x81\x10a\x12\xCAWa\x12\xCAa%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x12\xE1Wa\x12\xE1a%rV[` \x02\x01QQQ\x83a\x12\xF4\x83`\x02a&\xDAV[`\x0C\x81\x10a\x13\x04Wa\x13\x04a%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x13\x1BWa\x13\x1Ba%rV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x134\x83`\x03a&\xDAV[`\x0C\x81\x10a\x13DWa\x13Da%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x13[Wa\x13[a%rV[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x13uWa\x13ua%rV[` \x02\x01Q\x83a\x13\x86\x83`\x04a&\xDAV[`\x0C\x81\x10a\x13\x96Wa\x13\x96a%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x13\xADWa\x13\xADa%rV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x13\xC8Wa\x13\xC8a%rV[` \x02\x01Q\x83a\x13\xD9\x83`\x05a&\xDAV[`\x0C\x81\x10a\x13\xE9Wa\x13\xE9a%rV[` \x02\x01RP`\x01\x01a\x12BV[Pa\x14\0a\x1C\x01V[_` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R_\x91a\x14Qa\x1B\xE2V[_[`\x02\x81\x10\x15a\x16\x08W_a\x14h\x82`\x06a%\x9AV[\x90P\x84\x82`\x02\x81\x10a\x14|Wa\x14|a%rV[` \x02\x01QQ\x83a\x14\x8D\x83_a&\xDAV[`\x0C\x81\x10a\x14\x9DWa\x14\x9Da%rV[` \x02\x01R\x84\x82`\x02\x81\x10a\x14\xB4Wa\x14\xB4a%rV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x14\xCB\x91\x90a&\xDAV[`\x0C\x81\x10a\x14\xDBWa\x14\xDBa%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x14\xF2Wa\x14\xF2a%rV[` \x02\x01QQQ\x83a\x15\x05\x83`\x02a&\xDAV[`\x0C\x81\x10a\x15\x15Wa\x15\x15a%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x15,Wa\x15,a%rV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x15E\x83`\x03a&\xDAV[`\x0C\x81\x10a\x15UWa\x15Ua%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x15lWa\x15la%rV[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x15\x86Wa\x15\x86a%rV[` \x02\x01Q\x83a\x15\x97\x83`\x04a&\xDAV[`\x0C\x81\x10a\x15\xA7Wa\x15\xA7a%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x15\xBEWa\x15\xBEa%rV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x15\xD9Wa\x15\xD9a%rV[` \x02\x01Q\x83a\x15\xEA\x83`\x05a&\xDAV[`\x0C\x81\x10a\x15\xFAWa\x15\xFAa%rV[` \x02\x01RP`\x01\x01a\x14SV[Pa\x16\x11a\x1C\x01V[_` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x16+W\xFE[P\x80a\x16JW`@Qc$\xCC\xC7\x93`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x15\x15\x98\x97PPPPPPPPV[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x03\xFEW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\x89a\x1A\x9CV[_\x84\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x80\x88\x16\x85R\x90\x83R\x81\x84 \x86Q\x90\x91\x16\x84R\x82R\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T\x81\x84\x01\x90\x81R`\x01\x83\x01T``\x83\x01R\x81R`\x02\x82\x01\x80T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x91\x94\x92\x93\x85\x84\x01\x93\x90\x92\x90\x83\x01\x82\x82\x80\x15a\x17\x1EW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x17\nW[PPP\x91\x90\x92RPP\x81QQ\x91\x92P_\x91\x15\x15\x90P\x80a\x17BWP\x81Q` \x01Q\x15\x15[\x90P\x80a\x17\xEBW_a\x17b\x87\x87\x87_\x01Q\x88`@\x01Q\x89` \x01Qa\x19\nV[\x90P\x80a\x17\x82W`@QcC\x9C\xC0\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80\x86\x01Q_\x89\x81R`\x05` \x90\x81R\x83\x82 c\xFF\xFF\xFF\xFF\x80\x8C\x16\x84R\x90\x82R\x84\x83 \x8AQ\x90\x91\x16\x83R\x81R\x92\x90 \x81Q\x80Q\x82U\x83\x01Q`\x01\x82\x01U\x82\x82\x01Q\x80Q\x92\x93\x91\x92a\x17\xDA\x92`\x02\x85\x01\x92\x01\x90a\x1A\xC6V[P\x90PP\x84`@\x01Q\x93PPa\x17\xEFV[\x81\x92P[PP\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x18\x1CWP` \x82\x01Q\x15[\x15a\x189WPP`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01_Q` a&\xEE_9_Q\x90_R\x84` \x01Qa\x18j\x91\x90a&\xC7V[a\x18\x81\x90_Q` a&\xEE_9_Q\x90_Ra&\xB4V[\x90R\x92\x91PPV[\x91\x90PV[_\x80\x80_Q` a&\xEE_9_Q\x90_R`\x03_Q` a&\xEE_9_Q\x90_R\x86_Q` a&\xEE_9_Q\x90_R\x88\x89\t\t\x08\x90P_a\x18\xFE\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R_Q` a&\xEE_9_Q\x90_Ra\x19uV[\x91\x95\x91\x94P\x90\x92PPPV[__\x83`@Q` \x01a\x19\x1D\x91\x90a##V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x8A\x81R`\x04\x84R\x82\x81 c\xFF\xFF\xFF\xFF\x80\x8C\x16\x83R\x94R\x91\x90\x91 T\x90\x92P\x90a\x19i\x90\x85\x90\x83\x90\x85\x90\x8A\x81\x16\x90a\x19\xEE\x16V[\x98\x97PPPPPPPPV[__a\x19\x7Fa\x1C\x01V[a\x19\x87a\x1C\x1FV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a\x19\xC4W\xFE[P\x82a\x19\xE3W`@Qc\xD5\x1E\xDA\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x95\x94PPPPPV[_\x83a\x19\xFB\x86\x85\x85a\x1A\x05V[\x14\x95\x94PPPPPV[_` \x84Qa\x1A\x14\x91\x90a&\xC7V[\x15a\x1A2W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82` [\x85Q\x81\x11a\x1A\x93Wa\x1AI`\x02\x85a&\xC7V[_\x03a\x1AjW\x81_R\x80\x86\x01Q` R`@_ \x91P`\x02\x84\x04\x93Pa\x1A\x81V[\x80\x86\x01Q_R\x81` R`@_ \x91P`\x02\x84\x04\x93P[a\x1A\x8C` \x82a&\xDAV[\x90Pa\x1A6V[P\x94\x93PPPPV[`@\x80Q`\x80\x81\x01\x82R_\x91\x81\x01\x82\x81R``\x82\x01\x92\x90\x92R\x90\x81\x90[\x81R` \x01``\x81RP\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a\x1A\xFFW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x1A\xFFW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x1A\xE4V[Pa\x1B\x0B\x92\x91Pa\x1C=V[P\x90V[`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01a\x1A\xB9`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80_\x81R` \x01a\x1BZa\x1B\x0FV[\x81R` \x01``\x81R` \x01a\x1B\x81`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x90R\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x1B\xD5a\x1CQV[\x81R` \x01a\x1B\x81a\x1CQV[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a\x1B\x0BW_\x81U`\x01\x01a\x1C>V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xA5Wa\x1C\xA5a\x1CoV[`@R\x90V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xA5Wa\x1C\xA5a\x1CoV[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xA5Wa\x1C\xA5a\x1CoV[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xA5Wa\x1C\xA5a\x1CoV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D9Wa\x1D9a\x1CoV[`@R\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\x89W__\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\x89W__\xFD[_`@\x82\x84\x03\x12\x15a\x1DzW__\xFD[a\x1D\x82a\x1C\x83V[\x90Pa\x1D\x8D\x82a\x1DAV[\x81Ra\x1D\x9B` \x83\x01a\x1DWV[` \x82\x01R\x92\x91PPV[_`@\x82\x84\x03\x12\x15a\x1D\xB6W__\xFD[a\x1D\xBEa\x1C\x83V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x1D\xE4W__\xFD[a\x1D\xECa\x1C\x83V[\x80`@\x84\x01\x85\x81\x11\x15a\x1D\xFDW__\xFD[\x84[\x81\x81\x10\x15a\x1E\x17W\x805\x84R` \x93\x84\x01\x93\x01a\x1D\xFFV[P\x90\x95\x94PPPPPV[_`\x80\x82\x84\x03\x12\x15a\x1E2W__\xFD[a\x1E:a\x1C\x83V[\x90Pa\x1EF\x83\x83a\x1D\xD5V[\x81Ra\x1D\x9B\x83`@\x84\x01a\x1D\xD5V[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1EmWa\x1Ema\x1CoV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x1E\x86W__\xFD[\x815a\x1E\x99a\x1E\x94\x82a\x1EUV[a\x1D\x11V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x1E\xBAW__\xFD[` \x85\x01[\x83\x81\x10\x15a\x1E\xD7W\x805\x83R` \x92\x83\x01\x92\x01a\x1E\xBFV[P\x95\x94PPPPPV[_``\x82\x84\x03\x12\x15a\x1E\xF1W__\xFD[a\x1E\xF9a\x1C\x83V[\x90Pa\x1F\x05\x83\x83a\x1D\xA6V[\x81R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\x1FW__\xFD[a\x1F+\x84\x82\x85\x01a\x1EwV[` \x83\x01RP\x92\x91PPV[_a\x01 \x82\x84\x03\x12\x15a\x1FHW__\xFD[a\x1FPa\x1C\xABV[\x90Pa\x1F[\x82a\x1DWV[\x81R` \x82\x81\x015\x90\x82\x01Ra\x1Ft\x83`@\x84\x01a\x1D\xA6V[`@\x82\x01Ra\x1F\x86\x83`\x80\x84\x01a\x1E\"V[``\x82\x01Ra\x01\0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xA4W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1F\xB4W__\xFD[\x805a\x1F\xC2a\x1E\x94\x82a\x1EUV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a\x1F\xE3W__\xFD[` \x84\x01[\x83\x81\x10\x15a \xF3W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a \x05W__\xFD[\x85\x01``\x81\x8A\x03`\x1F\x19\x01\x12\x15a \x1AW__\xFD[a \"a\x1C\xCDV[a .` \x83\x01a\x1DWV[\x81R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a HW__\xFD[\x82\x01` \x81\x01\x90`?\x01\x8B\x13a \\W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a uWa ua\x1CoV[a \x88`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1D\x11V[\x81\x81R\x8C` \x83\x85\x01\x01\x11\x15a \x9CW__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80` \x85\x01RPPP``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \xCFW__\xFD[a \xDE\x8B` \x83\x86\x01\x01a\x1E\xE1V[`@\x83\x01RP\x84RP` \x92\x83\x01\x92\x01a\x1F\xE8V[P`\x80\x85\x01RP\x91\x94\x93PPPPV[___`\x80\x84\x86\x03\x12\x15a!\x15W__\xFD[a!\x1F\x85\x85a\x1DjV[\x92P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!9W__\xFD[a!E\x86\x82\x87\x01a\x1F7V[\x92PP``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!`W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a!pW__\xFD[\x805a!~a\x1E\x94\x82a\x1EUV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a!\x9FW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a!\xD0W\x835a\xFF\xFF\x81\x16\x81\x14a!\xBFW__\xFD[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a!\xA6V[\x80\x94PPPPP\x92P\x92P\x92V[__``\x83\x85\x03\x12\x15a!\xEFW__\xFD[a!\xF9\x84\x84a\x1DjV[\x91P`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x13W__\xFD[a\"\x1F\x85\x82\x86\x01a\x1F7V[\x91PP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x1E\x17W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\"BV[____a\x01 \x85\x87\x03\x12\x15a\"tW__\xFD[\x845\x93Pa\"\x85\x86` \x87\x01a\x1D\xA6V[\x92Pa\"\x94\x86``\x87\x01a\x1E\"V[\x91Pa\"\xA3\x86`\xE0\x87\x01a\x1D\xA6V[\x90P\x92\x95\x91\x94P\x92PV[___`\x80\x84\x86\x03\x12\x15a\"\xC0W__\xFD[a\"\xCA\x85\x85a\x1DjV[\x92Pa\"\xD8`@\x85\x01a\x1DWV[\x92\x95\x92\x94PPP``\x91\x90\x91\x015\x90V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a#\x19W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\"\xFBV[P\x93\x94\x93PPPPV[` \x80\x82R\x82Q\x80Q\x83\x83\x01R\x01Q`@\x82\x01R_` \x83\x01Q``\x80\x84\x01Ra#P`\x80\x84\x01\x82a\"\xE9V[\x94\x93PPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_`@\x82\x84\x03\x12\x15a#\x9DW__\xFD[a\x03\xFB\x83\x83a\x1DjV[_`@\x82\x84\x03\x12\x15a#\xB7W__\xFD[P\x91\x90PV[____`\xC0\x85\x87\x03\x12\x15a#\xD0W__\xFD[a#\xDA\x86\x86a#\xA7V[\x93Pa#\xE8`@\x86\x01a\x1DWV[\x92P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$\x02W__\xFD[\x85\x01`\xA0\x81\x88\x03\x12\x15a$\x13W__\xFD[a$\x1Ba\x1C\xEFV[\x815\x81R` \x80\x83\x015\x90\x82\x01Ra$6\x88`@\x84\x01a\x1D\xA6V[`@\x82\x01R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$SW__\xFD[a$_\x89\x82\x85\x01a\x1EwV[``\x83\x01RP\x92Pa\"\xA3\x90P\x86`\x80\x87\x01a#\xA7V[___`\x80\x84\x86\x03\x12\x15a$\x88W__\xFD[a$\x92\x85\x85a\x1DjV[\x92P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xACW__\xFD[a$\xB8\x86\x82\x87\x01a\x1F7V[\x92PP``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xD3W__\xFD[a$\xDF\x86\x82\x87\x01a\x1EwV[\x91PP\x92P\x92P\x92V[__``\x83\x85\x03\x12\x15a$\xFAW__\xFD[a%\x04\x84\x84a\x1DjV[\x91Pa%\x12`@\x84\x01a\x1DWV[\x90P\x92P\x92\x90PV[\x80Q\x82R` \x81\x01Q` \x83\x01R_`@\x82\x01Qa%F`@\x85\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P``\x82\x01Q`\xA0`\x80\x85\x01Ra#P`\xA0\x85\x01\x82a\"\xE9V[` \x81R_a\x03\xFB` \x83\x01\x84a%\x1BV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xFEWa\x03\xFEa%\x86V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a%\xD3Wa%\xD3a%\xB1V[P\x04\x90V[_` \x82\x84\x03\x12\x15a%\xE8W__\xFD[a\x03\xFB\x82a\x1DAV[_` \x82\x84\x03\x12\x15a&\x01W__\xFD[a\x03\xFB\x82a\x1DWV[`\x01`\x01`\xA0\x1B\x03a&\x1B\x85a\x1DAV[\x16\x81Rc\xFF\xFF\xFF\xFFa&/` \x86\x01a\x1DWV[\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x83\x16`@\x82\x01R`\x80``\x82\x01R_a\x10b`\x80\x83\x01\x84a%\x1BV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a#\xB7W_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x03\xFEWa\x03\xFEa%\x86V[_` \x82\x84\x03\x12\x15a&\xA5W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\xE8W__\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\xFEWa\x03\xFEa%\x86V[_\x82a&\xD5Wa&\xD5a%\xB1V[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\x03\xFEWa\x03\xFEa%\x86V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xA8e\xAAI\0Xs\xBC\xDB\x12\xA4\xB8\xFF&A\xCD\xAF\xED\x18\x0B\xD8\"Q\xBA\xB1\xAF(\xD7RRl`dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106100cb575f3560e01c80635ddb9b5b1161008857806368d6e0811161006357806368d6e081146101d95780638481892014610218578063dd2ae1b91461022b578063eb39e68f1461023e575f5ffd5b80635ddb9b5b146101895780636141879e146101b15780636738c40b146101c4575f5ffd5b8063017d7974146100cf578063080b7150146100f75780631a18746c1461011757806326af6a3c1461014157806354fd4d50146101615780635be8727414610176575b5f5ffd5b6100e26100dd366004612103565b61025e565b60405190151581526020015b60405180910390f35b61010a6101053660046121de565b6103ef565b6040516100ee9190612229565b61012a610125366004612260565b610404565b6040805192151583529015156020830152016100ee565b61015461014f3660046122ae565b610425565b6040516100ee9190612323565b6101696104dd565b6040516100ee9190612358565b6100e26101843660046122ae565b61050d565b61019c61019736600461238d565b6105d8565b60405163ffffffff90911681526020016100ee565b61019c6101bf36600461238d565b6105fe565b6101d76101d23660046123bd565b610624565b005b6102007f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016100ee565b61020061022636600461238d565b6107f1565b6100e2610239366004612476565b61081a565b61025161024c3660046124e9565b6108ad565b6040516100ee9190612560565b5f5f61026a8585610972565b90505f61027686610b32565b5f8181526004602081815260408084208a5163ffffffff16855282528084208151608081018352815481526001820154818501528251808401845260028301548152600383015481860152818401529381018054835181860281018601909452808452969750949593949093606086019383018282801561031457602002820191905f5260205f20905b815481526020019060010190808311610300575b50505050508152505090505f8160600151905085518451146103495760405163512509d360e11b815260040160405180910390fd5b5f5b84518110156103de575f61271088838151811061036a5761036a612572565b602002602001015161ffff1684848151811061038857610388612572565b602002602001015161039a919061259a565b6103a491906125c5565b9050808683815181106103b9576103b9612572565b602002602001015110156103d5575f96505050505050506103e8565b5060010161034b565b5060019450505050505b9392505050565b60606103fb8383610972565b90505b92915050565b5f5f61041886848787600162061a80610b95565b9150915094509492505050565b61042d611a9c565b5f61043785610b32565b5f81815260056020908152604080832063ffffffff8916845282528083208784528252918290208251608081018452815481850190815260018301546060830152815260028201805485518186028101860190965280865295965090949193858401939092908301828280156104ca57602002820191905f5260205f20905b8154815260200190600101908083116104b6575b5050505050815250509150509392505050565b60606105087f0000000000000000000000000000000000000000000000000000000000000000610c5d565b905090565b5f5f61051885610b32565b5f81815260056020908152604080832063ffffffff891684528252808320878452825280832081516080810183528154818401908152600183015460608301528152600282018054845181870281018701909552808552969750949590949193858101939291908301828280156105ac57602002820191905f5260205f20905b815481526020019060010190808311610598575b50505091909252505081515191925050158015906105ce575080516020015115155b9695505050505050565b5f5f6105e383610b32565b5f9081526003602052604090205463ffffffff169392505050565b5f5f61060983610b32565b5f9081526002602052604090205463ffffffff169392505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461066d5760405163030c1b6b60e11b815260040160405180910390fd5b5f6106856106803687900387018761238d565b610b32565b5f8181526003602052604090205490915063ffffffff908116908516116106bf57604051632f20889f60e01b815260040160405180910390fd5b5f81815260046020818152604080842063ffffffff8916855282529283902086518155818701516001820155928601518051600285015581015160038401556060860151805187949361071793908501920190611ac6565b5050505f818152600360209081526040909120805463ffffffff191663ffffffff8716179055610749908301836125d8565b5f8281526001602090815260409182902080546001600160a01b0319166001600160a01b039490941693909317909255610788919084019084016125f1565b5f8281526002602052604090819020805463ffffffff191663ffffffff9390931692909217909155517f93e6bea1c9b5dce4a5c07b00261e956df2a4a253d9ab6ca070ca2037d72ada9e906107e29087908790879061260a565b60405180910390a15050505050565b5f5f6107fc83610b32565b5f908152600160205260409020546001600160a01b03169392505050565b5f5f6108268585610972565b9050825181511461084a5760405163512509d360e11b815260040160405180910390fd5b5f5b81518110156108a15783818151811061086757610867612572565b602002602001015182828151811061088157610881612572565b60200260200101511015610899575f925050506103e8565b60010161084c565b50600195945050505050565b6108b5611b0f565b5f6108bf84610b32565b5f81815260046020818152604080842063ffffffff891685528252928390208351608081018552815481526001820154818401528451808601865260028301548152600383015481850152818601529281018054855181850281018501909652808652959650929490936060860193909290919083018282801561096057602002820191905f5260205f20905b81548152602001906001019080831161094c575b50505050508152505091505092915050565b606061097c611b41565b61098584610b32565b80825283516109949190610c9a565b80515f908152600460208181526040808420875163ffffffff1685528252928390208351608081018552815481526001820154818401528451808601865260028301548152600383015481850152818601529281018054855181850281018501909652808652939491936060860193830182828015610a3057602002820191905f5260205f20905b815481526020019060010190808311610a1c575b505050919092525050506020820181905251610a5f57604051630cad17b760e31b815260040160405180910390fd5b806020015160600151516001600160401b03811115610a8057610a80611c6f565b604051908082528060200260200182016040528015610aa9578160200160208202803683370190505b5060408201525f5b81602001516060015151811015610b0d578160200151606001518181518110610adc57610adc612572565b602002602001015182604001518281518110610afa57610afa612572565b6020908102919091010152600101610ab1565b50610b188184610d95565b6060820152610b278184610ebf565b604001519392505050565b5f815f0151826020015163ffffffff16604051602001610b7d92919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b6040516020818303038152906040526103fe90612656565b5f5f5f610ba189610f2d565b90505f610bb08a89898c610fb7565b90505f610bc7610bc08a8461106b565b8b906110db565b90505f610c09610c0284610bfc6040805180820182525f80825260209182015281518083019092526001825260029082015290565b9061106b565b85906110db565b90508715610c2e57610c2582610c1d61114f565b838c8b61120f565b96509450610c4e565b610c4182610c3a61114f565b838c611423565b95508515610c4e57600194505b50505050965096945050505050565b60605f610c698361165a565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b5f8281526002602052604090205463ffffffff16801580610cca5750610cc08183612679565b63ffffffff164211155b610ce75760405163640fcd6b60e11b815260040160405180910390fd5b60405163193877e160e21b815263ffffffff831660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906364e1df8490602401602060405180830381865afa158015610d4f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d739190612695565b610d9057604051631b14174b60e01b815260040160405180910390fd5b505050565b6040805180820182525f808252602091820181905282518084019093528083529082018190525b826080015151811015610eb8575f83608001518281518110610de057610de0612572565b60200260200101519050846020015160200151815f015163ffffffff1610610e1b576040516301fa53c760e11b815260040160405180910390fd5b845184515f91610e2b9184611681565b8051909150610e3b9085906110db565b93505f5b816020015151811015610ead57866040015151811015610ea55781602001518181518110610e6f57610e6f612572565b602002602001015187604001518281518110610e8d57610e8d612572565b60200260200101818151610ea191906126b4565b9052505b600101610e3f565b505050600101610dbc565b5092915050565b5f610edf610ed084606001516117f8565b602085015160400151906110db565b90505f5f610efb84602001518486606001518760400151610404565b91509150818015610f095750805b610f265760405163439cc0cd60e01b815260040160405180910390fd5b5050505050565b604080518082019091525f80825260208201525f8080610f5a5f5160206126ee5f395f51905f52866126c7565b90505b610f668161188e565b90935091505f5160206126ee5f395f51905f528283098303610f9e576040805180820190915290815260208101919091529392505050565b5f5160206126ee5f395f51905f52600182089050610f5d565b8251602080850151845180519083015186840151805190850151875188870151604080519889018e90528801989098526060870195909552608086019390935260a085019190915260c084015260e08301526101008201526101208101919091525f907f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f00000019061014001604051602081830303815290604052805190602001205f1c61106291906126c7565b95945050505050565b604080518082019091525f8082526020820152611086611b86565b835181526020808501519082015260408082018490525f908360608460076107d05a03fa905080806110b457fe5b50806110d357604051632319df1960e11b815260040160405180910390fd5b505092915050565b604080518082019091525f80825260208201526110f6611ba4565b835181526020808501518183015283516040808401919091529084015160608301525f908360808460066107d05a03fa9050808061113057fe5b50806110d35760405163d4b68fd760e01b815260040160405180910390fd5b611157611bc2565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b6040805180820182528681526020808201869052825180840190935286835282018490525f91829190611240611be2565b5f5b60028110156113f7575f61125782600661259a565b905084826002811061126b5761126b612572565b6020020151518361127c835f6126da565b600c811061128c5761128c612572565b60200201528482600281106112a3576112a3612572565b602002015160200151838260016112ba91906126da565b600c81106112ca576112ca612572565b60200201528382600281106112e1576112e1612572565b60200201515151836112f48360026126da565b600c811061130457611304612572565b602002015283826002811061131b5761131b612572565b60200201515160016020020151836113348360036126da565b600c811061134457611344612572565b602002015283826002811061135b5761135b612572565b6020020151602001515f6002811061137557611375612572565b6020020151836113868360046126da565b600c811061139657611396612572565b60200201528382600281106113ad576113ad612572565b6020020151602001516001600281106113c8576113c8612572565b6020020151836113d98360056126da565b600c81106113e9576113e9612572565b602002015250600101611242565b50611400611c01565b5f6020826101808560088cfa9151919c9115159b50909950505050505050505050565b6040805180820182528581526020808201859052825180840190935285835282018390525f91611451611be2565b5f5b6002811015611608575f61146882600661259a565b905084826002811061147c5761147c612572565b6020020151518361148d835f6126da565b600c811061149d5761149d612572565b60200201528482600281106114b4576114b4612572565b602002015160200151838260016114cb91906126da565b600c81106114db576114db612572565b60200201528382600281106114f2576114f2612572565b60200201515151836115058360026126da565b600c811061151557611515612572565b602002015283826002811061152c5761152c612572565b60200201515160016020020151836115458360036126da565b600c811061155557611555612572565b602002015283826002811061156c5761156c612572565b6020020151602001515f6002811061158657611586612572565b6020020151836115978360046126da565b600c81106115a7576115a7612572565b60200201528382600281106115be576115be612572565b6020020151602001516001600281106115d9576115d9612572565b6020020151836115ea8360056126da565b600c81106115fa576115fa612572565b602002015250600101611453565b50611611611c01565b5f6020826101808560086107d05a03fa9050808061162b57fe5b508061164a576040516324ccc79360e21b815260040160405180910390fd5b5051151598975050505050505050565b5f60ff8216601f8111156103fe57604051632cd44ac360e21b815260040160405180910390fd5b611689611a9c565b5f84815260056020908152604080832063ffffffff808816855290835281842086519091168452825280832081516080810183528154818401908152600183015460608301528152600282018054845181870281018701909552808552919492938584019390929083018282801561171e57602002820191905f5260205f20905b81548152602001906001019080831161170a575b5050509190925250508151519192505f911515905080611742575081516020015115155b9050806117eb575f6117628787875f01518860400151896020015161190a565b9050806117825760405163439cc0cd60e01b815260040160405180910390fd5b6040808601515f8981526005602090815283822063ffffffff808c1684529082528483208a51909116835281529290208151805182558301516001820155828201518051929391926117da9260028501920190611ac6565b5090505084604001519350506117ef565b8192505b50509392505050565b604080518082019091525f8082526020820152815115801561181c57506020820151155b15611839575050604080518082019091525f808252602082015290565b6040518060400160405280835f015181526020015f5160206126ee5f395f51905f52846020015161186a91906126c7565b611881905f5160206126ee5f395f51905f526126b4565b905292915050565b919050565b5f80805f5160206126ee5f395f51905f5260035f5160206126ee5f395f51905f52865f5160206126ee5f395f51905f52888909090890505f6118fe827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f525f5160206126ee5f395f51905f52611975565b91959194509092505050565b5f5f8360405160200161191d9190612323565b60408051601f1981840301815291815281516020928301205f8a81526004845282812063ffffffff808c1683529452919091205490925090611969908590839085908a8116906119ee16565b98975050505050505050565b5f5f61197f611c01565b611987611c1f565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa925082806119c457fe5b50826119e35760405163d51edae360e01b815260040160405180910390fd5b505195945050505050565b5f836119fb868585611a05565b1495945050505050565b5f60208451611a1491906126c7565b15611a32576040516313717da960e21b815260040160405180910390fd5b8260205b85518111611a9357611a496002856126c7565b5f03611a6a57815f528086015160205260405f209150600284049350611a81565b808601515f528160205260405f2091506002840493505b611a8c6020826126da565b9050611a36565b50949350505050565b604080516080810182525f91810182815260608201929092529081905b8152602001606081525090565b828054828255905f5260205f20908101928215611aff579160200282015b82811115611aff578251825591602001919060010190611ae4565b50611b0b929150611c3d565b5090565b60405180608001604052805f81526020015f8152602001611ab960405180604001604052805f81526020015f81525090565b60405180608001604052805f8152602001611b5a611b0f565b815260200160608152602001611b8160405180604001604052805f81526020015f81525090565b905290565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b6040518060400160405280611bd5611c51565b8152602001611b81611c51565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b5b80821115611b0b575f8155600101611c3e565b60405180604001604052806002906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b0381118282101715611ca557611ca5611c6f565b60405290565b60405160a081016001600160401b0381118282101715611ca557611ca5611c6f565b604051606081016001600160401b0381118282101715611ca557611ca5611c6f565b604051608081016001600160401b0381118282101715611ca557611ca5611c6f565b604051601f8201601f191681016001600160401b0381118282101715611d3957611d39611c6f565b604052919050565b80356001600160a01b0381168114611889575f5ffd5b803563ffffffff81168114611889575f5ffd5b5f60408284031215611d7a575f5ffd5b611d82611c83565b9050611d8d82611d41565b8152611d9b60208301611d57565b602082015292915050565b5f60408284031215611db6575f5ffd5b611dbe611c83565b823581526020928301359281019290925250919050565b5f82601f830112611de4575f5ffd5b611dec611c83565b806040840185811115611dfd575f5ffd5b845b81811015611e17578035845260209384019301611dff565b509095945050505050565b5f60808284031215611e32575f5ffd5b611e3a611c83565b9050611e468383611dd5565b8152611d9b8360408401611dd5565b5f6001600160401b03821115611e6d57611e6d611c6f565b5060051b60200190565b5f82601f830112611e86575f5ffd5b8135611e99611e9482611e55565b611d11565b8082825260208201915060208360051b860101925085831115611eba575f5ffd5b602085015b83811015611ed7578035835260209283019201611ebf565b5095945050505050565b5f60608284031215611ef1575f5ffd5b611ef9611c83565b9050611f058383611da6565b815260408201356001600160401b03811115611f1f575f5ffd5b611f2b84828501611e77565b60208301525092915050565b5f6101208284031215611f48575f5ffd5b611f50611cab565b9050611f5b82611d57565b815260208281013590820152611f748360408401611da6565b6040820152611f868360808401611e22565b60608201526101008201356001600160401b03811115611fa4575f5ffd5b8201601f81018413611fb4575f5ffd5b8035611fc2611e9482611e55565b8082825260208201915060208360051b850101925086831115611fe3575f5ffd5b602084015b838110156120f35780356001600160401b03811115612005575f5ffd5b85016060818a03601f1901121561201a575f5ffd5b612022611ccd565b61202e60208301611d57565b815260408201356001600160401b03811115612048575f5ffd5b82016020810190603f018b1361205c575f5ffd5b80356001600160401b0381111561207557612075611c6f565b612088601f8201601f1916602001611d11565b8181528c602083850101111561209c575f5ffd5b816020840160208301375f6020838301015280602085015250505060608201356001600160401b038111156120cf575f5ffd5b6120de8b602083860101611ee1565b60408301525084525060209283019201611fe8565b5060808501525091949350505050565b5f5f5f60808486031215612115575f5ffd5b61211f8585611d6a565b925060408401356001600160401b03811115612139575f5ffd5b61214586828701611f37565b92505060608401356001600160401b03811115612160575f5ffd5b8401601f81018613612170575f5ffd5b803561217e611e9482611e55565b8082825260208201915060208360051b85010192508883111561219f575f5ffd5b6020840193505b828410156121d057833561ffff811681146121bf575f5ffd5b8252602093840193909101906121a6565b809450505050509250925092565b5f5f606083850312156121ef575f5ffd5b6121f98484611d6a565b915060408301356001600160401b03811115612213575f5ffd5b61221f85828601611f37565b9150509250929050565b602080825282518282018190525f918401906040840190835b81811015611e17578351835260209384019390920191600101612242565b5f5f5f5f6101208587031215612274575f5ffd5b843593506122858660208701611da6565b92506122948660608701611e22565b91506122a38660e08701611da6565b905092959194509250565b5f5f5f608084860312156122c0575f5ffd5b6122ca8585611d6a565b92506122d860408501611d57565b929592945050506060919091013590565b5f8151808452602084019350602083015f5b828110156123195781518652602095860195909101906001016122fb565b5093949350505050565b60208082528251805183830152015160408201525f602083015160608084015261235060808401826122e9565b949350505050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b5f6040828403121561239d575f5ffd5b6103fb8383611d6a565b5f604082840312156123b7575f5ffd5b50919050565b5f5f5f5f60c085870312156123d0575f5ffd5b6123da86866123a7565b93506123e860408601611d57565b925060608501356001600160401b03811115612402575f5ffd5b850160a08188031215612413575f5ffd5b61241b611cef565b81358152602080830135908201526124368860408401611da6565b604082015260808201356001600160401b03811115612453575f5ffd5b61245f89828501611e77565b60608301525092506122a3905086608087016123a7565b5f5f5f60808486031215612488575f5ffd5b6124928585611d6a565b925060408401356001600160401b038111156124ac575f5ffd5b6124b886828701611f37565b92505060608401356001600160401b038111156124d3575f5ffd5b6124df86828701611e77565b9150509250925092565b5f5f606083850312156124fa575f5ffd5b6125048484611d6a565b915061251260408401611d57565b90509250929050565b80518252602081015160208301525f6040820151612546604085018280518252602090810151910152565b50606082015160a0608085015261235060a08501826122e9565b602081525f6103fb602083018461251b565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b80820281158282048414176103fe576103fe612586565b634e487b7160e01b5f52601260045260245ffd5b5f826125d3576125d36125b1565b500490565b5f602082840312156125e8575f5ffd5b6103fb82611d41565b5f60208284031215612601575f5ffd5b6103fb82611d57565b6001600160a01b0361261b85611d41565b16815263ffffffff61262f60208601611d57565b16602082015263ffffffff83166040820152608060608201525f611062608083018461251b565b805160208083015191908110156123b7575f1960209190910360031b1b16919050565b63ffffffff81811683821601908111156103fe576103fe612586565b5f602082840312156126a5575f5ffd5b815180151581146103e8575f5ffd5b818103818111156103fe576103fe612586565b5f826126d5576126d56125b1565b500690565b808201808211156103fe576103fe61258656fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220a865aa49005873bcdb12a4b8ff2641cdafed180bd82251bab1af28d752526c6064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\0\xCBW_5`\xE0\x1C\x80c]\xDB\x9B[\x11a\0\x88W\x80ch\xD6\xE0\x81\x11a\0cW\x80ch\xD6\xE0\x81\x14a\x01\xD9W\x80c\x84\x81\x89 \x14a\x02\x18W\x80c\xDD*\xE1\xB9\x14a\x02+W\x80c\xEB9\xE6\x8F\x14a\x02>W__\xFD[\x80c]\xDB\x9B[\x14a\x01\x89W\x80caA\x87\x9E\x14a\x01\xB1W\x80cg8\xC4\x0B\x14a\x01\xC4W__\xFD[\x80c\x01}yt\x14a\0\xCFW\x80c\x08\x0BqP\x14a\0\xF7W\x80c\x1A\x18tl\x14a\x01\x17W\x80c&\xAFj<\x14a\x01AW\x80cT\xFDMP\x14a\x01aW\x80c[\xE8rt\x14a\x01vW[__\xFD[a\0\xE2a\0\xDD6`\x04a!\x03V[a\x02^V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\na\x01\x056`\x04a!\xDEV[a\x03\xEFV[`@Qa\0\xEE\x91\x90a\")V[a\x01*a\x01%6`\x04a\"`V[a\x04\x04V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\0\xEEV[a\x01Ta\x01O6`\x04a\"\xAEV[a\x04%V[`@Qa\0\xEE\x91\x90a##V[a\x01ia\x04\xDDV[`@Qa\0\xEE\x91\x90a#XV[a\0\xE2a\x01\x846`\x04a\"\xAEV[a\x05\rV[a\x01\x9Ca\x01\x976`\x04a#\x8DV[a\x05\xD8V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xEEV[a\x01\x9Ca\x01\xBF6`\x04a#\x8DV[a\x05\xFEV[a\x01\xD7a\x01\xD26`\x04a#\xBDV[a\x06$V[\0[a\x02\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xEEV[a\x02\0a\x02&6`\x04a#\x8DV[a\x07\xF1V[a\0\xE2a\x0296`\x04a$vV[a\x08\x1AV[a\x02Qa\x02L6`\x04a$\xE9V[a\x08\xADV[`@Qa\0\xEE\x91\x90a%`V[__a\x02j\x85\x85a\trV[\x90P_a\x02v\x86a\x0B2V[_\x81\x81R`\x04` \x81\x81R`@\x80\x84 \x8AQc\xFF\xFF\xFF\xFF\x16\x85R\x82R\x80\x84 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x81\x85\x01R\x82Q\x80\x84\x01\x84R`\x02\x83\x01T\x81R`\x03\x83\x01T\x81\x86\x01R\x81\x84\x01R\x93\x81\x01\x80T\x83Q\x81\x86\x02\x81\x01\x86\x01\x90\x94R\x80\x84R\x96\x97P\x94\x95\x93\x94\x90\x93``\x86\x01\x93\x83\x01\x82\x82\x80\x15a\x03\x14W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x03\0W[PPPPP\x81RPP\x90P_\x81``\x01Q\x90P\x85Q\x84Q\x14a\x03IW`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84Q\x81\x10\x15a\x03\xDEW_a'\x10\x88\x83\x81Q\x81\x10a\x03jWa\x03ja%rV[` \x02` \x01\x01Qa\xFF\xFF\x16\x84\x84\x81Q\x81\x10a\x03\x88Wa\x03\x88a%rV[` \x02` \x01\x01Qa\x03\x9A\x91\x90a%\x9AV[a\x03\xA4\x91\x90a%\xC5V[\x90P\x80\x86\x83\x81Q\x81\x10a\x03\xB9Wa\x03\xB9a%rV[` \x02` \x01\x01Q\x10\x15a\x03\xD5W_\x96PPPPPPPa\x03\xE8V[P`\x01\x01a\x03KV[P`\x01\x94PPPPP[\x93\x92PPPV[``a\x03\xFB\x83\x83a\trV[\x90P[\x92\x91PPV[__a\x04\x18\x86\x84\x87\x87`\x01b\x06\x1A\x80a\x0B\x95V[\x91P\x91P\x94P\x94\x92PPPV[a\x04-a\x1A\x9CV[_a\x047\x85a\x0B2V[_\x81\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x82R\x80\x83 \x87\x84R\x82R\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T\x81\x85\x01\x90\x81R`\x01\x83\x01T``\x83\x01R\x81R`\x02\x82\x01\x80T\x85Q\x81\x86\x02\x81\x01\x86\x01\x90\x96R\x80\x86R\x95\x96P\x90\x94\x91\x93\x85\x84\x01\x93\x90\x92\x90\x83\x01\x82\x82\x80\x15a\x04\xCAW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x04\xB6W[PPPPP\x81RPP\x91PP\x93\x92PPPV[``a\x05\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0C]V[\x90P\x90V[__a\x05\x18\x85a\x0B2V[_\x81\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x89\x16\x84R\x82R\x80\x83 \x87\x84R\x82R\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T\x81\x84\x01\x90\x81R`\x01\x83\x01T``\x83\x01R\x81R`\x02\x82\x01\x80T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x96\x97P\x94\x95\x90\x94\x91\x93\x85\x81\x01\x93\x92\x91\x90\x83\x01\x82\x82\x80\x15a\x05\xACW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x05\x98W[PPP\x91\x90\x92RPP\x81QQ\x91\x92PP\x15\x80\x15\x90a\x05\xCEWP\x80Q` \x01Q\x15\x15[\x96\x95PPPPPPV[__a\x05\xE3\x83a\x0B2V[_\x90\x81R`\x03` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x93\x92PPPV[__a\x06\t\x83a\x0B2V[_\x90\x81R`\x02` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06mW`@Qc\x03\x0C\x1Bk`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x06\x85a\x06\x806\x87\x90\x03\x87\x01\x87a#\x8DV[a\x0B2V[_\x81\x81R`\x03` R`@\x90 T\x90\x91Pc\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x85\x16\x11a\x06\xBFW`@Qc/ \x88\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x04` \x81\x81R`@\x80\x84 c\xFF\xFF\xFF\xFF\x89\x16\x85R\x82R\x92\x83\x90 \x86Q\x81U\x81\x87\x01Q`\x01\x82\x01U\x92\x86\x01Q\x80Q`\x02\x85\x01U\x81\x01Q`\x03\x84\x01U``\x86\x01Q\x80Q\x87\x94\x93a\x07\x17\x93\x90\x85\x01\x92\x01\x90a\x1A\xC6V[PPP_\x81\x81R`\x03` \x90\x81R`@\x90\x91 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x87\x16\x17\x90Ua\x07I\x90\x83\x01\x83a%\xD8V[_\x82\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x93\x90\x93\x17\x90\x92Ua\x07\x88\x91\x90\x84\x01\x90\x84\x01a%\xF1V[_\x82\x81R`\x02` R`@\x90\x81\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91UQ\x7F\x93\xE6\xBE\xA1\xC9\xB5\xDC\xE4\xA5\xC0{\0&\x1E\x95m\xF2\xA4\xA2S\xD9\xABl\xA0p\xCA 7\xD7*\xDA\x9E\x90a\x07\xE2\x90\x87\x90\x87\x90\x87\x90a&\nV[`@Q\x80\x91\x03\x90\xA1PPPPPV[__a\x07\xFC\x83a\x0B2V[_\x90\x81R`\x01` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x93\x92PPPV[__a\x08&\x85\x85a\trV[\x90P\x82Q\x81Q\x14a\x08JW`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x81Q\x81\x10\x15a\x08\xA1W\x83\x81\x81Q\x81\x10a\x08gWa\x08ga%rV[` \x02` \x01\x01Q\x82\x82\x81Q\x81\x10a\x08\x81Wa\x08\x81a%rV[` \x02` \x01\x01Q\x10\x15a\x08\x99W_\x92PPPa\x03\xE8V[`\x01\x01a\x08LV[P`\x01\x95\x94PPPPPV[a\x08\xB5a\x1B\x0FV[_a\x08\xBF\x84a\x0B2V[_\x81\x81R`\x04` \x81\x81R`@\x80\x84 c\xFF\xFF\xFF\xFF\x89\x16\x85R\x82R\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x81\x84\x01R\x84Q\x80\x86\x01\x86R`\x02\x83\x01T\x81R`\x03\x83\x01T\x81\x85\x01R\x81\x86\x01R\x92\x81\x01\x80T\x85Q\x81\x85\x02\x81\x01\x85\x01\x90\x96R\x80\x86R\x95\x96P\x92\x94\x90\x93``\x86\x01\x93\x90\x92\x90\x91\x90\x83\x01\x82\x82\x80\x15a\t`W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\tLW[PPPPP\x81RPP\x91PP\x92\x91PPV[``a\t|a\x1BAV[a\t\x85\x84a\x0B2V[\x80\x82R\x83Qa\t\x94\x91\x90a\x0C\x9AV[\x80Q_\x90\x81R`\x04` \x81\x81R`@\x80\x84 \x87Qc\xFF\xFF\xFF\xFF\x16\x85R\x82R\x92\x83\x90 \x83Q`\x80\x81\x01\x85R\x81T\x81R`\x01\x82\x01T\x81\x84\x01R\x84Q\x80\x86\x01\x86R`\x02\x83\x01T\x81R`\x03\x83\x01T\x81\x85\x01R\x81\x86\x01R\x92\x81\x01\x80T\x85Q\x81\x85\x02\x81\x01\x85\x01\x90\x96R\x80\x86R\x93\x94\x91\x93``\x86\x01\x93\x83\x01\x82\x82\x80\x15a\n0W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\n\x1CW[PPP\x91\x90\x92RPPP` \x82\x01\x81\x90RQa\n_W`@Qc\x0C\xAD\x17\xB7`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80` \x01Q``\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x80Wa\n\x80a\x1CoV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xA9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P`@\x82\x01R_[\x81` \x01Q``\x01QQ\x81\x10\x15a\x0B\rW\x81` \x01Q``\x01Q\x81\x81Q\x81\x10a\n\xDCWa\n\xDCa%rV[` \x02` \x01\x01Q\x82`@\x01Q\x82\x81Q\x81\x10a\n\xFAWa\n\xFAa%rV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\n\xB1V[Pa\x0B\x18\x81\x84a\r\x95V[``\x82\x01Ra\x0B'\x81\x84a\x0E\xBFV[`@\x01Q\x93\x92PPPV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a\x0B}\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x03\xFE\x90a&VV[___a\x0B\xA1\x89a\x0F-V[\x90P_a\x0B\xB0\x8A\x89\x89\x8Ca\x0F\xB7V[\x90P_a\x0B\xC7a\x0B\xC0\x8A\x84a\x10kV[\x8B\x90a\x10\xDBV[\x90P_a\x0C\ta\x0C\x02\x84a\x0B\xFC`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\x10kV[\x85\x90a\x10\xDBV[\x90P\x87\x15a\x0C.Wa\x0C%\x82a\x0C\x1Da\x11OV[\x83\x8C\x8Ba\x12\x0FV[\x96P\x94Pa\x0CNV[a\x0CA\x82a\x0C:a\x11OV[\x83\x8Ca\x14#V[\x95P\x85\x15a\x0CNW`\x01\x94P[PPPP\x96P\x96\x94PPPPPV[``_a\x0Ci\x83a\x16ZV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[_\x82\x81R`\x02` R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x80\x15\x80a\x0C\xCAWPa\x0C\xC0\x81\x83a&yV[c\xFF\xFF\xFF\xFF\x16B\x11\x15[a\x0C\xE7W`@Qcd\x0F\xCDk`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x198w\xE1`\xE2\x1B\x81Rc\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cd\xE1\xDF\x84\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rOW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rs\x91\x90a&\x95V[a\r\x90W`@Qc\x1B\x14\x17K`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01\x81\x90R\x82Q\x80\x84\x01\x90\x93R\x80\x83R\x90\x82\x01\x81\x90R[\x82`\x80\x01QQ\x81\x10\x15a\x0E\xB8W_\x83`\x80\x01Q\x82\x81Q\x81\x10a\r\xE0Wa\r\xE0a%rV[` \x02` \x01\x01Q\x90P\x84` \x01Q` \x01Q\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x10a\x0E\x1BW`@Qc\x01\xFAS\xC7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Q\x84Q_\x91a\x0E+\x91\x84a\x16\x81V[\x80Q\x90\x91Pa\x0E;\x90\x85\x90a\x10\xDBV[\x93P_[\x81` \x01QQ\x81\x10\x15a\x0E\xADW\x86`@\x01QQ\x81\x10\x15a\x0E\xA5W\x81` \x01Q\x81\x81Q\x81\x10a\x0EoWa\x0Eoa%rV[` \x02` \x01\x01Q\x87`@\x01Q\x82\x81Q\x81\x10a\x0E\x8DWa\x0E\x8Da%rV[` \x02` \x01\x01\x81\x81Qa\x0E\xA1\x91\x90a&\xB4V[\x90RP[`\x01\x01a\x0E?V[PPP`\x01\x01a\r\xBCV[P\x92\x91PPV[_a\x0E\xDFa\x0E\xD0\x84``\x01Qa\x17\xF8V[` \x85\x01Q`@\x01Q\x90a\x10\xDBV[\x90P__a\x0E\xFB\x84` \x01Q\x84\x86``\x01Q\x87`@\x01Qa\x04\x04V[\x91P\x91P\x81\x80\x15a\x0F\tWP\x80[a\x0F&W`@QcC\x9C\xC0\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80\x80a\x0FZ_Q` a&\xEE_9_Q\x90_R\x86a&\xC7V[\x90P[a\x0Ff\x81a\x18\x8EV[\x90\x93P\x91P_Q` a&\xEE_9_Q\x90_R\x82\x83\t\x83\x03a\x0F\x9EW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[_Q` a&\xEE_9_Q\x90_R`\x01\x82\x08\x90Pa\x0F]V[\x82Q` \x80\x85\x01Q\x84Q\x80Q\x90\x83\x01Q\x86\x84\x01Q\x80Q\x90\x85\x01Q\x87Q\x88\x87\x01Q`@\x80Q\x98\x89\x01\x8E\x90R\x88\x01\x98\x90\x98R``\x87\x01\x95\x90\x95R`\x80\x86\x01\x93\x90\x93R`\xA0\x85\x01\x91\x90\x91R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x81\x01\x91\x90\x91R_\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90a\x01@\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\x10b\x91\x90a&\xC7V[\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x10\x86a\x1B\x86V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R_\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x10\xB4W\xFE[P\x80a\x10\xD3W`@Qc#\x19\xDF\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x10\xF6a\x1B\xA4V[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R_\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x110W\xFE[P\x80a\x10\xD3W`@Qc\xD4\xB6\x8F\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11Wa\x1B\xC2V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R_\x91\x82\x91\x90a\x12@a\x1B\xE2V[_[`\x02\x81\x10\x15a\x13\xF7W_a\x12W\x82`\x06a%\x9AV[\x90P\x84\x82`\x02\x81\x10a\x12kWa\x12ka%rV[` \x02\x01QQ\x83a\x12|\x83_a&\xDAV[`\x0C\x81\x10a\x12\x8CWa\x12\x8Ca%rV[` \x02\x01R\x84\x82`\x02\x81\x10a\x12\xA3Wa\x12\xA3a%rV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x12\xBA\x91\x90a&\xDAV[`\x0C\x81\x10a\x12\xCAWa\x12\xCAa%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x12\xE1Wa\x12\xE1a%rV[` \x02\x01QQQ\x83a\x12\xF4\x83`\x02a&\xDAV[`\x0C\x81\x10a\x13\x04Wa\x13\x04a%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x13\x1BWa\x13\x1Ba%rV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x134\x83`\x03a&\xDAV[`\x0C\x81\x10a\x13DWa\x13Da%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x13[Wa\x13[a%rV[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x13uWa\x13ua%rV[` \x02\x01Q\x83a\x13\x86\x83`\x04a&\xDAV[`\x0C\x81\x10a\x13\x96Wa\x13\x96a%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x13\xADWa\x13\xADa%rV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x13\xC8Wa\x13\xC8a%rV[` \x02\x01Q\x83a\x13\xD9\x83`\x05a&\xDAV[`\x0C\x81\x10a\x13\xE9Wa\x13\xE9a%rV[` \x02\x01RP`\x01\x01a\x12BV[Pa\x14\0a\x1C\x01V[_` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R_\x91a\x14Qa\x1B\xE2V[_[`\x02\x81\x10\x15a\x16\x08W_a\x14h\x82`\x06a%\x9AV[\x90P\x84\x82`\x02\x81\x10a\x14|Wa\x14|a%rV[` \x02\x01QQ\x83a\x14\x8D\x83_a&\xDAV[`\x0C\x81\x10a\x14\x9DWa\x14\x9Da%rV[` \x02\x01R\x84\x82`\x02\x81\x10a\x14\xB4Wa\x14\xB4a%rV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x14\xCB\x91\x90a&\xDAV[`\x0C\x81\x10a\x14\xDBWa\x14\xDBa%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x14\xF2Wa\x14\xF2a%rV[` \x02\x01QQQ\x83a\x15\x05\x83`\x02a&\xDAV[`\x0C\x81\x10a\x15\x15Wa\x15\x15a%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x15,Wa\x15,a%rV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x15E\x83`\x03a&\xDAV[`\x0C\x81\x10a\x15UWa\x15Ua%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x15lWa\x15la%rV[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x15\x86Wa\x15\x86a%rV[` \x02\x01Q\x83a\x15\x97\x83`\x04a&\xDAV[`\x0C\x81\x10a\x15\xA7Wa\x15\xA7a%rV[` \x02\x01R\x83\x82`\x02\x81\x10a\x15\xBEWa\x15\xBEa%rV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x15\xD9Wa\x15\xD9a%rV[` \x02\x01Q\x83a\x15\xEA\x83`\x05a&\xDAV[`\x0C\x81\x10a\x15\xFAWa\x15\xFAa%rV[` \x02\x01RP`\x01\x01a\x14SV[Pa\x16\x11a\x1C\x01V[_` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x16+W\xFE[P\x80a\x16JW`@Qc$\xCC\xC7\x93`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x15\x15\x98\x97PPPPPPPPV[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x03\xFEW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16\x89a\x1A\x9CV[_\x84\x81R`\x05` \x90\x81R`@\x80\x83 c\xFF\xFF\xFF\xFF\x80\x88\x16\x85R\x90\x83R\x81\x84 \x86Q\x90\x91\x16\x84R\x82R\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T\x81\x84\x01\x90\x81R`\x01\x83\x01T``\x83\x01R\x81R`\x02\x82\x01\x80T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x91\x94\x92\x93\x85\x84\x01\x93\x90\x92\x90\x83\x01\x82\x82\x80\x15a\x17\x1EW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x17\nW[PPP\x91\x90\x92RPP\x81QQ\x91\x92P_\x91\x15\x15\x90P\x80a\x17BWP\x81Q` \x01Q\x15\x15[\x90P\x80a\x17\xEBW_a\x17b\x87\x87\x87_\x01Q\x88`@\x01Q\x89` \x01Qa\x19\nV[\x90P\x80a\x17\x82W`@QcC\x9C\xC0\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80\x86\x01Q_\x89\x81R`\x05` \x90\x81R\x83\x82 c\xFF\xFF\xFF\xFF\x80\x8C\x16\x84R\x90\x82R\x84\x83 \x8AQ\x90\x91\x16\x83R\x81R\x92\x90 \x81Q\x80Q\x82U\x83\x01Q`\x01\x82\x01U\x82\x82\x01Q\x80Q\x92\x93\x91\x92a\x17\xDA\x92`\x02\x85\x01\x92\x01\x90a\x1A\xC6V[P\x90PP\x84`@\x01Q\x93PPa\x17\xEFV[\x81\x92P[PP\x93\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a\x18\x1CWP` \x82\x01Q\x15[\x15a\x189WPP`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83_\x01Q\x81R` \x01_Q` a&\xEE_9_Q\x90_R\x84` \x01Qa\x18j\x91\x90a&\xC7V[a\x18\x81\x90_Q` a&\xEE_9_Q\x90_Ra&\xB4V[\x90R\x92\x91PPV[\x91\x90PV[_\x80\x80_Q` a&\xEE_9_Q\x90_R`\x03_Q` a&\xEE_9_Q\x90_R\x86_Q` a&\xEE_9_Q\x90_R\x88\x89\t\t\x08\x90P_a\x18\xFE\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R_Q` a&\xEE_9_Q\x90_Ra\x19uV[\x91\x95\x91\x94P\x90\x92PPPV[__\x83`@Q` \x01a\x19\x1D\x91\x90a##V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x8A\x81R`\x04\x84R\x82\x81 c\xFF\xFF\xFF\xFF\x80\x8C\x16\x83R\x94R\x91\x90\x91 T\x90\x92P\x90a\x19i\x90\x85\x90\x83\x90\x85\x90\x8A\x81\x16\x90a\x19\xEE\x16V[\x98\x97PPPPPPPPV[__a\x19\x7Fa\x1C\x01V[a\x19\x87a\x1C\x1FV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a\x19\xC4W\xFE[P\x82a\x19\xE3W`@Qc\xD5\x1E\xDA\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x95\x94PPPPPV[_\x83a\x19\xFB\x86\x85\x85a\x1A\x05V[\x14\x95\x94PPPPPV[_` \x84Qa\x1A\x14\x91\x90a&\xC7V[\x15a\x1A2W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82` [\x85Q\x81\x11a\x1A\x93Wa\x1AI`\x02\x85a&\xC7V[_\x03a\x1AjW\x81_R\x80\x86\x01Q` R`@_ \x91P`\x02\x84\x04\x93Pa\x1A\x81V[\x80\x86\x01Q_R\x81` R`@_ \x91P`\x02\x84\x04\x93P[a\x1A\x8C` \x82a&\xDAV[\x90Pa\x1A6V[P\x94\x93PPPPV[`@\x80Q`\x80\x81\x01\x82R_\x91\x81\x01\x82\x81R``\x82\x01\x92\x90\x92R\x90\x81\x90[\x81R` \x01``\x81RP\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a\x1A\xFFW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x1A\xFFW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x1A\xE4V[Pa\x1B\x0B\x92\x91Pa\x1C=V[P\x90V[`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01a\x1A\xB9`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[`@Q\x80`\x80\x01`@R\x80_\x81R` \x01a\x1BZa\x1B\x0FV[\x81R` \x01``\x81R` \x01a\x1B\x81`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x90R\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a\x1B\xD5a\x1CQV[\x81R` \x01a\x1B\x81a\x1CQV[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a\x1B\x0BW_\x81U`\x01\x01a\x1C>V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xA5Wa\x1C\xA5a\x1CoV[`@R\x90V[`@Q`\xA0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xA5Wa\x1C\xA5a\x1CoV[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xA5Wa\x1C\xA5a\x1CoV[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xA5Wa\x1C\xA5a\x1CoV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D9Wa\x1D9a\x1CoV[`@R\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x18\x89W__\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x18\x89W__\xFD[_`@\x82\x84\x03\x12\x15a\x1DzW__\xFD[a\x1D\x82a\x1C\x83V[\x90Pa\x1D\x8D\x82a\x1DAV[\x81Ra\x1D\x9B` \x83\x01a\x1DWV[` \x82\x01R\x92\x91PPV[_`@\x82\x84\x03\x12\x15a\x1D\xB6W__\xFD[a\x1D\xBEa\x1C\x83V[\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_\x82`\x1F\x83\x01\x12a\x1D\xE4W__\xFD[a\x1D\xECa\x1C\x83V[\x80`@\x84\x01\x85\x81\x11\x15a\x1D\xFDW__\xFD[\x84[\x81\x81\x10\x15a\x1E\x17W\x805\x84R` \x93\x84\x01\x93\x01a\x1D\xFFV[P\x90\x95\x94PPPPPV[_`\x80\x82\x84\x03\x12\x15a\x1E2W__\xFD[a\x1E:a\x1C\x83V[\x90Pa\x1EF\x83\x83a\x1D\xD5V[\x81Ra\x1D\x9B\x83`@\x84\x01a\x1D\xD5V[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1EmWa\x1Ema\x1CoV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x1E\x86W__\xFD[\x815a\x1E\x99a\x1E\x94\x82a\x1EUV[a\x1D\x11V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x1E\xBAW__\xFD[` \x85\x01[\x83\x81\x10\x15a\x1E\xD7W\x805\x83R` \x92\x83\x01\x92\x01a\x1E\xBFV[P\x95\x94PPPPPV[_``\x82\x84\x03\x12\x15a\x1E\xF1W__\xFD[a\x1E\xF9a\x1C\x83V[\x90Pa\x1F\x05\x83\x83a\x1D\xA6V[\x81R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\x1FW__\xFD[a\x1F+\x84\x82\x85\x01a\x1EwV[` \x83\x01RP\x92\x91PPV[_a\x01 \x82\x84\x03\x12\x15a\x1FHW__\xFD[a\x1FPa\x1C\xABV[\x90Pa\x1F[\x82a\x1DWV[\x81R` \x82\x81\x015\x90\x82\x01Ra\x1Ft\x83`@\x84\x01a\x1D\xA6V[`@\x82\x01Ra\x1F\x86\x83`\x80\x84\x01a\x1E\"V[``\x82\x01Ra\x01\0\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xA4W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1F\xB4W__\xFD[\x805a\x1F\xC2a\x1E\x94\x82a\x1EUV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a\x1F\xE3W__\xFD[` \x84\x01[\x83\x81\x10\x15a \xF3W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a \x05W__\xFD[\x85\x01``\x81\x8A\x03`\x1F\x19\x01\x12\x15a \x1AW__\xFD[a \"a\x1C\xCDV[a .` \x83\x01a\x1DWV[\x81R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a HW__\xFD[\x82\x01` \x81\x01\x90`?\x01\x8B\x13a \\W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a uWa ua\x1CoV[a \x88`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1D\x11V[\x81\x81R\x8C` \x83\x85\x01\x01\x11\x15a \x9CW__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80` \x85\x01RPPP``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \xCFW__\xFD[a \xDE\x8B` \x83\x86\x01\x01a\x1E\xE1V[`@\x83\x01RP\x84RP` \x92\x83\x01\x92\x01a\x1F\xE8V[P`\x80\x85\x01RP\x91\x94\x93PPPPV[___`\x80\x84\x86\x03\x12\x15a!\x15W__\xFD[a!\x1F\x85\x85a\x1DjV[\x92P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!9W__\xFD[a!E\x86\x82\x87\x01a\x1F7V[\x92PP``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!`W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a!pW__\xFD[\x805a!~a\x1E\x94\x82a\x1EUV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a!\x9FW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a!\xD0W\x835a\xFF\xFF\x81\x16\x81\x14a!\xBFW__\xFD[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a!\xA6V[\x80\x94PPPPP\x92P\x92P\x92V[__``\x83\x85\x03\x12\x15a!\xEFW__\xFD[a!\xF9\x84\x84a\x1DjV[\x91P`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x13W__\xFD[a\"\x1F\x85\x82\x86\x01a\x1F7V[\x91PP\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x1E\x17W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\"BV[____a\x01 \x85\x87\x03\x12\x15a\"tW__\xFD[\x845\x93Pa\"\x85\x86` \x87\x01a\x1D\xA6V[\x92Pa\"\x94\x86``\x87\x01a\x1E\"V[\x91Pa\"\xA3\x86`\xE0\x87\x01a\x1D\xA6V[\x90P\x92\x95\x91\x94P\x92PV[___`\x80\x84\x86\x03\x12\x15a\"\xC0W__\xFD[a\"\xCA\x85\x85a\x1DjV[\x92Pa\"\xD8`@\x85\x01a\x1DWV[\x92\x95\x92\x94PPP``\x91\x90\x91\x015\x90V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a#\x19W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\"\xFBV[P\x93\x94\x93PPPPV[` \x80\x82R\x82Q\x80Q\x83\x83\x01R\x01Q`@\x82\x01R_` \x83\x01Q``\x80\x84\x01Ra#P`\x80\x84\x01\x82a\"\xE9V[\x94\x93PPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_`@\x82\x84\x03\x12\x15a#\x9DW__\xFD[a\x03\xFB\x83\x83a\x1DjV[_`@\x82\x84\x03\x12\x15a#\xB7W__\xFD[P\x91\x90PV[____`\xC0\x85\x87\x03\x12\x15a#\xD0W__\xFD[a#\xDA\x86\x86a#\xA7V[\x93Pa#\xE8`@\x86\x01a\x1DWV[\x92P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$\x02W__\xFD[\x85\x01`\xA0\x81\x88\x03\x12\x15a$\x13W__\xFD[a$\x1Ba\x1C\xEFV[\x815\x81R` \x80\x83\x015\x90\x82\x01Ra$6\x88`@\x84\x01a\x1D\xA6V[`@\x82\x01R`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$SW__\xFD[a$_\x89\x82\x85\x01a\x1EwV[``\x83\x01RP\x92Pa\"\xA3\x90P\x86`\x80\x87\x01a#\xA7V[___`\x80\x84\x86\x03\x12\x15a$\x88W__\xFD[a$\x92\x85\x85a\x1DjV[\x92P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xACW__\xFD[a$\xB8\x86\x82\x87\x01a\x1F7V[\x92PP``\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xD3W__\xFD[a$\xDF\x86\x82\x87\x01a\x1EwV[\x91PP\x92P\x92P\x92V[__``\x83\x85\x03\x12\x15a$\xFAW__\xFD[a%\x04\x84\x84a\x1DjV[\x91Pa%\x12`@\x84\x01a\x1DWV[\x90P\x92P\x92\x90PV[\x80Q\x82R` \x81\x01Q` \x83\x01R_`@\x82\x01Qa%F`@\x85\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P``\x82\x01Q`\xA0`\x80\x85\x01Ra#P`\xA0\x85\x01\x82a\"\xE9V[` \x81R_a\x03\xFB` \x83\x01\x84a%\x1BV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xFEWa\x03\xFEa%\x86V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a%\xD3Wa%\xD3a%\xB1V[P\x04\x90V[_` \x82\x84\x03\x12\x15a%\xE8W__\xFD[a\x03\xFB\x82a\x1DAV[_` \x82\x84\x03\x12\x15a&\x01W__\xFD[a\x03\xFB\x82a\x1DWV[`\x01`\x01`\xA0\x1B\x03a&\x1B\x85a\x1DAV[\x16\x81Rc\xFF\xFF\xFF\xFFa&/` \x86\x01a\x1DWV[\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x83\x16`@\x82\x01R`\x80``\x82\x01R_a\x10b`\x80\x83\x01\x84a%\x1BV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a#\xB7W_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x03\xFEWa\x03\xFEa%\x86V[_` \x82\x84\x03\x12\x15a&\xA5W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x03\xE8W__\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\xFEWa\x03\xFEa%\x86V[_\x82a&\xD5Wa&\xD5a%\xB1V[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\x03\xFEWa\x03\xFEa%\x86V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \xA8e\xAAI\0Xs\xBC\xDB\x12\xA4\xB8\xFF&A\xCD\xAF\xED\x18\x0B\xD8\"Q\xBA\xB1\xAF(\xD7RRl`dsolcC\0\x08\x1B\x003",
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
    /**Custom error with signature `ECAddFailed()` and selector `0xd4b68fd7`.
    ```solidity
    error ECAddFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECAddFailed;
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
        impl ::core::convert::From<ECAddFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECAddFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECAddFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECAddFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECAddFailed()";
            const SELECTOR: [u8; 4] = [212u8, 182u8, 143u8, 215u8];
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
    /**Custom error with signature `ECMulFailed()` and selector `0x4633be32`.
    ```solidity
    error ECMulFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECMulFailed;
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
        impl ::core::convert::From<ECMulFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECMulFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECMulFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECMulFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECMulFailed()";
            const SELECTOR: [u8; 4] = [70u8, 51u8, 190u8, 50u8];
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
    /**Custom error with signature `ECPairingFailed()` and selector `0x93331e4c`.
    ```solidity
    error ECPairingFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECPairingFailed;
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
        impl ::core::convert::From<ECPairingFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECPairingFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECPairingFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECPairingFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECPairingFailed()";
            const SELECTOR: [u8; 4] = [147u8, 51u8, 30u8, 76u8];
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
    /**Custom error with signature `ExpModFailed()` and selector `0xd51edae3`.
    ```solidity
    error ExpModFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpModFailed;
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
        impl ::core::convert::From<ExpModFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ExpModFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpModFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpModFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpModFailed()";
            const SELECTOR: [u8; 4] = [213u8, 30u8, 218u8, 227u8];
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
    /**Custom error with signature `InvalidOperatorIndex()` and selector `0x03f4a78e`.
    ```solidity
    error InvalidOperatorIndex();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidOperatorIndex;
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
        impl ::core::convert::From<InvalidOperatorIndex> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidOperatorIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOperatorIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidOperatorIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidOperatorIndex()";
            const SELECTOR: [u8; 4] = [3u8, 244u8, 167u8, 142u8];
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
    /**Event with signature `TableUpdated((address,uint32),uint32,(bytes32,uint256,(uint256,uint256),uint256[]))` and selector `0x93e6bea1c9b5dce4a5c07b00261e956df2a4a253d9ab6ca070ca2037d72ada9e`.
    ```solidity
    event TableUpdated(OperatorSet operatorSet, uint32 referenceTimestamp, IOperatorTableCalculatorTypes.BN254OperatorSetInfo operatorSetInfo);
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
        pub operatorSetInfo: <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
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
                IOperatorTableCalculatorTypes::BN254OperatorSetInfo,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TableUpdated((address,uint32),uint32,(bytes32,uint256,(uint256,uint256),uint256[]))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    147u8, 230u8, 190u8, 161u8, 201u8, 181u8, 220u8, 228u8, 165u8, 192u8, 123u8,
                    0u8, 38u8, 30u8, 149u8, 109u8, 242u8, 164u8, 162u8, 83u8, 217u8, 171u8, 108u8,
                    160u8, 112u8, 202u8, 32u8, 55u8, 215u8, 42u8, 218u8, 158u8,
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
                    operatorSetInfo: data.2,
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTimestamp),
                    <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetInfo,
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
    /**Function with signature `getNonsignerOperatorInfo((address,uint32),uint32,uint256)` and selector `0x26af6a3c`.
    ```solidity
    function getNonsignerOperatorInfo(OperatorSet memory operatorSet, uint32 referenceTimestamp, uint256 operatorIndex) external view returns (IOperatorTableCalculatorTypes.BN254OperatorInfo memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNonsignerOperatorInfoCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub operatorIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getNonsignerOperatorInfo((address,uint32),uint32,uint256)`](getNonsignerOperatorInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNonsignerOperatorInfoReturn {
        #[allow(missing_docs)]
        pub _0: <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getNonsignerOperatorInfoCall> for UnderlyingRustTuple<'_> {
                fn from(value: getNonsignerOperatorInfoCall) -> Self {
                    (
                        value.operatorSet,
                        value.referenceTimestamp,
                        value.operatorIndex,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getNonsignerOperatorInfoCall {
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
            type UnderlyingSolTuple<'a> = (IOperatorTableCalculatorTypes::BN254OperatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getNonsignerOperatorInfoReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getNonsignerOperatorInfoReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getNonsignerOperatorInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getNonsignerOperatorInfoCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IOperatorTableCalculatorTypes::BN254OperatorInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getNonsignerOperatorInfo((address,uint32),uint32,uint256)";
            const SELECTOR: [u8; 4] = [38u8, 175u8, 106u8, 60u8];
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
                    <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getNonsignerOperatorInfoReturn = r.into();
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
                    let r: getNonsignerOperatorInfoReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperatorSetInfo((address,uint32),uint32)` and selector `0xeb39e68f`.
    ```solidity
    function getOperatorSetInfo(OperatorSet memory operatorSet, uint32 referenceTimestamp) external view returns (IOperatorTableCalculatorTypes.BN254OperatorSetInfo memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetInfoCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorSetInfo((address,uint32),uint32)`](getOperatorSetInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetInfoReturn {
        #[allow(missing_docs)]
        pub _0: <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorSetInfoCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetInfoCall) -> Self {
                    (value.operatorSet, value.referenceTimestamp)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetInfoCall {
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
            type UnderlyingSolTuple<'a> = (IOperatorTableCalculatorTypes::BN254OperatorSetInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorSetInfoReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetInfoReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSetInfoCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<32>);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IOperatorTableCalculatorTypes::BN254OperatorSetInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSetInfo((address,uint32),uint32)";
            const SELECTOR: [u8; 4] = [235u8, 57u8, 230u8, 143u8];
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
                    <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getOperatorSetInfoReturn = r.into();
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
                    let r: getOperatorSetInfoReturn = r.into();
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
    /**Function with signature `isNonsignerCached((address,uint32),uint32,uint256)` and selector `0x5be87274`.
    ```solidity
    function isNonsignerCached(OperatorSet memory operatorSet, uint32 referenceTimestamp, uint256 operatorIndex) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isNonsignerCachedCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub operatorIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isNonsignerCached((address,uint32),uint32,uint256)`](isNonsignerCachedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isNonsignerCachedReturn {
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
            impl ::core::convert::From<isNonsignerCachedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isNonsignerCachedCall) -> Self {
                    (
                        value.operatorSet,
                        value.referenceTimestamp,
                        value.operatorIndex,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isNonsignerCachedCall {
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
            impl ::core::convert::From<isNonsignerCachedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isNonsignerCachedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isNonsignerCachedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isNonsignerCachedCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isNonsignerCached((address,uint32),uint32,uint256)";
            const SELECTOR: [u8; 4] = [91u8, 232u8, 114u8, 116u8];
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
                (<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: isNonsignerCachedReturn = r.into();
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
                    let r: isNonsignerCachedReturn = r.into();
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
    /**Function with signature `trySignatureVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))` and selector `0x1a18746c`.
    ```solidity
    function trySignatureVerification(bytes32 msgHash, BN254.G1Point memory aggPubkey, BN254.G2Point memory apkG2, BN254.G1Point memory signature) external view returns (bool pairingSuccessful, bool signatureValid);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureVerificationCall {
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub aggPubkey: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub signature: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`trySignatureVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))`](trySignatureVerificationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureVerificationReturn {
        #[allow(missing_docs)]
        pub pairingSuccessful: bool,
        #[allow(missing_docs)]
        pub signatureValid: bool,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                <BN254::G2Point as alloy::sol_types::SolType>::RustType,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<trySignatureVerificationCall> for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureVerificationCall) -> Self {
                    (value.msgHash, value.aggPubkey, value.apkG2, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for trySignatureVerificationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        aggPubkey: tuple.1,
                        apkG2: tuple.2,
                        signature: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, bool);
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
            impl ::core::convert::From<trySignatureVerificationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureVerificationReturn) -> Self {
                    (value.pairingSuccessful, value.signatureValid)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for trySignatureVerificationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pairingSuccessful: tuple.0,
                        signatureValid: tuple.1,
                    }
                }
            }
        }
        impl trySignatureVerificationReturn {
            fn _tokenize(
                &self,
            ) -> <trySignatureVerificationCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.pairingSuccessful,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.signatureValid,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for trySignatureVerificationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = trySignatureVerificationReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "trySignatureVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))";
            const SELECTOR: [u8; 4] = [26u8, 24u8, 116u8, 108u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(
                        &self.aggPubkey,
                    ),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                trySignatureVerificationReturn::_tokenize(ret)
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
    /**Function with signature `updateOperatorTable((address,uint32),uint32,(bytes32,uint256,(uint256,uint256),uint256[]),(address,uint32))` and selector `0x6738c40b`.
    ```solidity
    function updateOperatorTable(OperatorSet memory operatorSet, uint32 referenceTimestamp, IOperatorTableCalculatorTypes.BN254OperatorSetInfo memory operatorSetInfo, ICrossChainRegistryTypes.OperatorSetConfig memory operatorSetConfig) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorTableCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub operatorSetInfo: <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operatorSetConfig: <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`updateOperatorTable((address,uint32),uint32,(bytes32,uint256,(uint256,uint256),uint256[]),(address,uint32))`](updateOperatorTableCall) function.
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
                IOperatorTableCalculatorTypes::BN254OperatorSetInfo,
                ICrossChainRegistryTypes::OperatorSetConfig,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                u32,
                <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
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
                        value.operatorSetInfo,
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
                        operatorSetInfo: tuple.2,
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
                IOperatorTableCalculatorTypes::BN254OperatorSetInfo,
                ICrossChainRegistryTypes::OperatorSetConfig,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorTableReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorTable((address,uint32),uint32,(bytes32,uint256,(uint256,uint256),uint256[]),(address,uint32))";
            const SELECTOR: [u8; 4] = [103u8, 56u8, 196u8, 11u8];
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
                    <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetInfo,
                    ),
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
    #[derive(serde::Serialize, serde::Deserialize)]
    /**Function with signature `verifyCertificate((address,uint32),(uint32,bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint32,bytes,((uint256,uint256),uint256[]))[]))` and selector `0x080b7150`.
    ```solidity
    function verifyCertificate(OperatorSet memory operatorSet, IBN254CertificateVerifierTypes.BN254Certificate memory cert) external returns (uint256[] memory signedStakes);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCertificateCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub cert: <IBN254CertificateVerifierTypes::BN254Certificate as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`verifyCertificate((address,uint32),(uint32,bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint32,bytes,((uint256,uint256),uint256[]))[]))`](verifyCertificateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCertificateReturn {
        #[allow(missing_docs)]
        pub signedStakes:
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
            type UnderlyingSolTuple<'a> = (
                OperatorSet,
                IBN254CertificateVerifierTypes::BN254Certificate,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                <IBN254CertificateVerifierTypes::BN254Certificate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<verifyCertificateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCertificateReturn) -> Self {
                    (value.signedStakes,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCertificateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        signedStakes: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyCertificateCall {
            type Parameters<'a> = (
                OperatorSet,
                IBN254CertificateVerifierTypes::BN254Certificate,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyCertificate((address,uint32),(uint32,bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint32,bytes,((uint256,uint256),uint256[]))[]))";
            const SELECTOR: [u8; 4] = [8u8, 11u8, 113u8, 80u8];
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
                    <IBN254CertificateVerifierTypes::BN254Certificate as alloy_sol_types::SolType>::tokenize(
                        &self.cert,
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
                        let r: verifyCertificateReturn = r.into();
                        r.signedStakes
                    },
                )
            }
            #[inline]
            fn abi_decode_returns_validate(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence_validate(
                    data,
                )
                .map(|r| {
                    let r: verifyCertificateReturn = r.into();
                    r.signedStakes
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    /**Function with signature `verifyCertificateNominal((address,uint32),(uint32,bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint32,bytes,((uint256,uint256),uint256[]))[]),uint256[])` and selector `0xdd2ae1b9`.
    ```solidity
    function verifyCertificateNominal(OperatorSet memory operatorSet, IBN254CertificateVerifierTypes.BN254Certificate memory cert, uint256[] memory totalStakeNominalThresholds) external returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCertificateNominalCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub cert: <IBN254CertificateVerifierTypes::BN254Certificate as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub totalStakeNominalThresholds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`verifyCertificateNominal((address,uint32),(uint32,bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint32,bytes,((uint256,uint256),uint256[]))[]),uint256[])`](verifyCertificateNominalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCertificateNominalReturn {
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
            type UnderlyingSolTuple<'a> = (
                OperatorSet,
                IBN254CertificateVerifierTypes::BN254Certificate,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                <IBN254CertificateVerifierTypes::BN254Certificate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<verifyCertificateNominalReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCertificateNominalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCertificateNominalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyCertificateNominalCall {
            type Parameters<'a> = (
                OperatorSet,
                IBN254CertificateVerifierTypes::BN254Certificate,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyCertificateNominal((address,uint32),(uint32,bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint32,bytes,((uint256,uint256),uint256[]))[]),uint256[])";
            const SELECTOR: [u8; 4] = [221u8, 42u8, 225u8, 185u8];
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
                    <IBN254CertificateVerifierTypes::BN254Certificate as alloy_sol_types::SolType>::tokenize(
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
                (<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: verifyCertificateNominalReturn = r.into();
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
                    let r: verifyCertificateNominalReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    /**Function with signature `verifyCertificateProportion((address,uint32),(uint32,bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint32,bytes,((uint256,uint256),uint256[]))[]),uint16[])` and selector `0x017d7974`.
    ```solidity
    function verifyCertificateProportion(OperatorSet memory operatorSet, IBN254CertificateVerifierTypes.BN254Certificate memory cert, uint16[] memory totalStakeProportionThresholds) external returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCertificateProportionCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub cert: <IBN254CertificateVerifierTypes::BN254Certificate as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub totalStakeProportionThresholds: alloy::sol_types::private::Vec<u16>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`verifyCertificateProportion((address,uint32),(uint32,bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint32,bytes,((uint256,uint256),uint256[]))[]),uint16[])`](verifyCertificateProportionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCertificateProportionReturn {
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
            type UnderlyingSolTuple<'a> = (
                OperatorSet,
                IBN254CertificateVerifierTypes::BN254Certificate,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<16>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                <IBN254CertificateVerifierTypes::BN254Certificate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<verifyCertificateProportionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCertificateProportionReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCertificateProportionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyCertificateProportionCall {
            type Parameters<'a> = (
                OperatorSet,
                IBN254CertificateVerifierTypes::BN254Certificate,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<16>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyCertificateProportion((address,uint32),(uint32,bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint32,bytes,((uint256,uint256),uint256[]))[]),uint16[])";
            const SELECTOR: [u8; 4] = [1u8, 125u8, 121u8, 116u8];
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
                    <IBN254CertificateVerifierTypes::BN254Certificate as alloy_sol_types::SolType>::tokenize(
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
                (<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: verifyCertificateProportionReturn = r.into();
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
                    let r: verifyCertificateProportionReturn = r.into();
                    r._0
                })
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
    ///Container for all the [`BN254CertificateVerifier`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum BN254CertificateVerifierCalls {
        #[allow(missing_docs)]
        getNonsignerOperatorInfo(getNonsignerOperatorInfoCall),
        #[allow(missing_docs)]
        getOperatorSetInfo(getOperatorSetInfoCall),
        #[allow(missing_docs)]
        getOperatorSetOwner(getOperatorSetOwnerCall),
        #[allow(missing_docs)]
        isNonsignerCached(isNonsignerCachedCall),
        #[allow(missing_docs)]
        latestReferenceTimestamp(latestReferenceTimestampCall),
        #[allow(missing_docs)]
        maxOperatorTableStaleness(maxOperatorTableStalenessCall),
        #[allow(missing_docs)]
        operatorTableUpdater(operatorTableUpdaterCall),
        #[allow(missing_docs)]
        trySignatureVerification(trySignatureVerificationCall),
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
    impl BN254CertificateVerifierCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 125u8, 121u8, 116u8],
            [8u8, 11u8, 113u8, 80u8],
            [26u8, 24u8, 116u8, 108u8],
            [38u8, 175u8, 106u8, 60u8],
            [84u8, 253u8, 77u8, 80u8],
            [91u8, 232u8, 114u8, 116u8],
            [93u8, 219u8, 155u8, 91u8],
            [97u8, 65u8, 135u8, 158u8],
            [103u8, 56u8, 196u8, 11u8],
            [104u8, 214u8, 224u8, 129u8],
            [132u8, 129u8, 137u8, 32u8],
            [221u8, 42u8, 225u8, 185u8],
            [235u8, 57u8, 230u8, 143u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BN254CertificateVerifierCalls {
        const NAME: &'static str = "BN254CertificateVerifierCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::getNonsignerOperatorInfo(_) => {
                    <getNonsignerOperatorInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSetInfo(_) => {
                    <getOperatorSetInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSetOwner(_) => {
                    <getOperatorSetOwnerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isNonsignerCached(_) => {
                    <isNonsignerCachedCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::trySignatureVerification(_) => {
                    <trySignatureVerificationCall as alloy_sol_types::SolCall>::SELECTOR
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
                -> alloy_sol_types::Result<BN254CertificateVerifierCalls>] = &[
                {
                    fn verifyCertificateProportion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <verifyCertificateProportionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BN254CertificateVerifierCalls::verifyCertificateProportion,
                            )
                    }
                    verifyCertificateProportion
                },
                {
                    fn verifyCertificate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <verifyCertificateCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierCalls::verifyCertificate)
                    }
                    verifyCertificate
                },
                {
                    fn trySignatureVerification(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <trySignatureVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(BN254CertificateVerifierCalls::trySignatureVerification)
                    }
                    trySignatureVerification
                },
                {
                    fn getNonsignerOperatorInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <getNonsignerOperatorInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(BN254CertificateVerifierCalls::getNonsignerOperatorInfo)
                    }
                    getNonsignerOperatorInfo
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierCalls::version)
                    }
                    version
                },
                {
                    fn isNonsignerCached(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <isNonsignerCachedCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierCalls::isNonsignerCached)
                    }
                    isNonsignerCached
                },
                {
                    fn latestReferenceTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <latestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(BN254CertificateVerifierCalls::latestReferenceTimestamp)
                    }
                    latestReferenceTimestamp
                },
                {
                    fn maxOperatorTableStaleness(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <maxOperatorTableStalenessCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(BN254CertificateVerifierCalls::maxOperatorTableStaleness)
                    }
                    maxOperatorTableStaleness
                },
                {
                    fn updateOperatorTable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierCalls::updateOperatorTable)
                    }
                    updateOperatorTable
                },
                {
                    fn operatorTableUpdater(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <operatorTableUpdaterCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierCalls::operatorTableUpdater)
                    }
                    operatorTableUpdater
                },
                {
                    fn getOperatorSetOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <getOperatorSetOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierCalls::getOperatorSetOwner)
                    }
                    getOperatorSetOwner
                },
                {
                    fn verifyCertificateNominal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <verifyCertificateNominalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(BN254CertificateVerifierCalls::verifyCertificateNominal)
                    }
                    verifyCertificateNominal
                },
                {
                    fn getOperatorSetInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <getOperatorSetInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierCalls::getOperatorSetInfo)
                    }
                    getOperatorSetInfo
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
                BN254CertificateVerifierCalls,
            >] = &[
                {
                    fn verifyCertificateProportion(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <verifyCertificateProportionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BN254CertificateVerifierCalls::verifyCertificateProportion,
                            )
                    }
                    verifyCertificateProportion
                },
                {
                    fn verifyCertificate(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <verifyCertificateCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BN254CertificateVerifierCalls::verifyCertificate)
                    }
                    verifyCertificate
                },
                {
                    fn trySignatureVerification(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <trySignatureVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BN254CertificateVerifierCalls::trySignatureVerification)
                    }
                    trySignatureVerification
                },
                {
                    fn getNonsignerOperatorInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <getNonsignerOperatorInfoCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BN254CertificateVerifierCalls::getNonsignerOperatorInfo)
                    }
                    getNonsignerOperatorInfo
                },
                {
                    fn version(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(BN254CertificateVerifierCalls::version)
                    }
                    version
                },
                {
                    fn isNonsignerCached(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <isNonsignerCachedCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BN254CertificateVerifierCalls::isNonsignerCached)
                    }
                    isNonsignerCached
                },
                {
                    fn latestReferenceTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <latestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BN254CertificateVerifierCalls::latestReferenceTimestamp)
                    }
                    latestReferenceTimestamp
                },
                {
                    fn maxOperatorTableStaleness(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <maxOperatorTableStalenessCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BN254CertificateVerifierCalls::maxOperatorTableStaleness,
                            )
                    }
                    maxOperatorTableStaleness
                },
                {
                    fn updateOperatorTable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BN254CertificateVerifierCalls::updateOperatorTable)
                    }
                    updateOperatorTable
                },
                {
                    fn operatorTableUpdater(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <operatorTableUpdaterCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BN254CertificateVerifierCalls::operatorTableUpdater)
                    }
                    operatorTableUpdater
                },
                {
                    fn getOperatorSetOwner(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <getOperatorSetOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BN254CertificateVerifierCalls::getOperatorSetOwner)
                    }
                    getOperatorSetOwner
                },
                {
                    fn verifyCertificateNominal(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <verifyCertificateNominalCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BN254CertificateVerifierCalls::verifyCertificateNominal)
                    }
                    verifyCertificateNominal
                },
                {
                    fn getOperatorSetInfo(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierCalls>
                    {
                        <getOperatorSetInfoCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BN254CertificateVerifierCalls::getOperatorSetInfo)
                    }
                    getOperatorSetInfo
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
                Self::getNonsignerOperatorInfo(inner) => {
                    <getNonsignerOperatorInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSetInfo(inner) => {
                    <getOperatorSetInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getOperatorSetOwner(inner) => {
                    <getOperatorSetOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isNonsignerCached(inner) => {
                    <isNonsignerCachedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                    <operatorTableUpdaterCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::trySignatureVerification(inner) => {
                    <trySignatureVerificationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorTable(inner) => {
                    <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::verifyCertificate(inner) => {
                    <verifyCertificateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::getNonsignerOperatorInfo(inner) => {
                    <getNonsignerOperatorInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getOperatorSetInfo(inner) => {
                    <getOperatorSetInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getOperatorSetOwner(inner) => {
                    <getOperatorSetOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::isNonsignerCached(inner) => {
                    <isNonsignerCachedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::latestReferenceTimestamp(inner) => {
                    <latestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::maxOperatorTableStaleness(inner) => {
                    <maxOperatorTableStalenessCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::operatorTableUpdater(inner) => {
                    <operatorTableUpdaterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::trySignatureVerification(inner) => {
                    <trySignatureVerificationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::updateOperatorTable(inner) => {
                    <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::verifyCertificate(inner) => {
                    <verifyCertificateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::verifyCertificateNominal(inner) => {
                    <verifyCertificateNominalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::verifyCertificateProportion(inner) => {
                    <verifyCertificateProportionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`BN254CertificateVerifier`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum BN254CertificateVerifierErrors {
        #[allow(missing_docs)]
        ArrayLengthMismatch(ArrayLengthMismatch),
        #[allow(missing_docs)]
        CertificateStale(CertificateStale),
        #[allow(missing_docs)]
        ECAddFailed(ECAddFailed),
        #[allow(missing_docs)]
        ECMulFailed(ECMulFailed),
        #[allow(missing_docs)]
        ECPairingFailed(ECPairingFailed),
        #[allow(missing_docs)]
        ExpModFailed(ExpModFailed),
        #[allow(missing_docs)]
        InvalidOperatorIndex(InvalidOperatorIndex),
        #[allow(missing_docs)]
        InvalidProofLength(InvalidProofLength),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        OnlyTableUpdater(OnlyTableUpdater),
        #[allow(missing_docs)]
        ReferenceTimestampDoesNotExist(ReferenceTimestampDoesNotExist),
        #[allow(missing_docs)]
        RootDisabled(RootDisabled),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
        #[allow(missing_docs)]
        TableUpdateStale(TableUpdateStale),
        #[allow(missing_docs)]
        VerificationFailed(VerificationFailed),
    }
    #[automatically_derived]
    impl BN254CertificateVerifierErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 244u8, 167u8, 142u8],
            [6u8, 24u8, 54u8, 214u8],
            [27u8, 20u8, 23u8, 75u8],
            [47u8, 32u8, 136u8, 159u8],
            [48u8, 90u8, 39u8, 169u8],
            [67u8, 156u8, 192u8, 205u8],
            [70u8, 51u8, 190u8, 50u8],
            [77u8, 197u8, 246u8, 164u8],
            [101u8, 104u8, 189u8, 184u8],
            [147u8, 51u8, 30u8, 76u8],
            [162u8, 74u8, 19u8, 166u8],
            [179u8, 81u8, 43u8, 12u8],
            [200u8, 31u8, 154u8, 214u8],
            [212u8, 182u8, 143u8, 215u8],
            [213u8, 30u8, 218u8, 227u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BN254CertificateVerifierErrors {
        const NAME: &'static str = "BN254CertificateVerifierErrors";
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
                Self::ECAddFailed(_) => <ECAddFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ECMulFailed(_) => <ECMulFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ECPairingFailed(_) => {
                    <ECPairingFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpModFailed(_) => <ExpModFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidOperatorIndex(_) => {
                    <InvalidOperatorIndex as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidProofLength(_) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyTableUpdater(_) => {
                    <OnlyTableUpdater as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReferenceTimestampDoesNotExist(_) => {
                    <ReferenceTimestampDoesNotExist as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RootDisabled(_) => <RootDisabled as alloy_sol_types::SolError>::SELECTOR,
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
                BN254CertificateVerifierErrors,
            >] = &[
                {
                    fn InvalidOperatorIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <InvalidOperatorIndex as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::InvalidOperatorIndex)
                    }
                    InvalidOperatorIndex
                },
                {
                    fn OnlyTableUpdater(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <OnlyTableUpdater as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::OnlyTableUpdater)
                    }
                    OnlyTableUpdater
                },
                {
                    fn RootDisabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <RootDisabled as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::RootDisabled)
                    }
                    RootDisabled
                },
                {
                    fn TableUpdateStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <TableUpdateStale as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::TableUpdateStale)
                    }
                    TableUpdateStale
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn VerificationFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <VerificationFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::VerificationFailed)
                    }
                    VerificationFailed
                },
                {
                    fn ECMulFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::ECMulFailed)
                    }
                    ECMulFailed
                },
                {
                    fn InvalidProofLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <InvalidProofLength as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::InvalidProofLength)
                    }
                    InvalidProofLength
                },
                {
                    fn ReferenceTimestampDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <ReferenceTimestampDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                BN254CertificateVerifierErrors::ReferenceTimestampDoesNotExist,
                            )
                    }
                    ReferenceTimestampDoesNotExist
                },
                {
                    fn ECPairingFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <ECPairingFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::ECPairingFailed)
                    }
                    ECPairingFailed
                },
                {
                    fn ArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::ArrayLengthMismatch)
                    }
                    ArrayLengthMismatch
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn CertificateStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <CertificateStale as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::CertificateStale)
                    }
                    CertificateStale
                },
                {
                    fn ECAddFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(BN254CertificateVerifierErrors::ExpModFailed)
                    }
                    ExpModFailed
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
                BN254CertificateVerifierErrors,
            >] = &[
                {
                    fn InvalidOperatorIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <InvalidOperatorIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(BN254CertificateVerifierErrors::InvalidOperatorIndex)
                    }
                    InvalidOperatorIndex
                },
                {
                    fn OnlyTableUpdater(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <OnlyTableUpdater as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(BN254CertificateVerifierErrors::OnlyTableUpdater)
                    }
                    OnlyTableUpdater
                },
                {
                    fn RootDisabled(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <RootDisabled as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(BN254CertificateVerifierErrors::RootDisabled)
                    }
                    RootDisabled
                },
                {
                    fn TableUpdateStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <TableUpdateStale as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(BN254CertificateVerifierErrors::TableUpdateStale)
                    }
                    TableUpdateStale
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(BN254CertificateVerifierErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn VerificationFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <VerificationFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(BN254CertificateVerifierErrors::VerificationFailed)
                    }
                    VerificationFailed
                },
                {
                    fn ECMulFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(BN254CertificateVerifierErrors::ECMulFailed)
                    }
                    ECMulFailed
                },
                {
                    fn InvalidProofLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <InvalidProofLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(BN254CertificateVerifierErrors::InvalidProofLength)
                    }
                    InvalidProofLength
                },
                {
                    fn ReferenceTimestampDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <ReferenceTimestampDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                BN254CertificateVerifierErrors::ReferenceTimestampDoesNotExist,
                            )
                    }
                    ReferenceTimestampDoesNotExist
                },
                {
                    fn ECPairingFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <ECPairingFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(BN254CertificateVerifierErrors::ECPairingFailed)
                    }
                    ECPairingFailed
                },
                {
                    fn ArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(BN254CertificateVerifierErrors::ArrayLengthMismatch)
                    }
                    ArrayLengthMismatch
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(BN254CertificateVerifierErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn CertificateStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <CertificateStale as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(BN254CertificateVerifierErrors::CertificateStale)
                    }
                    CertificateStale
                },
                {
                    fn ECAddFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(BN254CertificateVerifierErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<BN254CertificateVerifierErrors>
                    {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(BN254CertificateVerifierErrors::ExpModFailed)
                    }
                    ExpModFailed
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
                Self::ECAddFailed(inner) => {
                    <ECAddFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ECMulFailed(inner) => {
                    <ECMulFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ECPairingFailed(inner) => {
                    <ECPairingFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidOperatorIndex(inner) => {
                    <InvalidOperatorIndex as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidProofLength(inner) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyTableUpdater(inner) => {
                    <OnlyTableUpdater as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ReferenceTimestampDoesNotExist(inner) => {
                    <ReferenceTimestampDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RootDisabled(inner) => {
                    <RootDisabled as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::ECAddFailed(inner) => {
                    <ECAddFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ECMulFailed(inner) => {
                    <ECMulFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ECPairingFailed(inner) => {
                    <ECPairingFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidOperatorIndex(inner) => {
                    <InvalidOperatorIndex as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidProofLength(inner) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyTableUpdater(inner) => {
                    <OnlyTableUpdater as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ReferenceTimestampDoesNotExist(inner) => {
                    <ReferenceTimestampDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::RootDisabled(inner) => {
                    <RootDisabled as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
    ///Container for all the [`BN254CertificateVerifier`](self) events.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum BN254CertificateVerifierEvents {
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
    impl BN254CertificateVerifierEvents {
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
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                128u8, 109u8, 195u8, 103u8, 9u8, 92u8, 11u8, 175u8, 149u8, 61u8, 113u8, 68u8,
                183u8, 196u8, 55u8, 98u8, 97u8, 103u8, 94u8, 224u8, 180u8, 224u8, 218u8, 39u8,
                97u8, 228u8, 54u8, 115u8, 5u8, 28u8, 115u8, 117u8,
            ],
            [
                147u8, 230u8, 190u8, 161u8, 201u8, 181u8, 220u8, 228u8, 165u8, 192u8, 123u8, 0u8,
                38u8, 30u8, 149u8, 109u8, 242u8, 164u8, 162u8, 83u8, 217u8, 171u8, 108u8, 160u8,
                112u8, 202u8, 32u8, 55u8, 215u8, 42u8, 218u8, 158u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for BN254CertificateVerifierEvents {
        const NAME: &'static str = "BN254CertificateVerifierEvents";
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
    impl alloy_sol_types::private::IntoLogData for BN254CertificateVerifierEvents {
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
    /**Creates a new wrapper around an on-chain [`BN254CertificateVerifier`](self) contract instance.

    See the [wrapper's documentation](`BN254CertificateVerifierInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BN254CertificateVerifierInstance<P, N> {
        BN254CertificateVerifierInstance::<P, N>::new(address, provider)
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
        Output = alloy_contract::Result<BN254CertificateVerifierInstance<P, N>>,
    > {
        BN254CertificateVerifierInstance::<P, N>::deploy(provider, _operatorTableUpdater, _version)
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
        BN254CertificateVerifierInstance::<P, N>::deploy_builder(
            provider,
            _operatorTableUpdater,
            _version,
        )
    }
    /**A [`BN254CertificateVerifier`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`BN254CertificateVerifier`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BN254CertificateVerifierInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for BN254CertificateVerifierInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BN254CertificateVerifierInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BN254CertificateVerifierInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`BN254CertificateVerifier`](self) contract instance.

        See the [wrapper's documentation](`BN254CertificateVerifierInstance`) for more details.*/
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
        ) -> alloy_contract::Result<BN254CertificateVerifierInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> BN254CertificateVerifierInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BN254CertificateVerifierInstance<P, N> {
            BN254CertificateVerifierInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        BN254CertificateVerifierInstance<P, N>
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
        ///Creates a new call builder for the [`getNonsignerOperatorInfo`] function.
        pub fn getNonsignerOperatorInfo(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            referenceTimestamp: u32,
            operatorIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getNonsignerOperatorInfoCall, N> {
            self.call_builder(&getNonsignerOperatorInfoCall {
                operatorSet,
                referenceTimestamp,
                operatorIndex,
            })
        }
        ///Creates a new call builder for the [`getOperatorSetInfo`] function.
        pub fn getOperatorSetInfo(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            referenceTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorSetInfoCall, N> {
            self.call_builder(&getOperatorSetInfoCall {
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
        ///Creates a new call builder for the [`isNonsignerCached`] function.
        pub fn isNonsignerCached(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            referenceTimestamp: u32,
            operatorIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, isNonsignerCachedCall, N> {
            self.call_builder(&isNonsignerCachedCall {
                operatorSet,
                referenceTimestamp,
                operatorIndex,
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
        ///Creates a new call builder for the [`trySignatureVerification`] function.
        pub fn trySignatureVerification(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            aggPubkey: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            signature: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, trySignatureVerificationCall, N> {
            self.call_builder(&trySignatureVerificationCall {
                msgHash,
                aggPubkey,
                apkG2,
                signature,
            })
        }
        ///Creates a new call builder for the [`updateOperatorTable`] function.
        pub fn updateOperatorTable(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            referenceTimestamp: u32,
            operatorSetInfo: <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
            operatorSetConfig: <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, updateOperatorTableCall, N> {
            self.call_builder(&updateOperatorTableCall {
                operatorSet,
                referenceTimestamp,
                operatorSetInfo,
                operatorSetConfig,
            })
        }
        ///Creates a new call builder for the [`verifyCertificate`] function.
        pub fn verifyCertificate(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            cert: <IBN254CertificateVerifierTypes::BN254Certificate as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, verifyCertificateCall, N> {
            self.call_builder(&verifyCertificateCall { operatorSet, cert })
        }
        ///Creates a new call builder for the [`verifyCertificateNominal`] function.
        pub fn verifyCertificateNominal(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            cert: <IBN254CertificateVerifierTypes::BN254Certificate as alloy::sol_types::SolType>::RustType,
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
            cert: <IBN254CertificateVerifierTypes::BN254Certificate as alloy::sol_types::SolType>::RustType,
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
        BN254CertificateVerifierInstance<P, N>
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
