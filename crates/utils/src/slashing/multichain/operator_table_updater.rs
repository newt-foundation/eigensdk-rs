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
library IKeyRegistrarTypes {
    type CurveType is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IKeyRegistrarTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CurveType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<CurveType> for u8 {
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
        impl CurveType {
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
        impl From<u8> for CurveType {
            fn from(value: u8) -> Self {
                Self::from_underlying(value)
            }
        }
        #[automatically_derived]
        impl From<CurveType> for u8 {
            fn from(value: CurveType) -> Self {
                value.into_underlying()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for CurveType {
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
        impl alloy_sol_types::EventTopic for CurveType {
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
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IKeyRegistrarTypes`](self) contract instance.

    See the [wrapper's documentation](`IKeyRegistrarTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IKeyRegistrarTypesInstance<P, N> {
        IKeyRegistrarTypesInstance::<P, N>::new(address, provider)
    }
    /**A [`IKeyRegistrarTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IKeyRegistrarTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IKeyRegistrarTypesInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IKeyRegistrarTypesInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IKeyRegistrarTypesInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IKeyRegistrarTypesInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`IKeyRegistrarTypes`](self) contract instance.

        See the [wrapper's documentation](`IKeyRegistrarTypesInstance`) for more details.*/
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
    impl<P: ::core::clone::Clone, N> IKeyRegistrarTypesInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IKeyRegistrarTypesInstance<P, N> {
            IKeyRegistrarTypesInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IKeyRegistrarTypesInstance<P, N>
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
        IKeyRegistrarTypesInstance<P, N>
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

library IKeyRegistrarTypes {
    type CurveType is uint8;
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

interface OperatorTableUpdater {
    struct OperatorSet {
        address avs;
        uint32 id;
    }

    error CannotDisableGeneratorRoot();
    error CertificateInvalid();
    error CurrentlyPaused();
    error EmptyRoot();
    error GlobalTableRootInFuture();
    error GlobalTableRootStale();
    error InputAddressZero();
    error InvalidConfirmationThreshold();
    error InvalidCurveType();
    error InvalidGenerator();
    error InvalidGlobalTableRoot();
    error InvalidIndex();
    error InvalidMessageHash();
    error InvalidNewPausedStatus();
    error InvalidOperatorSet();
    error InvalidOperatorSetProof();
    error InvalidProofLength();
    error InvalidRoot();
    error InvalidShortString();
    error OnlyPauser();
    error OnlyUnpauser();
    error StringTooLong(string str);
    error TableUpdateForPastTimestamp();

    event GeneratorUpdated(OperatorSet operatorSet);
    event GlobalRootConfirmationThresholdUpdated(uint16 bps);
    event GlobalRootDisabled(bytes32 indexed globalTableRoot);
    event Initialized(uint8 version);
    event NewGlobalTableRoot(uint32 indexed referenceTimestamp, bytes32 indexed globalTableRoot);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _bn254CertificateVerifier, address _ecdsaCertificateVerifier, address _pauserRegistry, string _version);

    function GENERATOR_GLOBAL_TABLE_ROOT() external view returns (bytes32);
    function GENERATOR_MAX_STALENESS_PERIOD() external view returns (uint32);
    function GENERATOR_REFERENCE_TIMESTAMP() external view returns (uint32);
    function GLOBAL_TABLE_ROOT_CERT_TYPEHASH() external view returns (bytes32);
    function MAX_BPS() external view returns (uint16);
    function OPERATOR_INFO_LEAF_SALT() external view returns (uint8);
    function OPERATOR_TABLE_LEAF_SALT() external view returns (uint8);
    function bn254CertificateVerifier() external view returns (address);
    function calculateOperatorInfoLeaf(IOperatorTableCalculatorTypes.BN254OperatorInfo memory operatorInfo) external pure returns (bytes32);
    function calculateOperatorTableLeaf(bytes memory operatorTableBytes) external pure returns (bytes32);
    function confirmGlobalTableRoot(IBN254CertificateVerifierTypes.BN254Certificate memory globalTableRootCert, bytes32 globalTableRoot, uint32 referenceTimestamp, uint32 referenceBlockNumber) external;
    function disableRoot(bytes32 globalTableRoot) external;
    function ecdsaCertificateVerifier() external view returns (address);
    function getCertificateVerifier(IKeyRegistrarTypes.CurveType curveType) external view returns (address);
    function getCurrentGlobalTableRoot() external view returns (bytes32);
    function getGenerator() external view returns (OperatorSet memory);
    function getGeneratorConfig() external view returns (ICrossChainRegistryTypes.OperatorSetConfig memory);
    function getGeneratorReferenceTimestamp() external view returns (uint32);
    function getGlobalTableRootByTimestamp(uint32 referenceTimestamp) external view returns (bytes32);
    function getGlobalTableUpdateMessageHash(bytes32 globalTableRoot, uint32 referenceTimestamp, uint32 referenceBlockNumber) external pure returns (bytes32);
    function getGlobalTableUpdateSignableDigest(bytes32 globalTableRoot, uint32 referenceTimestamp, uint32 referenceBlockNumber) external view returns (bytes32);
    function getLatestReferenceBlockNumber() external view returns (uint32);
    function getLatestReferenceTimestamp() external view returns (uint32);
    function getReferenceBlockNumberByTimestamp(uint32 referenceTimestamp) external view returns (uint32);
    function getReferenceTimestampByBlockNumber(uint32 referenceBlockNumber) external view returns (uint32);
    function globalRootConfirmationThreshold() external view returns (uint16);
    function initialize(address _owner, uint256 initialPausedStatus, OperatorSet memory _initialGenerator, uint16 _globalRootConfirmationThreshold, IOperatorTableCalculatorTypes.BN254OperatorSetInfo memory generatorInfo) external;
    function isRootValid(bytes32 globalTableRoot) external view returns (bool);
    function isRootValidByTimestamp(uint32 referenceTimestamp) external view returns (bool);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function renounceOwnership() external;
    function setGlobalRootConfirmationThreshold(uint16 bps) external;
    function transferOwnership(address newOwner) external;
    function unpause(uint256 newPausedStatus) external;
    function updateGenerator(OperatorSet memory generator, IOperatorTableCalculatorTypes.BN254OperatorSetInfo memory generatorInfo) external;
    function updateOperatorTable(uint32 referenceTimestamp, bytes32 globalTableRoot, uint32 operatorSetIndex, bytes memory proof, bytes memory operatorTableBytes) external;
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
        "name": "_bn254CertificateVerifier",
        "type": "address",
        "internalType": "contract IBN254CertificateVerifier"
      },
      {
        "name": "_ecdsaCertificateVerifier",
        "type": "address",
        "internalType": "contract IECDSACertificateVerifier"
      },
      {
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
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
    "name": "GENERATOR_GLOBAL_TABLE_ROOT",
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
    "name": "GENERATOR_MAX_STALENESS_PERIOD",
    "inputs": [],
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
    "name": "GENERATOR_REFERENCE_TIMESTAMP",
    "inputs": [],
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
    "name": "GLOBAL_TABLE_ROOT_CERT_TYPEHASH",
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
    "name": "MAX_BPS",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "OPERATOR_INFO_LEAF_SALT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "OPERATOR_TABLE_LEAF_SALT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "bn254CertificateVerifier",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBN254CertificateVerifier"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "calculateOperatorInfoLeaf",
    "inputs": [
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
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "calculateOperatorTableLeaf",
    "inputs": [
      {
        "name": "operatorTableBytes",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "confirmGlobalTableRoot",
    "inputs": [
      {
        "name": "globalTableRootCert",
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
        "name": "globalTableRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "disableRoot",
    "inputs": [
      {
        "name": "globalTableRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "ecdsaCertificateVerifier",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IECDSACertificateVerifier"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCertificateVerifier",
    "inputs": [
      {
        "name": "curveType",
        "type": "uint8",
        "internalType": "enum IKeyRegistrarTypes.CurveType"
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
    "name": "getCurrentGlobalTableRoot",
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
    "name": "getGenerator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getGeneratorConfig",
    "inputs": [],
    "outputs": [
      {
        "name": "",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getGeneratorReferenceTimestamp",
    "inputs": [],
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
    "name": "getGlobalTableRootByTimestamp",
    "inputs": [
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "getGlobalTableUpdateMessageHash",
    "inputs": [
      {
        "name": "globalTableRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "getGlobalTableUpdateSignableDigest",
    "inputs": [
      {
        "name": "globalTableRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "getLatestReferenceBlockNumber",
    "inputs": [],
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
    "name": "getLatestReferenceTimestamp",
    "inputs": [],
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
    "name": "getReferenceBlockNumberByTimestamp",
    "inputs": [
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "getReferenceTimestampByBlockNumber",
    "inputs": [
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "globalRootConfirmationThreshold",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "initialPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_initialGenerator",
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
        "name": "_globalRootConfirmationThreshold",
        "type": "uint16",
        "internalType": "uint16"
      },
      {
        "name": "generatorInfo",
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
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isRootValid",
    "inputs": [
      {
        "name": "globalTableRoot",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "isRootValidByTimestamp",
    "inputs": [
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
    "name": "owner",
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
    "name": "pause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "pauseAll",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "paused",
    "inputs": [
      {
        "name": "index",
        "type": "uint8",
        "internalType": "uint8"
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
    "name": "paused",
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
    "name": "pauserRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setGlobalRootConfirmationThreshold",
    "inputs": [
      {
        "name": "bps",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unpause",
    "inputs": [
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateGenerator",
    "inputs": [
      {
        "name": "generator",
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
        "name": "generatorInfo",
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
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorTable",
    "inputs": [
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "globalTableRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "operatorSetIndex",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "proof",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "operatorTableBytes",
        "type": "bytes",
        "internalType": "bytes"
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
    "type": "event",
    "name": "GeneratorUpdated",
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
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "GlobalRootConfirmationThresholdUpdated",
    "inputs": [
      {
        "name": "bps",
        "type": "uint16",
        "indexed": false,
        "internalType": "uint16"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "GlobalRootDisabled",
    "inputs": [
      {
        "name": "globalTableRoot",
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
    "name": "NewGlobalTableRoot",
    "inputs": [
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "globalTableRoot",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Paused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Unpaused",
    "inputs": [
      {
        "name": "account",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newPausedStatus",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "CannotDisableGeneratorRoot",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CertificateInvalid",
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
    "name": "GlobalTableRootInFuture",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GlobalTableRootStale",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputAddressZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidConfirmationThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidCurveType",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidGenerator",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidGlobalTableRoot",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidIndex",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidMessageHash",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNewPausedStatus",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidOperatorSet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidOperatorSetProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidProofLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidRoot",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidShortString",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyPauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyUnpauser",
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
    "name": "TableUpdateForPastTimestamp",
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
pub mod OperatorTableUpdater {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610100604052348015610010575f5ffd5b50604051612e6a380380612e6a83398101604081905261002f916101b9565b808484846001600160a01b03811661005a576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b0390811660805291821660a0521660c05261007b81610090565b60e052506100876100d6565b505050506102fe565b5f5f829050601f815111156100c3578260405163305a27a960e01b81526004016100ba91906102a3565b60405180910390fd5b80516100ce826102d8565b179392505050565b5f54610100900460ff161561013d5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b60648201526084016100ba565b5f5460ff9081161461018c575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146101a2575f5ffd5b50565b634e487b7160e01b5f52604160045260245ffd5b5f5f5f5f608085870312156101cc575f5ffd5b84516101d78161018e565b60208601519094506101e88161018e565b60408601519093506101f98161018e565b60608601519092506001600160401b03811115610214575f5ffd5b8501601f81018713610224575f5ffd5b80516001600160401b0381111561023d5761023d6101a5565b604051601f8201601f19908116603f011681016001600160401b038111828210171561026b5761026b6101a5565b604052818152828201602001891015610282575f5ffd5b8160208401602083015e5f6020838301015280935050505092959194509250565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b805160208083015191908110156102f8575f198160200360031b1b821691505b50919050565b60805160a05160c05160e051612aec61037e5f395f610a1301525f81816105b501528181610aad0152610e6d01525f818161061c0152818161094101528181610a6d01528181610b1901528181610dc8015281816111420152818161142b01526114dd01525f818161053c0152818161160901526118f50152612aec5ff3fe608060405234801561000f575f5ffd5b5060043610610255575f3560e01c80636f728c5011610140578063ad0f9582116100bf578063c3be1e3311610084578063c3be1e3314610672578063c5916a3914610685578063eaaed9d5146106aa578063f2fde38b146106bd578063fabc1cbc146106d0578063fd967f47146106e3575f5ffd5b8063ad0f9582146105b0578063b0cb3a24146105d7578063b8c1430614610617578063c252aa221461063e578063c3621f0a1461065f575f5ffd5b80638da5cb5b116101055780638da5cb5b1461055e5780639ea947781461056f5780639f7e206f14610582578063a2c902f514610595578063a2f2e24d1461059d575f5ffd5b80636f728c50146104f5578063715018a6146105205780637551ba3414610528578063790961ea14610530578063886f119514610537575f5ffd5b806331a599d2116101d757806354fd4d501161019c57806354fd4d5014610470578063595c6a67146104855780635ac86ab71461048d5780635c975abb146104b0578063612abcb0146104b857806364e1df84146104c0575f5ffd5b806331a599d2146103ea5780633ef6cd7a1461040f578063401c370f146104365780634624e6a314610449578063538a37901461045d575f5ffd5b80631e2ca2601161021d5780631e2ca260146103125780632370356c1461035b57806323b7b5b21461036e57806328522d791461039657806330ef41b4146103b8575f5ffd5b806306f5187514610259578063121409ea1461026e578063136439dd1461028d578063193b79f3146102a05780631bdc0deb146102dd575b5f5ffd5b61026c610267366004611b4f565b6106ec565b005b610276608e81565b60405160ff90911681526020015b60405180910390f35b61026c61029b366004611bc5565b6108c6565b6102c86102ae366004611bed565b63ffffffff9081165f908152609b60205260409020541690565b60405163ffffffff9091168152602001610284565b6103047fcefe99cb2e240b5f07de5cd472a75fc6e345370b73588ab161cb25c4a259a86981565b604051908152602001610284565b6040805180820182525f80825260209182015281518083019092526098546001600160a01b0381168352600160a01b900463ffffffff16908201525b6040516102849190611c26565b61026c610369366004611c34565b610900565b6102c861037c366004611bed565b63ffffffff9081165f908152609a60205260409020541690565b60975462010000900463ffffffff165f90815260996020526040902054610304565b6103da6103c6366004611bc5565b5f908152609c602052604090205460ff1690565b6040519015158152602001610284565b60975462010000900463ffffffff9081165f908152609a6020526040902054166102c8565b6103047f4491f5ee91595f938885ef73c9a1fa8a6d14ff9b9dab4aa24b8802bbb9bfc1cc81565b610304610444366004611c4d565b610914565b60975462010000900463ffffffff166102c8565b61030461046b366004611d3c565b6109bd565b610478610a0c565b6040516102849190611e34565b61026c610a3c565b6103da61049b366004611e69565b606654600160ff9092169190911b9081161490565b606654610304565b6102c8600181565b6103da6104ce366004611bed565b63ffffffff165f908152609960209081526040808320548352609c90915290205460ff1690565b610508610503366004611e97565b610a50565b6040516001600160a01b039091168152602001610284565b61026c610aef565b6102c8610b00565b6102c85f81565b6105087f000000000000000000000000000000000000000000000000000000000000000081565b6033546001600160a01b0316610508565b61026c61057d366004611ef4565b610b8e565b61026c610590366004611f8f565b610ede565b610276607581565b6103046105ab366004611fda565b610ef0565b6105087f000000000000000000000000000000000000000000000000000000000000000081565b6040805180820182525f8082526020918201528151808301909252609d546001600160a01b0381168352600160a01b900463ffffffff169082015261034e565b6105087f000000000000000000000000000000000000000000000000000000000000000081565b60975461064c9061ffff1681565b60405161ffff9091168152602001610284565b61026c61066d366004611bc5565b610f26565b610304610680366004611c4d565b610fdb565b610304610693366004611bed565b63ffffffff165f9081526099602052604090205490565b61026c6106b8366004612018565b611043565b61026c6106cb366004612084565b611292565b61026c6106de366004611bc5565b611308565b61064c61271081565b5f54610100900460ff161580801561070a57505f54600160ff909116105b806107235750303b15801561072357505f5460ff166001145b61078b5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156107ac575f805461ff0019166101001790555b6107b586611375565b6107be856113c6565b609d80546001600160c01b031916301790556107da8483611403565b6107e383611583565b7fcefe99cb2e240b5f07de5cd472a75fc6e345370b73588ab161cb25c4a259a8697fbb86fbc034f4e382929974bcd8419ed626b0ea647f962d89ba2fb6bd28785ab98190555f52609c6020527f38353ab40115e4013d688e07cff5857dde443bd05e72c49fcb5e684a9bb9efc4805460ff19166001179055609780544263ffffffff16620100000265ffffffff00001990911617905580156108be575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050505050565b6108ce6115f4565b60665481811681146108f35760405163c61dca5d60e01b815260040160405180910390fd5b6108fc826113c6565b5050565b610908611697565b61091181611583565b50565b5f5f610921858585610fdb565b6040516306119d0d60e21b815260016004820152602481018290529091507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690631846743490604401602060405180830381865afa15801561098e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109b2919061209f565b9150505b9392505050565b5f6075826040516020016109d191906120f0565b60408051601f19818403018152908290526109ef929160200161211d565b604051602081830303815290604052805190602001209050919050565b6060610a377f00000000000000000000000000000000000000000000000000000000000000006116f1565b905090565b610a446115f4565b610a4e5f196113c6565b565b5f6002826002811115610a6557610a65612146565b03610a9157507f0000000000000000000000000000000000000000000000000000000000000000919050565b6001826002811115610aa557610aa5612146565b03610ad157507f0000000000000000000000000000000000000000000000000000000000000000919050565b60405163fdea7c0960e01b815260040160405180910390fd5b919050565b610af7611697565b610a4e5f611375565b604051635ddb9b5b60e01b81525f906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690635ddb9b5b90610b4f9060989060040161215a565b602060405180830381865afa158015610b6a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a379190612181565b6001610b998161172e565b610ba1611759565b5f5f5f5f610baf87876117b2565b5f8f8152609c60205260409020549397509195509350915060ff16610be75760405163504570e360e01b815260040160405180910390fd5b604080518082019091526098546001600160a01b0381168252600160a01b900463ffffffff166020820152610c1b906117f9565b610c24856117f9565b03610c4257604051631fb1705560e21b815260040160405180910390fd5b610c4b83610a50565b6001600160a01b031663cd83a72b858e6040518363ffffffff1660e01b8152600401610c7892919061219c565b602060405180830381865afa158015610c93573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cb791906121bd565b15610cc55750505050610eca565b610cce83610a50565b6001600160a01b0316635ddb9b5b856040518263ffffffff1660e01b8152600401610cf99190611c26565b602060405180830381865afa158015610d14573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d389190612181565b63ffffffff168c63ffffffff1611610d635760405163207617df60e01b815260040160405180910390fd5b63ffffffff8c165f908152609960205260409020548b14610d975760405163639d09b560e11b815260040160405180910390fd5b610dad8b8b8b8b610da88c8c610ef0565b61185c565b6002836002811115610dc157610dc1612146565b03610e52577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316636738c40b858e610e00856118c1565b866040518563ffffffff1660e01b8152600401610e2094939291906121dc565b5f604051808303815f87803b158015610e37575f5ffd5b505af1158015610e49573d5f5f3e3d5ffd5b50505050610ec5565b6001836002811115610e6657610e66612146565b03610ad1577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166356d482f5858e610ea5856118dd565b866040518563ffffffff1660e01b8152600401610e20949392919061225d565b505050505b610ed4600160c955565b5050505050505050565b610ee6611697565b6108fc8282611403565b5f608e8383604051602001610f0793929190612302565b6040516020818303038152906040528051906020012090505b92915050565b610f2e6115f4565b5f818152609c602052604090205460ff16610f5c5760405163504570e360e01b815260040160405180910390fd5b7fcefe99cb2e240b5f07de5cd472a75fc6e345370b73588ab161cb25c4a259a8698103610f9c576040516319920afd60e11b815260040160405180910390fd5b5f818152609c6020526040808220805460ff191690555182917f8bd43de1250f58fe6ec9a78671a8b78dba70f0018656d157a3aeaabec389df3491a250565b604080517f4491f5ee91595f938885ef73c9a1fa8a6d14ff9b9dab4aa24b8802bbb9bfc1cc602082015290810184905263ffffffff8084166060830152821660808201525f9060a0016040516020818303038152906040528051906020012090509392505050565b5f61104d8161172e565b611055611759565b428363ffffffff16111561107c57604051635a119db560e11b815260040160405180910390fd5b60975463ffffffff620100009091048116908416116110ae5760405163037fa86b60e31b815260040160405180910390fd5b6110b9848484610fdb565b8560200135146110dc57604051638b56642d60e01b815260040160405180910390fd5b6040805160018082528183019092525f91602080830190803683375050609754825192935061ffff16918391505f9061111757611117612329565b61ffff90921660209283029190910190910152604051625f5e5d60e21b81525f906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063017d79749061117c906098908b90879060040161245b565b6020604051808303815f875af1158015611198573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111bc91906121bd565b9050806111dc57604051633042041f60e21b815260040160405180910390fd5b6097805463ffffffff80881662010000810265ffffffff000019909316929092179092555f818152609a602090815260408083208054958a1663ffffffff1996871681179091558352609b825280832080549095168417909455828252609981528382208a9055898252609c9052828120805460ff19166001179055915188927f010dcbe0d1e019c93357711f7bb6287d543b7ff7de74f29df3fb5ecceec8d36991a3505061128b600160c955565b5050505050565b61129a611697565b6001600160a01b0381166112ff5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610782565b61091181611375565b6113106118f3565b606654801982198116146113375760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c9060200160405180910390a25050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b81609861141082826125e8565b5050604051635ddb9b5b60e01b81525f906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690635ddb9b5b90611460908690600401612679565b602060405180830381865afa15801561147b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061149f9190612181565b905063ffffffff8116156114c657604051636446f91760e01b815260040160405180910390fd5b604051636738c40b60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690636738c40b9061151a9086906001908790609d90600401612687565b5f604051808303815f87803b158015611531575f5ffd5b505af1158015611543573d5f5f3e3d5ffd5b505050507f3463431b09dfd43dec7349f8f24acfa753fe4cf40a26235402d213373df15856836040516115769190612679565b60405180910390a1505050565b61271061ffff821611156115aa576040516307336f0360e11b815260040160405180910390fd5b6097805461ffff191661ffff83169081179091556040519081527ff5d1836df8fcd7c1e54047e94ac8773d2855395603e2ef9ba5f5f16905f225929060200160405180910390a150565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015611656573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061167a91906121bd565b610a4e57604051631d77d47760e21b815260040160405180910390fd5b6033546001600160a01b03163314610a4e5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610782565b60605f6116fd836119a4565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b606654600160ff83161b908116036109115760405163840a48d560e01b815260040160405180910390fd5b600260c954036117ab5760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606401610782565b600260c955565b604080518082019091525f8082526020820152604080518082019091525f808252602082018190529060606117e985870187612760565b9299919850965090945092505050565b5f815f0151826020015163ffffffff1660405160200161184492919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b604051602081830303815290604052610f2090612825565b6118a483838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508992508591505063ffffffff88166119cb565b61128b5760405163afa42ca760e01b815260040160405180910390fd5b6118c9611ac5565b81806020019051810190610f2091906128ad565b606081806020019051810190610f20919061295c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561194f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119739190612a5d565b6001600160a01b0316336001600160a01b031614610a4e5760405163794821ff60e01b815260040160405180910390fd5b5f60ff8216601f811115610f2057604051632cd44ac360e21b815260040160405180910390fd5b5f836119ea576040516329e7276760e11b815260040160405180910390fd5b836119f6868585611a00565b1495945050505050565b5f83515f03611a105750816109b6565b60208451611a1e9190612a78565b15611a3c576040516313717da960e21b815260040160405180910390fd5b8260205b85518111611a9d57611a53600285612a78565b5f03611a7457815f528086015160205260405f209150600284049350611a8b565b808601515f528160205260405f2091506002840493505b611a96602082612a97565b9050611a40565b508215611abd576040516363df817160e01b815260040160405180910390fd5b949350505050565b60405180608001604052805f81526020015f8152602001611af760405180604001604052805f81526020015f81525090565b8152602001606081525090565b6001600160a01b0381168114610911575f5ffd5b5f60408284031215611b28575f5ffd5b50919050565b803561ffff81168114610aea575f5ffd5b5f60a08284031215611b28575f5ffd5b5f5f5f5f5f60c08688031215611b63575f5ffd5b8535611b6e81611b04565b945060208601359350611b848760408801611b18565b9250611b9260808701611b2e565b915060a08601356001600160401b03811115611bac575f5ffd5b611bb888828901611b3f565b9150509295509295909350565b5f60208284031215611bd5575f5ffd5b5035919050565b63ffffffff81168114610911575f5ffd5b5f60208284031215611bfd575f5ffd5b81356109b681611bdc565b80516001600160a01b0316825260209081015163ffffffff16910152565b60408101610f208284611c08565b5f60208284031215611c44575f5ffd5b6109b682611b2e565b5f5f5f60608486031215611c5f575f5ffd5b833592506020840135611c7181611bdc565b91506040840135611c8181611bdc565b809150509250925092565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b0381118282101715611cc257611cc2611c8c565b60405290565b604051608081016001600160401b0381118282101715611cc257611cc2611c8c565b604051601f8201601f191681016001600160401b0381118282101715611d1257611d12611c8c565b604052919050565b5f6001600160401b03821115611d3257611d32611c8c565b5060051b60200190565b5f60208284031215611d4c575f5ffd5b81356001600160401b03811115611d61575f5ffd5b82018084036060811215611d73575f5ffd5b611d7b611ca0565b6040821215611d88575f5ffd5b611d90611ca0565b83358152602080850135908201528152604083013591506001600160401b03821115611dba575f5ffd5b818301925085601f840112611dcd575f5ffd5b82359150611de2611ddd83611d1a565b611cea565b8083825260208201915060208460051b860101935087841115611e03575f5ffd5b6020850194505b83851015611e25578435825260209485019490910190611e0a565b60208301525095945050505050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b5f60208284031215611e79575f5ffd5b813560ff811681146109b6575f5ffd5b803560038110610aea575f5ffd5b5f60208284031215611ea7575f5ffd5b6109b682611e89565b5f5f83601f840112611ec0575f5ffd5b5081356001600160401b03811115611ed6575f5ffd5b602083019150836020828501011115611eed575f5ffd5b9250929050565b5f5f5f5f5f5f5f60a0888a031215611f0a575f5ffd5b8735611f1581611bdc565b9650602088013595506040880135611f2c81611bdc565b945060608801356001600160401b03811115611f46575f5ffd5b611f528a828b01611eb0565b90955093505060808801356001600160401b03811115611f70575f5ffd5b611f7c8a828b01611eb0565b989b979a50959850939692959293505050565b5f5f60608385031215611fa0575f5ffd5b611faa8484611b18565b915060408301356001600160401b03811115611fc4575f5ffd5b611fd085828601611b3f565b9150509250929050565b5f5f60208385031215611feb575f5ffd5b82356001600160401b03811115612000575f5ffd5b61200c85828601611eb0565b90969095509350505050565b5f5f5f5f6080858703121561202b575f5ffd5b84356001600160401b03811115612040575f5ffd5b85016101208188031215612052575f5ffd5b935060208501359250604085013561206981611bdc565b9150606085013561207981611bdc565b939692955090935050565b5f60208284031215612094575f5ffd5b81356109b681611b04565b5f602082840312156120af575f5ffd5b5051919050565b5f8151808452602084019350602083015f5b828110156120e65781518652602095860195909101906001016120c8565b5093949350505050565b60208082528251805183830152015160408201525f6020830151606080840152611abd60808401826120b6565b60ff60f81b8360f81b1681525f82518060208501600185015e5f92016001019182525092915050565b634e487b7160e01b5f52602160045260245ffd5b60408101610f208284546001600160a01b038116825260a01c63ffffffff16602090910152565b5f60208284031215612191575f5ffd5b81516109b681611bdc565b606081016121aa8285611c08565b63ffffffff831660408301529392505050565b5f602082840312156121cd575f5ffd5b815180151581146109b6575f5ffd5b6121e68186611c08565b63ffffffff8416604082015260c06060820152825160c0820152602083015160e08201525f604084015161222861010084018280518252602090810151910152565b50606084015160a06101408401526122446101608401826120b6565b9150506122546080830184611c08565b95945050505050565b5f60c0820161226c8388611c08565b63ffffffff8616604084015260c0606084015280855180835260e08501915060e08160051b8601019250602087015f5b828110156122ed5786850360df19018452815180516001600160a01b031686526020908101516040918701829052906122d7908701826120b6565b955050602093840193919091019060010161229c565b50505050809150506122546080830184611c08565b60f884901b6001600160f81b0319168152818360018301375f910160010190815292915050565b634e487b7160e01b5f52603260045260245ffd5b5f5f8335601e19843603018112612352575f5ffd5b83016020810192503590506001600160401b03811115612370575f5ffd5b8060051b3603821315611eed575f5ffd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b5f8235605e198336030181126123bd575f5ffd5b90910192915050565b8183525f6001600160fb1b038311156123dd575f5ffd5b8260051b80836020870137939093016020019392505050565b80358252602080820135908301525f612412604083018361233d565b606060408601526122546060860182846123c6565b5f8151808452602084019350602083015f5b828110156120e657815161ffff16865260209586019590910190600101612439565b61247e8185546001600160a01b038116825260a01c63ffffffff16602090910152565b608060408201525f6101a08201843561249681611bdc565b63ffffffff166080840152602085013560a0840152604085013560c0840152606085013560e0840152604060808601610100850137604060c086016101408501376124e561010086018661233d565b610120610180860152828184526101c0860190506101c08260051b8701019350825f5b838110156125c7578786036101bf1901835261252482866123a9565b803561252f81611bdc565b63ffffffff168752602081013536829003601e1901811261254e575f5ffd5b81016020810190356001600160401b03811115612569575f5ffd5b803603821315612577575f5ffd5b606060208a015261258c60608a018284612381565b91505061259c60408301836123a9565b915087810360408901526125b081836123f6565b975050506020928301929190910190600101612508565b505050505082810360608401526125de8185612427565b9695505050505050565b81356125f381611b04565b81546001600160a01b031981166001600160a01b03929092169182178355602084013561261f81611bdc565b6001600160c01b03199190911690911760a09190911b63ffffffff60a01b1617905550565b803561264f81611b04565b6001600160a01b03168252602081013561266881611bdc565b63ffffffff81166020840152505050565b60408101610f208284612644565b6126918186612644565b63ffffffff841660408281019190915260c06060808401829052853591840191909152602085013560e0840152908401356101008301528301356101208201525f6126df608085018561233d565b60a06101408501526126f6610160850182846123c6565b925050506122546080830184546001600160a01b038116825260a01c63ffffffff16602090910152565b5f60408284031215612730575f5ffd5b612738611ca0565b9050813561274581611b04565b8152602082013561275581611bdc565b602082015292915050565b5f5f5f5f60c08587031215612773575f5ffd5b61277d8686612720565b935061278b60408601611e89565b925061279a8660608701612720565b915060a08501356001600160401b038111156127b4575f5ffd5b8501601f810187136127c4575f5ffd5b80356001600160401b038111156127dd576127dd611c8c565b6127f0601f8201601f1916602001611cea565b818152886020838501011115612804575f5ffd5b816020840160208301375f6020838301015280935050505092959194509250565b80516020808301519190811015611b28575f1960209190910360031b1b16919050565b5f82601f830112612857575f5ffd5b8151612865611ddd82611d1a565b8082825260208201915060208360051b860101925085831115612886575f5ffd5b602085015b838110156128a357805183526020928301920161288b565b5095945050505050565b5f602082840312156128bd575f5ffd5b81516001600160401b038111156128d2575f5ffd5b820180840360a08112156128e4575f5ffd5b6128ec611cc8565b82518152602080840151908201526040603f198301121561290b575f5ffd5b612913611ca0565b604084810151825260608501516020830152820152608083015191506001600160401b03821115612942575f5ffd5b61294e86838501612848565b606082015295945050505050565b5f6020828403121561296c575f5ffd5b81516001600160401b03811115612981575f5ffd5b8201601f81018413612991575f5ffd5b805161299f611ddd82611d1a565b8082825260208201915060208360051b8501019250868311156129c0575f5ffd5b602084015b83811015612a525780516001600160401b038111156129e2575f5ffd5b85016040818a03601f190112156129f7575f5ffd5b6129ff611ca0565b6020820151612a0d81611b04565b815260408201516001600160401b03811115612a27575f5ffd5b612a368b602083860101612848565b60208301525080855250506020830192506020810190506129c5565b509695505050505050565b5f60208284031215612a6d575f5ffd5b81516109b681611b04565b5f82612a9257634e487b7160e01b5f52601260045260245ffd5b500690565b80820180821115610f2057634e487b7160e01b5f52601160045260245ffdfea2646970667358221220383df0454ad9496b57e13a357e8f29ef958ab90c31374b06e0030e1fc9f1834164736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@Qa.j8\x03\x80a.j\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xB9V[\x80\x84\x84\x84`\x01`\x01`\xA0\x1B\x03\x81\x16a\0ZW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80R\x91\x82\x16`\xA0R\x16`\xC0Ra\0{\x81a\0\x90V[`\xE0RPa\0\x87a\0\xD6V[PPPPa\x02\xFEV[__\x82\x90P`\x1F\x81Q\x11\x15a\0\xC3W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\0\xBA\x91\x90a\x02\xA3V[`@Q\x80\x91\x03\x90\xFD[\x80Qa\0\xCE\x82a\x02\xD8V[\x17\x93\x92PPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x01=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01a\0\xBAV[_T`\xFF\x90\x81\x16\x14a\x01\x8CW_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xA2W__\xFD[PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[____`\x80\x85\x87\x03\x12\x15a\x01\xCCW__\xFD[\x84Qa\x01\xD7\x81a\x01\x8EV[` \x86\x01Q\x90\x94Pa\x01\xE8\x81a\x01\x8EV[`@\x86\x01Q\x90\x93Pa\x01\xF9\x81a\x01\x8EV[``\x86\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x14W__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x02$W__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02=Wa\x02=a\x01\xA5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02kWa\x02ka\x01\xA5V[`@R\x81\x81R\x82\x82\x01` \x01\x89\x10\x15a\x02\x82W__\xFD[\x81` \x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x93PPPP\x92\x95\x91\x94P\x92PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x02\xF8W_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa*\xECa\x03~_9_a\n\x13\x01R_\x81\x81a\x05\xB5\x01R\x81\x81a\n\xAD\x01Ra\x0Em\x01R_\x81\x81a\x06\x1C\x01R\x81\x81a\tA\x01R\x81\x81a\nm\x01R\x81\x81a\x0B\x19\x01R\x81\x81a\r\xC8\x01R\x81\x81a\x11B\x01R\x81\x81a\x14+\x01Ra\x14\xDD\x01R_\x81\x81a\x05<\x01R\x81\x81a\x16\t\x01Ra\x18\xF5\x01Ra*\xEC_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02UW_5`\xE0\x1C\x80cor\x8CP\x11a\x01@W\x80c\xAD\x0F\x95\x82\x11a\0\xBFW\x80c\xC3\xBE\x1E3\x11a\0\x84W\x80c\xC3\xBE\x1E3\x14a\x06rW\x80c\xC5\x91j9\x14a\x06\x85W\x80c\xEA\xAE\xD9\xD5\x14a\x06\xAAW\x80c\xF2\xFD\xE3\x8B\x14a\x06\xBDW\x80c\xFA\xBC\x1C\xBC\x14a\x06\xD0W\x80c\xFD\x96\x7FG\x14a\x06\xE3W__\xFD[\x80c\xAD\x0F\x95\x82\x14a\x05\xB0W\x80c\xB0\xCB:$\x14a\x05\xD7W\x80c\xB8\xC1C\x06\x14a\x06\x17W\x80c\xC2R\xAA\"\x14a\x06>W\x80c\xC3b\x1F\n\x14a\x06_W__\xFD[\x80c\x8D\xA5\xCB[\x11a\x01\x05W\x80c\x8D\xA5\xCB[\x14a\x05^W\x80c\x9E\xA9Gx\x14a\x05oW\x80c\x9F~ o\x14a\x05\x82W\x80c\xA2\xC9\x02\xF5\x14a\x05\x95W\x80c\xA2\xF2\xE2M\x14a\x05\x9DW__\xFD[\x80cor\x8CP\x14a\x04\xF5W\x80cqP\x18\xA6\x14a\x05 W\x80cuQ\xBA4\x14a\x05(W\x80cy\ta\xEA\x14a\x050W\x80c\x88o\x11\x95\x14a\x057W__\xFD[\x80c1\xA5\x99\xD2\x11a\x01\xD7W\x80cT\xFDMP\x11a\x01\x9CW\x80cT\xFDMP\x14a\x04pW\x80cY\\jg\x14a\x04\x85W\x80cZ\xC8j\xB7\x14a\x04\x8DW\x80c\\\x97Z\xBB\x14a\x04\xB0W\x80ca*\xBC\xB0\x14a\x04\xB8W\x80cd\xE1\xDF\x84\x14a\x04\xC0W__\xFD[\x80c1\xA5\x99\xD2\x14a\x03\xEAW\x80c>\xF6\xCDz\x14a\x04\x0FW\x80c@\x1C7\x0F\x14a\x046W\x80cF$\xE6\xA3\x14a\x04IW\x80cS\x8A7\x90\x14a\x04]W__\xFD[\x80c\x1E,\xA2`\x11a\x02\x1DW\x80c\x1E,\xA2`\x14a\x03\x12W\x80c#p5l\x14a\x03[W\x80c#\xB7\xB5\xB2\x14a\x03nW\x80c(R-y\x14a\x03\x96W\x80c0\xEFA\xB4\x14a\x03\xB8W__\xFD[\x80c\x06\xF5\x18u\x14a\x02YW\x80c\x12\x14\t\xEA\x14a\x02nW\x80c\x13d9\xDD\x14a\x02\x8DW\x80c\x19;y\xF3\x14a\x02\xA0W\x80c\x1B\xDC\r\xEB\x14a\x02\xDDW[__\xFD[a\x02la\x02g6`\x04a\x1BOV[a\x06\xECV[\0[a\x02v`\x8E\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02la\x02\x9B6`\x04a\x1B\xC5V[a\x08\xC6V[a\x02\xC8a\x02\xAE6`\x04a\x1B\xEDV[c\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`\x9B` R`@\x90 T\x16\x90V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x84V[a\x03\x04\x7F\xCE\xFE\x99\xCB.$\x0B_\x07\xDE\\\xD4r\xA7_\xC6\xE3E7\x0BsX\x8A\xB1a\xCB%\xC4\xA2Y\xA8i\x81V[`@Q\x90\x81R` \x01a\x02\x84V[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x98T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R[`@Qa\x02\x84\x91\x90a\x1C&V[a\x02la\x03i6`\x04a\x1C4V[a\t\0V[a\x02\xC8a\x03|6`\x04a\x1B\xEDV[c\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x90V[`\x97Tb\x01\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x99` R`@\x90 Ta\x03\x04V[a\x03\xDAa\x03\xC66`\x04a\x1B\xC5V[_\x90\x81R`\x9C` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02\x84V[`\x97Tb\x01\0\0\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16a\x02\xC8V[a\x03\x04\x7FD\x91\xF5\xEE\x91Y_\x93\x88\x85\xEFs\xC9\xA1\xFA\x8Am\x14\xFF\x9B\x9D\xABJ\xA2K\x88\x02\xBB\xB9\xBF\xC1\xCC\x81V[a\x03\x04a\x04D6`\x04a\x1CMV[a\t\x14V[`\x97Tb\x01\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16a\x02\xC8V[a\x03\x04a\x04k6`\x04a\x1D<V[a\t\xBDV[a\x04xa\n\x0CV[`@Qa\x02\x84\x91\x90a\x1E4V[a\x02la\n<V[a\x03\xDAa\x04\x9B6`\x04a\x1EiV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03\x04V[a\x02\xC8`\x01\x81V[a\x03\xDAa\x04\xCE6`\x04a\x1B\xEDV[c\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x99` \x90\x81R`@\x80\x83 T\x83R`\x9C\x90\x91R\x90 T`\xFF\x16\x90V[a\x05\x08a\x05\x036`\x04a\x1E\x97V[a\nPV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x84V[a\x02la\n\xEFV[a\x02\xC8a\x0B\0V[a\x02\xC8_\x81V[a\x05\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x05\x08V[a\x02la\x05}6`\x04a\x1E\xF4V[a\x0B\x8EV[a\x02la\x05\x906`\x04a\x1F\x8FV[a\x0E\xDEV[a\x02v`u\x81V[a\x03\x04a\x05\xAB6`\x04a\x1F\xDAV[a\x0E\xF0V[a\x05\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x9DT`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra\x03NV[a\x05\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x97Ta\x06L\x90a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x84V[a\x02la\x06m6`\x04a\x1B\xC5V[a\x0F&V[a\x03\x04a\x06\x806`\x04a\x1CMV[a\x0F\xDBV[a\x03\x04a\x06\x936`\x04a\x1B\xEDV[c\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x99` R`@\x90 T\x90V[a\x02la\x06\xB86`\x04a \x18V[a\x10CV[a\x02la\x06\xCB6`\x04a \x84V[a\x12\x92V[a\x02la\x06\xDE6`\x04a\x1B\xC5V[a\x13\x08V[a\x06La'\x10\x81V[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x07\nWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x07#WP0;\x15\x80\x15a\x07#WP_T`\xFF\x16`\x01\x14[a\x07\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x07\xACW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x07\xB5\x86a\x13uV[a\x07\xBE\x85a\x13\xC6V[`\x9D\x80T`\x01`\x01`\xC0\x1B\x03\x19\x160\x17\x90Ua\x07\xDA\x84\x83a\x14\x03V[a\x07\xE3\x83a\x15\x83V[\x7F\xCE\xFE\x99\xCB.$\x0B_\x07\xDE\\\xD4r\xA7_\xC6\xE3E7\x0BsX\x8A\xB1a\xCB%\xC4\xA2Y\xA8i\x7F\xBB\x86\xFB\xC04\xF4\xE3\x82\x92\x99t\xBC\xD8A\x9E\xD6&\xB0\xEAd\x7F\x96-\x89\xBA/\xB6\xBD(xZ\xB9\x81\x90U_R`\x9C` R\x7F85:\xB4\x01\x15\xE4\x01=h\x8E\x07\xCF\xF5\x85}\xDED;\xD0^r\xC4\x9F\xCB^hJ\x9B\xB9\xEF\xC4\x80T`\xFF\x19\x16`\x01\x17\x90U`\x97\x80TBc\xFF\xFF\xFF\xFF\x16b\x01\0\0\x02e\xFF\xFF\xFF\xFF\0\0\x19\x90\x91\x16\x17\x90U\x80\x15a\x08\xBEW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[a\x08\xCEa\x15\xF4V[`fT\x81\x81\x16\x81\x14a\x08\xF3W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xFC\x82a\x13\xC6V[PPV[a\t\x08a\x16\x97V[a\t\x11\x81a\x15\x83V[PV[__a\t!\x85\x85\x85a\x0F\xDBV[`@Qc\x06\x11\x9D\r`\xE2\x1B\x81R`\x01`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x18Ft4\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xB2\x91\x90a \x9FV[\x91PP[\x93\x92PPPV[_`u\x82`@Q` \x01a\t\xD1\x91\x90a \xF0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\t\xEF\x92\x91` \x01a!\x1DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``a\n7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x16\xF1V[\x90P\x90V[a\nDa\x15\xF4V[a\nN_\x19a\x13\xC6V[V[_`\x02\x82`\x02\x81\x11\x15a\neWa\nea!FV[\x03a\n\x91WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[`\x01\x82`\x02\x81\x11\x15a\n\xA5Wa\n\xA5a!FV[\x03a\n\xD1WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[`@Qc\xFD\xEA|\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a\n\xF7a\x16\x97V[a\nN_a\x13uV[`@Qc]\xDB\x9B[`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c]\xDB\x9B[\x90a\x0BO\x90`\x98\x90`\x04\x01a!ZV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BjW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n7\x91\x90a!\x81V[`\x01a\x0B\x99\x81a\x17.V[a\x0B\xA1a\x17YV[____a\x0B\xAF\x87\x87a\x17\xB2V[_\x8F\x81R`\x9C` R`@\x90 T\x93\x97P\x91\x95P\x93P\x91P`\xFF\x16a\x0B\xE7W`@QcPEp\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R`\x98T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16` \x82\x01Ra\x0C\x1B\x90a\x17\xF9V[a\x0C$\x85a\x17\xF9V[\x03a\x0CBW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0CK\x83a\nPV[`\x01`\x01`\xA0\x1B\x03\x16c\xCD\x83\xA7+\x85\x8E`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Cx\x92\x91\x90a!\x9CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xB7\x91\x90a!\xBDV[\x15a\x0C\xC5WPPPPa\x0E\xCAV[a\x0C\xCE\x83a\nPV[`\x01`\x01`\xA0\x1B\x03\x16c]\xDB\x9B[\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xF9\x91\x90a\x1C&V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x14W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r8\x91\x90a!\x81V[c\xFF\xFF\xFF\xFF\x16\x8Cc\xFF\xFF\xFF\xFF\x16\x11a\rcW`@Qc v\x17\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x8C\x16_\x90\x81R`\x99` R`@\x90 T\x8B\x14a\r\x97W`@Qcc\x9D\t\xB5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xAD\x8B\x8B\x8B\x8Ba\r\xA8\x8C\x8Ca\x0E\xF0V[a\x18\\V[`\x02\x83`\x02\x81\x11\x15a\r\xC1Wa\r\xC1a!FV[\x03a\x0ERW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cg8\xC4\x0B\x85\x8Ea\x0E\0\x85a\x18\xC1V[\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E \x94\x93\x92\x91\x90a!\xDCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0E7W__\xFD[PZ\xF1\x15\x80\x15a\x0EIW=__>=_\xFD[PPPPa\x0E\xC5V[`\x01\x83`\x02\x81\x11\x15a\x0EfWa\x0Efa!FV[\x03a\n\xD1W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cV\xD4\x82\xF5\x85\x8Ea\x0E\xA5\x85a\x18\xDDV[\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E \x94\x93\x92\x91\x90a\"]V[PPPP[a\x0E\xD4`\x01`\xC9UV[PPPPPPPPV[a\x0E\xE6a\x16\x97V[a\x08\xFC\x82\x82a\x14\x03V[_`\x8E\x83\x83`@Q` \x01a\x0F\x07\x93\x92\x91\x90a#\x02V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[a\x0F.a\x15\xF4V[_\x81\x81R`\x9C` R`@\x90 T`\xFF\x16a\x0F\\W`@QcPEp\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xCE\xFE\x99\xCB.$\x0B_\x07\xDE\\\xD4r\xA7_\xC6\xE3E7\x0BsX\x8A\xB1a\xCB%\xC4\xA2Y\xA8i\x81\x03a\x0F\x9CW`@Qc\x19\x92\n\xFD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x9C` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x82\x91\x7F\x8B\xD4=\xE1%\x0FX\xFEn\xC9\xA7\x86q\xA8\xB7\x8D\xBAp\xF0\x01\x86V\xD1W\xA3\xAE\xAA\xBE\xC3\x89\xDF4\x91\xA2PV[`@\x80Q\x7FD\x91\xF5\xEE\x91Y_\x93\x88\x85\xEFs\xC9\xA1\xFA\x8Am\x14\xFF\x9B\x9D\xABJ\xA2K\x88\x02\xBB\xB9\xBF\xC1\xCC` \x82\x01R\x90\x81\x01\x84\x90Rc\xFF\xFF\xFF\xFF\x80\x84\x16``\x83\x01R\x82\x16`\x80\x82\x01R_\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[_a\x10M\x81a\x17.V[a\x10Ua\x17YV[B\x83c\xFF\xFF\xFF\xFF\x16\x11\x15a\x10|W`@QcZ\x11\x9D\xB5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x97Tc\xFF\xFF\xFF\xFFb\x01\0\0\x90\x91\x04\x81\x16\x90\x84\x16\x11a\x10\xAEW`@Qc\x03\x7F\xA8k`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xB9\x84\x84\x84a\x0F\xDBV[\x85` \x015\x14a\x10\xDCW`@Qc\x8BVd-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837PP`\x97T\x82Q\x92\x93Pa\xFF\xFF\x16\x91\x83\x91P_\x90a\x11\x17Wa\x11\x17a#)V[a\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@Qb_^]`\xE2\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x01}yt\x90a\x11|\x90`\x98\x90\x8B\x90\x87\x90`\x04\x01a$[V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11\x98W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xBC\x91\x90a!\xBDV[\x90P\x80a\x11\xDCW`@Qc0B\x04\x1F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x97\x80Tc\xFF\xFF\xFF\xFF\x80\x88\x16b\x01\0\0\x81\x02e\xFF\xFF\xFF\xFF\0\0\x19\x90\x93\x16\x92\x90\x92\x17\x90\x92U_\x81\x81R`\x9A` \x90\x81R`@\x80\x83 \x80T\x95\x8A\x16c\xFF\xFF\xFF\xFF\x19\x96\x87\x16\x81\x17\x90\x91U\x83R`\x9B\x82R\x80\x83 \x80T\x90\x95\x16\x84\x17\x90\x94U\x82\x82R`\x99\x81R\x83\x82 \x8A\x90U\x89\x82R`\x9C\x90R\x82\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U\x91Q\x88\x92\x7F\x01\r\xCB\xE0\xD1\xE0\x19\xC93Wq\x1F{\xB6(}T;\x7F\xF7\xDEt\xF2\x9D\xF3\xFB^\xCC\xEE\xC8\xD3i\x91\xA3PPa\x12\x8B`\x01`\xC9UV[PPPPPV[a\x12\x9Aa\x16\x97V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\x82V[a\t\x11\x81a\x13uV[a\x13\x10a\x18\xF3V[`fT\x80\x19\x82\x19\x81\x16\x14a\x137W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[\x81`\x98a\x14\x10\x82\x82a%\xE8V[PP`@Qc]\xDB\x9B[`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c]\xDB\x9B[\x90a\x14`\x90\x86\x90`\x04\x01a&yV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x9F\x91\x90a!\x81V[\x90Pc\xFF\xFF\xFF\xFF\x81\x16\x15a\x14\xC6W`@QcdF\xF9\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcg8\xC4\x0B`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cg8\xC4\x0B\x90a\x15\x1A\x90\x86\x90`\x01\x90\x87\x90`\x9D\x90`\x04\x01a&\x87V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x151W__\xFD[PZ\xF1\x15\x80\x15a\x15CW=__>=_\xFD[PPPP\x7F4cC\x1B\t\xDF\xD4=\xECsI\xF8\xF2J\xCF\xA7S\xFEL\xF4\n&#T\x02\xD2\x137=\xF1XV\x83`@Qa\x15v\x91\x90a&yV[`@Q\x80\x91\x03\x90\xA1PPPV[a'\x10a\xFF\xFF\x82\x16\x11\x15a\x15\xAAW`@Qc\x073o\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x97\x80Ta\xFF\xFF\x19\x16a\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xF5\xD1\x83m\xF8\xFC\xD7\xC1\xE5@G\xE9J\xC8w=(U9V\x03\xE2\xEF\x9B\xA5\xF5\xF1i\x05\xF2%\x92\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16z\x91\x90a!\xBDV[a\nNW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\nNW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07\x82V[``_a\x16\xFD\x83a\x19\xA4V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`fT`\x01`\xFF\x83\x16\x1B\x90\x81\x16\x03a\t\x11W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\xC9T\x03a\x17\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x07\x82V[`\x02`\xC9UV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01\x81\x90R\x90``a\x17\xE9\x85\x87\x01\x87a'`V[\x92\x99\x91\x98P\x96P\x90\x94P\x92PPPV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a\x18D\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0F \x90a(%V[a\x18\xA4\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x89\x92P\x85\x91PPc\xFF\xFF\xFF\xFF\x88\x16a\x19\xCBV[a\x12\x8BW`@Qc\xAF\xA4,\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\xC9a\x1A\xC5V[\x81\x80` \x01\x90Q\x81\x01\x90a\x0F \x91\x90a(\xADV[``\x81\x80` \x01\x90Q\x81\x01\x90a\x0F \x91\x90a)\\V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19s\x91\x90a*]V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\nNW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x0F W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83a\x19\xEAW`@Qc)\xE7'g`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a\x19\xF6\x86\x85\x85a\x1A\0V[\x14\x95\x94PPPPPV[_\x83Q_\x03a\x1A\x10WP\x81a\t\xB6V[` \x84Qa\x1A\x1E\x91\x90a*xV[\x15a\x1A<W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82` [\x85Q\x81\x11a\x1A\x9DWa\x1AS`\x02\x85a*xV[_\x03a\x1AtW\x81_R\x80\x86\x01Q` R`@_ \x91P`\x02\x84\x04\x93Pa\x1A\x8BV[\x80\x86\x01Q_R\x81` R`@_ \x91P`\x02\x84\x04\x93P[a\x1A\x96` \x82a*\x97V[\x90Pa\x1A@V[P\x82\x15a\x1A\xBDW`@Qcc\xDF\x81q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x94\x93PPPPV[`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01a\x1A\xF7`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01``\x81RP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\x11W__\xFD[_`@\x82\x84\x03\x12\x15a\x1B(W__\xFD[P\x91\x90PV[\x805a\xFF\xFF\x81\x16\x81\x14a\n\xEAW__\xFD[_`\xA0\x82\x84\x03\x12\x15a\x1B(W__\xFD[_____`\xC0\x86\x88\x03\x12\x15a\x1BcW__\xFD[\x855a\x1Bn\x81a\x1B\x04V[\x94P` \x86\x015\x93Pa\x1B\x84\x87`@\x88\x01a\x1B\x18V[\x92Pa\x1B\x92`\x80\x87\x01a\x1B.V[\x91P`\xA0\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xACW__\xFD[a\x1B\xB8\x88\x82\x89\x01a\x1B?V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a\x1B\xD5W__\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\x11W__\xFD[_` \x82\x84\x03\x12\x15a\x1B\xFDW__\xFD[\x815a\t\xB6\x81a\x1B\xDCV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`@\x81\x01a\x0F \x82\x84a\x1C\x08V[_` \x82\x84\x03\x12\x15a\x1CDW__\xFD[a\t\xB6\x82a\x1B.V[___``\x84\x86\x03\x12\x15a\x1C_W__\xFD[\x835\x92P` \x84\x015a\x1Cq\x81a\x1B\xDCV[\x91P`@\x84\x015a\x1C\x81\x81a\x1B\xDCV[\x80\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xC2Wa\x1C\xC2a\x1C\x8CV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xC2Wa\x1C\xC2a\x1C\x8CV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D\x12Wa\x1D\x12a\x1C\x8CV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1D2Wa\x1D2a\x1C\x8CV[P`\x05\x1B` \x01\x90V[_` \x82\x84\x03\x12\x15a\x1DLW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DaW__\xFD[\x82\x01\x80\x84\x03``\x81\x12\x15a\x1DsW__\xFD[a\x1D{a\x1C\xA0V[`@\x82\x12\x15a\x1D\x88W__\xFD[a\x1D\x90a\x1C\xA0V[\x835\x81R` \x80\x85\x015\x90\x82\x01R\x81R`@\x83\x015\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1D\xBAW__\xFD[\x81\x83\x01\x92P\x85`\x1F\x84\x01\x12a\x1D\xCDW__\xFD[\x825\x91Pa\x1D\xE2a\x1D\xDD\x83a\x1D\x1AV[a\x1C\xEAV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x86\x01\x01\x93P\x87\x84\x11\x15a\x1E\x03W__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15a\x1E%W\x845\x82R` \x94\x85\x01\x94\x90\x91\x01\x90a\x1E\nV[` \x83\x01RP\x95\x94PPPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1EyW__\xFD[\x815`\xFF\x81\x16\x81\x14a\t\xB6W__\xFD[\x805`\x03\x81\x10a\n\xEAW__\xFD[_` \x82\x84\x03\x12\x15a\x1E\xA7W__\xFD[a\t\xB6\x82a\x1E\x89V[__\x83`\x1F\x84\x01\x12a\x1E\xC0W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xD6W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1E\xEDW__\xFD[\x92P\x92\x90PV[_______`\xA0\x88\x8A\x03\x12\x15a\x1F\nW__\xFD[\x875a\x1F\x15\x81a\x1B\xDCV[\x96P` \x88\x015\x95P`@\x88\x015a\x1F,\x81a\x1B\xDCV[\x94P``\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FFW__\xFD[a\x1FR\x8A\x82\x8B\x01a\x1E\xB0V[\x90\x95P\x93PP`\x80\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FpW__\xFD[a\x1F|\x8A\x82\x8B\x01a\x1E\xB0V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[__``\x83\x85\x03\x12\x15a\x1F\xA0W__\xFD[a\x1F\xAA\x84\x84a\x1B\x18V[\x91P`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xC4W__\xFD[a\x1F\xD0\x85\x82\x86\x01a\x1B?V[\x91PP\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a\x1F\xEBW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a \0W__\xFD[a \x0C\x85\x82\x86\x01a\x1E\xB0V[\x90\x96\x90\x95P\x93PPPPV[____`\x80\x85\x87\x03\x12\x15a +W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a @W__\xFD[\x85\x01a\x01 \x81\x88\x03\x12\x15a RW__\xFD[\x93P` \x85\x015\x92P`@\x85\x015a i\x81a\x1B\xDCV[\x91P``\x85\x015a y\x81a\x1B\xDCV[\x93\x96\x92\x95P\x90\x93PPV[_` \x82\x84\x03\x12\x15a \x94W__\xFD[\x815a\t\xB6\x81a\x1B\x04V[_` \x82\x84\x03\x12\x15a \xAFW__\xFD[PQ\x91\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a \xE6W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a \xC8V[P\x93\x94\x93PPPPV[` \x80\x82R\x82Q\x80Q\x83\x83\x01R\x01Q`@\x82\x01R_` \x83\x01Q``\x80\x84\x01Ra\x1A\xBD`\x80\x84\x01\x82a \xB6V[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R_\x82Q\x80` \x85\x01`\x01\x85\x01^_\x92\x01`\x01\x01\x91\x82RP\x92\x91PPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`@\x81\x01a\x0F \x82\x84T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16` \x90\x91\x01RV[_` \x82\x84\x03\x12\x15a!\x91W__\xFD[\x81Qa\t\xB6\x81a\x1B\xDCV[``\x81\x01a!\xAA\x82\x85a\x1C\x08V[c\xFF\xFF\xFF\xFF\x83\x16`@\x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a!\xCDW__\xFD[\x81Q\x80\x15\x15\x81\x14a\t\xB6W__\xFD[a!\xE6\x81\x86a\x1C\x08V[c\xFF\xFF\xFF\xFF\x84\x16`@\x82\x01R`\xC0``\x82\x01R\x82Q`\xC0\x82\x01R` \x83\x01Q`\xE0\x82\x01R_`@\x84\x01Qa\"(a\x01\0\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P``\x84\x01Q`\xA0a\x01@\x84\x01Ra\"Da\x01`\x84\x01\x82a \xB6V[\x91PPa\"T`\x80\x83\x01\x84a\x1C\x08V[\x95\x94PPPPPV[_`\xC0\x82\x01a\"l\x83\x88a\x1C\x08V[c\xFF\xFF\xFF\xFF\x86\x16`@\x84\x01R`\xC0``\x84\x01R\x80\x85Q\x80\x83R`\xE0\x85\x01\x91P`\xE0\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a\"\xEDW\x86\x85\x03`\xDF\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a\"\xD7\x90\x87\x01\x82a \xB6V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\"\x9CV[PPPP\x80\x91PPa\"T`\x80\x83\x01\x84a\x1C\x08V[`\xF8\x84\x90\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x81R\x81\x83`\x01\x83\x017_\x91\x01`\x01\x01\x90\x81R\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a#RW__\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a#pW__\xFD[\x80`\x05\x1B6\x03\x82\x13\x15a\x1E\xEDW__\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[_\x825`^\x19\x836\x03\x01\x81\x12a#\xBDW__\xFD[\x90\x91\x01\x92\x91PPV[\x81\x83R_`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a#\xDDW__\xFD[\x82`\x05\x1B\x80\x83` \x87\x017\x93\x90\x93\x01` \x01\x93\x92PPPV[\x805\x82R` \x80\x82\x015\x90\x83\x01R_a$\x12`@\x83\x01\x83a#=V[```@\x86\x01Ra\"T``\x86\x01\x82\x84a#\xC6V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a \xE6W\x81Qa\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a$9V[a$~\x81\x85T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16` \x90\x91\x01RV[`\x80`@\x82\x01R_a\x01\xA0\x82\x01\x845a$\x96\x81a\x1B\xDCV[c\xFF\xFF\xFF\xFF\x16`\x80\x84\x01R` \x85\x015`\xA0\x84\x01R`@\x85\x015`\xC0\x84\x01R``\x85\x015`\xE0\x84\x01R`@`\x80\x86\x01a\x01\0\x85\x017`@`\xC0\x86\x01a\x01@\x85\x017a$\xE5a\x01\0\x86\x01\x86a#=V[a\x01 a\x01\x80\x86\x01R\x82\x81\x84Ra\x01\xC0\x86\x01\x90Pa\x01\xC0\x82`\x05\x1B\x87\x01\x01\x93P\x82_[\x83\x81\x10\x15a%\xC7W\x87\x86\x03a\x01\xBF\x19\x01\x83Ra%$\x82\x86a#\xA9V[\x805a%/\x81a\x1B\xDCV[c\xFF\xFF\xFF\xFF\x16\x87R` \x81\x0156\x82\x90\x03`\x1E\x19\x01\x81\x12a%NW__\xFD[\x81\x01` \x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a%iW__\xFD[\x806\x03\x82\x13\x15a%wW__\xFD[``` \x8A\x01Ra%\x8C``\x8A\x01\x82\x84a#\x81V[\x91PPa%\x9C`@\x83\x01\x83a#\xA9V[\x91P\x87\x81\x03`@\x89\x01Ra%\xB0\x81\x83a#\xF6V[\x97PPP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a%\x08V[PPPPP\x82\x81\x03``\x84\x01Ra%\xDE\x81\x85a$'V[\x96\x95PPPPPPV[\x815a%\xF3\x81a\x1B\x04V[\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x83U` \x84\x015a&\x1F\x81a\x1B\xDCV[`\x01`\x01`\xC0\x1B\x03\x19\x91\x90\x91\x16\x90\x91\x17`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPV[\x805a&O\x81a\x1B\x04V[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x015a&h\x81a\x1B\xDCV[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPPPV[`@\x81\x01a\x0F \x82\x84a&DV[a&\x91\x81\x86a&DV[c\xFF\xFF\xFF\xFF\x84\x16`@\x82\x81\x01\x91\x90\x91R`\xC0``\x80\x84\x01\x82\x90R\x855\x91\x84\x01\x91\x90\x91R` \x85\x015`\xE0\x84\x01R\x90\x84\x015a\x01\0\x83\x01R\x83\x015a\x01 \x82\x01R_a&\xDF`\x80\x85\x01\x85a#=V[`\xA0a\x01@\x85\x01Ra&\xF6a\x01`\x85\x01\x82\x84a#\xC6V[\x92PPPa\"T`\x80\x83\x01\x84T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16` \x90\x91\x01RV[_`@\x82\x84\x03\x12\x15a'0W__\xFD[a'8a\x1C\xA0V[\x90P\x815a'E\x81a\x1B\x04V[\x81R` \x82\x015a'U\x81a\x1B\xDCV[` \x82\x01R\x92\x91PPV[____`\xC0\x85\x87\x03\x12\x15a'sW__\xFD[a'}\x86\x86a' V[\x93Pa'\x8B`@\x86\x01a\x1E\x89V[\x92Pa'\x9A\x86``\x87\x01a' V[\x91P`\xA0\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xB4W__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a'\xC4W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xDDWa'\xDDa\x1C\x8CV[a'\xF0`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1C\xEAV[\x81\x81R\x88` \x83\x85\x01\x01\x11\x15a(\x04W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92\x95\x91\x94P\x92PV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x1B(W_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[_\x82`\x1F\x83\x01\x12a(WW__\xFD[\x81Qa(ea\x1D\xDD\x82a\x1D\x1AV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a(\x86W__\xFD[` \x85\x01[\x83\x81\x10\x15a(\xA3W\x80Q\x83R` \x92\x83\x01\x92\x01a(\x8BV[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a(\xBDW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xD2W__\xFD[\x82\x01\x80\x84\x03`\xA0\x81\x12\x15a(\xE4W__\xFD[a(\xECa\x1C\xC8V[\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R`@`?\x19\x83\x01\x12\x15a)\x0BW__\xFD[a)\x13a\x1C\xA0V[`@\x84\x81\x01Q\x82R``\x85\x01Q` \x83\x01R\x82\x01R`\x80\x83\x01Q\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a)BW__\xFD[a)N\x86\x83\x85\x01a(HV[``\x82\x01R\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a)lW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a)\x81W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a)\x91W__\xFD[\x80Qa)\x9Fa\x1D\xDD\x82a\x1D\x1AV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a)\xC0W__\xFD[` \x84\x01[\x83\x81\x10\x15a*RW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xE2W__\xFD[\x85\x01`@\x81\x8A\x03`\x1F\x19\x01\x12\x15a)\xF7W__\xFD[a)\xFFa\x1C\xA0V[` \x82\x01Qa*\r\x81a\x1B\x04V[\x81R`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a*'W__\xFD[a*6\x8B` \x83\x86\x01\x01a(HV[` \x83\x01RP\x80\x85RPP` \x83\x01\x92P` \x81\x01\x90Pa)\xC5V[P\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a*mW__\xFD[\x81Qa\t\xB6\x81a\x1B\x04V[_\x82a*\x92WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0F WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 8=\xF0EJ\xD9IkW\xE1:5~\x8F)\xEF\x95\x8A\xB9\x0C17K\x06\xE0\x03\x0E\x1F\xC9\xF1\x83AdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610255575f3560e01c80636f728c5011610140578063ad0f9582116100bf578063c3be1e3311610084578063c3be1e3314610672578063c5916a3914610685578063eaaed9d5146106aa578063f2fde38b146106bd578063fabc1cbc146106d0578063fd967f47146106e3575f5ffd5b8063ad0f9582146105b0578063b0cb3a24146105d7578063b8c1430614610617578063c252aa221461063e578063c3621f0a1461065f575f5ffd5b80638da5cb5b116101055780638da5cb5b1461055e5780639ea947781461056f5780639f7e206f14610582578063a2c902f514610595578063a2f2e24d1461059d575f5ffd5b80636f728c50146104f5578063715018a6146105205780637551ba3414610528578063790961ea14610530578063886f119514610537575f5ffd5b806331a599d2116101d757806354fd4d501161019c57806354fd4d5014610470578063595c6a67146104855780635ac86ab71461048d5780635c975abb146104b0578063612abcb0146104b857806364e1df84146104c0575f5ffd5b806331a599d2146103ea5780633ef6cd7a1461040f578063401c370f146104365780634624e6a314610449578063538a37901461045d575f5ffd5b80631e2ca2601161021d5780631e2ca260146103125780632370356c1461035b57806323b7b5b21461036e57806328522d791461039657806330ef41b4146103b8575f5ffd5b806306f5187514610259578063121409ea1461026e578063136439dd1461028d578063193b79f3146102a05780631bdc0deb146102dd575b5f5ffd5b61026c610267366004611b4f565b6106ec565b005b610276608e81565b60405160ff90911681526020015b60405180910390f35b61026c61029b366004611bc5565b6108c6565b6102c86102ae366004611bed565b63ffffffff9081165f908152609b60205260409020541690565b60405163ffffffff9091168152602001610284565b6103047fcefe99cb2e240b5f07de5cd472a75fc6e345370b73588ab161cb25c4a259a86981565b604051908152602001610284565b6040805180820182525f80825260209182015281518083019092526098546001600160a01b0381168352600160a01b900463ffffffff16908201525b6040516102849190611c26565b61026c610369366004611c34565b610900565b6102c861037c366004611bed565b63ffffffff9081165f908152609a60205260409020541690565b60975462010000900463ffffffff165f90815260996020526040902054610304565b6103da6103c6366004611bc5565b5f908152609c602052604090205460ff1690565b6040519015158152602001610284565b60975462010000900463ffffffff9081165f908152609a6020526040902054166102c8565b6103047f4491f5ee91595f938885ef73c9a1fa8a6d14ff9b9dab4aa24b8802bbb9bfc1cc81565b610304610444366004611c4d565b610914565b60975462010000900463ffffffff166102c8565b61030461046b366004611d3c565b6109bd565b610478610a0c565b6040516102849190611e34565b61026c610a3c565b6103da61049b366004611e69565b606654600160ff9092169190911b9081161490565b606654610304565b6102c8600181565b6103da6104ce366004611bed565b63ffffffff165f908152609960209081526040808320548352609c90915290205460ff1690565b610508610503366004611e97565b610a50565b6040516001600160a01b039091168152602001610284565b61026c610aef565b6102c8610b00565b6102c85f81565b6105087f000000000000000000000000000000000000000000000000000000000000000081565b6033546001600160a01b0316610508565b61026c61057d366004611ef4565b610b8e565b61026c610590366004611f8f565b610ede565b610276607581565b6103046105ab366004611fda565b610ef0565b6105087f000000000000000000000000000000000000000000000000000000000000000081565b6040805180820182525f8082526020918201528151808301909252609d546001600160a01b0381168352600160a01b900463ffffffff169082015261034e565b6105087f000000000000000000000000000000000000000000000000000000000000000081565b60975461064c9061ffff1681565b60405161ffff9091168152602001610284565b61026c61066d366004611bc5565b610f26565b610304610680366004611c4d565b610fdb565b610304610693366004611bed565b63ffffffff165f9081526099602052604090205490565b61026c6106b8366004612018565b611043565b61026c6106cb366004612084565b611292565b61026c6106de366004611bc5565b611308565b61064c61271081565b5f54610100900460ff161580801561070a57505f54600160ff909116105b806107235750303b15801561072357505f5460ff166001145b61078b5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156107ac575f805461ff0019166101001790555b6107b586611375565b6107be856113c6565b609d80546001600160c01b031916301790556107da8483611403565b6107e383611583565b7fcefe99cb2e240b5f07de5cd472a75fc6e345370b73588ab161cb25c4a259a8697fbb86fbc034f4e382929974bcd8419ed626b0ea647f962d89ba2fb6bd28785ab98190555f52609c6020527f38353ab40115e4013d688e07cff5857dde443bd05e72c49fcb5e684a9bb9efc4805460ff19166001179055609780544263ffffffff16620100000265ffffffff00001990911617905580156108be575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050505050565b6108ce6115f4565b60665481811681146108f35760405163c61dca5d60e01b815260040160405180910390fd5b6108fc826113c6565b5050565b610908611697565b61091181611583565b50565b5f5f610921858585610fdb565b6040516306119d0d60e21b815260016004820152602481018290529091507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690631846743490604401602060405180830381865afa15801561098e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906109b2919061209f565b9150505b9392505050565b5f6075826040516020016109d191906120f0565b60408051601f19818403018152908290526109ef929160200161211d565b604051602081830303815290604052805190602001209050919050565b6060610a377f00000000000000000000000000000000000000000000000000000000000000006116f1565b905090565b610a446115f4565b610a4e5f196113c6565b565b5f6002826002811115610a6557610a65612146565b03610a9157507f0000000000000000000000000000000000000000000000000000000000000000919050565b6001826002811115610aa557610aa5612146565b03610ad157507f0000000000000000000000000000000000000000000000000000000000000000919050565b60405163fdea7c0960e01b815260040160405180910390fd5b919050565b610af7611697565b610a4e5f611375565b604051635ddb9b5b60e01b81525f906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690635ddb9b5b90610b4f9060989060040161215a565b602060405180830381865afa158015610b6a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a379190612181565b6001610b998161172e565b610ba1611759565b5f5f5f5f610baf87876117b2565b5f8f8152609c60205260409020549397509195509350915060ff16610be75760405163504570e360e01b815260040160405180910390fd5b604080518082019091526098546001600160a01b0381168252600160a01b900463ffffffff166020820152610c1b906117f9565b610c24856117f9565b03610c4257604051631fb1705560e21b815260040160405180910390fd5b610c4b83610a50565b6001600160a01b031663cd83a72b858e6040518363ffffffff1660e01b8152600401610c7892919061219c565b602060405180830381865afa158015610c93573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cb791906121bd565b15610cc55750505050610eca565b610cce83610a50565b6001600160a01b0316635ddb9b5b856040518263ffffffff1660e01b8152600401610cf99190611c26565b602060405180830381865afa158015610d14573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d389190612181565b63ffffffff168c63ffffffff1611610d635760405163207617df60e01b815260040160405180910390fd5b63ffffffff8c165f908152609960205260409020548b14610d975760405163639d09b560e11b815260040160405180910390fd5b610dad8b8b8b8b610da88c8c610ef0565b61185c565b6002836002811115610dc157610dc1612146565b03610e52577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316636738c40b858e610e00856118c1565b866040518563ffffffff1660e01b8152600401610e2094939291906121dc565b5f604051808303815f87803b158015610e37575f5ffd5b505af1158015610e49573d5f5f3e3d5ffd5b50505050610ec5565b6001836002811115610e6657610e66612146565b03610ad1577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166356d482f5858e610ea5856118dd565b866040518563ffffffff1660e01b8152600401610e20949392919061225d565b505050505b610ed4600160c955565b5050505050505050565b610ee6611697565b6108fc8282611403565b5f608e8383604051602001610f0793929190612302565b6040516020818303038152906040528051906020012090505b92915050565b610f2e6115f4565b5f818152609c602052604090205460ff16610f5c5760405163504570e360e01b815260040160405180910390fd5b7fcefe99cb2e240b5f07de5cd472a75fc6e345370b73588ab161cb25c4a259a8698103610f9c576040516319920afd60e11b815260040160405180910390fd5b5f818152609c6020526040808220805460ff191690555182917f8bd43de1250f58fe6ec9a78671a8b78dba70f0018656d157a3aeaabec389df3491a250565b604080517f4491f5ee91595f938885ef73c9a1fa8a6d14ff9b9dab4aa24b8802bbb9bfc1cc602082015290810184905263ffffffff8084166060830152821660808201525f9060a0016040516020818303038152906040528051906020012090509392505050565b5f61104d8161172e565b611055611759565b428363ffffffff16111561107c57604051635a119db560e11b815260040160405180910390fd5b60975463ffffffff620100009091048116908416116110ae5760405163037fa86b60e31b815260040160405180910390fd5b6110b9848484610fdb565b8560200135146110dc57604051638b56642d60e01b815260040160405180910390fd5b6040805160018082528183019092525f91602080830190803683375050609754825192935061ffff16918391505f9061111757611117612329565b61ffff90921660209283029190910190910152604051625f5e5d60e21b81525f906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063017d79749061117c906098908b90879060040161245b565b6020604051808303815f875af1158015611198573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906111bc91906121bd565b9050806111dc57604051633042041f60e21b815260040160405180910390fd5b6097805463ffffffff80881662010000810265ffffffff000019909316929092179092555f818152609a602090815260408083208054958a1663ffffffff1996871681179091558352609b825280832080549095168417909455828252609981528382208a9055898252609c9052828120805460ff19166001179055915188927f010dcbe0d1e019c93357711f7bb6287d543b7ff7de74f29df3fb5ecceec8d36991a3505061128b600160c955565b5050505050565b61129a611697565b6001600160a01b0381166112ff5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610782565b61091181611375565b6113106118f3565b606654801982198116146113375760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c9060200160405180910390a25050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b81609861141082826125e8565b5050604051635ddb9b5b60e01b81525f906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690635ddb9b5b90611460908690600401612679565b602060405180830381865afa15801561147b573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061149f9190612181565b905063ffffffff8116156114c657604051636446f91760e01b815260040160405180910390fd5b604051636738c40b60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690636738c40b9061151a9086906001908790609d90600401612687565b5f604051808303815f87803b158015611531575f5ffd5b505af1158015611543573d5f5f3e3d5ffd5b505050507f3463431b09dfd43dec7349f8f24acfa753fe4cf40a26235402d213373df15856836040516115769190612679565b60405180910390a1505050565b61271061ffff821611156115aa576040516307336f0360e11b815260040160405180910390fd5b6097805461ffff191661ffff83169081179091556040519081527ff5d1836df8fcd7c1e54047e94ac8773d2855395603e2ef9ba5f5f16905f225929060200160405180910390a150565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015611656573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061167a91906121bd565b610a4e57604051631d77d47760e21b815260040160405180910390fd5b6033546001600160a01b03163314610a4e5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610782565b60605f6116fd836119a4565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b606654600160ff83161b908116036109115760405163840a48d560e01b815260040160405180910390fd5b600260c954036117ab5760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606401610782565b600260c955565b604080518082019091525f8082526020820152604080518082019091525f808252602082018190529060606117e985870187612760565b9299919850965090945092505050565b5f815f0151826020015163ffffffff1660405160200161184492919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b604051602081830303815290604052610f2090612825565b6118a483838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508992508591505063ffffffff88166119cb565b61128b5760405163afa42ca760e01b815260040160405180910390fd5b6118c9611ac5565b81806020019051810190610f2091906128ad565b606081806020019051810190610f20919061295c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561194f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906119739190612a5d565b6001600160a01b0316336001600160a01b031614610a4e5760405163794821ff60e01b815260040160405180910390fd5b5f60ff8216601f811115610f2057604051632cd44ac360e21b815260040160405180910390fd5b5f836119ea576040516329e7276760e11b815260040160405180910390fd5b836119f6868585611a00565b1495945050505050565b5f83515f03611a105750816109b6565b60208451611a1e9190612a78565b15611a3c576040516313717da960e21b815260040160405180910390fd5b8260205b85518111611a9d57611a53600285612a78565b5f03611a7457815f528086015160205260405f209150600284049350611a8b565b808601515f528160205260405f2091506002840493505b611a96602082612a97565b9050611a40565b508215611abd576040516363df817160e01b815260040160405180910390fd5b949350505050565b60405180608001604052805f81526020015f8152602001611af760405180604001604052805f81526020015f81525090565b8152602001606081525090565b6001600160a01b0381168114610911575f5ffd5b5f60408284031215611b28575f5ffd5b50919050565b803561ffff81168114610aea575f5ffd5b5f60a08284031215611b28575f5ffd5b5f5f5f5f5f60c08688031215611b63575f5ffd5b8535611b6e81611b04565b945060208601359350611b848760408801611b18565b9250611b9260808701611b2e565b915060a08601356001600160401b03811115611bac575f5ffd5b611bb888828901611b3f565b9150509295509295909350565b5f60208284031215611bd5575f5ffd5b5035919050565b63ffffffff81168114610911575f5ffd5b5f60208284031215611bfd575f5ffd5b81356109b681611bdc565b80516001600160a01b0316825260209081015163ffffffff16910152565b60408101610f208284611c08565b5f60208284031215611c44575f5ffd5b6109b682611b2e565b5f5f5f60608486031215611c5f575f5ffd5b833592506020840135611c7181611bdc565b91506040840135611c8181611bdc565b809150509250925092565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b0381118282101715611cc257611cc2611c8c565b60405290565b604051608081016001600160401b0381118282101715611cc257611cc2611c8c565b604051601f8201601f191681016001600160401b0381118282101715611d1257611d12611c8c565b604052919050565b5f6001600160401b03821115611d3257611d32611c8c565b5060051b60200190565b5f60208284031215611d4c575f5ffd5b81356001600160401b03811115611d61575f5ffd5b82018084036060811215611d73575f5ffd5b611d7b611ca0565b6040821215611d88575f5ffd5b611d90611ca0565b83358152602080850135908201528152604083013591506001600160401b03821115611dba575f5ffd5b818301925085601f840112611dcd575f5ffd5b82359150611de2611ddd83611d1a565b611cea565b8083825260208201915060208460051b860101935087841115611e03575f5ffd5b6020850194505b83851015611e25578435825260209485019490910190611e0a565b60208301525095945050505050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b5f60208284031215611e79575f5ffd5b813560ff811681146109b6575f5ffd5b803560038110610aea575f5ffd5b5f60208284031215611ea7575f5ffd5b6109b682611e89565b5f5f83601f840112611ec0575f5ffd5b5081356001600160401b03811115611ed6575f5ffd5b602083019150836020828501011115611eed575f5ffd5b9250929050565b5f5f5f5f5f5f5f60a0888a031215611f0a575f5ffd5b8735611f1581611bdc565b9650602088013595506040880135611f2c81611bdc565b945060608801356001600160401b03811115611f46575f5ffd5b611f528a828b01611eb0565b90955093505060808801356001600160401b03811115611f70575f5ffd5b611f7c8a828b01611eb0565b989b979a50959850939692959293505050565b5f5f60608385031215611fa0575f5ffd5b611faa8484611b18565b915060408301356001600160401b03811115611fc4575f5ffd5b611fd085828601611b3f565b9150509250929050565b5f5f60208385031215611feb575f5ffd5b82356001600160401b03811115612000575f5ffd5b61200c85828601611eb0565b90969095509350505050565b5f5f5f5f6080858703121561202b575f5ffd5b84356001600160401b03811115612040575f5ffd5b85016101208188031215612052575f5ffd5b935060208501359250604085013561206981611bdc565b9150606085013561207981611bdc565b939692955090935050565b5f60208284031215612094575f5ffd5b81356109b681611b04565b5f602082840312156120af575f5ffd5b5051919050565b5f8151808452602084019350602083015f5b828110156120e65781518652602095860195909101906001016120c8565b5093949350505050565b60208082528251805183830152015160408201525f6020830151606080840152611abd60808401826120b6565b60ff60f81b8360f81b1681525f82518060208501600185015e5f92016001019182525092915050565b634e487b7160e01b5f52602160045260245ffd5b60408101610f208284546001600160a01b038116825260a01c63ffffffff16602090910152565b5f60208284031215612191575f5ffd5b81516109b681611bdc565b606081016121aa8285611c08565b63ffffffff831660408301529392505050565b5f602082840312156121cd575f5ffd5b815180151581146109b6575f5ffd5b6121e68186611c08565b63ffffffff8416604082015260c06060820152825160c0820152602083015160e08201525f604084015161222861010084018280518252602090810151910152565b50606084015160a06101408401526122446101608401826120b6565b9150506122546080830184611c08565b95945050505050565b5f60c0820161226c8388611c08565b63ffffffff8616604084015260c0606084015280855180835260e08501915060e08160051b8601019250602087015f5b828110156122ed5786850360df19018452815180516001600160a01b031686526020908101516040918701829052906122d7908701826120b6565b955050602093840193919091019060010161229c565b50505050809150506122546080830184611c08565b60f884901b6001600160f81b0319168152818360018301375f910160010190815292915050565b634e487b7160e01b5f52603260045260245ffd5b5f5f8335601e19843603018112612352575f5ffd5b83016020810192503590506001600160401b03811115612370575f5ffd5b8060051b3603821315611eed575f5ffd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b5f8235605e198336030181126123bd575f5ffd5b90910192915050565b8183525f6001600160fb1b038311156123dd575f5ffd5b8260051b80836020870137939093016020019392505050565b80358252602080820135908301525f612412604083018361233d565b606060408601526122546060860182846123c6565b5f8151808452602084019350602083015f5b828110156120e657815161ffff16865260209586019590910190600101612439565b61247e8185546001600160a01b038116825260a01c63ffffffff16602090910152565b608060408201525f6101a08201843561249681611bdc565b63ffffffff166080840152602085013560a0840152604085013560c0840152606085013560e0840152604060808601610100850137604060c086016101408501376124e561010086018661233d565b610120610180860152828184526101c0860190506101c08260051b8701019350825f5b838110156125c7578786036101bf1901835261252482866123a9565b803561252f81611bdc565b63ffffffff168752602081013536829003601e1901811261254e575f5ffd5b81016020810190356001600160401b03811115612569575f5ffd5b803603821315612577575f5ffd5b606060208a015261258c60608a018284612381565b91505061259c60408301836123a9565b915087810360408901526125b081836123f6565b975050506020928301929190910190600101612508565b505050505082810360608401526125de8185612427565b9695505050505050565b81356125f381611b04565b81546001600160a01b031981166001600160a01b03929092169182178355602084013561261f81611bdc565b6001600160c01b03199190911690911760a09190911b63ffffffff60a01b1617905550565b803561264f81611b04565b6001600160a01b03168252602081013561266881611bdc565b63ffffffff81166020840152505050565b60408101610f208284612644565b6126918186612644565b63ffffffff841660408281019190915260c06060808401829052853591840191909152602085013560e0840152908401356101008301528301356101208201525f6126df608085018561233d565b60a06101408501526126f6610160850182846123c6565b925050506122546080830184546001600160a01b038116825260a01c63ffffffff16602090910152565b5f60408284031215612730575f5ffd5b612738611ca0565b9050813561274581611b04565b8152602082013561275581611bdc565b602082015292915050565b5f5f5f5f60c08587031215612773575f5ffd5b61277d8686612720565b935061278b60408601611e89565b925061279a8660608701612720565b915060a08501356001600160401b038111156127b4575f5ffd5b8501601f810187136127c4575f5ffd5b80356001600160401b038111156127dd576127dd611c8c565b6127f0601f8201601f1916602001611cea565b818152886020838501011115612804575f5ffd5b816020840160208301375f6020838301015280935050505092959194509250565b80516020808301519190811015611b28575f1960209190910360031b1b16919050565b5f82601f830112612857575f5ffd5b8151612865611ddd82611d1a565b8082825260208201915060208360051b860101925085831115612886575f5ffd5b602085015b838110156128a357805183526020928301920161288b565b5095945050505050565b5f602082840312156128bd575f5ffd5b81516001600160401b038111156128d2575f5ffd5b820180840360a08112156128e4575f5ffd5b6128ec611cc8565b82518152602080840151908201526040603f198301121561290b575f5ffd5b612913611ca0565b604084810151825260608501516020830152820152608083015191506001600160401b03821115612942575f5ffd5b61294e86838501612848565b606082015295945050505050565b5f6020828403121561296c575f5ffd5b81516001600160401b03811115612981575f5ffd5b8201601f81018413612991575f5ffd5b805161299f611ddd82611d1a565b8082825260208201915060208360051b8501019250868311156129c0575f5ffd5b602084015b83811015612a525780516001600160401b038111156129e2575f5ffd5b85016040818a03601f190112156129f7575f5ffd5b6129ff611ca0565b6020820151612a0d81611b04565b815260408201516001600160401b03811115612a27575f5ffd5b612a368b602083860101612848565b60208301525080855250506020830192506020810190506129c5565b509695505050505050565b5f60208284031215612a6d575f5ffd5b81516109b681611b04565b5f82612a9257634e487b7160e01b5f52601260045260245ffd5b500690565b80820180821115610f2057634e487b7160e01b5f52601160045260245ffdfea2646970667358221220383df0454ad9496b57e13a357e8f29ef958ab90c31374b06e0030e1fc9f1834164736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02UW_5`\xE0\x1C\x80cor\x8CP\x11a\x01@W\x80c\xAD\x0F\x95\x82\x11a\0\xBFW\x80c\xC3\xBE\x1E3\x11a\0\x84W\x80c\xC3\xBE\x1E3\x14a\x06rW\x80c\xC5\x91j9\x14a\x06\x85W\x80c\xEA\xAE\xD9\xD5\x14a\x06\xAAW\x80c\xF2\xFD\xE3\x8B\x14a\x06\xBDW\x80c\xFA\xBC\x1C\xBC\x14a\x06\xD0W\x80c\xFD\x96\x7FG\x14a\x06\xE3W__\xFD[\x80c\xAD\x0F\x95\x82\x14a\x05\xB0W\x80c\xB0\xCB:$\x14a\x05\xD7W\x80c\xB8\xC1C\x06\x14a\x06\x17W\x80c\xC2R\xAA\"\x14a\x06>W\x80c\xC3b\x1F\n\x14a\x06_W__\xFD[\x80c\x8D\xA5\xCB[\x11a\x01\x05W\x80c\x8D\xA5\xCB[\x14a\x05^W\x80c\x9E\xA9Gx\x14a\x05oW\x80c\x9F~ o\x14a\x05\x82W\x80c\xA2\xC9\x02\xF5\x14a\x05\x95W\x80c\xA2\xF2\xE2M\x14a\x05\x9DW__\xFD[\x80cor\x8CP\x14a\x04\xF5W\x80cqP\x18\xA6\x14a\x05 W\x80cuQ\xBA4\x14a\x05(W\x80cy\ta\xEA\x14a\x050W\x80c\x88o\x11\x95\x14a\x057W__\xFD[\x80c1\xA5\x99\xD2\x11a\x01\xD7W\x80cT\xFDMP\x11a\x01\x9CW\x80cT\xFDMP\x14a\x04pW\x80cY\\jg\x14a\x04\x85W\x80cZ\xC8j\xB7\x14a\x04\x8DW\x80c\\\x97Z\xBB\x14a\x04\xB0W\x80ca*\xBC\xB0\x14a\x04\xB8W\x80cd\xE1\xDF\x84\x14a\x04\xC0W__\xFD[\x80c1\xA5\x99\xD2\x14a\x03\xEAW\x80c>\xF6\xCDz\x14a\x04\x0FW\x80c@\x1C7\x0F\x14a\x046W\x80cF$\xE6\xA3\x14a\x04IW\x80cS\x8A7\x90\x14a\x04]W__\xFD[\x80c\x1E,\xA2`\x11a\x02\x1DW\x80c\x1E,\xA2`\x14a\x03\x12W\x80c#p5l\x14a\x03[W\x80c#\xB7\xB5\xB2\x14a\x03nW\x80c(R-y\x14a\x03\x96W\x80c0\xEFA\xB4\x14a\x03\xB8W__\xFD[\x80c\x06\xF5\x18u\x14a\x02YW\x80c\x12\x14\t\xEA\x14a\x02nW\x80c\x13d9\xDD\x14a\x02\x8DW\x80c\x19;y\xF3\x14a\x02\xA0W\x80c\x1B\xDC\r\xEB\x14a\x02\xDDW[__\xFD[a\x02la\x02g6`\x04a\x1BOV[a\x06\xECV[\0[a\x02v`\x8E\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02la\x02\x9B6`\x04a\x1B\xC5V[a\x08\xC6V[a\x02\xC8a\x02\xAE6`\x04a\x1B\xEDV[c\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`\x9B` R`@\x90 T\x16\x90V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x84V[a\x03\x04\x7F\xCE\xFE\x99\xCB.$\x0B_\x07\xDE\\\xD4r\xA7_\xC6\xE3E7\x0BsX\x8A\xB1a\xCB%\xC4\xA2Y\xA8i\x81V[`@Q\x90\x81R` \x01a\x02\x84V[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x98T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R[`@Qa\x02\x84\x91\x90a\x1C&V[a\x02la\x03i6`\x04a\x1C4V[a\t\0V[a\x02\xC8a\x03|6`\x04a\x1B\xEDV[c\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16\x90V[`\x97Tb\x01\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x99` R`@\x90 Ta\x03\x04V[a\x03\xDAa\x03\xC66`\x04a\x1B\xC5V[_\x90\x81R`\x9C` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02\x84V[`\x97Tb\x01\0\0\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`\x9A` R`@\x90 T\x16a\x02\xC8V[a\x03\x04\x7FD\x91\xF5\xEE\x91Y_\x93\x88\x85\xEFs\xC9\xA1\xFA\x8Am\x14\xFF\x9B\x9D\xABJ\xA2K\x88\x02\xBB\xB9\xBF\xC1\xCC\x81V[a\x03\x04a\x04D6`\x04a\x1CMV[a\t\x14V[`\x97Tb\x01\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16a\x02\xC8V[a\x03\x04a\x04k6`\x04a\x1D<V[a\t\xBDV[a\x04xa\n\x0CV[`@Qa\x02\x84\x91\x90a\x1E4V[a\x02la\n<V[a\x03\xDAa\x04\x9B6`\x04a\x1EiV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03\x04V[a\x02\xC8`\x01\x81V[a\x03\xDAa\x04\xCE6`\x04a\x1B\xEDV[c\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x99` \x90\x81R`@\x80\x83 T\x83R`\x9C\x90\x91R\x90 T`\xFF\x16\x90V[a\x05\x08a\x05\x036`\x04a\x1E\x97V[a\nPV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x84V[a\x02la\n\xEFV[a\x02\xC8a\x0B\0V[a\x02\xC8_\x81V[a\x05\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x05\x08V[a\x02la\x05}6`\x04a\x1E\xF4V[a\x0B\x8EV[a\x02la\x05\x906`\x04a\x1F\x8FV[a\x0E\xDEV[a\x02v`u\x81V[a\x03\x04a\x05\xAB6`\x04a\x1F\xDAV[a\x0E\xF0V[a\x05\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x9DT`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra\x03NV[a\x05\x08\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x97Ta\x06L\x90a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x84V[a\x02la\x06m6`\x04a\x1B\xC5V[a\x0F&V[a\x03\x04a\x06\x806`\x04a\x1CMV[a\x0F\xDBV[a\x03\x04a\x06\x936`\x04a\x1B\xEDV[c\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x99` R`@\x90 T\x90V[a\x02la\x06\xB86`\x04a \x18V[a\x10CV[a\x02la\x06\xCB6`\x04a \x84V[a\x12\x92V[a\x02la\x06\xDE6`\x04a\x1B\xC5V[a\x13\x08V[a\x06La'\x10\x81V[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x07\nWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x07#WP0;\x15\x80\x15a\x07#WP_T`\xFF\x16`\x01\x14[a\x07\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x07\xACW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x07\xB5\x86a\x13uV[a\x07\xBE\x85a\x13\xC6V[`\x9D\x80T`\x01`\x01`\xC0\x1B\x03\x19\x160\x17\x90Ua\x07\xDA\x84\x83a\x14\x03V[a\x07\xE3\x83a\x15\x83V[\x7F\xCE\xFE\x99\xCB.$\x0B_\x07\xDE\\\xD4r\xA7_\xC6\xE3E7\x0BsX\x8A\xB1a\xCB%\xC4\xA2Y\xA8i\x7F\xBB\x86\xFB\xC04\xF4\xE3\x82\x92\x99t\xBC\xD8A\x9E\xD6&\xB0\xEAd\x7F\x96-\x89\xBA/\xB6\xBD(xZ\xB9\x81\x90U_R`\x9C` R\x7F85:\xB4\x01\x15\xE4\x01=h\x8E\x07\xCF\xF5\x85}\xDED;\xD0^r\xC4\x9F\xCB^hJ\x9B\xB9\xEF\xC4\x80T`\xFF\x19\x16`\x01\x17\x90U`\x97\x80TBc\xFF\xFF\xFF\xFF\x16b\x01\0\0\x02e\xFF\xFF\xFF\xFF\0\0\x19\x90\x91\x16\x17\x90U\x80\x15a\x08\xBEW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPV[a\x08\xCEa\x15\xF4V[`fT\x81\x81\x16\x81\x14a\x08\xF3W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xFC\x82a\x13\xC6V[PPV[a\t\x08a\x16\x97V[a\t\x11\x81a\x15\x83V[PV[__a\t!\x85\x85\x85a\x0F\xDBV[`@Qc\x06\x11\x9D\r`\xE2\x1B\x81R`\x01`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\x18Ft4\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xB2\x91\x90a \x9FV[\x91PP[\x93\x92PPPV[_`u\x82`@Q` \x01a\t\xD1\x91\x90a \xF0V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\t\xEF\x92\x91` \x01a!\x1DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[``a\n7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x16\xF1V[\x90P\x90V[a\nDa\x15\xF4V[a\nN_\x19a\x13\xC6V[V[_`\x02\x82`\x02\x81\x11\x15a\neWa\nea!FV[\x03a\n\x91WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[`\x01\x82`\x02\x81\x11\x15a\n\xA5Wa\n\xA5a!FV[\x03a\n\xD1WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[`@Qc\xFD\xEA|\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a\n\xF7a\x16\x97V[a\nN_a\x13uV[`@Qc]\xDB\x9B[`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c]\xDB\x9B[\x90a\x0BO\x90`\x98\x90`\x04\x01a!ZV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BjW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n7\x91\x90a!\x81V[`\x01a\x0B\x99\x81a\x17.V[a\x0B\xA1a\x17YV[____a\x0B\xAF\x87\x87a\x17\xB2V[_\x8F\x81R`\x9C` R`@\x90 T\x93\x97P\x91\x95P\x93P\x91P`\xFF\x16a\x0B\xE7W`@QcPEp\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R`\x98T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16` \x82\x01Ra\x0C\x1B\x90a\x17\xF9V[a\x0C$\x85a\x17\xF9V[\x03a\x0CBW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0CK\x83a\nPV[`\x01`\x01`\xA0\x1B\x03\x16c\xCD\x83\xA7+\x85\x8E`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Cx\x92\x91\x90a!\x9CV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x93W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xB7\x91\x90a!\xBDV[\x15a\x0C\xC5WPPPPa\x0E\xCAV[a\x0C\xCE\x83a\nPV[`\x01`\x01`\xA0\x1B\x03\x16c]\xDB\x9B[\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xF9\x91\x90a\x1C&V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x14W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r8\x91\x90a!\x81V[c\xFF\xFF\xFF\xFF\x16\x8Cc\xFF\xFF\xFF\xFF\x16\x11a\rcW`@Qc v\x17\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[c\xFF\xFF\xFF\xFF\x8C\x16_\x90\x81R`\x99` R`@\x90 T\x8B\x14a\r\x97W`@Qcc\x9D\t\xB5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xAD\x8B\x8B\x8B\x8Ba\r\xA8\x8C\x8Ca\x0E\xF0V[a\x18\\V[`\x02\x83`\x02\x81\x11\x15a\r\xC1Wa\r\xC1a!FV[\x03a\x0ERW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cg8\xC4\x0B\x85\x8Ea\x0E\0\x85a\x18\xC1V[\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E \x94\x93\x92\x91\x90a!\xDCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0E7W__\xFD[PZ\xF1\x15\x80\x15a\x0EIW=__>=_\xFD[PPPPa\x0E\xC5V[`\x01\x83`\x02\x81\x11\x15a\x0EfWa\x0Efa!FV[\x03a\n\xD1W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cV\xD4\x82\xF5\x85\x8Ea\x0E\xA5\x85a\x18\xDDV[\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E \x94\x93\x92\x91\x90a\"]V[PPPP[a\x0E\xD4`\x01`\xC9UV[PPPPPPPPV[a\x0E\xE6a\x16\x97V[a\x08\xFC\x82\x82a\x14\x03V[_`\x8E\x83\x83`@Q` \x01a\x0F\x07\x93\x92\x91\x90a#\x02V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P[\x92\x91PPV[a\x0F.a\x15\xF4V[_\x81\x81R`\x9C` R`@\x90 T`\xFF\x16a\x0F\\W`@QcPEp\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\xCE\xFE\x99\xCB.$\x0B_\x07\xDE\\\xD4r\xA7_\xC6\xE3E7\x0BsX\x8A\xB1a\xCB%\xC4\xA2Y\xA8i\x81\x03a\x0F\x9CW`@Qc\x19\x92\n\xFD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x9C` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x82\x91\x7F\x8B\xD4=\xE1%\x0FX\xFEn\xC9\xA7\x86q\xA8\xB7\x8D\xBAp\xF0\x01\x86V\xD1W\xA3\xAE\xAA\xBE\xC3\x89\xDF4\x91\xA2PV[`@\x80Q\x7FD\x91\xF5\xEE\x91Y_\x93\x88\x85\xEFs\xC9\xA1\xFA\x8Am\x14\xFF\x9B\x9D\xABJ\xA2K\x88\x02\xBB\xB9\xBF\xC1\xCC` \x82\x01R\x90\x81\x01\x84\x90Rc\xFF\xFF\xFF\xFF\x80\x84\x16``\x83\x01R\x82\x16`\x80\x82\x01R_\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[_a\x10M\x81a\x17.V[a\x10Ua\x17YV[B\x83c\xFF\xFF\xFF\xFF\x16\x11\x15a\x10|W`@QcZ\x11\x9D\xB5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x97Tc\xFF\xFF\xFF\xFFb\x01\0\0\x90\x91\x04\x81\x16\x90\x84\x16\x11a\x10\xAEW`@Qc\x03\x7F\xA8k`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xB9\x84\x84\x84a\x0F\xDBV[\x85` \x015\x14a\x10\xDCW`@Qc\x8BVd-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837PP`\x97T\x82Q\x92\x93Pa\xFF\xFF\x16\x91\x83\x91P_\x90a\x11\x17Wa\x11\x17a#)V[a\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@Qb_^]`\xE2\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x01}yt\x90a\x11|\x90`\x98\x90\x8B\x90\x87\x90`\x04\x01a$[V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11\x98W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\xBC\x91\x90a!\xBDV[\x90P\x80a\x11\xDCW`@Qc0B\x04\x1F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x97\x80Tc\xFF\xFF\xFF\xFF\x80\x88\x16b\x01\0\0\x81\x02e\xFF\xFF\xFF\xFF\0\0\x19\x90\x93\x16\x92\x90\x92\x17\x90\x92U_\x81\x81R`\x9A` \x90\x81R`@\x80\x83 \x80T\x95\x8A\x16c\xFF\xFF\xFF\xFF\x19\x96\x87\x16\x81\x17\x90\x91U\x83R`\x9B\x82R\x80\x83 \x80T\x90\x95\x16\x84\x17\x90\x94U\x82\x82R`\x99\x81R\x83\x82 \x8A\x90U\x89\x82R`\x9C\x90R\x82\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U\x91Q\x88\x92\x7F\x01\r\xCB\xE0\xD1\xE0\x19\xC93Wq\x1F{\xB6(}T;\x7F\xF7\xDEt\xF2\x9D\xF3\xFB^\xCC\xEE\xC8\xD3i\x91\xA3PPa\x12\x8B`\x01`\xC9UV[PPPPPV[a\x12\x9Aa\x16\x97V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x07\x82V[a\t\x11\x81a\x13uV[a\x13\x10a\x18\xF3V[`fT\x80\x19\x82\x19\x81\x16\x14a\x137W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[\x81`\x98a\x14\x10\x82\x82a%\xE8V[PP`@Qc]\xDB\x9B[`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c]\xDB\x9B[\x90a\x14`\x90\x86\x90`\x04\x01a&yV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14{W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x9F\x91\x90a!\x81V[\x90Pc\xFF\xFF\xFF\xFF\x81\x16\x15a\x14\xC6W`@QcdF\xF9\x17`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qcg8\xC4\x0B`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cg8\xC4\x0B\x90a\x15\x1A\x90\x86\x90`\x01\x90\x87\x90`\x9D\x90`\x04\x01a&\x87V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x151W__\xFD[PZ\xF1\x15\x80\x15a\x15CW=__>=_\xFD[PPPP\x7F4cC\x1B\t\xDF\xD4=\xECsI\xF8\xF2J\xCF\xA7S\xFEL\xF4\n&#T\x02\xD2\x137=\xF1XV\x83`@Qa\x15v\x91\x90a&yV[`@Q\x80\x91\x03\x90\xA1PPPV[a'\x10a\xFF\xFF\x82\x16\x11\x15a\x15\xAAW`@Qc\x073o\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x97\x80Ta\xFF\xFF\x19\x16a\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xF5\xD1\x83m\xF8\xFC\xD7\xC1\xE5@G\xE9J\xC8w=(U9V\x03\xE2\xEF\x9B\xA5\xF5\xF1i\x05\xF2%\x92\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16z\x91\x90a!\xBDV[a\nNW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\nNW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x07\x82V[``_a\x16\xFD\x83a\x19\xA4V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`fT`\x01`\xFF\x83\x16\x1B\x90\x81\x16\x03a\t\x11W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\xC9T\x03a\x17\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x07\x82V[`\x02`\xC9UV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01\x81\x90R\x90``a\x17\xE9\x85\x87\x01\x87a'`V[\x92\x99\x91\x98P\x96P\x90\x94P\x92PPPV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a\x18D\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0F \x90a(%V[a\x18\xA4\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x89\x92P\x85\x91PPc\xFF\xFF\xFF\xFF\x88\x16a\x19\xCBV[a\x12\x8BW`@Qc\xAF\xA4,\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x18\xC9a\x1A\xC5V[\x81\x80` \x01\x90Q\x81\x01\x90a\x0F \x91\x90a(\xADV[``\x81\x80` \x01\x90Q\x81\x01\x90a\x0F \x91\x90a)\\V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19OW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19s\x91\x90a*]V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\nNW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x0F W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83a\x19\xEAW`@Qc)\xE7'g`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a\x19\xF6\x86\x85\x85a\x1A\0V[\x14\x95\x94PPPPPV[_\x83Q_\x03a\x1A\x10WP\x81a\t\xB6V[` \x84Qa\x1A\x1E\x91\x90a*xV[\x15a\x1A<W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82` [\x85Q\x81\x11a\x1A\x9DWa\x1AS`\x02\x85a*xV[_\x03a\x1AtW\x81_R\x80\x86\x01Q` R`@_ \x91P`\x02\x84\x04\x93Pa\x1A\x8BV[\x80\x86\x01Q_R\x81` R`@_ \x91P`\x02\x84\x04\x93P[a\x1A\x96` \x82a*\x97V[\x90Pa\x1A@V[P\x82\x15a\x1A\xBDW`@Qcc\xDF\x81q`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x94\x93PPPPV[`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01a\x1A\xF7`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01``\x81RP\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\t\x11W__\xFD[_`@\x82\x84\x03\x12\x15a\x1B(W__\xFD[P\x91\x90PV[\x805a\xFF\xFF\x81\x16\x81\x14a\n\xEAW__\xFD[_`\xA0\x82\x84\x03\x12\x15a\x1B(W__\xFD[_____`\xC0\x86\x88\x03\x12\x15a\x1BcW__\xFD[\x855a\x1Bn\x81a\x1B\x04V[\x94P` \x86\x015\x93Pa\x1B\x84\x87`@\x88\x01a\x1B\x18V[\x92Pa\x1B\x92`\x80\x87\x01a\x1B.V[\x91P`\xA0\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xACW__\xFD[a\x1B\xB8\x88\x82\x89\x01a\x1B?V[\x91PP\x92\x95P\x92\x95\x90\x93PV[_` \x82\x84\x03\x12\x15a\x1B\xD5W__\xFD[P5\x91\x90PV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t\x11W__\xFD[_` \x82\x84\x03\x12\x15a\x1B\xFDW__\xFD[\x815a\t\xB6\x81a\x1B\xDCV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`@\x81\x01a\x0F \x82\x84a\x1C\x08V[_` \x82\x84\x03\x12\x15a\x1CDW__\xFD[a\t\xB6\x82a\x1B.V[___``\x84\x86\x03\x12\x15a\x1C_W__\xFD[\x835\x92P` \x84\x015a\x1Cq\x81a\x1B\xDCV[\x91P`@\x84\x015a\x1C\x81\x81a\x1B\xDCV[\x80\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xC2Wa\x1C\xC2a\x1C\x8CV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xC2Wa\x1C\xC2a\x1C\x8CV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D\x12Wa\x1D\x12a\x1C\x8CV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1D2Wa\x1D2a\x1C\x8CV[P`\x05\x1B` \x01\x90V[_` \x82\x84\x03\x12\x15a\x1DLW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1DaW__\xFD[\x82\x01\x80\x84\x03``\x81\x12\x15a\x1DsW__\xFD[a\x1D{a\x1C\xA0V[`@\x82\x12\x15a\x1D\x88W__\xFD[a\x1D\x90a\x1C\xA0V[\x835\x81R` \x80\x85\x015\x90\x82\x01R\x81R`@\x83\x015\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1D\xBAW__\xFD[\x81\x83\x01\x92P\x85`\x1F\x84\x01\x12a\x1D\xCDW__\xFD[\x825\x91Pa\x1D\xE2a\x1D\xDD\x83a\x1D\x1AV[a\x1C\xEAV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x86\x01\x01\x93P\x87\x84\x11\x15a\x1E\x03W__\xFD[` \x85\x01\x94P[\x83\x85\x10\x15a\x1E%W\x845\x82R` \x94\x85\x01\x94\x90\x91\x01\x90a\x1E\nV[` \x83\x01RP\x95\x94PPPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1EyW__\xFD[\x815`\xFF\x81\x16\x81\x14a\t\xB6W__\xFD[\x805`\x03\x81\x10a\n\xEAW__\xFD[_` \x82\x84\x03\x12\x15a\x1E\xA7W__\xFD[a\t\xB6\x82a\x1E\x89V[__\x83`\x1F\x84\x01\x12a\x1E\xC0W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xD6W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1E\xEDW__\xFD[\x92P\x92\x90PV[_______`\xA0\x88\x8A\x03\x12\x15a\x1F\nW__\xFD[\x875a\x1F\x15\x81a\x1B\xDCV[\x96P` \x88\x015\x95P`@\x88\x015a\x1F,\x81a\x1B\xDCV[\x94P``\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FFW__\xFD[a\x1FR\x8A\x82\x8B\x01a\x1E\xB0V[\x90\x95P\x93PP`\x80\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FpW__\xFD[a\x1F|\x8A\x82\x8B\x01a\x1E\xB0V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[__``\x83\x85\x03\x12\x15a\x1F\xA0W__\xFD[a\x1F\xAA\x84\x84a\x1B\x18V[\x91P`@\x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xC4W__\xFD[a\x1F\xD0\x85\x82\x86\x01a\x1B?V[\x91PP\x92P\x92\x90PV[__` \x83\x85\x03\x12\x15a\x1F\xEBW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a \0W__\xFD[a \x0C\x85\x82\x86\x01a\x1E\xB0V[\x90\x96\x90\x95P\x93PPPPV[____`\x80\x85\x87\x03\x12\x15a +W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a @W__\xFD[\x85\x01a\x01 \x81\x88\x03\x12\x15a RW__\xFD[\x93P` \x85\x015\x92P`@\x85\x015a i\x81a\x1B\xDCV[\x91P``\x85\x015a y\x81a\x1B\xDCV[\x93\x96\x92\x95P\x90\x93PPV[_` \x82\x84\x03\x12\x15a \x94W__\xFD[\x815a\t\xB6\x81a\x1B\x04V[_` \x82\x84\x03\x12\x15a \xAFW__\xFD[PQ\x91\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a \xE6W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a \xC8V[P\x93\x94\x93PPPPV[` \x80\x82R\x82Q\x80Q\x83\x83\x01R\x01Q`@\x82\x01R_` \x83\x01Q``\x80\x84\x01Ra\x1A\xBD`\x80\x84\x01\x82a \xB6V[`\xFF`\xF8\x1B\x83`\xF8\x1B\x16\x81R_\x82Q\x80` \x85\x01`\x01\x85\x01^_\x92\x01`\x01\x01\x91\x82RP\x92\x91PPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`@\x81\x01a\x0F \x82\x84T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16` \x90\x91\x01RV[_` \x82\x84\x03\x12\x15a!\x91W__\xFD[\x81Qa\t\xB6\x81a\x1B\xDCV[``\x81\x01a!\xAA\x82\x85a\x1C\x08V[c\xFF\xFF\xFF\xFF\x83\x16`@\x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a!\xCDW__\xFD[\x81Q\x80\x15\x15\x81\x14a\t\xB6W__\xFD[a!\xE6\x81\x86a\x1C\x08V[c\xFF\xFF\xFF\xFF\x84\x16`@\x82\x01R`\xC0``\x82\x01R\x82Q`\xC0\x82\x01R` \x83\x01Q`\xE0\x82\x01R_`@\x84\x01Qa\"(a\x01\0\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P``\x84\x01Q`\xA0a\x01@\x84\x01Ra\"Da\x01`\x84\x01\x82a \xB6V[\x91PPa\"T`\x80\x83\x01\x84a\x1C\x08V[\x95\x94PPPPPV[_`\xC0\x82\x01a\"l\x83\x88a\x1C\x08V[c\xFF\xFF\xFF\xFF\x86\x16`@\x84\x01R`\xC0``\x84\x01R\x80\x85Q\x80\x83R`\xE0\x85\x01\x91P`\xE0\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a\"\xEDW\x86\x85\x03`\xDF\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a\"\xD7\x90\x87\x01\x82a \xB6V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\"\x9CV[PPPP\x80\x91PPa\"T`\x80\x83\x01\x84a\x1C\x08V[`\xF8\x84\x90\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x81R\x81\x83`\x01\x83\x017_\x91\x01`\x01\x01\x90\x81R\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a#RW__\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a#pW__\xFD[\x80`\x05\x1B6\x03\x82\x13\x15a\x1E\xEDW__\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[_\x825`^\x19\x836\x03\x01\x81\x12a#\xBDW__\xFD[\x90\x91\x01\x92\x91PPV[\x81\x83R_`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a#\xDDW__\xFD[\x82`\x05\x1B\x80\x83` \x87\x017\x93\x90\x93\x01` \x01\x93\x92PPPV[\x805\x82R` \x80\x82\x015\x90\x83\x01R_a$\x12`@\x83\x01\x83a#=V[```@\x86\x01Ra\"T``\x86\x01\x82\x84a#\xC6V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a \xE6W\x81Qa\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a$9V[a$~\x81\x85T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16` \x90\x91\x01RV[`\x80`@\x82\x01R_a\x01\xA0\x82\x01\x845a$\x96\x81a\x1B\xDCV[c\xFF\xFF\xFF\xFF\x16`\x80\x84\x01R` \x85\x015`\xA0\x84\x01R`@\x85\x015`\xC0\x84\x01R``\x85\x015`\xE0\x84\x01R`@`\x80\x86\x01a\x01\0\x85\x017`@`\xC0\x86\x01a\x01@\x85\x017a$\xE5a\x01\0\x86\x01\x86a#=V[a\x01 a\x01\x80\x86\x01R\x82\x81\x84Ra\x01\xC0\x86\x01\x90Pa\x01\xC0\x82`\x05\x1B\x87\x01\x01\x93P\x82_[\x83\x81\x10\x15a%\xC7W\x87\x86\x03a\x01\xBF\x19\x01\x83Ra%$\x82\x86a#\xA9V[\x805a%/\x81a\x1B\xDCV[c\xFF\xFF\xFF\xFF\x16\x87R` \x81\x0156\x82\x90\x03`\x1E\x19\x01\x81\x12a%NW__\xFD[\x81\x01` \x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a%iW__\xFD[\x806\x03\x82\x13\x15a%wW__\xFD[``` \x8A\x01Ra%\x8C``\x8A\x01\x82\x84a#\x81V[\x91PPa%\x9C`@\x83\x01\x83a#\xA9V[\x91P\x87\x81\x03`@\x89\x01Ra%\xB0\x81\x83a#\xF6V[\x97PPP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a%\x08V[PPPPP\x82\x81\x03``\x84\x01Ra%\xDE\x81\x85a$'V[\x96\x95PPPPPPV[\x815a%\xF3\x81a\x1B\x04V[\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x83U` \x84\x015a&\x1F\x81a\x1B\xDCV[`\x01`\x01`\xC0\x1B\x03\x19\x91\x90\x91\x16\x90\x91\x17`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPV[\x805a&O\x81a\x1B\x04V[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x015a&h\x81a\x1B\xDCV[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPPPV[`@\x81\x01a\x0F \x82\x84a&DV[a&\x91\x81\x86a&DV[c\xFF\xFF\xFF\xFF\x84\x16`@\x82\x81\x01\x91\x90\x91R`\xC0``\x80\x84\x01\x82\x90R\x855\x91\x84\x01\x91\x90\x91R` \x85\x015`\xE0\x84\x01R\x90\x84\x015a\x01\0\x83\x01R\x83\x015a\x01 \x82\x01R_a&\xDF`\x80\x85\x01\x85a#=V[`\xA0a\x01@\x85\x01Ra&\xF6a\x01`\x85\x01\x82\x84a#\xC6V[\x92PPPa\"T`\x80\x83\x01\x84T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16` \x90\x91\x01RV[_`@\x82\x84\x03\x12\x15a'0W__\xFD[a'8a\x1C\xA0V[\x90P\x815a'E\x81a\x1B\x04V[\x81R` \x82\x015a'U\x81a\x1B\xDCV[` \x82\x01R\x92\x91PPV[____`\xC0\x85\x87\x03\x12\x15a'sW__\xFD[a'}\x86\x86a' V[\x93Pa'\x8B`@\x86\x01a\x1E\x89V[\x92Pa'\x9A\x86``\x87\x01a' V[\x91P`\xA0\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xB4W__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a'\xC4W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xDDWa'\xDDa\x1C\x8CV[a'\xF0`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1C\xEAV[\x81\x81R\x88` \x83\x85\x01\x01\x11\x15a(\x04W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92\x95\x91\x94P\x92PV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x1B(W_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[_\x82`\x1F\x83\x01\x12a(WW__\xFD[\x81Qa(ea\x1D\xDD\x82a\x1D\x1AV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a(\x86W__\xFD[` \x85\x01[\x83\x81\x10\x15a(\xA3W\x80Q\x83R` \x92\x83\x01\x92\x01a(\x8BV[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a(\xBDW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xD2W__\xFD[\x82\x01\x80\x84\x03`\xA0\x81\x12\x15a(\xE4W__\xFD[a(\xECa\x1C\xC8V[\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R`@`?\x19\x83\x01\x12\x15a)\x0BW__\xFD[a)\x13a\x1C\xA0V[`@\x84\x81\x01Q\x82R``\x85\x01Q` \x83\x01R\x82\x01R`\x80\x83\x01Q\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a)BW__\xFD[a)N\x86\x83\x85\x01a(HV[``\x82\x01R\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a)lW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a)\x81W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a)\x91W__\xFD[\x80Qa)\x9Fa\x1D\xDD\x82a\x1D\x1AV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a)\xC0W__\xFD[` \x84\x01[\x83\x81\x10\x15a*RW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xE2W__\xFD[\x85\x01`@\x81\x8A\x03`\x1F\x19\x01\x12\x15a)\xF7W__\xFD[a)\xFFa\x1C\xA0V[` \x82\x01Qa*\r\x81a\x1B\x04V[\x81R`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a*'W__\xFD[a*6\x8B` \x83\x86\x01\x01a(HV[` \x83\x01RP\x80\x85RPP` \x83\x01\x92P` \x81\x01\x90Pa)\xC5V[P\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a*mW__\xFD[\x81Qa\t\xB6\x81a\x1B\x04V[_\x82a*\x92WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\x0F WcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 8=\xF0EJ\xD9IkW\xE1:5~\x8F)\xEF\x95\x8A\xB9\x0C17K\x06\xE0\x03\x0E\x1F\xC9\xF1\x83AdsolcC\0\x08\x1B\x003",
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
    /**Custom error with signature `CannotDisableGeneratorRoot()` and selector `0x332415fa`.
    ```solidity
    error CannotDisableGeneratorRoot();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotDisableGeneratorRoot;
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
        impl ::core::convert::From<CannotDisableGeneratorRoot> for UnderlyingRustTuple<'_> {
            fn from(value: CannotDisableGeneratorRoot) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CannotDisableGeneratorRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotDisableGeneratorRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotDisableGeneratorRoot()";
            const SELECTOR: [u8; 4] = [51u8, 36u8, 21u8, 250u8];
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
    /**Custom error with signature `CertificateInvalid()` and selector `0xc108107c`.
    ```solidity
    error CertificateInvalid();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CertificateInvalid;
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
        impl ::core::convert::From<CertificateInvalid> for UnderlyingRustTuple<'_> {
            fn from(value: CertificateInvalid) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CertificateInvalid {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CertificateInvalid {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CertificateInvalid()";
            const SELECTOR: [u8; 4] = [193u8, 8u8, 16u8, 124u8];
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
    /**Custom error with signature `GlobalTableRootInFuture()` and selector `0xb4233b6a`.
    ```solidity
    error GlobalTableRootInFuture();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GlobalTableRootInFuture;
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
        impl ::core::convert::From<GlobalTableRootInFuture> for UnderlyingRustTuple<'_> {
            fn from(value: GlobalTableRootInFuture) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GlobalTableRootInFuture {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GlobalTableRootInFuture {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GlobalTableRootInFuture()";
            const SELECTOR: [u8; 4] = [180u8, 35u8, 59u8, 106u8];
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
    /**Custom error with signature `GlobalTableRootStale()` and selector `0x1bfd4358`.
    ```solidity
    error GlobalTableRootStale();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GlobalTableRootStale;
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
        impl ::core::convert::From<GlobalTableRootStale> for UnderlyingRustTuple<'_> {
            fn from(value: GlobalTableRootStale) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GlobalTableRootStale {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GlobalTableRootStale {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GlobalTableRootStale()";
            const SELECTOR: [u8; 4] = [27u8, 253u8, 67u8, 88u8];
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
    /**Custom error with signature `InvalidConfirmationThreshold()` and selector `0x0e66de06`.
    ```solidity
    error InvalidConfirmationThreshold();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidConfirmationThreshold;
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
        impl ::core::convert::From<InvalidConfirmationThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidConfirmationThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidConfirmationThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidConfirmationThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidConfirmationThreshold()";
            const SELECTOR: [u8; 4] = [14u8, 102u8, 222u8, 6u8];
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
    /**Custom error with signature `InvalidCurveType()` and selector `0xfdea7c09`.
    ```solidity
    error InvalidCurveType();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCurveType;
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
        impl ::core::convert::From<InvalidCurveType> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCurveType) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidCurveType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCurveType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCurveType()";
            const SELECTOR: [u8; 4] = [253u8, 234u8, 124u8, 9u8];
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
    /**Custom error with signature `InvalidGenerator()` and selector `0x6446f917`.
    ```solidity
    error InvalidGenerator();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidGenerator;
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
        impl ::core::convert::From<InvalidGenerator> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidGenerator) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidGenerator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidGenerator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidGenerator()";
            const SELECTOR: [u8; 4] = [100u8, 70u8, 249u8, 23u8];
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
    /**Custom error with signature `InvalidGlobalTableRoot()` and selector `0xc73a136a`.
    ```solidity
    error InvalidGlobalTableRoot();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidGlobalTableRoot;
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
        impl ::core::convert::From<InvalidGlobalTableRoot> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidGlobalTableRoot) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidGlobalTableRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidGlobalTableRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidGlobalTableRoot()";
            const SELECTOR: [u8; 4] = [199u8, 58u8, 19u8, 106u8];
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
    /**Custom error with signature `InvalidMessageHash()` and selector `0x8b56642d`.
    ```solidity
    error InvalidMessageHash();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidMessageHash;
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
        impl ::core::convert::From<InvalidMessageHash> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidMessageHash) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidMessageHash {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidMessageHash {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidMessageHash()";
            const SELECTOR: [u8; 4] = [139u8, 86u8, 100u8, 45u8];
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
    /**Custom error with signature `InvalidNewPausedStatus()` and selector `0xc61dca5d`.
    ```solidity
    error InvalidNewPausedStatus();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidNewPausedStatus;
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
        impl ::core::convert::From<InvalidNewPausedStatus> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidNewPausedStatus) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidNewPausedStatus {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidNewPausedStatus {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidNewPausedStatus()";
            const SELECTOR: [u8; 4] = [198u8, 29u8, 202u8, 93u8];
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
    /**Custom error with signature `InvalidOperatorSet()` and selector `0x7ec5c154`.
    ```solidity
    error InvalidOperatorSet();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidOperatorSet;
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
        impl ::core::convert::From<InvalidOperatorSet> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidOperatorSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidOperatorSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidOperatorSet()";
            const SELECTOR: [u8; 4] = [126u8, 197u8, 193u8, 84u8];
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
    /**Custom error with signature `InvalidOperatorSetProof()` and selector `0xafa42ca7`.
    ```solidity
    error InvalidOperatorSetProof();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidOperatorSetProof;
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
        impl ::core::convert::From<InvalidOperatorSetProof> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidOperatorSetProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOperatorSetProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidOperatorSetProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidOperatorSetProof()";
            const SELECTOR: [u8; 4] = [175u8, 164u8, 44u8, 167u8];
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
    /**Custom error with signature `InvalidRoot()` and selector `0x504570e3`.
    ```solidity
    error InvalidRoot();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRoot;
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
        impl ::core::convert::From<InvalidRoot> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRoot) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRoot()";
            const SELECTOR: [u8; 4] = [80u8, 69u8, 112u8, 227u8];
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
    /**Custom error with signature `OnlyPauser()` and selector `0x75df51dc`.
    ```solidity
    error OnlyPauser();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyPauser;
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
        impl ::core::convert::From<OnlyPauser> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyPauser) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyPauser {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyPauser {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyPauser()";
            const SELECTOR: [u8; 4] = [117u8, 223u8, 81u8, 220u8];
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
    /**Custom error with signature `OnlyUnpauser()` and selector `0x794821ff`.
    ```solidity
    error OnlyUnpauser();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyUnpauser;
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
        impl ::core::convert::From<OnlyUnpauser> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyUnpauser) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyUnpauser {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyUnpauser {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyUnpauser()";
            const SELECTOR: [u8; 4] = [121u8, 72u8, 33u8, 255u8];
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
    /**Custom error with signature `TableUpdateForPastTimestamp()` and selector `0x207617df`.
    ```solidity
    error TableUpdateForPastTimestamp();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TableUpdateForPastTimestamp;
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
        impl ::core::convert::From<TableUpdateForPastTimestamp> for UnderlyingRustTuple<'_> {
            fn from(value: TableUpdateForPastTimestamp) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TableUpdateForPastTimestamp {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TableUpdateForPastTimestamp {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TableUpdateForPastTimestamp()";
            const SELECTOR: [u8; 4] = [32u8, 118u8, 23u8, 223u8];
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
    /**Event with signature `GeneratorUpdated((address,uint32))` and selector `0x3463431b09dfd43dec7349f8f24acfa753fe4cf40a26235402d213373df15856`.
    ```solidity
    event GeneratorUpdated(OperatorSet operatorSet);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct GeneratorUpdated {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for GeneratorUpdated {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "GeneratorUpdated((address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    52u8, 99u8, 67u8, 27u8, 9u8, 223u8, 212u8, 61u8, 236u8, 115u8, 73u8, 248u8,
                    242u8, 74u8, 207u8, 167u8, 83u8, 254u8, 76u8, 244u8, 10u8, 38u8, 35u8, 84u8,
                    2u8, 210u8, 19u8, 55u8, 61u8, 241u8, 88u8, 86u8,
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(
                    &self.operatorSet,
                ),)
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
        impl alloy_sol_types::private::IntoLogData for GeneratorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&GeneratorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &GeneratorUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `GlobalRootConfirmationThresholdUpdated(uint16)` and selector `0xf5d1836df8fcd7c1e54047e94ac8773d2855395603e2ef9ba5f5f16905f22592`.
    ```solidity
    event GlobalRootConfirmationThresholdUpdated(uint16 bps);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct GlobalRootConfirmationThresholdUpdated {
        #[allow(missing_docs)]
        pub bps: u16,
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
        impl alloy_sol_types::SolEvent for GlobalRootConfirmationThresholdUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "GlobalRootConfirmationThresholdUpdated(uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    245u8, 209u8, 131u8, 109u8, 248u8, 252u8, 215u8, 193u8, 229u8, 64u8, 71u8,
                    233u8, 74u8, 200u8, 119u8, 61u8, 40u8, 85u8, 57u8, 86u8, 3u8, 226u8, 239u8,
                    155u8, 165u8, 245u8, 241u8, 105u8, 5u8, 242u8, 37u8, 146u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { bps: data.0 }
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
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.bps,
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
        impl alloy_sol_types::private::IntoLogData for GlobalRootConfirmationThresholdUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&GlobalRootConfirmationThresholdUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &GlobalRootConfirmationThresholdUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `GlobalRootDisabled(bytes32)` and selector `0x8bd43de1250f58fe6ec9a78671a8b78dba70f0018656d157a3aeaabec389df34`.
    ```solidity
    event GlobalRootDisabled(bytes32 indexed globalTableRoot);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct GlobalRootDisabled {
        #[allow(missing_docs)]
        pub globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for GlobalRootDisabled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "GlobalRootDisabled(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 212u8, 61u8, 225u8, 37u8, 15u8, 88u8, 254u8, 110u8, 201u8, 167u8, 134u8,
                    113u8, 168u8, 183u8, 141u8, 186u8, 112u8, 240u8, 1u8, 134u8, 86u8, 209u8, 87u8,
                    163u8, 174u8, 170u8, 190u8, 195u8, 137u8, 223u8, 52u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    globalTableRoot: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.globalTableRoot.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.globalTableRoot);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for GlobalRootDisabled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&GlobalRootDisabled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &GlobalRootDisabled) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `NewGlobalTableRoot(uint32,bytes32)` and selector `0x010dcbe0d1e019c93357711f7bb6287d543b7ff7de74f29df3fb5ecceec8d369`.
    ```solidity
    event NewGlobalTableRoot(uint32 indexed referenceTimestamp, bytes32 indexed globalTableRoot);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewGlobalTableRoot {
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for NewGlobalTableRoot {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "NewGlobalTableRoot(uint32,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    1u8, 13u8, 203u8, 224u8, 209u8, 224u8, 25u8, 201u8, 51u8, 87u8, 113u8, 31u8,
                    123u8, 182u8, 40u8, 125u8, 84u8, 59u8, 127u8, 247u8, 222u8, 116u8, 242u8,
                    157u8, 243u8, 251u8, 94u8, 204u8, 238u8, 200u8, 211u8, 105u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    referenceTimestamp: topics.1,
                    globalTableRoot: topics.2,
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
                    self.referenceTimestamp.clone(),
                    self.globalTableRoot.clone(),
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
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.referenceTimestamp,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.globalTableRoot);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewGlobalTableRoot {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewGlobalTableRoot> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewGlobalTableRoot) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
    ```solidity
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8,
                    208u8, 164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8,
                    175u8, 227u8, 180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
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
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
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
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Paused(address,uint256)` and selector `0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d`.
    ```solidity
    event Paused(address indexed account, uint256 newPausedStatus);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Paused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Paused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Paused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                    152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                    224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
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
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
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
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Paused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Paused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Paused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `Unpaused(address,uint256)` and selector `0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c`.
    ```solidity
    event Unpaused(address indexed account, uint256 newPausedStatus);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Unpaused {
        #[allow(missing_docs)]
        pub account: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for Unpaused {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Unpaused(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                    188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                    107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    account: topics.1,
                    newPausedStatus: data.0,
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
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.account.clone())
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
                    &self.account,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Unpaused {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Unpaused> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Unpaused) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _bn254CertificateVerifier, address _ecdsaCertificateVerifier, address _pauserRegistry, string _version);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _bn254CertificateVerifier: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _ecdsaCertificateVerifier: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _pauserRegistry: alloy::sol_types::private::Address,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (
                        value._bn254CertificateVerifier,
                        value._ecdsaCertificateVerifier,
                        value._pauserRegistry,
                        value._version,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _bn254CertificateVerifier: tuple.0,
                        _ecdsaCertificateVerifier: tuple.1,
                        _pauserRegistry: tuple.2,
                        _version: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
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
                        &self._bn254CertificateVerifier,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._ecdsaCertificateVerifier,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._version,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `GENERATOR_GLOBAL_TABLE_ROOT()` and selector `0x1bdc0deb`.
    ```solidity
    function GENERATOR_GLOBAL_TABLE_ROOT() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GENERATOR_GLOBAL_TABLE_ROOTCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`GENERATOR_GLOBAL_TABLE_ROOT()`](GENERATOR_GLOBAL_TABLE_ROOTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GENERATOR_GLOBAL_TABLE_ROOTReturn {
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
            impl ::core::convert::From<GENERATOR_GLOBAL_TABLE_ROOTCall> for UnderlyingRustTuple<'_> {
                fn from(value: GENERATOR_GLOBAL_TABLE_ROOTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GENERATOR_GLOBAL_TABLE_ROOTCall {
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
            impl ::core::convert::From<GENERATOR_GLOBAL_TABLE_ROOTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: GENERATOR_GLOBAL_TABLE_ROOTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GENERATOR_GLOBAL_TABLE_ROOTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for GENERATOR_GLOBAL_TABLE_ROOTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GENERATOR_GLOBAL_TABLE_ROOT()";
            const SELECTOR: [u8; 4] = [27u8, 220u8, 13u8, 235u8];
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
                        let r: GENERATOR_GLOBAL_TABLE_ROOTReturn = r.into();
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
                    let r: GENERATOR_GLOBAL_TABLE_ROOTReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `GENERATOR_MAX_STALENESS_PERIOD()` and selector `0x790961ea`.
    ```solidity
    function GENERATOR_MAX_STALENESS_PERIOD() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GENERATOR_MAX_STALENESS_PERIODCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`GENERATOR_MAX_STALENESS_PERIOD()`](GENERATOR_MAX_STALENESS_PERIODCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GENERATOR_MAX_STALENESS_PERIODReturn {
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
            impl ::core::convert::From<GENERATOR_MAX_STALENESS_PERIODCall> for UnderlyingRustTuple<'_> {
                fn from(value: GENERATOR_MAX_STALENESS_PERIODCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GENERATOR_MAX_STALENESS_PERIODCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<GENERATOR_MAX_STALENESS_PERIODReturn> for UnderlyingRustTuple<'_> {
                fn from(value: GENERATOR_MAX_STALENESS_PERIODReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GENERATOR_MAX_STALENESS_PERIODReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for GENERATOR_MAX_STALENESS_PERIODCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GENERATOR_MAX_STALENESS_PERIOD()";
            const SELECTOR: [u8; 4] = [121u8, 9u8, 97u8, 234u8];
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: GENERATOR_MAX_STALENESS_PERIODReturn = r.into();
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
                    let r: GENERATOR_MAX_STALENESS_PERIODReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `GENERATOR_REFERENCE_TIMESTAMP()` and selector `0x612abcb0`.
    ```solidity
    function GENERATOR_REFERENCE_TIMESTAMP() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GENERATOR_REFERENCE_TIMESTAMPCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`GENERATOR_REFERENCE_TIMESTAMP()`](GENERATOR_REFERENCE_TIMESTAMPCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GENERATOR_REFERENCE_TIMESTAMPReturn {
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
            impl ::core::convert::From<GENERATOR_REFERENCE_TIMESTAMPCall> for UnderlyingRustTuple<'_> {
                fn from(value: GENERATOR_REFERENCE_TIMESTAMPCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GENERATOR_REFERENCE_TIMESTAMPCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<GENERATOR_REFERENCE_TIMESTAMPReturn> for UnderlyingRustTuple<'_> {
                fn from(value: GENERATOR_REFERENCE_TIMESTAMPReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GENERATOR_REFERENCE_TIMESTAMPReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for GENERATOR_REFERENCE_TIMESTAMPCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GENERATOR_REFERENCE_TIMESTAMP()";
            const SELECTOR: [u8; 4] = [97u8, 42u8, 188u8, 176u8];
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: GENERATOR_REFERENCE_TIMESTAMPReturn = r.into();
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
                    let r: GENERATOR_REFERENCE_TIMESTAMPReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `GLOBAL_TABLE_ROOT_CERT_TYPEHASH()` and selector `0x3ef6cd7a`.
    ```solidity
    function GLOBAL_TABLE_ROOT_CERT_TYPEHASH() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`GLOBAL_TABLE_ROOT_CERT_TYPEHASH()`](GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GLOBAL_TABLE_ROOT_CERT_TYPEHASHReturn {
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
            impl ::core::convert::From<GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall {
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
            impl ::core::convert::From<GLOBAL_TABLE_ROOT_CERT_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: GLOBAL_TABLE_ROOT_CERT_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GLOBAL_TABLE_ROOT_CERT_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GLOBAL_TABLE_ROOT_CERT_TYPEHASH()";
            const SELECTOR: [u8; 4] = [62u8, 246u8, 205u8, 122u8];
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
                        let r: GLOBAL_TABLE_ROOT_CERT_TYPEHASHReturn = r.into();
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
                    let r: GLOBAL_TABLE_ROOT_CERT_TYPEHASHReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `MAX_BPS()` and selector `0xfd967f47`.
    ```solidity
    function MAX_BPS() external view returns (uint16);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_BPSCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`MAX_BPS()`](MAX_BPSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_BPSReturn {
        #[allow(missing_docs)]
        pub _0: u16,
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
            impl ::core::convert::From<MAX_BPSCall> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_BPSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_BPSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u16,);
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
            impl ::core::convert::From<MAX_BPSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_BPSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_BPSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_BPSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u16;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_BPS()";
            const SELECTOR: [u8; 4] = [253u8, 150u8, 127u8, 71u8];
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
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: MAX_BPSReturn = r.into();
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
                    let r: MAX_BPSReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `OPERATOR_INFO_LEAF_SALT()` and selector `0xa2c902f5`.
    ```solidity
    function OPERATOR_INFO_LEAF_SALT() external view returns (uint8);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_INFO_LEAF_SALTCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`OPERATOR_INFO_LEAF_SALT()`](OPERATOR_INFO_LEAF_SALTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_INFO_LEAF_SALTReturn {
        #[allow(missing_docs)]
        pub _0: u8,
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
            impl ::core::convert::From<OPERATOR_INFO_LEAF_SALTCall> for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_INFO_LEAF_SALTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for OPERATOR_INFO_LEAF_SALTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
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
            impl ::core::convert::From<OPERATOR_INFO_LEAF_SALTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_INFO_LEAF_SALTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for OPERATOR_INFO_LEAF_SALTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPERATOR_INFO_LEAF_SALTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u8;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPERATOR_INFO_LEAF_SALT()";
            const SELECTOR: [u8; 4] = [162u8, 201u8, 2u8, 245u8];
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: OPERATOR_INFO_LEAF_SALTReturn = r.into();
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
                    let r: OPERATOR_INFO_LEAF_SALTReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `OPERATOR_TABLE_LEAF_SALT()` and selector `0x121409ea`.
    ```solidity
    function OPERATOR_TABLE_LEAF_SALT() external view returns (uint8);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_TABLE_LEAF_SALTCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`OPERATOR_TABLE_LEAF_SALT()`](OPERATOR_TABLE_LEAF_SALTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_TABLE_LEAF_SALTReturn {
        #[allow(missing_docs)]
        pub _0: u8,
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
            impl ::core::convert::From<OPERATOR_TABLE_LEAF_SALTCall> for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_TABLE_LEAF_SALTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for OPERATOR_TABLE_LEAF_SALTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
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
            impl ::core::convert::From<OPERATOR_TABLE_LEAF_SALTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_TABLE_LEAF_SALTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for OPERATOR_TABLE_LEAF_SALTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPERATOR_TABLE_LEAF_SALTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u8;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPERATOR_TABLE_LEAF_SALT()";
            const SELECTOR: [u8; 4] = [18u8, 20u8, 9u8, 234u8];
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: OPERATOR_TABLE_LEAF_SALTReturn = r.into();
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
                    let r: OPERATOR_TABLE_LEAF_SALTReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `bn254CertificateVerifier()` and selector `0xb8c14306`.
    ```solidity
    function bn254CertificateVerifier() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bn254CertificateVerifierCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`bn254CertificateVerifier()`](bn254CertificateVerifierCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct bn254CertificateVerifierReturn {
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
            impl ::core::convert::From<bn254CertificateVerifierCall> for UnderlyingRustTuple<'_> {
                fn from(value: bn254CertificateVerifierCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bn254CertificateVerifierCall {
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
            impl ::core::convert::From<bn254CertificateVerifierReturn> for UnderlyingRustTuple<'_> {
                fn from(value: bn254CertificateVerifierReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for bn254CertificateVerifierReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for bn254CertificateVerifierCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "bn254CertificateVerifier()";
            const SELECTOR: [u8; 4] = [184u8, 193u8, 67u8, 6u8];
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
                        let r: bn254CertificateVerifierReturn = r.into();
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
                    let r: bn254CertificateVerifierReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `calculateOperatorInfoLeaf(((uint256,uint256),uint256[]))` and selector `0x538a3790`.
    ```solidity
    function calculateOperatorInfoLeaf(IOperatorTableCalculatorTypes.BN254OperatorInfo memory operatorInfo) external pure returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorInfoLeafCall {
        #[allow(missing_docs)]
        pub operatorInfo: <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`calculateOperatorInfoLeaf(((uint256,uint256),uint256[]))`](calculateOperatorInfoLeafCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorInfoLeafReturn {
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
            impl ::core::convert::From<calculateOperatorInfoLeafCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateOperatorInfoLeafCall) -> Self {
                    (value.operatorInfo,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateOperatorInfoLeafCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorInfo: tuple.0,
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
            impl ::core::convert::From<calculateOperatorInfoLeafReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calculateOperatorInfoLeafReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateOperatorInfoLeafReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateOperatorInfoLeafCall {
            type Parameters<'a> = (IOperatorTableCalculatorTypes::BN254OperatorInfo,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "calculateOperatorInfoLeaf(((uint256,uint256),uint256[]))";
            const SELECTOR: [u8; 4] = [83u8, 138u8, 55u8, 144u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy_sol_types::SolType>::tokenize(
                        &self.operatorInfo,
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
                        let r: calculateOperatorInfoLeafReturn = r.into();
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
                    let r: calculateOperatorInfoLeafReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `calculateOperatorTableLeaf(bytes)` and selector `0xa2f2e24d`.
    ```solidity
    function calculateOperatorTableLeaf(bytes memory operatorTableBytes) external pure returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorTableLeafCall {
        #[allow(missing_docs)]
        pub operatorTableBytes: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`calculateOperatorTableLeaf(bytes)`](calculateOperatorTableLeafCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorTableLeafReturn {
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
            impl ::core::convert::From<calculateOperatorTableLeafCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateOperatorTableLeafCall) -> Self {
                    (value.operatorTableBytes,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateOperatorTableLeafCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorTableBytes: tuple.0,
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
            impl ::core::convert::From<calculateOperatorTableLeafReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calculateOperatorTableLeafReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateOperatorTableLeafReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateOperatorTableLeafCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateOperatorTableLeaf(bytes)";
            const SELECTOR: [u8; 4] = [162u8, 242u8, 226u8, 77u8];
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
                        &self.operatorTableBytes,
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
                        let r: calculateOperatorTableLeafReturn = r.into();
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
                    let r: calculateOperatorTableLeafReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize)]
    /**Function with signature `confirmGlobalTableRoot((uint32,bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint32,bytes,((uint256,uint256),uint256[]))[]),bytes32,uint32,uint32)` and selector `0xeaaed9d5`.
    ```solidity
    function confirmGlobalTableRoot(IBN254CertificateVerifierTypes.BN254Certificate memory globalTableRootCert, bytes32 globalTableRoot, uint32 referenceTimestamp, uint32 referenceBlockNumber) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmGlobalTableRootCall {
        #[allow(missing_docs)]
        pub globalTableRootCert: <IBN254CertificateVerifierTypes::BN254Certificate as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub referenceBlockNumber: u32,
    }
    ///Container type for the return parameters of the [`confirmGlobalTableRoot((uint32,bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint32,bytes,((uint256,uint256),uint256[]))[]),bytes32,uint32,uint32)`](confirmGlobalTableRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct confirmGlobalTableRootReturn {}
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
                IBN254CertificateVerifierTypes::BN254Certificate,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IBN254CertificateVerifierTypes::BN254Certificate as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::FixedBytes<32>,
                u32,
                u32,
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
            impl ::core::convert::From<confirmGlobalTableRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: confirmGlobalTableRootCall) -> Self {
                    (
                        value.globalTableRootCert,
                        value.globalTableRoot,
                        value.referenceTimestamp,
                        value.referenceBlockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for confirmGlobalTableRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        globalTableRootCert: tuple.0,
                        globalTableRoot: tuple.1,
                        referenceTimestamp: tuple.2,
                        referenceBlockNumber: tuple.3,
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
            impl ::core::convert::From<confirmGlobalTableRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: confirmGlobalTableRootReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for confirmGlobalTableRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl confirmGlobalTableRootReturn {
            fn _tokenize(
                &self,
            ) -> <confirmGlobalTableRootCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for confirmGlobalTableRootCall {
            type Parameters<'a> = (
                IBN254CertificateVerifierTypes::BN254Certificate,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = confirmGlobalTableRootReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "confirmGlobalTableRoot((uint32,bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint32,bytes,((uint256,uint256),uint256[]))[]),bytes32,uint32,uint32)";
            const SELECTOR: [u8; 4] = [234u8, 174u8, 217u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IBN254CertificateVerifierTypes::BN254Certificate as alloy_sol_types::SolType>::tokenize(
                        &self.globalTableRootCert,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.globalTableRoot),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                confirmGlobalTableRootReturn::_tokenize(ret)
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
    /**Function with signature `disableRoot(bytes32)` and selector `0xc3621f0a`.
    ```solidity
    function disableRoot(bytes32 globalTableRoot) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableRootCall {
        #[allow(missing_docs)]
        pub globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`disableRoot(bytes32)`](disableRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableRootReturn {}
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
            impl ::core::convert::From<disableRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: disableRootCall) -> Self {
                    (value.globalTableRoot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disableRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        globalTableRoot: tuple.0,
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
            impl ::core::convert::From<disableRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: disableRootReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disableRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl disableRootReturn {
            fn _tokenize(&self) -> <disableRootCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disableRootCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = disableRootReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disableRoot(bytes32)";
            const SELECTOR: [u8; 4] = [195u8, 98u8, 31u8, 10u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.globalTableRoot),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                disableRootReturn::_tokenize(ret)
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
    /**Function with signature `ecdsaCertificateVerifier()` and selector `0xad0f9582`.
    ```solidity
    function ecdsaCertificateVerifier() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ecdsaCertificateVerifierCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ecdsaCertificateVerifier()`](ecdsaCertificateVerifierCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ecdsaCertificateVerifierReturn {
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
            impl ::core::convert::From<ecdsaCertificateVerifierCall> for UnderlyingRustTuple<'_> {
                fn from(value: ecdsaCertificateVerifierCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ecdsaCertificateVerifierCall {
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
            impl ::core::convert::From<ecdsaCertificateVerifierReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ecdsaCertificateVerifierReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ecdsaCertificateVerifierReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ecdsaCertificateVerifierCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ecdsaCertificateVerifier()";
            const SELECTOR: [u8; 4] = [173u8, 15u8, 149u8, 130u8];
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
                        let r: ecdsaCertificateVerifierReturn = r.into();
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
                    let r: ecdsaCertificateVerifierReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCertificateVerifier(uint8)` and selector `0x6f728c50`.
    ```solidity
    function getCertificateVerifier(IKeyRegistrarTypes.CurveType curveType) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCertificateVerifierCall {
        #[allow(missing_docs)]
        pub curveType: <IKeyRegistrarTypes::CurveType as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCertificateVerifier(uint8)`](getCertificateVerifierCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCertificateVerifierReturn {
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
            type UnderlyingSolTuple<'a> = (IKeyRegistrarTypes::CurveType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IKeyRegistrarTypes::CurveType as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getCertificateVerifierCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCertificateVerifierCall) -> Self {
                    (value.curveType,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCertificateVerifierCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { curveType: tuple.0 }
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
            impl ::core::convert::From<getCertificateVerifierReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCertificateVerifierReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCertificateVerifierReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCertificateVerifierCall {
            type Parameters<'a> = (IKeyRegistrarTypes::CurveType,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCertificateVerifier(uint8)";
            const SELECTOR: [u8; 4] = [111u8, 114u8, 140u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IKeyRegistrarTypes::CurveType as alloy_sol_types::SolType>::tokenize(
                        &self.curveType,
                    ),
                )
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
                        let r: getCertificateVerifierReturn = r.into();
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
                    let r: getCertificateVerifierReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getCurrentGlobalTableRoot()` and selector `0x28522d79`.
    ```solidity
    function getCurrentGlobalTableRoot() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentGlobalTableRootCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getCurrentGlobalTableRoot()`](getCurrentGlobalTableRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentGlobalTableRootReturn {
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
            impl ::core::convert::From<getCurrentGlobalTableRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentGlobalTableRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentGlobalTableRootCall {
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
            impl ::core::convert::From<getCurrentGlobalTableRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentGlobalTableRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentGlobalTableRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentGlobalTableRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentGlobalTableRoot()";
            const SELECTOR: [u8; 4] = [40u8, 82u8, 45u8, 121u8];
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
                        let r: getCurrentGlobalTableRootReturn = r.into();
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
                    let r: getCurrentGlobalTableRootReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getGenerator()` and selector `0x1e2ca260`.
    ```solidity
    function getGenerator() external view returns (OperatorSet memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGeneratorCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getGenerator()`](getGeneratorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGeneratorReturn {
        #[allow(missing_docs)]
        pub _0: <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getGeneratorCall> for UnderlyingRustTuple<'_> {
                fn from(value: getGeneratorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGeneratorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
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
            impl ::core::convert::From<getGeneratorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getGeneratorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGeneratorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGeneratorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <OperatorSet as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (OperatorSet,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGenerator()";
            const SELECTOR: [u8; 4] = [30u8, 44u8, 162u8, 96u8];
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getGeneratorReturn = r.into();
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
                    let r: getGeneratorReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getGeneratorConfig()` and selector `0xb0cb3a24`.
    ```solidity
    function getGeneratorConfig() external view returns (ICrossChainRegistryTypes.OperatorSetConfig memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGeneratorConfigCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getGeneratorConfig()`](getGeneratorConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGeneratorConfigReturn {
        #[allow(missing_docs)]
        pub _0:
            <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getGeneratorConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: getGeneratorConfigCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGeneratorConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ICrossChainRegistryTypes::OperatorSetConfig,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getGeneratorConfigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getGeneratorConfigReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGeneratorConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGeneratorConfigCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (ICrossChainRegistryTypes::OperatorSetConfig,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGeneratorConfig()";
            const SELECTOR: [u8; 4] = [176u8, 203u8, 58u8, 36u8];
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
                    <ICrossChainRegistryTypes::OperatorSetConfig as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getGeneratorConfigReturn = r.into();
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
                    let r: getGeneratorConfigReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getGeneratorReferenceTimestamp()` and selector `0x7551ba34`.
    ```solidity
    function getGeneratorReferenceTimestamp() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGeneratorReferenceTimestampCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getGeneratorReferenceTimestamp()`](getGeneratorReferenceTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGeneratorReferenceTimestampReturn {
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
            impl ::core::convert::From<getGeneratorReferenceTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: getGeneratorReferenceTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGeneratorReferenceTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<getGeneratorReferenceTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getGeneratorReferenceTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGeneratorReferenceTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGeneratorReferenceTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGeneratorReferenceTimestamp()";
            const SELECTOR: [u8; 4] = [117u8, 81u8, 186u8, 52u8];
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getGeneratorReferenceTimestampReturn = r.into();
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
                    let r: getGeneratorReferenceTimestampReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getGlobalTableRootByTimestamp(uint32)` and selector `0xc5916a39`.
    ```solidity
    function getGlobalTableRootByTimestamp(uint32 referenceTimestamp) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGlobalTableRootByTimestampCall {
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getGlobalTableRootByTimestamp(uint32)`](getGlobalTableRootByTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGlobalTableRootByTimestampReturn {
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
            impl ::core::convert::From<getGlobalTableRootByTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: getGlobalTableRootByTimestampCall) -> Self {
                    (value.referenceTimestamp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGlobalTableRootByTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        referenceTimestamp: tuple.0,
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
            impl ::core::convert::From<getGlobalTableRootByTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getGlobalTableRootByTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGlobalTableRootByTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGlobalTableRootByTimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGlobalTableRootByTimestamp(uint32)";
            const SELECTOR: [u8; 4] = [197u8, 145u8, 106u8, 57u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceTimestamp,
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
                        let r: getGlobalTableRootByTimestampReturn = r.into();
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
                    let r: getGlobalTableRootByTimestampReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getGlobalTableUpdateMessageHash(bytes32,uint32,uint32)` and selector `0xc3be1e33`.
    ```solidity
    function getGlobalTableUpdateMessageHash(bytes32 globalTableRoot, uint32 referenceTimestamp, uint32 referenceBlockNumber) external pure returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGlobalTableUpdateMessageHashCall {
        #[allow(missing_docs)]
        pub globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub referenceBlockNumber: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getGlobalTableUpdateMessageHash(bytes32,uint32,uint32)`](getGlobalTableUpdateMessageHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGlobalTableUpdateMessageHashReturn {
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u32, u32);
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
            impl ::core::convert::From<getGlobalTableUpdateMessageHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: getGlobalTableUpdateMessageHashCall) -> Self {
                    (
                        value.globalTableRoot,
                        value.referenceTimestamp,
                        value.referenceBlockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGlobalTableUpdateMessageHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        globalTableRoot: tuple.0,
                        referenceTimestamp: tuple.1,
                        referenceBlockNumber: tuple.2,
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
            impl ::core::convert::From<getGlobalTableUpdateMessageHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getGlobalTableUpdateMessageHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGlobalTableUpdateMessageHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGlobalTableUpdateMessageHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getGlobalTableUpdateMessageHash(bytes32,uint32,uint32)";
            const SELECTOR: [u8; 4] = [195u8, 190u8, 30u8, 51u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.globalTableRoot),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
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
                        let r: getGlobalTableUpdateMessageHashReturn = r.into();
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
                    let r: getGlobalTableUpdateMessageHashReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getGlobalTableUpdateSignableDigest(bytes32,uint32,uint32)` and selector `0x401c370f`.
    ```solidity
    function getGlobalTableUpdateSignableDigest(bytes32 globalTableRoot, uint32 referenceTimestamp, uint32 referenceBlockNumber) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGlobalTableUpdateSignableDigestCall {
        #[allow(missing_docs)]
        pub globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub referenceBlockNumber: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getGlobalTableUpdateSignableDigest(bytes32,uint32,uint32)`](getGlobalTableUpdateSignableDigestCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGlobalTableUpdateSignableDigestReturn {
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u32, u32);
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
            impl ::core::convert::From<getGlobalTableUpdateSignableDigestCall> for UnderlyingRustTuple<'_> {
                fn from(value: getGlobalTableUpdateSignableDigestCall) -> Self {
                    (
                        value.globalTableRoot,
                        value.referenceTimestamp,
                        value.referenceBlockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGlobalTableUpdateSignableDigestCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        globalTableRoot: tuple.0,
                        referenceTimestamp: tuple.1,
                        referenceBlockNumber: tuple.2,
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
            impl ::core::convert::From<getGlobalTableUpdateSignableDigestReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getGlobalTableUpdateSignableDigestReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGlobalTableUpdateSignableDigestReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGlobalTableUpdateSignableDigestCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getGlobalTableUpdateSignableDigest(bytes32,uint32,uint32)";
            const SELECTOR: [u8; 4] = [64u8, 28u8, 55u8, 15u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.globalTableRoot),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
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
                        let r: getGlobalTableUpdateSignableDigestReturn = r.into();
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
                    let r: getGlobalTableUpdateSignableDigestReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getLatestReferenceBlockNumber()` and selector `0x31a599d2`.
    ```solidity
    function getLatestReferenceBlockNumber() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestReferenceBlockNumberCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getLatestReferenceBlockNumber()`](getLatestReferenceBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestReferenceBlockNumberReturn {
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
            impl ::core::convert::From<getLatestReferenceBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestReferenceBlockNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestReferenceBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<getLatestReferenceBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestReferenceBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestReferenceBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestReferenceBlockNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestReferenceBlockNumber()";
            const SELECTOR: [u8; 4] = [49u8, 165u8, 153u8, 210u8];
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getLatestReferenceBlockNumberReturn = r.into();
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
                    let r: getLatestReferenceBlockNumberReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getLatestReferenceTimestamp()` and selector `0x4624e6a3`.
    ```solidity
    function getLatestReferenceTimestamp() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestReferenceTimestampCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getLatestReferenceTimestamp()`](getLatestReferenceTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestReferenceTimestampReturn {
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
            impl ::core::convert::From<getLatestReferenceTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestReferenceTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestReferenceTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<getLatestReferenceTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestReferenceTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestReferenceTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestReferenceTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestReferenceTimestamp()";
            const SELECTOR: [u8; 4] = [70u8, 36u8, 230u8, 163u8];
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getLatestReferenceTimestampReturn = r.into();
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
                    let r: getLatestReferenceTimestampReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getReferenceBlockNumberByTimestamp(uint32)` and selector `0x23b7b5b2`.
    ```solidity
    function getReferenceBlockNumberByTimestamp(uint32 referenceTimestamp) external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getReferenceBlockNumberByTimestampCall {
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getReferenceBlockNumberByTimestamp(uint32)`](getReferenceBlockNumberByTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getReferenceBlockNumberByTimestampReturn {
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
            impl ::core::convert::From<getReferenceBlockNumberByTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: getReferenceBlockNumberByTimestampCall) -> Self {
                    (value.referenceTimestamp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getReferenceBlockNumberByTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        referenceTimestamp: tuple.0,
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
            impl ::core::convert::From<getReferenceBlockNumberByTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getReferenceBlockNumberByTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getReferenceBlockNumberByTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getReferenceBlockNumberByTimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getReferenceBlockNumberByTimestamp(uint32)";
            const SELECTOR: [u8; 4] = [35u8, 183u8, 181u8, 178u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceTimestamp,
                    ),
                )
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
                        let r: getReferenceBlockNumberByTimestampReturn = r.into();
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
                    let r: getReferenceBlockNumberByTimestampReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getReferenceTimestampByBlockNumber(uint32)` and selector `0x193b79f3`.
    ```solidity
    function getReferenceTimestampByBlockNumber(uint32 referenceBlockNumber) external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getReferenceTimestampByBlockNumberCall {
        #[allow(missing_docs)]
        pub referenceBlockNumber: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getReferenceTimestampByBlockNumber(uint32)`](getReferenceTimestampByBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getReferenceTimestampByBlockNumberReturn {
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
            impl ::core::convert::From<getReferenceTimestampByBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getReferenceTimestampByBlockNumberCall) -> Self {
                    (value.referenceBlockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getReferenceTimestampByBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        referenceBlockNumber: tuple.0,
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
            impl ::core::convert::From<getReferenceTimestampByBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getReferenceTimestampByBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getReferenceTimestampByBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getReferenceTimestampByBlockNumberCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getReferenceTimestampByBlockNumber(uint32)";
            const SELECTOR: [u8; 4] = [25u8, 59u8, 121u8, 243u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.referenceBlockNumber,
                    ),
                )
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
                        let r: getReferenceTimestampByBlockNumberReturn = r.into();
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
                    let r: getReferenceTimestampByBlockNumberReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `globalRootConfirmationThreshold()` and selector `0xc252aa22`.
    ```solidity
    function globalRootConfirmationThreshold() external view returns (uint16);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct globalRootConfirmationThresholdCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`globalRootConfirmationThreshold()`](globalRootConfirmationThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct globalRootConfirmationThresholdReturn {
        #[allow(missing_docs)]
        pub _0: u16,
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
            impl ::core::convert::From<globalRootConfirmationThresholdCall> for UnderlyingRustTuple<'_> {
                fn from(value: globalRootConfirmationThresholdCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for globalRootConfirmationThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u16,);
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
            impl ::core::convert::From<globalRootConfirmationThresholdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: globalRootConfirmationThresholdReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for globalRootConfirmationThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for globalRootConfirmationThresholdCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u16;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "globalRootConfirmationThreshold()";
            const SELECTOR: [u8; 4] = [194u8, 82u8, 170u8, 34u8];
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
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: globalRootConfirmationThresholdReturn = r.into();
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
                    let r: globalRootConfirmationThresholdReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initialize(address,uint256,(address,uint32),uint16,(bytes32,uint256,(uint256,uint256),uint256[]))` and selector `0x06f51875`.
    ```solidity
    function initialize(address _owner, uint256 initialPausedStatus, OperatorSet memory _initialGenerator, uint16 _globalRootConfirmationThreshold, IOperatorTableCalculatorTypes.BN254OperatorSetInfo memory generatorInfo) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub _owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _initialGenerator: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _globalRootConfirmationThreshold: u16,
        #[allow(missing_docs)]
        pub generatorInfo: <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initialize(address,uint256,(address,uint32),uint16,(bytes32,uint256,(uint256,uint256),uint256[]))`](initializeCall) function.
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                OperatorSet,
                alloy::sol_types::sol_data::Uint<16>,
                IOperatorTableCalculatorTypes::BN254OperatorSetInfo,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                u16,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value._owner,
                        value.initialPausedStatus,
                        value._initialGenerator,
                        value._globalRootConfirmationThreshold,
                        value.generatorInfo,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _owner: tuple.0,
                        initialPausedStatus: tuple.1,
                        _initialGenerator: tuple.2,
                        _globalRootConfirmationThreshold: tuple.3,
                        generatorInfo: tuple.4,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                OperatorSet,
                alloy::sol_types::sol_data::Uint<16>,
                IOperatorTableCalculatorTypes::BN254OperatorSetInfo,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,uint256,(address,uint32),uint16,(bytes32,uint256,(uint256,uint256),uint256[]))";
            const SELECTOR: [u8; 4] = [6u8, 245u8, 24u8, 117u8];
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
                        &self._owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialPausedStatus),
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self._initialGenerator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._globalRootConfirmationThreshold,
                    ),
                    <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy_sol_types::SolType>::tokenize(
                        &self.generatorInfo,
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
    /**Function with signature `isRootValid(bytes32)` and selector `0x30ef41b4`.
    ```solidity
    function isRootValid(bytes32 globalTableRoot) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRootValidCall {
        #[allow(missing_docs)]
        pub globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isRootValid(bytes32)`](isRootValidCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRootValidReturn {
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
            impl ::core::convert::From<isRootValidCall> for UnderlyingRustTuple<'_> {
                fn from(value: isRootValidCall) -> Self {
                    (value.globalTableRoot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRootValidCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        globalTableRoot: tuple.0,
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
            impl ::core::convert::From<isRootValidReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isRootValidReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRootValidReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRootValidCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isRootValid(bytes32)";
            const SELECTOR: [u8; 4] = [48u8, 239u8, 65u8, 180u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.globalTableRoot),
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
                        let r: isRootValidReturn = r.into();
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
                    let r: isRootValidReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isRootValidByTimestamp(uint32)` and selector `0x64e1df84`.
    ```solidity
    function isRootValidByTimestamp(uint32 referenceTimestamp) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRootValidByTimestampCall {
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isRootValidByTimestamp(uint32)`](isRootValidByTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRootValidByTimestampReturn {
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
            impl ::core::convert::From<isRootValidByTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: isRootValidByTimestampCall) -> Self {
                    (value.referenceTimestamp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRootValidByTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        referenceTimestamp: tuple.0,
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
            impl ::core::convert::From<isRootValidByTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isRootValidByTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRootValidByTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRootValidByTimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isRootValidByTimestamp(uint32)";
            const SELECTOR: [u8; 4] = [100u8, 225u8, 223u8, 132u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
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
                        let r: isRootValidByTimestampReturn = r.into();
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
                    let r: isRootValidByTimestampReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
    ```solidity
    function owner() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
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
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
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
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
                        let r: ownerReturn = r.into();
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
                    let r: ownerReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pause(uint256)` and selector `0x136439dd`.
    ```solidity
    function pause(uint256 newPausedStatus) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseCall {
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`pause(uint256)`](pauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseReturn {}
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
            impl ::core::convert::From<pauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPausedStatus: tuple.0,
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
            impl ::core::convert::From<pauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl pauseReturn {
            fn _tokenize(&self) -> <pauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pause(uint256)";
            const SELECTOR: [u8; 4] = [19u8, 100u8, 57u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                pauseReturn::_tokenize(ret)
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
    /**Function with signature `pauseAll()` and selector `0x595c6a67`.
    ```solidity
    function pauseAll() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllCall;
    ///Container type for the return parameters of the [`pauseAll()`](pauseAllCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauseAllReturn {}
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
            impl ::core::convert::From<pauseAllCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<pauseAllReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauseAllReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauseAllReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl pauseAllReturn {
            fn _tokenize(&self) -> <pauseAllCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauseAllCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pauseAllReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauseAll()";
            const SELECTOR: [u8; 4] = [89u8, 92u8, 106u8, 103u8];
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
                pauseAllReturn::_tokenize(ret)
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
    /**Function with signature `paused(uint8)` and selector `0x5ac86ab7`.
    ```solidity
    function paused(uint8 index) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Call {
        #[allow(missing_docs)]
        pub index: u8,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paused(uint8)`](paused_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_0Return {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8,);
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
            impl ::core::convert::From<paused_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Call) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
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
            impl ::core::convert::From<paused_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused(uint8)";
            const SELECTOR: [u8; 4] = [90u8, 200u8, 106u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.index,
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
                        let r: paused_0Return = r.into();
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
                    let r: paused_0Return = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `paused()` and selector `0x5c975abb`.
    ```solidity
    function paused() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Call;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`paused()`](paused_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct paused_1Return {
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
            impl ::core::convert::From<paused_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Call) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Call {
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
            impl ::core::convert::From<paused_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: paused_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for paused_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for paused_1Call {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "paused()";
            const SELECTOR: [u8; 4] = [92u8, 151u8, 90u8, 187u8];
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
                        let r: paused_1Return = r.into();
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
                    let r: paused_1Return = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `pauserRegistry()` and selector `0x886f1195`.
    ```solidity
    function pauserRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`pauserRegistry()`](pauserRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pauserRegistryReturn {
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
            impl ::core::convert::From<pauserRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauserRegistryCall {
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
            impl ::core::convert::From<pauserRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pauserRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pauserRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pauserRegistry()";
            const SELECTOR: [u8; 4] = [136u8, 111u8, 17u8, 149u8];
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
                        let r: pauserRegistryReturn = r.into();
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
                    let r: pauserRegistryReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
    ```solidity
    function renounceOwnership() external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall;
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
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
            impl ::core::convert::From<renounceOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<renounceOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl renounceOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <renounceOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
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
                renounceOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `setGlobalRootConfirmationThreshold(uint16)` and selector `0x2370356c`.
    ```solidity
    function setGlobalRootConfirmationThreshold(uint16 bps) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGlobalRootConfirmationThresholdCall {
        #[allow(missing_docs)]
        pub bps: u16,
    }
    ///Container type for the return parameters of the [`setGlobalRootConfirmationThreshold(uint16)`](setGlobalRootConfirmationThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGlobalRootConfirmationThresholdReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u16,);
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
            impl ::core::convert::From<setGlobalRootConfirmationThresholdCall> for UnderlyingRustTuple<'_> {
                fn from(value: setGlobalRootConfirmationThresholdCall) -> Self {
                    (value.bps,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setGlobalRootConfirmationThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { bps: tuple.0 }
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
            impl ::core::convert::From<setGlobalRootConfirmationThresholdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setGlobalRootConfirmationThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setGlobalRootConfirmationThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setGlobalRootConfirmationThresholdReturn {
            fn _tokenize(
                &self,
            ) -> <setGlobalRootConfirmationThresholdCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setGlobalRootConfirmationThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setGlobalRootConfirmationThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setGlobalRootConfirmationThreshold(uint16)";
            const SELECTOR: [u8; 4] = [35u8, 112u8, 53u8, 108u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.bps,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setGlobalRootConfirmationThresholdReturn::_tokenize(ret)
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
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
    ```solidity
    function transferOwnership(address newOwner) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
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
            impl ::core::convert::From<transferOwnershipCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
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
            impl ::core::convert::From<transferOwnershipReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl transferOwnershipReturn {
            fn _tokenize(
                &self,
            ) -> <transferOwnershipCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
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
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                transferOwnershipReturn::_tokenize(ret)
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
    /**Function with signature `unpause(uint256)` and selector `0xfabc1cbc`.
    ```solidity
    function unpause(uint256 newPausedStatus) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseCall {
        #[allow(missing_docs)]
        pub newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`unpause(uint256)`](unpauseCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct unpauseReturn {}
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
            impl ::core::convert::From<unpauseCall> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseCall) -> Self {
                    (value.newPausedStatus,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPausedStatus: tuple.0,
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
            impl ::core::convert::From<unpauseReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unpauseReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unpauseReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl unpauseReturn {
            fn _tokenize(&self) -> <unpauseCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unpauseCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = unpauseReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unpause(uint256)";
            const SELECTOR: [u8; 4] = [250u8, 188u8, 28u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.newPausedStatus,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                unpauseReturn::_tokenize(ret)
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
    /**Function with signature `updateGenerator((address,uint32),(bytes32,uint256,(uint256,uint256),uint256[]))` and selector `0x9f7e206f`.
    ```solidity
    function updateGenerator(OperatorSet memory generator, IOperatorTableCalculatorTypes.BN254OperatorSetInfo memory generatorInfo) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateGeneratorCall {
        #[allow(missing_docs)]
        pub generator: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub generatorInfo: <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`updateGenerator((address,uint32),(bytes32,uint256,(uint256,uint256),uint256[]))`](updateGeneratorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateGeneratorReturn {}
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
                IOperatorTableCalculatorTypes::BN254OperatorSetInfo,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<updateGeneratorCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateGeneratorCall) -> Self {
                    (value.generator, value.generatorInfo)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateGeneratorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        generator: tuple.0,
                        generatorInfo: tuple.1,
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
            impl ::core::convert::From<updateGeneratorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateGeneratorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateGeneratorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateGeneratorReturn {
            fn _tokenize(
                &self,
            ) -> <updateGeneratorCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateGeneratorCall {
            type Parameters<'a> = (
                OperatorSet,
                IOperatorTableCalculatorTypes::BN254OperatorSetInfo,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateGeneratorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "updateGenerator((address,uint32),(bytes32,uint256,(uint256,uint256),uint256[]))";
            const SELECTOR: [u8; 4] = [159u8, 126u8, 32u8, 111u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.generator),
                    <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy_sol_types::SolType>::tokenize(
                        &self.generatorInfo,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateGeneratorReturn::_tokenize(ret)
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
    /**Function with signature `updateOperatorTable(uint32,bytes32,uint32,bytes,bytes)` and selector `0x9ea94778`.
    ```solidity
    function updateOperatorTable(uint32 referenceTimestamp, bytes32 globalTableRoot, uint32 operatorSetIndex, bytes memory proof, bytes memory operatorTableBytes) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorTableCall {
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub operatorSetIndex: u32,
        #[allow(missing_docs)]
        pub proof: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub operatorTableBytes: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`updateOperatorTable(uint32,bytes32,uint32,bytes,bytes)`](updateOperatorTableCall) function.
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
                alloy::sol_types::private::FixedBytes<32>,
                u32,
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
            impl ::core::convert::From<updateOperatorTableCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorTableCall) -> Self {
                    (
                        value.referenceTimestamp,
                        value.globalTableRoot,
                        value.operatorSetIndex,
                        value.proof,
                        value.operatorTableBytes,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorTableCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        referenceTimestamp: tuple.0,
                        globalTableRoot: tuple.1,
                        operatorSetIndex: tuple.2,
                        proof: tuple.3,
                        operatorTableBytes: tuple.4,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorTableReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "updateOperatorTable(uint32,bytes32,uint32,bytes,bytes)";
            const SELECTOR: [u8; 4] = [158u8, 169u8, 71u8, 120u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.globalTableRoot),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIndex),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.proof,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.operatorTableBytes,
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
    ///Container for all the [`OperatorTableUpdater`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum OperatorTableUpdaterCalls {
        #[allow(missing_docs)]
        GENERATOR_GLOBAL_TABLE_ROOT(GENERATOR_GLOBAL_TABLE_ROOTCall),
        #[allow(missing_docs)]
        GENERATOR_MAX_STALENESS_PERIOD(GENERATOR_MAX_STALENESS_PERIODCall),
        #[allow(missing_docs)]
        GENERATOR_REFERENCE_TIMESTAMP(GENERATOR_REFERENCE_TIMESTAMPCall),
        #[allow(missing_docs)]
        GLOBAL_TABLE_ROOT_CERT_TYPEHASH(GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall),
        #[allow(missing_docs)]
        MAX_BPS(MAX_BPSCall),
        #[allow(missing_docs)]
        OPERATOR_INFO_LEAF_SALT(OPERATOR_INFO_LEAF_SALTCall),
        #[allow(missing_docs)]
        OPERATOR_TABLE_LEAF_SALT(OPERATOR_TABLE_LEAF_SALTCall),
        #[allow(missing_docs)]
        bn254CertificateVerifier(bn254CertificateVerifierCall),
        #[allow(missing_docs)]
        calculateOperatorInfoLeaf(calculateOperatorInfoLeafCall),
        #[allow(missing_docs)]
        calculateOperatorTableLeaf(calculateOperatorTableLeafCall),
        #[allow(missing_docs)]
        confirmGlobalTableRoot(confirmGlobalTableRootCall),
        #[allow(missing_docs)]
        disableRoot(disableRootCall),
        #[allow(missing_docs)]
        ecdsaCertificateVerifier(ecdsaCertificateVerifierCall),
        #[allow(missing_docs)]
        getCertificateVerifier(getCertificateVerifierCall),
        #[allow(missing_docs)]
        getCurrentGlobalTableRoot(getCurrentGlobalTableRootCall),
        #[allow(missing_docs)]
        getGenerator(getGeneratorCall),
        #[allow(missing_docs)]
        getGeneratorConfig(getGeneratorConfigCall),
        #[allow(missing_docs)]
        getGeneratorReferenceTimestamp(getGeneratorReferenceTimestampCall),
        #[allow(missing_docs)]
        getGlobalTableRootByTimestamp(getGlobalTableRootByTimestampCall),
        #[allow(missing_docs)]
        getGlobalTableUpdateMessageHash(getGlobalTableUpdateMessageHashCall),
        #[allow(missing_docs)]
        getGlobalTableUpdateSignableDigest(getGlobalTableUpdateSignableDigestCall),
        #[allow(missing_docs)]
        getLatestReferenceBlockNumber(getLatestReferenceBlockNumberCall),
        #[allow(missing_docs)]
        getLatestReferenceTimestamp(getLatestReferenceTimestampCall),
        #[allow(missing_docs)]
        getReferenceBlockNumberByTimestamp(getReferenceBlockNumberByTimestampCall),
        #[allow(missing_docs)]
        getReferenceTimestampByBlockNumber(getReferenceTimestampByBlockNumberCall),
        #[allow(missing_docs)]
        globalRootConfirmationThreshold(globalRootConfirmationThresholdCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isRootValid(isRootValidCall),
        #[allow(missing_docs)]
        isRootValidByTimestamp(isRootValidByTimestampCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        pause(pauseCall),
        #[allow(missing_docs)]
        pauseAll(pauseAllCall),
        #[allow(missing_docs)]
        paused_0(paused_0Call),
        #[allow(missing_docs)]
        paused_1(paused_1Call),
        #[allow(missing_docs)]
        pauserRegistry(pauserRegistryCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        setGlobalRootConfirmationThreshold(setGlobalRootConfirmationThresholdCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
        updateGenerator(updateGeneratorCall),
        #[allow(missing_docs)]
        updateOperatorTable(updateOperatorTableCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    #[automatically_derived]
    impl OperatorTableUpdaterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [6u8, 245u8, 24u8, 117u8],
            [18u8, 20u8, 9u8, 234u8],
            [19u8, 100u8, 57u8, 221u8],
            [25u8, 59u8, 121u8, 243u8],
            [27u8, 220u8, 13u8, 235u8],
            [30u8, 44u8, 162u8, 96u8],
            [35u8, 112u8, 53u8, 108u8],
            [35u8, 183u8, 181u8, 178u8],
            [40u8, 82u8, 45u8, 121u8],
            [48u8, 239u8, 65u8, 180u8],
            [49u8, 165u8, 153u8, 210u8],
            [62u8, 246u8, 205u8, 122u8],
            [64u8, 28u8, 55u8, 15u8],
            [70u8, 36u8, 230u8, 163u8],
            [83u8, 138u8, 55u8, 144u8],
            [84u8, 253u8, 77u8, 80u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [97u8, 42u8, 188u8, 176u8],
            [100u8, 225u8, 223u8, 132u8],
            [111u8, 114u8, 140u8, 80u8],
            [113u8, 80u8, 24u8, 166u8],
            [117u8, 81u8, 186u8, 52u8],
            [121u8, 9u8, 97u8, 234u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [158u8, 169u8, 71u8, 120u8],
            [159u8, 126u8, 32u8, 111u8],
            [162u8, 201u8, 2u8, 245u8],
            [162u8, 242u8, 226u8, 77u8],
            [173u8, 15u8, 149u8, 130u8],
            [176u8, 203u8, 58u8, 36u8],
            [184u8, 193u8, 67u8, 6u8],
            [194u8, 82u8, 170u8, 34u8],
            [195u8, 98u8, 31u8, 10u8],
            [195u8, 190u8, 30u8, 51u8],
            [197u8, 145u8, 106u8, 57u8],
            [234u8, 174u8, 217u8, 213u8],
            [242u8, 253u8, 227u8, 139u8],
            [250u8, 188u8, 28u8, 188u8],
            [253u8, 150u8, 127u8, 71u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OperatorTableUpdaterCalls {
        const NAME: &'static str = "OperatorTableUpdaterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 42usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::GENERATOR_GLOBAL_TABLE_ROOT(_) => {
                    <GENERATOR_GLOBAL_TABLE_ROOTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::GENERATOR_MAX_STALENESS_PERIOD(_) => {
                    <GENERATOR_MAX_STALENESS_PERIODCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::GENERATOR_REFERENCE_TIMESTAMP(_) => {
                    <GENERATOR_REFERENCE_TIMESTAMPCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::GLOBAL_TABLE_ROOT_CERT_TYPEHASH(_) => {
                    <GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_BPS(_) => <MAX_BPSCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::OPERATOR_INFO_LEAF_SALT(_) => {
                    <OPERATOR_INFO_LEAF_SALTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::OPERATOR_TABLE_LEAF_SALT(_) => {
                    <OPERATOR_TABLE_LEAF_SALTCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::bn254CertificateVerifier(_) => {
                    <bn254CertificateVerifierCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateOperatorInfoLeaf(_) => {
                    <calculateOperatorInfoLeafCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateOperatorTableLeaf(_) => {
                    <calculateOperatorTableLeafCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::confirmGlobalTableRoot(_) => {
                    <confirmGlobalTableRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disableRoot(_) => <disableRootCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::ecdsaCertificateVerifier(_) => {
                    <ecdsaCertificateVerifierCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCertificateVerifier(_) => {
                    <getCertificateVerifierCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentGlobalTableRoot(_) => {
                    <getCurrentGlobalTableRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGenerator(_) => <getGeneratorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getGeneratorConfig(_) => {
                    <getGeneratorConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGeneratorReferenceTimestamp(_) => {
                    <getGeneratorReferenceTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGlobalTableRootByTimestamp(_) => {
                    <getGlobalTableRootByTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGlobalTableUpdateMessageHash(_) => {
                    <getGlobalTableUpdateMessageHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGlobalTableUpdateSignableDigest(_) => {
                    <getGlobalTableUpdateSignableDigestCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLatestReferenceBlockNumber(_) => {
                    <getLatestReferenceBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLatestReferenceTimestamp(_) => {
                    <getLatestReferenceTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getReferenceBlockNumberByTimestamp(_) => {
                    <getReferenceBlockNumberByTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getReferenceTimestampByBlockNumber(_) => {
                    <getReferenceTimestampByBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::globalRootConfirmationThreshold(_) => {
                    <globalRootConfirmationThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isRootValid(_) => <isRootValidCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isRootValidByTimestamp(_) => {
                    <isRootValidByTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setGlobalRootConfirmationThreshold(_) => {
                    <setGlobalRootConfirmationThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateGenerator(_) => {
                    <updateGeneratorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorTable(_) => {
                    <updateOperatorTableCall as alloy_sol_types::SolCall>::SELECTOR
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
                -> alloy_sol_types::Result<OperatorTableUpdaterCalls>] = &[
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::initialize)
                    }
                    initialize
                },
                {
                    fn OPERATOR_TABLE_LEAF_SALT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <OPERATOR_TABLE_LEAF_SALTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::OPERATOR_TABLE_LEAF_SALT)
                    }
                    OPERATOR_TABLE_LEAF_SALT
                },
                {
                    fn pause(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::pause)
                    }
                    pause
                },
                {
                    fn getReferenceTimestampByBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getReferenceTimestampByBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getReferenceTimestampByBlockNumber,
                            )
                    }
                    getReferenceTimestampByBlockNumber
                },
                {
                    fn GENERATOR_GLOBAL_TABLE_ROOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <GENERATOR_GLOBAL_TABLE_ROOTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::GENERATOR_GLOBAL_TABLE_ROOT)
                    }
                    GENERATOR_GLOBAL_TABLE_ROOT
                },
                {
                    fn getGenerator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGeneratorCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::getGenerator)
                    }
                    getGenerator
                },
                {
                    fn setGlobalRootConfirmationThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <setGlobalRootConfirmationThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::setGlobalRootConfirmationThreshold,
                            )
                    }
                    setGlobalRootConfirmationThreshold
                },
                {
                    fn getReferenceBlockNumberByTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getReferenceBlockNumberByTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getReferenceBlockNumberByTimestamp,
                            )
                    }
                    getReferenceBlockNumberByTimestamp
                },
                {
                    fn getCurrentGlobalTableRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getCurrentGlobalTableRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::getCurrentGlobalTableRoot)
                    }
                    getCurrentGlobalTableRoot
                },
                {
                    fn isRootValid(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <isRootValidCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::isRootValid)
                    }
                    isRootValid
                },
                {
                    fn getLatestReferenceBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getLatestReferenceBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getLatestReferenceBlockNumber,
                            )
                    }
                    getLatestReferenceBlockNumber
                },
                {
                    fn GLOBAL_TABLE_ROOT_CERT_TYPEHASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::GLOBAL_TABLE_ROOT_CERT_TYPEHASH,
                            )
                    }
                    GLOBAL_TABLE_ROOT_CERT_TYPEHASH
                },
                {
                    fn getGlobalTableUpdateSignableDigest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGlobalTableUpdateSignableDigestCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getGlobalTableUpdateSignableDigest,
                            )
                    }
                    getGlobalTableUpdateSignableDigest
                },
                {
                    fn getLatestReferenceTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getLatestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::getLatestReferenceTimestamp)
                    }
                    getLatestReferenceTimestamp
                },
                {
                    fn calculateOperatorInfoLeaf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <calculateOperatorInfoLeafCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::calculateOperatorInfoLeaf)
                    }
                    calculateOperatorInfoLeaf
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::version)
                    }
                    version
                },
                {
                    fn pauseAll(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn GENERATOR_REFERENCE_TIMESTAMP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <GENERATOR_REFERENCE_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::GENERATOR_REFERENCE_TIMESTAMP,
                            )
                    }
                    GENERATOR_REFERENCE_TIMESTAMP
                },
                {
                    fn isRootValidByTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <isRootValidByTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::isRootValidByTimestamp)
                    }
                    isRootValidByTimestamp
                },
                {
                    fn getCertificateVerifier(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getCertificateVerifierCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::getCertificateVerifier)
                    }
                    getCertificateVerifier
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn getGeneratorReferenceTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGeneratorReferenceTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getGeneratorReferenceTimestamp,
                            )
                    }
                    getGeneratorReferenceTimestamp
                },
                {
                    fn GENERATOR_MAX_STALENESS_PERIOD(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <GENERATOR_MAX_STALENESS_PERIODCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::GENERATOR_MAX_STALENESS_PERIOD,
                            )
                    }
                    GENERATOR_MAX_STALENESS_PERIOD
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::owner)
                    }
                    owner
                },
                {
                    fn updateOperatorTable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::updateOperatorTable)
                    }
                    updateOperatorTable
                },
                {
                    fn updateGenerator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <updateGeneratorCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::updateGenerator)
                    }
                    updateGenerator
                },
                {
                    fn OPERATOR_INFO_LEAF_SALT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <OPERATOR_INFO_LEAF_SALTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::OPERATOR_INFO_LEAF_SALT)
                    }
                    OPERATOR_INFO_LEAF_SALT
                },
                {
                    fn calculateOperatorTableLeaf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <calculateOperatorTableLeafCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::calculateOperatorTableLeaf)
                    }
                    calculateOperatorTableLeaf
                },
                {
                    fn ecdsaCertificateVerifier(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <ecdsaCertificateVerifierCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::ecdsaCertificateVerifier)
                    }
                    ecdsaCertificateVerifier
                },
                {
                    fn getGeneratorConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGeneratorConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::getGeneratorConfig)
                    }
                    getGeneratorConfig
                },
                {
                    fn bn254CertificateVerifier(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <bn254CertificateVerifierCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::bn254CertificateVerifier)
                    }
                    bn254CertificateVerifier
                },
                {
                    fn globalRootConfirmationThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <globalRootConfirmationThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::globalRootConfirmationThreshold,
                            )
                    }
                    globalRootConfirmationThreshold
                },
                {
                    fn disableRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <disableRootCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::disableRoot)
                    }
                    disableRoot
                },
                {
                    fn getGlobalTableUpdateMessageHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGlobalTableUpdateMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getGlobalTableUpdateMessageHash,
                            )
                    }
                    getGlobalTableUpdateMessageHash
                },
                {
                    fn getGlobalTableRootByTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGlobalTableRootByTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getGlobalTableRootByTimestamp,
                            )
                    }
                    getGlobalTableRootByTimestamp
                },
                {
                    fn confirmGlobalTableRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <confirmGlobalTableRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::confirmGlobalTableRoot)
                    }
                    confirmGlobalTableRoot
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn unpause(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::unpause)
                    }
                    unpause
                },
                {
                    fn MAX_BPS(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <MAX_BPSCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::MAX_BPS)
                    }
                    MAX_BPS
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
                OperatorTableUpdaterCalls,
            >] = &[
                {
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::initialize)
                    }
                    initialize
                },
                {
                    fn OPERATOR_TABLE_LEAF_SALT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <OPERATOR_TABLE_LEAF_SALTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::OPERATOR_TABLE_LEAF_SALT)
                    }
                    OPERATOR_TABLE_LEAF_SALT
                },
                {
                    fn pause(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::pause)
                    }
                    pause
                },
                {
                    fn getReferenceTimestampByBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getReferenceTimestampByBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getReferenceTimestampByBlockNumber,
                            )
                    }
                    getReferenceTimestampByBlockNumber
                },
                {
                    fn GENERATOR_GLOBAL_TABLE_ROOT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <GENERATOR_GLOBAL_TABLE_ROOTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::GENERATOR_GLOBAL_TABLE_ROOT)
                    }
                    GENERATOR_GLOBAL_TABLE_ROOT
                },
                {
                    fn getGenerator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGeneratorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::getGenerator)
                    }
                    getGenerator
                },
                {
                    fn setGlobalRootConfirmationThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <setGlobalRootConfirmationThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::setGlobalRootConfirmationThreshold,
                            )
                    }
                    setGlobalRootConfirmationThreshold
                },
                {
                    fn getReferenceBlockNumberByTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getReferenceBlockNumberByTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getReferenceBlockNumberByTimestamp,
                            )
                    }
                    getReferenceBlockNumberByTimestamp
                },
                {
                    fn getCurrentGlobalTableRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getCurrentGlobalTableRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::getCurrentGlobalTableRoot)
                    }
                    getCurrentGlobalTableRoot
                },
                {
                    fn isRootValid(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <isRootValidCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::isRootValid)
                    }
                    isRootValid
                },
                {
                    fn getLatestReferenceBlockNumber(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getLatestReferenceBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getLatestReferenceBlockNumber,
                            )
                    }
                    getLatestReferenceBlockNumber
                },
                {
                    fn GLOBAL_TABLE_ROOT_CERT_TYPEHASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::GLOBAL_TABLE_ROOT_CERT_TYPEHASH,
                            )
                    }
                    GLOBAL_TABLE_ROOT_CERT_TYPEHASH
                },
                {
                    fn getGlobalTableUpdateSignableDigest(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGlobalTableUpdateSignableDigestCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getGlobalTableUpdateSignableDigest,
                            )
                    }
                    getGlobalTableUpdateSignableDigest
                },
                {
                    fn getLatestReferenceTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getLatestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::getLatestReferenceTimestamp)
                    }
                    getLatestReferenceTimestamp
                },
                {
                    fn calculateOperatorInfoLeaf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <calculateOperatorInfoLeafCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::calculateOperatorInfoLeaf)
                    }
                    calculateOperatorInfoLeaf
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::version)
                    }
                    version
                },
                {
                    fn pauseAll(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn GENERATOR_REFERENCE_TIMESTAMP(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <GENERATOR_REFERENCE_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::GENERATOR_REFERENCE_TIMESTAMP,
                            )
                    }
                    GENERATOR_REFERENCE_TIMESTAMP
                },
                {
                    fn isRootValidByTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <isRootValidByTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::isRootValidByTimestamp)
                    }
                    isRootValidByTimestamp
                },
                {
                    fn getCertificateVerifier(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getCertificateVerifierCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::getCertificateVerifier)
                    }
                    getCertificateVerifier
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn getGeneratorReferenceTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGeneratorReferenceTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getGeneratorReferenceTimestamp,
                            )
                    }
                    getGeneratorReferenceTimestamp
                },
                {
                    fn GENERATOR_MAX_STALENESS_PERIOD(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <GENERATOR_MAX_STALENESS_PERIODCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::GENERATOR_MAX_STALENESS_PERIOD,
                            )
                    }
                    GENERATOR_MAX_STALENESS_PERIOD
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::owner)
                    }
                    owner
                },
                {
                    fn updateOperatorTable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::updateOperatorTable)
                    }
                    updateOperatorTable
                },
                {
                    fn updateGenerator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <updateGeneratorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::updateGenerator)
                    }
                    updateGenerator
                },
                {
                    fn OPERATOR_INFO_LEAF_SALT(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <OPERATOR_INFO_LEAF_SALTCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::OPERATOR_INFO_LEAF_SALT)
                    }
                    OPERATOR_INFO_LEAF_SALT
                },
                {
                    fn calculateOperatorTableLeaf(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <calculateOperatorTableLeafCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::calculateOperatorTableLeaf)
                    }
                    calculateOperatorTableLeaf
                },
                {
                    fn ecdsaCertificateVerifier(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <ecdsaCertificateVerifierCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::ecdsaCertificateVerifier)
                    }
                    ecdsaCertificateVerifier
                },
                {
                    fn getGeneratorConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGeneratorConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::getGeneratorConfig)
                    }
                    getGeneratorConfig
                },
                {
                    fn bn254CertificateVerifier(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <bn254CertificateVerifierCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::bn254CertificateVerifier)
                    }
                    bn254CertificateVerifier
                },
                {
                    fn globalRootConfirmationThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <globalRootConfirmationThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::globalRootConfirmationThreshold,
                            )
                    }
                    globalRootConfirmationThreshold
                },
                {
                    fn disableRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <disableRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::disableRoot)
                    }
                    disableRoot
                },
                {
                    fn getGlobalTableUpdateMessageHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGlobalTableUpdateMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getGlobalTableUpdateMessageHash,
                            )
                    }
                    getGlobalTableUpdateMessageHash
                },
                {
                    fn getGlobalTableRootByTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGlobalTableRootByTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getGlobalTableRootByTimestamp,
                            )
                    }
                    getGlobalTableRootByTimestamp
                },
                {
                    fn confirmGlobalTableRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <confirmGlobalTableRootCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::confirmGlobalTableRoot)
                    }
                    confirmGlobalTableRoot
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn unpause(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::unpause)
                    }
                    unpause
                },
                {
                    fn MAX_BPS(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <MAX_BPSCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::MAX_BPS)
                    }
                    MAX_BPS
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
                Self::GENERATOR_GLOBAL_TABLE_ROOT(inner) => {
                    <GENERATOR_GLOBAL_TABLE_ROOTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GENERATOR_MAX_STALENESS_PERIOD(inner) => {
                    <GENERATOR_MAX_STALENESS_PERIODCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GENERATOR_REFERENCE_TIMESTAMP(inner) => {
                    <GENERATOR_REFERENCE_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GLOBAL_TABLE_ROOT_CERT_TYPEHASH(inner) => {
                    <GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_BPS(inner) => {
                    <MAX_BPSCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::OPERATOR_INFO_LEAF_SALT(inner) => {
                    <OPERATOR_INFO_LEAF_SALTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OPERATOR_TABLE_LEAF_SALT(inner) => {
                    <OPERATOR_TABLE_LEAF_SALTCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::bn254CertificateVerifier(inner) => {
                    <bn254CertificateVerifierCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateOperatorInfoLeaf(inner) => {
                    <calculateOperatorInfoLeafCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateOperatorTableLeaf(inner) => {
                    <calculateOperatorTableLeafCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::confirmGlobalTableRoot(inner) => {
                    <confirmGlobalTableRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disableRoot(inner) => {
                    <disableRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ecdsaCertificateVerifier(inner) => {
                    <ecdsaCertificateVerifierCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCertificateVerifier(inner) => {
                    <getCertificateVerifierCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentGlobalTableRoot(inner) => {
                    <getCurrentGlobalTableRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getGenerator(inner) => {
                    <getGeneratorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getGeneratorConfig(inner) => {
                    <getGeneratorConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getGeneratorReferenceTimestamp(inner) => {
                    <getGeneratorReferenceTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getGlobalTableRootByTimestamp(inner) => {
                    <getGlobalTableRootByTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getGlobalTableUpdateMessageHash(inner) => {
                    <getGlobalTableUpdateMessageHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getGlobalTableUpdateSignableDigest(inner) => {
                    <getGlobalTableUpdateSignableDigestCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestReferenceBlockNumber(inner) => {
                    <getLatestReferenceBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestReferenceTimestamp(inner) => {
                    <getLatestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getReferenceBlockNumberByTimestamp(inner) => {
                    <getReferenceBlockNumberByTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getReferenceTimestampByBlockNumber(inner) => {
                    <getReferenceTimestampByBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::globalRootConfirmationThreshold(inner) => {
                    <globalRootConfirmationThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isRootValid(inner) => {
                    <isRootValidCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isRootValidByTimestamp(inner) => {
                    <isRootValidByTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setGlobalRootConfirmationThreshold(inner) => {
                    <setGlobalRootConfirmationThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateGenerator(inner) => {
                    <updateGeneratorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorTable(inner) => {
                    <updateOperatorTableCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::GENERATOR_GLOBAL_TABLE_ROOT(inner) => {
                    <GENERATOR_GLOBAL_TABLE_ROOTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GENERATOR_MAX_STALENESS_PERIOD(inner) => {
                    <GENERATOR_MAX_STALENESS_PERIODCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GENERATOR_REFERENCE_TIMESTAMP(inner) => {
                    <GENERATOR_REFERENCE_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GLOBAL_TABLE_ROOT_CERT_TYPEHASH(inner) => {
                    <GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_BPS(inner) => {
                    <MAX_BPSCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::OPERATOR_INFO_LEAF_SALT(inner) => {
                    <OPERATOR_INFO_LEAF_SALTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OPERATOR_TABLE_LEAF_SALT(inner) => {
                    <OPERATOR_TABLE_LEAF_SALTCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::bn254CertificateVerifier(inner) => {
                    <bn254CertificateVerifierCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateOperatorInfoLeaf(inner) => {
                    <calculateOperatorInfoLeafCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateOperatorTableLeaf(inner) => {
                    <calculateOperatorTableLeafCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::confirmGlobalTableRoot(inner) => {
                    <confirmGlobalTableRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disableRoot(inner) => {
                    <disableRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ecdsaCertificateVerifier(inner) => {
                    <ecdsaCertificateVerifierCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCertificateVerifier(inner) => {
                    <getCertificateVerifierCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentGlobalTableRoot(inner) => {
                    <getCurrentGlobalTableRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getGenerator(inner) => {
                    <getGeneratorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getGeneratorConfig(inner) => {
                    <getGeneratorConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getGeneratorReferenceTimestamp(inner) => {
                    <getGeneratorReferenceTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getGlobalTableRootByTimestamp(inner) => {
                    <getGlobalTableRootByTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getGlobalTableUpdateMessageHash(inner) => {
                    <getGlobalTableUpdateMessageHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getGlobalTableUpdateSignableDigest(inner) => {
                    <getGlobalTableUpdateSignableDigestCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLatestReferenceBlockNumber(inner) => {
                    <getLatestReferenceBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLatestReferenceTimestamp(inner) => {
                    <getLatestReferenceTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getReferenceBlockNumberByTimestamp(inner) => {
                    <getReferenceBlockNumberByTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getReferenceTimestampByBlockNumber(inner) => {
                    <getReferenceTimestampByBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::globalRootConfirmationThreshold(inner) => {
                    <globalRootConfirmationThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isRootValid(inner) => {
                    <isRootValidCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isRootValidByTimestamp(inner) => {
                    <isRootValidByTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setGlobalRootConfirmationThreshold(inner) => {
                    <setGlobalRootConfirmationThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateGenerator(inner) => {
                    <updateGeneratorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`OperatorTableUpdater`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum OperatorTableUpdaterErrors {
        #[allow(missing_docs)]
        CannotDisableGeneratorRoot(CannotDisableGeneratorRoot),
        #[allow(missing_docs)]
        CertificateInvalid(CertificateInvalid),
        #[allow(missing_docs)]
        CurrentlyPaused(CurrentlyPaused),
        #[allow(missing_docs)]
        EmptyRoot(EmptyRoot),
        #[allow(missing_docs)]
        GlobalTableRootInFuture(GlobalTableRootInFuture),
        #[allow(missing_docs)]
        GlobalTableRootStale(GlobalTableRootStale),
        #[allow(missing_docs)]
        InputAddressZero(InputAddressZero),
        #[allow(missing_docs)]
        InvalidConfirmationThreshold(InvalidConfirmationThreshold),
        #[allow(missing_docs)]
        InvalidCurveType(InvalidCurveType),
        #[allow(missing_docs)]
        InvalidGenerator(InvalidGenerator),
        #[allow(missing_docs)]
        InvalidGlobalTableRoot(InvalidGlobalTableRoot),
        #[allow(missing_docs)]
        InvalidIndex(InvalidIndex),
        #[allow(missing_docs)]
        InvalidMessageHash(InvalidMessageHash),
        #[allow(missing_docs)]
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        #[allow(missing_docs)]
        InvalidOperatorSet(InvalidOperatorSet),
        #[allow(missing_docs)]
        InvalidOperatorSetProof(InvalidOperatorSetProof),
        #[allow(missing_docs)]
        InvalidProofLength(InvalidProofLength),
        #[allow(missing_docs)]
        InvalidRoot(InvalidRoot),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        OnlyPauser(OnlyPauser),
        #[allow(missing_docs)]
        OnlyUnpauser(OnlyUnpauser),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
        #[allow(missing_docs)]
        TableUpdateForPastTimestamp(TableUpdateForPastTimestamp),
    }
    #[automatically_derived]
    impl OperatorTableUpdaterErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [14u8, 102u8, 222u8, 6u8],
            [27u8, 253u8, 67u8, 88u8],
            [32u8, 118u8, 23u8, 223u8],
            [48u8, 90u8, 39u8, 169u8],
            [51u8, 36u8, 21u8, 250u8],
            [77u8, 197u8, 246u8, 164u8],
            [80u8, 69u8, 112u8, 227u8],
            [83u8, 206u8, 78u8, 206u8],
            [99u8, 223u8, 129u8, 113u8],
            [100u8, 70u8, 249u8, 23u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [126u8, 197u8, 193u8, 84u8],
            [132u8, 10u8, 72u8, 213u8],
            [139u8, 86u8, 100u8, 45u8],
            [175u8, 164u8, 44u8, 167u8],
            [179u8, 81u8, 43u8, 12u8],
            [180u8, 35u8, 59u8, 106u8],
            [193u8, 8u8, 16u8, 124u8],
            [198u8, 29u8, 202u8, 93u8],
            [199u8, 58u8, 19u8, 106u8],
            [253u8, 234u8, 124u8, 9u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OperatorTableUpdaterErrors {
        const NAME: &'static str = "OperatorTableUpdaterErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 23usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CannotDisableGeneratorRoot(_) => {
                    <CannotDisableGeneratorRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CertificateInvalid(_) => {
                    <CertificateInvalid as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyRoot(_) => <EmptyRoot as alloy_sol_types::SolError>::SELECTOR,
                Self::GlobalTableRootInFuture(_) => {
                    <GlobalTableRootInFuture as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GlobalTableRootStale(_) => {
                    <GlobalTableRootStale as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidConfirmationThreshold(_) => {
                    <InvalidConfirmationThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCurveType(_) => {
                    <InvalidCurveType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidGenerator(_) => {
                    <InvalidGenerator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidGlobalTableRoot(_) => {
                    <InvalidGlobalTableRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidIndex(_) => <InvalidIndex as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidMessageHash(_) => {
                    <InvalidMessageHash as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOperatorSet(_) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOperatorSetProof(_) => {
                    <InvalidOperatorSetProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidProofLength(_) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRoot(_) => <InvalidRoot as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
                Self::StringTooLong(_) => <StringTooLong as alloy_sol_types::SolError>::SELECTOR,
                Self::TableUpdateForPastTimestamp(_) => {
                    <TableUpdateForPastTimestamp as alloy_sol_types::SolError>::SELECTOR
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
            )
                -> alloy_sol_types::Result<OperatorTableUpdaterErrors>] = &[
                {
                    fn InvalidConfirmationThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidConfirmationThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterErrors::InvalidConfirmationThreshold)
                    }
                    InvalidConfirmationThreshold
                },
                {
                    fn GlobalTableRootStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <GlobalTableRootStale as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::GlobalTableRootStale)
                    }
                    GlobalTableRootStale
                },
                {
                    fn TableUpdateForPastTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <TableUpdateForPastTimestamp as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterErrors::TableUpdateForPastTimestamp)
                    }
                    TableUpdateForPastTimestamp
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn CannotDisableGeneratorRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <CannotDisableGeneratorRoot as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterErrors::CannotDisableGeneratorRoot)
                    }
                    CannotDisableGeneratorRoot
                },
                {
                    fn InvalidProofLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidProofLength as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InvalidProofLength)
                    }
                    InvalidProofLength
                },
                {
                    fn InvalidRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidRoot as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InvalidRoot)
                    }
                    InvalidRoot
                },
                {
                    fn EmptyRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <EmptyRoot as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::EmptyRoot)
                    }
                    EmptyRoot
                },
                {
                    fn InvalidIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidIndex as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InvalidIndex)
                    }
                    InvalidIndex
                },
                {
                    fn InvalidGenerator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidGenerator as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InvalidGenerator)
                    }
                    InvalidGenerator
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn InvalidOperatorSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidOperatorSet as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InvalidOperatorSet)
                    }
                    InvalidOperatorSet
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidMessageHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidMessageHash as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InvalidMessageHash)
                    }
                    InvalidMessageHash
                },
                {
                    fn InvalidOperatorSetProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidOperatorSetProof as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InvalidOperatorSetProof)
                    }
                    InvalidOperatorSetProof
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn GlobalTableRootInFuture(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <GlobalTableRootInFuture as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::GlobalTableRootInFuture)
                    }
                    GlobalTableRootInFuture
                },
                {
                    fn CertificateInvalid(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <CertificateInvalid as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::CertificateInvalid)
                    }
                    CertificateInvalid
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn InvalidGlobalTableRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidGlobalTableRoot as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InvalidGlobalTableRoot)
                    }
                    InvalidGlobalTableRoot
                },
                {
                    fn InvalidCurveType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidCurveType as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InvalidCurveType)
                    }
                    InvalidCurveType
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
                OperatorTableUpdaterErrors,
            >] = &[
                {
                    fn InvalidConfirmationThreshold(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidConfirmationThreshold as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterErrors::InvalidConfirmationThreshold,
                            )
                    }
                    InvalidConfirmationThreshold
                },
                {
                    fn GlobalTableRootStale(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <GlobalTableRootStale as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterErrors::GlobalTableRootStale)
                    }
                    GlobalTableRootStale
                },
                {
                    fn TableUpdateForPastTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <TableUpdateForPastTimestamp as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterErrors::TableUpdateForPastTimestamp)
                    }
                    TableUpdateForPastTimestamp
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn CannotDisableGeneratorRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <CannotDisableGeneratorRoot as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterErrors::CannotDisableGeneratorRoot)
                    }
                    CannotDisableGeneratorRoot
                },
                {
                    fn InvalidProofLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidProofLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(OperatorTableUpdaterErrors::InvalidProofLength)
                    }
                    InvalidProofLength
                },
                {
                    fn InvalidRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidRoot as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterErrors::InvalidRoot)
                    }
                    InvalidRoot
                },
                {
                    fn EmptyRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <EmptyRoot as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterErrors::EmptyRoot)
                    }
                    EmptyRoot
                },
                {
                    fn InvalidIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterErrors::InvalidIndex)
                    }
                    InvalidIndex
                },
                {
                    fn InvalidGenerator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidGenerator as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(OperatorTableUpdaterErrors::InvalidGenerator)
                    }
                    InvalidGenerator
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(OperatorTableUpdaterErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn InvalidOperatorSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidOperatorSet as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(OperatorTableUpdaterErrors::InvalidOperatorSet)
                    }
                    InvalidOperatorSet
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(OperatorTableUpdaterErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidMessageHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidMessageHash as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(OperatorTableUpdaterErrors::InvalidMessageHash)
                    }
                    InvalidMessageHash
                },
                {
                    fn InvalidOperatorSetProof(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidOperatorSetProof as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterErrors::InvalidOperatorSetProof)
                    }
                    InvalidOperatorSetProof
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(OperatorTableUpdaterErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn GlobalTableRootInFuture(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <GlobalTableRootInFuture as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterErrors::GlobalTableRootInFuture)
                    }
                    GlobalTableRootInFuture
                },
                {
                    fn CertificateInvalid(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <CertificateInvalid as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(OperatorTableUpdaterErrors::CertificateInvalid)
                    }
                    CertificateInvalid
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn InvalidGlobalTableRoot(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidGlobalTableRoot as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterErrors::InvalidGlobalTableRoot)
                    }
                    InvalidGlobalTableRoot
                },
                {
                    fn InvalidCurveType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidCurveType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(OperatorTableUpdaterErrors::InvalidCurveType)
                    }
                    InvalidCurveType
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
                Self::CannotDisableGeneratorRoot(inner) => {
                    <CannotDisableGeneratorRoot as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CertificateInvalid(inner) => {
                    <CertificateInvalid as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::EmptyRoot(inner) => {
                    <EmptyRoot as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::GlobalTableRootInFuture(inner) => {
                    <GlobalTableRootInFuture as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::GlobalTableRootStale(inner) => {
                    <GlobalTableRootStale as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidConfirmationThreshold(inner) => {
                    <InvalidConfirmationThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidCurveType(inner) => {
                    <InvalidCurveType as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidGenerator(inner) => {
                    <InvalidGenerator as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidGlobalTableRoot(inner) => {
                    <InvalidGlobalTableRoot as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidIndex(inner) => {
                    <InvalidIndex as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidMessageHash(inner) => {
                    <InvalidMessageHash as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidOperatorSet(inner) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidOperatorSetProof(inner) => {
                    <InvalidOperatorSetProof as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidProofLength(inner) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidRoot(inner) => {
                    <InvalidRoot as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::TableUpdateForPastTimestamp(inner) => {
                    <TableUpdateForPastTimestamp as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CannotDisableGeneratorRoot(inner) => {
                    <CannotDisableGeneratorRoot as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::CertificateInvalid(inner) => {
                    <CertificateInvalid as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::EmptyRoot(inner) => {
                    <EmptyRoot as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::GlobalTableRootInFuture(inner) => {
                    <GlobalTableRootInFuture as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::GlobalTableRootStale(inner) => {
                    <GlobalTableRootStale as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidConfirmationThreshold(inner) => {
                    <InvalidConfirmationThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidCurveType(inner) => {
                    <InvalidCurveType as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidGenerator(inner) => {
                    <InvalidGenerator as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidGlobalTableRoot(inner) => {
                    <InvalidGlobalTableRoot as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidIndex(inner) => {
                    <InvalidIndex as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidMessageHash(inner) => {
                    <InvalidMessageHash as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidOperatorSet(inner) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidOperatorSetProof(inner) => {
                    <InvalidOperatorSetProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidProofLength(inner) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidRoot(inner) => {
                    <InvalidRoot as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::TableUpdateForPastTimestamp(inner) => {
                    <TableUpdateForPastTimestamp as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
            }
        }
    }
    ///Container for all the [`OperatorTableUpdater`](self) events.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum OperatorTableUpdaterEvents {
        #[allow(missing_docs)]
        GeneratorUpdated(GeneratorUpdated),
        #[allow(missing_docs)]
        GlobalRootConfirmationThresholdUpdated(GlobalRootConfirmationThresholdUpdated),
        #[allow(missing_docs)]
        GlobalRootDisabled(GlobalRootDisabled),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        NewGlobalTableRoot(NewGlobalTableRoot),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl OperatorTableUpdaterEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                1u8, 13u8, 203u8, 224u8, 209u8, 224u8, 25u8, 201u8, 51u8, 87u8, 113u8, 31u8, 123u8,
                182u8, 40u8, 125u8, 84u8, 59u8, 127u8, 247u8, 222u8, 116u8, 242u8, 157u8, 243u8,
                251u8, 94u8, 204u8, 238u8, 200u8, 211u8, 105u8,
            ],
            [
                52u8, 99u8, 67u8, 27u8, 9u8, 223u8, 212u8, 61u8, 236u8, 115u8, 73u8, 248u8, 242u8,
                74u8, 207u8, 167u8, 83u8, 254u8, 76u8, 244u8, 10u8, 38u8, 35u8, 84u8, 2u8, 210u8,
                19u8, 55u8, 61u8, 241u8, 88u8, 86u8,
            ],
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                139u8, 212u8, 61u8, 225u8, 37u8, 15u8, 88u8, 254u8, 110u8, 201u8, 167u8, 134u8,
                113u8, 168u8, 183u8, 141u8, 186u8, 112u8, 240u8, 1u8, 134u8, 86u8, 209u8, 87u8,
                163u8, 174u8, 170u8, 190u8, 195u8, 137u8, 223u8, 52u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ],
            [
                245u8, 209u8, 131u8, 109u8, 248u8, 252u8, 215u8, 193u8, 229u8, 64u8, 71u8, 233u8,
                74u8, 200u8, 119u8, 61u8, 40u8, 85u8, 57u8, 86u8, 3u8, 226u8, 239u8, 155u8, 165u8,
                245u8, 241u8, 105u8, 5u8, 242u8, 37u8, 146u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for OperatorTableUpdaterEvents {
        const NAME: &'static str = "OperatorTableUpdaterEvents";
        const COUNT: usize = 8usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<GeneratorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <GeneratorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::GeneratorUpdated)
                }
                Some(
                    <GlobalRootConfirmationThresholdUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <GlobalRootConfirmationThresholdUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::GlobalRootConfirmationThresholdUpdated)
                }
                Some(
                    <GlobalRootDisabled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <GlobalRootDisabled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::GlobalRootDisabled)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <NewGlobalTableRoot as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <NewGlobalTableRoot as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::NewGlobalTableRoot)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Paused)
                }
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Unpaused)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for OperatorTableUpdaterEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::GeneratorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::GlobalRootConfirmationThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::GlobalRootDisabled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewGlobalTableRoot(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::GeneratorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::GlobalRootConfirmationThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::GlobalRootDisabled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewGlobalTableRoot(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`OperatorTableUpdater`](self) contract instance.

    See the [wrapper's documentation](`OperatorTableUpdaterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> OperatorTableUpdaterInstance<P, N> {
        OperatorTableUpdaterInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
        provider: P,
        _bn254CertificateVerifier: alloy::sol_types::private::Address,
        _ecdsaCertificateVerifier: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _version: alloy::sol_types::private::String,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<OperatorTableUpdaterInstance<P, N>>>
    {
        OperatorTableUpdaterInstance::<P, N>::deploy(
            provider,
            _bn254CertificateVerifier,
            _ecdsaCertificateVerifier,
            _pauserRegistry,
            _version,
        )
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
        _bn254CertificateVerifier: alloy::sol_types::private::Address,
        _ecdsaCertificateVerifier: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _version: alloy::sol_types::private::String,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        OperatorTableUpdaterInstance::<P, N>::deploy_builder(
            provider,
            _bn254CertificateVerifier,
            _ecdsaCertificateVerifier,
            _pauserRegistry,
            _version,
        )
    }
    /**A [`OperatorTableUpdater`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`OperatorTableUpdater`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OperatorTableUpdaterInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for OperatorTableUpdaterInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OperatorTableUpdaterInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        OperatorTableUpdaterInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`OperatorTableUpdater`](self) contract instance.

        See the [wrapper's documentation](`OperatorTableUpdaterInstance`) for more details.*/
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
            _bn254CertificateVerifier: alloy::sol_types::private::Address,
            _ecdsaCertificateVerifier: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::Result<OperatorTableUpdaterInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _bn254CertificateVerifier,
                _ecdsaCertificateVerifier,
                _pauserRegistry,
                _version,
            );
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
            _bn254CertificateVerifier: alloy::sol_types::private::Address,
            _ecdsaCertificateVerifier: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _bn254CertificateVerifier,
                        _ecdsaCertificateVerifier,
                        _pauserRegistry,
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
    impl<P: ::core::clone::Clone, N> OperatorTableUpdaterInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OperatorTableUpdaterInstance<P, N> {
            OperatorTableUpdaterInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        OperatorTableUpdaterInstance<P, N>
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
        ///Creates a new call builder for the [`GENERATOR_GLOBAL_TABLE_ROOT`] function.
        pub fn GENERATOR_GLOBAL_TABLE_ROOT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, GENERATOR_GLOBAL_TABLE_ROOTCall, N> {
            self.call_builder(&GENERATOR_GLOBAL_TABLE_ROOTCall)
        }
        ///Creates a new call builder for the [`GENERATOR_MAX_STALENESS_PERIOD`] function.
        pub fn GENERATOR_MAX_STALENESS_PERIOD(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, GENERATOR_MAX_STALENESS_PERIODCall, N> {
            self.call_builder(&GENERATOR_MAX_STALENESS_PERIODCall)
        }
        ///Creates a new call builder for the [`GENERATOR_REFERENCE_TIMESTAMP`] function.
        pub fn GENERATOR_REFERENCE_TIMESTAMP(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, GENERATOR_REFERENCE_TIMESTAMPCall, N> {
            self.call_builder(&GENERATOR_REFERENCE_TIMESTAMPCall)
        }
        ///Creates a new call builder for the [`GLOBAL_TABLE_ROOT_CERT_TYPEHASH`] function.
        pub fn GLOBAL_TABLE_ROOT_CERT_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall, N> {
            self.call_builder(&GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall)
        }
        ///Creates a new call builder for the [`MAX_BPS`] function.
        pub fn MAX_BPS(&self) -> alloy_contract::SolCallBuilder<&P, MAX_BPSCall, N> {
            self.call_builder(&MAX_BPSCall)
        }
        ///Creates a new call builder for the [`OPERATOR_INFO_LEAF_SALT`] function.
        pub fn OPERATOR_INFO_LEAF_SALT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, OPERATOR_INFO_LEAF_SALTCall, N> {
            self.call_builder(&OPERATOR_INFO_LEAF_SALTCall)
        }
        ///Creates a new call builder for the [`OPERATOR_TABLE_LEAF_SALT`] function.
        pub fn OPERATOR_TABLE_LEAF_SALT(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, OPERATOR_TABLE_LEAF_SALTCall, N> {
            self.call_builder(&OPERATOR_TABLE_LEAF_SALTCall)
        }
        ///Creates a new call builder for the [`bn254CertificateVerifier`] function.
        pub fn bn254CertificateVerifier(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, bn254CertificateVerifierCall, N> {
            self.call_builder(&bn254CertificateVerifierCall)
        }
        ///Creates a new call builder for the [`calculateOperatorInfoLeaf`] function.
        pub fn calculateOperatorInfoLeaf(
            &self,
            operatorInfo: <IOperatorTableCalculatorTypes::BN254OperatorInfo as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, calculateOperatorInfoLeafCall, N> {
            self.call_builder(&calculateOperatorInfoLeafCall { operatorInfo })
        }
        ///Creates a new call builder for the [`calculateOperatorTableLeaf`] function.
        pub fn calculateOperatorTableLeaf(
            &self,
            operatorTableBytes: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, calculateOperatorTableLeafCall, N> {
            self.call_builder(&calculateOperatorTableLeafCall { operatorTableBytes })
        }
        ///Creates a new call builder for the [`confirmGlobalTableRoot`] function.
        pub fn confirmGlobalTableRoot(
            &self,
            globalTableRootCert: <IBN254CertificateVerifierTypes::BN254Certificate as alloy::sol_types::SolType>::RustType,
            globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
            referenceTimestamp: u32,
            referenceBlockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<&P, confirmGlobalTableRootCall, N> {
            self.call_builder(&confirmGlobalTableRootCall {
                globalTableRootCert,
                globalTableRoot,
                referenceTimestamp,
                referenceBlockNumber,
            })
        }
        ///Creates a new call builder for the [`disableRoot`] function.
        pub fn disableRoot(
            &self,
            globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, disableRootCall, N> {
            self.call_builder(&disableRootCall { globalTableRoot })
        }
        ///Creates a new call builder for the [`ecdsaCertificateVerifier`] function.
        pub fn ecdsaCertificateVerifier(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, ecdsaCertificateVerifierCall, N> {
            self.call_builder(&ecdsaCertificateVerifierCall)
        }
        ///Creates a new call builder for the [`getCertificateVerifier`] function.
        pub fn getCertificateVerifier(
            &self,
            curveType: <IKeyRegistrarTypes::CurveType as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, getCertificateVerifierCall, N> {
            self.call_builder(&getCertificateVerifierCall { curveType })
        }
        ///Creates a new call builder for the [`getCurrentGlobalTableRoot`] function.
        pub fn getCurrentGlobalTableRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getCurrentGlobalTableRootCall, N> {
            self.call_builder(&getCurrentGlobalTableRootCall)
        }
        ///Creates a new call builder for the [`getGenerator`] function.
        pub fn getGenerator(&self) -> alloy_contract::SolCallBuilder<&P, getGeneratorCall, N> {
            self.call_builder(&getGeneratorCall)
        }
        ///Creates a new call builder for the [`getGeneratorConfig`] function.
        pub fn getGeneratorConfig(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getGeneratorConfigCall, N> {
            self.call_builder(&getGeneratorConfigCall)
        }
        ///Creates a new call builder for the [`getGeneratorReferenceTimestamp`] function.
        pub fn getGeneratorReferenceTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getGeneratorReferenceTimestampCall, N> {
            self.call_builder(&getGeneratorReferenceTimestampCall)
        }
        ///Creates a new call builder for the [`getGlobalTableRootByTimestamp`] function.
        pub fn getGlobalTableRootByTimestamp(
            &self,
            referenceTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getGlobalTableRootByTimestampCall, N> {
            self.call_builder(&getGlobalTableRootByTimestampCall { referenceTimestamp })
        }
        ///Creates a new call builder for the [`getGlobalTableUpdateMessageHash`] function.
        pub fn getGlobalTableUpdateMessageHash(
            &self,
            globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
            referenceTimestamp: u32,
            referenceBlockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getGlobalTableUpdateMessageHashCall, N> {
            self.call_builder(&getGlobalTableUpdateMessageHashCall {
                globalTableRoot,
                referenceTimestamp,
                referenceBlockNumber,
            })
        }
        ///Creates a new call builder for the [`getGlobalTableUpdateSignableDigest`] function.
        pub fn getGlobalTableUpdateSignableDigest(
            &self,
            globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
            referenceTimestamp: u32,
            referenceBlockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getGlobalTableUpdateSignableDigestCall, N> {
            self.call_builder(&getGlobalTableUpdateSignableDigestCall {
                globalTableRoot,
                referenceTimestamp,
                referenceBlockNumber,
            })
        }
        ///Creates a new call builder for the [`getLatestReferenceBlockNumber`] function.
        pub fn getLatestReferenceBlockNumber(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getLatestReferenceBlockNumberCall, N> {
            self.call_builder(&getLatestReferenceBlockNumberCall)
        }
        ///Creates a new call builder for the [`getLatestReferenceTimestamp`] function.
        pub fn getLatestReferenceTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getLatestReferenceTimestampCall, N> {
            self.call_builder(&getLatestReferenceTimestampCall)
        }
        ///Creates a new call builder for the [`getReferenceBlockNumberByTimestamp`] function.
        pub fn getReferenceBlockNumberByTimestamp(
            &self,
            referenceTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getReferenceBlockNumberByTimestampCall, N> {
            self.call_builder(&getReferenceBlockNumberByTimestampCall { referenceTimestamp })
        }
        ///Creates a new call builder for the [`getReferenceTimestampByBlockNumber`] function.
        pub fn getReferenceTimestampByBlockNumber(
            &self,
            referenceBlockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<&P, getReferenceTimestampByBlockNumberCall, N> {
            self.call_builder(&getReferenceTimestampByBlockNumberCall {
                referenceBlockNumber,
            })
        }
        ///Creates a new call builder for the [`globalRootConfirmationThreshold`] function.
        pub fn globalRootConfirmationThreshold(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, globalRootConfirmationThresholdCall, N> {
            self.call_builder(&globalRootConfirmationThresholdCall)
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _owner: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
            _initialGenerator: <OperatorSet as alloy::sol_types::SolType>::RustType,
            _globalRootConfirmationThreshold: u16,
            generatorInfo: <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall {
                _owner,
                initialPausedStatus,
                _initialGenerator,
                _globalRootConfirmationThreshold,
                generatorInfo,
            })
        }
        ///Creates a new call builder for the [`isRootValid`] function.
        pub fn isRootValid(
            &self,
            globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, isRootValidCall, N> {
            self.call_builder(&isRootValidCall { globalTableRoot })
        }
        ///Creates a new call builder for the [`isRootValidByTimestamp`] function.
        pub fn isRootValidByTimestamp(
            &self,
            referenceTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<&P, isRootValidByTimestampCall, N> {
            self.call_builder(&isRootValidByTimestampCall { referenceTimestamp })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<&P, ownerCall, N> {
            self.call_builder(&ownerCall)
        }
        ///Creates a new call builder for the [`pause`] function.
        pub fn pause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, pauseCall, N> {
            self.call_builder(&pauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`pauseAll`] function.
        pub fn pauseAll(&self) -> alloy_contract::SolCallBuilder<&P, pauseAllCall, N> {
            self.call_builder(&pauseAllCall)
        }
        ///Creates a new call builder for the [`paused_0`] function.
        pub fn paused_0(&self, index: u8) -> alloy_contract::SolCallBuilder<&P, paused_0Call, N> {
            self.call_builder(&paused_0Call { index })
        }
        ///Creates a new call builder for the [`paused_1`] function.
        pub fn paused_1(&self) -> alloy_contract::SolCallBuilder<&P, paused_1Call, N> {
            self.call_builder(&paused_1Call)
        }
        ///Creates a new call builder for the [`pauserRegistry`] function.
        pub fn pauserRegistry(&self) -> alloy_contract::SolCallBuilder<&P, pauserRegistryCall, N> {
            self.call_builder(&pauserRegistryCall)
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
        }
        ///Creates a new call builder for the [`setGlobalRootConfirmationThreshold`] function.
        pub fn setGlobalRootConfirmationThreshold(
            &self,
            bps: u16,
        ) -> alloy_contract::SolCallBuilder<&P, setGlobalRootConfirmationThresholdCall, N> {
            self.call_builder(&setGlobalRootConfirmationThresholdCall { bps })
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
        ///Creates a new call builder for the [`updateGenerator`] function.
        pub fn updateGenerator(
            &self,
            generator: <OperatorSet as alloy::sol_types::SolType>::RustType,
            generatorInfo: <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, updateGeneratorCall, N> {
            self.call_builder(&updateGeneratorCall {
                generator,
                generatorInfo,
            })
        }
        ///Creates a new call builder for the [`updateOperatorTable`] function.
        pub fn updateOperatorTable(
            &self,
            referenceTimestamp: u32,
            globalTableRoot: alloy::sol_types::private::FixedBytes<32>,
            operatorSetIndex: u32,
            proof: alloy::sol_types::private::Bytes,
            operatorTableBytes: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, updateOperatorTableCall, N> {
            self.call_builder(&updateOperatorTableCall {
                referenceTimestamp,
                globalTableRoot,
                operatorSetIndex,
                proof,
                operatorTableBytes,
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
        OperatorTableUpdaterInstance<P, N>
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
        ///Creates a new event filter for the [`GeneratorUpdated`] event.
        pub fn GeneratorUpdated_filter(&self) -> alloy_contract::Event<&P, GeneratorUpdated, N> {
            self.event_filter::<GeneratorUpdated>()
        }
        ///Creates a new event filter for the [`GlobalRootConfirmationThresholdUpdated`] event.
        pub fn GlobalRootConfirmationThresholdUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, GlobalRootConfirmationThresholdUpdated, N> {
            self.event_filter::<GlobalRootConfirmationThresholdUpdated>()
        }
        ///Creates a new event filter for the [`GlobalRootDisabled`] event.
        pub fn GlobalRootDisabled_filter(
            &self,
        ) -> alloy_contract::Event<&P, GlobalRootDisabled, N> {
            self.event_filter::<GlobalRootDisabled>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`NewGlobalTableRoot`] event.
        pub fn NewGlobalTableRoot_filter(
            &self,
        ) -> alloy_contract::Event<&P, NewGlobalTableRoot, N> {
            self.event_filter::<NewGlobalTableRoot>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<&P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`Paused`] event.
        pub fn Paused_filter(&self) -> alloy_contract::Event<&P, Paused, N> {
            self.event_filter::<Paused>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<&P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
