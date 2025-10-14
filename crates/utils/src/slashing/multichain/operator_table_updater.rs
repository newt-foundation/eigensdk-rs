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

    error CertificateInvalid();
    error GlobalTableRootInFuture();
    error GlobalTableRootStale();
    error InvalidConfirmationThreshold();
    error InvalidCurveType();
    error InvalidGlobalTableRoot();
    error InvalidMessageHash();
    error InvalidOperatorSetProof();
    error InvalidProofLength();
    error InvalidRoot();
    error InvalidShortString();
    error InvalidSignatureLength();
    error StringTooLong(string str);
    error TableUpdateForPastTimestamp();

    event GlobalRootConfirmationThresholdUpdated(uint16 bps);
    event GlobalRootConfirmerSetUpdated(OperatorSet operatorSet);
    event GlobalRootDisabled(bytes32 indexed globalTableRoot);
    event Initialized(uint8 version);
    event NewGlobalTableRoot(uint32 indexed referenceTimestamp, bytes32 indexed globalTableRoot);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    constructor(address _bn254CertificateVerifier, address _ecdsaCertificateVerifier, string _version);

    function GLOBAL_TABLE_ROOT_CERT_TYPEHASH() external view returns (bytes32);
    function MAX_BPS() external view returns (uint16);
    function bn254CertificateVerifier() external view returns (address);
    function confirmGlobalTableRoot(IBN254CertificateVerifierTypes.BN254Certificate memory globalTableRootCert, bytes32 globalTableRoot, uint32 referenceTimestamp, uint32 referenceBlockNumber) external;
    function disableRoot(bytes32 globalTableRoot) external;
    function ecdsaCertificateVerifier() external view returns (address);
    function getCertificateVerifier(IKeyRegistrarTypes.CurveType curveType) external view returns (address);
    function getCurrentGlobalTableRoot() external view returns (bytes32);
    function getGlobalConfirmerSetReferenceTimestamp() external view returns (uint32);
    function getGlobalRootConfirmerSet() external view returns (OperatorSet memory);
    function getGlobalTableRootByTimestamp(uint32 referenceTimestamp) external view returns (bytes32);
    function getGlobalTableUpdateMessageHash(bytes32 globalTableRoot, uint32 referenceTimestamp, uint32 referenceBlockNumber) external pure returns (bytes32);
    function getLatestReferenceBlockNumber() external view returns (uint32);
    function getLatestReferenceTimestamp() external view returns (uint32);
    function getReferenceBlockNumberByTimestamp(uint32 referenceTimestamp) external view returns (uint32);
    function getReferenceTimestampByBlockNumber(uint32 referenceBlockNumber) external view returns (uint32);
    function globalRootConfirmationThreshold() external view returns (uint16);
    function initialize(address owner, OperatorSet memory _globalRootConfirmerSet, uint16 _globalRootConfirmationThreshold, uint32 referenceTimestamp, IOperatorTableCalculatorTypes.BN254OperatorSetInfo memory globalRootConfirmerSetInfo, ICrossChainRegistryTypes.OperatorSetConfig memory globalRootConfirmerSetConfig, bytes32 initialGlobalTableRoot) external;
    function isRootValid(bytes32 globalTableRoot) external view returns (bool);
    function isRootValidByTimestamp(uint32 referenceTimestamp) external view returns (bool);
    function owner() external view returns (address);
    function renounceOwnership() external;
    function setGlobalRootConfirmationThreshold(uint16 bps) external;
    function setGlobalRootConfirmerSet(OperatorSet memory operatorSet) external;
    function transferOwnership(address newOwner) external;
    function updateGlobalRootConfirmerSet(uint32 referenceTimestamp, IOperatorTableCalculatorTypes.BN254OperatorSetInfo memory globalRootConfirmerSetInfo, ICrossChainRegistryTypes.OperatorSetConfig memory globalRootConfirmerSetConfig) external;
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
        "name": "_version",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "nonpayable"
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
    "name": "getGlobalConfirmerSetReferenceTimestamp",
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
    "name": "getGlobalRootConfirmerSet",
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
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_globalRootConfirmerSet",
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
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "globalRootConfirmerSetInfo",
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
        "name": "globalRootConfirmerSetConfig",
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
      },
      {
        "name": "initialGlobalTableRoot",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "setGlobalRootConfirmerSet",
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
    "name": "updateGlobalRootConfirmerSet",
    "inputs": [
      {
        "name": "referenceTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "globalRootConfirmerSetInfo",
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
        "name": "globalRootConfirmerSetConfig",
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
    "name": "GlobalRootConfirmerSetUpdated",
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
    "type": "error",
    "name": "CertificateInvalid",
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
    "name": "InvalidGlobalTableRoot",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidMessageHash",
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
    "name": "InvalidSignatureLength",
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
    ///0x60e060405234801561000f575f5ffd5b506040516123b73803806123b783398101604081905261002e91610188565b6001600160a01b03808416608052821660a0528061004b8161005f565b60c052506100576100a5565b5050506102b9565b5f5f829050601f81511115610092578260405163305a27a960e01b8152600401610089919061025e565b60405180910390fd5b805161009d82610293565b179392505050565b5f54610100900460ff161561010c5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608401610089565b5f5460ff9081161461015b575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b0381168114610171575f5ffd5b50565b634e487b7160e01b5f52604160045260245ffd5b5f5f5f6060848603121561019a575f5ffd5b83516101a58161015d565b60208501519093506101b68161015d565b60408501519092506001600160401b038111156101d1575f5ffd5b8401601f810186136101e1575f5ffd5b80516001600160401b038111156101fa576101fa610174565b604051601f8201601f19908116603f011681016001600160401b038111828210171561022857610228610174565b60405281815282820160200188101561023f575f5ffd5b8160208401602083015e5f602083830101528093505050509250925092565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b805160208083015191908110156102b3575f198160200360031b1b821691505b50919050565b60805160a05160c0516120a36103145f395f6107bc01525f818161042b0152818161083d0152610a5b01525f81816104520152818161053c015281816107fd015281816109b601528181610c8a0152610f0001526120a35ff3fe608060405234801561000f575f5ffd5b50600436106101bb575f3560e01c806364e1df84116100f3578063c252aa2211610093578063c5916a391161006e578063c5916a39146104bb578063eaaed9d5146104e0578063f2fde38b146104f3578063fd967f4714610506575f5ffd5b8063c252aa2214610474578063c3621f0a14610495578063c3be1e33146104a8575f5ffd5b80638da5cb5b116100ce5780638da5cb5b146104025780639ea9477814610413578063ad0f958214610426578063b8c143061461044d575f5ffd5b806364e1df841461039a5780636f728c50146103cf578063715018a6146103fa575f5ffd5b806330ef41b41161015e5780633ef6cd7a116101395780633ef6cd7a146103025780634624e6a314610329578063462828891461033d57806354fd4d5014610385575f5ffd5b806330ef41b41461029857806331a599d2146102ca578063383b9b70146102ef575f5ffd5b80631ab78d90116101995780631ab78d901461021e5780632370356c1461023157806323b7b5b21461024457806328522d791461026c575f5ffd5b80630371406e146101bf5780630f3f8edd146101d4578063193b79f3146101f6575b5f5ffd5b6101d26101cd3660046112b1565b61050f565b005b6101dc610523565b60405163ffffffff90911681526020015b60405180910390f35b6101dc6102043660046112e3565b63ffffffff9081165f908152606960205260409020541690565b6101d261022c36600461130e565b6105b6565b6101d261023f36600461137b565b6105ce565b6101dc6102523660046112e3565b63ffffffff9081165f908152606860205260409020541690565b60655462010000900463ffffffff165f908152606760205260409020545b6040519081526020016101ed565b6102ba6102a6366004611394565b5f908152606a602052604090205460ff1690565b60405190151581526020016101ed565b60655462010000900463ffffffff9081165f90815260686020526040902054166101dc565b6101d26102fd3660046113bf565b6105df565b61028a7f4491f5ee91595f938885ef73c9a1fa8a6d14ff9b9dab4aa24b8802bbb9bfc1cc81565b60655462010000900463ffffffff166101dc565b6040805180820182525f80825260209182015281518083019092526066546001600160a01b0381168352600160a01b900463ffffffff16908201526040516101ed919061147b565b61038d6107b5565b6040516101ed9190611489565b6102ba6103a83660046112e3565b63ffffffff165f908152606760209081526040808320548352606a90915290205460ff1690565b6103e26103dd3660046114cc565b6107e0565b6040516001600160a01b0390911681526020016101ed565b6101d261087f565b6033546001600160a01b03166103e2565b6101d2610421366004611529565b610892565b6103e27f000000000000000000000000000000000000000000000000000000000000000081565b6103e27f000000000000000000000000000000000000000000000000000000000000000081565b6065546104829061ffff1681565b60405161ffff90911681526020016101ed565b6101d26104a3366004611394565b610ac0565b61028a6104b63660046115c4565b610b35565b61028a6104c93660046112e3565b63ffffffff165f9081526067602052604090205490565b6101d26104ee366004611603565b610b9d565b6101d261050136600461166f565b610dcf565b61048261271081565b610517610e45565b61052081610e9f565b50565b604051635ddb9b5b60e01b81525f906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690635ddb9b5b906105729060669060040161168a565b602060405180830381865afa15801561058d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105b191906116b1565b905090565b6105be610e45565b6105c9838383610ee9565b505050565b6105d6610e45565b61052081610f6e565b5f54610100900460ff16158080156105fd57505f54600160ff909116105b806106165750303b15801561061657505f5460ff166001145b61067e5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff19166001179055801561069f575f805461ff0019166101001790555b6106a888610fd9565b6106b187610e9f565b6106ba86610f6e565b6106c5858585610ee9565b63ffffffff8086165f818152606760209081526040808320879055868352606a8252808320805460ff191660011790558383526068825280832080544390961663ffffffff19968716811790915583526069909152808220805490941683179093556065805462010000840265ffffffff000019909116179055915184927f010dcbe0d1e019c93357711f7bb6287d543b7ff7de74f29df3fb5ecceec8d36991a380156107ab575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050505050505050565b60606105b17f000000000000000000000000000000000000000000000000000000000000000061102a565b5f60028260028111156107f5576107f56116cc565b0361082157507f0000000000000000000000000000000000000000000000000000000000000000919050565b6001826002811115610835576108356116cc565b0361086157507f0000000000000000000000000000000000000000000000000000000000000000919050565b60405163fdea7c0960e01b815260040160405180910390fd5b919050565b610887610e45565b6108905f610fd9565b565b5f5f5f5f6108a08686611067565b5f8e8152606a60205260409020549397509195509350915060ff166108d85760405163504570e360e01b815260040160405180910390fd5b6108e1836107e0565b6001600160a01b0316635ddb9b5b856040518263ffffffff1660e01b815260040161090c919061147b565b602060405180830381865afa158015610927573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061094b91906116b1565b63ffffffff168b63ffffffff16116109765760405163207617df60e01b815260040160405180910390fd5b61099b8b8b8b8b8b8b8b60405161098e9291906116e0565b60405180910390206110ae565b60028360028111156109af576109af6116cc565b03610a40577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316636738c40b858d6109ee8561114f565b866040518563ffffffff1660e01b8152600401610a0e9493929190611729565b5f604051808303815f87803b158015610a25575f5ffd5b505af1158015610a37573d5f5f3e3d5ffd5b50505050610ab3565b6001836002811115610a5457610a546116cc565b03610861577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166356d482f5858d610a9385611171565b866040518563ffffffff1660e01b8152600401610a0e94939291906117a5565b5050505050505050505050565b610ac8610e45565b5f818152606a602052604090205460ff16610af65760405163504570e360e01b815260040160405180910390fd5b5f818152606a6020526040808220805460ff191690555182917f8bd43de1250f58fe6ec9a78671a8b78dba70f0018656d157a3aeaabec389df3491a250565b604080517f4491f5ee91595f938885ef73c9a1fa8a6d14ff9b9dab4aa24b8802bbb9bfc1cc602082015290810184905263ffffffff8084166060830152821660808201525f9060a0016040516020818303038152906040528051906020012090509392505050565b428263ffffffff161115610bc457604051635a119db560e11b815260040160405180910390fd5b60655463ffffffff62010000909104811690831611610bf65760405163037fa86b60e31b815260040160405180910390fd5b610c01838383610b35565b846020013514610c2457604051638b56642d60e01b815260040160405180910390fd5b6040805160018082528183019092525f91602080830190803683375050606554825192935061ffff16918391505f90610c5f57610c5f61185e565b61ffff90921660209283029190910190910152604051625f5e5d60e21b81525f906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063017d797490610cc4906066908a908790600401611990565b6020604051808303815f875af1158015610ce0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d049190611b1d565b905080610d2457604051633042041f60e21b815260040160405180910390fd5b6065805463ffffffff80871662010000810265ffffffff000019909316929092179092555f818152606860209081526040808320805495891663ffffffff1996871681179091558352606982528083208054909516841790945582825260678152838220899055888252606a9052828120805460ff19166001179055915187927f010dcbe0d1e019c93357711f7bb6287d543b7ff7de74f29df3fb5ecceec8d36991a3505050505050565b610dd7610e45565b6001600160a01b038116610e3c5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610675565b61052081610fd9565b6033546001600160a01b031633146108905760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610675565b806066610eac8282611b3c565b9050507f20100394950e66014c25009b45d12b675210a6e7a002044a0e3de6544e3c4b3781604051610ede9190611bcd565b60405180910390a150565b604051636738c40b60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690636738c40b90610f3c90606690879087908790600401611bdb565b5f604051808303815f87803b158015610f53575f5ffd5b505af1158015610f65573d5f5f3e3d5ffd5b50505050505050565b61271061ffff82161115610f95576040516307336f0360e11b815260040160405180910390fd5b6065805461ffff191661ffff83169081179091556040519081527ff5d1836df8fcd7c1e54047e94ac8773d2855395603e2ef9ba5f5f16905f2259290602001610ede565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b60605f61103683611187565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b604080518082019091525f8082526020820152604080518082019091525f8082526020820181905290606061109e85870187611d2e565b9299919850965090945092505050565b63ffffffff86165f9081526067602052604090205485146110e25760405163639d09b560e11b815260040160405180910390fd5b61112a83838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508992508591505063ffffffff88166111ae565b6111475760405163afa42ca760e01b815260040160405180910390fd5b505050505050565b61115761125c565b8180602001905181019061116b9190611e7f565b92915050565b60608180602001905181019061116b9190611f2e565b5f60ff8216601f81111561116b57604051632cd44ac360e21b815260040160405180910390fd5b5f836111bb8685856111c5565b1495945050505050565b5f602084516111d4919061202f565b156111f2576040516313717da960e21b815260040160405180910390fd5b8260205b855181116112535761120960028561202f565b5f0361122a57815f528086015160205260405f209150600284049350611241565b808601515f528160205260405f2091506002840493505b61124c60208261204e565b90506111f6565b50949350505050565b60405180608001604052805f81526020015f815260200161128e60405180604001604052805f81526020015f81525090565b8152602001606081525090565b5f604082840312156112ab575f5ffd5b50919050565b5f604082840312156112c1575f5ffd5b6112cb838361129b565b9392505050565b63ffffffff81168114610520575f5ffd5b5f602082840312156112f3575f5ffd5b81356112cb816112d2565b5f60a082840312156112ab575f5ffd5b5f5f5f60808486031215611320575f5ffd5b833561132b816112d2565b925060208401356001600160401b03811115611345575f5ffd5b611351868287016112fe565b925050611361856040860161129b565b90509250925092565b803561ffff8116811461087a575f5ffd5b5f6020828403121561138b575f5ffd5b6112cb8261136a565b5f602082840312156113a4575f5ffd5b5035919050565b6001600160a01b0381168114610520575f5ffd5b5f5f5f5f5f5f5f610120888a0312156113d6575f5ffd5b87356113e1816113ab565b96506113f08960208a0161129b565b95506113fe6060890161136a565b9450608088013561140e816112d2565b935060a08801356001600160401b03811115611428575f5ffd5b6114348a828b016112fe565b9350506114448960c08a0161129b565b9699959850939692959194919350506101009091013590565b80516001600160a01b0316825260209081015163ffffffff16910152565b6040810161116b828461145d565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80356003811061087a575f5ffd5b5f602082840312156114dc575f5ffd5b6112cb826114be565b5f5f83601f8401126114f5575f5ffd5b5081356001600160401b0381111561150b575f5ffd5b602083019150836020828501011115611522575f5ffd5b9250929050565b5f5f5f5f5f5f5f60a0888a03121561153f575f5ffd5b873561154a816112d2565b9650602088013595506040880135611561816112d2565b945060608801356001600160401b0381111561157b575f5ffd5b6115878a828b016114e5565b90955093505060808801356001600160401b038111156115a5575f5ffd5b6115b18a828b016114e5565b989b979a50959850939692959293505050565b5f5f5f606084860312156115d6575f5ffd5b8335925060208401356115e8816112d2565b915060408401356115f8816112d2565b809150509250925092565b5f5f5f5f60808587031215611616575f5ffd5b84356001600160401b0381111561162b575f5ffd5b8501610120818803121561163d575f5ffd5b9350602085013592506040850135611654816112d2565b91506060850135611664816112d2565b939692955090935050565b5f6020828403121561167f575f5ffd5b81356112cb816113ab565b6040810161116b8284546001600160a01b038116825260a01c63ffffffff16602090910152565b5f602082840312156116c1575f5ffd5b81516112cb816112d2565b634e487b7160e01b5f52602160045260245ffd5b818382375f9101908152919050565b5f8151808452602084019350602083015f5b8281101561171f578151865260209586019590910190600101611701565b5093949350505050565b611733818661145d565b63ffffffff8416604082015260c06060820152825160c0820152602083015160e08201525f60408401518051610100840152602081015161012084015250606084015160a061014084015261178c6101608401826116ef565b91505061179c608083018461145d565b95945050505050565b5f60c082016117b4838861145d565b63ffffffff8616604084015260c0606084015280855180835260e08501915060e08160051b8601019250602087015f5b828110156118355786850360df19018452815180516001600160a01b0316865260209081015160409187018290529061181f908701826116ef565b95505060209384019391909101906001016117e4565b505050508091505061179c608083018461145d565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b5f5f8335601e19843603018112611887575f5ffd5b83016020810192503590506001600160401b038111156118a5575f5ffd5b8060051b3603821315611522575f5ffd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b5f8235605e198336030181126118f2575f5ffd5b90910192915050565b8183525f6001600160fb1b03831115611912575f5ffd5b8260051b80836020870137939093016020019392505050565b80358252602080820135908301525f6119476040830183611872565b6060604086015261179c6060860182846118fb565b5f8151808452602084019350602083015f5b8281101561171f57815161ffff1686526020958601959091019060010161196e565b6119b38185546001600160a01b038116825260a01c63ffffffff16602090910152565b608060408201525f6101a0820184356119cb816112d2565b63ffffffff166080840152602085013560a0840152604085013560c0840152606085013560e0840152604060808601610100850137604060c08601610140850137611a1a610100860186611872565b610120610180860152828184526101c0860190506101c08260051b8701019350825f5b83811015611afc578786036101bf19018352611a5982866118de565b8035611a64816112d2565b63ffffffff168752602081013536829003601e19018112611a83575f5ffd5b81016020810190356001600160401b03811115611a9e575f5ffd5b803603821315611aac575f5ffd5b606060208a0152611ac160608a0182846118b6565b915050611ad160408301836118de565b91508781036040890152611ae5818361192b565b975050506020928301929190910190600101611a3d565b50505050508281036060840152611b13818561195c565b9695505050505050565b5f60208284031215611b2d575f5ffd5b815180151581146112cb575f5ffd5b8135611b47816113ab565b81546001600160a01b031981166001600160a01b039290921691821783556020840135611b73816112d2565b6001600160c01b03199190911690911760a09190911b63ffffffff60a01b1617905550565b8035611ba3816113ab565b6001600160a01b031682526020810135611bbc816112d2565b63ffffffff81166020840152505050565b6040810161116b8284611b98565b611bfe8186546001600160a01b038116825260a01c63ffffffff16602090910152565b63ffffffff841660408281019190915260c06060808401829052853591840191909152602085013560e0840152908401356101008301528301356101208201525f611c4c6080850185611872565b60a0610140850152611c63610160850182846118fb565b9250505061179c6080830184611b98565b604080519081016001600160401b0381118282101715611c9657611c9661184a565b60405290565b604051608081016001600160401b0381118282101715611c9657611c9661184a565b604051601f8201601f191681016001600160401b0381118282101715611ce657611ce661184a565b604052919050565b5f60408284031215611cfe575f5ffd5b611d06611c74565b90508135611d13816113ab565b81526020820135611d23816112d2565b602082015292915050565b5f5f5f5f60c08587031215611d41575f5ffd5b611d4b8686611cee565b9350611d59604086016114be565b9250611d688660608701611cee565b915060a08501356001600160401b03811115611d82575f5ffd5b8501601f81018713611d92575f5ffd5b80356001600160401b03811115611dab57611dab61184a565b611dbe601f8201601f1916602001611cbe565b818152886020838501011115611dd2575f5ffd5b816020840160208301375f6020838301015280935050505092959194509250565b5f6001600160401b03821115611e0b57611e0b61184a565b5060051b60200190565b5f82601f830112611e24575f5ffd5b8151611e37611e3282611df3565b611cbe565b8082825260208201915060208360051b860101925085831115611e58575f5ffd5b602085015b83811015611e75578051835260209283019201611e5d565b5095945050505050565b5f60208284031215611e8f575f5ffd5b81516001600160401b03811115611ea4575f5ffd5b820180840360a0811215611eb6575f5ffd5b611ebe611c9c565b82518152602080840151908201526040603f1983011215611edd575f5ffd5b611ee5611c74565b604084810151825260608501516020830152820152608083015191506001600160401b03821115611f14575f5ffd5b611f2086838501611e15565b606082015295945050505050565b5f60208284031215611f3e575f5ffd5b81516001600160401b03811115611f53575f5ffd5b8201601f81018413611f63575f5ffd5b8051611f71611e3282611df3565b8082825260208201915060208360051b850101925086831115611f92575f5ffd5b602084015b838110156120245780516001600160401b03811115611fb4575f5ffd5b85016040818a03601f19011215611fc9575f5ffd5b611fd1611c74565b6020820151611fdf816113ab565b815260408201516001600160401b03811115611ff9575f5ffd5b6120088b602083860101611e15565b6020830152508085525050602083019250602081019050611f97565b509695505050505050565b5f8261204957634e487b7160e01b5f52601260045260245ffd5b500690565b8082018082111561116b57634e487b7160e01b5f52601160045260245ffdfea26469706673582212209a012b8a0dd1d70e35673949eea8a5b8de19ce04fe109790849f9420776708ba64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa#\xB78\x03\x80a#\xB7\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\x88V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x80R\x82\x16`\xA0R\x80a\0K\x81a\0_V[`\xC0RPa\0Wa\0\xA5V[PPPa\x02\xB9V[__\x82\x90P`\x1F\x81Q\x11\x15a\0\x92W\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\0\x89\x91\x90a\x02^V[`@Q\x80\x91\x03\x90\xFD[\x80Qa\0\x9D\x82a\x02\x93V[\x17\x93\x92PPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x01\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01a\0\x89V[_T`\xFF\x90\x81\x16\x14a\x01[W_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01qW__\xFD[PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[___``\x84\x86\x03\x12\x15a\x01\x9AW__\xFD[\x83Qa\x01\xA5\x81a\x01]V[` \x85\x01Q\x90\x93Pa\x01\xB6\x81a\x01]V[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xD1W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x01\xE1W__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\xFAWa\x01\xFAa\x01tV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02(Wa\x02(a\x01tV[`@R\x81\x81R\x82\x82\x01` \x01\x88\x10\x15a\x02?W__\xFD[\x81` \x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x02\xB3W_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\x80Q`\xA0Q`\xC0Qa \xA3a\x03\x14_9_a\x07\xBC\x01R_\x81\x81a\x04+\x01R\x81\x81a\x08=\x01Ra\n[\x01R_\x81\x81a\x04R\x01R\x81\x81a\x05<\x01R\x81\x81a\x07\xFD\x01R\x81\x81a\t\xB6\x01R\x81\x81a\x0C\x8A\x01Ra\x0F\0\x01Ra \xA3_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xBBW_5`\xE0\x1C\x80cd\xE1\xDF\x84\x11a\0\xF3W\x80c\xC2R\xAA\"\x11a\0\x93W\x80c\xC5\x91j9\x11a\0nW\x80c\xC5\x91j9\x14a\x04\xBBW\x80c\xEA\xAE\xD9\xD5\x14a\x04\xE0W\x80c\xF2\xFD\xE3\x8B\x14a\x04\xF3W\x80c\xFD\x96\x7FG\x14a\x05\x06W__\xFD[\x80c\xC2R\xAA\"\x14a\x04tW\x80c\xC3b\x1F\n\x14a\x04\x95W\x80c\xC3\xBE\x1E3\x14a\x04\xA8W__\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xCEW\x80c\x8D\xA5\xCB[\x14a\x04\x02W\x80c\x9E\xA9Gx\x14a\x04\x13W\x80c\xAD\x0F\x95\x82\x14a\x04&W\x80c\xB8\xC1C\x06\x14a\x04MW__\xFD[\x80cd\xE1\xDF\x84\x14a\x03\x9AW\x80cor\x8CP\x14a\x03\xCFW\x80cqP\x18\xA6\x14a\x03\xFAW__\xFD[\x80c0\xEFA\xB4\x11a\x01^W\x80c>\xF6\xCDz\x11a\x019W\x80c>\xF6\xCDz\x14a\x03\x02W\x80cF$\xE6\xA3\x14a\x03)W\x80cF((\x89\x14a\x03=W\x80cT\xFDMP\x14a\x03\x85W__\xFD[\x80c0\xEFA\xB4\x14a\x02\x98W\x80c1\xA5\x99\xD2\x14a\x02\xCAW\x80c8;\x9Bp\x14a\x02\xEFW__\xFD[\x80c\x1A\xB7\x8D\x90\x11a\x01\x99W\x80c\x1A\xB7\x8D\x90\x14a\x02\x1EW\x80c#p5l\x14a\x021W\x80c#\xB7\xB5\xB2\x14a\x02DW\x80c(R-y\x14a\x02lW__\xFD[\x80c\x03q@n\x14a\x01\xBFW\x80c\x0F?\x8E\xDD\x14a\x01\xD4W\x80c\x19;y\xF3\x14a\x01\xF6W[__\xFD[a\x01\xD2a\x01\xCD6`\x04a\x12\xB1V[a\x05\x0FV[\0[a\x01\xDCa\x05#V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xDCa\x02\x046`\x04a\x12\xE3V[c\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`i` R`@\x90 T\x16\x90V[a\x01\xD2a\x02,6`\x04a\x13\x0EV[a\x05\xB6V[a\x01\xD2a\x02?6`\x04a\x13{V[a\x05\xCEV[a\x01\xDCa\x02R6`\x04a\x12\xE3V[c\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`h` R`@\x90 T\x16\x90V[`eTb\x01\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16_\x90\x81R`g` R`@\x90 T[`@Q\x90\x81R` \x01a\x01\xEDV[a\x02\xBAa\x02\xA66`\x04a\x13\x94V[_\x90\x81R`j` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xEDV[`eTb\x01\0\0\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`h` R`@\x90 T\x16a\x01\xDCV[a\x01\xD2a\x02\xFD6`\x04a\x13\xBFV[a\x05\xDFV[a\x02\x8A\x7FD\x91\xF5\xEE\x91Y_\x93\x88\x85\xEFs\xC9\xA1\xFA\x8Am\x14\xFF\x9B\x9D\xABJ\xA2K\x88\x02\xBB\xB9\xBF\xC1\xCC\x81V[`eTb\x01\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16a\x01\xDCV[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`fT`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@Qa\x01\xED\x91\x90a\x14{V[a\x03\x8Da\x07\xB5V[`@Qa\x01\xED\x91\x90a\x14\x89V[a\x02\xBAa\x03\xA86`\x04a\x12\xE3V[c\xFF\xFF\xFF\xFF\x16_\x90\x81R`g` \x90\x81R`@\x80\x83 T\x83R`j\x90\x91R\x90 T`\xFF\x16\x90V[a\x03\xE2a\x03\xDD6`\x04a\x14\xCCV[a\x07\xE0V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xEDV[a\x01\xD2a\x08\x7FV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xE2V[a\x01\xD2a\x04!6`\x04a\x15)V[a\x08\x92V[a\x03\xE2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xE2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`eTa\x04\x82\x90a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xEDV[a\x01\xD2a\x04\xA36`\x04a\x13\x94V[a\n\xC0V[a\x02\x8Aa\x04\xB66`\x04a\x15\xC4V[a\x0B5V[a\x02\x8Aa\x04\xC96`\x04a\x12\xE3V[c\xFF\xFF\xFF\xFF\x16_\x90\x81R`g` R`@\x90 T\x90V[a\x01\xD2a\x04\xEE6`\x04a\x16\x03V[a\x0B\x9DV[a\x01\xD2a\x05\x016`\x04a\x16oV[a\r\xCFV[a\x04\x82a'\x10\x81V[a\x05\x17a\x0EEV[a\x05 \x81a\x0E\x9FV[PV[`@Qc]\xDB\x9B[`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c]\xDB\x9B[\x90a\x05r\x90`f\x90`\x04\x01a\x16\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xB1\x91\x90a\x16\xB1V[\x90P\x90V[a\x05\xBEa\x0EEV[a\x05\xC9\x83\x83\x83a\x0E\xE9V[PPPV[a\x05\xD6a\x0EEV[a\x05 \x81a\x0FnV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x05\xFDWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x06\x16WP0;\x15\x80\x15a\x06\x16WP_T`\xFF\x16`\x01\x14[a\x06~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x06\x9FW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x06\xA8\x88a\x0F\xD9V[a\x06\xB1\x87a\x0E\x9FV[a\x06\xBA\x86a\x0FnV[a\x06\xC5\x85\x85\x85a\x0E\xE9V[c\xFF\xFF\xFF\xFF\x80\x86\x16_\x81\x81R`g` \x90\x81R`@\x80\x83 \x87\x90U\x86\x83R`j\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U\x83\x83R`h\x82R\x80\x83 \x80TC\x90\x96\x16c\xFF\xFF\xFF\xFF\x19\x96\x87\x16\x81\x17\x90\x91U\x83R`i\x90\x91R\x80\x82 \x80T\x90\x94\x16\x83\x17\x90\x93U`e\x80Tb\x01\0\0\x84\x02e\xFF\xFF\xFF\xFF\0\0\x19\x90\x91\x16\x17\x90U\x91Q\x84\x92\x7F\x01\r\xCB\xE0\xD1\xE0\x19\xC93Wq\x1F{\xB6(}T;\x7F\xF7\xDEt\xF2\x9D\xF3\xFB^\xCC\xEE\xC8\xD3i\x91\xA3\x80\x15a\x07\xABW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPV[``a\x05\xB1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10*V[_`\x02\x82`\x02\x81\x11\x15a\x07\xF5Wa\x07\xF5a\x16\xCCV[\x03a\x08!WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[`\x01\x82`\x02\x81\x11\x15a\x085Wa\x085a\x16\xCCV[\x03a\x08aWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[`@Qc\xFD\xEA|\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a\x08\x87a\x0EEV[a\x08\x90_a\x0F\xD9V[V[____a\x08\xA0\x86\x86a\x10gV[_\x8E\x81R`j` R`@\x90 T\x93\x97P\x91\x95P\x93P\x91P`\xFF\x16a\x08\xD8W`@QcPEp\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xE1\x83a\x07\xE0V[`\x01`\x01`\xA0\x1B\x03\x16c]\xDB\x9B[\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x0C\x91\x90a\x14{V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tK\x91\x90a\x16\xB1V[c\xFF\xFF\xFF\xFF\x16\x8Bc\xFF\xFF\xFF\xFF\x16\x11a\tvW`@Qc v\x17\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\x9B\x8B\x8B\x8B\x8B\x8B\x8B\x8B`@Qa\t\x8E\x92\x91\x90a\x16\xE0V[`@Q\x80\x91\x03\x90 a\x10\xAEV[`\x02\x83`\x02\x81\x11\x15a\t\xAFWa\t\xAFa\x16\xCCV[\x03a\n@W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cg8\xC4\x0B\x85\x8Da\t\xEE\x85a\x11OV[\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x0E\x94\x93\x92\x91\x90a\x17)V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n%W__\xFD[PZ\xF1\x15\x80\x15a\n7W=__>=_\xFD[PPPPa\n\xB3V[`\x01\x83`\x02\x81\x11\x15a\nTWa\nTa\x16\xCCV[\x03a\x08aW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cV\xD4\x82\xF5\x85\x8Da\n\x93\x85a\x11qV[\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x0E\x94\x93\x92\x91\x90a\x17\xA5V[PPPPPPPPPPPV[a\n\xC8a\x0EEV[_\x81\x81R`j` R`@\x90 T`\xFF\x16a\n\xF6W`@QcPEp\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`j` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x82\x91\x7F\x8B\xD4=\xE1%\x0FX\xFEn\xC9\xA7\x86q\xA8\xB7\x8D\xBAp\xF0\x01\x86V\xD1W\xA3\xAE\xAA\xBE\xC3\x89\xDF4\x91\xA2PV[`@\x80Q\x7FD\x91\xF5\xEE\x91Y_\x93\x88\x85\xEFs\xC9\xA1\xFA\x8Am\x14\xFF\x9B\x9D\xABJ\xA2K\x88\x02\xBB\xB9\xBF\xC1\xCC` \x82\x01R\x90\x81\x01\x84\x90Rc\xFF\xFF\xFF\xFF\x80\x84\x16``\x83\x01R\x82\x16`\x80\x82\x01R_\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[B\x82c\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\xC4W`@QcZ\x11\x9D\xB5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`eTc\xFF\xFF\xFF\xFFb\x01\0\0\x90\x91\x04\x81\x16\x90\x83\x16\x11a\x0B\xF6W`@Qc\x03\x7F\xA8k`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\x01\x83\x83\x83a\x0B5V[\x84` \x015\x14a\x0C$W`@Qc\x8BVd-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837PP`eT\x82Q\x92\x93Pa\xFF\xFF\x16\x91\x83\x91P_\x90a\x0C_Wa\x0C_a\x18^V[a\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@Qb_^]`\xE2\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x01}yt\x90a\x0C\xC4\x90`f\x90\x8A\x90\x87\x90`\x04\x01a\x19\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xE0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x04\x91\x90a\x1B\x1DV[\x90P\x80a\r$W`@Qc0B\x04\x1F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80Tc\xFF\xFF\xFF\xFF\x80\x87\x16b\x01\0\0\x81\x02e\xFF\xFF\xFF\xFF\0\0\x19\x90\x93\x16\x92\x90\x92\x17\x90\x92U_\x81\x81R`h` \x90\x81R`@\x80\x83 \x80T\x95\x89\x16c\xFF\xFF\xFF\xFF\x19\x96\x87\x16\x81\x17\x90\x91U\x83R`i\x82R\x80\x83 \x80T\x90\x95\x16\x84\x17\x90\x94U\x82\x82R`g\x81R\x83\x82 \x89\x90U\x88\x82R`j\x90R\x82\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U\x91Q\x87\x92\x7F\x01\r\xCB\xE0\xD1\xE0\x19\xC93Wq\x1F{\xB6(}T;\x7F\xF7\xDEt\xF2\x9D\xF3\xFB^\xCC\xEE\xC8\xD3i\x91\xA3PPPPPPV[a\r\xD7a\x0EEV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06uV[a\x05 \x81a\x0F\xD9V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06uV[\x80`fa\x0E\xAC\x82\x82a\x1B<V[\x90PP\x7F \x10\x03\x94\x95\x0Ef\x01L%\0\x9BE\xD1+gR\x10\xA6\xE7\xA0\x02\x04J\x0E=\xE6TN<K7\x81`@Qa\x0E\xDE\x91\x90a\x1B\xCDV[`@Q\x80\x91\x03\x90\xA1PV[`@Qcg8\xC4\x0B`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cg8\xC4\x0B\x90a\x0F<\x90`f\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x1B\xDBV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0FSW__\xFD[PZ\xF1\x15\x80\x15a\x0FeW=__>=_\xFD[PPPPPPPV[a'\x10a\xFF\xFF\x82\x16\x11\x15a\x0F\x95W`@Qc\x073o\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80Ta\xFF\xFF\x19\x16a\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xF5\xD1\x83m\xF8\xFC\xD7\xC1\xE5@G\xE9J\xC8w=(U9V\x03\xE2\xEF\x9B\xA5\xF5\xF1i\x05\xF2%\x92\x90` \x01a\x0E\xDEV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[``_a\x106\x83a\x11\x87V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01\x81\x90R\x90``a\x10\x9E\x85\x87\x01\x87a\x1D.V[\x92\x99\x91\x98P\x96P\x90\x94P\x92PPPV[c\xFF\xFF\xFF\xFF\x86\x16_\x90\x81R`g` R`@\x90 T\x85\x14a\x10\xE2W`@Qcc\x9D\t\xB5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11*\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x89\x92P\x85\x91PPc\xFF\xFF\xFF\xFF\x88\x16a\x11\xAEV[a\x11GW`@Qc\xAF\xA4,\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[a\x11Wa\x12\\V[\x81\x80` \x01\x90Q\x81\x01\x90a\x11k\x91\x90a\x1E\x7FV[\x92\x91PPV[``\x81\x80` \x01\x90Q\x81\x01\x90a\x11k\x91\x90a\x1F.V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x11kW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83a\x11\xBB\x86\x85\x85a\x11\xC5V[\x14\x95\x94PPPPPV[_` \x84Qa\x11\xD4\x91\x90a /V[\x15a\x11\xF2W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82` [\x85Q\x81\x11a\x12SWa\x12\t`\x02\x85a /V[_\x03a\x12*W\x81_R\x80\x86\x01Q` R`@_ \x91P`\x02\x84\x04\x93Pa\x12AV[\x80\x86\x01Q_R\x81` R`@_ \x91P`\x02\x84\x04\x93P[a\x12L` \x82a NV[\x90Pa\x11\xF6V[P\x94\x93PPPPV[`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01a\x12\x8E`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01``\x81RP\x90V[_`@\x82\x84\x03\x12\x15a\x12\xABW__\xFD[P\x91\x90PV[_`@\x82\x84\x03\x12\x15a\x12\xC1W__\xFD[a\x12\xCB\x83\x83a\x12\x9BV[\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05 W__\xFD[_` \x82\x84\x03\x12\x15a\x12\xF3W__\xFD[\x815a\x12\xCB\x81a\x12\xD2V[_`\xA0\x82\x84\x03\x12\x15a\x12\xABW__\xFD[___`\x80\x84\x86\x03\x12\x15a\x13 W__\xFD[\x835a\x13+\x81a\x12\xD2V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13EW__\xFD[a\x13Q\x86\x82\x87\x01a\x12\xFEV[\x92PPa\x13a\x85`@\x86\x01a\x12\x9BV[\x90P\x92P\x92P\x92V[\x805a\xFF\xFF\x81\x16\x81\x14a\x08zW__\xFD[_` \x82\x84\x03\x12\x15a\x13\x8BW__\xFD[a\x12\xCB\x82a\x13jV[_` \x82\x84\x03\x12\x15a\x13\xA4W__\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05 W__\xFD[_______a\x01 \x88\x8A\x03\x12\x15a\x13\xD6W__\xFD[\x875a\x13\xE1\x81a\x13\xABV[\x96Pa\x13\xF0\x89` \x8A\x01a\x12\x9BV[\x95Pa\x13\xFE``\x89\x01a\x13jV[\x94P`\x80\x88\x015a\x14\x0E\x81a\x12\xD2V[\x93P`\xA0\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14(W__\xFD[a\x144\x8A\x82\x8B\x01a\x12\xFEV[\x93PPa\x14D\x89`\xC0\x8A\x01a\x12\x9BV[\x96\x99\x95\x98P\x93\x96\x92\x95\x91\x94\x91\x93PPa\x01\0\x90\x91\x015\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`@\x81\x01a\x11k\x82\x84a\x14]V[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x03\x81\x10a\x08zW__\xFD[_` \x82\x84\x03\x12\x15a\x14\xDCW__\xFD[a\x12\xCB\x82a\x14\xBEV[__\x83`\x1F\x84\x01\x12a\x14\xF5W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\x0BW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x15\"W__\xFD[\x92P\x92\x90PV[_______`\xA0\x88\x8A\x03\x12\x15a\x15?W__\xFD[\x875a\x15J\x81a\x12\xD2V[\x96P` \x88\x015\x95P`@\x88\x015a\x15a\x81a\x12\xD2V[\x94P``\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15{W__\xFD[a\x15\x87\x8A\x82\x8B\x01a\x14\xE5V[\x90\x95P\x93PP`\x80\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xA5W__\xFD[a\x15\xB1\x8A\x82\x8B\x01a\x14\xE5V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[___``\x84\x86\x03\x12\x15a\x15\xD6W__\xFD[\x835\x92P` \x84\x015a\x15\xE8\x81a\x12\xD2V[\x91P`@\x84\x015a\x15\xF8\x81a\x12\xD2V[\x80\x91PP\x92P\x92P\x92V[____`\x80\x85\x87\x03\x12\x15a\x16\x16W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16+W__\xFD[\x85\x01a\x01 \x81\x88\x03\x12\x15a\x16=W__\xFD[\x93P` \x85\x015\x92P`@\x85\x015a\x16T\x81a\x12\xD2V[\x91P``\x85\x015a\x16d\x81a\x12\xD2V[\x93\x96\x92\x95P\x90\x93PPV[_` \x82\x84\x03\x12\x15a\x16\x7FW__\xFD[\x815a\x12\xCB\x81a\x13\xABV[`@\x81\x01a\x11k\x82\x84T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16` \x90\x91\x01RV[_` \x82\x84\x03\x12\x15a\x16\xC1W__\xFD[\x81Qa\x12\xCB\x81a\x12\xD2V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x17\x1FW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x17\x01V[P\x93\x94\x93PPPPV[a\x173\x81\x86a\x14]V[c\xFF\xFF\xFF\xFF\x84\x16`@\x82\x01R`\xC0``\x82\x01R\x82Q`\xC0\x82\x01R` \x83\x01Q`\xE0\x82\x01R_`@\x84\x01Q\x80Qa\x01\0\x84\x01R` \x81\x01Qa\x01 \x84\x01RP``\x84\x01Q`\xA0a\x01@\x84\x01Ra\x17\x8Ca\x01`\x84\x01\x82a\x16\xEFV[\x91PPa\x17\x9C`\x80\x83\x01\x84a\x14]V[\x95\x94PPPPPV[_`\xC0\x82\x01a\x17\xB4\x83\x88a\x14]V[c\xFF\xFF\xFF\xFF\x86\x16`@\x84\x01R`\xC0``\x84\x01R\x80\x85Q\x80\x83R`\xE0\x85\x01\x91P`\xE0\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a\x185W\x86\x85\x03`\xDF\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a\x18\x1F\x90\x87\x01\x82a\x16\xEFV[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x17\xE4V[PPPP\x80\x91PPa\x17\x9C`\x80\x83\x01\x84a\x14]V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18\x87W__\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xA5W__\xFD[\x80`\x05\x1B6\x03\x82\x13\x15a\x15\"W__\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[_\x825`^\x19\x836\x03\x01\x81\x12a\x18\xF2W__\xFD[\x90\x91\x01\x92\x91PPV[\x81\x83R_`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a\x19\x12W__\xFD[\x82`\x05\x1B\x80\x83` \x87\x017\x93\x90\x93\x01` \x01\x93\x92PPPV[\x805\x82R` \x80\x82\x015\x90\x83\x01R_a\x19G`@\x83\x01\x83a\x18rV[```@\x86\x01Ra\x17\x9C``\x86\x01\x82\x84a\x18\xFBV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x17\x1FW\x81Qa\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x19nV[a\x19\xB3\x81\x85T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16` \x90\x91\x01RV[`\x80`@\x82\x01R_a\x01\xA0\x82\x01\x845a\x19\xCB\x81a\x12\xD2V[c\xFF\xFF\xFF\xFF\x16`\x80\x84\x01R` \x85\x015`\xA0\x84\x01R`@\x85\x015`\xC0\x84\x01R``\x85\x015`\xE0\x84\x01R`@`\x80\x86\x01a\x01\0\x85\x017`@`\xC0\x86\x01a\x01@\x85\x017a\x1A\x1Aa\x01\0\x86\x01\x86a\x18rV[a\x01 a\x01\x80\x86\x01R\x82\x81\x84Ra\x01\xC0\x86\x01\x90Pa\x01\xC0\x82`\x05\x1B\x87\x01\x01\x93P\x82_[\x83\x81\x10\x15a\x1A\xFCW\x87\x86\x03a\x01\xBF\x19\x01\x83Ra\x1AY\x82\x86a\x18\xDEV[\x805a\x1Ad\x81a\x12\xD2V[c\xFF\xFF\xFF\xFF\x16\x87R` \x81\x0156\x82\x90\x03`\x1E\x19\x01\x81\x12a\x1A\x83W__\xFD[\x81\x01` \x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\x9EW__\xFD[\x806\x03\x82\x13\x15a\x1A\xACW__\xFD[``` \x8A\x01Ra\x1A\xC1``\x8A\x01\x82\x84a\x18\xB6V[\x91PPa\x1A\xD1`@\x83\x01\x83a\x18\xDEV[\x91P\x87\x81\x03`@\x89\x01Ra\x1A\xE5\x81\x83a\x19+V[\x97PPP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a\x1A=V[PPPPP\x82\x81\x03``\x84\x01Ra\x1B\x13\x81\x85a\x19\\V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\x1B-W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x12\xCBW__\xFD[\x815a\x1BG\x81a\x13\xABV[\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x83U` \x84\x015a\x1Bs\x81a\x12\xD2V[`\x01`\x01`\xC0\x1B\x03\x19\x91\x90\x91\x16\x90\x91\x17`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPV[\x805a\x1B\xA3\x81a\x13\xABV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x015a\x1B\xBC\x81a\x12\xD2V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPPPV[`@\x81\x01a\x11k\x82\x84a\x1B\x98V[a\x1B\xFE\x81\x86T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16` \x90\x91\x01RV[c\xFF\xFF\xFF\xFF\x84\x16`@\x82\x81\x01\x91\x90\x91R`\xC0``\x80\x84\x01\x82\x90R\x855\x91\x84\x01\x91\x90\x91R` \x85\x015`\xE0\x84\x01R\x90\x84\x015a\x01\0\x83\x01R\x83\x015a\x01 \x82\x01R_a\x1CL`\x80\x85\x01\x85a\x18rV[`\xA0a\x01@\x85\x01Ra\x1Cca\x01`\x85\x01\x82\x84a\x18\xFBV[\x92PPPa\x17\x9C`\x80\x83\x01\x84a\x1B\x98V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\x96Wa\x1C\x96a\x18JV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\x96Wa\x1C\x96a\x18JV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xE6Wa\x1C\xE6a\x18JV[`@R\x91\x90PV[_`@\x82\x84\x03\x12\x15a\x1C\xFEW__\xFD[a\x1D\x06a\x1CtV[\x90P\x815a\x1D\x13\x81a\x13\xABV[\x81R` \x82\x015a\x1D#\x81a\x12\xD2V[` \x82\x01R\x92\x91PPV[____`\xC0\x85\x87\x03\x12\x15a\x1DAW__\xFD[a\x1DK\x86\x86a\x1C\xEEV[\x93Pa\x1DY`@\x86\x01a\x14\xBEV[\x92Pa\x1Dh\x86``\x87\x01a\x1C\xEEV[\x91P`\xA0\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x82W__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x1D\x92W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xABWa\x1D\xABa\x18JV[a\x1D\xBE`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1C\xBEV[\x81\x81R\x88` \x83\x85\x01\x01\x11\x15a\x1D\xD2W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92\x95\x91\x94P\x92PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1E\x0BWa\x1E\x0Ba\x18JV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x1E$W__\xFD[\x81Qa\x1E7a\x1E2\x82a\x1D\xF3V[a\x1C\xBEV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x1EXW__\xFD[` \x85\x01[\x83\x81\x10\x15a\x1EuW\x80Q\x83R` \x92\x83\x01\x92\x01a\x1E]V[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x1E\x8FW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xA4W__\xFD[\x82\x01\x80\x84\x03`\xA0\x81\x12\x15a\x1E\xB6W__\xFD[a\x1E\xBEa\x1C\x9CV[\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R`@`?\x19\x83\x01\x12\x15a\x1E\xDDW__\xFD[a\x1E\xE5a\x1CtV[`@\x84\x81\x01Q\x82R``\x85\x01Q` \x83\x01R\x82\x01R`\x80\x83\x01Q\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1F\x14W__\xFD[a\x1F \x86\x83\x85\x01a\x1E\x15V[``\x82\x01R\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x1F>W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FSW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1FcW__\xFD[\x80Qa\x1Fqa\x1E2\x82a\x1D\xF3V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a\x1F\x92W__\xFD[` \x84\x01[\x83\x81\x10\x15a $W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xB4W__\xFD[\x85\x01`@\x81\x8A\x03`\x1F\x19\x01\x12\x15a\x1F\xC9W__\xFD[a\x1F\xD1a\x1CtV[` \x82\x01Qa\x1F\xDF\x81a\x13\xABV[\x81R`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xF9W__\xFD[a \x08\x8B` \x83\x86\x01\x01a\x1E\x15V[` \x83\x01RP\x80\x85RPP` \x83\x01\x92P` \x81\x01\x90Pa\x1F\x97V[P\x96\x95PPPPPPV[_\x82a IWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\x11kWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x9A\x01+\x8A\r\xD1\xD7\x0E5g9I\xEE\xA8\xA5\xB8\xDE\x19\xCE\x04\xFE\x10\x97\x90\x84\x9F\x94 wg\x08\xBAdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106101bb575f3560e01c806364e1df84116100f3578063c252aa2211610093578063c5916a391161006e578063c5916a39146104bb578063eaaed9d5146104e0578063f2fde38b146104f3578063fd967f4714610506575f5ffd5b8063c252aa2214610474578063c3621f0a14610495578063c3be1e33146104a8575f5ffd5b80638da5cb5b116100ce5780638da5cb5b146104025780639ea9477814610413578063ad0f958214610426578063b8c143061461044d575f5ffd5b806364e1df841461039a5780636f728c50146103cf578063715018a6146103fa575f5ffd5b806330ef41b41161015e5780633ef6cd7a116101395780633ef6cd7a146103025780634624e6a314610329578063462828891461033d57806354fd4d5014610385575f5ffd5b806330ef41b41461029857806331a599d2146102ca578063383b9b70146102ef575f5ffd5b80631ab78d90116101995780631ab78d901461021e5780632370356c1461023157806323b7b5b21461024457806328522d791461026c575f5ffd5b80630371406e146101bf5780630f3f8edd146101d4578063193b79f3146101f6575b5f5ffd5b6101d26101cd3660046112b1565b61050f565b005b6101dc610523565b60405163ffffffff90911681526020015b60405180910390f35b6101dc6102043660046112e3565b63ffffffff9081165f908152606960205260409020541690565b6101d261022c36600461130e565b6105b6565b6101d261023f36600461137b565b6105ce565b6101dc6102523660046112e3565b63ffffffff9081165f908152606860205260409020541690565b60655462010000900463ffffffff165f908152606760205260409020545b6040519081526020016101ed565b6102ba6102a6366004611394565b5f908152606a602052604090205460ff1690565b60405190151581526020016101ed565b60655462010000900463ffffffff9081165f90815260686020526040902054166101dc565b6101d26102fd3660046113bf565b6105df565b61028a7f4491f5ee91595f938885ef73c9a1fa8a6d14ff9b9dab4aa24b8802bbb9bfc1cc81565b60655462010000900463ffffffff166101dc565b6040805180820182525f80825260209182015281518083019092526066546001600160a01b0381168352600160a01b900463ffffffff16908201526040516101ed919061147b565b61038d6107b5565b6040516101ed9190611489565b6102ba6103a83660046112e3565b63ffffffff165f908152606760209081526040808320548352606a90915290205460ff1690565b6103e26103dd3660046114cc565b6107e0565b6040516001600160a01b0390911681526020016101ed565b6101d261087f565b6033546001600160a01b03166103e2565b6101d2610421366004611529565b610892565b6103e27f000000000000000000000000000000000000000000000000000000000000000081565b6103e27f000000000000000000000000000000000000000000000000000000000000000081565b6065546104829061ffff1681565b60405161ffff90911681526020016101ed565b6101d26104a3366004611394565b610ac0565b61028a6104b63660046115c4565b610b35565b61028a6104c93660046112e3565b63ffffffff165f9081526067602052604090205490565b6101d26104ee366004611603565b610b9d565b6101d261050136600461166f565b610dcf565b61048261271081565b610517610e45565b61052081610e9f565b50565b604051635ddb9b5b60e01b81525f906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690635ddb9b5b906105729060669060040161168a565b602060405180830381865afa15801561058d573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906105b191906116b1565b905090565b6105be610e45565b6105c9838383610ee9565b505050565b6105d6610e45565b61052081610f6e565b5f54610100900460ff16158080156105fd57505f54600160ff909116105b806106165750303b15801561061657505f5460ff166001145b61067e5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff19166001179055801561069f575f805461ff0019166101001790555b6106a888610fd9565b6106b187610e9f565b6106ba86610f6e565b6106c5858585610ee9565b63ffffffff8086165f818152606760209081526040808320879055868352606a8252808320805460ff191660011790558383526068825280832080544390961663ffffffff19968716811790915583526069909152808220805490941683179093556065805462010000840265ffffffff000019909116179055915184927f010dcbe0d1e019c93357711f7bb6287d543b7ff7de74f29df3fb5ecceec8d36991a380156107ab575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050505050505050565b60606105b17f000000000000000000000000000000000000000000000000000000000000000061102a565b5f60028260028111156107f5576107f56116cc565b0361082157507f0000000000000000000000000000000000000000000000000000000000000000919050565b6001826002811115610835576108356116cc565b0361086157507f0000000000000000000000000000000000000000000000000000000000000000919050565b60405163fdea7c0960e01b815260040160405180910390fd5b919050565b610887610e45565b6108905f610fd9565b565b5f5f5f5f6108a08686611067565b5f8e8152606a60205260409020549397509195509350915060ff166108d85760405163504570e360e01b815260040160405180910390fd5b6108e1836107e0565b6001600160a01b0316635ddb9b5b856040518263ffffffff1660e01b815260040161090c919061147b565b602060405180830381865afa158015610927573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061094b91906116b1565b63ffffffff168b63ffffffff16116109765760405163207617df60e01b815260040160405180910390fd5b61099b8b8b8b8b8b8b8b60405161098e9291906116e0565b60405180910390206110ae565b60028360028111156109af576109af6116cc565b03610a40577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316636738c40b858d6109ee8561114f565b866040518563ffffffff1660e01b8152600401610a0e9493929190611729565b5f604051808303815f87803b158015610a25575f5ffd5b505af1158015610a37573d5f5f3e3d5ffd5b50505050610ab3565b6001836002811115610a5457610a546116cc565b03610861577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166356d482f5858d610a9385611171565b866040518563ffffffff1660e01b8152600401610a0e94939291906117a5565b5050505050505050505050565b610ac8610e45565b5f818152606a602052604090205460ff16610af65760405163504570e360e01b815260040160405180910390fd5b5f818152606a6020526040808220805460ff191690555182917f8bd43de1250f58fe6ec9a78671a8b78dba70f0018656d157a3aeaabec389df3491a250565b604080517f4491f5ee91595f938885ef73c9a1fa8a6d14ff9b9dab4aa24b8802bbb9bfc1cc602082015290810184905263ffffffff8084166060830152821660808201525f9060a0016040516020818303038152906040528051906020012090509392505050565b428263ffffffff161115610bc457604051635a119db560e11b815260040160405180910390fd5b60655463ffffffff62010000909104811690831611610bf65760405163037fa86b60e31b815260040160405180910390fd5b610c01838383610b35565b846020013514610c2457604051638b56642d60e01b815260040160405180910390fd5b6040805160018082528183019092525f91602080830190803683375050606554825192935061ffff16918391505f90610c5f57610c5f61185e565b61ffff90921660209283029190910190910152604051625f5e5d60e21b81525f906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063017d797490610cc4906066908a908790600401611990565b6020604051808303815f875af1158015610ce0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d049190611b1d565b905080610d2457604051633042041f60e21b815260040160405180910390fd5b6065805463ffffffff80871662010000810265ffffffff000019909316929092179092555f818152606860209081526040808320805495891663ffffffff1996871681179091558352606982528083208054909516841790945582825260678152838220899055888252606a9052828120805460ff19166001179055915187927f010dcbe0d1e019c93357711f7bb6287d543b7ff7de74f29df3fb5ecceec8d36991a3505050505050565b610dd7610e45565b6001600160a01b038116610e3c5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610675565b61052081610fd9565b6033546001600160a01b031633146108905760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610675565b806066610eac8282611b3c565b9050507f20100394950e66014c25009b45d12b675210a6e7a002044a0e3de6544e3c4b3781604051610ede9190611bcd565b60405180910390a150565b604051636738c40b60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690636738c40b90610f3c90606690879087908790600401611bdb565b5f604051808303815f87803b158015610f53575f5ffd5b505af1158015610f65573d5f5f3e3d5ffd5b50505050505050565b61271061ffff82161115610f95576040516307336f0360e11b815260040160405180910390fd5b6065805461ffff191661ffff83169081179091556040519081527ff5d1836df8fcd7c1e54047e94ac8773d2855395603e2ef9ba5f5f16905f2259290602001610ede565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b60605f61103683611187565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b604080518082019091525f8082526020820152604080518082019091525f8082526020820181905290606061109e85870187611d2e565b9299919850965090945092505050565b63ffffffff86165f9081526067602052604090205485146110e25760405163639d09b560e11b815260040160405180910390fd5b61112a83838080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508992508591505063ffffffff88166111ae565b6111475760405163afa42ca760e01b815260040160405180910390fd5b505050505050565b61115761125c565b8180602001905181019061116b9190611e7f565b92915050565b60608180602001905181019061116b9190611f2e565b5f60ff8216601f81111561116b57604051632cd44ac360e21b815260040160405180910390fd5b5f836111bb8685856111c5565b1495945050505050565b5f602084516111d4919061202f565b156111f2576040516313717da960e21b815260040160405180910390fd5b8260205b855181116112535761120960028561202f565b5f0361122a57815f528086015160205260405f209150600284049350611241565b808601515f528160205260405f2091506002840493505b61124c60208261204e565b90506111f6565b50949350505050565b60405180608001604052805f81526020015f815260200161128e60405180604001604052805f81526020015f81525090565b8152602001606081525090565b5f604082840312156112ab575f5ffd5b50919050565b5f604082840312156112c1575f5ffd5b6112cb838361129b565b9392505050565b63ffffffff81168114610520575f5ffd5b5f602082840312156112f3575f5ffd5b81356112cb816112d2565b5f60a082840312156112ab575f5ffd5b5f5f5f60808486031215611320575f5ffd5b833561132b816112d2565b925060208401356001600160401b03811115611345575f5ffd5b611351868287016112fe565b925050611361856040860161129b565b90509250925092565b803561ffff8116811461087a575f5ffd5b5f6020828403121561138b575f5ffd5b6112cb8261136a565b5f602082840312156113a4575f5ffd5b5035919050565b6001600160a01b0381168114610520575f5ffd5b5f5f5f5f5f5f5f610120888a0312156113d6575f5ffd5b87356113e1816113ab565b96506113f08960208a0161129b565b95506113fe6060890161136a565b9450608088013561140e816112d2565b935060a08801356001600160401b03811115611428575f5ffd5b6114348a828b016112fe565b9350506114448960c08a0161129b565b9699959850939692959194919350506101009091013590565b80516001600160a01b0316825260209081015163ffffffff16910152565b6040810161116b828461145d565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80356003811061087a575f5ffd5b5f602082840312156114dc575f5ffd5b6112cb826114be565b5f5f83601f8401126114f5575f5ffd5b5081356001600160401b0381111561150b575f5ffd5b602083019150836020828501011115611522575f5ffd5b9250929050565b5f5f5f5f5f5f5f60a0888a03121561153f575f5ffd5b873561154a816112d2565b9650602088013595506040880135611561816112d2565b945060608801356001600160401b0381111561157b575f5ffd5b6115878a828b016114e5565b90955093505060808801356001600160401b038111156115a5575f5ffd5b6115b18a828b016114e5565b989b979a50959850939692959293505050565b5f5f5f606084860312156115d6575f5ffd5b8335925060208401356115e8816112d2565b915060408401356115f8816112d2565b809150509250925092565b5f5f5f5f60808587031215611616575f5ffd5b84356001600160401b0381111561162b575f5ffd5b8501610120818803121561163d575f5ffd5b9350602085013592506040850135611654816112d2565b91506060850135611664816112d2565b939692955090935050565b5f6020828403121561167f575f5ffd5b81356112cb816113ab565b6040810161116b8284546001600160a01b038116825260a01c63ffffffff16602090910152565b5f602082840312156116c1575f5ffd5b81516112cb816112d2565b634e487b7160e01b5f52602160045260245ffd5b818382375f9101908152919050565b5f8151808452602084019350602083015f5b8281101561171f578151865260209586019590910190600101611701565b5093949350505050565b611733818661145d565b63ffffffff8416604082015260c06060820152825160c0820152602083015160e08201525f60408401518051610100840152602081015161012084015250606084015160a061014084015261178c6101608401826116ef565b91505061179c608083018461145d565b95945050505050565b5f60c082016117b4838861145d565b63ffffffff8616604084015260c0606084015280855180835260e08501915060e08160051b8601019250602087015f5b828110156118355786850360df19018452815180516001600160a01b0316865260209081015160409187018290529061181f908701826116ef565b95505060209384019391909101906001016117e4565b505050508091505061179c608083018461145d565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b5f5f8335601e19843603018112611887575f5ffd5b83016020810192503590506001600160401b038111156118a5575f5ffd5b8060051b3603821315611522575f5ffd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b5f8235605e198336030181126118f2575f5ffd5b90910192915050565b8183525f6001600160fb1b03831115611912575f5ffd5b8260051b80836020870137939093016020019392505050565b80358252602080820135908301525f6119476040830183611872565b6060604086015261179c6060860182846118fb565b5f8151808452602084019350602083015f5b8281101561171f57815161ffff1686526020958601959091019060010161196e565b6119b38185546001600160a01b038116825260a01c63ffffffff16602090910152565b608060408201525f6101a0820184356119cb816112d2565b63ffffffff166080840152602085013560a0840152604085013560c0840152606085013560e0840152604060808601610100850137604060c08601610140850137611a1a610100860186611872565b610120610180860152828184526101c0860190506101c08260051b8701019350825f5b83811015611afc578786036101bf19018352611a5982866118de565b8035611a64816112d2565b63ffffffff168752602081013536829003601e19018112611a83575f5ffd5b81016020810190356001600160401b03811115611a9e575f5ffd5b803603821315611aac575f5ffd5b606060208a0152611ac160608a0182846118b6565b915050611ad160408301836118de565b91508781036040890152611ae5818361192b565b975050506020928301929190910190600101611a3d565b50505050508281036060840152611b13818561195c565b9695505050505050565b5f60208284031215611b2d575f5ffd5b815180151581146112cb575f5ffd5b8135611b47816113ab565b81546001600160a01b031981166001600160a01b039290921691821783556020840135611b73816112d2565b6001600160c01b03199190911690911760a09190911b63ffffffff60a01b1617905550565b8035611ba3816113ab565b6001600160a01b031682526020810135611bbc816112d2565b63ffffffff81166020840152505050565b6040810161116b8284611b98565b611bfe8186546001600160a01b038116825260a01c63ffffffff16602090910152565b63ffffffff841660408281019190915260c06060808401829052853591840191909152602085013560e0840152908401356101008301528301356101208201525f611c4c6080850185611872565b60a0610140850152611c63610160850182846118fb565b9250505061179c6080830184611b98565b604080519081016001600160401b0381118282101715611c9657611c9661184a565b60405290565b604051608081016001600160401b0381118282101715611c9657611c9661184a565b604051601f8201601f191681016001600160401b0381118282101715611ce657611ce661184a565b604052919050565b5f60408284031215611cfe575f5ffd5b611d06611c74565b90508135611d13816113ab565b81526020820135611d23816112d2565b602082015292915050565b5f5f5f5f60c08587031215611d41575f5ffd5b611d4b8686611cee565b9350611d59604086016114be565b9250611d688660608701611cee565b915060a08501356001600160401b03811115611d82575f5ffd5b8501601f81018713611d92575f5ffd5b80356001600160401b03811115611dab57611dab61184a565b611dbe601f8201601f1916602001611cbe565b818152886020838501011115611dd2575f5ffd5b816020840160208301375f6020838301015280935050505092959194509250565b5f6001600160401b03821115611e0b57611e0b61184a565b5060051b60200190565b5f82601f830112611e24575f5ffd5b8151611e37611e3282611df3565b611cbe565b8082825260208201915060208360051b860101925085831115611e58575f5ffd5b602085015b83811015611e75578051835260209283019201611e5d565b5095945050505050565b5f60208284031215611e8f575f5ffd5b81516001600160401b03811115611ea4575f5ffd5b820180840360a0811215611eb6575f5ffd5b611ebe611c9c565b82518152602080840151908201526040603f1983011215611edd575f5ffd5b611ee5611c74565b604084810151825260608501516020830152820152608083015191506001600160401b03821115611f14575f5ffd5b611f2086838501611e15565b606082015295945050505050565b5f60208284031215611f3e575f5ffd5b81516001600160401b03811115611f53575f5ffd5b8201601f81018413611f63575f5ffd5b8051611f71611e3282611df3565b8082825260208201915060208360051b850101925086831115611f92575f5ffd5b602084015b838110156120245780516001600160401b03811115611fb4575f5ffd5b85016040818a03601f19011215611fc9575f5ffd5b611fd1611c74565b6020820151611fdf816113ab565b815260408201516001600160401b03811115611ff9575f5ffd5b6120088b602083860101611e15565b6020830152508085525050602083019250602081019050611f97565b509695505050505050565b5f8261204957634e487b7160e01b5f52601260045260245ffd5b500690565b8082018082111561116b57634e487b7160e01b5f52601160045260245ffdfea26469706673582212209a012b8a0dd1d70e35673949eea8a5b8de19ce04fe109790849f9420776708ba64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xBBW_5`\xE0\x1C\x80cd\xE1\xDF\x84\x11a\0\xF3W\x80c\xC2R\xAA\"\x11a\0\x93W\x80c\xC5\x91j9\x11a\0nW\x80c\xC5\x91j9\x14a\x04\xBBW\x80c\xEA\xAE\xD9\xD5\x14a\x04\xE0W\x80c\xF2\xFD\xE3\x8B\x14a\x04\xF3W\x80c\xFD\x96\x7FG\x14a\x05\x06W__\xFD[\x80c\xC2R\xAA\"\x14a\x04tW\x80c\xC3b\x1F\n\x14a\x04\x95W\x80c\xC3\xBE\x1E3\x14a\x04\xA8W__\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xCEW\x80c\x8D\xA5\xCB[\x14a\x04\x02W\x80c\x9E\xA9Gx\x14a\x04\x13W\x80c\xAD\x0F\x95\x82\x14a\x04&W\x80c\xB8\xC1C\x06\x14a\x04MW__\xFD[\x80cd\xE1\xDF\x84\x14a\x03\x9AW\x80cor\x8CP\x14a\x03\xCFW\x80cqP\x18\xA6\x14a\x03\xFAW__\xFD[\x80c0\xEFA\xB4\x11a\x01^W\x80c>\xF6\xCDz\x11a\x019W\x80c>\xF6\xCDz\x14a\x03\x02W\x80cF$\xE6\xA3\x14a\x03)W\x80cF((\x89\x14a\x03=W\x80cT\xFDMP\x14a\x03\x85W__\xFD[\x80c0\xEFA\xB4\x14a\x02\x98W\x80c1\xA5\x99\xD2\x14a\x02\xCAW\x80c8;\x9Bp\x14a\x02\xEFW__\xFD[\x80c\x1A\xB7\x8D\x90\x11a\x01\x99W\x80c\x1A\xB7\x8D\x90\x14a\x02\x1EW\x80c#p5l\x14a\x021W\x80c#\xB7\xB5\xB2\x14a\x02DW\x80c(R-y\x14a\x02lW__\xFD[\x80c\x03q@n\x14a\x01\xBFW\x80c\x0F?\x8E\xDD\x14a\x01\xD4W\x80c\x19;y\xF3\x14a\x01\xF6W[__\xFD[a\x01\xD2a\x01\xCD6`\x04a\x12\xB1V[a\x05\x0FV[\0[a\x01\xDCa\x05#V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xDCa\x02\x046`\x04a\x12\xE3V[c\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`i` R`@\x90 T\x16\x90V[a\x01\xD2a\x02,6`\x04a\x13\x0EV[a\x05\xB6V[a\x01\xD2a\x02?6`\x04a\x13{V[a\x05\xCEV[a\x01\xDCa\x02R6`\x04a\x12\xE3V[c\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`h` R`@\x90 T\x16\x90V[`eTb\x01\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16_\x90\x81R`g` R`@\x90 T[`@Q\x90\x81R` \x01a\x01\xEDV[a\x02\xBAa\x02\xA66`\x04a\x13\x94V[_\x90\x81R`j` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xEDV[`eTb\x01\0\0\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16_\x90\x81R`h` R`@\x90 T\x16a\x01\xDCV[a\x01\xD2a\x02\xFD6`\x04a\x13\xBFV[a\x05\xDFV[a\x02\x8A\x7FD\x91\xF5\xEE\x91Y_\x93\x88\x85\xEFs\xC9\xA1\xFA\x8Am\x14\xFF\x9B\x9D\xABJ\xA2K\x88\x02\xBB\xB9\xBF\xC1\xCC\x81V[`eTb\x01\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16a\x01\xDCV[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`fT`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R`@Qa\x01\xED\x91\x90a\x14{V[a\x03\x8Da\x07\xB5V[`@Qa\x01\xED\x91\x90a\x14\x89V[a\x02\xBAa\x03\xA86`\x04a\x12\xE3V[c\xFF\xFF\xFF\xFF\x16_\x90\x81R`g` \x90\x81R`@\x80\x83 T\x83R`j\x90\x91R\x90 T`\xFF\x16\x90V[a\x03\xE2a\x03\xDD6`\x04a\x14\xCCV[a\x07\xE0V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xEDV[a\x01\xD2a\x08\x7FV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xE2V[a\x01\xD2a\x04!6`\x04a\x15)V[a\x08\x92V[a\x03\xE2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xE2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`eTa\x04\x82\x90a\xFF\xFF\x16\x81V[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xEDV[a\x01\xD2a\x04\xA36`\x04a\x13\x94V[a\n\xC0V[a\x02\x8Aa\x04\xB66`\x04a\x15\xC4V[a\x0B5V[a\x02\x8Aa\x04\xC96`\x04a\x12\xE3V[c\xFF\xFF\xFF\xFF\x16_\x90\x81R`g` R`@\x90 T\x90V[a\x01\xD2a\x04\xEE6`\x04a\x16\x03V[a\x0B\x9DV[a\x01\xD2a\x05\x016`\x04a\x16oV[a\r\xCFV[a\x04\x82a'\x10\x81V[a\x05\x17a\x0EEV[a\x05 \x81a\x0E\x9FV[PV[`@Qc]\xDB\x9B[`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c]\xDB\x9B[\x90a\x05r\x90`f\x90`\x04\x01a\x16\x8AV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x8DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xB1\x91\x90a\x16\xB1V[\x90P\x90V[a\x05\xBEa\x0EEV[a\x05\xC9\x83\x83\x83a\x0E\xE9V[PPPV[a\x05\xD6a\x0EEV[a\x05 \x81a\x0FnV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x05\xFDWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x06\x16WP0;\x15\x80\x15a\x06\x16WP_T`\xFF\x16`\x01\x14[a\x06~W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x06\x9FW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x06\xA8\x88a\x0F\xD9V[a\x06\xB1\x87a\x0E\x9FV[a\x06\xBA\x86a\x0FnV[a\x06\xC5\x85\x85\x85a\x0E\xE9V[c\xFF\xFF\xFF\xFF\x80\x86\x16_\x81\x81R`g` \x90\x81R`@\x80\x83 \x87\x90U\x86\x83R`j\x82R\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U\x83\x83R`h\x82R\x80\x83 \x80TC\x90\x96\x16c\xFF\xFF\xFF\xFF\x19\x96\x87\x16\x81\x17\x90\x91U\x83R`i\x90\x91R\x80\x82 \x80T\x90\x94\x16\x83\x17\x90\x93U`e\x80Tb\x01\0\0\x84\x02e\xFF\xFF\xFF\xFF\0\0\x19\x90\x91\x16\x17\x90U\x91Q\x84\x92\x7F\x01\r\xCB\xE0\xD1\xE0\x19\xC93Wq\x1F{\xB6(}T;\x7F\xF7\xDEt\xF2\x9D\xF3\xFB^\xCC\xEE\xC8\xD3i\x91\xA3\x80\x15a\x07\xABW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPV[``a\x05\xB1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10*V[_`\x02\x82`\x02\x81\x11\x15a\x07\xF5Wa\x07\xF5a\x16\xCCV[\x03a\x08!WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[`\x01\x82`\x02\x81\x11\x15a\x085Wa\x085a\x16\xCCV[\x03a\x08aWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[`@Qc\xFD\xEA|\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[a\x08\x87a\x0EEV[a\x08\x90_a\x0F\xD9V[V[____a\x08\xA0\x86\x86a\x10gV[_\x8E\x81R`j` R`@\x90 T\x93\x97P\x91\x95P\x93P\x91P`\xFF\x16a\x08\xD8W`@QcPEp\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xE1\x83a\x07\xE0V[`\x01`\x01`\xA0\x1B\x03\x16c]\xDB\x9B[\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x0C\x91\x90a\x14{V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t'W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tK\x91\x90a\x16\xB1V[c\xFF\xFF\xFF\xFF\x16\x8Bc\xFF\xFF\xFF\xFF\x16\x11a\tvW`@Qc v\x17\xDF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\x9B\x8B\x8B\x8B\x8B\x8B\x8B\x8B`@Qa\t\x8E\x92\x91\x90a\x16\xE0V[`@Q\x80\x91\x03\x90 a\x10\xAEV[`\x02\x83`\x02\x81\x11\x15a\t\xAFWa\t\xAFa\x16\xCCV[\x03a\n@W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cg8\xC4\x0B\x85\x8Da\t\xEE\x85a\x11OV[\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x0E\x94\x93\x92\x91\x90a\x17)V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n%W__\xFD[PZ\xF1\x15\x80\x15a\n7W=__>=_\xFD[PPPPa\n\xB3V[`\x01\x83`\x02\x81\x11\x15a\nTWa\nTa\x16\xCCV[\x03a\x08aW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cV\xD4\x82\xF5\x85\x8Da\n\x93\x85a\x11qV[\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x0E\x94\x93\x92\x91\x90a\x17\xA5V[PPPPPPPPPPPV[a\n\xC8a\x0EEV[_\x81\x81R`j` R`@\x90 T`\xFF\x16a\n\xF6W`@QcPEp\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`j` R`@\x80\x82 \x80T`\xFF\x19\x16\x90UQ\x82\x91\x7F\x8B\xD4=\xE1%\x0FX\xFEn\xC9\xA7\x86q\xA8\xB7\x8D\xBAp\xF0\x01\x86V\xD1W\xA3\xAE\xAA\xBE\xC3\x89\xDF4\x91\xA2PV[`@\x80Q\x7FD\x91\xF5\xEE\x91Y_\x93\x88\x85\xEFs\xC9\xA1\xFA\x8Am\x14\xFF\x9B\x9D\xABJ\xA2K\x88\x02\xBB\xB9\xBF\xC1\xCC` \x82\x01R\x90\x81\x01\x84\x90Rc\xFF\xFF\xFF\xFF\x80\x84\x16``\x83\x01R\x82\x16`\x80\x82\x01R_\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x93\x92PPPV[B\x82c\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\xC4W`@QcZ\x11\x9D\xB5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`eTc\xFF\xFF\xFF\xFFb\x01\0\0\x90\x91\x04\x81\x16\x90\x83\x16\x11a\x0B\xF6W`@Qc\x03\x7F\xA8k`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\x01\x83\x83\x83a\x0B5V[\x84` \x015\x14a\x0C$W`@Qc\x8BVd-`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R_\x91` \x80\x83\x01\x90\x806\x837PP`eT\x82Q\x92\x93Pa\xFF\xFF\x16\x91\x83\x91P_\x90a\x0C_Wa\x0C_a\x18^V[a\xFF\xFF\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@Qb_^]`\xE2\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x01}yt\x90a\x0C\xC4\x90`f\x90\x8A\x90\x87\x90`\x04\x01a\x19\x90V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0C\xE0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x04\x91\x90a\x1B\x1DV[\x90P\x80a\r$W`@Qc0B\x04\x1F`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80Tc\xFF\xFF\xFF\xFF\x80\x87\x16b\x01\0\0\x81\x02e\xFF\xFF\xFF\xFF\0\0\x19\x90\x93\x16\x92\x90\x92\x17\x90\x92U_\x81\x81R`h` \x90\x81R`@\x80\x83 \x80T\x95\x89\x16c\xFF\xFF\xFF\xFF\x19\x96\x87\x16\x81\x17\x90\x91U\x83R`i\x82R\x80\x83 \x80T\x90\x95\x16\x84\x17\x90\x94U\x82\x82R`g\x81R\x83\x82 \x89\x90U\x88\x82R`j\x90R\x82\x81 \x80T`\xFF\x19\x16`\x01\x17\x90U\x91Q\x87\x92\x7F\x01\r\xCB\xE0\xD1\xE0\x19\xC93Wq\x1F{\xB6(}T;\x7F\xF7\xDEt\xF2\x9D\xF3\xFB^\xCC\xEE\xC8\xD3i\x91\xA3PPPPPPV[a\r\xD7a\x0EEV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06uV[a\x05 \x81a\x0F\xD9V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x08\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06uV[\x80`fa\x0E\xAC\x82\x82a\x1B<V[\x90PP\x7F \x10\x03\x94\x95\x0Ef\x01L%\0\x9BE\xD1+gR\x10\xA6\xE7\xA0\x02\x04J\x0E=\xE6TN<K7\x81`@Qa\x0E\xDE\x91\x90a\x1B\xCDV[`@Q\x80\x91\x03\x90\xA1PV[`@Qcg8\xC4\x0B`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cg8\xC4\x0B\x90a\x0F<\x90`f\x90\x87\x90\x87\x90\x87\x90`\x04\x01a\x1B\xDBV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0FSW__\xFD[PZ\xF1\x15\x80\x15a\x0FeW=__>=_\xFD[PPPPPPPV[a'\x10a\xFF\xFF\x82\x16\x11\x15a\x0F\x95W`@Qc\x073o\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80Ta\xFF\xFF\x19\x16a\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xF5\xD1\x83m\xF8\xFC\xD7\xC1\xE5@G\xE9J\xC8w=(U9V\x03\xE2\xEF\x9B\xA5\xF5\xF1i\x05\xF2%\x92\x90` \x01a\x0E\xDEV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[``_a\x106\x83a\x11\x87V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01\x81\x90R\x90``a\x10\x9E\x85\x87\x01\x87a\x1D.V[\x92\x99\x91\x98P\x96P\x90\x94P\x92PPPV[c\xFF\xFF\xFF\xFF\x86\x16_\x90\x81R`g` R`@\x90 T\x85\x14a\x10\xE2W`@Qcc\x9D\t\xB5`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11*\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x89\x92P\x85\x91PPc\xFF\xFF\xFF\xFF\x88\x16a\x11\xAEV[a\x11GW`@Qc\xAF\xA4,\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[a\x11Wa\x12\\V[\x81\x80` \x01\x90Q\x81\x01\x90a\x11k\x91\x90a\x1E\x7FV[\x92\x91PPV[``\x81\x80` \x01\x90Q\x81\x01\x90a\x11k\x91\x90a\x1F.V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x11kW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83a\x11\xBB\x86\x85\x85a\x11\xC5V[\x14\x95\x94PPPPPV[_` \x84Qa\x11\xD4\x91\x90a /V[\x15a\x11\xF2W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82` [\x85Q\x81\x11a\x12SWa\x12\t`\x02\x85a /V[_\x03a\x12*W\x81_R\x80\x86\x01Q` R`@_ \x91P`\x02\x84\x04\x93Pa\x12AV[\x80\x86\x01Q_R\x81` R`@_ \x91P`\x02\x84\x04\x93P[a\x12L` \x82a NV[\x90Pa\x11\xF6V[P\x94\x93PPPPV[`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01a\x12\x8E`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01``\x81RP\x90V[_`@\x82\x84\x03\x12\x15a\x12\xABW__\xFD[P\x91\x90PV[_`@\x82\x84\x03\x12\x15a\x12\xC1W__\xFD[a\x12\xCB\x83\x83a\x12\x9BV[\x93\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05 W__\xFD[_` \x82\x84\x03\x12\x15a\x12\xF3W__\xFD[\x815a\x12\xCB\x81a\x12\xD2V[_`\xA0\x82\x84\x03\x12\x15a\x12\xABW__\xFD[___`\x80\x84\x86\x03\x12\x15a\x13 W__\xFD[\x835a\x13+\x81a\x12\xD2V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13EW__\xFD[a\x13Q\x86\x82\x87\x01a\x12\xFEV[\x92PPa\x13a\x85`@\x86\x01a\x12\x9BV[\x90P\x92P\x92P\x92V[\x805a\xFF\xFF\x81\x16\x81\x14a\x08zW__\xFD[_` \x82\x84\x03\x12\x15a\x13\x8BW__\xFD[a\x12\xCB\x82a\x13jV[_` \x82\x84\x03\x12\x15a\x13\xA4W__\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05 W__\xFD[_______a\x01 \x88\x8A\x03\x12\x15a\x13\xD6W__\xFD[\x875a\x13\xE1\x81a\x13\xABV[\x96Pa\x13\xF0\x89` \x8A\x01a\x12\x9BV[\x95Pa\x13\xFE``\x89\x01a\x13jV[\x94P`\x80\x88\x015a\x14\x0E\x81a\x12\xD2V[\x93P`\xA0\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14(W__\xFD[a\x144\x8A\x82\x8B\x01a\x12\xFEV[\x93PPa\x14D\x89`\xC0\x8A\x01a\x12\x9BV[\x96\x99\x95\x98P\x93\x96\x92\x95\x91\x94\x91\x93PPa\x01\0\x90\x91\x015\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`@\x81\x01a\x11k\x82\x84a\x14]V[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x805`\x03\x81\x10a\x08zW__\xFD[_` \x82\x84\x03\x12\x15a\x14\xDCW__\xFD[a\x12\xCB\x82a\x14\xBEV[__\x83`\x1F\x84\x01\x12a\x14\xF5W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\x0BW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x15\"W__\xFD[\x92P\x92\x90PV[_______`\xA0\x88\x8A\x03\x12\x15a\x15?W__\xFD[\x875a\x15J\x81a\x12\xD2V[\x96P` \x88\x015\x95P`@\x88\x015a\x15a\x81a\x12\xD2V[\x94P``\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15{W__\xFD[a\x15\x87\x8A\x82\x8B\x01a\x14\xE5V[\x90\x95P\x93PP`\x80\x88\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xA5W__\xFD[a\x15\xB1\x8A\x82\x8B\x01a\x14\xE5V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[___``\x84\x86\x03\x12\x15a\x15\xD6W__\xFD[\x835\x92P` \x84\x015a\x15\xE8\x81a\x12\xD2V[\x91P`@\x84\x015a\x15\xF8\x81a\x12\xD2V[\x80\x91PP\x92P\x92P\x92V[____`\x80\x85\x87\x03\x12\x15a\x16\x16W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16+W__\xFD[\x85\x01a\x01 \x81\x88\x03\x12\x15a\x16=W__\xFD[\x93P` \x85\x015\x92P`@\x85\x015a\x16T\x81a\x12\xD2V[\x91P``\x85\x015a\x16d\x81a\x12\xD2V[\x93\x96\x92\x95P\x90\x93PPV[_` \x82\x84\x03\x12\x15a\x16\x7FW__\xFD[\x815a\x12\xCB\x81a\x13\xABV[`@\x81\x01a\x11k\x82\x84T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16` \x90\x91\x01RV[_` \x82\x84\x03\x12\x15a\x16\xC1W__\xFD[\x81Qa\x12\xCB\x81a\x12\xD2V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x17\x1FW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x17\x01V[P\x93\x94\x93PPPPV[a\x173\x81\x86a\x14]V[c\xFF\xFF\xFF\xFF\x84\x16`@\x82\x01R`\xC0``\x82\x01R\x82Q`\xC0\x82\x01R` \x83\x01Q`\xE0\x82\x01R_`@\x84\x01Q\x80Qa\x01\0\x84\x01R` \x81\x01Qa\x01 \x84\x01RP``\x84\x01Q`\xA0a\x01@\x84\x01Ra\x17\x8Ca\x01`\x84\x01\x82a\x16\xEFV[\x91PPa\x17\x9C`\x80\x83\x01\x84a\x14]V[\x95\x94PPPPPV[_`\xC0\x82\x01a\x17\xB4\x83\x88a\x14]V[c\xFF\xFF\xFF\xFF\x86\x16`@\x84\x01R`\xC0``\x84\x01R\x80\x85Q\x80\x83R`\xE0\x85\x01\x91P`\xE0\x81`\x05\x1B\x86\x01\x01\x92P` \x87\x01_[\x82\x81\x10\x15a\x185W\x86\x85\x03`\xDF\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a\x18\x1F\x90\x87\x01\x82a\x16\xEFV[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x17\xE4V[PPPP\x80\x91PPa\x17\x9C`\x80\x83\x01\x84a\x14]V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a\x18\x87W__\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xA5W__\xFD[\x80`\x05\x1B6\x03\x82\x13\x15a\x15\"W__\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[_\x825`^\x19\x836\x03\x01\x81\x12a\x18\xF2W__\xFD[\x90\x91\x01\x92\x91PPV[\x81\x83R_`\x01`\x01`\xFB\x1B\x03\x83\x11\x15a\x19\x12W__\xFD[\x82`\x05\x1B\x80\x83` \x87\x017\x93\x90\x93\x01` \x01\x93\x92PPPV[\x805\x82R` \x80\x82\x015\x90\x83\x01R_a\x19G`@\x83\x01\x83a\x18rV[```@\x86\x01Ra\x17\x9C``\x86\x01\x82\x84a\x18\xFBV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a\x17\x1FW\x81Qa\xFF\xFF\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a\x19nV[a\x19\xB3\x81\x85T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16` \x90\x91\x01RV[`\x80`@\x82\x01R_a\x01\xA0\x82\x01\x845a\x19\xCB\x81a\x12\xD2V[c\xFF\xFF\xFF\xFF\x16`\x80\x84\x01R` \x85\x015`\xA0\x84\x01R`@\x85\x015`\xC0\x84\x01R``\x85\x015`\xE0\x84\x01R`@`\x80\x86\x01a\x01\0\x85\x017`@`\xC0\x86\x01a\x01@\x85\x017a\x1A\x1Aa\x01\0\x86\x01\x86a\x18rV[a\x01 a\x01\x80\x86\x01R\x82\x81\x84Ra\x01\xC0\x86\x01\x90Pa\x01\xC0\x82`\x05\x1B\x87\x01\x01\x93P\x82_[\x83\x81\x10\x15a\x1A\xFCW\x87\x86\x03a\x01\xBF\x19\x01\x83Ra\x1AY\x82\x86a\x18\xDEV[\x805a\x1Ad\x81a\x12\xD2V[c\xFF\xFF\xFF\xFF\x16\x87R` \x81\x0156\x82\x90\x03`\x1E\x19\x01\x81\x12a\x1A\x83W__\xFD[\x81\x01` \x81\x01\x905`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\x9EW__\xFD[\x806\x03\x82\x13\x15a\x1A\xACW__\xFD[``` \x8A\x01Ra\x1A\xC1``\x8A\x01\x82\x84a\x18\xB6V[\x91PPa\x1A\xD1`@\x83\x01\x83a\x18\xDEV[\x91P\x87\x81\x03`@\x89\x01Ra\x1A\xE5\x81\x83a\x19+V[\x97PPP` \x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a\x1A=V[PPPPP\x82\x81\x03``\x84\x01Ra\x1B\x13\x81\x85a\x19\\V[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\x1B-W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x12\xCBW__\xFD[\x815a\x1BG\x81a\x13\xABV[\x81T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x82\x17\x83U` \x84\x015a\x1Bs\x81a\x12\xD2V[`\x01`\x01`\xC0\x1B\x03\x19\x91\x90\x91\x16\x90\x91\x17`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x90UPV[\x805a\x1B\xA3\x81a\x13\xABV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x015a\x1B\xBC\x81a\x12\xD2V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPPPV[`@\x81\x01a\x11k\x82\x84a\x1B\x98V[a\x1B\xFE\x81\x86T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16` \x90\x91\x01RV[c\xFF\xFF\xFF\xFF\x84\x16`@\x82\x81\x01\x91\x90\x91R`\xC0``\x80\x84\x01\x82\x90R\x855\x91\x84\x01\x91\x90\x91R` \x85\x015`\xE0\x84\x01R\x90\x84\x015a\x01\0\x83\x01R\x83\x015a\x01 \x82\x01R_a\x1CL`\x80\x85\x01\x85a\x18rV[`\xA0a\x01@\x85\x01Ra\x1Cca\x01`\x85\x01\x82\x84a\x18\xFBV[\x92PPPa\x17\x9C`\x80\x83\x01\x84a\x1B\x98V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\x96Wa\x1C\x96a\x18JV[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\x96Wa\x1C\x96a\x18JV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C\xE6Wa\x1C\xE6a\x18JV[`@R\x91\x90PV[_`@\x82\x84\x03\x12\x15a\x1C\xFEW__\xFD[a\x1D\x06a\x1CtV[\x90P\x815a\x1D\x13\x81a\x13\xABV[\x81R` \x82\x015a\x1D#\x81a\x12\xD2V[` \x82\x01R\x92\x91PPV[____`\xC0\x85\x87\x03\x12\x15a\x1DAW__\xFD[a\x1DK\x86\x86a\x1C\xEEV[\x93Pa\x1DY`@\x86\x01a\x14\xBEV[\x92Pa\x1Dh\x86``\x87\x01a\x1C\xEEV[\x91P`\xA0\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x82W__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\x1D\x92W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xABWa\x1D\xABa\x18JV[a\x1D\xBE`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1C\xBEV[\x81\x81R\x88` \x83\x85\x01\x01\x11\x15a\x1D\xD2W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92\x95\x91\x94P\x92PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1E\x0BWa\x1E\x0Ba\x18JV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x1E$W__\xFD[\x81Qa\x1E7a\x1E2\x82a\x1D\xF3V[a\x1C\xBEV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x1EXW__\xFD[` \x85\x01[\x83\x81\x10\x15a\x1EuW\x80Q\x83R` \x92\x83\x01\x92\x01a\x1E]V[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x1E\x8FW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xA4W__\xFD[\x82\x01\x80\x84\x03`\xA0\x81\x12\x15a\x1E\xB6W__\xFD[a\x1E\xBEa\x1C\x9CV[\x82Q\x81R` \x80\x84\x01Q\x90\x82\x01R`@`?\x19\x83\x01\x12\x15a\x1E\xDDW__\xFD[a\x1E\xE5a\x1CtV[`@\x84\x81\x01Q\x82R``\x85\x01Q` \x83\x01R\x82\x01R`\x80\x83\x01Q\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1F\x14W__\xFD[a\x1F \x86\x83\x85\x01a\x1E\x15V[``\x82\x01R\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x1F>W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FSW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1FcW__\xFD[\x80Qa\x1Fqa\x1E2\x82a\x1D\xF3V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a\x1F\x92W__\xFD[` \x84\x01[\x83\x81\x10\x15a $W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xB4W__\xFD[\x85\x01`@\x81\x8A\x03`\x1F\x19\x01\x12\x15a\x1F\xC9W__\xFD[a\x1F\xD1a\x1CtV[` \x82\x01Qa\x1F\xDF\x81a\x13\xABV[\x81R`@\x82\x01Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xF9W__\xFD[a \x08\x8B` \x83\x86\x01\x01a\x1E\x15V[` \x83\x01RP\x80\x85RPP` \x83\x01\x92P` \x81\x01\x90Pa\x1F\x97V[P\x96\x95PPPPPPV[_\x82a IWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[\x80\x82\x01\x80\x82\x11\x15a\x11kWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x9A\x01+\x8A\r\xD1\xD7\x0E5g9I\xEE\xA8\xA5\xB8\xDE\x19\xCE\x04\xFE\x10\x97\x90\x84\x9F\x94 wg\x08\xBAdsolcC\0\x08\x1B\x003",
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
    /**Event with signature `GlobalRootConfirmerSetUpdated((address,uint32))` and selector `0x20100394950e66014c25009b45d12b675210a6e7a002044a0e3de6544e3c4b37`.
    ```solidity
    event GlobalRootConfirmerSetUpdated(OperatorSet operatorSet);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct GlobalRootConfirmerSetUpdated {
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
        impl alloy_sol_types::SolEvent for GlobalRootConfirmerSetUpdated {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "GlobalRootConfirmerSetUpdated((address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    32u8, 16u8, 3u8, 148u8, 149u8, 14u8, 102u8, 1u8, 76u8, 37u8, 0u8, 155u8, 69u8,
                    209u8, 43u8, 103u8, 82u8, 16u8, 166u8, 231u8, 160u8, 2u8, 4u8, 74u8, 14u8,
                    61u8, 230u8, 84u8, 78u8, 60u8, 75u8, 55u8,
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
        impl alloy_sol_types::private::IntoLogData for GlobalRootConfirmerSetUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&GlobalRootConfirmerSetUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &GlobalRootConfirmerSetUpdated) -> alloy_sol_types::private::LogData {
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
    /**Constructor`.
    ```solidity
    constructor(address _bn254CertificateVerifier, address _ecdsaCertificateVerifier, string _version);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _bn254CertificateVerifier: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _ecdsaCertificateVerifier: alloy::sol_types::private::Address,
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
                    (
                        value._bn254CertificateVerifier,
                        value._ecdsaCertificateVerifier,
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
                        &self._bn254CertificateVerifier,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._ecdsaCertificateVerifier,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._version,
                    ),
                )
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
    /**Function with signature `getGlobalConfirmerSetReferenceTimestamp()` and selector `0x0f3f8edd`.
    ```solidity
    function getGlobalConfirmerSetReferenceTimestamp() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGlobalConfirmerSetReferenceTimestampCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getGlobalConfirmerSetReferenceTimestamp()`](getGlobalConfirmerSetReferenceTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGlobalConfirmerSetReferenceTimestampReturn {
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
            impl ::core::convert::From<getGlobalConfirmerSetReferenceTimestampCall>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: getGlobalConfirmerSetReferenceTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for getGlobalConfirmerSetReferenceTimestampCall
            {
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
            impl ::core::convert::From<getGlobalConfirmerSetReferenceTimestampReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: getGlobalConfirmerSetReferenceTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for getGlobalConfirmerSetReferenceTimestampReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGlobalConfirmerSetReferenceTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGlobalConfirmerSetReferenceTimestamp()";
            const SELECTOR: [u8; 4] = [15u8, 63u8, 142u8, 221u8];
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
                        let r: getGlobalConfirmerSetReferenceTimestampReturn = r.into();
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
                    let r: getGlobalConfirmerSetReferenceTimestampReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getGlobalRootConfirmerSet()` and selector `0x46282889`.
    ```solidity
    function getGlobalRootConfirmerSet() external view returns (OperatorSet memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGlobalRootConfirmerSetCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getGlobalRootConfirmerSet()`](getGlobalRootConfirmerSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getGlobalRootConfirmerSetReturn {
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
            impl ::core::convert::From<getGlobalRootConfirmerSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: getGlobalRootConfirmerSetCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGlobalRootConfirmerSetCall {
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
            impl ::core::convert::From<getGlobalRootConfirmerSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getGlobalRootConfirmerSetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getGlobalRootConfirmerSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGlobalRootConfirmerSetCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <OperatorSet as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (OperatorSet,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGlobalRootConfirmerSet()";
            const SELECTOR: [u8; 4] = [70u8, 40u8, 40u8, 137u8];
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
                        let r: getGlobalRootConfirmerSetReturn = r.into();
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
                    let r: getGlobalRootConfirmerSetReturn = r.into();
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
    /**Function with signature `initialize(address,(address,uint32),uint16,uint32,(bytes32,uint256,(uint256,uint256),uint256[]),(address,uint32),bytes32)` and selector `0x383b9b70`.
    ```solidity
    function initialize(address owner, OperatorSet memory _globalRootConfirmerSet, uint16 _globalRootConfirmationThreshold, uint32 referenceTimestamp, IOperatorTableCalculatorTypes.BN254OperatorSetInfo memory globalRootConfirmerSetInfo, ICrossChainRegistryTypes.OperatorSetConfig memory globalRootConfirmerSetConfig, bytes32 initialGlobalTableRoot) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _globalRootConfirmerSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _globalRootConfirmationThreshold: u16,
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub globalRootConfirmerSetInfo: <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub globalRootConfirmerSetConfig: <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub initialGlobalTableRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`initialize(address,(address,uint32),uint16,uint32,(bytes32,uint256,(uint256,uint256),uint256[]),(address,uint32),bytes32)`](initializeCall) function.
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
                OperatorSet,
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Uint<32>,
                IOperatorTableCalculatorTypes::BN254OperatorSetInfo,
                ICrossChainRegistryTypes::OperatorSetConfig,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                u16,
                u32,
                <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
                <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value.owner,
                        value._globalRootConfirmerSet,
                        value._globalRootConfirmationThreshold,
                        value.referenceTimestamp,
                        value.globalRootConfirmerSetInfo,
                        value.globalRootConfirmerSetConfig,
                        value.initialGlobalTableRoot,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        _globalRootConfirmerSet: tuple.1,
                        _globalRootConfirmationThreshold: tuple.2,
                        referenceTimestamp: tuple.3,
                        globalRootConfirmerSetInfo: tuple.4,
                        globalRootConfirmerSetConfig: tuple.5,
                        initialGlobalTableRoot: tuple.6,
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
                OperatorSet,
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Uint<32>,
                IOperatorTableCalculatorTypes::BN254OperatorSetInfo,
                ICrossChainRegistryTypes::OperatorSetConfig,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,(address,uint32),uint16,uint32,(bytes32,uint256,(uint256,uint256),uint256[]),(address,uint32),bytes32)";
            const SELECTOR: [u8; 4] = [56u8, 59u8, 155u8, 112u8];
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
                        &self.owner,
                    ),
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self._globalRootConfirmerSet,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._globalRootConfirmationThreshold,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTimestamp),
                    <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy_sol_types::SolType>::tokenize(
                        &self.globalRootConfirmerSetInfo,
                    ),
                    <ICrossChainRegistryTypes::OperatorSetConfig as alloy_sol_types::SolType>::tokenize(
                        &self.globalRootConfirmerSetConfig,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.initialGlobalTableRoot,
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
    /**Function with signature `setGlobalRootConfirmerSet((address,uint32))` and selector `0x0371406e`.
    ```solidity
    function setGlobalRootConfirmerSet(OperatorSet memory operatorSet) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGlobalRootConfirmerSetCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setGlobalRootConfirmerSet((address,uint32))`](setGlobalRootConfirmerSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGlobalRootConfirmerSetReturn {}
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
            impl ::core::convert::From<setGlobalRootConfirmerSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: setGlobalRootConfirmerSetCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setGlobalRootConfirmerSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
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
            impl ::core::convert::From<setGlobalRootConfirmerSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setGlobalRootConfirmerSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setGlobalRootConfirmerSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setGlobalRootConfirmerSetReturn {
            fn _tokenize(
                &self,
            ) -> <setGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setGlobalRootConfirmerSetCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setGlobalRootConfirmerSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setGlobalRootConfirmerSet((address,uint32))";
            const SELECTOR: [u8; 4] = [3u8, 113u8, 64u8, 110u8];
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
                setGlobalRootConfirmerSetReturn::_tokenize(ret)
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
    /**Function with signature `updateGlobalRootConfirmerSet(uint32,(bytes32,uint256,(uint256,uint256),uint256[]),(address,uint32))` and selector `0x1ab78d90`.
    ```solidity
    function updateGlobalRootConfirmerSet(uint32 referenceTimestamp, IOperatorTableCalculatorTypes.BN254OperatorSetInfo memory globalRootConfirmerSetInfo, ICrossChainRegistryTypes.OperatorSetConfig memory globalRootConfirmerSetConfig) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateGlobalRootConfirmerSetCall {
        #[allow(missing_docs)]
        pub referenceTimestamp: u32,
        #[allow(missing_docs)]
        pub globalRootConfirmerSetInfo: <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub globalRootConfirmerSetConfig: <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`updateGlobalRootConfirmerSet(uint32,(bytes32,uint256,(uint256,uint256),uint256[]),(address,uint32))`](updateGlobalRootConfirmerSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateGlobalRootConfirmerSetReturn {}
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
                IOperatorTableCalculatorTypes::BN254OperatorSetInfo,
                ICrossChainRegistryTypes::OperatorSetConfig,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<updateGlobalRootConfirmerSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateGlobalRootConfirmerSetCall) -> Self {
                    (
                        value.referenceTimestamp,
                        value.globalRootConfirmerSetInfo,
                        value.globalRootConfirmerSetConfig,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateGlobalRootConfirmerSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        referenceTimestamp: tuple.0,
                        globalRootConfirmerSetInfo: tuple.1,
                        globalRootConfirmerSetConfig: tuple.2,
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
            impl ::core::convert::From<updateGlobalRootConfirmerSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateGlobalRootConfirmerSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateGlobalRootConfirmerSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl updateGlobalRootConfirmerSetReturn {
            fn _tokenize(
                &self,
            ) -> <updateGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateGlobalRootConfirmerSetCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                IOperatorTableCalculatorTypes::BN254OperatorSetInfo,
                ICrossChainRegistryTypes::OperatorSetConfig,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateGlobalRootConfirmerSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateGlobalRootConfirmerSet(uint32,(bytes32,uint256,(uint256,uint256),uint256[]),(address,uint32))";
            const SELECTOR: [u8; 4] = [26u8, 183u8, 141u8, 144u8];
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
                    <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy_sol_types::SolType>::tokenize(
                        &self.globalRootConfirmerSetInfo,
                    ),
                    <ICrossChainRegistryTypes::OperatorSetConfig as alloy_sol_types::SolType>::tokenize(
                        &self.globalRootConfirmerSetConfig,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                updateGlobalRootConfirmerSetReturn::_tokenize(ret)
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
        GLOBAL_TABLE_ROOT_CERT_TYPEHASH(GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall),
        #[allow(missing_docs)]
        MAX_BPS(MAX_BPSCall),
        #[allow(missing_docs)]
        bn254CertificateVerifier(bn254CertificateVerifierCall),
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
        getGlobalConfirmerSetReferenceTimestamp(getGlobalConfirmerSetReferenceTimestampCall),
        #[allow(missing_docs)]
        getGlobalRootConfirmerSet(getGlobalRootConfirmerSetCall),
        #[allow(missing_docs)]
        getGlobalTableRootByTimestamp(getGlobalTableRootByTimestampCall),
        #[allow(missing_docs)]
        getGlobalTableUpdateMessageHash(getGlobalTableUpdateMessageHashCall),
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
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        setGlobalRootConfirmationThreshold(setGlobalRootConfirmationThresholdCall),
        #[allow(missing_docs)]
        setGlobalRootConfirmerSet(setGlobalRootConfirmerSetCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        updateGlobalRootConfirmerSet(updateGlobalRootConfirmerSetCall),
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
            [3u8, 113u8, 64u8, 110u8],
            [15u8, 63u8, 142u8, 221u8],
            [25u8, 59u8, 121u8, 243u8],
            [26u8, 183u8, 141u8, 144u8],
            [35u8, 112u8, 53u8, 108u8],
            [35u8, 183u8, 181u8, 178u8],
            [40u8, 82u8, 45u8, 121u8],
            [48u8, 239u8, 65u8, 180u8],
            [49u8, 165u8, 153u8, 210u8],
            [56u8, 59u8, 155u8, 112u8],
            [62u8, 246u8, 205u8, 122u8],
            [70u8, 36u8, 230u8, 163u8],
            [70u8, 40u8, 40u8, 137u8],
            [84u8, 253u8, 77u8, 80u8],
            [100u8, 225u8, 223u8, 132u8],
            [111u8, 114u8, 140u8, 80u8],
            [113u8, 80u8, 24u8, 166u8],
            [141u8, 165u8, 203u8, 91u8],
            [158u8, 169u8, 71u8, 120u8],
            [173u8, 15u8, 149u8, 130u8],
            [184u8, 193u8, 67u8, 6u8],
            [194u8, 82u8, 170u8, 34u8],
            [195u8, 98u8, 31u8, 10u8],
            [195u8, 190u8, 30u8, 51u8],
            [197u8, 145u8, 106u8, 57u8],
            [234u8, 174u8, 217u8, 213u8],
            [242u8, 253u8, 227u8, 139u8],
            [253u8, 150u8, 127u8, 71u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OperatorTableUpdaterCalls {
        const NAME: &'static str = "OperatorTableUpdaterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 28usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::GLOBAL_TABLE_ROOT_CERT_TYPEHASH(_) => {
                    <GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_BPS(_) => <MAX_BPSCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::bn254CertificateVerifier(_) => {
                    <bn254CertificateVerifierCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::confirmGlobalTableRoot(_) => {
                    <confirmGlobalTableRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disableRoot(_) => {
                    <disableRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ecdsaCertificateVerifier(_) => {
                    <ecdsaCertificateVerifierCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCertificateVerifier(_) => {
                    <getCertificateVerifierCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentGlobalTableRoot(_) => {
                    <getCurrentGlobalTableRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGlobalConfirmerSetReferenceTimestamp(_) => {
                    <getGlobalConfirmerSetReferenceTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGlobalRootConfirmerSet(_) => {
                    <getGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGlobalTableRootByTimestamp(_) => {
                    <getGlobalTableRootByTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGlobalTableUpdateMessageHash(_) => {
                    <getGlobalTableUpdateMessageHashCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isRootValid(_) => {
                    <isRootValidCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isRootValidByTimestamp(_) => {
                    <isRootValidByTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setGlobalRootConfirmationThreshold(_) => {
                    <setGlobalRootConfirmationThresholdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setGlobalRootConfirmerSet(_) => {
                    <setGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateGlobalRootConfirmerSet(_) => {
                    <updateGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn setGlobalRootConfirmerSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <setGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::setGlobalRootConfirmerSet)
                    }
                    setGlobalRootConfirmerSet
                },
                {
                    fn getGlobalConfirmerSetReferenceTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGlobalConfirmerSetReferenceTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getGlobalConfirmerSetReferenceTimestamp,
                            )
                    }
                    getGlobalConfirmerSetReferenceTimestamp
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
                    fn updateGlobalRootConfirmerSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <updateGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::updateGlobalRootConfirmerSet)
                    }
                    updateGlobalRootConfirmerSet
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
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::initialize)
                    }
                    initialize
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
                    fn getGlobalRootConfirmerSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(OperatorTableUpdaterCalls::getGlobalRootConfirmerSet)
                    }
                    getGlobalRootConfirmerSet
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterCalls::version)
                    }
                    version
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
                    fn setGlobalRootConfirmerSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <setGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::setGlobalRootConfirmerSet)
                    }
                    setGlobalRootConfirmerSet
                },
                {
                    fn getGlobalConfirmerSetReferenceTimestamp(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGlobalConfirmerSetReferenceTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                OperatorTableUpdaterCalls::getGlobalConfirmerSetReferenceTimestamp,
                            )
                    }
                    getGlobalConfirmerSetReferenceTimestamp
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
                    fn updateGlobalRootConfirmerSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <updateGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::updateGlobalRootConfirmerSet)
                    }
                    updateGlobalRootConfirmerSet
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
                    fn initialize(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::initialize)
                    }
                    initialize
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
                    fn getGlobalRootConfirmerSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <getGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterCalls::getGlobalRootConfirmerSet)
                    }
                    getGlobalRootConfirmerSet
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<OperatorTableUpdaterCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(OperatorTableUpdaterCalls::version)
                    }
                    version
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
                Self::GLOBAL_TABLE_ROOT_CERT_TYPEHASH(inner) => {
                    <GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_BPS(inner) => {
                    <MAX_BPSCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::bn254CertificateVerifier(inner) => {
                    <bn254CertificateVerifierCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getGlobalConfirmerSetReferenceTimestamp(inner) => {
                    <getGlobalConfirmerSetReferenceTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getGlobalRootConfirmerSet(inner) => {
                    <getGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setGlobalRootConfirmerSet(inner) => {
                    <setGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateGlobalRootConfirmerSet(inner) => {
                    <updateGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::GLOBAL_TABLE_ROOT_CERT_TYPEHASH(inner) => {
                    <GLOBAL_TABLE_ROOT_CERT_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_BPS(inner) => {
                    <MAX_BPSCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::bn254CertificateVerifier(inner) => {
                    <bn254CertificateVerifierCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getGlobalConfirmerSetReferenceTimestamp(inner) => {
                    <getGlobalConfirmerSetReferenceTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getGlobalRootConfirmerSet(inner) => {
                    <getGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setGlobalRootConfirmerSet(inner) => {
                    <setGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::updateGlobalRootConfirmerSet(inner) => {
                    <updateGlobalRootConfirmerSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        CertificateInvalid(CertificateInvalid),
        #[allow(missing_docs)]
        GlobalTableRootInFuture(GlobalTableRootInFuture),
        #[allow(missing_docs)]
        GlobalTableRootStale(GlobalTableRootStale),
        #[allow(missing_docs)]
        InvalidConfirmationThreshold(InvalidConfirmationThreshold),
        #[allow(missing_docs)]
        InvalidCurveType(InvalidCurveType),
        #[allow(missing_docs)]
        InvalidGlobalTableRoot(InvalidGlobalTableRoot),
        #[allow(missing_docs)]
        InvalidMessageHash(InvalidMessageHash),
        #[allow(missing_docs)]
        InvalidOperatorSetProof(InvalidOperatorSetProof),
        #[allow(missing_docs)]
        InvalidProofLength(InvalidProofLength),
        #[allow(missing_docs)]
        InvalidRoot(InvalidRoot),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        InvalidSignatureLength(InvalidSignatureLength),
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
            [75u8, 230u8, 50u8, 27u8],
            [77u8, 197u8, 246u8, 164u8],
            [80u8, 69u8, 112u8, 227u8],
            [139u8, 86u8, 100u8, 45u8],
            [175u8, 164u8, 44u8, 167u8],
            [179u8, 81u8, 43u8, 12u8],
            [180u8, 35u8, 59u8, 106u8],
            [193u8, 8u8, 16u8, 124u8],
            [199u8, 58u8, 19u8, 106u8],
            [253u8, 234u8, 124u8, 9u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for OperatorTableUpdaterErrors {
        const NAME: &'static str = "OperatorTableUpdaterErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 14usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CertificateInvalid(_) => {
                    <CertificateInvalid as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GlobalTableRootInFuture(_) => {
                    <GlobalTableRootInFuture as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GlobalTableRootStale(_) => {
                    <GlobalTableRootStale as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidConfirmationThreshold(_) => {
                    <InvalidConfirmationThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCurveType(_) => {
                    <InvalidCurveType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidGlobalTableRoot(_) => {
                    <InvalidGlobalTableRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidMessageHash(_) => {
                    <InvalidMessageHash as alloy_sol_types::SolError>::SELECTOR
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
                Self::InvalidSignatureLength(_) => {
                    <InvalidSignatureLength as alloy_sol_types::SolError>::SELECTOR
                }
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
                    fn InvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(OperatorTableUpdaterErrors::InvalidSignatureLength)
                    }
                    InvalidSignatureLength
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
                    fn InvalidSignatureLength(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<OperatorTableUpdaterErrors> {
                        <InvalidSignatureLength as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(OperatorTableUpdaterErrors::InvalidSignatureLength)
                    }
                    InvalidSignatureLength
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
                Self::CertificateInvalid(inner) => {
                    <CertificateInvalid as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::GlobalTableRootInFuture(inner) => {
                    <GlobalTableRootInFuture as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::GlobalTableRootStale(inner) => {
                    <GlobalTableRootStale as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidConfirmationThreshold(inner) => {
                    <InvalidConfirmationThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidCurveType(inner) => {
                    <InvalidCurveType as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidGlobalTableRoot(inner) => {
                    <InvalidGlobalTableRoot as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidMessageHash(inner) => {
                    <InvalidMessageHash as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InvalidSignatureLength(inner) => {
                    <InvalidSignatureLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::CertificateInvalid(inner) => {
                    <CertificateInvalid as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::GlobalTableRootInFuture(inner) => {
                    <GlobalTableRootInFuture as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::GlobalTableRootStale(inner) => {
                    <GlobalTableRootStale as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidConfirmationThreshold(inner) => {
                    <InvalidConfirmationThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidCurveType(inner) => {
                    <InvalidCurveType as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidGlobalTableRoot(inner) => {
                    <InvalidGlobalTableRoot as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidMessageHash(inner) => {
                    <InvalidMessageHash as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::InvalidSignatureLength(inner) => {
                    <InvalidSignatureLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
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
        GlobalRootConfirmationThresholdUpdated(GlobalRootConfirmationThresholdUpdated),
        #[allow(missing_docs)]
        GlobalRootConfirmerSetUpdated(GlobalRootConfirmerSetUpdated),
        #[allow(missing_docs)]
        GlobalRootDisabled(GlobalRootDisabled),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        NewGlobalTableRoot(NewGlobalTableRoot),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
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
                32u8, 16u8, 3u8, 148u8, 149u8, 14u8, 102u8, 1u8, 76u8, 37u8, 0u8, 155u8, 69u8,
                209u8, 43u8, 103u8, 82u8, 16u8, 166u8, 231u8, 160u8, 2u8, 4u8, 74u8, 14u8, 61u8,
                230u8, 84u8, 78u8, 60u8, 75u8, 55u8,
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
                245u8, 209u8, 131u8, 109u8, 248u8, 252u8, 215u8, 193u8, 229u8, 64u8, 71u8, 233u8,
                74u8, 200u8, 119u8, 61u8, 40u8, 85u8, 57u8, 86u8, 3u8, 226u8, 239u8, 155u8, 165u8,
                245u8, 241u8, 105u8, 5u8, 242u8, 37u8, 146u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for OperatorTableUpdaterEvents {
        const NAME: &'static str = "OperatorTableUpdaterEvents";
        const COUNT: usize = 6usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
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
                    <GlobalRootConfirmerSetUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <GlobalRootConfirmerSetUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                        )
                        .map(Self::GlobalRootConfirmerSetUpdated)
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
                Self::GlobalRootConfirmationThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::GlobalRootConfirmerSetUpdated(inner) => {
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
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::GlobalRootConfirmationThresholdUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::GlobalRootConfirmerSetUpdated(inner) => {
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
        _version: alloy::sol_types::private::String,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<OperatorTableUpdaterInstance<P, N>>>
    {
        OperatorTableUpdaterInstance::<P, N>::deploy(
            provider,
            _bn254CertificateVerifier,
            _ecdsaCertificateVerifier,
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
        _version: alloy::sol_types::private::String,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        OperatorTableUpdaterInstance::<P, N>::deploy_builder(
            provider,
            _bn254CertificateVerifier,
            _ecdsaCertificateVerifier,
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
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::Result<OperatorTableUpdaterInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _bn254CertificateVerifier,
                _ecdsaCertificateVerifier,
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
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _bn254CertificateVerifier,
                        _ecdsaCertificateVerifier,
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
        ///Creates a new call builder for the [`bn254CertificateVerifier`] function.
        pub fn bn254CertificateVerifier(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, bn254CertificateVerifierCall, N> {
            self.call_builder(&bn254CertificateVerifierCall)
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
        ///Creates a new call builder for the [`getGlobalConfirmerSetReferenceTimestamp`] function.
        pub fn getGlobalConfirmerSetReferenceTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getGlobalConfirmerSetReferenceTimestampCall, N>
        {
            self.call_builder(&getGlobalConfirmerSetReferenceTimestampCall)
        }
        ///Creates a new call builder for the [`getGlobalRootConfirmerSet`] function.
        pub fn getGlobalRootConfirmerSet(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getGlobalRootConfirmerSetCall, N> {
            self.call_builder(&getGlobalRootConfirmerSetCall)
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
            owner: alloy::sol_types::private::Address,
            _globalRootConfirmerSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            _globalRootConfirmationThreshold: u16,
            referenceTimestamp: u32,
            globalRootConfirmerSetInfo: <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
            globalRootConfirmerSetConfig: <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
            initialGlobalTableRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall {
                owner,
                _globalRootConfirmerSet,
                _globalRootConfirmationThreshold,
                referenceTimestamp,
                globalRootConfirmerSetInfo,
                globalRootConfirmerSetConfig,
                initialGlobalTableRoot,
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
        ///Creates a new call builder for the [`setGlobalRootConfirmerSet`] function.
        pub fn setGlobalRootConfirmerSet(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, setGlobalRootConfirmerSetCall, N> {
            self.call_builder(&setGlobalRootConfirmerSetCall { operatorSet })
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`updateGlobalRootConfirmerSet`] function.
        pub fn updateGlobalRootConfirmerSet(
            &self,
            referenceTimestamp: u32,
            globalRootConfirmerSetInfo: <IOperatorTableCalculatorTypes::BN254OperatorSetInfo as alloy::sol_types::SolType>::RustType,
            globalRootConfirmerSetConfig: <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, updateGlobalRootConfirmerSetCall, N> {
            self.call_builder(&updateGlobalRootConfirmerSetCall {
                referenceTimestamp,
                globalRootConfirmerSetInfo,
                globalRootConfirmerSetConfig,
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
        ///Creates a new event filter for the [`GlobalRootConfirmationThresholdUpdated`] event.
        pub fn GlobalRootConfirmationThresholdUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, GlobalRootConfirmationThresholdUpdated, N> {
            self.event_filter::<GlobalRootConfirmationThresholdUpdated>()
        }
        ///Creates a new event filter for the [`GlobalRootConfirmerSetUpdated`] event.
        pub fn GlobalRootConfirmerSetUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, GlobalRootConfirmerSetUpdated, N> {
            self.event_filter::<GlobalRootConfirmerSetUpdated>()
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
    }
}
