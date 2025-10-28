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

library IKeyRegistrarTypes {
    type CurveType is uint8;
}

interface IKeyRegistrar {
    struct OperatorSet {
        address avs;
        uint32 id;
    }

    error ConfigurationAlreadySet();
    error InvalidCurveType();
    error InvalidKeyFormat();
    error InvalidKeypair();
    error KeyAlreadyRegistered();
    error KeyNotFound(OperatorSet operatorSet, address operator);
    error OperatorAlreadyRegistered();
    error OperatorSetNotConfigured();
    error OperatorStillSlashable(OperatorSet operatorSet, address operator);
    error ZeroAddress();
    error ZeroPubkey();

    event AggregateBN254KeyUpdated(OperatorSet operatorSet, BN254.G1Point newAggregateKey);
    event KeyDeregistered(OperatorSet operatorSet, address indexed operator, IKeyRegistrarTypes.CurveType curveType);
    event KeyRegistered(OperatorSet operatorSet, address indexed operator, IKeyRegistrarTypes.CurveType curveType, bytes pubkey);
    event OperatorSetConfigured(OperatorSet operatorSet, IKeyRegistrarTypes.CurveType curveType);

    function configureOperatorSet(OperatorSet memory operatorSet, IKeyRegistrarTypes.CurveType curveType) external;
    function deregisterKey(address operator, OperatorSet memory operatorSet) external;
    function encodeBN254KeyData(BN254.G1Point memory g1Point, BN254.G2Point memory g2Point) external pure returns (bytes memory);
    function getBN254Key(OperatorSet memory operatorSet, address operator) external view returns (BN254.G1Point memory g1Point, BN254.G2Point memory g2Point);
    function getBN254KeyRegistrationMessageHash(address operator, OperatorSet memory operatorSet, bytes memory keyData) external view returns (bytes32);
    function getECDSAAddress(OperatorSet memory operatorSet, address operator) external view returns (address);
    function getECDSAKey(OperatorSet memory operatorSet, address operator) external view returns (bytes memory);
    function getECDSAKeyRegistrationMessageHash(address operator, OperatorSet memory operatorSet, address keyAddress) external view returns (bytes32);
    function getKeyHash(OperatorSet memory operatorSet, address operator) external view returns (bytes32);
    function getOperatorFromSigningKey(OperatorSet memory operatorSet, bytes memory keyData) external view returns (address, bool);
    function getOperatorSetCurveType(OperatorSet memory operatorSet) external view returns (IKeyRegistrarTypes.CurveType);
    function isKeyGloballyRegistered(bytes32 keyHash) external view returns (bool);
    function isRegistered(OperatorSet memory operatorSet, address operator) external view returns (bool);
    function registerKey(address operator, OperatorSet memory operatorSet, bytes memory pubkey, bytes memory signature) external;
    function version() external view returns (string memory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "configureOperatorSet",
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
        "name": "curveType",
        "type": "uint8",
        "internalType": "enum IKeyRegistrarTypes.CurveType"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterKey",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
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
    "name": "encodeBN254KeyData",
    "inputs": [
      {
        "name": "g1Point",
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
        "name": "g2Point",
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
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "getBN254Key",
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
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "g1Point",
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
        "name": "g2Point",
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
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getBN254KeyRegistrationMessageHash",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
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
        "name": "keyData",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getECDSAAddress",
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
        "name": "operator",
        "type": "address",
        "internalType": "address"
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
    "name": "getECDSAKey",
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
        "name": "operator",
        "type": "address",
        "internalType": "address"
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
    "name": "getECDSAKeyRegistrationMessageHash",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
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
        "name": "keyAddress",
        "type": "address",
        "internalType": "address"
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
    "name": "getKeyHash",
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
        "name": "operator",
        "type": "address",
        "internalType": "address"
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
    "name": "getOperatorFromSigningKey",
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
        "name": "keyData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
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
    "name": "getOperatorSetCurveType",
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
        "type": "uint8",
        "internalType": "enum IKeyRegistrarTypes.CurveType"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isKeyGloballyRegistered",
    "inputs": [
      {
        "name": "keyHash",
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
    "name": "isRegistered",
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
        "name": "operator",
        "type": "address",
        "internalType": "address"
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
    "name": "registerKey",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
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
        "name": "pubkey",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
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
    "name": "AggregateBN254KeyUpdated",
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
        "name": "newAggregateKey",
        "type": "tuple",
        "indexed": false,
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
    "anonymous": false
  },
  {
    "type": "event",
    "name": "KeyDeregistered",
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
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "curveType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IKeyRegistrarTypes.CurveType"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "KeyRegistered",
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
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "curveType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IKeyRegistrarTypes.CurveType"
      },
      {
        "name": "pubkey",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSetConfigured",
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
        "name": "curveType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IKeyRegistrarTypes.CurveType"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "ConfigurationAlreadySet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidCurveType",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidKeyFormat",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidKeypair",
    "inputs": []
  },
  {
    "type": "error",
    "name": "KeyAlreadyRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "KeyNotFound",
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
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "OperatorAlreadyRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorSetNotConfigured",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorStillSlashable",
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
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "ZeroAddress",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ZeroPubkey",
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
pub mod IKeyRegistrar {
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
    /**Custom error with signature `ConfigurationAlreadySet()` and selector `0x0081f09f`.
    ```solidity
    error ConfigurationAlreadySet();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ConfigurationAlreadySet;
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
        impl ::core::convert::From<ConfigurationAlreadySet> for UnderlyingRustTuple<'_> {
            fn from(value: ConfigurationAlreadySet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ConfigurationAlreadySet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ConfigurationAlreadySet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ConfigurationAlreadySet()";
            const SELECTOR: [u8; 4] = [0u8, 129u8, 240u8, 159u8];
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
    /**Custom error with signature `InvalidKeyFormat()` and selector `0xd1091181`.
    ```solidity
    error InvalidKeyFormat();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidKeyFormat;
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
        impl ::core::convert::From<InvalidKeyFormat> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidKeyFormat) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidKeyFormat {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidKeyFormat {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidKeyFormat()";
            const SELECTOR: [u8; 4] = [209u8, 9u8, 17u8, 129u8];
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
    /**Custom error with signature `InvalidKeypair()` and selector `0x1b56a68b`.
    ```solidity
    error InvalidKeypair();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidKeypair;
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
        impl ::core::convert::From<InvalidKeypair> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidKeypair) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidKeypair {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidKeypair {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidKeypair()";
            const SELECTOR: [u8; 4] = [27u8, 86u8, 166u8, 139u8];
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
    /**Custom error with signature `KeyAlreadyRegistered()` and selector `0x18f78402`.
    ```solidity
    error KeyAlreadyRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyAlreadyRegistered;
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
        impl ::core::convert::From<KeyAlreadyRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: KeyAlreadyRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyAlreadyRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyAlreadyRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyAlreadyRegistered()";
            const SELECTOR: [u8; 4] = [24u8, 247u8, 132u8, 2u8];
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
    /**Custom error with signature `KeyNotFound((address,uint32),address)` and selector `0x2e40e187`.
    ```solidity
    error KeyNotFound(OperatorSet operatorSet, address operator);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyNotFound {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
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
        type UnderlyingSolTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <OperatorSet as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<KeyNotFound> for UnderlyingRustTuple<'_> {
            fn from(value: KeyNotFound) -> Self {
                (value.operatorSet, value.operator)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyNotFound {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorSet: tuple.0,
                    operator: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyNotFound {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyNotFound((address,uint32),address)";
            const SELECTOR: [u8; 4] = [46u8, 64u8, 225u8, 135u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
    /**Custom error with signature `OperatorAlreadyRegistered()` and selector `0x42ee68b5`.
    ```solidity
    error OperatorAlreadyRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorAlreadyRegistered;
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
        impl ::core::convert::From<OperatorAlreadyRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorAlreadyRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorAlreadyRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorAlreadyRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorAlreadyRegistered()";
            const SELECTOR: [u8; 4] = [66u8, 238u8, 104u8, 181u8];
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
    /**Custom error with signature `OperatorSetNotConfigured()` and selector `0xb9a620da`.
    ```solidity
    error OperatorSetNotConfigured();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSetNotConfigured;
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
        impl ::core::convert::From<OperatorSetNotConfigured> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorSetNotConfigured) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSetNotConfigured {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorSetNotConfigured {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorSetNotConfigured()";
            const SELECTOR: [u8; 4] = [185u8, 166u8, 32u8, 218u8];
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
    /**Custom error with signature `OperatorStillSlashable((address,uint32),address)` and selector `0x10702879`.
    ```solidity
    error OperatorStillSlashable(OperatorSet operatorSet, address operator);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorStillSlashable {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
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
        type UnderlyingSolTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <OperatorSet as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<OperatorStillSlashable> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorStillSlashable) -> Self {
                (value.operatorSet, value.operator)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorStillSlashable {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorSet: tuple.0,
                    operator: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorStillSlashable {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorStillSlashable((address,uint32),address)";
            const SELECTOR: [u8; 4] = [16u8, 112u8, 40u8, 121u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
    /**Custom error with signature `ZeroAddress()` and selector `0xd92e233d`.
    ```solidity
    error ZeroAddress();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroAddress;
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
        impl ::core::convert::From<ZeroAddress> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroAddress) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroAddress {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroAddress {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroAddress()";
            const SELECTOR: [u8; 4] = [217u8, 46u8, 35u8, 61u8];
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
    /**Custom error with signature `ZeroPubkey()` and selector `0x4935505f`.
    ```solidity
    error ZeroPubkey();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ZeroPubkey;
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
        impl ::core::convert::From<ZeroPubkey> for UnderlyingRustTuple<'_> {
            fn from(value: ZeroPubkey) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ZeroPubkey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ZeroPubkey {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ZeroPubkey()";
            const SELECTOR: [u8; 4] = [73u8, 53u8, 80u8, 95u8];
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
    /**Event with signature `AggregateBN254KeyUpdated((address,uint32),(uint256,uint256))` and selector `0xdfa2f59e55747ba641fbdff4eb78577de8789d605920d5be4a74ee3a6470d1d1`.
    ```solidity
    event AggregateBN254KeyUpdated(OperatorSet operatorSet, BN254.G1Point newAggregateKey);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AggregateBN254KeyUpdated {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub newAggregateKey: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for AggregateBN254KeyUpdated {
            type DataTuple<'a> = (OperatorSet, BN254::G1Point);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "AggregateBN254KeyUpdated((address,uint32),(uint256,uint256))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    223u8, 162u8, 245u8, 158u8, 85u8, 116u8, 123u8, 166u8, 65u8, 251u8, 223u8,
                    244u8, 235u8, 120u8, 87u8, 125u8, 232u8, 120u8, 157u8, 96u8, 89u8, 32u8, 213u8,
                    190u8, 74u8, 116u8, 238u8, 58u8, 100u8, 112u8, 209u8, 209u8,
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
                    newAggregateKey: data.1,
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
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.newAggregateKey),
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
        impl alloy_sol_types::private::IntoLogData for AggregateBN254KeyUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AggregateBN254KeyUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AggregateBN254KeyUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `KeyDeregistered((address,uint32),address,uint8)` and selector `0x28d3c3cee49478ec6fd219cfd685cd15cd01d95cabf69b4b7b57f9eaa3eb6442`.
    ```solidity
    event KeyDeregistered(OperatorSet operatorSet, address indexed operator, IKeyRegistrarTypes.CurveType curveType);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeyDeregistered {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub curveType: <IKeyRegistrarTypes::CurveType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for KeyDeregistered {
            type DataTuple<'a> = (OperatorSet, IKeyRegistrarTypes::CurveType);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "KeyDeregistered((address,uint32),address,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    40u8, 211u8, 195u8, 206u8, 228u8, 148u8, 120u8, 236u8, 111u8, 210u8, 25u8,
                    207u8, 214u8, 133u8, 205u8, 21u8, 205u8, 1u8, 217u8, 92u8, 171u8, 246u8, 155u8,
                    75u8, 123u8, 87u8, 249u8, 234u8, 163u8, 235u8, 100u8, 66u8,
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
                    operator: topics.1,
                    curveType: data.1,
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
                    <IKeyRegistrarTypes::CurveType as alloy_sol_types::SolType>::tokenize(
                        &self.curveType,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for KeyDeregistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeyDeregistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KeyDeregistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `KeyRegistered((address,uint32),address,uint8,bytes)` and selector `0x1201ce0c5e577111bce91e907fd99cb183da5edc1e3fb650ca40769e4e9176dd`.
    ```solidity
    event KeyRegistered(OperatorSet operatorSet, address indexed operator, IKeyRegistrarTypes.CurveType curveType, bytes pubkey);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct KeyRegistered {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub curveType: <IKeyRegistrarTypes::CurveType as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pubkey: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for KeyRegistered {
            type DataTuple<'a> = (
                OperatorSet,
                IKeyRegistrarTypes::CurveType,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "KeyRegistered((address,uint32),address,uint8,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    18u8, 1u8, 206u8, 12u8, 94u8, 87u8, 113u8, 17u8, 188u8, 233u8, 30u8, 144u8,
                    127u8, 217u8, 156u8, 177u8, 131u8, 218u8, 94u8, 220u8, 30u8, 63u8, 182u8, 80u8,
                    202u8, 64u8, 118u8, 158u8, 78u8, 145u8, 118u8, 221u8,
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
                    operator: topics.1,
                    curveType: data.1,
                    pubkey: data.2,
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
                    <IKeyRegistrarTypes::CurveType as alloy_sol_types::SolType>::tokenize(
                        &self.curveType,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.pubkey,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone())
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
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for KeyRegistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&KeyRegistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &KeyRegistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OperatorSetConfigured((address,uint32),uint8)` and selector `0xb2266cb118e57095fcdbedb24dabd9fc9f5127e2dbedf62ce6ee71696fb8b6e7`.
    ```solidity
    event OperatorSetConfigured(OperatorSet operatorSet, IKeyRegistrarTypes.CurveType curveType);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSetConfigured {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub curveType: <IKeyRegistrarTypes::CurveType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorSetConfigured {
            type DataTuple<'a> = (OperatorSet, IKeyRegistrarTypes::CurveType);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorSetConfigured((address,uint32),uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    178u8, 38u8, 108u8, 177u8, 24u8, 229u8, 112u8, 149u8, 252u8, 219u8, 237u8,
                    178u8, 77u8, 171u8, 217u8, 252u8, 159u8, 81u8, 39u8, 226u8, 219u8, 237u8,
                    246u8, 44u8, 230u8, 238u8, 113u8, 105u8, 111u8, 184u8, 182u8, 231u8,
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
                    curveType: data.1,
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
                    <IKeyRegistrarTypes::CurveType as alloy_sol_types::SolType>::tokenize(
                        &self.curveType,
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
        impl alloy_sol_types::private::IntoLogData for OperatorSetConfigured {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSetConfigured> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSetConfigured) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `configureOperatorSet((address,uint32),uint8)` and selector `0xea0d8149`.
    ```solidity
    function configureOperatorSet(OperatorSet memory operatorSet, IKeyRegistrarTypes.CurveType curveType) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configureOperatorSetCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub curveType: <IKeyRegistrarTypes::CurveType as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`configureOperatorSet((address,uint32),uint8)`](configureOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct configureOperatorSetReturn {}
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
            type UnderlyingSolTuple<'a> = (OperatorSet, IKeyRegistrarTypes::CurveType);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                <IKeyRegistrarTypes::CurveType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<configureOperatorSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: configureOperatorSetCall) -> Self {
                    (value.operatorSet, value.curveType)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configureOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        curveType: tuple.1,
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
            impl ::core::convert::From<configureOperatorSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: configureOperatorSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configureOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl configureOperatorSetReturn {
            fn _tokenize(
                &self,
            ) -> <configureOperatorSetCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for configureOperatorSetCall {
            type Parameters<'a> = (OperatorSet, IKeyRegistrarTypes::CurveType);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = configureOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "configureOperatorSet((address,uint32),uint8)";
            const SELECTOR: [u8; 4] = [234u8, 13u8, 129u8, 73u8];
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
                    <IKeyRegistrarTypes::CurveType as alloy_sol_types::SolType>::tokenize(
                        &self.curveType,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                configureOperatorSetReturn::_tokenize(ret)
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
    /**Function with signature `deregisterKey(address,(address,uint32))` and selector `0x87ab86f4`.
    ```solidity
    function deregisterKey(address operator, OperatorSet memory operatorSet) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterKeyCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`deregisterKey(address,(address,uint32))`](deregisterKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterKeyReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address, OperatorSet);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<deregisterKeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterKeyCall) -> Self {
                    (value.operator, value.operatorSet)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSet: tuple.1,
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
            impl ::core::convert::From<deregisterKeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterKeyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl deregisterKeyReturn {
            fn _tokenize(
                &self,
            ) -> <deregisterKeyCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, OperatorSet);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterKeyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterKey(address,(address,uint32))";
            const SELECTOR: [u8; 4] = [135u8, 171u8, 134u8, 244u8];
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
                        &self.operator,
                    ),
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                deregisterKeyReturn::_tokenize(ret)
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
    /**Function with signature `encodeBN254KeyData((uint256,uint256),(uint256[2],uint256[2]))` and selector `0x50435add`.
    ```solidity
    function encodeBN254KeyData(BN254.G1Point memory g1Point, BN254.G2Point memory g2Point) external pure returns (bytes memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct encodeBN254KeyDataCall {
        #[allow(missing_docs)]
        pub g1Point: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub g2Point: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`encodeBN254KeyData((uint256,uint256),(uint256[2],uint256[2]))`](encodeBN254KeyDataCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct encodeBN254KeyDataReturn {
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
            type UnderlyingSolTuple<'a> = (BN254::G1Point, BN254::G2Point);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                <BN254::G2Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<encodeBN254KeyDataCall> for UnderlyingRustTuple<'_> {
                fn from(value: encodeBN254KeyDataCall) -> Self {
                    (value.g1Point, value.g2Point)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for encodeBN254KeyDataCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        g1Point: tuple.0,
                        g2Point: tuple.1,
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
            impl ::core::convert::From<encodeBN254KeyDataReturn> for UnderlyingRustTuple<'_> {
                fn from(value: encodeBN254KeyDataReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for encodeBN254KeyDataReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for encodeBN254KeyDataCall {
            type Parameters<'a> = (BN254::G1Point, BN254::G2Point);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Bytes;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "encodeBN254KeyData((uint256,uint256),(uint256[2],uint256[2]))";
            const SELECTOR: [u8; 4] = [80u8, 67u8, 90u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.g1Point),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.g2Point),
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
                        let r: encodeBN254KeyDataReturn = r.into();
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
                    let r: encodeBN254KeyDataReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getBN254Key((address,uint32),address)` and selector `0x9a43e3fb`.
    ```solidity
    function getBN254Key(OperatorSet memory operatorSet, address operator) external view returns (BN254.G1Point memory g1Point, BN254.G2Point memory g2Point);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBN254KeyCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getBN254Key((address,uint32),address)`](getBN254KeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBN254KeyReturn {
        #[allow(missing_docs)]
        pub g1Point: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub g2Point: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getBN254KeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBN254KeyCall) -> Self {
                    (value.operatorSet, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBN254KeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        operator: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BN254::G1Point, BN254::G2Point);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                <BN254::G2Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getBN254KeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBN254KeyReturn) -> Self {
                    (value.g1Point, value.g2Point)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBN254KeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        g1Point: tuple.0,
                        g2Point: tuple.1,
                    }
                }
            }
        }
        impl getBN254KeyReturn {
            fn _tokenize(&self) -> <getBN254KeyCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                (
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.g1Point),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.g2Point),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBN254KeyCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBN254KeyReturn;
            type ReturnTuple<'a> = (BN254::G1Point, BN254::G2Point);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBN254Key((address,uint32),address)";
            const SELECTOR: [u8; 4] = [154u8, 67u8, 227u8, 251u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getBN254KeyReturn::_tokenize(ret)
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
    /**Function with signature `getBN254KeyRegistrationMessageHash(address,(address,uint32),bytes)` and selector `0x7690e395`.
    ```solidity
    function getBN254KeyRegistrationMessageHash(address operator, OperatorSet memory operatorSet, bytes memory keyData) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBN254KeyRegistrationMessageHashCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub keyData: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getBN254KeyRegistrationMessageHash(address,(address,uint32),bytes)`](getBN254KeyRegistrationMessageHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBN254KeyRegistrationMessageHashReturn {
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
                alloy::sol_types::sol_data::Address,
                OperatorSet,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getBN254KeyRegistrationMessageHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBN254KeyRegistrationMessageHashCall) -> Self {
                    (value.operator, value.operatorSet, value.keyData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBN254KeyRegistrationMessageHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSet: tuple.1,
                        keyData: tuple.2,
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
            impl ::core::convert::From<getBN254KeyRegistrationMessageHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBN254KeyRegistrationMessageHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBN254KeyRegistrationMessageHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBN254KeyRegistrationMessageHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                OperatorSet,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getBN254KeyRegistrationMessageHash(address,(address,uint32),bytes)";
            const SELECTOR: [u8; 4] = [118u8, 144u8, 227u8, 149u8];
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
                        &self.operator,
                    ),
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.keyData,
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
                        let r: getBN254KeyRegistrationMessageHashReturn = r.into();
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
                    let r: getBN254KeyRegistrationMessageHashReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getECDSAAddress((address,uint32),address)` and selector `0x3b32a7bd`.
    ```solidity
    function getECDSAAddress(OperatorSet memory operatorSet, address operator) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getECDSAAddressCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getECDSAAddress((address,uint32),address)`](getECDSAAddressCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getECDSAAddressReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getECDSAAddressCall> for UnderlyingRustTuple<'_> {
                fn from(value: getECDSAAddressCall) -> Self {
                    (value.operatorSet, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getECDSAAddressCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        operator: tuple.1,
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
            impl ::core::convert::From<getECDSAAddressReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getECDSAAddressReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getECDSAAddressReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getECDSAAddressCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getECDSAAddress((address,uint32),address)";
            const SELECTOR: [u8; 4] = [59u8, 50u8, 167u8, 189u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
                        let r: getECDSAAddressReturn = r.into();
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
                    let r: getECDSAAddressReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getECDSAKey((address,uint32),address)` and selector `0xaa165c30`.
    ```solidity
    function getECDSAKey(OperatorSet memory operatorSet, address operator) external view returns (bytes memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getECDSAKeyCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getECDSAKey((address,uint32),address)`](getECDSAKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getECDSAKeyReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getECDSAKeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: getECDSAKeyCall) -> Self {
                    (value.operatorSet, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getECDSAKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        operator: tuple.1,
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
            impl ::core::convert::From<getECDSAKeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getECDSAKeyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getECDSAKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getECDSAKeyCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Bytes;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getECDSAKey((address,uint32),address)";
            const SELECTOR: [u8; 4] = [170u8, 22u8, 92u8, 48u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
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
                        let r: getECDSAKeyReturn = r.into();
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
                    let r: getECDSAKeyReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getECDSAKeyRegistrationMessageHash(address,(address,uint32),address)` and selector `0xd9f12db2`.
    ```solidity
    function getECDSAKeyRegistrationMessageHash(address operator, OperatorSet memory operatorSet, address keyAddress) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getECDSAKeyRegistrationMessageHashCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub keyAddress: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getECDSAKeyRegistrationMessageHash(address,(address,uint32),address)`](getECDSAKeyRegistrationMessageHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getECDSAKeyRegistrationMessageHashReturn {
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
                alloy::sol_types::sol_data::Address,
                OperatorSet,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getECDSAKeyRegistrationMessageHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: getECDSAKeyRegistrationMessageHashCall) -> Self {
                    (value.operator, value.operatorSet, value.keyAddress)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getECDSAKeyRegistrationMessageHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSet: tuple.1,
                        keyAddress: tuple.2,
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
            impl ::core::convert::From<getECDSAKeyRegistrationMessageHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getECDSAKeyRegistrationMessageHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getECDSAKeyRegistrationMessageHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getECDSAKeyRegistrationMessageHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                OperatorSet,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getECDSAKeyRegistrationMessageHash(address,(address,uint32),address)";
            const SELECTOR: [u8; 4] = [217u8, 241u8, 45u8, 178u8];
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
                        &self.operator,
                    ),
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.keyAddress,
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
                        let r: getECDSAKeyRegistrationMessageHashReturn = r.into();
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
                    let r: getECDSAKeyRegistrationMessageHashReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getKeyHash((address,uint32),address)` and selector `0xea194e2e`.
    ```solidity
    function getKeyHash(OperatorSet memory operatorSet, address operator) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyHashCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getKeyHash((address,uint32),address)`](getKeyHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getKeyHashReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getKeyHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: getKeyHashCall) -> Self {
                    (value.operatorSet, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKeyHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        operator: tuple.1,
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
            impl ::core::convert::From<getKeyHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getKeyHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getKeyHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getKeyHashCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getKeyHash((address,uint32),address)";
            const SELECTOR: [u8; 4] = [234u8, 25u8, 78u8, 46u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
                        let r: getKeyHashReturn = r.into();
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
                    let r: getKeyHashReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperatorFromSigningKey((address,uint32),bytes)` and selector `0x8256909c`.
    ```solidity
    function getOperatorFromSigningKey(OperatorSet memory operatorSet, bytes memory keyData) external view returns (address, bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorFromSigningKeyCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub keyData: alloy::sol_types::private::Bytes,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorFromSigningKey((address,uint32),bytes)`](getOperatorFromSigningKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorFromSigningKeyReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: bool,
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
            type UnderlyingSolTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Bytes);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorFromSigningKeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromSigningKeyCall) -> Self {
                    (value.operatorSet, value.keyData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorFromSigningKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        keyData: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorFromSigningKeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorFromSigningKeyReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorFromSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        impl getOperatorFromSigningKeyReturn {
            fn _tokenize(
                &self,
            ) -> <getOperatorFromSigningKeyCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorFromSigningKeyCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Bytes);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorFromSigningKeyReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorFromSigningKey((address,uint32),bytes)";
            const SELECTOR: [u8; 4] = [130u8, 86u8, 144u8, 156u8];
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.keyData,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                getOperatorFromSigningKeyReturn::_tokenize(ret)
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
    /**Function with signature `getOperatorSetCurveType((address,uint32))` and selector `0x7cffe48c`.
    ```solidity
    function getOperatorSetCurveType(OperatorSet memory operatorSet) external view returns (IKeyRegistrarTypes.CurveType);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetCurveTypeCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorSetCurveType((address,uint32))`](getOperatorSetCurveTypeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetCurveTypeReturn {
        #[allow(missing_docs)]
        pub _0: <IKeyRegistrarTypes::CurveType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorSetCurveTypeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetCurveTypeCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetCurveTypeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                    }
                }
            }
        }
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
            impl ::core::convert::From<getOperatorSetCurveTypeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetCurveTypeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetCurveTypeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSetCurveTypeCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <IKeyRegistrarTypes::CurveType as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (IKeyRegistrarTypes::CurveType,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSetCurveType((address,uint32))";
            const SELECTOR: [u8; 4] = [124u8, 255u8, 228u8, 140u8];
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
                (<IKeyRegistrarTypes::CurveType as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getOperatorSetCurveTypeReturn = r.into();
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
                    let r: getOperatorSetCurveTypeReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isKeyGloballyRegistered(bytes32)` and selector `0xdab42d7e`.
    ```solidity
    function isKeyGloballyRegistered(bytes32 keyHash) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isKeyGloballyRegisteredCall {
        #[allow(missing_docs)]
        pub keyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isKeyGloballyRegistered(bytes32)`](isKeyGloballyRegisteredCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isKeyGloballyRegisteredReturn {
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
            impl ::core::convert::From<isKeyGloballyRegisteredCall> for UnderlyingRustTuple<'_> {
                fn from(value: isKeyGloballyRegisteredCall) -> Self {
                    (value.keyHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isKeyGloballyRegisteredCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { keyHash: tuple.0 }
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
            impl ::core::convert::From<isKeyGloballyRegisteredReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isKeyGloballyRegisteredReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isKeyGloballyRegisteredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isKeyGloballyRegisteredCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isKeyGloballyRegistered(bytes32)";
            const SELECTOR: [u8; 4] = [218u8, 180u8, 45u8, 126u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.keyHash),
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
                        let r: isKeyGloballyRegisteredReturn = r.into();
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
                    let r: isKeyGloballyRegisteredReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `isRegistered((address,uint32),address)` and selector `0xbd30a0b9`.
    ```solidity
    function isRegistered(OperatorSet memory operatorSet, address operator) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRegisteredCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`isRegistered((address,uint32),address)`](isRegisteredCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRegisteredReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<isRegisteredCall> for UnderlyingRustTuple<'_> {
                fn from(value: isRegisteredCall) -> Self {
                    (value.operatorSet, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRegisteredCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        operator: tuple.1,
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
            impl ::core::convert::From<isRegisteredReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isRegisteredReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRegisteredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRegisteredCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isRegistered((address,uint32),address)";
            const SELECTOR: [u8; 4] = [189u8, 48u8, 160u8, 185u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
                        let r: isRegisteredReturn = r.into();
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
                    let r: isRegisteredReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `registerKey(address,(address,uint32),bytes,bytes)` and selector `0xd40cda16`.
    ```solidity
    function registerKey(address operator, OperatorSet memory operatorSet, bytes memory pubkey, bytes memory signature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerKeyCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pubkey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerKey(address,(address,uint32),bytes,bytes)`](registerKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerKeyReturn {}
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerKeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerKeyCall) -> Self {
                    (
                        value.operator,
                        value.operatorSet,
                        value.pubkey,
                        value.signature,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSet: tuple.1,
                        pubkey: tuple.2,
                        signature: tuple.3,
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
            impl ::core::convert::From<registerKeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerKeyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl registerKeyReturn {
            fn _tokenize(&self) -> <registerKeyCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerKeyCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                OperatorSet,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerKeyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerKey(address,(address,uint32),bytes,bytes)";
            const SELECTOR: [u8; 4] = [212u8, 12u8, 218u8, 22u8];
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
                        &self.operator,
                    ),
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.pubkey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                registerKeyReturn::_tokenize(ret)
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
    ///Container for all the [`IKeyRegistrar`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum IKeyRegistrarCalls {
        #[allow(missing_docs)]
        configureOperatorSet(configureOperatorSetCall),
        #[allow(missing_docs)]
        deregisterKey(deregisterKeyCall),
        #[allow(missing_docs)]
        encodeBN254KeyData(encodeBN254KeyDataCall),
        #[allow(missing_docs)]
        getBN254Key(getBN254KeyCall),
        #[allow(missing_docs)]
        getBN254KeyRegistrationMessageHash(getBN254KeyRegistrationMessageHashCall),
        #[allow(missing_docs)]
        getECDSAAddress(getECDSAAddressCall),
        #[allow(missing_docs)]
        getECDSAKey(getECDSAKeyCall),
        #[allow(missing_docs)]
        getECDSAKeyRegistrationMessageHash(getECDSAKeyRegistrationMessageHashCall),
        #[allow(missing_docs)]
        getKeyHash(getKeyHashCall),
        #[allow(missing_docs)]
        getOperatorFromSigningKey(getOperatorFromSigningKeyCall),
        #[allow(missing_docs)]
        getOperatorSetCurveType(getOperatorSetCurveTypeCall),
        #[allow(missing_docs)]
        isKeyGloballyRegistered(isKeyGloballyRegisteredCall),
        #[allow(missing_docs)]
        isRegistered(isRegisteredCall),
        #[allow(missing_docs)]
        registerKey(registerKeyCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    #[automatically_derived]
    impl IKeyRegistrarCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [59u8, 50u8, 167u8, 189u8],
            [80u8, 67u8, 90u8, 221u8],
            [84u8, 253u8, 77u8, 80u8],
            [118u8, 144u8, 227u8, 149u8],
            [124u8, 255u8, 228u8, 140u8],
            [130u8, 86u8, 144u8, 156u8],
            [135u8, 171u8, 134u8, 244u8],
            [154u8, 67u8, 227u8, 251u8],
            [170u8, 22u8, 92u8, 48u8],
            [189u8, 48u8, 160u8, 185u8],
            [212u8, 12u8, 218u8, 22u8],
            [217u8, 241u8, 45u8, 178u8],
            [218u8, 180u8, 45u8, 126u8],
            [234u8, 13u8, 129u8, 73u8],
            [234u8, 25u8, 78u8, 46u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IKeyRegistrarCalls {
        const NAME: &'static str = "IKeyRegistrarCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 15usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::configureOperatorSet(_) => {
                    <configureOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterKey(_) => <deregisterKeyCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::encodeBN254KeyData(_) => {
                    <encodeBN254KeyDataCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBN254Key(_) => <getBN254KeyCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getBN254KeyRegistrationMessageHash(_) => {
                    <getBN254KeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getECDSAAddress(_) => {
                    <getECDSAAddressCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getECDSAKey(_) => <getECDSAKeyCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getECDSAKeyRegistrationMessageHash(_) => {
                    <getECDSAKeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getKeyHash(_) => <getKeyHashCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getOperatorFromSigningKey(_) => {
                    <getOperatorFromSigningKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSetCurveType(_) => {
                    <getOperatorSetCurveTypeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isKeyGloballyRegistered(_) => {
                    <isKeyGloballyRegisteredCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isRegistered(_) => <isRegisteredCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::registerKey(_) => <registerKeyCall as alloy_sol_types::SolCall>::SELECTOR,
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
            static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls>] = &[
                {
                    fn getECDSAAddress(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getECDSAAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IKeyRegistrarCalls::getECDSAAddress)
                    }
                    getECDSAAddress
                },
                {
                    fn encodeBN254KeyData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <encodeBN254KeyDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IKeyRegistrarCalls::encodeBN254KeyData)
                    }
                    encodeBN254KeyData
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IKeyRegistrarCalls::version)
                    }
                    version
                },
                {
                    fn getBN254KeyRegistrationMessageHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getBN254KeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IKeyRegistrarCalls::getBN254KeyRegistrationMessageHash)
                    }
                    getBN254KeyRegistrationMessageHash
                },
                {
                    fn getOperatorSetCurveType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getOperatorSetCurveTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(IKeyRegistrarCalls::getOperatorSetCurveType)
                    }
                    getOperatorSetCurveType
                },
                {
                    fn getOperatorFromSigningKey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getOperatorFromSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(IKeyRegistrarCalls::getOperatorFromSigningKey)
                    }
                    getOperatorFromSigningKey
                },
                {
                    fn deregisterKey(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <deregisterKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IKeyRegistrarCalls::deregisterKey)
                    }
                    deregisterKey
                },
                {
                    fn getBN254Key(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getBN254KeyCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IKeyRegistrarCalls::getBN254Key)
                    }
                    getBN254Key
                },
                {
                    fn getECDSAKey(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getECDSAKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IKeyRegistrarCalls::getECDSAKey)
                    }
                    getECDSAKey
                },
                {
                    fn isRegistered(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <isRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IKeyRegistrarCalls::isRegistered)
                    }
                    isRegistered
                },
                {
                    fn registerKey(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <registerKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IKeyRegistrarCalls::registerKey)
                    }
                    registerKey
                },
                {
                    fn getECDSAKeyRegistrationMessageHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getECDSAKeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(IKeyRegistrarCalls::getECDSAKeyRegistrationMessageHash)
                    }
                    getECDSAKeyRegistrationMessageHash
                },
                {
                    fn isKeyGloballyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <isKeyGloballyRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(IKeyRegistrarCalls::isKeyGloballyRegistered)
                    }
                    isKeyGloballyRegistered
                },
                {
                    fn configureOperatorSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <configureOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IKeyRegistrarCalls::configureOperatorSet)
                    }
                    configureOperatorSet
                },
                {
                    fn getKeyHash(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getKeyHashCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(IKeyRegistrarCalls::getKeyHash)
                    }
                    getKeyHash
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
                -> alloy_sol_types::Result<IKeyRegistrarCalls>] = &[
                {
                    fn getECDSAAddress(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getECDSAAddressCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IKeyRegistrarCalls::getECDSAAddress)
                    }
                    getECDSAAddress
                },
                {
                    fn encodeBN254KeyData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <encodeBN254KeyDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IKeyRegistrarCalls::encodeBN254KeyData)
                    }
                    encodeBN254KeyData
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IKeyRegistrarCalls::version)
                    }
                    version
                },
                {
                    fn getBN254KeyRegistrationMessageHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getBN254KeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IKeyRegistrarCalls::getBN254KeyRegistrationMessageHash)
                    }
                    getBN254KeyRegistrationMessageHash
                },
                {
                    fn getOperatorSetCurveType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getOperatorSetCurveTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IKeyRegistrarCalls::getOperatorSetCurveType)
                    }
                    getOperatorSetCurveType
                },
                {
                    fn getOperatorFromSigningKey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getOperatorFromSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IKeyRegistrarCalls::getOperatorFromSigningKey)
                    }
                    getOperatorFromSigningKey
                },
                {
                    fn deregisterKey(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <deregisterKeyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IKeyRegistrarCalls::deregisterKey)
                    }
                    deregisterKey
                },
                {
                    fn getBN254Key(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getBN254KeyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IKeyRegistrarCalls::getBN254Key)
                    }
                    getBN254Key
                },
                {
                    fn getECDSAKey(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getECDSAKeyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IKeyRegistrarCalls::getECDSAKey)
                    }
                    getECDSAKey
                },
                {
                    fn isRegistered(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <isRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IKeyRegistrarCalls::isRegistered)
                    }
                    isRegistered
                },
                {
                    fn registerKey(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <registerKeyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IKeyRegistrarCalls::registerKey)
                    }
                    registerKey
                },
                {
                    fn getECDSAKeyRegistrationMessageHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getECDSAKeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IKeyRegistrarCalls::getECDSAKeyRegistrationMessageHash)
                    }
                    getECDSAKeyRegistrationMessageHash
                },
                {
                    fn isKeyGloballyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <isKeyGloballyRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IKeyRegistrarCalls::isKeyGloballyRegistered)
                    }
                    isKeyGloballyRegistered
                },
                {
                    fn configureOperatorSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <configureOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IKeyRegistrarCalls::configureOperatorSet)
                    }
                    configureOperatorSet
                },
                {
                    fn getKeyHash(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarCalls> {
                        <getKeyHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(IKeyRegistrarCalls::getKeyHash)
                    }
                    getKeyHash
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
                Self::configureOperatorSet(inner) => {
                    <configureOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterKey(inner) => {
                    <deregisterKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::encodeBN254KeyData(inner) => {
                    <encodeBN254KeyDataCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBN254Key(inner) => {
                    <getBN254KeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBN254KeyRegistrationMessageHash(inner) => {
                    <getBN254KeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getECDSAAddress(inner) => {
                    <getECDSAAddressCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getECDSAKey(inner) => {
                    <getECDSAKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getECDSAKeyRegistrationMessageHash(inner) => {
                    <getECDSAKeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getKeyHash(inner) => {
                    <getKeyHashCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getOperatorFromSigningKey(inner) => {
                    <getOperatorFromSigningKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSetCurveType(inner) => {
                    <getOperatorSetCurveTypeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isKeyGloballyRegistered(inner) => {
                    <isKeyGloballyRegisteredCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isRegistered(inner) => {
                    <isRegisteredCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerKey(inner) => {
                    <registerKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::configureOperatorSet(inner) => {
                    <configureOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterKey(inner) => {
                    <deregisterKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::encodeBN254KeyData(inner) => {
                    <encodeBN254KeyDataCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBN254Key(inner) => {
                    <getBN254KeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBN254KeyRegistrationMessageHash(inner) => {
                    <getBN254KeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getECDSAAddress(inner) => {
                    <getECDSAAddressCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getECDSAKey(inner) => {
                    <getECDSAKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getECDSAKeyRegistrationMessageHash(inner) => {
                    <getECDSAKeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getKeyHash(inner) => {
                    <getKeyHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorFromSigningKey(inner) => {
                    <getOperatorFromSigningKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorSetCurveType(inner) => {
                    <getOperatorSetCurveTypeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isKeyGloballyRegistered(inner) => {
                    <isKeyGloballyRegisteredCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isRegistered(inner) => {
                    <isRegisteredCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerKey(inner) => {
                    <registerKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`IKeyRegistrar`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum IKeyRegistrarErrors {
        #[allow(missing_docs)]
        ConfigurationAlreadySet(ConfigurationAlreadySet),
        #[allow(missing_docs)]
        InvalidCurveType(InvalidCurveType),
        #[allow(missing_docs)]
        InvalidKeyFormat(InvalidKeyFormat),
        #[allow(missing_docs)]
        InvalidKeypair(InvalidKeypair),
        #[allow(missing_docs)]
        KeyAlreadyRegistered(KeyAlreadyRegistered),
        #[allow(missing_docs)]
        KeyNotFound(KeyNotFound),
        #[allow(missing_docs)]
        OperatorAlreadyRegistered(OperatorAlreadyRegistered),
        #[allow(missing_docs)]
        OperatorSetNotConfigured(OperatorSetNotConfigured),
        #[allow(missing_docs)]
        OperatorStillSlashable(OperatorStillSlashable),
        #[allow(missing_docs)]
        ZeroAddress(ZeroAddress),
        #[allow(missing_docs)]
        ZeroPubkey(ZeroPubkey),
    }
    #[automatically_derived]
    impl IKeyRegistrarErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 129u8, 240u8, 159u8],
            [16u8, 112u8, 40u8, 121u8],
            [24u8, 247u8, 132u8, 2u8],
            [27u8, 86u8, 166u8, 139u8],
            [46u8, 64u8, 225u8, 135u8],
            [66u8, 238u8, 104u8, 181u8],
            [73u8, 53u8, 80u8, 95u8],
            [185u8, 166u8, 32u8, 218u8],
            [209u8, 9u8, 17u8, 129u8],
            [217u8, 46u8, 35u8, 61u8],
            [253u8, 234u8, 124u8, 9u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for IKeyRegistrarErrors {
        const NAME: &'static str = "IKeyRegistrarErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 11usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ConfigurationAlreadySet(_) => {
                    <ConfigurationAlreadySet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCurveType(_) => {
                    <InvalidCurveType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidKeyFormat(_) => {
                    <InvalidKeyFormat as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidKeypair(_) => <InvalidKeypair as alloy_sol_types::SolError>::SELECTOR,
                Self::KeyAlreadyRegistered(_) => {
                    <KeyAlreadyRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeyNotFound(_) => <KeyNotFound as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorAlreadyRegistered(_) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorSetNotConfigured(_) => {
                    <OperatorSetNotConfigured as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorStillSlashable(_) => {
                    <OperatorStillSlashable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ZeroAddress(_) => <ZeroAddress as alloy_sol_types::SolError>::SELECTOR,
                Self::ZeroPubkey(_) => <ZeroPubkey as alloy_sol_types::SolError>::SELECTOR,
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
            static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<IKeyRegistrarErrors>] = &[
                {
                    fn ConfigurationAlreadySet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <ConfigurationAlreadySet as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IKeyRegistrarErrors::ConfigurationAlreadySet)
                    }
                    ConfigurationAlreadySet
                },
                {
                    fn OperatorStillSlashable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <OperatorStillSlashable as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IKeyRegistrarErrors::OperatorStillSlashable)
                    }
                    OperatorStillSlashable
                },
                {
                    fn KeyAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <KeyAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IKeyRegistrarErrors::KeyAlreadyRegistered)
                    }
                    KeyAlreadyRegistered
                },
                {
                    fn InvalidKeypair(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <InvalidKeypair as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IKeyRegistrarErrors::InvalidKeypair)
                    }
                    InvalidKeypair
                },
                {
                    fn KeyNotFound(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <KeyNotFound as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IKeyRegistrarErrors::KeyNotFound)
                    }
                    KeyNotFound
                },
                {
                    fn OperatorAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(IKeyRegistrarErrors::OperatorAlreadyRegistered)
                    }
                    OperatorAlreadyRegistered
                },
                {
                    fn ZeroPubkey(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <ZeroPubkey as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IKeyRegistrarErrors::ZeroPubkey)
                    }
                    ZeroPubkey
                },
                {
                    fn OperatorSetNotConfigured(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <OperatorSetNotConfigured as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(IKeyRegistrarErrors::OperatorSetNotConfigured)
                    }
                    OperatorSetNotConfigured
                },
                {
                    fn InvalidKeyFormat(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <InvalidKeyFormat as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IKeyRegistrarErrors::InvalidKeyFormat)
                    }
                    InvalidKeyFormat
                },
                {
                    fn ZeroAddress(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IKeyRegistrarErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn InvalidCurveType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <InvalidCurveType as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(IKeyRegistrarErrors::InvalidCurveType)
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
            )
                -> alloy_sol_types::Result<IKeyRegistrarErrors>] = &[
                {
                    fn ConfigurationAlreadySet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <ConfigurationAlreadySet as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IKeyRegistrarErrors::ConfigurationAlreadySet)
                    }
                    ConfigurationAlreadySet
                },
                {
                    fn OperatorStillSlashable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <OperatorStillSlashable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IKeyRegistrarErrors::OperatorStillSlashable)
                    }
                    OperatorStillSlashable
                },
                {
                    fn KeyAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <KeyAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IKeyRegistrarErrors::KeyAlreadyRegistered)
                    }
                    KeyAlreadyRegistered
                },
                {
                    fn InvalidKeypair(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <InvalidKeypair as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(IKeyRegistrarErrors::InvalidKeypair)
                    }
                    InvalidKeypair
                },
                {
                    fn KeyNotFound(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <KeyNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(IKeyRegistrarErrors::KeyNotFound)
                    }
                    KeyNotFound
                },
                {
                    fn OperatorAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IKeyRegistrarErrors::OperatorAlreadyRegistered)
                    }
                    OperatorAlreadyRegistered
                },
                {
                    fn ZeroPubkey(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <ZeroPubkey as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(IKeyRegistrarErrors::ZeroPubkey)
                    }
                    ZeroPubkey
                },
                {
                    fn OperatorSetNotConfigured(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <OperatorSetNotConfigured as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(IKeyRegistrarErrors::OperatorSetNotConfigured)
                    }
                    OperatorSetNotConfigured
                },
                {
                    fn InvalidKeyFormat(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <InvalidKeyFormat as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IKeyRegistrarErrors::InvalidKeyFormat)
                    }
                    InvalidKeyFormat
                },
                {
                    fn ZeroAddress(data: &[u8]) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(IKeyRegistrarErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn InvalidCurveType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<IKeyRegistrarErrors> {
                        <InvalidCurveType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(IKeyRegistrarErrors::InvalidCurveType)
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
                Self::ConfigurationAlreadySet(inner) => {
                    <ConfigurationAlreadySet as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidCurveType(inner) => {
                    <InvalidCurveType as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidKeyFormat(inner) => {
                    <InvalidKeyFormat as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidKeypair(inner) => {
                    <InvalidKeypair as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::KeyAlreadyRegistered(inner) => {
                    <KeyAlreadyRegistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::KeyNotFound(inner) => {
                    <KeyNotFound as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorSetNotConfigured(inner) => {
                    <OperatorSetNotConfigured as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorStillSlashable(inner) => {
                    <OperatorStillSlashable as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroAddress(inner) => {
                    <ZeroAddress as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ZeroPubkey(inner) => {
                    <ZeroPubkey as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ConfigurationAlreadySet(inner) => {
                    <ConfigurationAlreadySet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidCurveType(inner) => {
                    <InvalidCurveType as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidKeyFormat(inner) => {
                    <InvalidKeyFormat as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidKeypair(inner) => {
                    <InvalidKeypair as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::KeyAlreadyRegistered(inner) => {
                    <KeyAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::KeyNotFound(inner) => {
                    <KeyNotFound as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OperatorSetNotConfigured(inner) => {
                    <OperatorSetNotConfigured as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OperatorStillSlashable(inner) => {
                    <OperatorStillSlashable as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::ZeroAddress(inner) => {
                    <ZeroAddress as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ZeroPubkey(inner) => {
                    <ZeroPubkey as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`IKeyRegistrar`](self) events.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum IKeyRegistrarEvents {
        #[allow(missing_docs)]
        AggregateBN254KeyUpdated(AggregateBN254KeyUpdated),
        #[allow(missing_docs)]
        KeyDeregistered(KeyDeregistered),
        #[allow(missing_docs)]
        KeyRegistered(KeyRegistered),
        #[allow(missing_docs)]
        OperatorSetConfigured(OperatorSetConfigured),
    }
    #[automatically_derived]
    impl IKeyRegistrarEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                18u8, 1u8, 206u8, 12u8, 94u8, 87u8, 113u8, 17u8, 188u8, 233u8, 30u8, 144u8, 127u8,
                217u8, 156u8, 177u8, 131u8, 218u8, 94u8, 220u8, 30u8, 63u8, 182u8, 80u8, 202u8,
                64u8, 118u8, 158u8, 78u8, 145u8, 118u8, 221u8,
            ],
            [
                40u8, 211u8, 195u8, 206u8, 228u8, 148u8, 120u8, 236u8, 111u8, 210u8, 25u8, 207u8,
                214u8, 133u8, 205u8, 21u8, 205u8, 1u8, 217u8, 92u8, 171u8, 246u8, 155u8, 75u8,
                123u8, 87u8, 249u8, 234u8, 163u8, 235u8, 100u8, 66u8,
            ],
            [
                178u8, 38u8, 108u8, 177u8, 24u8, 229u8, 112u8, 149u8, 252u8, 219u8, 237u8, 178u8,
                77u8, 171u8, 217u8, 252u8, 159u8, 81u8, 39u8, 226u8, 219u8, 237u8, 246u8, 44u8,
                230u8, 238u8, 113u8, 105u8, 111u8, 184u8, 182u8, 231u8,
            ],
            [
                223u8, 162u8, 245u8, 158u8, 85u8, 116u8, 123u8, 166u8, 65u8, 251u8, 223u8, 244u8,
                235u8, 120u8, 87u8, 125u8, 232u8, 120u8, 157u8, 96u8, 89u8, 32u8, 213u8, 190u8,
                74u8, 116u8, 238u8, 58u8, 100u8, 112u8, 209u8, 209u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for IKeyRegistrarEvents {
        const NAME: &'static str = "IKeyRegistrarEvents";
        const COUNT: usize = 4usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<AggregateBN254KeyUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AggregateBN254KeyUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::AggregateBN254KeyUpdated)
                }
                Some(<KeyDeregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <KeyDeregistered as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::KeyDeregistered)
                }
                Some(<KeyRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <KeyRegistered as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::KeyRegistered)
                }
                Some(<OperatorSetConfigured as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSetConfigured as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::OperatorSetConfigured)
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
    impl alloy_sol_types::private::IntoLogData for IKeyRegistrarEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AggregateBN254KeyUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeyDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::KeyRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetConfigured(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AggregateBN254KeyUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeyDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::KeyRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetConfigured(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IKeyRegistrar`](self) contract instance.

    See the [wrapper's documentation](`IKeyRegistrarInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IKeyRegistrarInstance<P, N> {
        IKeyRegistrarInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
        provider: P,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<IKeyRegistrarInstance<P, N>>>
    {
        IKeyRegistrarInstance::<P, N>::deploy(provider)
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
        IKeyRegistrarInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`IKeyRegistrar`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IKeyRegistrar`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IKeyRegistrarInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for IKeyRegistrarInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IKeyRegistrarInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IKeyRegistrarInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`IKeyRegistrar`](self) contract instance.

        See the [wrapper's documentation](`IKeyRegistrarInstance`) for more details.*/
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
        pub async fn deploy(provider: P) -> alloy_contract::Result<IKeyRegistrarInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> IKeyRegistrarInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IKeyRegistrarInstance<P, N> {
            IKeyRegistrarInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        IKeyRegistrarInstance<P, N>
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
        ///Creates a new call builder for the [`configureOperatorSet`] function.
        pub fn configureOperatorSet(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            curveType: <IKeyRegistrarTypes::CurveType as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, configureOperatorSetCall, N> {
            self.call_builder(&configureOperatorSetCall {
                operatorSet,
                curveType,
            })
        }
        ///Creates a new call builder for the [`deregisterKey`] function.
        pub fn deregisterKey(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, deregisterKeyCall, N> {
            self.call_builder(&deregisterKeyCall {
                operator,
                operatorSet,
            })
        }
        ///Creates a new call builder for the [`encodeBN254KeyData`] function.
        pub fn encodeBN254KeyData(
            &self,
            g1Point: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            g2Point: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, encodeBN254KeyDataCall, N> {
            self.call_builder(&encodeBN254KeyDataCall { g1Point, g2Point })
        }
        ///Creates a new call builder for the [`getBN254Key`] function.
        pub fn getBN254Key(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getBN254KeyCall, N> {
            self.call_builder(&getBN254KeyCall {
                operatorSet,
                operator,
            })
        }
        ///Creates a new call builder for the [`getBN254KeyRegistrationMessageHash`] function.
        pub fn getBN254KeyRegistrationMessageHash(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            keyData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, getBN254KeyRegistrationMessageHashCall, N> {
            self.call_builder(&getBN254KeyRegistrationMessageHashCall {
                operator,
                operatorSet,
                keyData,
            })
        }
        ///Creates a new call builder for the [`getECDSAAddress`] function.
        pub fn getECDSAAddress(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getECDSAAddressCall, N> {
            self.call_builder(&getECDSAAddressCall {
                operatorSet,
                operator,
            })
        }
        ///Creates a new call builder for the [`getECDSAKey`] function.
        pub fn getECDSAKey(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getECDSAKeyCall, N> {
            self.call_builder(&getECDSAKeyCall {
                operatorSet,
                operator,
            })
        }
        ///Creates a new call builder for the [`getECDSAKeyRegistrationMessageHash`] function.
        pub fn getECDSAKeyRegistrationMessageHash(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            keyAddress: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getECDSAKeyRegistrationMessageHashCall, N> {
            self.call_builder(&getECDSAKeyRegistrationMessageHashCall {
                operator,
                operatorSet,
                keyAddress,
            })
        }
        ///Creates a new call builder for the [`getKeyHash`] function.
        pub fn getKeyHash(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, getKeyHashCall, N> {
            self.call_builder(&getKeyHashCall {
                operatorSet,
                operator,
            })
        }
        ///Creates a new call builder for the [`getOperatorFromSigningKey`] function.
        pub fn getOperatorFromSigningKey(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            keyData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorFromSigningKeyCall, N> {
            self.call_builder(&getOperatorFromSigningKeyCall {
                operatorSet,
                keyData,
            })
        }
        ///Creates a new call builder for the [`getOperatorSetCurveType`] function.
        pub fn getOperatorSetCurveType(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorSetCurveTypeCall, N> {
            self.call_builder(&getOperatorSetCurveTypeCall { operatorSet })
        }
        ///Creates a new call builder for the [`isKeyGloballyRegistered`] function.
        pub fn isKeyGloballyRegistered(
            &self,
            keyHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<&P, isKeyGloballyRegisteredCall, N> {
            self.call_builder(&isKeyGloballyRegisteredCall { keyHash })
        }
        ///Creates a new call builder for the [`isRegistered`] function.
        pub fn isRegistered(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, isRegisteredCall, N> {
            self.call_builder(&isRegisteredCall {
                operatorSet,
                operator,
            })
        }
        ///Creates a new call builder for the [`registerKey`] function.
        pub fn registerKey(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            pubkey: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, registerKeyCall, N> {
            self.call_builder(&registerKeyCall {
                operator,
                operatorSet,
                pubkey,
                signature,
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
        IKeyRegistrarInstance<P, N>
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
        ///Creates a new event filter for the [`AggregateBN254KeyUpdated`] event.
        pub fn AggregateBN254KeyUpdated_filter(
            &self,
        ) -> alloy_contract::Event<&P, AggregateBN254KeyUpdated, N> {
            self.event_filter::<AggregateBN254KeyUpdated>()
        }
        ///Creates a new event filter for the [`KeyDeregistered`] event.
        pub fn KeyDeregistered_filter(&self) -> alloy_contract::Event<&P, KeyDeregistered, N> {
            self.event_filter::<KeyDeregistered>()
        }
        ///Creates a new event filter for the [`KeyRegistered`] event.
        pub fn KeyRegistered_filter(&self) -> alloy_contract::Event<&P, KeyRegistered, N> {
            self.event_filter::<KeyRegistered>()
        }
        ///Creates a new event filter for the [`OperatorSetConfigured`] event.
        pub fn OperatorSetConfigured_filter(
            &self,
        ) -> alloy_contract::Event<&P, OperatorSetConfigured, N> {
            self.event_filter::<OperatorSetConfigured>()
        }
    }
}
