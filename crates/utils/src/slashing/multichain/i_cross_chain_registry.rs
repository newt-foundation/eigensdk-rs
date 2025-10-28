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
/**

Generated by the following Solidity interface...
```solidity
library ICrossChainRegistryTypes {
    struct OperatorSetConfig {
        address owner;
        uint32 maxStalenessPeriod;
    }
}

interface ICrossChainRegistry {
    struct OperatorSet {
        address avs;
        uint32 id;
    }

    error ArrayLengthMismatch();
    error ChainIDAlreadyWhitelisted();
    error ChainIDNotWhitelisted();
    error EmptyChainIDsArray();
    error GenerationReservationAlreadyExists();
    error GenerationReservationDoesNotExist();
    error InvalidChainId();
    error InvalidEndIndex();
    error InvalidOperatorSet();
    error InvalidRange();
    error InvalidStalenessPeriod();
    error InvalidTableUpdateCadence();
    error KeyTypeNotSet();

    event ChainIDAddedToWhitelist(uint256 chainID, address operatorTableUpdater);
    event ChainIDRemovedFromWhitelist(uint256 chainID);
    event GenerationReservationCreated(OperatorSet operatorSet);
    event GenerationReservationRemoved(OperatorSet operatorSet);
    event OperatorSetConfigRemoved(OperatorSet operatorSet);
    event OperatorSetConfigSet(OperatorSet operatorSet, ICrossChainRegistryTypes.OperatorSetConfig config);
    event OperatorTableCalculatorRemoved(OperatorSet operatorSet);
    event OperatorTableCalculatorSet(OperatorSet operatorSet, address operatorTableCalculator);
    event TableUpdateCadenceSet(uint32 tableUpdateCadence);

    function addChainIDsToWhitelist(uint256[] memory chainIDs, address[] memory operatorTableUpdaters) external;
    function calculateOperatorTableBytes(OperatorSet memory operatorSet) external view returns (bytes memory);
    function createGenerationReservation(OperatorSet memory operatorSet, address operatorTableCalculator, ICrossChainRegistryTypes.OperatorSetConfig memory config) external;
    function getActiveGenerationReservationCount() external view returns (uint256);
    function getActiveGenerationReservations() external view returns (OperatorSet[] memory);
    function getActiveGenerationReservationsByRange(uint256 startIndex, uint256 endIndex) external view returns (OperatorSet[] memory);
    function getOperatorSetConfig(OperatorSet memory operatorSet) external view returns (ICrossChainRegistryTypes.OperatorSetConfig memory);
    function getOperatorTableCalculator(OperatorSet memory operatorSet) external view returns (address);
    function getSupportedChains() external view returns (uint256[] memory, address[] memory);
    function getTableUpdateCadence() external view returns (uint32);
    function hasActiveGenerationReservation(OperatorSet memory operatorSet) external view returns (bool);
    function removeChainIDsFromWhitelist(uint256[] memory chainIDs) external;
    function removeGenerationReservation(OperatorSet memory operatorSet) external;
    function setOperatorSetConfig(OperatorSet memory operatorSet, ICrossChainRegistryTypes.OperatorSetConfig memory config) external;
    function setOperatorTableCalculator(OperatorSet memory operatorSet, address operatorTableCalculator) external;
    function setTableUpdateCadence(uint32 tableUpdateCadence) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "addChainIDsToWhitelist",
    "inputs": [
      {
        "name": "chainIDs",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "operatorTableUpdaters",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "calculateOperatorTableBytes",
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
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "createGenerationReservation",
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
        "name": "operatorTableCalculator",
        "type": "address",
        "internalType": "contract IOperatorTableCalculator"
      },
      {
        "name": "config",
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
    "name": "getActiveGenerationReservationCount",
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
    "name": "getActiveGenerationReservations",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct OperatorSet[]",
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
    "name": "getActiveGenerationReservationsByRange",
    "inputs": [
      {
        "name": "startIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "endIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct OperatorSet[]",
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
    "name": "getOperatorSetConfig",
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
    "name": "getOperatorTableCalculator",
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
        "internalType": "contract IOperatorTableCalculator"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getSupportedChains",
    "inputs": [],
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
    "name": "getTableUpdateCadence",
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
    "name": "hasActiveGenerationReservation",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "removeChainIDsFromWhitelist",
    "inputs": [
      {
        "name": "chainIDs",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeGenerationReservation",
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
    "name": "setOperatorSetConfig",
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
        "name": "config",
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
    "name": "setOperatorTableCalculator",
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
        "name": "operatorTableCalculator",
        "type": "address",
        "internalType": "contract IOperatorTableCalculator"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setTableUpdateCadence",
    "inputs": [
      {
        "name": "tableUpdateCadence",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "ChainIDAddedToWhitelist",
    "inputs": [
      {
        "name": "chainID",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "operatorTableUpdater",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ChainIDRemovedFromWhitelist",
    "inputs": [
      {
        "name": "chainID",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "GenerationReservationCreated",
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
    "name": "GenerationReservationRemoved",
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
    "name": "OperatorSetConfigRemoved",
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
    "name": "OperatorSetConfigSet",
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
        "name": "config",
        "type": "tuple",
        "indexed": false,
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
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorTableCalculatorRemoved",
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
    "name": "OperatorTableCalculatorSet",
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
        "name": "operatorTableCalculator",
        "type": "address",
        "indexed": false,
        "internalType": "contract IOperatorTableCalculator"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TableUpdateCadenceSet",
    "inputs": [
      {
        "name": "tableUpdateCadence",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
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
    "name": "ChainIDAlreadyWhitelisted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ChainIDNotWhitelisted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EmptyChainIDsArray",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GenerationReservationAlreadyExists",
    "inputs": []
  },
  {
    "type": "error",
    "name": "GenerationReservationDoesNotExist",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidChainId",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidEndIndex",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidOperatorSet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidRange",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidStalenessPeriod",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTableUpdateCadence",
    "inputs": []
  },
  {
    "type": "error",
    "name": "KeyTypeNotSet",
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
pub mod ICrossChainRegistry {
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
    /**Custom error with signature `ChainIDAlreadyWhitelisted()` and selector `0x497ec636`.
    ```solidity
    error ChainIDAlreadyWhitelisted();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChainIDAlreadyWhitelisted;
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
        impl ::core::convert::From<ChainIDAlreadyWhitelisted> for UnderlyingRustTuple<'_> {
            fn from(value: ChainIDAlreadyWhitelisted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChainIDAlreadyWhitelisted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ChainIDAlreadyWhitelisted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ChainIDAlreadyWhitelisted()";
            const SELECTOR: [u8; 4] = [73u8, 126u8, 198u8, 54u8];
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
    /**Custom error with signature `ChainIDNotWhitelisted()` and selector `0xb3f92ba1`.
    ```solidity
    error ChainIDNotWhitelisted();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ChainIDNotWhitelisted;
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
        impl ::core::convert::From<ChainIDNotWhitelisted> for UnderlyingRustTuple<'_> {
            fn from(value: ChainIDNotWhitelisted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ChainIDNotWhitelisted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ChainIDNotWhitelisted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ChainIDNotWhitelisted()";
            const SELECTOR: [u8; 4] = [179u8, 249u8, 43u8, 161u8];
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
    /**Custom error with signature `EmptyChainIDsArray()` and selector `0x8631a075`.
    ```solidity
    error EmptyChainIDsArray();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EmptyChainIDsArray;
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
        impl ::core::convert::From<EmptyChainIDsArray> for UnderlyingRustTuple<'_> {
            fn from(value: EmptyChainIDsArray) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EmptyChainIDsArray {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EmptyChainIDsArray {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EmptyChainIDsArray()";
            const SELECTOR: [u8; 4] = [134u8, 49u8, 160u8, 117u8];
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
    /**Custom error with signature `GenerationReservationAlreadyExists()` and selector `0x18834615`.
    ```solidity
    error GenerationReservationAlreadyExists();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GenerationReservationAlreadyExists;
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
        impl ::core::convert::From<GenerationReservationAlreadyExists> for UnderlyingRustTuple<'_> {
            fn from(value: GenerationReservationAlreadyExists) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GenerationReservationAlreadyExists {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GenerationReservationAlreadyExists {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GenerationReservationAlreadyExists()";
            const SELECTOR: [u8; 4] = [24u8, 131u8, 70u8, 21u8];
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
    /**Custom error with signature `GenerationReservationDoesNotExist()` and selector `0x9a575d52`.
    ```solidity
    error GenerationReservationDoesNotExist();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GenerationReservationDoesNotExist;
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
        impl ::core::convert::From<GenerationReservationDoesNotExist> for UnderlyingRustTuple<'_> {
            fn from(value: GenerationReservationDoesNotExist) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GenerationReservationDoesNotExist {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GenerationReservationDoesNotExist {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GenerationReservationDoesNotExist()";
            const SELECTOR: [u8; 4] = [154u8, 87u8, 93u8, 82u8];
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
    /**Custom error with signature `InvalidChainId()` and selector `0x7a47c9a2`.
    ```solidity
    error InvalidChainId();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidChainId;
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
        impl ::core::convert::From<InvalidChainId> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidChainId) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidChainId {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidChainId {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidChainId()";
            const SELECTOR: [u8; 4] = [122u8, 71u8, 201u8, 162u8];
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
    /**Custom error with signature `InvalidEndIndex()` and selector `0xb68d84c0`.
    ```solidity
    error InvalidEndIndex();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidEndIndex;
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
        impl ::core::convert::From<InvalidEndIndex> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidEndIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidEndIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidEndIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidEndIndex()";
            const SELECTOR: [u8; 4] = [182u8, 141u8, 132u8, 192u8];
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
    /**Custom error with signature `InvalidRange()` and selector `0x561ce9bb`.
    ```solidity
    error InvalidRange();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRange;
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
        impl ::core::convert::From<InvalidRange> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRange) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRange {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRange {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRange()";
            const SELECTOR: [u8; 4] = [86u8, 28u8, 233u8, 187u8];
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
    /**Custom error with signature `InvalidStalenessPeriod()` and selector `0x5c8c9062`.
    ```solidity
    error InvalidStalenessPeriod();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidStalenessPeriod;
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
        impl ::core::convert::From<InvalidStalenessPeriod> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidStalenessPeriod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidStalenessPeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidStalenessPeriod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidStalenessPeriod()";
            const SELECTOR: [u8; 4] = [92u8, 140u8, 144u8, 98u8];
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
    /**Custom error with signature `InvalidTableUpdateCadence()` and selector `0xb6cc70d8`.
    ```solidity
    error InvalidTableUpdateCadence();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTableUpdateCadence;
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
        impl ::core::convert::From<InvalidTableUpdateCadence> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTableUpdateCadence) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTableUpdateCadence {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTableUpdateCadence {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTableUpdateCadence()";
            const SELECTOR: [u8; 4] = [182u8, 204u8, 112u8, 216u8];
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
    /**Custom error with signature `KeyTypeNotSet()` and selector `0xe57cacbd`.
    ```solidity
    error KeyTypeNotSet();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct KeyTypeNotSet;
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
        impl ::core::convert::From<KeyTypeNotSet> for UnderlyingRustTuple<'_> {
            fn from(value: KeyTypeNotSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for KeyTypeNotSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for KeyTypeNotSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "KeyTypeNotSet()";
            const SELECTOR: [u8; 4] = [229u8, 124u8, 172u8, 189u8];
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
    /**Event with signature `ChainIDAddedToWhitelist(uint256,address)` and selector `0x7a0a76d85b582b17996dd7371a407aa7a79b870db8539247fba315c7b6beff62`.
    ```solidity
    event ChainIDAddedToWhitelist(uint256 chainID, address operatorTableUpdater);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChainIDAddedToWhitelist {
        #[allow(missing_docs)]
        pub chainID: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub operatorTableUpdater: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ChainIDAddedToWhitelist {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ChainIDAddedToWhitelist(uint256,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    122u8, 10u8, 118u8, 216u8, 91u8, 88u8, 43u8, 23u8, 153u8, 109u8, 215u8, 55u8,
                    26u8, 64u8, 122u8, 167u8, 167u8, 155u8, 135u8, 13u8, 184u8, 83u8, 146u8, 71u8,
                    251u8, 163u8, 21u8, 199u8, 182u8, 190u8, 255u8, 98u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    chainID: data.0,
                    operatorTableUpdater: data.1,
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
                        &self.chainID,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operatorTableUpdater,
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
        impl alloy_sol_types::private::IntoLogData for ChainIDAddedToWhitelist {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChainIDAddedToWhitelist> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChainIDAddedToWhitelist) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `ChainIDRemovedFromWhitelist(uint256)` and selector `0x6824d36084ecf2cd819b137cb5d837cc6e73afce1e0e348c9fdecaa81d0341e5`.
    ```solidity
    event ChainIDRemovedFromWhitelist(uint256 chainID);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ChainIDRemovedFromWhitelist {
        #[allow(missing_docs)]
        pub chainID: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for ChainIDRemovedFromWhitelist {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ChainIDRemovedFromWhitelist(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    104u8, 36u8, 211u8, 96u8, 132u8, 236u8, 242u8, 205u8, 129u8, 155u8, 19u8,
                    124u8, 181u8, 216u8, 55u8, 204u8, 110u8, 115u8, 175u8, 206u8, 30u8, 14u8, 52u8,
                    140u8, 159u8, 222u8, 202u8, 168u8, 29u8, 3u8, 65u8, 229u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { chainID: data.0 }
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
                        &self.chainID,
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
        impl alloy_sol_types::private::IntoLogData for ChainIDRemovedFromWhitelist {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ChainIDRemovedFromWhitelist> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ChainIDRemovedFromWhitelist) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `GenerationReservationCreated((address,uint32))` and selector `0x4fb6efec7dd60036ce3a7af8d5c48425019daa0fb61eb471a966a7ac2c6fa6a6`.
    ```solidity
    event GenerationReservationCreated(OperatorSet operatorSet);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct GenerationReservationCreated {
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
        impl alloy_sol_types::SolEvent for GenerationReservationCreated {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "GenerationReservationCreated((address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    79u8, 182u8, 239u8, 236u8, 125u8, 214u8, 0u8, 54u8, 206u8, 58u8, 122u8, 248u8,
                    213u8, 196u8, 132u8, 37u8, 1u8, 157u8, 170u8, 15u8, 182u8, 30u8, 180u8, 113u8,
                    169u8, 102u8, 167u8, 172u8, 44u8, 111u8, 166u8, 166u8,
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
        impl alloy_sol_types::private::IntoLogData for GenerationReservationCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&GenerationReservationCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &GenerationReservationCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `GenerationReservationRemoved((address,uint32))` and selector `0x4ffdfdd59e9e1e3c301608788f78dd458e61cb8c045ca92b62a7b484c80824fb`.
    ```solidity
    event GenerationReservationRemoved(OperatorSet operatorSet);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct GenerationReservationRemoved {
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
        impl alloy_sol_types::SolEvent for GenerationReservationRemoved {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "GenerationReservationRemoved((address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    79u8, 253u8, 253u8, 213u8, 158u8, 158u8, 30u8, 60u8, 48u8, 22u8, 8u8, 120u8,
                    143u8, 120u8, 221u8, 69u8, 142u8, 97u8, 203u8, 140u8, 4u8, 92u8, 169u8, 43u8,
                    98u8, 167u8, 180u8, 132u8, 200u8, 8u8, 36u8, 251u8,
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
        impl alloy_sol_types::private::IntoLogData for GenerationReservationRemoved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&GenerationReservationRemoved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &GenerationReservationRemoved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OperatorSetConfigRemoved((address,uint32))` and selector `0x210a1118a869246162804e2a7f21ef808ebd93f4be7ed512014fe29a7a8be02e`.
    ```solidity
    event OperatorSetConfigRemoved(OperatorSet operatorSet);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSetConfigRemoved {
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
        impl alloy_sol_types::SolEvent for OperatorSetConfigRemoved {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorSetConfigRemoved((address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    33u8, 10u8, 17u8, 24u8, 168u8, 105u8, 36u8, 97u8, 98u8, 128u8, 78u8, 42u8,
                    127u8, 33u8, 239u8, 128u8, 142u8, 189u8, 147u8, 244u8, 190u8, 126u8, 213u8,
                    18u8, 1u8, 79u8, 226u8, 154u8, 122u8, 139u8, 224u8, 46u8,
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
        impl alloy_sol_types::private::IntoLogData for OperatorSetConfigRemoved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSetConfigRemoved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSetConfigRemoved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OperatorSetConfigSet((address,uint32),(address,uint32))` and selector `0x3147846ee526009000671c20380b856a633345691300f82585f90034715cf0e2`.
    ```solidity
    event OperatorSetConfigSet(OperatorSet operatorSet, ICrossChainRegistryTypes.OperatorSetConfig config);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSetConfigSet {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub config:
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorSetConfigSet {
            type DataTuple<'a> = (OperatorSet, ICrossChainRegistryTypes::OperatorSetConfig);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "OperatorSetConfigSet((address,uint32),(address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 71u8, 132u8, 110u8, 229u8, 38u8, 0u8, 144u8, 0u8, 103u8, 28u8, 32u8,
                    56u8, 11u8, 133u8, 106u8, 99u8, 51u8, 69u8, 105u8, 19u8, 0u8, 248u8, 37u8,
                    133u8, 249u8, 0u8, 52u8, 113u8, 92u8, 240u8, 226u8,
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
                    config: data.1,
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
                    <ICrossChainRegistryTypes::OperatorSetConfig as alloy_sol_types::SolType>::tokenize(
                        &self.config,
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
        impl alloy_sol_types::private::IntoLogData for OperatorSetConfigSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSetConfigSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSetConfigSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OperatorTableCalculatorRemoved((address,uint32))` and selector `0xd7811913efd5d98fc7ea0d1fdd022b3d31987815360842d05b1d1cf55578d16a`.
    ```solidity
    event OperatorTableCalculatorRemoved(OperatorSet operatorSet);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorTableCalculatorRemoved {
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
        impl alloy_sol_types::SolEvent for OperatorTableCalculatorRemoved {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorTableCalculatorRemoved((address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    215u8, 129u8, 25u8, 19u8, 239u8, 213u8, 217u8, 143u8, 199u8, 234u8, 13u8, 31u8,
                    221u8, 2u8, 43u8, 61u8, 49u8, 152u8, 120u8, 21u8, 54u8, 8u8, 66u8, 208u8, 91u8,
                    29u8, 28u8, 245u8, 85u8, 120u8, 209u8, 106u8,
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
        impl alloy_sol_types::private::IntoLogData for OperatorTableCalculatorRemoved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorTableCalculatorRemoved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorTableCalculatorRemoved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `OperatorTableCalculatorSet((address,uint32),address)` and selector `0x7f7ccafd92d20fdb39dee184a0dce002a9da420ed0def461f2a027abc9b3f6df`.
    ```solidity
    event OperatorTableCalculatorSet(OperatorSet operatorSet, address operatorTableCalculator);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorTableCalculatorSet {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operatorTableCalculator: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OperatorTableCalculatorSet {
            type DataTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorTableCalculatorSet((address,uint32),address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    127u8, 124u8, 202u8, 253u8, 146u8, 210u8, 15u8, 219u8, 57u8, 222u8, 225u8,
                    132u8, 160u8, 220u8, 224u8, 2u8, 169u8, 218u8, 66u8, 14u8, 208u8, 222u8, 244u8,
                    97u8, 242u8, 160u8, 39u8, 171u8, 201u8, 179u8, 246u8, 223u8,
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
                    operatorTableCalculator: data.1,
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
                        &self.operatorTableCalculator,
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
        impl alloy_sol_types::private::IntoLogData for OperatorTableCalculatorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorTableCalculatorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorTableCalculatorSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TableUpdateCadenceSet(uint32)` and selector `0x4fbcd0cca70015b33db8af4aa4f2bd6fd6c1efa9460b8e2333f252c1467a6327`.
    ```solidity
    event TableUpdateCadenceSet(uint32 tableUpdateCadence);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TableUpdateCadenceSet {
        #[allow(missing_docs)]
        pub tableUpdateCadence: u32,
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
        impl alloy_sol_types::SolEvent for TableUpdateCadenceSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TableUpdateCadenceSet(uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    79u8, 188u8, 208u8, 204u8, 167u8, 0u8, 21u8, 179u8, 61u8, 184u8, 175u8, 74u8,
                    164u8, 242u8, 189u8, 111u8, 214u8, 193u8, 239u8, 169u8, 70u8, 11u8, 142u8,
                    35u8, 51u8, 242u8, 82u8, 193u8, 70u8, 122u8, 99u8, 39u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    tableUpdateCadence: data.0,
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.tableUpdateCadence,
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
        impl alloy_sol_types::private::IntoLogData for TableUpdateCadenceSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TableUpdateCadenceSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TableUpdateCadenceSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `addChainIDsToWhitelist(uint256[],address[])` and selector `0x04e98be3`.
    ```solidity
    function addChainIDsToWhitelist(uint256[] memory chainIDs, address[] memory operatorTableUpdaters) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addChainIDsToWhitelistCall {
        #[allow(missing_docs)]
        pub chainIDs:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        #[allow(missing_docs)]
        pub operatorTableUpdaters:
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`addChainIDsToWhitelist(uint256[],address[])`](addChainIDsToWhitelistCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addChainIDsToWhitelistReturn {}
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
            impl ::core::convert::From<addChainIDsToWhitelistCall> for UnderlyingRustTuple<'_> {
                fn from(value: addChainIDsToWhitelistCall) -> Self {
                    (value.chainIDs, value.operatorTableUpdaters)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addChainIDsToWhitelistCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        chainIDs: tuple.0,
                        operatorTableUpdaters: tuple.1,
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
            impl ::core::convert::From<addChainIDsToWhitelistReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addChainIDsToWhitelistReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addChainIDsToWhitelistReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addChainIDsToWhitelistReturn {
            fn _tokenize(
                &self,
            ) -> <addChainIDsToWhitelistCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addChainIDsToWhitelistCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addChainIDsToWhitelistReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addChainIDsToWhitelist(uint256[],address[])";
            const SELECTOR: [u8; 4] = [4u8, 233u8, 139u8, 227u8];
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
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainIDs),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorTableUpdaters),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addChainIDsToWhitelistReturn::_tokenize(ret)
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
    /**Function with signature `calculateOperatorTableBytes((address,uint32))` and selector `0x41ee6d0e`.
    ```solidity
    function calculateOperatorTableBytes(OperatorSet memory operatorSet) external view returns (bytes memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorTableBytesCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`calculateOperatorTableBytes((address,uint32))`](calculateOperatorTableBytesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorTableBytesReturn {
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
            impl ::core::convert::From<calculateOperatorTableBytesCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateOperatorTableBytesCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateOperatorTableBytesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
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
            impl ::core::convert::From<calculateOperatorTableBytesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calculateOperatorTableBytesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateOperatorTableBytesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateOperatorTableBytesCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Bytes;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateOperatorTableBytes((address,uint32))";
            const SELECTOR: [u8; 4] = [65u8, 238u8, 109u8, 14u8];
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
                (<alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: calculateOperatorTableBytesReturn = r.into();
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
                    let r: calculateOperatorTableBytesReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `createGenerationReservation((address,uint32),address,(address,uint32))` and selector `0xd5044911`.
    ```solidity
    function createGenerationReservation(OperatorSet memory operatorSet, address operatorTableCalculator, ICrossChainRegistryTypes.OperatorSetConfig memory config) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createGenerationReservationCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operatorTableCalculator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub config:
            <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`createGenerationReservation((address,uint32),address,(address,uint32))`](createGenerationReservationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createGenerationReservationReturn {}
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
                alloy::sol_types::sol_data::Address,
                ICrossChainRegistryTypes::OperatorSetConfig,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<createGenerationReservationCall> for UnderlyingRustTuple<'_> {
                fn from(value: createGenerationReservationCall) -> Self {
                    (
                        value.operatorSet,
                        value.operatorTableCalculator,
                        value.config,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createGenerationReservationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        operatorTableCalculator: tuple.1,
                        config: tuple.2,
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
            impl ::core::convert::From<createGenerationReservationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createGenerationReservationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createGenerationReservationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl createGenerationReservationReturn {
            fn _tokenize(
                &self,
            ) -> <createGenerationReservationCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createGenerationReservationCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Address,
                ICrossChainRegistryTypes::OperatorSetConfig,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createGenerationReservationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "createGenerationReservation((address,uint32),address,(address,uint32))";
            const SELECTOR: [u8; 4] = [213u8, 4u8, 73u8, 17u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operatorTableCalculator,
                    ),
                    <ICrossChainRegistryTypes::OperatorSetConfig as alloy_sol_types::SolType>::tokenize(
                        &self.config,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                createGenerationReservationReturn::_tokenize(ret)
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
    /**Function with signature `getActiveGenerationReservationCount()` and selector `0xb186a60e`.
    ```solidity
    function getActiveGenerationReservationCount() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveGenerationReservationCountCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getActiveGenerationReservationCount()`](getActiveGenerationReservationCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveGenerationReservationCountReturn {
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
            impl ::core::convert::From<getActiveGenerationReservationCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveGenerationReservationCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveGenerationReservationCountCall {
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
            impl ::core::convert::From<getActiveGenerationReservationCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveGenerationReservationCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveGenerationReservationCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActiveGenerationReservationCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::primitives::aliases::U256;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getActiveGenerationReservationCount()";
            const SELECTOR: [u8; 4] = [177u8, 134u8, 166u8, 14u8];
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
                        let r: getActiveGenerationReservationCountReturn = r.into();
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
                    let r: getActiveGenerationReservationCountReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getActiveGenerationReservations()` and selector `0xd09b978b`.
    ```solidity
    function getActiveGenerationReservations() external view returns (OperatorSet[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveGenerationReservationsCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getActiveGenerationReservations()`](getActiveGenerationReservationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveGenerationReservationsReturn {
        #[allow(missing_docs)]
        pub _0:
            alloy::sol_types::private::Vec<<OperatorSet as alloy::sol_types::SolType>::RustType>,
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
            impl ::core::convert::From<getActiveGenerationReservationsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveGenerationReservationsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveGenerationReservationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getActiveGenerationReservationsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveGenerationReservationsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveGenerationReservationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActiveGenerationReservationsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <OperatorSet as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getActiveGenerationReservations()";
            const SELECTOR: [u8; 4] = [208u8, 155u8, 151u8, 139u8];
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
                    <alloy::sol_types::sol_data::Array<
                        OperatorSet,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getActiveGenerationReservationsReturn = r.into();
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
                    let r: getActiveGenerationReservationsReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getActiveGenerationReservationsByRange(uint256,uint256)` and selector `0xd9a6729e`.
    ```solidity
    function getActiveGenerationReservationsByRange(uint256 startIndex, uint256 endIndex) external view returns (OperatorSet[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveGenerationReservationsByRangeCall {
        #[allow(missing_docs)]
        pub startIndex: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub endIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getActiveGenerationReservationsByRange(uint256,uint256)`](getActiveGenerationReservationsByRangeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveGenerationReservationsByRangeReturn {
        #[allow(missing_docs)]
        pub _0:
            alloy::sol_types::private::Vec<<OperatorSet as alloy::sol_types::SolType>::RustType>,
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
            impl ::core::convert::From<getActiveGenerationReservationsByRangeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveGenerationReservationsByRangeCall) -> Self {
                    (value.startIndex, value.endIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveGenerationReservationsByRangeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        startIndex: tuple.0,
                        endIndex: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getActiveGenerationReservationsByRangeReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: getActiveGenerationReservationsByRangeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for getActiveGenerationReservationsByRangeReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActiveGenerationReservationsByRangeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                <OperatorSet as alloy::sol_types::SolType>::RustType,
            >;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getActiveGenerationReservationsByRange(uint256,uint256)";
            const SELECTOR: [u8; 4] = [217u8, 166u8, 114u8, 158u8];
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
                        &self.startIndex,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.endIndex,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        OperatorSet,
                    > as alloy_sol_types::SolType>::tokenize(ret),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getActiveGenerationReservationsByRangeReturn = r.into();
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
                    let r: getActiveGenerationReservationsByRangeReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperatorSetConfig((address,uint32))` and selector `0x21fa7fdc`.
    ```solidity
    function getOperatorSetConfig(OperatorSet memory operatorSet) external view returns (ICrossChainRegistryTypes.OperatorSetConfig memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetConfigCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorSetConfig((address,uint32))`](getOperatorSetConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetConfigReturn {
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
            impl ::core::convert::From<getOperatorSetConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetConfigCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                    }
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
            impl ::core::convert::From<getOperatorSetConfigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetConfigReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSetConfigCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType;
            type ReturnTuple<'a> = (ICrossChainRegistryTypes::OperatorSetConfig,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSetConfig((address,uint32))";
            const SELECTOR: [u8; 4] = [33u8, 250u8, 127u8, 220u8];
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
                    <ICrossChainRegistryTypes::OperatorSetConfig as alloy_sol_types::SolType>::tokenize(
                        ret,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getOperatorSetConfigReturn = r.into();
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
                    let r: getOperatorSetConfigReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getOperatorTableCalculator((address,uint32))` and selector `0x75e4b539`.
    ```solidity
    function getOperatorTableCalculator(OperatorSet memory operatorSet) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorTableCalculatorCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getOperatorTableCalculator((address,uint32))`](getOperatorTableCalculatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorTableCalculatorReturn {
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
            impl ::core::convert::From<getOperatorTableCalculatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorTableCalculatorCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorTableCalculatorCall {
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
            impl ::core::convert::From<getOperatorTableCalculatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorTableCalculatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorTableCalculatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorTableCalculatorCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorTableCalculator((address,uint32))";
            const SELECTOR: [u8; 4] = [117u8, 228u8, 181u8, 57u8];
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
                        let r: getOperatorTableCalculatorReturn = r.into();
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
                    let r: getOperatorTableCalculatorReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `getSupportedChains()` and selector `0xc4bffe2b`.
    ```solidity
    function getSupportedChains() external view returns (uint256[] memory, address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSupportedChainsCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getSupportedChains()`](getSupportedChainsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSupportedChainsReturn {
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
            impl ::core::convert::From<getSupportedChainsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSupportedChainsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSupportedChainsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
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
            impl ::core::convert::From<getSupportedChainsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getSupportedChainsReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSupportedChainsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        impl getSupportedChainsReturn {
            fn _tokenize(
                &self,
            ) -> <getSupportedChainsCall as alloy_sol_types::SolCall>::ReturnToken<'_> {
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
        impl alloy_sol_types::SolCall for getSupportedChainsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSupportedChainsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSupportedChains()";
            const SELECTOR: [u8; 4] = [196u8, 191u8, 254u8, 43u8];
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
                getSupportedChainsReturn::_tokenize(ret)
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
    /**Function with signature `getTableUpdateCadence()` and selector `0xac505f4b`.
    ```solidity
    function getTableUpdateCadence() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTableUpdateCadenceCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getTableUpdateCadence()`](getTableUpdateCadenceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTableUpdateCadenceReturn {
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
            impl ::core::convert::From<getTableUpdateCadenceCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTableUpdateCadenceCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTableUpdateCadenceCall {
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
            impl ::core::convert::From<getTableUpdateCadenceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTableUpdateCadenceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTableUpdateCadenceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTableUpdateCadenceCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = u32;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTableUpdateCadence()";
            const SELECTOR: [u8; 4] = [172u8, 80u8, 95u8, 75u8];
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
                        let r: getTableUpdateCadenceReturn = r.into();
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
                    let r: getTableUpdateCadenceReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `hasActiveGenerationReservation((address,uint32))` and selector `0x36b200de`.
    ```solidity
    function hasActiveGenerationReservation(OperatorSet memory operatorSet) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasActiveGenerationReservationCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`hasActiveGenerationReservation((address,uint32))`](hasActiveGenerationReservationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasActiveGenerationReservationReturn {
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
            impl ::core::convert::From<hasActiveGenerationReservationCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasActiveGenerationReservationCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasActiveGenerationReservationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
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
            impl ::core::convert::From<hasActiveGenerationReservationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasActiveGenerationReservationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasActiveGenerationReservationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasActiveGenerationReservationCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = bool;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasActiveGenerationReservation((address,uint32))";
            const SELECTOR: [u8; 4] = [54u8, 178u8, 0u8, 222u8];
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
                (<alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: hasActiveGenerationReservationReturn = r.into();
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
                    let r: hasActiveGenerationReservationReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `removeChainIDsFromWhitelist(uint256[])` and selector `0xdfbd9dfd`.
    ```solidity
    function removeChainIDsFromWhitelist(uint256[] memory chainIDs) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeChainIDsFromWhitelistCall {
        #[allow(missing_docs)]
        pub chainIDs:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
    }
    ///Container type for the return parameters of the [`removeChainIDsFromWhitelist(uint256[])`](removeChainIDsFromWhitelistCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeChainIDsFromWhitelistReturn {}
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
            impl ::core::convert::From<removeChainIDsFromWhitelistCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeChainIDsFromWhitelistCall) -> Self {
                    (value.chainIDs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeChainIDsFromWhitelistCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { chainIDs: tuple.0 }
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
            impl ::core::convert::From<removeChainIDsFromWhitelistReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeChainIDsFromWhitelistReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeChainIDsFromWhitelistReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl removeChainIDsFromWhitelistReturn {
            fn _tokenize(
                &self,
            ) -> <removeChainIDsFromWhitelistCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeChainIDsFromWhitelistCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeChainIDsFromWhitelistReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeChainIDsFromWhitelist(uint256[])";
            const SELECTOR: [u8; 4] = [223u8, 189u8, 157u8, 253u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.chainIDs
                ),)
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                removeChainIDsFromWhitelistReturn::_tokenize(ret)
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
    /**Function with signature `removeGenerationReservation((address,uint32))` and selector `0x6c55a37f`.
    ```solidity
    function removeGenerationReservation(OperatorSet memory operatorSet) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeGenerationReservationCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`removeGenerationReservation((address,uint32))`](removeGenerationReservationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeGenerationReservationReturn {}
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
            impl ::core::convert::From<removeGenerationReservationCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeGenerationReservationCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeGenerationReservationCall {
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
            impl ::core::convert::From<removeGenerationReservationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeGenerationReservationReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeGenerationReservationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl removeGenerationReservationReturn {
            fn _tokenize(
                &self,
            ) -> <removeGenerationReservationCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeGenerationReservationCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeGenerationReservationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeGenerationReservation((address,uint32))";
            const SELECTOR: [u8; 4] = [108u8, 85u8, 163u8, 127u8];
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
                removeGenerationReservationReturn::_tokenize(ret)
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
    /**Function with signature `setOperatorSetConfig((address,uint32),(address,uint32))` and selector `0x277e1e62`.
    ```solidity
    function setOperatorSetConfig(OperatorSet memory operatorSet, ICrossChainRegistryTypes.OperatorSetConfig memory config) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorSetConfigCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub config:
            <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setOperatorSetConfig((address,uint32),(address,uint32))`](setOperatorSetConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorSetConfigReturn {}
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
                (OperatorSet, ICrossChainRegistryTypes::OperatorSetConfig);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<setOperatorSetConfigCall> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorSetConfigCall) -> Self {
                    (value.operatorSet, value.config)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorSetConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        config: tuple.1,
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
            impl ::core::convert::From<setOperatorSetConfigReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorSetConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorSetConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setOperatorSetConfigReturn {
            fn _tokenize(
                &self,
            ) -> <setOperatorSetConfigCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOperatorSetConfigCall {
            type Parameters<'a> = (OperatorSet, ICrossChainRegistryTypes::OperatorSetConfig);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorSetConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "setOperatorSetConfig((address,uint32),(address,uint32))";
            const SELECTOR: [u8; 4] = [39u8, 126u8, 30u8, 98u8];
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
                    <ICrossChainRegistryTypes::OperatorSetConfig as alloy_sol_types::SolType>::tokenize(
                        &self.config,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setOperatorSetConfigReturn::_tokenize(ret)
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
    /**Function with signature `setOperatorTableCalculator((address,uint32),address)` and selector `0x1ca9142a`.
    ```solidity
    function setOperatorTableCalculator(OperatorSet memory operatorSet, address operatorTableCalculator) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorTableCalculatorCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operatorTableCalculator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setOperatorTableCalculator((address,uint32),address)`](setOperatorTableCalculatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorTableCalculatorReturn {}
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
            impl ::core::convert::From<setOperatorTableCalculatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorTableCalculatorCall) -> Self {
                    (value.operatorSet, value.operatorTableCalculator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorTableCalculatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        operatorTableCalculator: tuple.1,
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
            impl ::core::convert::From<setOperatorTableCalculatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorTableCalculatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorTableCalculatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setOperatorTableCalculatorReturn {
            fn _tokenize(
                &self,
            ) -> <setOperatorTableCalculatorCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOperatorTableCalculatorCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorTableCalculatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setOperatorTableCalculator((address,uint32),address)";
            const SELECTOR: [u8; 4] = [28u8, 169u8, 20u8, 42u8];
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
                        &self.operatorTableCalculator,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setOperatorTableCalculatorReturn::_tokenize(ret)
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
    /**Function with signature `setTableUpdateCadence(uint32)` and selector `0xd6db9e25`.
    ```solidity
    function setTableUpdateCadence(uint32 tableUpdateCadence) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setTableUpdateCadenceCall {
        #[allow(missing_docs)]
        pub tableUpdateCadence: u32,
    }
    ///Container type for the return parameters of the [`setTableUpdateCadence(uint32)`](setTableUpdateCadenceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setTableUpdateCadenceReturn {}
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
            impl ::core::convert::From<setTableUpdateCadenceCall> for UnderlyingRustTuple<'_> {
                fn from(value: setTableUpdateCadenceCall) -> Self {
                    (value.tableUpdateCadence,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setTableUpdateCadenceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tableUpdateCadence: tuple.0,
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
            impl ::core::convert::From<setTableUpdateCadenceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setTableUpdateCadenceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setTableUpdateCadenceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl setTableUpdateCadenceReturn {
            fn _tokenize(
                &self,
            ) -> <setTableUpdateCadenceCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setTableUpdateCadenceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setTableUpdateCadenceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setTableUpdateCadence(uint32)";
            const SELECTOR: [u8; 4] = [214u8, 219u8, 158u8, 37u8];
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
                        &self.tableUpdateCadence,
                    ),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                setTableUpdateCadenceReturn::_tokenize(ret)
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
    ///Container for all the [`ICrossChainRegistry`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum ICrossChainRegistryCalls {
        #[allow(missing_docs)]
        addChainIDsToWhitelist(addChainIDsToWhitelistCall),
        #[allow(missing_docs)]
        calculateOperatorTableBytes(calculateOperatorTableBytesCall),
        #[allow(missing_docs)]
        createGenerationReservation(createGenerationReservationCall),
        #[allow(missing_docs)]
        getActiveGenerationReservationCount(getActiveGenerationReservationCountCall),
        #[allow(missing_docs)]
        getActiveGenerationReservations(getActiveGenerationReservationsCall),
        #[allow(missing_docs)]
        getActiveGenerationReservationsByRange(getActiveGenerationReservationsByRangeCall),
        #[allow(missing_docs)]
        getOperatorSetConfig(getOperatorSetConfigCall),
        #[allow(missing_docs)]
        getOperatorTableCalculator(getOperatorTableCalculatorCall),
        #[allow(missing_docs)]
        getSupportedChains(getSupportedChainsCall),
        #[allow(missing_docs)]
        getTableUpdateCadence(getTableUpdateCadenceCall),
        #[allow(missing_docs)]
        hasActiveGenerationReservation(hasActiveGenerationReservationCall),
        #[allow(missing_docs)]
        removeChainIDsFromWhitelist(removeChainIDsFromWhitelistCall),
        #[allow(missing_docs)]
        removeGenerationReservation(removeGenerationReservationCall),
        #[allow(missing_docs)]
        setOperatorSetConfig(setOperatorSetConfigCall),
        #[allow(missing_docs)]
        setOperatorTableCalculator(setOperatorTableCalculatorCall),
        #[allow(missing_docs)]
        setTableUpdateCadence(setTableUpdateCadenceCall),
    }
    #[automatically_derived]
    impl ICrossChainRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 233u8, 139u8, 227u8],
            [28u8, 169u8, 20u8, 42u8],
            [33u8, 250u8, 127u8, 220u8],
            [39u8, 126u8, 30u8, 98u8],
            [54u8, 178u8, 0u8, 222u8],
            [65u8, 238u8, 109u8, 14u8],
            [108u8, 85u8, 163u8, 127u8],
            [117u8, 228u8, 181u8, 57u8],
            [172u8, 80u8, 95u8, 75u8],
            [177u8, 134u8, 166u8, 14u8],
            [196u8, 191u8, 254u8, 43u8],
            [208u8, 155u8, 151u8, 139u8],
            [213u8, 4u8, 73u8, 17u8],
            [214u8, 219u8, 158u8, 37u8],
            [217u8, 166u8, 114u8, 158u8],
            [223u8, 189u8, 157u8, 253u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ICrossChainRegistryCalls {
        const NAME: &'static str = "ICrossChainRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 16usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addChainIDsToWhitelist(_) => {
                    <addChainIDsToWhitelistCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateOperatorTableBytes(_) => {
                    <calculateOperatorTableBytesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createGenerationReservation(_) => {
                    <createGenerationReservationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActiveGenerationReservationCount(_) => {
                    <getActiveGenerationReservationCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActiveGenerationReservations(_) => {
                    <getActiveGenerationReservationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActiveGenerationReservationsByRange(_) => {
                    <getActiveGenerationReservationsByRangeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSetConfig(_) => {
                    <getOperatorSetConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorTableCalculator(_) => {
                    <getOperatorTableCalculatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSupportedChains(_) => {
                    <getSupportedChainsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTableUpdateCadence(_) => {
                    <getTableUpdateCadenceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasActiveGenerationReservation(_) => {
                    <hasActiveGenerationReservationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeChainIDsFromWhitelist(_) => {
                    <removeChainIDsFromWhitelistCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeGenerationReservation(_) => {
                    <removeGenerationReservationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperatorSetConfig(_) => {
                    <setOperatorSetConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperatorTableCalculator(_) => {
                    <setOperatorTableCalculatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setTableUpdateCadence(_) => {
                    <setTableUpdateCadenceCall as alloy_sol_types::SolCall>::SELECTOR
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
                -> alloy_sol_types::Result<ICrossChainRegistryCalls>] = &[
                {
                    fn addChainIDsToWhitelist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <addChainIDsToWhitelistCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(ICrossChainRegistryCalls::addChainIDsToWhitelist)
                    }
                    addChainIDsToWhitelist
                },
                {
                    fn setOperatorTableCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <setOperatorTableCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::setOperatorTableCalculator)
                    }
                    setOperatorTableCalculator
                },
                {
                    fn getOperatorSetConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getOperatorSetConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ICrossChainRegistryCalls::getOperatorSetConfig)
                    }
                    getOperatorSetConfig
                },
                {
                    fn setOperatorSetConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <setOperatorSetConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ICrossChainRegistryCalls::setOperatorSetConfig)
                    }
                    setOperatorSetConfig
                },
                {
                    fn hasActiveGenerationReservation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <hasActiveGenerationReservationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ICrossChainRegistryCalls::hasActiveGenerationReservation,
                            )
                    }
                    hasActiveGenerationReservation
                },
                {
                    fn calculateOperatorTableBytes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <calculateOperatorTableBytesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::calculateOperatorTableBytes)
                    }
                    calculateOperatorTableBytes
                },
                {
                    fn removeGenerationReservation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <removeGenerationReservationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::removeGenerationReservation)
                    }
                    removeGenerationReservation
                },
                {
                    fn getOperatorTableCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getOperatorTableCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::getOperatorTableCalculator)
                    }
                    getOperatorTableCalculator
                },
                {
                    fn getTableUpdateCadence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(ICrossChainRegistryCalls::getTableUpdateCadence)
                    }
                    getTableUpdateCadence
                },
                {
                    fn getActiveGenerationReservationCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getActiveGenerationReservationCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ICrossChainRegistryCalls::getActiveGenerationReservationCount,
                            )
                    }
                    getActiveGenerationReservationCount
                },
                {
                    fn getSupportedChains(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getSupportedChainsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(ICrossChainRegistryCalls::getSupportedChains)
                    }
                    getSupportedChains
                },
                {
                    fn getActiveGenerationReservations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getActiveGenerationReservationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ICrossChainRegistryCalls::getActiveGenerationReservations,
                            )
                    }
                    getActiveGenerationReservations
                },
                {
                    fn createGenerationReservation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <createGenerationReservationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::createGenerationReservation)
                    }
                    createGenerationReservation
                },
                {
                    fn setTableUpdateCadence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <setTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(ICrossChainRegistryCalls::setTableUpdateCadence)
                    }
                    setTableUpdateCadence
                },
                {
                    fn getActiveGenerationReservationsByRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getActiveGenerationReservationsByRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ICrossChainRegistryCalls::getActiveGenerationReservationsByRange,
                            )
                    }
                    getActiveGenerationReservationsByRange
                },
                {
                    fn removeChainIDsFromWhitelist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <removeChainIDsFromWhitelistCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::removeChainIDsFromWhitelist)
                    }
                    removeChainIDsFromWhitelist
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
                ICrossChainRegistryCalls,
            >] = &[
                {
                    fn addChainIDsToWhitelist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <addChainIDsToWhitelistCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::addChainIDsToWhitelist)
                    }
                    addChainIDsToWhitelist
                },
                {
                    fn setOperatorTableCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <setOperatorTableCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::setOperatorTableCalculator)
                    }
                    setOperatorTableCalculator
                },
                {
                    fn getOperatorSetConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getOperatorSetConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::getOperatorSetConfig)
                    }
                    getOperatorSetConfig
                },
                {
                    fn setOperatorSetConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <setOperatorSetConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::setOperatorSetConfig)
                    }
                    setOperatorSetConfig
                },
                {
                    fn hasActiveGenerationReservation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <hasActiveGenerationReservationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ICrossChainRegistryCalls::hasActiveGenerationReservation,
                            )
                    }
                    hasActiveGenerationReservation
                },
                {
                    fn calculateOperatorTableBytes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <calculateOperatorTableBytesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::calculateOperatorTableBytes)
                    }
                    calculateOperatorTableBytes
                },
                {
                    fn removeGenerationReservation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <removeGenerationReservationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::removeGenerationReservation)
                    }
                    removeGenerationReservation
                },
                {
                    fn getOperatorTableCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getOperatorTableCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::getOperatorTableCalculator)
                    }
                    getOperatorTableCalculator
                },
                {
                    fn getTableUpdateCadence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::getTableUpdateCadence)
                    }
                    getTableUpdateCadence
                },
                {
                    fn getActiveGenerationReservationCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getActiveGenerationReservationCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ICrossChainRegistryCalls::getActiveGenerationReservationCount,
                            )
                    }
                    getActiveGenerationReservationCount
                },
                {
                    fn getSupportedChains(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getSupportedChainsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::getSupportedChains)
                    }
                    getSupportedChains
                },
                {
                    fn getActiveGenerationReservations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getActiveGenerationReservationsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ICrossChainRegistryCalls::getActiveGenerationReservations,
                            )
                    }
                    getActiveGenerationReservations
                },
                {
                    fn createGenerationReservation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <createGenerationReservationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::createGenerationReservation)
                    }
                    createGenerationReservation
                },
                {
                    fn setTableUpdateCadence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <setTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::setTableUpdateCadence)
                    }
                    setTableUpdateCadence
                },
                {
                    fn getActiveGenerationReservationsByRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <getActiveGenerationReservationsByRangeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ICrossChainRegistryCalls::getActiveGenerationReservationsByRange,
                            )
                    }
                    getActiveGenerationReservationsByRange
                },
                {
                    fn removeChainIDsFromWhitelist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryCalls> {
                        <removeChainIDsFromWhitelistCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryCalls::removeChainIDsFromWhitelist)
                    }
                    removeChainIDsFromWhitelist
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
                Self::addChainIDsToWhitelist(inner) => {
                    <addChainIDsToWhitelistCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateOperatorTableBytes(inner) => {
                    <calculateOperatorTableBytesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createGenerationReservation(inner) => {
                    <createGenerationReservationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getActiveGenerationReservationCount(inner) => {
                    <getActiveGenerationReservationCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getActiveGenerationReservations(inner) => {
                    <getActiveGenerationReservationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getActiveGenerationReservationsByRange(inner) => {
                    <getActiveGenerationReservationsByRangeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSetConfig(inner) => {
                    <getOperatorSetConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorTableCalculator(inner) => {
                    <getOperatorTableCalculatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSupportedChains(inner) => {
                    <getSupportedChainsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTableUpdateCadence(inner) => {
                    <getTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::hasActiveGenerationReservation(inner) => {
                    <hasActiveGenerationReservationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeChainIDsFromWhitelist(inner) => {
                    <removeChainIDsFromWhitelistCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeGenerationReservation(inner) => {
                    <removeGenerationReservationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setOperatorSetConfig(inner) => {
                    <setOperatorSetConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setOperatorTableCalculator(inner) => {
                    <setOperatorTableCalculatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setTableUpdateCadence(inner) => {
                    <setTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::addChainIDsToWhitelist(inner) => {
                    <addChainIDsToWhitelistCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateOperatorTableBytes(inner) => {
                    <calculateOperatorTableBytesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createGenerationReservation(inner) => {
                    <createGenerationReservationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getActiveGenerationReservationCount(inner) => {
                    <getActiveGenerationReservationCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getActiveGenerationReservations(inner) => {
                    <getActiveGenerationReservationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getActiveGenerationReservationsByRange(inner) => {
                    <getActiveGenerationReservationsByRangeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorSetConfig(inner) => {
                    <getOperatorSetConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorTableCalculator(inner) => {
                    <getOperatorTableCalculatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSupportedChains(inner) => {
                    <getSupportedChainsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTableUpdateCadence(inner) => {
                    <getTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasActiveGenerationReservation(inner) => {
                    <hasActiveGenerationReservationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeChainIDsFromWhitelist(inner) => {
                    <removeChainIDsFromWhitelistCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeGenerationReservation(inner) => {
                    <removeGenerationReservationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setOperatorSetConfig(inner) => {
                    <setOperatorSetConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setOperatorTableCalculator(inner) => {
                    <setOperatorTableCalculatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setTableUpdateCadence(inner) => {
                    <setTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ICrossChainRegistry`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum ICrossChainRegistryErrors {
        #[allow(missing_docs)]
        ArrayLengthMismatch(ArrayLengthMismatch),
        #[allow(missing_docs)]
        ChainIDAlreadyWhitelisted(ChainIDAlreadyWhitelisted),
        #[allow(missing_docs)]
        ChainIDNotWhitelisted(ChainIDNotWhitelisted),
        #[allow(missing_docs)]
        EmptyChainIDsArray(EmptyChainIDsArray),
        #[allow(missing_docs)]
        GenerationReservationAlreadyExists(GenerationReservationAlreadyExists),
        #[allow(missing_docs)]
        GenerationReservationDoesNotExist(GenerationReservationDoesNotExist),
        #[allow(missing_docs)]
        InvalidChainId(InvalidChainId),
        #[allow(missing_docs)]
        InvalidEndIndex(InvalidEndIndex),
        #[allow(missing_docs)]
        InvalidOperatorSet(InvalidOperatorSet),
        #[allow(missing_docs)]
        InvalidRange(InvalidRange),
        #[allow(missing_docs)]
        InvalidStalenessPeriod(InvalidStalenessPeriod),
        #[allow(missing_docs)]
        InvalidTableUpdateCadence(InvalidTableUpdateCadence),
        #[allow(missing_docs)]
        KeyTypeNotSet(KeyTypeNotSet),
    }
    #[automatically_derived]
    impl ICrossChainRegistryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [24u8, 131u8, 70u8, 21u8],
            [73u8, 126u8, 198u8, 54u8],
            [86u8, 28u8, 233u8, 187u8],
            [92u8, 140u8, 144u8, 98u8],
            [122u8, 71u8, 201u8, 162u8],
            [126u8, 197u8, 193u8, 84u8],
            [134u8, 49u8, 160u8, 117u8],
            [154u8, 87u8, 93u8, 82u8],
            [162u8, 74u8, 19u8, 166u8],
            [179u8, 249u8, 43u8, 161u8],
            [182u8, 141u8, 132u8, 192u8],
            [182u8, 204u8, 112u8, 216u8],
            [229u8, 124u8, 172u8, 189u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ICrossChainRegistryErrors {
        const NAME: &'static str = "ICrossChainRegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ArrayLengthMismatch(_) => {
                    <ArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ChainIDAlreadyWhitelisted(_) => {
                    <ChainIDAlreadyWhitelisted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ChainIDNotWhitelisted(_) => {
                    <ChainIDNotWhitelisted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EmptyChainIDsArray(_) => {
                    <EmptyChainIDsArray as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GenerationReservationAlreadyExists(_) => {
                    <GenerationReservationAlreadyExists as alloy_sol_types::SolError>::SELECTOR
                }
                Self::GenerationReservationDoesNotExist(_) => {
                    <GenerationReservationDoesNotExist as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidChainId(_) => <InvalidChainId as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidEndIndex(_) => {
                    <InvalidEndIndex as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOperatorSet(_) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRange(_) => <InvalidRange as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidStalenessPeriod(_) => {
                    <InvalidStalenessPeriod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTableUpdateCadence(_) => {
                    <InvalidTableUpdateCadence as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeyTypeNotSet(_) => <KeyTypeNotSet as alloy_sol_types::SolError>::SELECTOR,
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
                -> alloy_sol_types::Result<ICrossChainRegistryErrors>] = &[
                {
                    fn GenerationReservationAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <GenerationReservationAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ICrossChainRegistryErrors::GenerationReservationAlreadyExists,
                            )
                    }
                    GenerationReservationAlreadyExists
                },
                {
                    fn ChainIDAlreadyWhitelisted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <ChainIDAlreadyWhitelisted as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(ICrossChainRegistryErrors::ChainIDAlreadyWhitelisted)
                    }
                    ChainIDAlreadyWhitelisted
                },
                {
                    fn InvalidRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <InvalidRange as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ICrossChainRegistryErrors::InvalidRange)
                    }
                    InvalidRange
                },
                {
                    fn InvalidStalenessPeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <InvalidStalenessPeriod as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ICrossChainRegistryErrors::InvalidStalenessPeriod)
                    }
                    InvalidStalenessPeriod
                },
                {
                    fn InvalidChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <InvalidChainId as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ICrossChainRegistryErrors::InvalidChainId)
                    }
                    InvalidChainId
                },
                {
                    fn InvalidOperatorSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <InvalidOperatorSet as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ICrossChainRegistryErrors::InvalidOperatorSet)
                    }
                    InvalidOperatorSet
                },
                {
                    fn EmptyChainIDsArray(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <EmptyChainIDsArray as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ICrossChainRegistryErrors::EmptyChainIDsArray)
                    }
                    EmptyChainIDsArray
                },
                {
                    fn GenerationReservationDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <GenerationReservationDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                ICrossChainRegistryErrors::GenerationReservationDoesNotExist,
                            )
                    }
                    GenerationReservationDoesNotExist
                },
                {
                    fn ArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ICrossChainRegistryErrors::ArrayLengthMismatch)
                    }
                    ArrayLengthMismatch
                },
                {
                    fn ChainIDNotWhitelisted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <ChainIDNotWhitelisted as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ICrossChainRegistryErrors::ChainIDNotWhitelisted)
                    }
                    ChainIDNotWhitelisted
                },
                {
                    fn InvalidEndIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <InvalidEndIndex as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ICrossChainRegistryErrors::InvalidEndIndex)
                    }
                    InvalidEndIndex
                },
                {
                    fn InvalidTableUpdateCadence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <InvalidTableUpdateCadence as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(ICrossChainRegistryErrors::InvalidTableUpdateCadence)
                    }
                    InvalidTableUpdateCadence
                },
                {
                    fn KeyTypeNotSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <KeyTypeNotSet as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(ICrossChainRegistryErrors::KeyTypeNotSet)
                    }
                    KeyTypeNotSet
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
                ICrossChainRegistryErrors,
            >] = &[
                {
                    fn GenerationReservationAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <GenerationReservationAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ICrossChainRegistryErrors::GenerationReservationAlreadyExists,
                            )
                    }
                    GenerationReservationAlreadyExists
                },
                {
                    fn ChainIDAlreadyWhitelisted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <ChainIDAlreadyWhitelisted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryErrors::ChainIDAlreadyWhitelisted)
                    }
                    ChainIDAlreadyWhitelisted
                },
                {
                    fn InvalidRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <InvalidRange as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(ICrossChainRegistryErrors::InvalidRange)
                    }
                    InvalidRange
                },
                {
                    fn InvalidStalenessPeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <InvalidStalenessPeriod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryErrors::InvalidStalenessPeriod)
                    }
                    InvalidStalenessPeriod
                },
                {
                    fn InvalidChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <InvalidChainId as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(ICrossChainRegistryErrors::InvalidChainId)
                    }
                    InvalidChainId
                },
                {
                    fn InvalidOperatorSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <InvalidOperatorSet as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ICrossChainRegistryErrors::InvalidOperatorSet)
                    }
                    InvalidOperatorSet
                },
                {
                    fn EmptyChainIDsArray(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <EmptyChainIDsArray as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ICrossChainRegistryErrors::EmptyChainIDsArray)
                    }
                    EmptyChainIDsArray
                },
                {
                    fn GenerationReservationDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <GenerationReservationDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                ICrossChainRegistryErrors::GenerationReservationDoesNotExist,
                            )
                    }
                    GenerationReservationDoesNotExist
                },
                {
                    fn ArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ICrossChainRegistryErrors::ArrayLengthMismatch)
                    }
                    ArrayLengthMismatch
                },
                {
                    fn ChainIDNotWhitelisted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <ChainIDNotWhitelisted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryErrors::ChainIDNotWhitelisted)
                    }
                    ChainIDNotWhitelisted
                },
                {
                    fn InvalidEndIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <InvalidEndIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(ICrossChainRegistryErrors::InvalidEndIndex)
                    }
                    InvalidEndIndex
                },
                {
                    fn InvalidTableUpdateCadence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <InvalidTableUpdateCadence as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(ICrossChainRegistryErrors::InvalidTableUpdateCadence)
                    }
                    InvalidTableUpdateCadence
                },
                {
                    fn KeyTypeNotSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<ICrossChainRegistryErrors> {
                        <KeyTypeNotSet as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(ICrossChainRegistryErrors::KeyTypeNotSet)
                    }
                    KeyTypeNotSet
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
                    <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ChainIDAlreadyWhitelisted(inner) => {
                    <ChainIDAlreadyWhitelisted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ChainIDNotWhitelisted(inner) => {
                    <ChainIDNotWhitelisted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EmptyChainIDsArray(inner) => {
                    <EmptyChainIDsArray as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GenerationReservationAlreadyExists(inner) => {
                    <GenerationReservationAlreadyExists as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GenerationReservationDoesNotExist(inner) => {
                    <GenerationReservationDoesNotExist as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidChainId(inner) => {
                    <InvalidChainId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidEndIndex(inner) => {
                    <InvalidEndIndex as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidOperatorSet(inner) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidRange(inner) => {
                    <InvalidRange as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidStalenessPeriod(inner) => {
                    <InvalidStalenessPeriod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTableUpdateCadence(inner) => {
                    <InvalidTableUpdateCadence as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::KeyTypeNotSet(inner) => {
                    <KeyTypeNotSet as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::ArrayLengthMismatch(inner) => {
                    <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ChainIDAlreadyWhitelisted(inner) => {
                    <ChainIDAlreadyWhitelisted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ChainIDNotWhitelisted(inner) => {
                    <ChainIDNotWhitelisted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EmptyChainIDsArray(inner) => {
                    <EmptyChainIDsArray as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GenerationReservationAlreadyExists(inner) => {
                    <GenerationReservationAlreadyExists as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GenerationReservationDoesNotExist(inner) => {
                    <GenerationReservationDoesNotExist as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidChainId(inner) => {
                    <InvalidChainId as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidEndIndex(inner) => {
                    <InvalidEndIndex as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidOperatorSet(inner) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidRange(inner) => {
                    <InvalidRange as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidStalenessPeriod(inner) => {
                    <InvalidStalenessPeriod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTableUpdateCadence(inner) => {
                    <InvalidTableUpdateCadence as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::KeyTypeNotSet(inner) => {
                    <KeyTypeNotSet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ICrossChainRegistry`](self) events.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum ICrossChainRegistryEvents {
        #[allow(missing_docs)]
        ChainIDAddedToWhitelist(ChainIDAddedToWhitelist),
        #[allow(missing_docs)]
        ChainIDRemovedFromWhitelist(ChainIDRemovedFromWhitelist),
        #[allow(missing_docs)]
        GenerationReservationCreated(GenerationReservationCreated),
        #[allow(missing_docs)]
        GenerationReservationRemoved(GenerationReservationRemoved),
        #[allow(missing_docs)]
        OperatorSetConfigRemoved(OperatorSetConfigRemoved),
        #[allow(missing_docs)]
        OperatorSetConfigSet(OperatorSetConfigSet),
        #[allow(missing_docs)]
        OperatorTableCalculatorRemoved(OperatorTableCalculatorRemoved),
        #[allow(missing_docs)]
        OperatorTableCalculatorSet(OperatorTableCalculatorSet),
        #[allow(missing_docs)]
        TableUpdateCadenceSet(TableUpdateCadenceSet),
    }
    #[automatically_derived]
    impl ICrossChainRegistryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                33u8, 10u8, 17u8, 24u8, 168u8, 105u8, 36u8, 97u8, 98u8, 128u8, 78u8, 42u8, 127u8,
                33u8, 239u8, 128u8, 142u8, 189u8, 147u8, 244u8, 190u8, 126u8, 213u8, 18u8, 1u8,
                79u8, 226u8, 154u8, 122u8, 139u8, 224u8, 46u8,
            ],
            [
                49u8, 71u8, 132u8, 110u8, 229u8, 38u8, 0u8, 144u8, 0u8, 103u8, 28u8, 32u8, 56u8,
                11u8, 133u8, 106u8, 99u8, 51u8, 69u8, 105u8, 19u8, 0u8, 248u8, 37u8, 133u8, 249u8,
                0u8, 52u8, 113u8, 92u8, 240u8, 226u8,
            ],
            [
                79u8, 182u8, 239u8, 236u8, 125u8, 214u8, 0u8, 54u8, 206u8, 58u8, 122u8, 248u8,
                213u8, 196u8, 132u8, 37u8, 1u8, 157u8, 170u8, 15u8, 182u8, 30u8, 180u8, 113u8,
                169u8, 102u8, 167u8, 172u8, 44u8, 111u8, 166u8, 166u8,
            ],
            [
                79u8, 188u8, 208u8, 204u8, 167u8, 0u8, 21u8, 179u8, 61u8, 184u8, 175u8, 74u8,
                164u8, 242u8, 189u8, 111u8, 214u8, 193u8, 239u8, 169u8, 70u8, 11u8, 142u8, 35u8,
                51u8, 242u8, 82u8, 193u8, 70u8, 122u8, 99u8, 39u8,
            ],
            [
                79u8, 253u8, 253u8, 213u8, 158u8, 158u8, 30u8, 60u8, 48u8, 22u8, 8u8, 120u8, 143u8,
                120u8, 221u8, 69u8, 142u8, 97u8, 203u8, 140u8, 4u8, 92u8, 169u8, 43u8, 98u8, 167u8,
                180u8, 132u8, 200u8, 8u8, 36u8, 251u8,
            ],
            [
                104u8, 36u8, 211u8, 96u8, 132u8, 236u8, 242u8, 205u8, 129u8, 155u8, 19u8, 124u8,
                181u8, 216u8, 55u8, 204u8, 110u8, 115u8, 175u8, 206u8, 30u8, 14u8, 52u8, 140u8,
                159u8, 222u8, 202u8, 168u8, 29u8, 3u8, 65u8, 229u8,
            ],
            [
                122u8, 10u8, 118u8, 216u8, 91u8, 88u8, 43u8, 23u8, 153u8, 109u8, 215u8, 55u8, 26u8,
                64u8, 122u8, 167u8, 167u8, 155u8, 135u8, 13u8, 184u8, 83u8, 146u8, 71u8, 251u8,
                163u8, 21u8, 199u8, 182u8, 190u8, 255u8, 98u8,
            ],
            [
                127u8, 124u8, 202u8, 253u8, 146u8, 210u8, 15u8, 219u8, 57u8, 222u8, 225u8, 132u8,
                160u8, 220u8, 224u8, 2u8, 169u8, 218u8, 66u8, 14u8, 208u8, 222u8, 244u8, 97u8,
                242u8, 160u8, 39u8, 171u8, 201u8, 179u8, 246u8, 223u8,
            ],
            [
                215u8, 129u8, 25u8, 19u8, 239u8, 213u8, 217u8, 143u8, 199u8, 234u8, 13u8, 31u8,
                221u8, 2u8, 43u8, 61u8, 49u8, 152u8, 120u8, 21u8, 54u8, 8u8, 66u8, 208u8, 91u8,
                29u8, 28u8, 245u8, 85u8, 120u8, 209u8, 106u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ICrossChainRegistryEvents {
        const NAME: &'static str = "ICrossChainRegistryEvents";
        const COUNT: usize = 9usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<ChainIDAddedToWhitelist as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ChainIDAddedToWhitelist as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::ChainIDAddedToWhitelist)
                }
                Some(
                    <ChainIDRemovedFromWhitelist as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <ChainIDRemovedFromWhitelist as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data,
                )
                .map(Self::ChainIDRemovedFromWhitelist),
                Some(
                    <GenerationReservationCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <GenerationReservationCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data,
                )
                .map(Self::GenerationReservationCreated),
                Some(
                    <GenerationReservationRemoved as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <GenerationReservationRemoved as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data,
                )
                .map(Self::GenerationReservationRemoved),
                Some(<OperatorSetConfigRemoved as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSetConfigRemoved as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::OperatorSetConfigRemoved)
                }
                Some(<OperatorSetConfigSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSetConfigSet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::OperatorSetConfigSet)
                }
                Some(
                    <OperatorTableCalculatorRemoved as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <OperatorTableCalculatorRemoved as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data,
                )
                .map(Self::OperatorTableCalculatorRemoved),
                Some(<OperatorTableCalculatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorTableCalculatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::OperatorTableCalculatorSet)
                }
                Some(<TableUpdateCadenceSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TableUpdateCadenceSet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::TableUpdateCadenceSet)
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
    impl alloy_sol_types::private::IntoLogData for ICrossChainRegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChainIDAddedToWhitelist(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ChainIDRemovedFromWhitelist(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::GenerationReservationCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::GenerationReservationRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetConfigRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetConfigSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorTableCalculatorRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorTableCalculatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TableUpdateCadenceSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::ChainIDAddedToWhitelist(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ChainIDRemovedFromWhitelist(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::GenerationReservationCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::GenerationReservationRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetConfigRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetConfigSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorTableCalculatorRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorTableCalculatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TableUpdateCadenceSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ICrossChainRegistry`](self) contract instance.

    See the [wrapper's documentation](`ICrossChainRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ICrossChainRegistryInstance<P, N> {
        ICrossChainRegistryInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
        provider: P,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<ICrossChainRegistryInstance<P, N>>>
    {
        ICrossChainRegistryInstance::<P, N>::deploy(provider)
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
        ICrossChainRegistryInstance::<P, N>::deploy_builder(provider)
    }
    /**A [`ICrossChainRegistry`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ICrossChainRegistry`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ICrossChainRegistryInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for ICrossChainRegistryInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ICrossChainRegistryInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        ICrossChainRegistryInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`ICrossChainRegistry`](self) contract instance.

        See the [wrapper's documentation](`ICrossChainRegistryInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ICrossChainRegistryInstance<P, N>> {
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
    impl<P: ::core::clone::Clone, N> ICrossChainRegistryInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ICrossChainRegistryInstance<P, N> {
            ICrossChainRegistryInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        ICrossChainRegistryInstance<P, N>
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
        ///Creates a new call builder for the [`addChainIDsToWhitelist`] function.
        pub fn addChainIDsToWhitelist(
            &self,
            chainIDs: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            operatorTableUpdaters: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, addChainIDsToWhitelistCall, N> {
            self.call_builder(&addChainIDsToWhitelistCall {
                chainIDs,
                operatorTableUpdaters,
            })
        }
        ///Creates a new call builder for the [`calculateOperatorTableBytes`] function.
        pub fn calculateOperatorTableBytes(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, calculateOperatorTableBytesCall, N> {
            self.call_builder(&calculateOperatorTableBytesCall { operatorSet })
        }
        ///Creates a new call builder for the [`createGenerationReservation`] function.
        pub fn createGenerationReservation(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operatorTableCalculator: alloy::sol_types::private::Address,
            config: <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, createGenerationReservationCall, N> {
            self.call_builder(&createGenerationReservationCall {
                operatorSet,
                operatorTableCalculator,
                config,
            })
        }
        ///Creates a new call builder for the [`getActiveGenerationReservationCount`] function.
        pub fn getActiveGenerationReservationCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getActiveGenerationReservationCountCall, N>
        {
            self.call_builder(&getActiveGenerationReservationCountCall)
        }
        ///Creates a new call builder for the [`getActiveGenerationReservations`] function.
        pub fn getActiveGenerationReservations(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getActiveGenerationReservationsCall, N> {
            self.call_builder(&getActiveGenerationReservationsCall)
        }
        ///Creates a new call builder for the [`getActiveGenerationReservationsByRange`] function.
        pub fn getActiveGenerationReservationsByRange(
            &self,
            startIndex: alloy::sol_types::private::primitives::aliases::U256,
            endIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, getActiveGenerationReservationsByRangeCall, N>
        {
            self.call_builder(&getActiveGenerationReservationsByRangeCall {
                startIndex,
                endIndex,
            })
        }
        ///Creates a new call builder for the [`getOperatorSetConfig`] function.
        pub fn getOperatorSetConfig(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorSetConfigCall, N> {
            self.call_builder(&getOperatorSetConfigCall { operatorSet })
        }
        ///Creates a new call builder for the [`getOperatorTableCalculator`] function.
        pub fn getOperatorTableCalculator(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, getOperatorTableCalculatorCall, N> {
            self.call_builder(&getOperatorTableCalculatorCall { operatorSet })
        }
        ///Creates a new call builder for the [`getSupportedChains`] function.
        pub fn getSupportedChains(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getSupportedChainsCall, N> {
            self.call_builder(&getSupportedChainsCall)
        }
        ///Creates a new call builder for the [`getTableUpdateCadence`] function.
        pub fn getTableUpdateCadence(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getTableUpdateCadenceCall, N> {
            self.call_builder(&getTableUpdateCadenceCall)
        }
        ///Creates a new call builder for the [`hasActiveGenerationReservation`] function.
        pub fn hasActiveGenerationReservation(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, hasActiveGenerationReservationCall, N> {
            self.call_builder(&hasActiveGenerationReservationCall { operatorSet })
        }
        ///Creates a new call builder for the [`removeChainIDsFromWhitelist`] function.
        pub fn removeChainIDsFromWhitelist(
            &self,
            chainIDs: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, removeChainIDsFromWhitelistCall, N> {
            self.call_builder(&removeChainIDsFromWhitelistCall { chainIDs })
        }
        ///Creates a new call builder for the [`removeGenerationReservation`] function.
        pub fn removeGenerationReservation(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, removeGenerationReservationCall, N> {
            self.call_builder(&removeGenerationReservationCall { operatorSet })
        }
        ///Creates a new call builder for the [`setOperatorSetConfig`] function.
        pub fn setOperatorSetConfig(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            config: <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, setOperatorSetConfigCall, N> {
            self.call_builder(&setOperatorSetConfigCall {
                operatorSet,
                config,
            })
        }
        ///Creates a new call builder for the [`setOperatorTableCalculator`] function.
        pub fn setOperatorTableCalculator(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operatorTableCalculator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<&P, setOperatorTableCalculatorCall, N> {
            self.call_builder(&setOperatorTableCalculatorCall {
                operatorSet,
                operatorTableCalculator,
            })
        }
        ///Creates a new call builder for the [`setTableUpdateCadence`] function.
        pub fn setTableUpdateCadence(
            &self,
            tableUpdateCadence: u32,
        ) -> alloy_contract::SolCallBuilder<&P, setTableUpdateCadenceCall, N> {
            self.call_builder(&setTableUpdateCadenceCall { tableUpdateCadence })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        ICrossChainRegistryInstance<P, N>
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
        ///Creates a new event filter for the [`ChainIDAddedToWhitelist`] event.
        pub fn ChainIDAddedToWhitelist_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChainIDAddedToWhitelist, N> {
            self.event_filter::<ChainIDAddedToWhitelist>()
        }
        ///Creates a new event filter for the [`ChainIDRemovedFromWhitelist`] event.
        pub fn ChainIDRemovedFromWhitelist_filter(
            &self,
        ) -> alloy_contract::Event<&P, ChainIDRemovedFromWhitelist, N> {
            self.event_filter::<ChainIDRemovedFromWhitelist>()
        }
        ///Creates a new event filter for the [`GenerationReservationCreated`] event.
        pub fn GenerationReservationCreated_filter(
            &self,
        ) -> alloy_contract::Event<&P, GenerationReservationCreated, N> {
            self.event_filter::<GenerationReservationCreated>()
        }
        ///Creates a new event filter for the [`GenerationReservationRemoved`] event.
        pub fn GenerationReservationRemoved_filter(
            &self,
        ) -> alloy_contract::Event<&P, GenerationReservationRemoved, N> {
            self.event_filter::<GenerationReservationRemoved>()
        }
        ///Creates a new event filter for the [`OperatorSetConfigRemoved`] event.
        pub fn OperatorSetConfigRemoved_filter(
            &self,
        ) -> alloy_contract::Event<&P, OperatorSetConfigRemoved, N> {
            self.event_filter::<OperatorSetConfigRemoved>()
        }
        ///Creates a new event filter for the [`OperatorSetConfigSet`] event.
        pub fn OperatorSetConfigSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, OperatorSetConfigSet, N> {
            self.event_filter::<OperatorSetConfigSet>()
        }
        ///Creates a new event filter for the [`OperatorTableCalculatorRemoved`] event.
        pub fn OperatorTableCalculatorRemoved_filter(
            &self,
        ) -> alloy_contract::Event<&P, OperatorTableCalculatorRemoved, N> {
            self.event_filter::<OperatorTableCalculatorRemoved>()
        }
        ///Creates a new event filter for the [`OperatorTableCalculatorSet`] event.
        pub fn OperatorTableCalculatorSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, OperatorTableCalculatorSet, N> {
            self.event_filter::<OperatorTableCalculatorSet>()
        }
        ///Creates a new event filter for the [`TableUpdateCadenceSet`] event.
        pub fn TableUpdateCadenceSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, TableUpdateCadenceSet, N> {
            self.event_filter::<TableUpdateCadenceSet>()
        }
    }
}
