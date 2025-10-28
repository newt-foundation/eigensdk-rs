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

interface KeyRegistrar {
    struct OperatorSet {
        address avs;
        uint32 id;
    }

    error ConfigurationAlreadySet();
    error ECAddFailed();
    error ECMulFailed();
    error ECPairingFailed();
    error ExpModFailed();
    error InvalidCurveType();
    error InvalidKeyFormat();
    error InvalidKeypair();
    error InvalidPermissions();
    error InvalidShortString();
    error InvalidSignature();
    error KeyAlreadyRegistered();
    error KeyNotFound(OperatorSet operatorSet, address operator);
    error OperatorAlreadyRegistered();
    error OperatorSetNotConfigured();
    error OperatorStillSlashable(OperatorSet operatorSet, address operator);
    error SignatureExpired();
    error StringTooLong(string str);
    error ZeroAddress();
    error ZeroPubkey();

    event AggregateBN254KeyUpdated(OperatorSet operatorSet, BN254.G1Point newAggregateKey);
    event KeyDeregistered(OperatorSet operatorSet, address indexed operator, IKeyRegistrarTypes.CurveType curveType);
    event KeyRegistered(OperatorSet operatorSet, address indexed operator, IKeyRegistrarTypes.CurveType curveType, bytes pubkey);
    event OperatorSetConfigured(OperatorSet operatorSet, IKeyRegistrarTypes.CurveType curveType);

    constructor(address _permissionController, address _allocationManager, string _version);

    function BN254_KEY_REGISTRATION_TYPEHASH() external view returns (bytes32);
    function ECDSA_KEY_REGISTRATION_TYPEHASH() external view returns (bytes32);
    function allocationManager() external view returns (address);
    function configureOperatorSet(OperatorSet memory operatorSet, IKeyRegistrarTypes.CurveType curveType) external;
    function deregisterKey(address operator, OperatorSet memory operatorSet) external;
    function domainSeparator() external view returns (bytes32);
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
    function permissionController() external view returns (address);
    function registerKey(address operator, OperatorSet memory operatorSet, bytes memory keyData, bytes memory signature) external;
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
        "name": "_permissionController",
        "type": "address",
        "internalType": "contract IPermissionController"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "contract IAllocationManager"
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
    "name": "BN254_KEY_REGISTRATION_TYPEHASH",
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
    "name": "ECDSA_KEY_REGISTRATION_TYPEHASH",
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
    "name": "allocationManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAllocationManager"
      }
    ],
    "stateMutability": "view"
  },
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
    "name": "permissionController",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IPermissionController"
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
        "name": "keyData",
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
    "name": "InvalidPermissions",
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
    "name": "SignatureExpired",
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
pub mod KeyRegistrar {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e060405234801561000f575f5ffd5b5060405161314a38038061314a83398101604081905261002e916100cb565b6001600160a01b03808316608052831660a052808061004c8161005a565b60c052506101fc9350505050565b5f5f829050601f8151111561008d578260405163305a27a960e01b815260040161008491906101a1565b60405180910390fd5b8051610098826101d6565b179392505050565b6001600160a01b03811681146100b4575f5ffd5b50565b634e487b7160e01b5f52604160045260245ffd5b5f5f5f606084860312156100dd575f5ffd5b83516100e8816100a0565b60208501519093506100f9816100a0565b60408501519092506001600160401b03811115610114575f5ffd5b8401601f81018613610124575f5ffd5b80516001600160401b0381111561013d5761013d6100b7565b604051601f8201601f19908116603f011681016001600160401b038111828210171561016b5761016b6100b7565b604052818152828201602001881015610182575f5ffd5b8160208401602083015e5f602083830101528093505050509250925092565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b805160208083015191908110156101f6575f198160200360031b1b821691505b50919050565b60805160a05160c051612f0f61023b5f395f81816103fa01526116f001525f8181610195015261129f01525f818161033b015261061d0152612f0f5ff3fe608060405234801561000f575f5ffd5b5060043610610127575f3560e01c8063aa165c30116100a9578063d9f12db21161006e578063d9f12db214610370578063dab42d7e14610383578063ea0d8149146103a5578063ea194e2e146103b8578063f698da25146103cb575f5ffd5b8063aa165c30146102d9578063b05c8f6d146102ec578063bd30a0b914610313578063ca8aa7c714610336578063d40cda161461035d575f5ffd5b80637690e395116100ef5780637690e3951461023e5780637cffe48c146102515780638256909c1461027157806387ab86f4146102a35780639a43e3fb146102b8575f5ffd5b8063166aa1271461012b5780633b32a7bd146101655780634657e26a1461019057806350435add146101b757806354fd4d5014610236575b5f5ffd5b6101527f991b0a3376ce87f8ecc5d70962279ac09cdce934e8b5b9683e73c8ff087c7f8181565b6040519081526020015b60405180910390f35b61017861017336600461259c565b6103d3565b6040516001600160a01b03909116815260200161015c565b6101787f000000000000000000000000000000000000000000000000000000000000000081565b6102296101c536600461261d565b8151602080840151835180519083015185840151805190850151604080519687019790975295850193909352606084810192909252608084015260a083019190915260c082019290925260e001604051602081830303815290604052905092915050565b60405161015c91906126c8565b6102296103f3565b61015261024c366004612718565b610423565b61026461025f366004612776565b6104cb565b60405161015c91906127c4565b61028461027f3660046127d2565b6104f1565b604080516001600160a01b03909316835290151560208301520161015c565b6102b66102b1366004612878565b6105df565b005b6102cb6102c636600461259c565b6108f0565b60405161015c9291906128c3565b6102296102e736600461259c565b610ad8565b6101527fda86e76deaed01641f80ff5f72c372a038fa5182697aeb967e8b1f9819d58d8181565b61032661032136600461259c565b610c15565b604051901515815260200161015c565b6101787f000000000000000000000000000000000000000000000000000000000000000081565b6102b661036b366004612900565b610c52565b61015261037e366004612991565b610dc3565b6103266103913660046129d2565b5f9081526002602052604090205460ff1690565b6102b66103b33660046129e9565b610e5c565b6101526103c636600461259c565b610f9f565b6101526110c7565b5f6103de8383610ad8565b6103e790612a22565b60601c90505b92915050565b606061041e7f0000000000000000000000000000000000000000000000000000000000000000611180565b905090565b5f5f7fda86e76deaed01641f80ff5f72c372a038fa5182697aeb967e8b1f9819d58d8186865f015187602001518787604051610460929190612a7a565b6040805191829003822060208301969096526001600160a01b039485169082015292909116606083015263ffffffff16608082015260a081019190915260c0016040516020818303038152906040528051906020012090506104c1816111bd565b9695505050505050565b5f60015f6104d884611203565b815260208101919091526040015f205460ff1692915050565b5f5f5f60015f61050087611203565b815260208101919091526040015f9081205460ff169150600182600281111561052b5761052b612790565b0361053d5750835160208501206105ae565b600282600281111561055157610551612790565b03610595575f5f8680602001905181019061056c9190612a89565b60408051808201825283815260209081019283525f9384529151909152902092506105ae915050565b60405163fdea7c0960e01b815260040160405180910390fd5b5f818152600360205260409020546001600160a01b0316806105d08882610c15565b945094505050505b9250929050565b816105e981611261565b6106065760405163932d94f760e01b815260040160405180910390fd5b6040516309a961f360e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690631352c3e6906106549086908690600401612aab565b602060405180830381865afa15801561066f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106939190612ae1565b15828490916106e057604051631070287960e01b815282516001600160a01b03908116600483015260209093015163ffffffff166024820152911660448201526064015b60405180910390fd5b50505f60015f6106ef85611203565b815260208101919091526040015f9081205460ff16915081600281111561071857610718612790565b0361073657604051635cd3106d60e11b815260040160405180910390fd5b5f5f5f61074286611203565b815260208082019290925260409081015f9081206001600160a01b038916825283528190208151808301909252805460ff1615158252600181018054929391929184019161078f90612b00565b80601f01602080910402602001604051908101604052809291908181526020018280546107bb90612b00565b80156108065780601f106107dd57610100808354040283529160200191610806565b820191905f5260205f20905b8154815290600101906020018083116107e957829003601f168201915b5050505050815250509050805f01518486909161085c57604051632e40e18760e01b815282516001600160a01b03908116600483015260209093015163ffffffff166024820152911660448201526064016106d7565b50505f5f61086986611203565b815260208082019290925260409081015f9081206001600160a01b03891682529092528120805460ff19168155906108a460018301826123a1565b5050846001600160a01b03167f28d3c3cee49478ec6fd219cfd685cd15cd01d95cabf69b4b7b57f9eaa3eb644285846040516108e1929190612b38565b60405180910390a25050505050565b604080518082019091525f808252602082015261090b6123db565b5f60015f61091887611203565b815260208101919091526040015f205460ff169050600281600281111561094157610941612790565b1461095f5760405163fdea7c0960e01b815260040160405180910390fd5b5f5f5f61096b88611203565b815260208082019290925260409081015f9081206001600160a01b038916825283528190208151808301909252805460ff161515825260018101805492939192918401916109b890612b00565b80601f01602080910402602001604051908101604052809291908181526020018280546109e490612b00565b8015610a2f5780601f10610a0657610100808354040283529160200191610a2f565b820191905f5260205f20905b815481529060010190602001808311610a1257829003601f168201915b5050505050815250509050805f0151610a845750506040805180820182525f808252602080830182905283518085018552828152808201929092528351808501909452828452830191909152925090506105d8565b5f5f5f5f8460200151806020019051810190610aa09190612baa565b604080518082018252948552602080860194909452805180820190915291825291810191909152909b909a5098505050505050505050565b60605f60015f610ae786611203565b815260208101919091526040015f205460ff1690506001816002811115610b1057610b10612790565b14610b2e5760405163fdea7c0960e01b815260040160405180910390fd5b5f5f5f610b3a87611203565b815260208082019290925260409081015f9081206001600160a01b038816825283528190208151808301909252805460ff16151582526001810180549293919291840191610b8790612b00565b80601f0160208091040260200160405190810160405280929190818152602001828054610bb390612b00565b8015610bfe5780601f10610bd557610100808354040283529160200191610bfe565b820191905f5260205f20905b815481529060010190602001808311610be157829003601f168201915b505050919092525050506020015195945050505050565b5f5f5f610c2185611203565b815260208082019290925260409081015f9081206001600160a01b038616825290925290205460ff16905092915050565b85610c5c81611261565b610c795760405163932d94f760e01b815260040160405180910390fd5b5f60015f610c8689611203565b815260208101919091526040015f9081205460ff169150816002811115610caf57610caf612790565b03610ccd57604051635cd3106d60e11b815260040160405180910390fd5b5f5f610cd889611203565b815260208082019290925260409081015f9081206001600160a01b038c16825290925290205460ff1615610d1f576040516342ee68b560e01b815260040160405180910390fd5b6001816002811115610d3357610d33612790565b03610d4b57610d4687898888888861130b565b610d72565b6002816002811115610d5f57610d5f612790565b0361059557610d4687898888888861146c565b876001600160a01b03167f1201ce0c5e577111bce91e907fd99cb183da5edc1e3fb650ca40769e4e9176dd88838989604051610db19493929190612bf0565b60405180910390a25050505050505050565b81516020808401516040515f938493610e30937f991b0a3376ce87f8ecc5d70962279ac09cdce934e8b5b9683e73c8ff087c7f81938a93928991019485526001600160a01b039384166020860152918316604085015263ffffffff16606084015216608082015260a00190565b604051602081830303815290604052805190602001209050610e51816111bd565b9150505b9392505050565b8151610e6781611261565b610e845760405163932d94f760e01b815260040160405180910390fd5b6001826002811115610e9857610e98612790565b1480610eb557506002826002811115610eb357610eb3612790565b145b610ed25760405163fdea7c0960e01b815260040160405180910390fd5b5f60015f610edf86611203565b815260208101919091526040015f9081205460ff169150816002811115610f0857610f08612790565b14610f25576040516281f09f60e01b815260040160405180910390fd5b8260015f610f3287611203565b815260208101919091526040015f20805460ff19166001836002811115610f5b57610f5b612790565b02179055507fb2266cb118e57095fcdbedb24dabd9fc9f5127e2dbedf62ce6ee71696fb8b6e78484604051610f91929190612b38565b60405180910390a150505050565b5f5f5f5f610fac86611203565b815260208082019290925260409081015f9081206001600160a01b038716825283528190208151808301909252805460ff16151582526001810180549293919291840191610ff990612b00565b80601f016020809104026020016040519081016040528092919081815260200182805461102590612b00565b80156110705780601f1061104757610100808354040283529160200191611070565b820191905f5260205f20905b81548152906001019060200180831161105357829003601f168201915b50505050508152505090505f60015f61108887611203565b815260208101919091526040015f2054825160ff90911691506110b057505f91506103ed9050565b6110be826020015182611667565b95945050505050565b60408051808201909152600a81526922b4b3b2b72630bcb2b960b11b6020909101525f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f7f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea6111346116e8565b805160209182012060408051928301949094529281019190915260608101919091524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b60605f61118c8361175d565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b5f6111c66110c7565b60405161190160f01b6020820152602281019190915260428101839052606201604051602081830303815290604052805190602001209050919050565b5f815f0151826020015163ffffffff1660405160200161124992919060609290921b6001600160601b031916825260a01b6001600160a01b031916601482015260200190565b6040516020818303038152906040526103ed90612c4d565b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af11580156112e7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103ed9190612ae1565b6014831461132c5760405163d109118160e01b815260040160405180910390fd5b5f6113378486612c70565b60601c90508061135a57604051634935505f60e01b815260040160405180910390fd5b5f61139c86868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525060019250611667915050565b5f8181526002602052604090205490915060ff16156113ce57604051630c7bc20160e11b815260040160405180910390fd5b5f6113da888a85610dc3565b905061141f838287878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505f199250611784915050565b611461898989898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508892506117dc915050565b505050505050505050565b60c0831461148d5760405163d109118160e01b815260040160405180910390fd5b604081146114ae57604051638baa579f60e01b815260040160405180910390fd5b604080518082019091525f80825260208201526114c96123db565b5f8080806114d9898b018b612cae565b93509350935093506040518060400160405280858152602001848152509550835f148015611505575082155b1561152357604051634935505f60e01b815260040160405180910390fd5b60408051808201909152918252602082015292505f91506115489050888a8989610423565b90505f8061155886880188612ce8565b604080518082019091528281526020810182905291935091505f611580858389898580611889565b915050806115a157604051638baa579f60e01b815260040160405180910390fd5b5f6115e38c8c8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525060029250611667915050565b5f8181526002602052604090205490915060ff161561161557604051630c7bc20160e11b815260040160405180910390fd5b6116578e8e8e8e8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508792506117dc915050565b5050505050505050505050505050565b5f600182600281111561167c5761167c612790565b0361168e5750815160208301206103ed565b60028260028111156116a2576116a2612790565b03610595575f5f848060200190518101906116bd9190612baa565b505060408051808201825283815260209081019283525f9384529151909152902092506103ed915050565b60605f6117147f0000000000000000000000000000000000000000000000000000000000000000611180565b9050805f8151811061172857611728612a66565b016020908101516040516001600160f81b03199091169181019190915260210160405160208183030381529060405291505090565b5f60ff8216601f8111156103ed57604051632cd44ac360e21b815260040160405180910390fd5b428110156117a557604051630819bdcd60e01b815260040160405180910390fd5b6117b96001600160a01b0385168484611951565b6117d657604051638baa579f60e01b815260040160405180910390fd5b50505050565b6040805180820190915260018152602081018390525f806117fc87611203565b815260208082019290925260409081015f9081206001600160a01b03881682528352208251815460ff19169015151781559082015160018201906118409082612d54565b5050505f908152600260209081526040808320805460ff191660011790556003909152902080546001600160a01b039093166001600160a01b0319909316929092179091555050565b5f5f5f611895896119a5565b90505f6118a48a89898c611a2f565b90505f6118bb6118b48a84611ada565b8b90611b4a565b90505f6118fd6118f6846118f06040805180820182525f80825260209182015281518083019092526001825260029082015290565b90611ada565b8590611b4a565b905087156119225761191982611911611bbe565b838c8b611c7e565b96509450611942565b6119358261192e611bbe565b838c611e92565b9550851561194257600194505b50505050965096945050505050565b5f5f5f61195e85856120c9565b90925090505f81600481111561197657611976612790565b1480156119945750856001600160a01b0316826001600160a01b0316145b806104c157506104c1868686612108565b604080518082019091525f80825260208201525f80806119d25f516020612eba5f395f51905f5286612e0f565b90505b6119de816121ef565b90935091505f516020612eba5f395f51905f528283098303611a16576040805180820190915290815260208101919091529392505050565b5f516020612eba5f395f51905f526001820890506119d5565b8251602080850151845180519083015186840151805190850151875188870151604080519889018e90528801989098526060870195909552608086019390935260a085019190915260c084015260e08301526101008201526101208101919091525f907f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f00000019061014001604051602081830303815290604052805190602001205f1c6110be9190612e0f565b604080518082019091525f8082526020820152611af5612400565b835181526020808501519082015260408082018490525f908360608460076107d05a03fa90508080611b2357fe5b5080611b4257604051632319df1960e11b815260040160405180910390fd5b505092915050565b604080518082019091525f8082526020820152611b6561241e565b835181526020808501518183015283516040808401919091529084015160608301525f908360808460066107d05a03fa90508080611b9f57fe5b5080611b425760405163d4b68fd760e01b815260040160405180910390fd5b611bc66123db565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b6040805180820182528681526020808201869052825180840190935286835282018490525f91829190611caf61243c565b5f5b6002811015611e66575f611cc6826006612e42565b9050848260028110611cda57611cda612a66565b60200201515183611ceb835f612e59565b600c8110611cfb57611cfb612a66565b6020020152848260028110611d1257611d12612a66565b60200201516020015183826001611d299190612e59565b600c8110611d3957611d39612a66565b6020020152838260028110611d5057611d50612a66565b6020020151515183611d63836002612e59565b600c8110611d7357611d73612a66565b6020020152838260028110611d8a57611d8a612a66565b6020020151516001602002015183611da3836003612e59565b600c8110611db357611db3612a66565b6020020152838260028110611dca57611dca612a66565b6020020151602001515f60028110611de457611de4612a66565b602002015183611df5836004612e59565b600c8110611e0557611e05612a66565b6020020152838260028110611e1c57611e1c612a66565b602002015160200151600160028110611e3757611e37612a66565b602002015183611e48836005612e59565b600c8110611e5857611e58612a66565b602002015250600101611cb1565b50611e6f61245b565b5f6020826101808560088cfa9151919c9115159b50909950505050505050505050565b6040805180820182528581526020808201859052825180840190935285835282018390525f91611ec061243c565b5f5b6002811015612077575f611ed7826006612e42565b9050848260028110611eeb57611eeb612a66565b60200201515183611efc835f612e59565b600c8110611f0c57611f0c612a66565b6020020152848260028110611f2357611f23612a66565b60200201516020015183826001611f3a9190612e59565b600c8110611f4a57611f4a612a66565b6020020152838260028110611f6157611f61612a66565b6020020151515183611f74836002612e59565b600c8110611f8457611f84612a66565b6020020152838260028110611f9b57611f9b612a66565b6020020151516001602002015183611fb4836003612e59565b600c8110611fc457611fc4612a66565b6020020152838260028110611fdb57611fdb612a66565b6020020151602001515f60028110611ff557611ff5612a66565b602002015183612006836004612e59565b600c811061201657612016612a66565b602002015283826002811061202d5761202d612a66565b60200201516020015160016002811061204857612048612a66565b602002015183612059836005612e59565b600c811061206957612069612a66565b602002015250600101611ec2565b5061208061245b565b5f6020826101808560086107d05a03fa9050808061209a57fe5b50806120b9576040516324ccc79360e21b815260040160405180910390fd5b5051151598975050505050505050565b5f5f82516041036120fd576020830151604084015160608501515f1a6120f18782858561226b565b945094505050506105d8565b505f905060026105d8565b5f5f5f856001600160a01b0316631626ba7e60e01b8686604051602401612130929190612e6c565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905161216e9190612e8c565b5f60405180830381855afa9150503d805f81146121a6576040519150601f19603f3d011682016040523d82523d5f602084013e6121ab565b606091505b50915091508180156121bf57506020815110155b80156104c157508051630b135d3f60e11b906121e49083016020908101908401612ea2565b149695505050505050565b5f80805f516020612eba5f395f51905f5260035f516020612eba5f395f51905f52865f516020612eba5f395f51905f52888909090890505f61225f827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f525f516020612eba5f395f51905f52612328565b91959194509092505050565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156122a057505f9050600361231f565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa1580156122f1573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116612319575f6001925092505061231f565b91505f90505b94509492505050565b5f5f61233261245b565b61233a612479565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa9250828061237757fe5b50826123965760405163d51edae360e01b815260040160405180910390fd5b505195945050505050565b5080546123ad90612b00565b5f825580601f106123bc575050565b601f0160209004905f5260205f20908101906123d89190612497565b50565b60405180604001604052806123ee6124af565b81526020016123fb6124af565b905290565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b5b808211156124ab575f8155600101612498565b5090565b60405180604001604052806002906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b6040805190810167ffffffffffffffff81118282101715612504576125046124cd565b60405290565b604051601f8201601f1916810167ffffffffffffffff81118282101715612533576125336124cd565b604052919050565b80356001600160a01b0381168114612551575f5ffd5b919050565b5f60408284031215612566575f5ffd5b61256e6124e1565b90506125798261253b565b8152602082013563ffffffff81168114612591575f5ffd5b602082015292915050565b5f5f606083850312156125ad575f5ffd5b6125b78484612556565b91506125c56040840161253b565b90509250929050565b5f82601f8301126125dd575f5ffd5b6125e7604061250a565b8060408401858111156125f8575f5ffd5b845b818110156126125780358452602093840193016125fa565b509095945050505050565b5f5f82840360c081121561262f575f5ffd5b604081121561263c575f5ffd5b6126446124e1565b843581526020808601359082015292506080603f1982011215612665575f5ffd5b5061266e6124e1565b61267b85604086016125ce565b815261268a85608086016125ce565b6020820152809150509250929050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f610e55602083018461269a565b5f5f83601f8401126126ea575f5ffd5b50813567ffffffffffffffff811115612701575f5ffd5b6020830191508360208285010111156105d8575f5ffd5b5f5f5f5f6080858703121561272b575f5ffd5b6127348561253b565b93506127438660208701612556565b9250606085013567ffffffffffffffff81111561275e575f5ffd5b61276a878288016126da565b95989497509550505050565b5f60408284031215612786575f5ffd5b610e558383612556565b634e487b7160e01b5f52602160045260245ffd5b600381106127c057634e487b7160e01b5f52602160045260245ffd5b9052565b602081016103ed82846127a4565b5f5f606083850312156127e3575f5ffd5b6127ed8484612556565b9150604083013567ffffffffffffffff811115612808575f5ffd5b8301601f81018513612818575f5ffd5b803567ffffffffffffffff811115612832576128326124cd565b612845601f8201601f191660200161250a565b818152866020838501011115612859575f5ffd5b816020840160208301375f602083830101528093505050509250929050565b5f5f60608385031215612889575f5ffd5b6128928361253b565b91506125c58460208501612556565b805f5b60028110156117d65781518452602093840193909101906001016128a4565b5f60c08201905083518252602084015160208301526128e66040830184516128a1565b60208301516128f860808401826128a1565b509392505050565b5f5f5f5f5f5f60a08789031215612915575f5ffd5b61291e8761253b565b955061292d8860208901612556565b9450606087013567ffffffffffffffff811115612948575f5ffd5b61295489828a016126da565b909550935050608087013567ffffffffffffffff811115612973575f5ffd5b61297f89828a016126da565b979a9699509497509295939492505050565b5f5f5f608084860312156129a3575f5ffd5b6129ac8461253b565b92506129bb8560208601612556565b91506129c96060850161253b565b90509250925092565b5f602082840312156129e2575f5ffd5b5035919050565b5f5f606083850312156129fa575f5ffd5b612a048484612556565b9150604083013560038110612a17575f5ffd5b809150509250929050565b805160208201516001600160601b0319811691906014821015612a5f576001600160601b03196001600160601b03198360140360031b1b82161692505b5050919050565b634e487b7160e01b5f52603260045260245ffd5b818382375f9101908152919050565b5f5f60408385031215612a9a575f5ffd5b505080516020909101519092909150565b6001600160a01b038316815260608101610e55602083018480516001600160a01b0316825260209081015163ffffffff16910152565b5f60208284031215612af1575f5ffd5b81518015158114610e55575f5ffd5b600181811c90821680612b1457607f821691505b602082108103612b3257634e487b7160e01b5f52602260045260245ffd5b50919050565b82516001600160a01b0316815260208084015163ffffffff169082015260608101610e5560408301846127a4565b5f82601f830112612b75575f5ffd5b612b7f604061250a565b806040840185811115612b90575f5ffd5b845b81811015612612578051845260209384019301612b92565b5f5f5f5f60c08587031215612bbd575f5ffd5b845160208601519094509250612bd68660408701612b66565b9150612be58660808701612b66565b905092959194509250565b84516001600160a01b0316815260208086015163ffffffff1690820152612c1a60408201856127a4565b60806060820152816080820152818360a08301375f81830160a090810191909152601f909201601f191601019392505050565b80516020808301519190811015612b32575f1960209190910360031b1b16919050565b80356001600160601b03198116906014841015612ca7576001600160601b03196001600160601b03198560140360031b1b82161691505b5092915050565b5f5f5f5f60c08587031215612cc1575f5ffd5b8435935060208501359250612cd986604087016125ce565b9150612be586608087016125ce565b5f5f60408385031215612cf9575f5ffd5b50508035926020909101359150565b601f821115612d4f57805f5260205f20601f840160051c81016020851015612d2d5750805b601f840160051c820191505b81811015612d4c575f8155600101612d39565b50505b505050565b815167ffffffffffffffff811115612d6e57612d6e6124cd565b612d8281612d7c8454612b00565b84612d08565b6020601f821160018114612db4575f8315612d9d5750848201515b5f19600385901b1c1916600184901b178455612d4c565b5f84815260208120601f198516915b82811015612de35787850151825560209485019460019092019101612dc3565b5084821015612e0057868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f82612e2957634e487b7160e01b5f52601260045260245ffd5b500690565b634e487b7160e01b5f52601160045260245ffd5b80820281158282048414176103ed576103ed612e2e565b808201808211156103ed576103ed612e2e565b828152604060208201525f612e84604083018461269a565b949350505050565b5f82518060208501845e5f920191825250919050565b5f60208284031215612eb2575f5ffd5b505191905056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a26469706673582212201df3a560128f27a0612f2f095e7e575dd271a835148632603757c72234c7b14364736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa1J8\x03\x80a1J\x839\x81\x01`@\x81\x90Ra\0.\x91a\0\xCBV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x80R\x83\x16`\xA0R\x80\x80a\0L\x81a\0ZV[`\xC0RPa\x01\xFC\x93PPPPV[__\x82\x90P`\x1F\x81Q\x11\x15a\0\x8DW\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\0\x84\x91\x90a\x01\xA1V[`@Q\x80\x91\x03\x90\xFD[\x80Qa\0\x98\x82a\x01\xD6V[\x17\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xB4W__\xFD[PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[___``\x84\x86\x03\x12\x15a\0\xDDW__\xFD[\x83Qa\0\xE8\x81a\0\xA0V[` \x85\x01Q\x90\x93Pa\0\xF9\x81a\0\xA0V[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01\x14W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a\x01$W__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01=Wa\x01=a\0\xB7V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x01kWa\x01ka\0\xB7V[`@R\x81\x81R\x82\x82\x01` \x01\x88\x10\x15a\x01\x82W__\xFD[\x81` \x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92P\x92V[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x01\xF6W_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\x80Q`\xA0Q`\xC0Qa/\x0Fa\x02;_9_\x81\x81a\x03\xFA\x01Ra\x16\xF0\x01R_\x81\x81a\x01\x95\x01Ra\x12\x9F\x01R_\x81\x81a\x03;\x01Ra\x06\x1D\x01Ra/\x0F_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01'W_5`\xE0\x1C\x80c\xAA\x16\\0\x11a\0\xA9W\x80c\xD9\xF1-\xB2\x11a\0nW\x80c\xD9\xF1-\xB2\x14a\x03pW\x80c\xDA\xB4-~\x14a\x03\x83W\x80c\xEA\r\x81I\x14a\x03\xA5W\x80c\xEA\x19N.\x14a\x03\xB8W\x80c\xF6\x98\xDA%\x14a\x03\xCBW__\xFD[\x80c\xAA\x16\\0\x14a\x02\xD9W\x80c\xB0\\\x8Fm\x14a\x02\xECW\x80c\xBD0\xA0\xB9\x14a\x03\x13W\x80c\xCA\x8A\xA7\xC7\x14a\x036W\x80c\xD4\x0C\xDA\x16\x14a\x03]W__\xFD[\x80cv\x90\xE3\x95\x11a\0\xEFW\x80cv\x90\xE3\x95\x14a\x02>W\x80c|\xFF\xE4\x8C\x14a\x02QW\x80c\x82V\x90\x9C\x14a\x02qW\x80c\x87\xAB\x86\xF4\x14a\x02\xA3W\x80c\x9AC\xE3\xFB\x14a\x02\xB8W__\xFD[\x80c\x16j\xA1'\x14a\x01+W\x80c;2\xA7\xBD\x14a\x01eW\x80cFW\xE2j\x14a\x01\x90W\x80cPCZ\xDD\x14a\x01\xB7W\x80cT\xFDMP\x14a\x026W[__\xFD[a\x01R\x7F\x99\x1B\n3v\xCE\x87\xF8\xEC\xC5\xD7\tb'\x9A\xC0\x9C\xDC\xE94\xE8\xB5\xB9h>s\xC8\xFF\x08|\x7F\x81\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01xa\x01s6`\x04a%\x9CV[a\x03\xD3V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\\V[a\x01x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02)a\x01\xC56`\x04a&\x1DV[\x81Q` \x80\x84\x01Q\x83Q\x80Q\x90\x83\x01Q\x85\x84\x01Q\x80Q\x90\x85\x01Q`@\x80Q\x96\x87\x01\x97\x90\x97R\x95\x85\x01\x93\x90\x93R``\x84\x81\x01\x92\x90\x92R`\x80\x84\x01R`\xA0\x83\x01\x91\x90\x91R`\xC0\x82\x01\x92\x90\x92R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`@Qa\x01\\\x91\x90a&\xC8V[a\x02)a\x03\xF3V[a\x01Ra\x02L6`\x04a'\x18V[a\x04#V[a\x02da\x02_6`\x04a'vV[a\x04\xCBV[`@Qa\x01\\\x91\x90a'\xC4V[a\x02\x84a\x02\x7F6`\x04a'\xD2V[a\x04\xF1V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R\x90\x15\x15` \x83\x01R\x01a\x01\\V[a\x02\xB6a\x02\xB16`\x04a(xV[a\x05\xDFV[\0[a\x02\xCBa\x02\xC66`\x04a%\x9CV[a\x08\xF0V[`@Qa\x01\\\x92\x91\x90a(\xC3V[a\x02)a\x02\xE76`\x04a%\x9CV[a\n\xD8V[a\x01R\x7F\xDA\x86\xE7m\xEA\xED\x01d\x1F\x80\xFF_r\xC3r\xA08\xFAQ\x82iz\xEB\x96~\x8B\x1F\x98\x19\xD5\x8D\x81\x81V[a\x03&a\x03!6`\x04a%\x9CV[a\x0C\x15V[`@Q\x90\x15\x15\x81R` \x01a\x01\\V[a\x01x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xB6a\x03k6`\x04a)\0V[a\x0CRV[a\x01Ra\x03~6`\x04a)\x91V[a\r\xC3V[a\x03&a\x03\x916`\x04a)\xD2V[_\x90\x81R`\x02` R`@\x90 T`\xFF\x16\x90V[a\x02\xB6a\x03\xB36`\x04a)\xE9V[a\x0E\\V[a\x01Ra\x03\xC66`\x04a%\x9CV[a\x0F\x9FV[a\x01Ra\x10\xC7V[_a\x03\xDE\x83\x83a\n\xD8V[a\x03\xE7\x90a*\"V[``\x1C\x90P[\x92\x91PPV[``a\x04\x1E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x11\x80V[\x90P\x90V[__\x7F\xDA\x86\xE7m\xEA\xED\x01d\x1F\x80\xFF_r\xC3r\xA08\xFAQ\x82iz\xEB\x96~\x8B\x1F\x98\x19\xD5\x8D\x81\x86\x86_\x01Q\x87` \x01Q\x87\x87`@Qa\x04`\x92\x91\x90a*zV[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x82\x01R\x92\x90\x91\x16``\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x04\xC1\x81a\x11\xBDV[\x96\x95PPPPPPV[_`\x01_a\x04\xD8\x84a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16\x92\x91PPV[___`\x01_a\x05\0\x87a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_\x90\x81 T`\xFF\x16\x91P`\x01\x82`\x02\x81\x11\x15a\x05+Wa\x05+a'\x90V[\x03a\x05=WP\x83Q` \x85\x01 a\x05\xAEV[`\x02\x82`\x02\x81\x11\x15a\x05QWa\x05Qa'\x90V[\x03a\x05\x95W__\x86\x80` \x01\x90Q\x81\x01\x90a\x05l\x91\x90a*\x89V[`@\x80Q\x80\x82\x01\x82R\x83\x81R` \x90\x81\x01\x92\x83R_\x93\x84R\x91Q\x90\x91R\x90 \x92Pa\x05\xAE\x91PPV[`@Qc\xFD\xEA|\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x03` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x05\xD0\x88\x82a\x0C\x15V[\x94P\x94PPPP[\x92P\x92\x90PV[\x81a\x05\xE9\x81a\x12aV[a\x06\x06W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\t\xA9a\xF3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13R\xC3\xE6\x90a\x06T\x90\x86\x90\x86\x90`\x04\x01a*\xABV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06oW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x93\x91\x90a*\xE1V[\x15\x82\x84\x90\x91a\x06\xE0W`@Qc\x10p(y`\xE0\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R` \x90\x93\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x91\x16`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PP_`\x01_a\x06\xEF\x85a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_\x90\x81 T`\xFF\x16\x91P\x81`\x02\x81\x11\x15a\x07\x18Wa\x07\x18a'\x90V[\x03a\x076W`@Qc\\\xD3\x10m`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___a\x07B\x86a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x89\x16\x82R\x83R\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T`\xFF\x16\x15\x15\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a\x07\x8F\x90a+\0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xBB\x90a+\0V[\x80\x15a\x08\x06W\x80`\x1F\x10a\x07\xDDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x06V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xE9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80_\x01Q\x84\x86\x90\x91a\x08\\W`@Qc.@\xE1\x87`\xE0\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R` \x90\x93\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x91\x16`D\x82\x01R`d\x01a\x06\xD7V[PP__a\x08i\x86a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x89\x16\x82R\x90\x92R\x81 \x80T`\xFF\x19\x16\x81U\x90a\x08\xA4`\x01\x83\x01\x82a#\xA1V[PP\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F(\xD3\xC3\xCE\xE4\x94x\xECo\xD2\x19\xCF\xD6\x85\xCD\x15\xCD\x01\xD9\\\xAB\xF6\x9BK{W\xF9\xEA\xA3\xEBdB\x85\x84`@Qa\x08\xE1\x92\x91\x90a+8V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\t\x0Ba#\xDBV[_`\x01_a\t\x18\x87a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16\x90P`\x02\x81`\x02\x81\x11\x15a\tAWa\tAa'\x90V[\x14a\t_W`@Qc\xFD\xEA|\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___a\tk\x88a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x89\x16\x82R\x83R\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T`\xFF\x16\x15\x15\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a\t\xB8\x90a+\0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xE4\x90a+\0V[\x80\x15a\n/W\x80`\x1F\x10a\n\x06Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n/V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\x12W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80_\x01Qa\n\x84WPP`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x85R\x82\x81R\x80\x82\x01\x92\x90\x92R\x83Q\x80\x85\x01\x90\x94R\x82\x84R\x83\x01\x91\x90\x91R\x92P\x90Pa\x05\xD8V[____\x84` \x01Q\x80` \x01\x90Q\x81\x01\x90a\n\xA0\x91\x90a+\xAAV[`@\x80Q\x80\x82\x01\x82R\x94\x85R` \x80\x86\x01\x94\x90\x94R\x80Q\x80\x82\x01\x90\x91R\x91\x82R\x91\x81\x01\x91\x90\x91R\x90\x9B\x90\x9AP\x98PPPPPPPPPV[``_`\x01_a\n\xE7\x86a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16\x90P`\x01\x81`\x02\x81\x11\x15a\x0B\x10Wa\x0B\x10a'\x90V[\x14a\x0B.W`@Qc\xFD\xEA|\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___a\x0B:\x87a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x88\x16\x82R\x83R\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T`\xFF\x16\x15\x15\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a\x0B\x87\x90a+\0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xB3\x90a+\0V[\x80\x15a\x0B\xFEW\x80`\x1F\x10a\x0B\xD5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xFEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xE1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x01Q\x95\x94PPPPPV[___a\x0C!\x85a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x86\x16\x82R\x90\x92R\x90 T`\xFF\x16\x90P\x92\x91PPV[\x85a\x0C\\\x81a\x12aV[a\x0CyW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\x01_a\x0C\x86\x89a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_\x90\x81 T`\xFF\x16\x91P\x81`\x02\x81\x11\x15a\x0C\xAFWa\x0C\xAFa'\x90V[\x03a\x0C\xCDW`@Qc\\\xD3\x10m`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x0C\xD8\x89a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x8C\x16\x82R\x90\x92R\x90 T`\xFF\x16\x15a\r\x1FW`@QcB\xEEh\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x02\x81\x11\x15a\r3Wa\r3a'\x90V[\x03a\rKWa\rF\x87\x89\x88\x88\x88\x88a\x13\x0BV[a\rrV[`\x02\x81`\x02\x81\x11\x15a\r_Wa\r_a'\x90V[\x03a\x05\x95Wa\rF\x87\x89\x88\x88\x88\x88a\x14lV[\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\x12\x01\xCE\x0C^Wq\x11\xBC\xE9\x1E\x90\x7F\xD9\x9C\xB1\x83\xDA^\xDC\x1E?\xB6P\xCA@v\x9EN\x91v\xDD\x88\x83\x89\x89`@Qa\r\xB1\x94\x93\x92\x91\x90a+\xF0V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[\x81Q` \x80\x84\x01Q`@Q_\x93\x84\x93a\x0E0\x93\x7F\x99\x1B\n3v\xCE\x87\xF8\xEC\xC5\xD7\tb'\x9A\xC0\x9C\xDC\xE94\xE8\xB5\xB9h>s\xC8\xFF\x08|\x7F\x81\x93\x8A\x93\x92\x89\x91\x01\x94\x85R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x86\x01R\x91\x83\x16`@\x85\x01Rc\xFF\xFF\xFF\xFF\x16``\x84\x01R\x16`\x80\x82\x01R`\xA0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x0EQ\x81a\x11\xBDV[\x91PP[\x93\x92PPPV[\x81Qa\x0Eg\x81a\x12aV[a\x0E\x84W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x02\x81\x11\x15a\x0E\x98Wa\x0E\x98a'\x90V[\x14\x80a\x0E\xB5WP`\x02\x82`\x02\x81\x11\x15a\x0E\xB3Wa\x0E\xB3a'\x90V[\x14[a\x0E\xD2W`@Qc\xFD\xEA|\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\x01_a\x0E\xDF\x86a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_\x90\x81 T`\xFF\x16\x91P\x81`\x02\x81\x11\x15a\x0F\x08Wa\x0F\x08a'\x90V[\x14a\x0F%W`@Qb\x81\xF0\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01_a\x0F2\x87a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16`\x01\x83`\x02\x81\x11\x15a\x0F[Wa\x0F[a'\x90V[\x02\x17\x90UP\x7F\xB2&l\xB1\x18\xE5p\x95\xFC\xDB\xED\xB2M\xAB\xD9\xFC\x9FQ'\xE2\xDB\xED\xF6,\xE6\xEEqio\xB8\xB6\xE7\x84\x84`@Qa\x0F\x91\x92\x91\x90a+8V[`@Q\x80\x91\x03\x90\xA1PPPPV[____a\x0F\xAC\x86a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x87\x16\x82R\x83R\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T`\xFF\x16\x15\x15\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a\x0F\xF9\x90a+\0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10%\x90a+\0V[\x80\x15a\x10pW\x80`\x1F\x10a\x10GWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10pV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10SW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P_`\x01_a\x10\x88\x87a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T\x82Q`\xFF\x90\x91\x16\x91Pa\x10\xB0WP_\x91Pa\x03\xED\x90PV[a\x10\xBE\x82` \x01Q\x82a\x16gV[\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x90\x91\x01R_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEAa\x114a\x16\xE8V[\x80Q` \x91\x82\x01 `@\x80Q\x92\x83\x01\x94\x90\x94R\x92\x81\x01\x91\x90\x91R``\x81\x01\x91\x90\x91RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[``_a\x11\x8C\x83a\x17]V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[_a\x11\xC6a\x10\xC7V[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a\x12I\x92\x91\x90``\x92\x90\x92\x1B`\x01`\x01``\x1B\x03\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x03\xED\x90a,MV[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12\xE7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xED\x91\x90a*\xE1V[`\x14\x83\x14a\x13,W`@Qc\xD1\t\x11\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x137\x84\x86a,pV[``\x1C\x90P\x80a\x13ZW`@QcI5P_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\x9C\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP`\x01\x92Pa\x16g\x91PPV[_\x81\x81R`\x02` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x13\xCEW`@Qc\x0C{\xC2\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\xDA\x88\x8A\x85a\r\xC3V[\x90Pa\x14\x1F\x83\x82\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP_\x19\x92Pa\x17\x84\x91PPV[a\x14a\x89\x89\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x17\xDC\x91PPV[PPPPPPPPPV[`\xC0\x83\x14a\x14\x8DW`@Qc\xD1\t\x11\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x81\x14a\x14\xAEW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x14\xC9a#\xDBV[_\x80\x80\x80a\x14\xD9\x89\x8B\x01\x8Ba,\xAEV[\x93P\x93P\x93P\x93P`@Q\x80`@\x01`@R\x80\x85\x81R` \x01\x84\x81RP\x95P\x83_\x14\x80\x15a\x15\x05WP\x82\x15[\x15a\x15#W`@QcI5P_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x91\x82R` \x82\x01R\x92P_\x91Pa\x15H\x90P\x88\x8A\x89\x89a\x04#V[\x90P_\x80a\x15X\x86\x88\x01\x88a,\xE8V[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91P_a\x15\x80\x85\x83\x89\x89\x85\x80a\x18\x89V[\x91PP\x80a\x15\xA1W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x15\xE3\x8C\x8C\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP`\x02\x92Pa\x16g\x91PPV[_\x81\x81R`\x02` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x16\x15W`@Qc\x0C{\xC2\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16W\x8E\x8E\x8E\x8E\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x87\x92Pa\x17\xDC\x91PPV[PPPPPPPPPPPPPPV[_`\x01\x82`\x02\x81\x11\x15a\x16|Wa\x16|a'\x90V[\x03a\x16\x8EWP\x81Q` \x83\x01 a\x03\xEDV[`\x02\x82`\x02\x81\x11\x15a\x16\xA2Wa\x16\xA2a'\x90V[\x03a\x05\x95W__\x84\x80` \x01\x90Q\x81\x01\x90a\x16\xBD\x91\x90a+\xAAV[PP`@\x80Q\x80\x82\x01\x82R\x83\x81R` \x90\x81\x01\x92\x83R_\x93\x84R\x91Q\x90\x91R\x90 \x92Pa\x03\xED\x91PPV[``_a\x17\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x11\x80V[\x90P\x80_\x81Q\x81\x10a\x17(Wa\x17(a*fV[\x01` \x90\x81\x01Q`@Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x91\x81\x01\x91\x90\x91R`!\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x90V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x03\xEDW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x81\x10\x15a\x17\xA5W`@Qc\x08\x19\xBD\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17\xB9`\x01`\x01`\xA0\x1B\x03\x85\x16\x84\x84a\x19QV[a\x17\xD6W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R` \x81\x01\x83\x90R_\x80a\x17\xFC\x87a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x88\x16\x82R\x83R \x82Q\x81T`\xFF\x19\x16\x90\x15\x15\x17\x81U\x90\x82\x01Q`\x01\x82\x01\x90a\x18@\x90\x82a-TV[PPP_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x03\x90\x91R\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91UPPV[___a\x18\x95\x89a\x19\xA5V[\x90P_a\x18\xA4\x8A\x89\x89\x8Ca\x1A/V[\x90P_a\x18\xBBa\x18\xB4\x8A\x84a\x1A\xDAV[\x8B\x90a\x1BJV[\x90P_a\x18\xFDa\x18\xF6\x84a\x18\xF0`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\x1A\xDAV[\x85\x90a\x1BJV[\x90P\x87\x15a\x19\"Wa\x19\x19\x82a\x19\x11a\x1B\xBEV[\x83\x8C\x8Ba\x1C~V[\x96P\x94Pa\x19BV[a\x195\x82a\x19.a\x1B\xBEV[\x83\x8Ca\x1E\x92V[\x95P\x85\x15a\x19BW`\x01\x94P[PPPP\x96P\x96\x94PPPPPV[___a\x19^\x85\x85a \xC9V[\x90\x92P\x90P_\x81`\x04\x81\x11\x15a\x19vWa\x19va'\x90V[\x14\x80\x15a\x19\x94WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80a\x04\xC1WPa\x04\xC1\x86\x86\x86a!\x08V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80\x80a\x19\xD2_Q` a.\xBA_9_Q\x90_R\x86a.\x0FV[\x90P[a\x19\xDE\x81a!\xEFV[\x90\x93P\x91P_Q` a.\xBA_9_Q\x90_R\x82\x83\t\x83\x03a\x1A\x16W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[_Q` a.\xBA_9_Q\x90_R`\x01\x82\x08\x90Pa\x19\xD5V[\x82Q` \x80\x85\x01Q\x84Q\x80Q\x90\x83\x01Q\x86\x84\x01Q\x80Q\x90\x85\x01Q\x87Q\x88\x87\x01Q`@\x80Q\x98\x89\x01\x8E\x90R\x88\x01\x98\x90\x98R``\x87\x01\x95\x90\x95R`\x80\x86\x01\x93\x90\x93R`\xA0\x85\x01\x91\x90\x91R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x81\x01\x91\x90\x91R_\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90a\x01@\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\x10\xBE\x91\x90a.\x0FV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x1A\xF5a$\0V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R_\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x1B#W\xFE[P\x80a\x1BBW`@Qc#\x19\xDF\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x1Bea$\x1EV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R_\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x1B\x9FW\xFE[P\x80a\x1BBW`@Qc\xD4\xB6\x8F\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B\xC6a#\xDBV[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R_\x91\x82\x91\x90a\x1C\xAFa$<V[_[`\x02\x81\x10\x15a\x1EfW_a\x1C\xC6\x82`\x06a.BV[\x90P\x84\x82`\x02\x81\x10a\x1C\xDAWa\x1C\xDAa*fV[` \x02\x01QQ\x83a\x1C\xEB\x83_a.YV[`\x0C\x81\x10a\x1C\xFBWa\x1C\xFBa*fV[` \x02\x01R\x84\x82`\x02\x81\x10a\x1D\x12Wa\x1D\x12a*fV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x1D)\x91\x90a.YV[`\x0C\x81\x10a\x1D9Wa\x1D9a*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1DPWa\x1DPa*fV[` \x02\x01QQQ\x83a\x1Dc\x83`\x02a.YV[`\x0C\x81\x10a\x1DsWa\x1Dsa*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1D\x8AWa\x1D\x8Aa*fV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x1D\xA3\x83`\x03a.YV[`\x0C\x81\x10a\x1D\xB3Wa\x1D\xB3a*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1D\xCAWa\x1D\xCAa*fV[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x1D\xE4Wa\x1D\xE4a*fV[` \x02\x01Q\x83a\x1D\xF5\x83`\x04a.YV[`\x0C\x81\x10a\x1E\x05Wa\x1E\x05a*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1E\x1CWa\x1E\x1Ca*fV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x1E7Wa\x1E7a*fV[` \x02\x01Q\x83a\x1EH\x83`\x05a.YV[`\x0C\x81\x10a\x1EXWa\x1EXa*fV[` \x02\x01RP`\x01\x01a\x1C\xB1V[Pa\x1Eoa$[V[_` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R_\x91a\x1E\xC0a$<V[_[`\x02\x81\x10\x15a wW_a\x1E\xD7\x82`\x06a.BV[\x90P\x84\x82`\x02\x81\x10a\x1E\xEBWa\x1E\xEBa*fV[` \x02\x01QQ\x83a\x1E\xFC\x83_a.YV[`\x0C\x81\x10a\x1F\x0CWa\x1F\x0Ca*fV[` \x02\x01R\x84\x82`\x02\x81\x10a\x1F#Wa\x1F#a*fV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x1F:\x91\x90a.YV[`\x0C\x81\x10a\x1FJWa\x1FJa*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1FaWa\x1Faa*fV[` \x02\x01QQQ\x83a\x1Ft\x83`\x02a.YV[`\x0C\x81\x10a\x1F\x84Wa\x1F\x84a*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1F\x9BWa\x1F\x9Ba*fV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x1F\xB4\x83`\x03a.YV[`\x0C\x81\x10a\x1F\xC4Wa\x1F\xC4a*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1F\xDBWa\x1F\xDBa*fV[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x1F\xF5Wa\x1F\xF5a*fV[` \x02\x01Q\x83a \x06\x83`\x04a.YV[`\x0C\x81\x10a \x16Wa \x16a*fV[` \x02\x01R\x83\x82`\x02\x81\x10a -Wa -a*fV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a HWa Ha*fV[` \x02\x01Q\x83a Y\x83`\x05a.YV[`\x0C\x81\x10a iWa ia*fV[` \x02\x01RP`\x01\x01a\x1E\xC2V[Pa \x80a$[V[_` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80a \x9AW\xFE[P\x80a \xB9W`@Qc$\xCC\xC7\x93`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x15\x15\x98\x97PPPPPPPPV[__\x82Q`A\x03a \xFDW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa \xF1\x87\x82\x85\x85a\"kV[\x94P\x94PPPPa\x05\xD8V[P_\x90P`\x02a\x05\xD8V[___\x85`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x86\x86`@Q`$\x01a!0\x92\x91\x90a.lV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa!n\x91\x90a.\x8CV[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a!\xA6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a!\xABV[``\x91P[P\x91P\x91P\x81\x80\x15a!\xBFWP` \x81Q\x10\x15[\x80\x15a\x04\xC1WP\x80Qc\x0B\x13]?`\xE1\x1B\x90a!\xE4\x90\x83\x01` \x90\x81\x01\x90\x84\x01a.\xA2V[\x14\x96\x95PPPPPPV[_\x80\x80_Q` a.\xBA_9_Q\x90_R`\x03_Q` a.\xBA_9_Q\x90_R\x86_Q` a.\xBA_9_Q\x90_R\x88\x89\t\t\x08\x90P_a\"_\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R_Q` a.\xBA_9_Q\x90_Ra#(V[\x91\x95\x91\x94P\x90\x92PPPV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\"\xA0WP_\x90P`\x03a#\x1FV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\"\xF1W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a#\x19W_`\x01\x92P\x92PPa#\x1FV[\x91P_\x90P[\x94P\x94\x92PPPV[__a#2a$[V[a#:a$yV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a#wW\xFE[P\x82a#\x96W`@Qc\xD5\x1E\xDA\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x95\x94PPPPPV[P\x80Ta#\xAD\x90a+\0V[_\x82U\x80`\x1F\x10a#\xBCWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a#\xD8\x91\x90a$\x97V[PV[`@Q\x80`@\x01`@R\x80a#\xEEa$\xAFV[\x81R` \x01a#\xFBa$\xAFV[\x90R\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a$\xABW_\x81U`\x01\x01a$\x98V[P\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\x04Wa%\x04a$\xCDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%3Wa%3a$\xCDV[`@R\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a%QW__\xFD[\x91\x90PV[_`@\x82\x84\x03\x12\x15a%fW__\xFD[a%na$\xE1V[\x90Pa%y\x82a%;V[\x81R` \x82\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a%\x91W__\xFD[` \x82\x01R\x92\x91PPV[__``\x83\x85\x03\x12\x15a%\xADW__\xFD[a%\xB7\x84\x84a%VV[\x91Pa%\xC5`@\x84\x01a%;V[\x90P\x92P\x92\x90PV[_\x82`\x1F\x83\x01\x12a%\xDDW__\xFD[a%\xE7`@a%\nV[\x80`@\x84\x01\x85\x81\x11\x15a%\xF8W__\xFD[\x84[\x81\x81\x10\x15a&\x12W\x805\x84R` \x93\x84\x01\x93\x01a%\xFAV[P\x90\x95\x94PPPPPV[__\x82\x84\x03`\xC0\x81\x12\x15a&/W__\xFD[`@\x81\x12\x15a&<W__\xFD[a&Da$\xE1V[\x845\x81R` \x80\x86\x015\x90\x82\x01R\x92P`\x80`?\x19\x82\x01\x12\x15a&eW__\xFD[Pa&na$\xE1V[a&{\x85`@\x86\x01a%\xCEV[\x81Ra&\x8A\x85`\x80\x86\x01a%\xCEV[` \x82\x01R\x80\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0EU` \x83\x01\x84a&\x9AV[__\x83`\x1F\x84\x01\x12a&\xEAW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\x01W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x05\xD8W__\xFD[____`\x80\x85\x87\x03\x12\x15a'+W__\xFD[a'4\x85a%;V[\x93Pa'C\x86` \x87\x01a%VV[\x92P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'^W__\xFD[a'j\x87\x82\x88\x01a&\xDAV[\x95\x98\x94\x97P\x95PPPPV[_`@\x82\x84\x03\x12\x15a'\x86W__\xFD[a\x0EU\x83\x83a%VV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10a'\xC0WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[` \x81\x01a\x03\xED\x82\x84a'\xA4V[__``\x83\x85\x03\x12\x15a'\xE3W__\xFD[a'\xED\x84\x84a%VV[\x91P`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x08W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a(\x18W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(2Wa(2a$\xCDV[a(E`\x1F\x82\x01`\x1F\x19\x16` \x01a%\nV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a(YW__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[__``\x83\x85\x03\x12\x15a(\x89W__\xFD[a(\x92\x83a%;V[\x91Pa%\xC5\x84` \x85\x01a%VV[\x80_[`\x02\x81\x10\x15a\x17\xD6W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a(\xA4V[_`\xC0\x82\x01\x90P\x83Q\x82R` \x84\x01Q` \x83\x01Ra(\xE6`@\x83\x01\x84Qa(\xA1V[` \x83\x01Qa(\xF8`\x80\x84\x01\x82a(\xA1V[P\x93\x92PPPV[______`\xA0\x87\x89\x03\x12\x15a)\x15W__\xFD[a)\x1E\x87a%;V[\x95Pa)-\x88` \x89\x01a%VV[\x94P``\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)HW__\xFD[a)T\x89\x82\x8A\x01a&\xDAV[\x90\x95P\x93PP`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)sW__\xFD[a)\x7F\x89\x82\x8A\x01a&\xDAV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[___`\x80\x84\x86\x03\x12\x15a)\xA3W__\xFD[a)\xAC\x84a%;V[\x92Pa)\xBB\x85` \x86\x01a%VV[\x91Pa)\xC9``\x85\x01a%;V[\x90P\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a)\xE2W__\xFD[P5\x91\x90PV[__``\x83\x85\x03\x12\x15a)\xFAW__\xFD[a*\x04\x84\x84a%VV[\x91P`@\x83\x015`\x03\x81\x10a*\x17W__\xFD[\x80\x91PP\x92P\x92\x90PV[\x80Q` \x82\x01Q`\x01`\x01``\x1B\x03\x19\x81\x16\x91\x90`\x14\x82\x10\x15a*_W`\x01`\x01``\x1B\x03\x19`\x01`\x01``\x1B\x03\x19\x83`\x14\x03`\x03\x1B\x1B\x82\x16\x16\x92P[PP\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[__`@\x83\x85\x03\x12\x15a*\x9AW__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x0EU` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[_` \x82\x84\x03\x12\x15a*\xF1W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0EUW__\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a+\x14W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a+2WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x81\x01a\x0EU`@\x83\x01\x84a'\xA4V[_\x82`\x1F\x83\x01\x12a+uW__\xFD[a+\x7F`@a%\nV[\x80`@\x84\x01\x85\x81\x11\x15a+\x90W__\xFD[\x84[\x81\x81\x10\x15a&\x12W\x80Q\x84R` \x93\x84\x01\x93\x01a+\x92V[____`\xC0\x85\x87\x03\x12\x15a+\xBDW__\xFD[\x84Q` \x86\x01Q\x90\x94P\x92Pa+\xD6\x86`@\x87\x01a+fV[\x91Pa+\xE5\x86`\x80\x87\x01a+fV[\x90P\x92\x95\x91\x94P\x92PV[\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra,\x1A`@\x82\x01\x85a'\xA4V[`\x80``\x82\x01R\x81`\x80\x82\x01R\x81\x83`\xA0\x83\x017_\x81\x83\x01`\xA0\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x93\x92PPPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a+2W_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[\x805`\x01`\x01``\x1B\x03\x19\x81\x16\x90`\x14\x84\x10\x15a,\xA7W`\x01`\x01``\x1B\x03\x19`\x01`\x01``\x1B\x03\x19\x85`\x14\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[____`\xC0\x85\x87\x03\x12\x15a,\xC1W__\xFD[\x845\x93P` \x85\x015\x92Pa,\xD9\x86`@\x87\x01a%\xCEV[\x91Pa+\xE5\x86`\x80\x87\x01a%\xCEV[__`@\x83\x85\x03\x12\x15a,\xF9W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\x1F\x82\x11\x15a-OW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a--WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a-LW_\x81U`\x01\x01a-9V[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-nWa-na$\xCDV[a-\x82\x81a-|\x84Ta+\0V[\x84a-\x08V[` `\x1F\x82\x11`\x01\x81\x14a-\xB4W_\x83\x15a-\x9DWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua-LV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a-\xE3W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a-\xC3V[P\x84\x82\x10\x15a.\0W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_\x82a.)WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xEDWa\x03\xEDa..V[\x80\x82\x01\x80\x82\x11\x15a\x03\xEDWa\x03\xEDa..V[\x82\x81R`@` \x82\x01R_a.\x84`@\x83\x01\x84a&\x9AV[\x94\x93PPPPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a.\xB2W__\xFD[PQ\x91\x90PV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \x1D\xF3\xA5`\x12\x8F'\xA0a//\t^~W]\xD2q\xA85\x14\x862`7W\xC7\"4\xC7\xB1CdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610127575f3560e01c8063aa165c30116100a9578063d9f12db21161006e578063d9f12db214610370578063dab42d7e14610383578063ea0d8149146103a5578063ea194e2e146103b8578063f698da25146103cb575f5ffd5b8063aa165c30146102d9578063b05c8f6d146102ec578063bd30a0b914610313578063ca8aa7c714610336578063d40cda161461035d575f5ffd5b80637690e395116100ef5780637690e3951461023e5780637cffe48c146102515780638256909c1461027157806387ab86f4146102a35780639a43e3fb146102b8575f5ffd5b8063166aa1271461012b5780633b32a7bd146101655780634657e26a1461019057806350435add146101b757806354fd4d5014610236575b5f5ffd5b6101527f991b0a3376ce87f8ecc5d70962279ac09cdce934e8b5b9683e73c8ff087c7f8181565b6040519081526020015b60405180910390f35b61017861017336600461259c565b6103d3565b6040516001600160a01b03909116815260200161015c565b6101787f000000000000000000000000000000000000000000000000000000000000000081565b6102296101c536600461261d565b8151602080840151835180519083015185840151805190850151604080519687019790975295850193909352606084810192909252608084015260a083019190915260c082019290925260e001604051602081830303815290604052905092915050565b60405161015c91906126c8565b6102296103f3565b61015261024c366004612718565b610423565b61026461025f366004612776565b6104cb565b60405161015c91906127c4565b61028461027f3660046127d2565b6104f1565b604080516001600160a01b03909316835290151560208301520161015c565b6102b66102b1366004612878565b6105df565b005b6102cb6102c636600461259c565b6108f0565b60405161015c9291906128c3565b6102296102e736600461259c565b610ad8565b6101527fda86e76deaed01641f80ff5f72c372a038fa5182697aeb967e8b1f9819d58d8181565b61032661032136600461259c565b610c15565b604051901515815260200161015c565b6101787f000000000000000000000000000000000000000000000000000000000000000081565b6102b661036b366004612900565b610c52565b61015261037e366004612991565b610dc3565b6103266103913660046129d2565b5f9081526002602052604090205460ff1690565b6102b66103b33660046129e9565b610e5c565b6101526103c636600461259c565b610f9f565b6101526110c7565b5f6103de8383610ad8565b6103e790612a22565b60601c90505b92915050565b606061041e7f0000000000000000000000000000000000000000000000000000000000000000611180565b905090565b5f5f7fda86e76deaed01641f80ff5f72c372a038fa5182697aeb967e8b1f9819d58d8186865f015187602001518787604051610460929190612a7a565b6040805191829003822060208301969096526001600160a01b039485169082015292909116606083015263ffffffff16608082015260a081019190915260c0016040516020818303038152906040528051906020012090506104c1816111bd565b9695505050505050565b5f60015f6104d884611203565b815260208101919091526040015f205460ff1692915050565b5f5f5f60015f61050087611203565b815260208101919091526040015f9081205460ff169150600182600281111561052b5761052b612790565b0361053d5750835160208501206105ae565b600282600281111561055157610551612790565b03610595575f5f8680602001905181019061056c9190612a89565b60408051808201825283815260209081019283525f9384529151909152902092506105ae915050565b60405163fdea7c0960e01b815260040160405180910390fd5b5f818152600360205260409020546001600160a01b0316806105d08882610c15565b945094505050505b9250929050565b816105e981611261565b6106065760405163932d94f760e01b815260040160405180910390fd5b6040516309a961f360e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690631352c3e6906106549086908690600401612aab565b602060405180830381865afa15801561066f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106939190612ae1565b15828490916106e057604051631070287960e01b815282516001600160a01b03908116600483015260209093015163ffffffff166024820152911660448201526064015b60405180910390fd5b50505f60015f6106ef85611203565b815260208101919091526040015f9081205460ff16915081600281111561071857610718612790565b0361073657604051635cd3106d60e11b815260040160405180910390fd5b5f5f5f61074286611203565b815260208082019290925260409081015f9081206001600160a01b038916825283528190208151808301909252805460ff1615158252600181018054929391929184019161078f90612b00565b80601f01602080910402602001604051908101604052809291908181526020018280546107bb90612b00565b80156108065780601f106107dd57610100808354040283529160200191610806565b820191905f5260205f20905b8154815290600101906020018083116107e957829003601f168201915b5050505050815250509050805f01518486909161085c57604051632e40e18760e01b815282516001600160a01b03908116600483015260209093015163ffffffff166024820152911660448201526064016106d7565b50505f5f61086986611203565b815260208082019290925260409081015f9081206001600160a01b03891682529092528120805460ff19168155906108a460018301826123a1565b5050846001600160a01b03167f28d3c3cee49478ec6fd219cfd685cd15cd01d95cabf69b4b7b57f9eaa3eb644285846040516108e1929190612b38565b60405180910390a25050505050565b604080518082019091525f808252602082015261090b6123db565b5f60015f61091887611203565b815260208101919091526040015f205460ff169050600281600281111561094157610941612790565b1461095f5760405163fdea7c0960e01b815260040160405180910390fd5b5f5f5f61096b88611203565b815260208082019290925260409081015f9081206001600160a01b038916825283528190208151808301909252805460ff161515825260018101805492939192918401916109b890612b00565b80601f01602080910402602001604051908101604052809291908181526020018280546109e490612b00565b8015610a2f5780601f10610a0657610100808354040283529160200191610a2f565b820191905f5260205f20905b815481529060010190602001808311610a1257829003601f168201915b5050505050815250509050805f0151610a845750506040805180820182525f808252602080830182905283518085018552828152808201929092528351808501909452828452830191909152925090506105d8565b5f5f5f5f8460200151806020019051810190610aa09190612baa565b604080518082018252948552602080860194909452805180820190915291825291810191909152909b909a5098505050505050505050565b60605f60015f610ae786611203565b815260208101919091526040015f205460ff1690506001816002811115610b1057610b10612790565b14610b2e5760405163fdea7c0960e01b815260040160405180910390fd5b5f5f5f610b3a87611203565b815260208082019290925260409081015f9081206001600160a01b038816825283528190208151808301909252805460ff16151582526001810180549293919291840191610b8790612b00565b80601f0160208091040260200160405190810160405280929190818152602001828054610bb390612b00565b8015610bfe5780601f10610bd557610100808354040283529160200191610bfe565b820191905f5260205f20905b815481529060010190602001808311610be157829003601f168201915b505050919092525050506020015195945050505050565b5f5f5f610c2185611203565b815260208082019290925260409081015f9081206001600160a01b038616825290925290205460ff16905092915050565b85610c5c81611261565b610c795760405163932d94f760e01b815260040160405180910390fd5b5f60015f610c8689611203565b815260208101919091526040015f9081205460ff169150816002811115610caf57610caf612790565b03610ccd57604051635cd3106d60e11b815260040160405180910390fd5b5f5f610cd889611203565b815260208082019290925260409081015f9081206001600160a01b038c16825290925290205460ff1615610d1f576040516342ee68b560e01b815260040160405180910390fd5b6001816002811115610d3357610d33612790565b03610d4b57610d4687898888888861130b565b610d72565b6002816002811115610d5f57610d5f612790565b0361059557610d4687898888888861146c565b876001600160a01b03167f1201ce0c5e577111bce91e907fd99cb183da5edc1e3fb650ca40769e4e9176dd88838989604051610db19493929190612bf0565b60405180910390a25050505050505050565b81516020808401516040515f938493610e30937f991b0a3376ce87f8ecc5d70962279ac09cdce934e8b5b9683e73c8ff087c7f81938a93928991019485526001600160a01b039384166020860152918316604085015263ffffffff16606084015216608082015260a00190565b604051602081830303815290604052805190602001209050610e51816111bd565b9150505b9392505050565b8151610e6781611261565b610e845760405163932d94f760e01b815260040160405180910390fd5b6001826002811115610e9857610e98612790565b1480610eb557506002826002811115610eb357610eb3612790565b145b610ed25760405163fdea7c0960e01b815260040160405180910390fd5b5f60015f610edf86611203565b815260208101919091526040015f9081205460ff169150816002811115610f0857610f08612790565b14610f25576040516281f09f60e01b815260040160405180910390fd5b8260015f610f3287611203565b815260208101919091526040015f20805460ff19166001836002811115610f5b57610f5b612790565b02179055507fb2266cb118e57095fcdbedb24dabd9fc9f5127e2dbedf62ce6ee71696fb8b6e78484604051610f91929190612b38565b60405180910390a150505050565b5f5f5f5f610fac86611203565b815260208082019290925260409081015f9081206001600160a01b038716825283528190208151808301909252805460ff16151582526001810180549293919291840191610ff990612b00565b80601f016020809104026020016040519081016040528092919081815260200182805461102590612b00565b80156110705780601f1061104757610100808354040283529160200191611070565b820191905f5260205f20905b81548152906001019060200180831161105357829003601f168201915b50505050508152505090505f60015f61108887611203565b815260208101919091526040015f2054825160ff90911691506110b057505f91506103ed9050565b6110be826020015182611667565b95945050505050565b60408051808201909152600a81526922b4b3b2b72630bcb2b960b11b6020909101525f7f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f7f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea6111346116e8565b805160209182012060408051928301949094529281019190915260608101919091524660808201523060a082015260c00160405160208183030381529060405280519060200120905090565b60605f61118c8361175d565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b5f6111c66110c7565b60405161190160f01b6020820152602281019190915260428101839052606201604051602081830303815290604052805190602001209050919050565b5f815f0151826020015163ffffffff1660405160200161124992919060609290921b6001600160601b031916825260a01b6001600160a01b031916601482015260200190565b6040516020818303038152906040526103ed90612c4d565b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af11580156112e7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103ed9190612ae1565b6014831461132c5760405163d109118160e01b815260040160405180910390fd5b5f6113378486612c70565b60601c90508061135a57604051634935505f60e01b815260040160405180910390fd5b5f61139c86868080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525060019250611667915050565b5f8181526002602052604090205490915060ff16156113ce57604051630c7bc20160e11b815260040160405180910390fd5b5f6113da888a85610dc3565b905061141f838287878080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505f199250611784915050565b611461898989898080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508892506117dc915050565b505050505050505050565b60c0831461148d5760405163d109118160e01b815260040160405180910390fd5b604081146114ae57604051638baa579f60e01b815260040160405180910390fd5b604080518082019091525f80825260208201526114c96123db565b5f8080806114d9898b018b612cae565b93509350935093506040518060400160405280858152602001848152509550835f148015611505575082155b1561152357604051634935505f60e01b815260040160405180910390fd5b60408051808201909152918252602082015292505f91506115489050888a8989610423565b90505f8061155886880188612ce8565b604080518082019091528281526020810182905291935091505f611580858389898580611889565b915050806115a157604051638baa579f60e01b815260040160405180910390fd5b5f6115e38c8c8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201919091525060029250611667915050565b5f8181526002602052604090205490915060ff161561161557604051630c7bc20160e11b815260040160405180910390fd5b6116578e8e8e8e8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152508792506117dc915050565b5050505050505050505050505050565b5f600182600281111561167c5761167c612790565b0361168e5750815160208301206103ed565b60028260028111156116a2576116a2612790565b03610595575f5f848060200190518101906116bd9190612baa565b505060408051808201825283815260209081019283525f9384529151909152902092506103ed915050565b60605f6117147f0000000000000000000000000000000000000000000000000000000000000000611180565b9050805f8151811061172857611728612a66565b016020908101516040516001600160f81b03199091169181019190915260210160405160208183030381529060405291505090565b5f60ff8216601f8111156103ed57604051632cd44ac360e21b815260040160405180910390fd5b428110156117a557604051630819bdcd60e01b815260040160405180910390fd5b6117b96001600160a01b0385168484611951565b6117d657604051638baa579f60e01b815260040160405180910390fd5b50505050565b6040805180820190915260018152602081018390525f806117fc87611203565b815260208082019290925260409081015f9081206001600160a01b03881682528352208251815460ff19169015151781559082015160018201906118409082612d54565b5050505f908152600260209081526040808320805460ff191660011790556003909152902080546001600160a01b039093166001600160a01b0319909316929092179091555050565b5f5f5f611895896119a5565b90505f6118a48a89898c611a2f565b90505f6118bb6118b48a84611ada565b8b90611b4a565b90505f6118fd6118f6846118f06040805180820182525f80825260209182015281518083019092526001825260029082015290565b90611ada565b8590611b4a565b905087156119225761191982611911611bbe565b838c8b611c7e565b96509450611942565b6119358261192e611bbe565b838c611e92565b9550851561194257600194505b50505050965096945050505050565b5f5f5f61195e85856120c9565b90925090505f81600481111561197657611976612790565b1480156119945750856001600160a01b0316826001600160a01b0316145b806104c157506104c1868686612108565b604080518082019091525f80825260208201525f80806119d25f516020612eba5f395f51905f5286612e0f565b90505b6119de816121ef565b90935091505f516020612eba5f395f51905f528283098303611a16576040805180820190915290815260208101919091529392505050565b5f516020612eba5f395f51905f526001820890506119d5565b8251602080850151845180519083015186840151805190850151875188870151604080519889018e90528801989098526060870195909552608086019390935260a085019190915260c084015260e08301526101008201526101208101919091525f907f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f00000019061014001604051602081830303815290604052805190602001205f1c6110be9190612e0f565b604080518082019091525f8082526020820152611af5612400565b835181526020808501519082015260408082018490525f908360608460076107d05a03fa90508080611b2357fe5b5080611b4257604051632319df1960e11b815260040160405180910390fd5b505092915050565b604080518082019091525f8082526020820152611b6561241e565b835181526020808501518183015283516040808401919091529084015160608301525f908360808460066107d05a03fa90508080611b9f57fe5b5080611b425760405163d4b68fd760e01b815260040160405180910390fd5b611bc66123db565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b6040805180820182528681526020808201869052825180840190935286835282018490525f91829190611caf61243c565b5f5b6002811015611e66575f611cc6826006612e42565b9050848260028110611cda57611cda612a66565b60200201515183611ceb835f612e59565b600c8110611cfb57611cfb612a66565b6020020152848260028110611d1257611d12612a66565b60200201516020015183826001611d299190612e59565b600c8110611d3957611d39612a66565b6020020152838260028110611d5057611d50612a66565b6020020151515183611d63836002612e59565b600c8110611d7357611d73612a66565b6020020152838260028110611d8a57611d8a612a66565b6020020151516001602002015183611da3836003612e59565b600c8110611db357611db3612a66565b6020020152838260028110611dca57611dca612a66565b6020020151602001515f60028110611de457611de4612a66565b602002015183611df5836004612e59565b600c8110611e0557611e05612a66565b6020020152838260028110611e1c57611e1c612a66565b602002015160200151600160028110611e3757611e37612a66565b602002015183611e48836005612e59565b600c8110611e5857611e58612a66565b602002015250600101611cb1565b50611e6f61245b565b5f6020826101808560088cfa9151919c9115159b50909950505050505050505050565b6040805180820182528581526020808201859052825180840190935285835282018390525f91611ec061243c565b5f5b6002811015612077575f611ed7826006612e42565b9050848260028110611eeb57611eeb612a66565b60200201515183611efc835f612e59565b600c8110611f0c57611f0c612a66565b6020020152848260028110611f2357611f23612a66565b60200201516020015183826001611f3a9190612e59565b600c8110611f4a57611f4a612a66565b6020020152838260028110611f6157611f61612a66565b6020020151515183611f74836002612e59565b600c8110611f8457611f84612a66565b6020020152838260028110611f9b57611f9b612a66565b6020020151516001602002015183611fb4836003612e59565b600c8110611fc457611fc4612a66565b6020020152838260028110611fdb57611fdb612a66565b6020020151602001515f60028110611ff557611ff5612a66565b602002015183612006836004612e59565b600c811061201657612016612a66565b602002015283826002811061202d5761202d612a66565b60200201516020015160016002811061204857612048612a66565b602002015183612059836005612e59565b600c811061206957612069612a66565b602002015250600101611ec2565b5061208061245b565b5f6020826101808560086107d05a03fa9050808061209a57fe5b50806120b9576040516324ccc79360e21b815260040160405180910390fd5b5051151598975050505050505050565b5f5f82516041036120fd576020830151604084015160608501515f1a6120f18782858561226b565b945094505050506105d8565b505f905060026105d8565b5f5f5f856001600160a01b0316631626ba7e60e01b8686604051602401612130929190612e6c565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905161216e9190612e8c565b5f60405180830381855afa9150503d805f81146121a6576040519150601f19603f3d011682016040523d82523d5f602084013e6121ab565b606091505b50915091508180156121bf57506020815110155b80156104c157508051630b135d3f60e11b906121e49083016020908101908401612ea2565b149695505050505050565b5f80805f516020612eba5f395f51905f5260035f516020612eba5f395f51905f52865f516020612eba5f395f51905f52888909090890505f61225f827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f525f516020612eba5f395f51905f52612328565b91959194509092505050565b5f807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156122a057505f9050600361231f565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa1580156122f1573d5f5f3e3d5ffd5b5050604051601f1901519150506001600160a01b038116612319575f6001925092505061231f565b91505f90505b94509492505050565b5f5f61233261245b565b61233a612479565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa9250828061237757fe5b50826123965760405163d51edae360e01b815260040160405180910390fd5b505195945050505050565b5080546123ad90612b00565b5f825580601f106123bc575050565b601f0160209004905f5260205f20908101906123d89190612497565b50565b60405180604001604052806123ee6124af565b81526020016123fb6124af565b905290565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b5b808211156124ab575f8155600101612498565b5090565b60405180604001604052806002906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b6040805190810167ffffffffffffffff81118282101715612504576125046124cd565b60405290565b604051601f8201601f1916810167ffffffffffffffff81118282101715612533576125336124cd565b604052919050565b80356001600160a01b0381168114612551575f5ffd5b919050565b5f60408284031215612566575f5ffd5b61256e6124e1565b90506125798261253b565b8152602082013563ffffffff81168114612591575f5ffd5b602082015292915050565b5f5f606083850312156125ad575f5ffd5b6125b78484612556565b91506125c56040840161253b565b90509250929050565b5f82601f8301126125dd575f5ffd5b6125e7604061250a565b8060408401858111156125f8575f5ffd5b845b818110156126125780358452602093840193016125fa565b509095945050505050565b5f5f82840360c081121561262f575f5ffd5b604081121561263c575f5ffd5b6126446124e1565b843581526020808601359082015292506080603f1982011215612665575f5ffd5b5061266e6124e1565b61267b85604086016125ce565b815261268a85608086016125ce565b6020820152809150509250929050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f610e55602083018461269a565b5f5f83601f8401126126ea575f5ffd5b50813567ffffffffffffffff811115612701575f5ffd5b6020830191508360208285010111156105d8575f5ffd5b5f5f5f5f6080858703121561272b575f5ffd5b6127348561253b565b93506127438660208701612556565b9250606085013567ffffffffffffffff81111561275e575f5ffd5b61276a878288016126da565b95989497509550505050565b5f60408284031215612786575f5ffd5b610e558383612556565b634e487b7160e01b5f52602160045260245ffd5b600381106127c057634e487b7160e01b5f52602160045260245ffd5b9052565b602081016103ed82846127a4565b5f5f606083850312156127e3575f5ffd5b6127ed8484612556565b9150604083013567ffffffffffffffff811115612808575f5ffd5b8301601f81018513612818575f5ffd5b803567ffffffffffffffff811115612832576128326124cd565b612845601f8201601f191660200161250a565b818152866020838501011115612859575f5ffd5b816020840160208301375f602083830101528093505050509250929050565b5f5f60608385031215612889575f5ffd5b6128928361253b565b91506125c58460208501612556565b805f5b60028110156117d65781518452602093840193909101906001016128a4565b5f60c08201905083518252602084015160208301526128e66040830184516128a1565b60208301516128f860808401826128a1565b509392505050565b5f5f5f5f5f5f60a08789031215612915575f5ffd5b61291e8761253b565b955061292d8860208901612556565b9450606087013567ffffffffffffffff811115612948575f5ffd5b61295489828a016126da565b909550935050608087013567ffffffffffffffff811115612973575f5ffd5b61297f89828a016126da565b979a9699509497509295939492505050565b5f5f5f608084860312156129a3575f5ffd5b6129ac8461253b565b92506129bb8560208601612556565b91506129c96060850161253b565b90509250925092565b5f602082840312156129e2575f5ffd5b5035919050565b5f5f606083850312156129fa575f5ffd5b612a048484612556565b9150604083013560038110612a17575f5ffd5b809150509250929050565b805160208201516001600160601b0319811691906014821015612a5f576001600160601b03196001600160601b03198360140360031b1b82161692505b5050919050565b634e487b7160e01b5f52603260045260245ffd5b818382375f9101908152919050565b5f5f60408385031215612a9a575f5ffd5b505080516020909101519092909150565b6001600160a01b038316815260608101610e55602083018480516001600160a01b0316825260209081015163ffffffff16910152565b5f60208284031215612af1575f5ffd5b81518015158114610e55575f5ffd5b600181811c90821680612b1457607f821691505b602082108103612b3257634e487b7160e01b5f52602260045260245ffd5b50919050565b82516001600160a01b0316815260208084015163ffffffff169082015260608101610e5560408301846127a4565b5f82601f830112612b75575f5ffd5b612b7f604061250a565b806040840185811115612b90575f5ffd5b845b81811015612612578051845260209384019301612b92565b5f5f5f5f60c08587031215612bbd575f5ffd5b845160208601519094509250612bd68660408701612b66565b9150612be58660808701612b66565b905092959194509250565b84516001600160a01b0316815260208086015163ffffffff1690820152612c1a60408201856127a4565b60806060820152816080820152818360a08301375f81830160a090810191909152601f909201601f191601019392505050565b80516020808301519190811015612b32575f1960209190910360031b1b16919050565b80356001600160601b03198116906014841015612ca7576001600160601b03196001600160601b03198560140360031b1b82161691505b5092915050565b5f5f5f5f60c08587031215612cc1575f5ffd5b8435935060208501359250612cd986604087016125ce565b9150612be586608087016125ce565b5f5f60408385031215612cf9575f5ffd5b50508035926020909101359150565b601f821115612d4f57805f5260205f20601f840160051c81016020851015612d2d5750805b601f840160051c820191505b81811015612d4c575f8155600101612d39565b50505b505050565b815167ffffffffffffffff811115612d6e57612d6e6124cd565b612d8281612d7c8454612b00565b84612d08565b6020601f821160018114612db4575f8315612d9d5750848201515b5f19600385901b1c1916600184901b178455612d4c565b5f84815260208120601f198516915b82811015612de35787850151825560209485019460019092019101612dc3565b5084821015612e0057868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f82612e2957634e487b7160e01b5f52601260045260245ffd5b500690565b634e487b7160e01b5f52601160045260245ffd5b80820281158282048414176103ed576103ed612e2e565b808201808211156103ed576103ed612e2e565b828152604060208201525f612e84604083018461269a565b949350505050565b5f82518060208501845e5f920191825250919050565b5f60208284031215612eb2575f5ffd5b505191905056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a26469706673582212201df3a560128f27a0612f2f095e7e575dd271a835148632603757c72234c7b14364736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01'W_5`\xE0\x1C\x80c\xAA\x16\\0\x11a\0\xA9W\x80c\xD9\xF1-\xB2\x11a\0nW\x80c\xD9\xF1-\xB2\x14a\x03pW\x80c\xDA\xB4-~\x14a\x03\x83W\x80c\xEA\r\x81I\x14a\x03\xA5W\x80c\xEA\x19N.\x14a\x03\xB8W\x80c\xF6\x98\xDA%\x14a\x03\xCBW__\xFD[\x80c\xAA\x16\\0\x14a\x02\xD9W\x80c\xB0\\\x8Fm\x14a\x02\xECW\x80c\xBD0\xA0\xB9\x14a\x03\x13W\x80c\xCA\x8A\xA7\xC7\x14a\x036W\x80c\xD4\x0C\xDA\x16\x14a\x03]W__\xFD[\x80cv\x90\xE3\x95\x11a\0\xEFW\x80cv\x90\xE3\x95\x14a\x02>W\x80c|\xFF\xE4\x8C\x14a\x02QW\x80c\x82V\x90\x9C\x14a\x02qW\x80c\x87\xAB\x86\xF4\x14a\x02\xA3W\x80c\x9AC\xE3\xFB\x14a\x02\xB8W__\xFD[\x80c\x16j\xA1'\x14a\x01+W\x80c;2\xA7\xBD\x14a\x01eW\x80cFW\xE2j\x14a\x01\x90W\x80cPCZ\xDD\x14a\x01\xB7W\x80cT\xFDMP\x14a\x026W[__\xFD[a\x01R\x7F\x99\x1B\n3v\xCE\x87\xF8\xEC\xC5\xD7\tb'\x9A\xC0\x9C\xDC\xE94\xE8\xB5\xB9h>s\xC8\xFF\x08|\x7F\x81\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01xa\x01s6`\x04a%\x9CV[a\x03\xD3V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\\V[a\x01x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02)a\x01\xC56`\x04a&\x1DV[\x81Q` \x80\x84\x01Q\x83Q\x80Q\x90\x83\x01Q\x85\x84\x01Q\x80Q\x90\x85\x01Q`@\x80Q\x96\x87\x01\x97\x90\x97R\x95\x85\x01\x93\x90\x93R``\x84\x81\x01\x92\x90\x92R`\x80\x84\x01R`\xA0\x83\x01\x91\x90\x91R`\xC0\x82\x01\x92\x90\x92R`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`@Qa\x01\\\x91\x90a&\xC8V[a\x02)a\x03\xF3V[a\x01Ra\x02L6`\x04a'\x18V[a\x04#V[a\x02da\x02_6`\x04a'vV[a\x04\xCBV[`@Qa\x01\\\x91\x90a'\xC4V[a\x02\x84a\x02\x7F6`\x04a'\xD2V[a\x04\xF1V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R\x90\x15\x15` \x83\x01R\x01a\x01\\V[a\x02\xB6a\x02\xB16`\x04a(xV[a\x05\xDFV[\0[a\x02\xCBa\x02\xC66`\x04a%\x9CV[a\x08\xF0V[`@Qa\x01\\\x92\x91\x90a(\xC3V[a\x02)a\x02\xE76`\x04a%\x9CV[a\n\xD8V[a\x01R\x7F\xDA\x86\xE7m\xEA\xED\x01d\x1F\x80\xFF_r\xC3r\xA08\xFAQ\x82iz\xEB\x96~\x8B\x1F\x98\x19\xD5\x8D\x81\x81V[a\x03&a\x03!6`\x04a%\x9CV[a\x0C\x15V[`@Q\x90\x15\x15\x81R` \x01a\x01\\V[a\x01x\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xB6a\x03k6`\x04a)\0V[a\x0CRV[a\x01Ra\x03~6`\x04a)\x91V[a\r\xC3V[a\x03&a\x03\x916`\x04a)\xD2V[_\x90\x81R`\x02` R`@\x90 T`\xFF\x16\x90V[a\x02\xB6a\x03\xB36`\x04a)\xE9V[a\x0E\\V[a\x01Ra\x03\xC66`\x04a%\x9CV[a\x0F\x9FV[a\x01Ra\x10\xC7V[_a\x03\xDE\x83\x83a\n\xD8V[a\x03\xE7\x90a*\"V[``\x1C\x90P[\x92\x91PPV[``a\x04\x1E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x11\x80V[\x90P\x90V[__\x7F\xDA\x86\xE7m\xEA\xED\x01d\x1F\x80\xFF_r\xC3r\xA08\xFAQ\x82iz\xEB\x96~\x8B\x1F\x98\x19\xD5\x8D\x81\x86\x86_\x01Q\x87` \x01Q\x87\x87`@Qa\x04`\x92\x91\x90a*zV[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x96\x90\x96R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x90\x82\x01R\x92\x90\x91\x16``\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x04\xC1\x81a\x11\xBDV[\x96\x95PPPPPPV[_`\x01_a\x04\xD8\x84a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16\x92\x91PPV[___`\x01_a\x05\0\x87a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_\x90\x81 T`\xFF\x16\x91P`\x01\x82`\x02\x81\x11\x15a\x05+Wa\x05+a'\x90V[\x03a\x05=WP\x83Q` \x85\x01 a\x05\xAEV[`\x02\x82`\x02\x81\x11\x15a\x05QWa\x05Qa'\x90V[\x03a\x05\x95W__\x86\x80` \x01\x90Q\x81\x01\x90a\x05l\x91\x90a*\x89V[`@\x80Q\x80\x82\x01\x82R\x83\x81R` \x90\x81\x01\x92\x83R_\x93\x84R\x91Q\x90\x91R\x90 \x92Pa\x05\xAE\x91PPV[`@Qc\xFD\xEA|\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x03` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x80a\x05\xD0\x88\x82a\x0C\x15V[\x94P\x94PPPP[\x92P\x92\x90PV[\x81a\x05\xE9\x81a\x12aV[a\x06\x06W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\t\xA9a\xF3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x13R\xC3\xE6\x90a\x06T\x90\x86\x90\x86\x90`\x04\x01a*\xABV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06oW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x93\x91\x90a*\xE1V[\x15\x82\x84\x90\x91a\x06\xE0W`@Qc\x10p(y`\xE0\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R` \x90\x93\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x91\x16`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[PP_`\x01_a\x06\xEF\x85a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_\x90\x81 T`\xFF\x16\x91P\x81`\x02\x81\x11\x15a\x07\x18Wa\x07\x18a'\x90V[\x03a\x076W`@Qc\\\xD3\x10m`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___a\x07B\x86a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x89\x16\x82R\x83R\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T`\xFF\x16\x15\x15\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a\x07\x8F\x90a+\0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xBB\x90a+\0V[\x80\x15a\x08\x06W\x80`\x1F\x10a\x07\xDDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x06V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xE9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80_\x01Q\x84\x86\x90\x91a\x08\\W`@Qc.@\xE1\x87`\xE0\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x04\x83\x01R` \x90\x93\x01Qc\xFF\xFF\xFF\xFF\x16`$\x82\x01R\x91\x16`D\x82\x01R`d\x01a\x06\xD7V[PP__a\x08i\x86a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x89\x16\x82R\x90\x92R\x81 \x80T`\xFF\x19\x16\x81U\x90a\x08\xA4`\x01\x83\x01\x82a#\xA1V[PP\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F(\xD3\xC3\xCE\xE4\x94x\xECo\xD2\x19\xCF\xD6\x85\xCD\x15\xCD\x01\xD9\\\xAB\xF6\x9BK{W\xF9\xEA\xA3\xEBdB\x85\x84`@Qa\x08\xE1\x92\x91\x90a+8V[`@Q\x80\x91\x03\x90\xA2PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\t\x0Ba#\xDBV[_`\x01_a\t\x18\x87a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16\x90P`\x02\x81`\x02\x81\x11\x15a\tAWa\tAa'\x90V[\x14a\t_W`@Qc\xFD\xEA|\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___a\tk\x88a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x89\x16\x82R\x83R\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T`\xFF\x16\x15\x15\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a\t\xB8\x90a+\0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\xE4\x90a+\0V[\x80\x15a\n/W\x80`\x1F\x10a\n\x06Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n/V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n\x12W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80_\x01Qa\n\x84WPP`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x83Q\x80\x85\x01\x85R\x82\x81R\x80\x82\x01\x92\x90\x92R\x83Q\x80\x85\x01\x90\x94R\x82\x84R\x83\x01\x91\x90\x91R\x92P\x90Pa\x05\xD8V[____\x84` \x01Q\x80` \x01\x90Q\x81\x01\x90a\n\xA0\x91\x90a+\xAAV[`@\x80Q\x80\x82\x01\x82R\x94\x85R` \x80\x86\x01\x94\x90\x94R\x80Q\x80\x82\x01\x90\x91R\x91\x82R\x91\x81\x01\x91\x90\x91R\x90\x9B\x90\x9AP\x98PPPPPPPPPV[``_`\x01_a\n\xE7\x86a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16\x90P`\x01\x81`\x02\x81\x11\x15a\x0B\x10Wa\x0B\x10a'\x90V[\x14a\x0B.W`@Qc\xFD\xEA|\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___a\x0B:\x87a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x88\x16\x82R\x83R\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T`\xFF\x16\x15\x15\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a\x0B\x87\x90a+\0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0B\xB3\x90a+\0V[\x80\x15a\x0B\xFEW\x80`\x1F\x10a\x0B\xD5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0B\xFEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0B\xE1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP` \x01Q\x95\x94PPPPPV[___a\x0C!\x85a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x86\x16\x82R\x90\x92R\x90 T`\xFF\x16\x90P\x92\x91PPV[\x85a\x0C\\\x81a\x12aV[a\x0CyW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\x01_a\x0C\x86\x89a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_\x90\x81 T`\xFF\x16\x91P\x81`\x02\x81\x11\x15a\x0C\xAFWa\x0C\xAFa'\x90V[\x03a\x0C\xCDW`@Qc\\\xD3\x10m`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x0C\xD8\x89a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x8C\x16\x82R\x90\x92R\x90 T`\xFF\x16\x15a\r\x1FW`@QcB\xEEh\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81`\x02\x81\x11\x15a\r3Wa\r3a'\x90V[\x03a\rKWa\rF\x87\x89\x88\x88\x88\x88a\x13\x0BV[a\rrV[`\x02\x81`\x02\x81\x11\x15a\r_Wa\r_a'\x90V[\x03a\x05\x95Wa\rF\x87\x89\x88\x88\x88\x88a\x14lV[\x87`\x01`\x01`\xA0\x1B\x03\x16\x7F\x12\x01\xCE\x0C^Wq\x11\xBC\xE9\x1E\x90\x7F\xD9\x9C\xB1\x83\xDA^\xDC\x1E?\xB6P\xCA@v\x9EN\x91v\xDD\x88\x83\x89\x89`@Qa\r\xB1\x94\x93\x92\x91\x90a+\xF0V[`@Q\x80\x91\x03\x90\xA2PPPPPPPPV[\x81Q` \x80\x84\x01Q`@Q_\x93\x84\x93a\x0E0\x93\x7F\x99\x1B\n3v\xCE\x87\xF8\xEC\xC5\xD7\tb'\x9A\xC0\x9C\xDC\xE94\xE8\xB5\xB9h>s\xC8\xFF\x08|\x7F\x81\x93\x8A\x93\x92\x89\x91\x01\x94\x85R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16` \x86\x01R\x91\x83\x16`@\x85\x01Rc\xFF\xFF\xFF\xFF\x16``\x84\x01R\x16`\x80\x82\x01R`\xA0\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x0EQ\x81a\x11\xBDV[\x91PP[\x93\x92PPPV[\x81Qa\x0Eg\x81a\x12aV[a\x0E\x84W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x82`\x02\x81\x11\x15a\x0E\x98Wa\x0E\x98a'\x90V[\x14\x80a\x0E\xB5WP`\x02\x82`\x02\x81\x11\x15a\x0E\xB3Wa\x0E\xB3a'\x90V[\x14[a\x0E\xD2W`@Qc\xFD\xEA|\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`\x01_a\x0E\xDF\x86a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_\x90\x81 T`\xFF\x16\x91P\x81`\x02\x81\x11\x15a\x0F\x08Wa\x0F\x08a'\x90V[\x14a\x0F%W`@Qb\x81\xF0\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01_a\x0F2\x87a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16`\x01\x83`\x02\x81\x11\x15a\x0F[Wa\x0F[a'\x90V[\x02\x17\x90UP\x7F\xB2&l\xB1\x18\xE5p\x95\xFC\xDB\xED\xB2M\xAB\xD9\xFC\x9FQ'\xE2\xDB\xED\xF6,\xE6\xEEqio\xB8\xB6\xE7\x84\x84`@Qa\x0F\x91\x92\x91\x90a+8V[`@Q\x80\x91\x03\x90\xA1PPPPV[____a\x0F\xAC\x86a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x87\x16\x82R\x83R\x81\x90 \x81Q\x80\x83\x01\x90\x92R\x80T`\xFF\x16\x15\x15\x82R`\x01\x81\x01\x80T\x92\x93\x91\x92\x91\x84\x01\x91a\x0F\xF9\x90a+\0V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10%\x90a+\0V[\x80\x15a\x10pW\x80`\x1F\x10a\x10GWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10pV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10SW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P_`\x01_a\x10\x88\x87a\x12\x03V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T\x82Q`\xFF\x90\x91\x16\x91Pa\x10\xB0WP_\x91Pa\x03\xED\x90PV[a\x10\xBE\x82` \x01Q\x82a\x16gV[\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x90\x91\x01R_\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEAa\x114a\x16\xE8V[\x80Q` \x91\x82\x01 `@\x80Q\x92\x83\x01\x94\x90\x94R\x92\x81\x01\x91\x90\x91R``\x81\x01\x91\x90\x91RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[``_a\x11\x8C\x83a\x17]V[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[_a\x11\xC6a\x10\xC7V[`@Qa\x19\x01`\xF0\x1B` \x82\x01R`\"\x81\x01\x91\x90\x91R`B\x81\x01\x83\x90R`b\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a\x12I\x92\x91\x90``\x92\x90\x92\x1B`\x01`\x01``\x1B\x03\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x03\xED\x90a,MV[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12\xE7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xED\x91\x90a*\xE1V[`\x14\x83\x14a\x13,W`@Qc\xD1\t\x11\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x137\x84\x86a,pV[``\x1C\x90P\x80a\x13ZW`@QcI5P_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\x9C\x86\x86\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP`\x01\x92Pa\x16g\x91PPV[_\x81\x81R`\x02` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x13\xCEW`@Qc\x0C{\xC2\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13\xDA\x88\x8A\x85a\r\xC3V[\x90Pa\x14\x1F\x83\x82\x87\x87\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP_\x19\x92Pa\x17\x84\x91PPV[a\x14a\x89\x89\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x88\x92Pa\x17\xDC\x91PPV[PPPPPPPPPV[`\xC0\x83\x14a\x14\x8DW`@Qc\xD1\t\x11\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x81\x14a\x14\xAEW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x14\xC9a#\xDBV[_\x80\x80\x80a\x14\xD9\x89\x8B\x01\x8Ba,\xAEV[\x93P\x93P\x93P\x93P`@Q\x80`@\x01`@R\x80\x85\x81R` \x01\x84\x81RP\x95P\x83_\x14\x80\x15a\x15\x05WP\x82\x15[\x15a\x15#W`@QcI5P_`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R\x91\x82R` \x82\x01R\x92P_\x91Pa\x15H\x90P\x88\x8A\x89\x89a\x04#V[\x90P_\x80a\x15X\x86\x88\x01\x88a,\xE8V[`@\x80Q\x80\x82\x01\x90\x91R\x82\x81R` \x81\x01\x82\x90R\x91\x93P\x91P_a\x15\x80\x85\x83\x89\x89\x85\x80a\x18\x89V[\x91PP\x80a\x15\xA1W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x15\xE3\x8C\x8C\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP`\x02\x92Pa\x16g\x91PPV[_\x81\x81R`\x02` R`@\x90 T\x90\x91P`\xFF\x16\x15a\x16\x15W`@Qc\x0C{\xC2\x01`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x16W\x8E\x8E\x8E\x8E\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x87\x92Pa\x17\xDC\x91PPV[PPPPPPPPPPPPPPV[_`\x01\x82`\x02\x81\x11\x15a\x16|Wa\x16|a'\x90V[\x03a\x16\x8EWP\x81Q` \x83\x01 a\x03\xEDV[`\x02\x82`\x02\x81\x11\x15a\x16\xA2Wa\x16\xA2a'\x90V[\x03a\x05\x95W__\x84\x80` \x01\x90Q\x81\x01\x90a\x16\xBD\x91\x90a+\xAAV[PP`@\x80Q\x80\x82\x01\x82R\x83\x81R` \x90\x81\x01\x92\x83R_\x93\x84R\x91Q\x90\x91R\x90 \x92Pa\x03\xED\x91PPV[``_a\x17\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x11\x80V[\x90P\x80_\x81Q\x81\x10a\x17(Wa\x17(a*fV[\x01` \x90\x81\x01Q`@Q`\x01`\x01`\xF8\x1B\x03\x19\x90\x91\x16\x91\x81\x01\x91\x90\x91R`!\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x90V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x03\xEDW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[B\x81\x10\x15a\x17\xA5W`@Qc\x08\x19\xBD\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17\xB9`\x01`\x01`\xA0\x1B\x03\x85\x16\x84\x84a\x19QV[a\x17\xD6W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R` \x81\x01\x83\x90R_\x80a\x17\xFC\x87a\x12\x03V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_\x90\x81 `\x01`\x01`\xA0\x1B\x03\x88\x16\x82R\x83R \x82Q\x81T`\xFF\x19\x16\x90\x15\x15\x17\x81U\x90\x82\x01Q`\x01\x82\x01\x90a\x18@\x90\x82a-TV[PPP_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16`\x01\x17\x90U`\x03\x90\x91R\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91UPPV[___a\x18\x95\x89a\x19\xA5V[\x90P_a\x18\xA4\x8A\x89\x89\x8Ca\x1A/V[\x90P_a\x18\xBBa\x18\xB4\x8A\x84a\x1A\xDAV[\x8B\x90a\x1BJV[\x90P_a\x18\xFDa\x18\xF6\x84a\x18\xF0`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a\x1A\xDAV[\x85\x90a\x1BJV[\x90P\x87\x15a\x19\"Wa\x19\x19\x82a\x19\x11a\x1B\xBEV[\x83\x8C\x8Ba\x1C~V[\x96P\x94Pa\x19BV[a\x195\x82a\x19.a\x1B\xBEV[\x83\x8Ca\x1E\x92V[\x95P\x85\x15a\x19BW`\x01\x94P[PPPP\x96P\x96\x94PPPPPV[___a\x19^\x85\x85a \xC9V[\x90\x92P\x90P_\x81`\x04\x81\x11\x15a\x19vWa\x19va'\x90V[\x14\x80\x15a\x19\x94WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x80a\x04\xC1WPa\x04\xC1\x86\x86\x86a!\x08V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80\x80a\x19\xD2_Q` a.\xBA_9_Q\x90_R\x86a.\x0FV[\x90P[a\x19\xDE\x81a!\xEFV[\x90\x93P\x91P_Q` a.\xBA_9_Q\x90_R\x82\x83\t\x83\x03a\x1A\x16W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[_Q` a.\xBA_9_Q\x90_R`\x01\x82\x08\x90Pa\x19\xD5V[\x82Q` \x80\x85\x01Q\x84Q\x80Q\x90\x83\x01Q\x86\x84\x01Q\x80Q\x90\x85\x01Q\x87Q\x88\x87\x01Q`@\x80Q\x98\x89\x01\x8E\x90R\x88\x01\x98\x90\x98R``\x87\x01\x95\x90\x95R`\x80\x86\x01\x93\x90\x93R`\xA0\x85\x01\x91\x90\x91R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x81\x01\x91\x90\x91R_\x90\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90a\x01@\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1Ca\x10\xBE\x91\x90a.\x0FV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x1A\xF5a$\0V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R_\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x1B#W\xFE[P\x80a\x1BBW`@Qc#\x19\xDF\x19`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01Ra\x1Bea$\x1EV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R_\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80a\x1B\x9FW\xFE[P\x80a\x1BBW`@Qc\xD4\xB6\x8F\xD7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1B\xC6a#\xDBV[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R_\x91\x82\x91\x90a\x1C\xAFa$<V[_[`\x02\x81\x10\x15a\x1EfW_a\x1C\xC6\x82`\x06a.BV[\x90P\x84\x82`\x02\x81\x10a\x1C\xDAWa\x1C\xDAa*fV[` \x02\x01QQ\x83a\x1C\xEB\x83_a.YV[`\x0C\x81\x10a\x1C\xFBWa\x1C\xFBa*fV[` \x02\x01R\x84\x82`\x02\x81\x10a\x1D\x12Wa\x1D\x12a*fV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x1D)\x91\x90a.YV[`\x0C\x81\x10a\x1D9Wa\x1D9a*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1DPWa\x1DPa*fV[` \x02\x01QQQ\x83a\x1Dc\x83`\x02a.YV[`\x0C\x81\x10a\x1DsWa\x1Dsa*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1D\x8AWa\x1D\x8Aa*fV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x1D\xA3\x83`\x03a.YV[`\x0C\x81\x10a\x1D\xB3Wa\x1D\xB3a*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1D\xCAWa\x1D\xCAa*fV[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x1D\xE4Wa\x1D\xE4a*fV[` \x02\x01Q\x83a\x1D\xF5\x83`\x04a.YV[`\x0C\x81\x10a\x1E\x05Wa\x1E\x05a*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1E\x1CWa\x1E\x1Ca*fV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a\x1E7Wa\x1E7a*fV[` \x02\x01Q\x83a\x1EH\x83`\x05a.YV[`\x0C\x81\x10a\x1EXWa\x1EXa*fV[` \x02\x01RP`\x01\x01a\x1C\xB1V[Pa\x1Eoa$[V[_` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x85\x90R\x82Q\x80\x84\x01\x90\x93R\x85\x83R\x82\x01\x83\x90R_\x91a\x1E\xC0a$<V[_[`\x02\x81\x10\x15a wW_a\x1E\xD7\x82`\x06a.BV[\x90P\x84\x82`\x02\x81\x10a\x1E\xEBWa\x1E\xEBa*fV[` \x02\x01QQ\x83a\x1E\xFC\x83_a.YV[`\x0C\x81\x10a\x1F\x0CWa\x1F\x0Ca*fV[` \x02\x01R\x84\x82`\x02\x81\x10a\x1F#Wa\x1F#a*fV[` \x02\x01Q` \x01Q\x83\x82`\x01a\x1F:\x91\x90a.YV[`\x0C\x81\x10a\x1FJWa\x1FJa*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1FaWa\x1Faa*fV[` \x02\x01QQQ\x83a\x1Ft\x83`\x02a.YV[`\x0C\x81\x10a\x1F\x84Wa\x1F\x84a*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1F\x9BWa\x1F\x9Ba*fV[` \x02\x01QQ`\x01` \x02\x01Q\x83a\x1F\xB4\x83`\x03a.YV[`\x0C\x81\x10a\x1F\xC4Wa\x1F\xC4a*fV[` \x02\x01R\x83\x82`\x02\x81\x10a\x1F\xDBWa\x1F\xDBa*fV[` \x02\x01Q` \x01Q_`\x02\x81\x10a\x1F\xF5Wa\x1F\xF5a*fV[` \x02\x01Q\x83a \x06\x83`\x04a.YV[`\x0C\x81\x10a \x16Wa \x16a*fV[` \x02\x01R\x83\x82`\x02\x81\x10a -Wa -a*fV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a HWa Ha*fV[` \x02\x01Q\x83a Y\x83`\x05a.YV[`\x0C\x81\x10a iWa ia*fV[` \x02\x01RP`\x01\x01a\x1E\xC2V[Pa \x80a$[V[_` \x82a\x01\x80\x85`\x08a\x07\xD0Z\x03\xFA\x90P\x80\x80a \x9AW\xFE[P\x80a \xB9W`@Qc$\xCC\xC7\x93`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x15\x15\x98\x97PPPPPPPPV[__\x82Q`A\x03a \xFDW` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa \xF1\x87\x82\x85\x85a\"kV[\x94P\x94PPPPa\x05\xD8V[P_\x90P`\x02a\x05\xD8V[___\x85`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x86\x86`@Q`$\x01a!0\x92\x91\x90a.lV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa!n\x91\x90a.\x8CV[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a!\xA6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a!\xABV[``\x91P[P\x91P\x91P\x81\x80\x15a!\xBFWP` \x81Q\x10\x15[\x80\x15a\x04\xC1WP\x80Qc\x0B\x13]?`\xE1\x1B\x90a!\xE4\x90\x83\x01` \x90\x81\x01\x90\x84\x01a.\xA2V[\x14\x96\x95PPPPPPV[_\x80\x80_Q` a.\xBA_9_Q\x90_R`\x03_Q` a.\xBA_9_Q\x90_R\x86_Q` a.\xBA_9_Q\x90_R\x88\x89\t\t\x08\x90P_a\"_\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R_Q` a.\xBA_9_Q\x90_Ra#(V[\x91\x95\x91\x94P\x90\x92PPPV[_\x80\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x83\x11\x15a\"\xA0WP_\x90P`\x03a#\x1FV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\"\xF1W=__>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a#\x19W_`\x01\x92P\x92PPa#\x1FV[\x91P_\x90P[\x94P\x94\x92PPPV[__a#2a$[V[a#:a$yV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80a#wW\xFE[P\x82a#\x96W`@Qc\xD5\x1E\xDA\xE3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PQ\x95\x94PPPPPV[P\x80Ta#\xAD\x90a+\0V[_\x82U\x80`\x1F\x10a#\xBCWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a#\xD8\x91\x90a$\x97V[PV[`@Q\x80`@\x01`@R\x80a#\xEEa$\xAFV[\x81R` \x01a#\xFBa$\xAFV[\x90R\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a$\xABW_\x81U`\x01\x01a$\x98V[P\x90V[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\x04Wa%\x04a$\xCDV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%3Wa%3a$\xCDV[`@R\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a%QW__\xFD[\x91\x90PV[_`@\x82\x84\x03\x12\x15a%fW__\xFD[a%na$\xE1V[\x90Pa%y\x82a%;V[\x81R` \x82\x015c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a%\x91W__\xFD[` \x82\x01R\x92\x91PPV[__``\x83\x85\x03\x12\x15a%\xADW__\xFD[a%\xB7\x84\x84a%VV[\x91Pa%\xC5`@\x84\x01a%;V[\x90P\x92P\x92\x90PV[_\x82`\x1F\x83\x01\x12a%\xDDW__\xFD[a%\xE7`@a%\nV[\x80`@\x84\x01\x85\x81\x11\x15a%\xF8W__\xFD[\x84[\x81\x81\x10\x15a&\x12W\x805\x84R` \x93\x84\x01\x93\x01a%\xFAV[P\x90\x95\x94PPPPPV[__\x82\x84\x03`\xC0\x81\x12\x15a&/W__\xFD[`@\x81\x12\x15a&<W__\xFD[a&Da$\xE1V[\x845\x81R` \x80\x86\x015\x90\x82\x01R\x92P`\x80`?\x19\x82\x01\x12\x15a&eW__\xFD[Pa&na$\xE1V[a&{\x85`@\x86\x01a%\xCEV[\x81Ra&\x8A\x85`\x80\x86\x01a%\xCEV[` \x82\x01R\x80\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0EU` \x83\x01\x84a&\x9AV[__\x83`\x1F\x84\x01\x12a&\xEAW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'\x01W__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x05\xD8W__\xFD[____`\x80\x85\x87\x03\x12\x15a'+W__\xFD[a'4\x85a%;V[\x93Pa'C\x86` \x87\x01a%VV[\x92P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a'^W__\xFD[a'j\x87\x82\x88\x01a&\xDAV[\x95\x98\x94\x97P\x95PPPPV[_`@\x82\x84\x03\x12\x15a'\x86W__\xFD[a\x0EU\x83\x83a%VV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[`\x03\x81\x10a'\xC0WcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90RV[` \x81\x01a\x03\xED\x82\x84a'\xA4V[__``\x83\x85\x03\x12\x15a'\xE3W__\xFD[a'\xED\x84\x84a%VV[\x91P`@\x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x08W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a(\x18W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(2Wa(2a$\xCDV[a(E`\x1F\x82\x01`\x1F\x19\x16` \x01a%\nV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a(YW__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[__``\x83\x85\x03\x12\x15a(\x89W__\xFD[a(\x92\x83a%;V[\x91Pa%\xC5\x84` \x85\x01a%VV[\x80_[`\x02\x81\x10\x15a\x17\xD6W\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a(\xA4V[_`\xC0\x82\x01\x90P\x83Q\x82R` \x84\x01Q` \x83\x01Ra(\xE6`@\x83\x01\x84Qa(\xA1V[` \x83\x01Qa(\xF8`\x80\x84\x01\x82a(\xA1V[P\x93\x92PPPV[______`\xA0\x87\x89\x03\x12\x15a)\x15W__\xFD[a)\x1E\x87a%;V[\x95Pa)-\x88` \x89\x01a%VV[\x94P``\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)HW__\xFD[a)T\x89\x82\x8A\x01a&\xDAV[\x90\x95P\x93PP`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)sW__\xFD[a)\x7F\x89\x82\x8A\x01a&\xDAV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[___`\x80\x84\x86\x03\x12\x15a)\xA3W__\xFD[a)\xAC\x84a%;V[\x92Pa)\xBB\x85` \x86\x01a%VV[\x91Pa)\xC9``\x85\x01a%;V[\x90P\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a)\xE2W__\xFD[P5\x91\x90PV[__``\x83\x85\x03\x12\x15a)\xFAW__\xFD[a*\x04\x84\x84a%VV[\x91P`@\x83\x015`\x03\x81\x10a*\x17W__\xFD[\x80\x91PP\x92P\x92\x90PV[\x80Q` \x82\x01Q`\x01`\x01``\x1B\x03\x19\x81\x16\x91\x90`\x14\x82\x10\x15a*_W`\x01`\x01``\x1B\x03\x19`\x01`\x01``\x1B\x03\x19\x83`\x14\x03`\x03\x1B\x1B\x82\x16\x16\x92P[PP\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[__`@\x83\x85\x03\x12\x15a*\x9AW__\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x0EU` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[_` \x82\x84\x03\x12\x15a*\xF1W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0EUW__\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80a+\x14W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a+2WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x81\x01a\x0EU`@\x83\x01\x84a'\xA4V[_\x82`\x1F\x83\x01\x12a+uW__\xFD[a+\x7F`@a%\nV[\x80`@\x84\x01\x85\x81\x11\x15a+\x90W__\xFD[\x84[\x81\x81\x10\x15a&\x12W\x80Q\x84R` \x93\x84\x01\x93\x01a+\x92V[____`\xC0\x85\x87\x03\x12\x15a+\xBDW__\xFD[\x84Q` \x86\x01Q\x90\x94P\x92Pa+\xD6\x86`@\x87\x01a+fV[\x91Pa+\xE5\x86`\x80\x87\x01a+fV[\x90P\x92\x95\x91\x94P\x92PV[\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra,\x1A`@\x82\x01\x85a'\xA4V[`\x80``\x82\x01R\x81`\x80\x82\x01R\x81\x83`\xA0\x83\x017_\x81\x83\x01`\xA0\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01`\x1F\x19\x16\x01\x01\x93\x92PPPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a+2W_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[\x805`\x01`\x01``\x1B\x03\x19\x81\x16\x90`\x14\x84\x10\x15a,\xA7W`\x01`\x01``\x1B\x03\x19`\x01`\x01``\x1B\x03\x19\x85`\x14\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[____`\xC0\x85\x87\x03\x12\x15a,\xC1W__\xFD[\x845\x93P` \x85\x015\x92Pa,\xD9\x86`@\x87\x01a%\xCEV[\x91Pa+\xE5\x86`\x80\x87\x01a%\xCEV[__`@\x83\x85\x03\x12\x15a,\xF9W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\x1F\x82\x11\x15a-OW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a--WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a-LW_\x81U`\x01\x01a-9V[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-nWa-na$\xCDV[a-\x82\x81a-|\x84Ta+\0V[\x84a-\x08V[` `\x1F\x82\x11`\x01\x81\x14a-\xB4W_\x83\x15a-\x9DWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua-LV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a-\xE3W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a-\xC3V[P\x84\x82\x10\x15a.\0W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_\x82a.)WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xEDWa\x03\xEDa..V[\x80\x82\x01\x80\x82\x11\x15a\x03\xEDWa\x03\xEDa..V[\x82\x81R`@` \x82\x01R_a.\x84`@\x83\x01\x84a&\x9AV[\x94\x93PPPPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a.\xB2W__\xFD[PQ\x91\x90PV\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \x1D\xF3\xA5`\x12\x8F'\xA0a//\t^~W]\xD2q\xA85\x14\x862`7W\xC7\"4\xC7\xB1CdsolcC\0\x08\x1B\x003",
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
    /**Custom error with signature `InvalidPermissions()` and selector `0x932d94f7`.
    ```solidity
    error InvalidPermissions();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidPermissions;
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
        impl ::core::convert::From<InvalidPermissions> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPermissions) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPermissions {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPermissions {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPermissions()";
            const SELECTOR: [u8; 4] = [147u8, 45u8, 148u8, 247u8];
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
    /**Constructor`.
    ```solidity
    constructor(address _permissionController, address _allocationManager, string _version);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _permissionController: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
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
                        value._permissionController,
                        value._allocationManager,
                        value._version,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _permissionController: tuple.0,
                        _allocationManager: tuple.1,
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
                        &self._permissionController,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._version,
                    ),
                )
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `BN254_KEY_REGISTRATION_TYPEHASH()` and selector `0xb05c8f6d`.
    ```solidity
    function BN254_KEY_REGISTRATION_TYPEHASH() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BN254_KEY_REGISTRATION_TYPEHASHCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`BN254_KEY_REGISTRATION_TYPEHASH()`](BN254_KEY_REGISTRATION_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BN254_KEY_REGISTRATION_TYPEHASHReturn {
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
            impl ::core::convert::From<BN254_KEY_REGISTRATION_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: BN254_KEY_REGISTRATION_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BN254_KEY_REGISTRATION_TYPEHASHCall {
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
            impl ::core::convert::From<BN254_KEY_REGISTRATION_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: BN254_KEY_REGISTRATION_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for BN254_KEY_REGISTRATION_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for BN254_KEY_REGISTRATION_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BN254_KEY_REGISTRATION_TYPEHASH()";
            const SELECTOR: [u8; 4] = [176u8, 92u8, 143u8, 109u8];
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
                        let r: BN254_KEY_REGISTRATION_TYPEHASHReturn = r.into();
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
                    let r: BN254_KEY_REGISTRATION_TYPEHASHReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `ECDSA_KEY_REGISTRATION_TYPEHASH()` and selector `0x166aa127`.
    ```solidity
    function ECDSA_KEY_REGISTRATION_TYPEHASH() external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSA_KEY_REGISTRATION_TYPEHASHCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`ECDSA_KEY_REGISTRATION_TYPEHASH()`](ECDSA_KEY_REGISTRATION_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECDSA_KEY_REGISTRATION_TYPEHASHReturn {
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
            impl ::core::convert::From<ECDSA_KEY_REGISTRATION_TYPEHASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: ECDSA_KEY_REGISTRATION_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSA_KEY_REGISTRATION_TYPEHASHCall {
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
            impl ::core::convert::From<ECDSA_KEY_REGISTRATION_TYPEHASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ECDSA_KEY_REGISTRATION_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECDSA_KEY_REGISTRATION_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ECDSA_KEY_REGISTRATION_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::FixedBytes<32>;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECDSA_KEY_REGISTRATION_TYPEHASH()";
            const SELECTOR: [u8; 4] = [22u8, 106u8, 161u8, 39u8];
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
                        let r: ECDSA_KEY_REGISTRATION_TYPEHASHReturn = r.into();
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
                    let r: ECDSA_KEY_REGISTRATION_TYPEHASHReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `allocationManager()` and selector `0xca8aa7c7`.
    ```solidity
    function allocationManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`allocationManager()`](allocationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerReturn {
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
            impl ::core::convert::From<allocationManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerCall {
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
            impl ::core::convert::From<allocationManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allocationManager()";
            const SELECTOR: [u8; 4] = [202u8, 138u8, 167u8, 199u8];
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
                        let r: allocationManagerReturn = r.into();
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
                    let r: allocationManagerReturn = r.into();
                    r._0
                })
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
    /**Function with signature `permissionController()` and selector `0x4657e26a`.
    ```solidity
    function permissionController() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionControllerCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`permissionController()`](permissionControllerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct permissionControllerReturn {
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
            impl ::core::convert::From<permissionControllerCall> for UnderlyingRustTuple<'_> {
                fn from(value: permissionControllerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permissionControllerCall {
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
            impl ::core::convert::From<permissionControllerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: permissionControllerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permissionControllerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permissionControllerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "permissionController()";
            const SELECTOR: [u8; 4] = [70u8, 87u8, 226u8, 106u8];
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
                        let r: permissionControllerReturn = r.into();
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
                    let r: permissionControllerReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `registerKey(address,(address,uint32),bytes,bytes)` and selector `0xd40cda16`.
    ```solidity
    function registerKey(address operator, OperatorSet memory operatorSet, bytes memory keyData, bytes memory signature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerKeyCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub keyData: alloy::sol_types::private::Bytes,
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
                        value.keyData,
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
                        keyData: tuple.2,
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
                        &self.keyData,
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
    ///Container for all the [`KeyRegistrar`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum KeyRegistrarCalls {
        #[allow(missing_docs)]
        BN254_KEY_REGISTRATION_TYPEHASH(BN254_KEY_REGISTRATION_TYPEHASHCall),
        #[allow(missing_docs)]
        ECDSA_KEY_REGISTRATION_TYPEHASH(ECDSA_KEY_REGISTRATION_TYPEHASHCall),
        #[allow(missing_docs)]
        allocationManager(allocationManagerCall),
        #[allow(missing_docs)]
        configureOperatorSet(configureOperatorSetCall),
        #[allow(missing_docs)]
        deregisterKey(deregisterKeyCall),
        #[allow(missing_docs)]
        domainSeparator(domainSeparatorCall),
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
        permissionController(permissionControllerCall),
        #[allow(missing_docs)]
        registerKey(registerKeyCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    #[automatically_derived]
    impl KeyRegistrarCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [22u8, 106u8, 161u8, 39u8],
            [59u8, 50u8, 167u8, 189u8],
            [70u8, 87u8, 226u8, 106u8],
            [80u8, 67u8, 90u8, 221u8],
            [84u8, 253u8, 77u8, 80u8],
            [118u8, 144u8, 227u8, 149u8],
            [124u8, 255u8, 228u8, 140u8],
            [130u8, 86u8, 144u8, 156u8],
            [135u8, 171u8, 134u8, 244u8],
            [154u8, 67u8, 227u8, 251u8],
            [170u8, 22u8, 92u8, 48u8],
            [176u8, 92u8, 143u8, 109u8],
            [189u8, 48u8, 160u8, 185u8],
            [202u8, 138u8, 167u8, 199u8],
            [212u8, 12u8, 218u8, 22u8],
            [217u8, 241u8, 45u8, 178u8],
            [218u8, 180u8, 45u8, 126u8],
            [234u8, 13u8, 129u8, 73u8],
            [234u8, 25u8, 78u8, 46u8],
            [246u8, 152u8, 218u8, 37u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for KeyRegistrarCalls {
        const NAME: &'static str = "KeyRegistrarCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BN254_KEY_REGISTRATION_TYPEHASH(_) => {
                    <BN254_KEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ECDSA_KEY_REGISTRATION_TYPEHASH(_) => {
                    <ECDSA_KEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::configureOperatorSet(_) => {
                    <configureOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterKey(_) => <deregisterKeyCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::domainSeparator(_) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::permissionController(_) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
            static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls>] = &[
                {
                    fn ECDSA_KEY_REGISTRATION_TYPEHASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <ECDSA_KEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KeyRegistrarCalls::ECDSA_KEY_REGISTRATION_TYPEHASH)
                    }
                    ECDSA_KEY_REGISTRATION_TYPEHASH
                },
                {
                    fn getECDSAAddress(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getECDSAAddressCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::getECDSAAddress)
                    }
                    getECDSAAddress
                },
                {
                    fn permissionController(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <permissionControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::permissionController)
                    }
                    permissionController
                },
                {
                    fn encodeBN254KeyData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <encodeBN254KeyDataCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::encodeBN254KeyData)
                    }
                    encodeBN254KeyData
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::version)
                    }
                    version
                },
                {
                    fn getBN254KeyRegistrationMessageHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getBN254KeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KeyRegistrarCalls::getBN254KeyRegistrationMessageHash)
                    }
                    getBN254KeyRegistrationMessageHash
                },
                {
                    fn getOperatorSetCurveType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getOperatorSetCurveTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(KeyRegistrarCalls::getOperatorSetCurveType)
                    }
                    getOperatorSetCurveType
                },
                {
                    fn getOperatorFromSigningKey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getOperatorFromSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(KeyRegistrarCalls::getOperatorFromSigningKey)
                    }
                    getOperatorFromSigningKey
                },
                {
                    fn deregisterKey(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <deregisterKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::deregisterKey)
                    }
                    deregisterKey
                },
                {
                    fn getBN254Key(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getBN254KeyCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::getBN254Key)
                    }
                    getBN254Key
                },
                {
                    fn getECDSAKey(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getECDSAKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::getECDSAKey)
                    }
                    getECDSAKey
                },
                {
                    fn BN254_KEY_REGISTRATION_TYPEHASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <BN254_KEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KeyRegistrarCalls::BN254_KEY_REGISTRATION_TYPEHASH)
                    }
                    BN254_KEY_REGISTRATION_TYPEHASH
                },
                {
                    fn isRegistered(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <isRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::isRegistered)
                    }
                    isRegistered
                },
                {
                    fn allocationManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn registerKey(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <registerKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::registerKey)
                    }
                    registerKey
                },
                {
                    fn getECDSAKeyRegistrationMessageHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getECDSAKeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(KeyRegistrarCalls::getECDSAKeyRegistrationMessageHash)
                    }
                    getECDSAKeyRegistrationMessageHash
                },
                {
                    fn isKeyGloballyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <isKeyGloballyRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(KeyRegistrarCalls::isKeyGloballyRegistered)
                    }
                    isKeyGloballyRegistered
                },
                {
                    fn configureOperatorSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <configureOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::configureOperatorSet)
                    }
                    configureOperatorSet
                },
                {
                    fn getKeyHash(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getKeyHashCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::getKeyHash)
                    }
                    getKeyHash
                },
                {
                    fn domainSeparator(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(KeyRegistrarCalls::domainSeparator)
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
            )
                -> alloy_sol_types::Result<KeyRegistrarCalls>] = &[
                {
                    fn ECDSA_KEY_REGISTRATION_TYPEHASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <ECDSA_KEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarCalls::ECDSA_KEY_REGISTRATION_TYPEHASH)
                    }
                    ECDSA_KEY_REGISTRATION_TYPEHASH
                },
                {
                    fn getECDSAAddress(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getECDSAAddressCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(KeyRegistrarCalls::getECDSAAddress)
                    }
                    getECDSAAddress
                },
                {
                    fn permissionController(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <permissionControllerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarCalls::permissionController)
                    }
                    permissionController
                },
                {
                    fn encodeBN254KeyData(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <encodeBN254KeyDataCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarCalls::encodeBN254KeyData)
                    }
                    encodeBN254KeyData
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarCalls::version)
                    }
                    version
                },
                {
                    fn getBN254KeyRegistrationMessageHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getBN254KeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarCalls::getBN254KeyRegistrationMessageHash)
                    }
                    getBN254KeyRegistrationMessageHash
                },
                {
                    fn getOperatorSetCurveType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getOperatorSetCurveTypeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarCalls::getOperatorSetCurveType)
                    }
                    getOperatorSetCurveType
                },
                {
                    fn getOperatorFromSigningKey(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getOperatorFromSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarCalls::getOperatorFromSigningKey)
                    }
                    getOperatorFromSigningKey
                },
                {
                    fn deregisterKey(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <deregisterKeyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(KeyRegistrarCalls::deregisterKey)
                    }
                    deregisterKey
                },
                {
                    fn getBN254Key(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getBN254KeyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarCalls::getBN254Key)
                    }
                    getBN254Key
                },
                {
                    fn getECDSAKey(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getECDSAKeyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarCalls::getECDSAKey)
                    }
                    getECDSAKey
                },
                {
                    fn BN254_KEY_REGISTRATION_TYPEHASH(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <BN254_KEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarCalls::BN254_KEY_REGISTRATION_TYPEHASH)
                    }
                    BN254_KEY_REGISTRATION_TYPEHASH
                },
                {
                    fn isRegistered(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <isRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(KeyRegistrarCalls::isRegistered)
                    }
                    isRegistered
                },
                {
                    fn allocationManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn registerKey(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <registerKeyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarCalls::registerKey)
                    }
                    registerKey
                },
                {
                    fn getECDSAKeyRegistrationMessageHash(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getECDSAKeyRegistrationMessageHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarCalls::getECDSAKeyRegistrationMessageHash)
                    }
                    getECDSAKeyRegistrationMessageHash
                },
                {
                    fn isKeyGloballyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <isKeyGloballyRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarCalls::isKeyGloballyRegistered)
                    }
                    isKeyGloballyRegistered
                },
                {
                    fn configureOperatorSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <configureOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarCalls::configureOperatorSet)
                    }
                    configureOperatorSet
                },
                {
                    fn getKeyHash(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <getKeyHashCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarCalls::getKeyHash)
                    }
                    getKeyHash
                },
                {
                    fn domainSeparator(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarCalls> {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(KeyRegistrarCalls::domainSeparator)
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
                Self::BN254_KEY_REGISTRATION_TYPEHASH(inner) => {
                    <BN254_KEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ECDSA_KEY_REGISTRATION_TYPEHASH(inner) => {
                    <ECDSA_KEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
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
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::BN254_KEY_REGISTRATION_TYPEHASH(inner) => {
                    <BN254_KEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ECDSA_KEY_REGISTRATION_TYPEHASH(inner) => {
                    <ECDSA_KEY_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
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
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`KeyRegistrar`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum KeyRegistrarErrors {
        #[allow(missing_docs)]
        ConfigurationAlreadySet(ConfigurationAlreadySet),
        #[allow(missing_docs)]
        ECAddFailed(ECAddFailed),
        #[allow(missing_docs)]
        ECMulFailed(ECMulFailed),
        #[allow(missing_docs)]
        ECPairingFailed(ECPairingFailed),
        #[allow(missing_docs)]
        ExpModFailed(ExpModFailed),
        #[allow(missing_docs)]
        InvalidCurveType(InvalidCurveType),
        #[allow(missing_docs)]
        InvalidKeyFormat(InvalidKeyFormat),
        #[allow(missing_docs)]
        InvalidKeypair(InvalidKeypair),
        #[allow(missing_docs)]
        InvalidPermissions(InvalidPermissions),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        InvalidSignature(InvalidSignature),
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
        SignatureExpired(SignatureExpired),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
        #[allow(missing_docs)]
        ZeroAddress(ZeroAddress),
        #[allow(missing_docs)]
        ZeroPubkey(ZeroPubkey),
    }
    #[automatically_derived]
    impl KeyRegistrarErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 129u8, 240u8, 159u8],
            [8u8, 25u8, 189u8, 205u8],
            [16u8, 112u8, 40u8, 121u8],
            [24u8, 247u8, 132u8, 2u8],
            [27u8, 86u8, 166u8, 139u8],
            [46u8, 64u8, 225u8, 135u8],
            [48u8, 90u8, 39u8, 169u8],
            [66u8, 238u8, 104u8, 181u8],
            [70u8, 51u8, 190u8, 50u8],
            [73u8, 53u8, 80u8, 95u8],
            [139u8, 170u8, 87u8, 159u8],
            [147u8, 45u8, 148u8, 247u8],
            [147u8, 51u8, 30u8, 76u8],
            [179u8, 81u8, 43u8, 12u8],
            [185u8, 166u8, 32u8, 218u8],
            [209u8, 9u8, 17u8, 129u8],
            [212u8, 182u8, 143u8, 215u8],
            [213u8, 30u8, 218u8, 227u8],
            [217u8, 46u8, 35u8, 61u8],
            [253u8, 234u8, 124u8, 9u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for KeyRegistrarErrors {
        const NAME: &'static str = "KeyRegistrarErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ConfigurationAlreadySet(_) => {
                    <ConfigurationAlreadySet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECAddFailed(_) => <ECAddFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ECMulFailed(_) => <ECMulFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ECPairingFailed(_) => {
                    <ECPairingFailed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ExpModFailed(_) => <ExpModFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidCurveType(_) => {
                    <InvalidCurveType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidKeyFormat(_) => {
                    <InvalidKeyFormat as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidKeypair(_) => <InvalidKeypair as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidPermissions(_) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
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
                Self::SignatureExpired(_) => {
                    <SignatureExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringTooLong(_) => <StringTooLong as alloy_sol_types::SolError>::SELECTOR,
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
            static DECODE_SHIMS: &[fn(&[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors>] = &[
                {
                    fn ConfigurationAlreadySet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ConfigurationAlreadySet as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::ConfigurationAlreadySet)
                    }
                    ConfigurationAlreadySet
                },
                {
                    fn SignatureExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn OperatorStillSlashable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <OperatorStillSlashable as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::OperatorStillSlashable)
                    }
                    OperatorStillSlashable
                },
                {
                    fn KeyAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <KeyAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::KeyAlreadyRegistered)
                    }
                    KeyAlreadyRegistered
                },
                {
                    fn InvalidKeypair(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <InvalidKeypair as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::InvalidKeypair)
                    }
                    InvalidKeypair
                },
                {
                    fn KeyNotFound(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <KeyNotFound as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::KeyNotFound)
                    }
                    KeyNotFound
                },
                {
                    fn StringTooLong(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn OperatorAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(KeyRegistrarErrors::OperatorAlreadyRegistered)
                    }
                    OperatorAlreadyRegistered
                },
                {
                    fn ECMulFailed(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::ECMulFailed)
                    }
                    ECMulFailed
                },
                {
                    fn ZeroPubkey(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ZeroPubkey as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::ZeroPubkey)
                    }
                    ZeroPubkey
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn InvalidPermissions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <InvalidPermissions as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::InvalidPermissions)
                    }
                    InvalidPermissions
                },
                {
                    fn ECPairingFailed(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ECPairingFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::ECPairingFailed)
                    }
                    ECPairingFailed
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn OperatorSetNotConfigured(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <OperatorSetNotConfigured as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(KeyRegistrarErrors::OperatorSetNotConfigured)
                    }
                    OperatorSetNotConfigured
                },
                {
                    fn InvalidKeyFormat(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <InvalidKeyFormat as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::InvalidKeyFormat)
                    }
                    InvalidKeyFormat
                },
                {
                    fn ECAddFailed(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn ZeroAddress(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn InvalidCurveType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <InvalidCurveType as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(KeyRegistrarErrors::InvalidCurveType)
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
                -> alloy_sol_types::Result<KeyRegistrarErrors>] = &[
                {
                    fn ConfigurationAlreadySet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ConfigurationAlreadySet as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarErrors::ConfigurationAlreadySet)
                    }
                    ConfigurationAlreadySet
                },
                {
                    fn SignatureExpired(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(KeyRegistrarErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn OperatorStillSlashable(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <OperatorStillSlashable as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarErrors::OperatorStillSlashable)
                    }
                    OperatorStillSlashable
                },
                {
                    fn KeyAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <KeyAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarErrors::KeyAlreadyRegistered)
                    }
                    KeyAlreadyRegistered
                },
                {
                    fn InvalidKeypair(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <InvalidKeypair as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarErrors::InvalidKeypair)
                    }
                    InvalidKeypair
                },
                {
                    fn KeyNotFound(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <KeyNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarErrors::KeyNotFound)
                    }
                    KeyNotFound
                },
                {
                    fn StringTooLong(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn OperatorAlreadyRegistered(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarErrors::OperatorAlreadyRegistered)
                    }
                    OperatorAlreadyRegistered
                },
                {
                    fn ECMulFailed(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarErrors::ECMulFailed)
                    }
                    ECMulFailed
                },
                {
                    fn ZeroPubkey(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ZeroPubkey as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarErrors::ZeroPubkey)
                    }
                    ZeroPubkey
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(KeyRegistrarErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn InvalidPermissions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <InvalidPermissions as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(KeyRegistrarErrors::InvalidPermissions)
                    }
                    InvalidPermissions
                },
                {
                    fn ECPairingFailed(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ECPairingFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(KeyRegistrarErrors::ECPairingFailed)
                    }
                    ECPairingFailed
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(KeyRegistrarErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn OperatorSetNotConfigured(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <OperatorSetNotConfigured as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(KeyRegistrarErrors::OperatorSetNotConfigured)
                    }
                    OperatorSetNotConfigured
                },
                {
                    fn InvalidKeyFormat(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <InvalidKeyFormat as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(KeyRegistrarErrors::InvalidKeyFormat)
                    }
                    InvalidKeyFormat
                },
                {
                    fn ECAddFailed(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn ZeroAddress(data: &[u8]) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <ZeroAddress as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(KeyRegistrarErrors::ZeroAddress)
                    }
                    ZeroAddress
                },
                {
                    fn InvalidCurveType(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<KeyRegistrarErrors> {
                        <InvalidCurveType as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(KeyRegistrarErrors::InvalidCurveType)
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
                Self::InvalidCurveType(inner) => {
                    <InvalidCurveType as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidKeyFormat(inner) => {
                    <InvalidKeyFormat as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidKeypair(inner) => {
                    <InvalidKeypair as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidPermissions(inner) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InvalidCurveType(inner) => {
                    <InvalidCurveType as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidKeyFormat(inner) => {
                    <InvalidKeyFormat as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidKeypair(inner) => {
                    <InvalidKeypair as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidPermissions(inner) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
    ///Container for all the [`KeyRegistrar`](self) events.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum KeyRegistrarEvents {
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
    impl KeyRegistrarEvents {
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
    impl alloy_sol_types::SolEventInterface for KeyRegistrarEvents {
        const NAME: &'static str = "KeyRegistrarEvents";
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
    impl alloy_sol_types::private::IntoLogData for KeyRegistrarEvents {
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
    /**Creates a new wrapper around an on-chain [`KeyRegistrar`](self) contract instance.

    See the [wrapper's documentation](`KeyRegistrarInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> KeyRegistrarInstance<P, N> {
        KeyRegistrarInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
        provider: P,
        _permissionController: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _version: alloy::sol_types::private::String,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<KeyRegistrarInstance<P, N>>>
    {
        KeyRegistrarInstance::<P, N>::deploy(
            provider,
            _permissionController,
            _allocationManager,
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
        _permissionController: alloy::sol_types::private::Address,
        _allocationManager: alloy::sol_types::private::Address,
        _version: alloy::sol_types::private::String,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        KeyRegistrarInstance::<P, N>::deploy_builder(
            provider,
            _permissionController,
            _allocationManager,
            _version,
        )
    }
    /**A [`KeyRegistrar`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`KeyRegistrar`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct KeyRegistrarInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for KeyRegistrarInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("KeyRegistrarInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        KeyRegistrarInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`KeyRegistrar`](self) contract instance.

        See the [wrapper's documentation](`KeyRegistrarInstance`) for more details.*/
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
            _permissionController: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::Result<KeyRegistrarInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _permissionController,
                _allocationManager,
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
            _permissionController: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _permissionController,
                        _allocationManager,
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
    impl<P: ::core::clone::Clone, N> KeyRegistrarInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> KeyRegistrarInstance<P, N> {
            KeyRegistrarInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        KeyRegistrarInstance<P, N>
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
        ///Creates a new call builder for the [`BN254_KEY_REGISTRATION_TYPEHASH`] function.
        pub fn BN254_KEY_REGISTRATION_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, BN254_KEY_REGISTRATION_TYPEHASHCall, N> {
            self.call_builder(&BN254_KEY_REGISTRATION_TYPEHASHCall)
        }
        ///Creates a new call builder for the [`ECDSA_KEY_REGISTRATION_TYPEHASH`] function.
        pub fn ECDSA_KEY_REGISTRATION_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, ECDSA_KEY_REGISTRATION_TYPEHASHCall, N> {
            self.call_builder(&ECDSA_KEY_REGISTRATION_TYPEHASHCall)
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall)
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
        ///Creates a new call builder for the [`domainSeparator`] function.
        pub fn domainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, domainSeparatorCall, N> {
            self.call_builder(&domainSeparatorCall)
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
        ///Creates a new call builder for the [`permissionController`] function.
        pub fn permissionController(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, permissionControllerCall, N> {
            self.call_builder(&permissionControllerCall)
        }
        ///Creates a new call builder for the [`registerKey`] function.
        pub fn registerKey(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            keyData: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<&P, registerKeyCall, N> {
            self.call_builder(&registerKeyCall {
                operator,
                operatorSet,
                keyData,
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
        KeyRegistrarInstance<P, N>
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
