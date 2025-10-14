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

interface CrossChainRegistry {
    struct OperatorSet {
        address avs;
        uint32 id;
    }

    error ArrayLengthMismatch();
    error ChainIDAlreadyWhitelisted();
    error ChainIDNotWhitelisted();
    error CurrentlyPaused();
    error EmptyChainIDsArray();
    error GenerationReservationAlreadyExists();
    error GenerationReservationDoesNotExist();
    error InputAddressZero();
    error InvalidChainId();
    error InvalidNewPausedStatus();
    error InvalidOperatorSet();
    error InvalidPermissions();
    error InvalidShortString();
    error OnlyPauser();
    error OnlyUnpauser();
    error RequireAtLeastOneTransportDestination();
    error StringTooLong(string str);
    error TransportDestinationAlreadyAdded();
    error TransportDestinationNotFound();

    event ChainIDAddedToWhitelist(uint256 chainID, address operatorTableUpdater);
    event ChainIDRemovedFromWhitelist(uint256 chainID);
    event GenerationReservationCreated(OperatorSet operatorSet);
    event GenerationReservationRemoved(OperatorSet operatorSet);
    event Initialized(uint8 version);
    event OperatorSetConfigRemoved(OperatorSet operatorSet);
    event OperatorSetConfigSet(OperatorSet operatorSet, ICrossChainRegistryTypes.OperatorSetConfig config);
    event OperatorTableCalculatorRemoved(OperatorSet operatorSet);
    event OperatorTableCalculatorSet(OperatorSet operatorSet, address operatorTableCalculator);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event TransportDestinationChainAdded(OperatorSet operatorSet, uint256 chainID);
    event TransportDestinationChainRemoved(OperatorSet operatorSet, uint256 chainID);
    event TransportDestinationsRemoved(OperatorSet operatorSet);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _allocationManager, address _keyRegistrar, address _permissionController, address _pauserRegistry, string _version);

    function addChainIDsToWhitelist(uint256[] memory chainIDs, address[] memory operatorTableUpdaters) external;
    function addTransportDestinations(OperatorSet memory operatorSet, uint256[] memory chainIDs) external;
    function allocationManager() external view returns (address);
    function calculateOperatorTableBytes(OperatorSet memory operatorSet) external view returns (bytes memory);
    function createGenerationReservation(OperatorSet memory operatorSet, address operatorTableCalculator, ICrossChainRegistryTypes.OperatorSetConfig memory config, uint256[] memory chainIDs) external;
    function getActiveGenerationReservations() external view returns (OperatorSet[] memory);
    function getActiveTransportReservations() external view returns (OperatorSet[] memory, uint256[][] memory);
    function getOperatorSetConfig(OperatorSet memory operatorSet) external view returns (ICrossChainRegistryTypes.OperatorSetConfig memory);
    function getOperatorTableCalculator(OperatorSet memory operatorSet) external view returns (address);
    function getSupportedChains() external view returns (uint256[] memory, address[] memory);
    function getTransportDestinations(OperatorSet memory operatorSet) external view returns (uint256[] memory);
    function initialize(address initialOwner, uint256 initialPausedStatus) external;
    function keyRegistrar() external view returns (address);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function permissionController() external view returns (address);
    function removeChainIDsFromWhitelist(uint256[] memory chainIDs) external;
    function removeGenerationReservation(OperatorSet memory operatorSet) external;
    function removeTransportDestinations(OperatorSet memory operatorSet, uint256[] memory chainIDs) external;
    function renounceOwnership() external;
    function setOperatorSetConfig(OperatorSet memory operatorSet, ICrossChainRegistryTypes.OperatorSetConfig memory config) external;
    function setOperatorTableCalculator(OperatorSet memory operatorSet, address operatorTableCalculator) external;
    function transferOwnership(address newOwner) external;
    function unpause(uint256 newPausedStatus) external;
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
        "name": "_allocationManager",
        "type": "address",
        "internalType": "contract IAllocationManager"
      },
      {
        "name": "_keyRegistrar",
        "type": "address",
        "internalType": "contract IKeyRegistrar"
      },
      {
        "name": "_permissionController",
        "type": "address",
        "internalType": "contract IPermissionController"
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
    "name": "addTransportDestinations",
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
      },
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
    "name": "getActiveTransportReservations",
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
      },
      {
        "name": "",
        "type": "uint256[][]",
        "internalType": "uint256[][]"
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
    "name": "getTransportDestinations",
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
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "initialOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "initialPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "keyRegistrar",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IKeyRegistrar"
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
    "name": "removeTransportDestinations",
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
    "name": "renounceOwnership",
    "inputs": [],
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
    "name": "TransportDestinationChainAdded",
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
    "name": "TransportDestinationChainRemoved",
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
    "name": "TransportDestinationsRemoved",
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
    "name": "CurrentlyPaused",
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
    "name": "InputAddressZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidChainId",
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
    "name": "RequireAtLeastOneTransportDestination",
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
    "name": "TransportDestinationAlreadyAdded",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TransportDestinationNotFound",
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
pub mod CrossChainRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610120604052348015610010575f5ffd5b50604051612e0c380380612e0c83398101604081905261002f916101c1565b80838686856001600160a01b03811661005b576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b0390811660805291821660a052811660c0521660e05261008181610098565b610100525061008e6100de565b5050505050610319565b5f5f829050601f815111156100cb578260405163305a27a960e01b81526004016100c291906102be565b60405180910390fd5b80516100d6826102f3565b179392505050565b5f54610100900460ff16156101455760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b60648201526084016100c2565b5f5460ff90811614610194575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146101aa575f5ffd5b50565b634e487b7160e01b5f52604160045260245ffd5b5f5f5f5f5f60a086880312156101d5575f5ffd5b85516101e081610196565b60208701519095506101f181610196565b604087015190945061020281610196565b606087015190935061021381610196565b60808701519092506001600160401b0381111561022e575f5ffd5b8601601f8101881361023e575f5ffd5b80516001600160401b03811115610257576102576101ad565b604051601f8201601f19908116603f011681016001600160401b0381118282101715610285576102856101ad565b6040528181528282016020018a101561029c575f5ffd5b8160208401602083015e5f602083830101528093505050509295509295909350565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80516020808301519190811015610313575f198160200360031b1b821691505b50919050565b60805160a05160c05160e05161010051612a7061039c5f395f610c8b01525f81816102c50152611a9c01525f818161026601526109d401525f81816103e5015281816106630152818161080e01528181610b6d01528181610d1e0152818161157a015261174b01525f8181610381015281816119930152611f430152612a705ff3fe608060405234801561000f575f5ffd5b50600436106101c6575f3560e01c80636c55a37f116100fe578063ca8aa7c71161009e578063f2fde38b1161006e578063f2fde38b14610442578063f3e9f5d414610455578063fabc1cbc14610468578063fe596dee1461047b575f5ffd5b8063ca8aa7c7146103e0578063cd6dc68714610407578063d09b978b1461041a578063dfbd9dfd1461042f575f5ffd5b8063886f1195116100d9578063886f11951461037c5780638da5cb5b146103a3578063bfda3b3d146103b4578063c4bffe2b146103ca575f5ffd5b80636c55a37f1461034e578063715018a61461036157806375e4b53914610369575f5ffd5b806341ee6d0e1161016957806354fd4d501161014457806354fd4d50146102fa578063595c6a67146103025780635ac86ab71461030a5780635c975abb1461033d575f5ffd5b806341ee6d0e146102a05780634657e26a146102c057806349be7d6f146102e7575f5ffd5b806321fa7fdc116101a457806321fa7fdc14610205578063277e1e621461022e5780633c75fddf146102415780633ec45c7e14610261575f5ffd5b806304e98be3146101ca578063136439dd146101df5780631ca9142a146101f2575b5f5ffd5b6101dd6101d8366004612269565b61048e565b005b6101dd6101ed3660046122d3565b6105d2565b6101dd610200366004612314565b61060c565b610218610213366004612403565b610757565b604051610225919061243b565b60405180910390f35b6101dd61023c366004612449565b6107b7565b61025461024f366004612403565b6108f8565b60405161022591906124b6565b6102887f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610225565b6102b36102ae3660046124c8565b6109cf565b6040516102259190612510565b6102887f000000000000000000000000000000000000000000000000000000000000000081565b6101dd6102f5366004612522565b610b16565b6102b3610c84565b6101dd610cb4565b61032d610318366004612571565b606654600160ff9092169190911b9081161490565b6040519015158152602001610225565b606654604051908152602001610225565b6101dd61035c3660046124c8565b610cc8565b6101dd610f3e565b610288610377366004612403565b610f4f565b6102887f000000000000000000000000000000000000000000000000000000000000000081565b6033546001600160a01b0316610288565b6103bc610f7b565b6040516102259291906125cc565b6103d26110e7565b604051610225929190612669565b6102887f000000000000000000000000000000000000000000000000000000000000000081565b6101dd6104153660046126ca565b6111f0565b61042261130c565b60405161022591906126f4565b6101dd61043d366004612706565b6113fe565b6101dd610450366004612744565b6114aa565b6101dd610463366004612522565b611523565b6101dd6104763660046122d3565b611688565b6101dd61048936600461275f565b6116f5565b6104966118da565b60046104a181611934565b8382146104c15760405163512509d360e11b815260040160405180910390fd5b5f5b848110156105ca575f8686838181106104de576104de6127d3565b905060200201359050805f0361050757604051633d23e4d160e11b815260040160405180910390fd5b61053b8186868581811061051d5761051d6127d3565b90506020020160208101906105329190612744565b609c919061195f565b610558576040516324bf631b60e11b815260040160405180910390fd5b7f7a0a76d85b582b17996dd7371a407aa7a79b870db8539247fba315c7b6beff628186868581811061058c5761058c6127d3565b90506020020160208101906105a19190612744565b604080519283526001600160a01b0390911660208301520160405180910390a1506001016104c3565b505050505050565b6105da61197e565b60665481811681146105ff5760405163c61dca5d60e01b815260040160405180910390fd5b61060882611a21565b5050565b600161061781611934565b6106246020840184612744565b61062d81611a5e565b61064a5760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815284906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890610698908490600401612818565b602060405180830381865afa1580156106b3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106d79190612826565b6106f457604051631fb1705560e21b815260040160405180910390fd5b8461072261070f61070a36849003840184612403565b611b0e565b5f90815260986020526040902054151590565b61073f57604051634d2baea960e11b815260040160405180910390fd5b6105ca61075136889003880188612403565b86611b71565b604080518082019091525f8082526020820152609a5f61077684611b0e565b815260208082019290925260409081015f208151808301909252546001600160a01b0381168252600160a01b900463ffffffff169181019190915292915050565b60026107c281611934565b6107cf6020840184612744565b6107d881611a5e565b6107f55760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815284906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890610843908490600401612818565b602060405180830381865afa15801561085e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108829190612826565b61089f57604051631fb1705560e21b815260040160405180910390fd5b846108b561070f61070a36849003840184612403565b6108d257604051634d2baea960e11b815260040160405180910390fd5b6105ca6108e436889003880188612403565b6108f336889003880188612403565b611beb565b60605f609b5f61090785611b0e565b81526020019081526020015f2090505f61092082611c70565b90505f816001600160401b0381111561093b5761093b61234a565b604051908082528060200260200182016040528015610964578160200160208202803683370190505b5090505f805b838110156109c4575f61097d8683611c79565b905061098a609c82611c84565b156109bb57808484815181106109a2576109a26127d3565b6020908102919091010152826109b781612859565b9350505b5060010161096a565b508152949350505050565b6060817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316637cffe48c846040518263ffffffff1660e01b8152600401610a1e9190612818565b602060405180830381865afa158015610a39573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a5d9190612871565b610a6f61021336869003860186612403565b610a8161037736879003870187612403565b6001600160a01b03166341ee6d0e866040518263ffffffff1660e01b8152600401610aac9190612818565b5f60405180830381865afa158015610ac6573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610aed919081019061288f565b604051602001610b009493929190612921565b6040516020818303038152906040529050919050565b6003610b2181611934565b610b2e6020850185612744565b610b3781611a5e565b610b545760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815285906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890610ba2908490600401612818565b602060405180830381865afa158015610bbd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610be19190612826565b610bfe57604051631fb1705560e21b815260040160405180910390fd5b85610c1461070f61070a36849003840184612403565b610c3157604051634d2baea960e11b815260040160405180910390fd5b610c7b610c4336899003890189612403565b8787808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250611c8f92505050565b50505050505050565b6060610caf7f0000000000000000000000000000000000000000000000000000000000000000611d8b565b905090565b610cbc61197e565b610cc65f19611a21565b565b5f610cd281611934565b610cdf6020830183612744565b610ce881611a5e565b610d055760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815283906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890610d53908490600401612818565b602060405180830381865afa158015610d6e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d929190612826565b610daf57604051631fb1705560e21b815260040160405180910390fd5b83610dc561070f61070a36849003840184612403565b610de257604051634d2baea960e11b815260040160405180910390fd5b5f610df561070a36889003880188612403565b5f818152609960205260409081902080546001600160a01b0319169055519091507fd7811913efd5d98fc7ea0d1fdd022b3d31987815360842d05b1d1cf55578d16a90610e43908890612818565b60405180910390a15f818152609a60205260409081902080546001600160c01b0319169055517f210a1118a869246162804e2a7f21ef808ebd93f4be7ed512014fe29a7a8be02e90610e96908890612818565b60405180910390a15f818152609b60205260408120908181610eb882826121fb565b505050507faf209f19ac00e8ccb4539e96d4141cdc96fea479d258d99910307c7365e6875986604051610eeb9190612818565b60405180910390a1610efe609782611dc8565b507f4ffdfdd59e9e1e3c301608788f78dd458e61cb8c045ca92b62a7b484c80824fb86604051610f2e9190612818565b60405180910390a1505050505050565b610f466118da565b610cc65f611dd3565b5f60995f610f5c84611b0e565b815260208101919091526040015f20546001600160a01b031692915050565b6060805f610f896097611c70565b90505f816001600160401b03811115610fa457610fa461234a565b604051908082528060200260200182016040528015610fe857816020015b604080518082019091525f8082526020820152815260200190600190039081610fc25790505b5090505f826001600160401b038111156110045761100461234a565b60405190808252806020026020018201604052801561103757816020015b60608152602001906001900390816110225790505b5090505f5b838110156110dc575f611050609783611c79565b90505f61108c82604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b9050808584815181106110a1576110a16127d3565b60200260200101819052506110b5816108f8565b8484815181106110c7576110c76127d3565b6020908102919091010152505060010161103c565b509094909350915050565b6060805f6110f5609c611e24565b90505f816001600160401b038111156111105761111061234a565b604051908082528060200260200182016040528015611139578160200160208202803683370190505b5090505f826001600160401b038111156111555761115561234a565b60405190808252806020026020018201604052801561117e578160200160208202803683370190505b5090505f5b838110156110dc575f80611198609c84611e2e565b91509150818584815181106111af576111af6127d3565b602002602001018181525050808484815181106111ce576111ce6127d3565b6001600160a01b03909216602092830291909101909101525050600101611183565b5f54610100900460ff161580801561120e57505f54600160ff909116105b806112275750303b15801561122757505f5460ff166001145b61128f5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156112b0575f805461ff0019166101001790555b6112b983611dd3565b6112c282611a21565b8015611307575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050565b60605f6113196097611c70565b90505f816001600160401b038111156113345761133461234a565b60405190808252806020026020018201604052801561137857816020015b604080518082019091525f80825260208201528152602001906001900390816113525790505b5090505f5b828110156113f7575f611391609783611c79565b90505f6113cd82604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b9050808484815181106113e2576113e26127d3565b6020908102919091010152505060010161137d565b5092915050565b6114066118da565b600461141181611934565b5f5b828110156114a4575f84848381811061142e5761142e6127d3565b90506020020135905061144b81609c611e4b90919063ffffffff16565b6114685760405163b3f92ba160e01b815260040160405180910390fd5b6040518181527f6824d36084ecf2cd819b137cb5d837cc6e73afce1e0e348c9fdecaa81d0341e59060200160405180910390a150600101611413565b50505050565b6114b26118da565b6001600160a01b0381166115175760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401611286565b61152081611dd3565b50565b600361152e81611934565b61153b6020850185612744565b61154481611a5e565b6115615760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815285906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc758906115af908490600401612818565b602060405180830381865afa1580156115ca573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115ee9190612826565b61160b57604051631fb1705560e21b815260040160405180910390fd5b8561162161070f61070a36849003840184612403565b61163e57604051634d2baea960e11b815260040160405180910390fd5b610c7b61165036899003890189612403565b8787808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250611e5692505050565b611690611f41565b606654801982198116146116b75760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c9060200160405180910390a25050565b5f6116ff81611934565b61170c6020870187612744565b61171581611a5e565b6117325760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815287906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890611780908490600401612818565b602060405180830381865afa15801561179b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117bf9190612826565b6117dc57604051631fb1705560e21b815260040160405180910390fd5b6117f96117f161070a368b90038b018b612403565b609790611ff2565b61181657604051631883461560e01b815260040160405180910390fd5b7f4fb6efec7dd60036ce3a7af8d5c48425019daa0fb61eb471a966a7ac2c6fa6a6886040516118459190612818565b60405180910390a161186561185f368a90038a018a612403565b88611b71565b611886611877368a90038a018a612403565b6108f336899003890189612403565b6118d0611898368a90038a018a612403565b8686808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250611c8f92505050565b5050505050505050565b6033546001600160a01b03163314610cc65760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401611286565b606654600160ff83161b908116036115205760405163840a48d560e01b815260040160405180910390fd5b5f61197484846001600160a01b038516611ffd565b90505b9392505050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa1580156119e0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a049190612826565b610cc657604051631d77d47760e21b815260040160405180910390fd5b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af1158015611ae4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b089190612826565b92915050565b5f815f0151826020015163ffffffff16604051602001611b5992919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b604051602081830303815290604052611b0890612979565b8060995f611b7e85611b0e565b81526020019081526020015f205f6101000a8154816001600160a01b0302191690836001600160a01b031602179055507f7f7ccafd92d20fdb39dee184a0dce002a9da420ed0def461f2a027abc9b3f6df8282604051611bdf92919061299c565b60405180910390a15050565b80609a5f611bf885611b0e565b815260208082019290925260409081015f2083518154949093015163ffffffff16600160a01b026001600160c01b03199094166001600160a01b0390931692909217929092179055517f3147846ee526009000671c20380b856a633345691300f82585f90034715cf0e290611bdf90849084906129c2565b5f611b08825490565b5f6119778383612019565b5f611977838361203f565b5f815111611cb057604051638631a07560e01b815260040160405180910390fd5b5f611cba83611b0e565b90505f5b82518110156114a4575f838281518110611cda57611cda6127d3565b60200260200101519050611cf881609c611c8490919063ffffffff16565b611d155760405163b3f92ba160e01b815260040160405180910390fd5b5f838152609b60205260409020611d2c9082611ff2565b611d49576040516396d81ac960e01b815260040160405180910390fd5b7f57a1fcb3d9cd447695c46f20944ba562d9547989dcddea0afb119115060c7f0b8582604051611d7a9291906129dd565b60405180910390a150600101611cbe565b60605f611d9783612056565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b5f611977838361207d565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b5f611b0882612160565b5f808080611e3c868661216a565b909450925050505b9250929050565b5f6119778383612193565b5f611e6083611b0e565b90505f5b8251811015611f0c575f838281518110611e8057611e806127d3565b60200260200101519050611ead81609b5f8681526020019081526020015f20611dc890919063ffffffff16565b611eca5760405163ab6cce0760e01b815260040160405180910390fd5b7f499955d838e6f0ca31e83adf81d191cfe6cd8fe252bf826c75c9a80ba077e25e8582604051611efb9291906129dd565b60405180910390a150600101611e64565b505f818152609b60205260408120611f2390611c70565b11611307576040516343629f7b60e01b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611f9d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611fc191906129f8565b6001600160a01b0316336001600160a01b031614610cc65760405163794821ff60e01b815260040160405180910390fd5b5f61197783836121af565b5f82815260028401602052604081208290556119748484611ff2565b5f825f01828154811061202e5761202e6127d3565b905f5260205f200154905092915050565b5f8181526001830160205260408120541515611977565b5f60ff8216601f811115611b0857604051632cd44ac360e21b815260040160405180910390fd5b5f8181526001830160205260408120548015612157575f61209f600183612a13565b85549091505f906120b290600190612a13565b9050818114612111575f865f0182815481106120d0576120d06127d3565b905f5260205f200154905080875f0184815481106120f0576120f06127d3565b5f918252602080832090910192909255918252600188019052604090208390555b855486908061212257612122612a26565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050611b08565b5f915050611b08565b5f611b0882611c70565b5f80806121778585611c79565b5f81815260029690960160205260409095205494959350505050565b5f81815260028301602052604081208190556119778383611dc8565b5f8181526001830160205260408120546121f457508154600181810184555f848152602080822090930184905584548482528286019093526040902091909155611b08565b505f611b08565b5080545f8255905f5260205f209081019061152091905b80821115612225575f8155600101612212565b5090565b5f5f83601f840112612239575f5ffd5b5081356001600160401b0381111561224f575f5ffd5b6020830191508360208260051b8501011115611e44575f5ffd5b5f5f5f5f6040858703121561227c575f5ffd5b84356001600160401b03811115612291575f5ffd5b61229d87828801612229565b90955093505060208501356001600160401b038111156122bb575f5ffd5b6122c787828801612229565b95989497509550505050565b5f602082840312156122e3575f5ffd5b5035919050565b5f604082840312156122fa575f5ffd5b50919050565b6001600160a01b0381168114611520575f5ffd5b5f5f60608385031215612325575f5ffd5b61232f84846122ea565b9150604083013561233f81612300565b809150509250929050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b03811182821017156123865761238661234a565b604052919050565b803563ffffffff811681146123a1575f5ffd5b919050565b5f604082840312156123b6575f5ffd5b604080519081016001600160401b03811182821017156123d8576123d861234a565b60405290508082356123e981612300565b81526123f76020840161238e565b60208201525092915050565b5f60408284031215612413575f5ffd5b61197783836123a6565b80516001600160a01b0316825260209081015163ffffffff16910152565b60408101611b08828461241d565b5f5f6080838503121561245a575f5ffd5b61246484846122ea565b915061247384604085016122ea565b90509250929050565b5f8151808452602084019350602083015f5b828110156124ac57815186526020958601959091019060010161248e565b5093949350505050565b602081525f611977602083018461247c565b5f604082840312156124d8575f5ffd5b61197783836122ea565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f61197760208301846124e2565b5f5f5f60608486031215612534575f5ffd5b61253e85856122ea565b925060408401356001600160401b03811115612558575f5ffd5b61256486828701612229565b9497909650939450505050565b5f60208284031215612581575f5ffd5b813560ff81168114611977575f5ffd5b5f8151808452602084019350602083015f5b828110156124ac576125b686835161241d565b60409590950194602091909101906001016125a3565b604081525f6125de6040830185612591565b828103602084015280845180835260208301915060208160051b840101602087015f5b8381101561265b57858303601f19018552815180518085526020918201918501905f5b81811015612642578351835260209384019390920191600101612624565b5050602096870196909450929092019150600101612601565b509098975050505050505050565b604081525f61267b604083018561247c565b82810360208401528084518083526020830191506020860192505f5b818110156126be5783516001600160a01b0316835260209384019390920191600101612697565b50909695505050505050565b5f5f604083850312156126db575f5ffd5b82356126e681612300565b946020939093013593505050565b602081525f6119776020830184612591565b5f5f60208385031215612717575f5ffd5b82356001600160401b0381111561272c575f5ffd5b61273885828601612229565b90969095509350505050565b5f60208284031215612754575f5ffd5b813561197781612300565b5f5f5f5f5f60c08688031215612773575f5ffd5b61277d87876122ea565b9450604086013561278d81612300565b935061279c87606088016122ea565b925060a08601356001600160401b038111156127b6575f5ffd5b6127c288828901612229565b969995985093965092949392505050565b634e487b7160e01b5f52603260045260245ffd5b80356127f281612300565b6001600160a01b0316825263ffffffff61280e6020830161238e565b1660208301525050565b60408101611b0882846127e7565b5f60208284031215612836575f5ffd5b81518015158114611977575f5ffd5b634e487b7160e01b5f52601160045260245ffd5b5f6001820161286a5761286a612845565b5060010190565b5f60208284031215612881575f5ffd5b815160038110611977575f5ffd5b5f6020828403121561289f575f5ffd5b81516001600160401b038111156128b4575f5ffd5b8201601f810184136128c4575f5ffd5b80516001600160401b038111156128dd576128dd61234a565b6128f0601f8201601f191660200161235e565b818152856020838501011115612904575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b61292b81866127e7565b5f6003851061294857634e487b7160e01b5f52602160045260245ffd5b84604083015261295b606083018561241d565b60c060a083015261296f60c08301846124e2565b9695505050505050565b805160208083015191908110156122fa575f1960209190910360031b1b16919050565b606081016129aa828561241d565b6001600160a01b039290921660409190910152919050565b608081016129d0828561241d565b611977604083018461241d565b606081016129eb828561241d565b8260408301529392505050565b5f60208284031215612a08575f5ffd5b815161197781612300565b81810381811115611b0857611b08612845565b634e487b7160e01b5f52603160045260245ffdfea264697066735822122079cbe8fbbc74928b40bff8981593ef67cf90bac6e471ca7182fc5733d7da90c264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01 `@R4\x80\x15a\0\x10W__\xFD[P`@Qa.\x0C8\x03\x80a.\x0C\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xC1V[\x80\x83\x86\x86\x85`\x01`\x01`\xA0\x1B\x03\x81\x16a\0[W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80R\x91\x82\x16`\xA0R\x81\x16`\xC0R\x16`\xE0Ra\0\x81\x81a\0\x98V[a\x01\0RPa\0\x8Ea\0\xDEV[PPPPPa\x03\x19V[__\x82\x90P`\x1F\x81Q\x11\x15a\0\xCBW\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\0\xC2\x91\x90a\x02\xBEV[`@Q\x80\x91\x03\x90\xFD[\x80Qa\0\xD6\x82a\x02\xF3V[\x17\x93\x92PPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x01EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01a\0\xC2V[_T`\xFF\x90\x81\x16\x14a\x01\x94W_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xAAW__\xFD[PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_____`\xA0\x86\x88\x03\x12\x15a\x01\xD5W__\xFD[\x85Qa\x01\xE0\x81a\x01\x96V[` \x87\x01Q\x90\x95Pa\x01\xF1\x81a\x01\x96V[`@\x87\x01Q\x90\x94Pa\x02\x02\x81a\x01\x96V[``\x87\x01Q\x90\x93Pa\x02\x13\x81a\x01\x96V[`\x80\x87\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02.W__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\x02>W__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02WWa\x02Wa\x01\xADV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02\x85Wa\x02\x85a\x01\xADV[`@R\x81\x81R\x82\x82\x01` \x01\x8A\x10\x15a\x02\x9CW__\xFD[\x81` \x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x93PPPP\x92\x95P\x92\x95\x90\x93PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x03\x13W_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa*pa\x03\x9C_9_a\x0C\x8B\x01R_\x81\x81a\x02\xC5\x01Ra\x1A\x9C\x01R_\x81\x81a\x02f\x01Ra\t\xD4\x01R_\x81\x81a\x03\xE5\x01R\x81\x81a\x06c\x01R\x81\x81a\x08\x0E\x01R\x81\x81a\x0Bm\x01R\x81\x81a\r\x1E\x01R\x81\x81a\x15z\x01Ra\x17K\x01R_\x81\x81a\x03\x81\x01R\x81\x81a\x19\x93\x01Ra\x1FC\x01Ra*p_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xC6W_5`\xE0\x1C\x80clU\xA3\x7F\x11a\0\xFEW\x80c\xCA\x8A\xA7\xC7\x11a\0\x9EW\x80c\xF2\xFD\xE3\x8B\x11a\0nW\x80c\xF2\xFD\xE3\x8B\x14a\x04BW\x80c\xF3\xE9\xF5\xD4\x14a\x04UW\x80c\xFA\xBC\x1C\xBC\x14a\x04hW\x80c\xFEYm\xEE\x14a\x04{W__\xFD[\x80c\xCA\x8A\xA7\xC7\x14a\x03\xE0W\x80c\xCDm\xC6\x87\x14a\x04\x07W\x80c\xD0\x9B\x97\x8B\x14a\x04\x1AW\x80c\xDF\xBD\x9D\xFD\x14a\x04/W__\xFD[\x80c\x88o\x11\x95\x11a\0\xD9W\x80c\x88o\x11\x95\x14a\x03|W\x80c\x8D\xA5\xCB[\x14a\x03\xA3W\x80c\xBF\xDA;=\x14a\x03\xB4W\x80c\xC4\xBF\xFE+\x14a\x03\xCAW__\xFD[\x80clU\xA3\x7F\x14a\x03NW\x80cqP\x18\xA6\x14a\x03aW\x80cu\xE4\xB59\x14a\x03iW__\xFD[\x80cA\xEEm\x0E\x11a\x01iW\x80cT\xFDMP\x11a\x01DW\x80cT\xFDMP\x14a\x02\xFAW\x80cY\\jg\x14a\x03\x02W\x80cZ\xC8j\xB7\x14a\x03\nW\x80c\\\x97Z\xBB\x14a\x03=W__\xFD[\x80cA\xEEm\x0E\x14a\x02\xA0W\x80cFW\xE2j\x14a\x02\xC0W\x80cI\xBE}o\x14a\x02\xE7W__\xFD[\x80c!\xFA\x7F\xDC\x11a\x01\xA4W\x80c!\xFA\x7F\xDC\x14a\x02\x05W\x80c'~\x1Eb\x14a\x02.W\x80c<u\xFD\xDF\x14a\x02AW\x80c>\xC4\\~\x14a\x02aW__\xFD[\x80c\x04\xE9\x8B\xE3\x14a\x01\xCAW\x80c\x13d9\xDD\x14a\x01\xDFW\x80c\x1C\xA9\x14*\x14a\x01\xF2W[__\xFD[a\x01\xDDa\x01\xD86`\x04a\"iV[a\x04\x8EV[\0[a\x01\xDDa\x01\xED6`\x04a\"\xD3V[a\x05\xD2V[a\x01\xDDa\x02\x006`\x04a#\x14V[a\x06\x0CV[a\x02\x18a\x02\x136`\x04a$\x03V[a\x07WV[`@Qa\x02%\x91\x90a$;V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDDa\x02<6`\x04a$IV[a\x07\xB7V[a\x02Ta\x02O6`\x04a$\x03V[a\x08\xF8V[`@Qa\x02%\x91\x90a$\xB6V[a\x02\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02%V[a\x02\xB3a\x02\xAE6`\x04a$\xC8V[a\t\xCFV[`@Qa\x02%\x91\x90a%\x10V[a\x02\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xDDa\x02\xF56`\x04a%\"V[a\x0B\x16V[a\x02\xB3a\x0C\x84V[a\x01\xDDa\x0C\xB4V[a\x03-a\x03\x186`\x04a%qV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02%V[`fT`@Q\x90\x81R` \x01a\x02%V[a\x01\xDDa\x03\\6`\x04a$\xC8V[a\x0C\xC8V[a\x01\xDDa\x0F>V[a\x02\x88a\x03w6`\x04a$\x03V[a\x0FOV[a\x02\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\x88V[a\x03\xBCa\x0F{V[`@Qa\x02%\x92\x91\x90a%\xCCV[a\x03\xD2a\x10\xE7V[`@Qa\x02%\x92\x91\x90a&iV[a\x02\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xDDa\x04\x156`\x04a&\xCAV[a\x11\xF0V[a\x04\"a\x13\x0CV[`@Qa\x02%\x91\x90a&\xF4V[a\x01\xDDa\x04=6`\x04a'\x06V[a\x13\xFEV[a\x01\xDDa\x04P6`\x04a'DV[a\x14\xAAV[a\x01\xDDa\x04c6`\x04a%\"V[a\x15#V[a\x01\xDDa\x04v6`\x04a\"\xD3V[a\x16\x88V[a\x01\xDDa\x04\x896`\x04a'_V[a\x16\xF5V[a\x04\x96a\x18\xDAV[`\x04a\x04\xA1\x81a\x194V[\x83\x82\x14a\x04\xC1W`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x05\xCAW_\x86\x86\x83\x81\x81\x10a\x04\xDEWa\x04\xDEa'\xD3V[\x90P` \x02\x015\x90P\x80_\x03a\x05\x07W`@Qc=#\xE4\xD1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05;\x81\x86\x86\x85\x81\x81\x10a\x05\x1DWa\x05\x1Da'\xD3V[\x90P` \x02\x01` \x81\x01\x90a\x052\x91\x90a'DV[`\x9C\x91\x90a\x19_V[a\x05XW`@Qc$\xBFc\x1B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7Fz\nv\xD8[X+\x17\x99m\xD77\x1A@z\xA7\xA7\x9B\x87\r\xB8S\x92G\xFB\xA3\x15\xC7\xB6\xBE\xFFb\x81\x86\x86\x85\x81\x81\x10a\x05\x8CWa\x05\x8Ca'\xD3V[\x90P` \x02\x01` \x81\x01\x90a\x05\xA1\x91\x90a'DV[`@\x80Q\x92\x83R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1P`\x01\x01a\x04\xC3V[PPPPPPV[a\x05\xDAa\x19~V[`fT\x81\x81\x16\x81\x14a\x05\xFFW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\x08\x82a\x1A!V[PPV[`\x01a\x06\x17\x81a\x194V[a\x06$` \x84\x01\x84a'DV[a\x06-\x81a\x1A^V[a\x06JW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x84\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x06\x98\x90\x84\x90`\x04\x01a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xB3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD7\x91\x90a(&V[a\x06\xF4W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84a\x07\"a\x07\x0Fa\x07\n6\x84\x90\x03\x84\x01\x84a$\x03V[a\x1B\x0EV[_\x90\x81R`\x98` R`@\x90 T\x15\x15\x90V[a\x07?W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xCAa\x07Q6\x88\x90\x03\x88\x01\x88a$\x03V[\x86a\x1BqV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x9A_a\x07v\x84a\x1B\x0EV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q\x80\x83\x01\x90\x92RT`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x92\x91PPV[`\x02a\x07\xC2\x81a\x194V[a\x07\xCF` \x84\x01\x84a'DV[a\x07\xD8\x81a\x1A^V[a\x07\xF5W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x84\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x08C\x90\x84\x90`\x04\x01a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08^W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x82\x91\x90a(&V[a\x08\x9FW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84a\x08\xB5a\x07\x0Fa\x07\n6\x84\x90\x03\x84\x01\x84a$\x03V[a\x08\xD2W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xCAa\x08\xE46\x88\x90\x03\x88\x01\x88a$\x03V[a\x08\xF36\x88\x90\x03\x88\x01\x88a$\x03V[a\x1B\xEBV[``_`\x9B_a\t\x07\x85a\x1B\x0EV[\x81R` \x01\x90\x81R` \x01_ \x90P_a\t \x82a\x1CpV[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\t;Wa\t;a#JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\tdW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x83\x81\x10\x15a\t\xC4W_a\t}\x86\x83a\x1CyV[\x90Pa\t\x8A`\x9C\x82a\x1C\x84V[\x15a\t\xBBW\x80\x84\x84\x81Q\x81\x10a\t\xA2Wa\t\xA2a'\xD3V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x82a\t\xB7\x81a(YV[\x93PP[P`\x01\x01a\tjV[P\x81R\x94\x93PPPPV[``\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c|\xFF\xE4\x8C\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x1E\x91\x90a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n]\x91\x90a(qV[a\noa\x02\x136\x86\x90\x03\x86\x01\x86a$\x03V[a\n\x81a\x03w6\x87\x90\x03\x87\x01\x87a$\x03V[`\x01`\x01`\xA0\x1B\x03\x16cA\xEEm\x0E\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xAC\x91\x90a(\x18V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xC6W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\xED\x91\x90\x81\x01\x90a(\x8FV[`@Q` \x01a\x0B\0\x94\x93\x92\x91\x90a)!V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\x03a\x0B!\x81a\x194V[a\x0B.` \x85\x01\x85a'DV[a\x0B7\x81a\x1A^V[a\x0BTW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x85\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x0B\xA2\x90\x84\x90`\x04\x01a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xBDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xE1\x91\x90a(&V[a\x0B\xFEW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85a\x0C\x14a\x07\x0Fa\x07\n6\x84\x90\x03\x84\x01\x84a$\x03V[a\x0C1W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C{a\x0CC6\x89\x90\x03\x89\x01\x89a$\x03V[\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x1C\x8F\x92PPPV[PPPPPPPV[``a\x0C\xAF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x8BV[\x90P\x90V[a\x0C\xBCa\x19~V[a\x0C\xC6_\x19a\x1A!V[V[_a\x0C\xD2\x81a\x194V[a\x0C\xDF` \x83\x01\x83a'DV[a\x0C\xE8\x81a\x1A^V[a\r\x05W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x83\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\rS\x90\x84\x90`\x04\x01a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rnW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x92\x91\x90a(&V[a\r\xAFW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a\r\xC5a\x07\x0Fa\x07\n6\x84\x90\x03\x84\x01\x84a$\x03V[a\r\xE2W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\r\xF5a\x07\n6\x88\x90\x03\x88\x01\x88a$\x03V[_\x81\x81R`\x99` R`@\x90\x81\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UQ\x90\x91P\x7F\xD7\x81\x19\x13\xEF\xD5\xD9\x8F\xC7\xEA\r\x1F\xDD\x02+=1\x98x\x156\x08B\xD0[\x1D\x1C\xF5Ux\xD1j\x90a\x0EC\x90\x88\x90a(\x18V[`@Q\x80\x91\x03\x90\xA1_\x81\x81R`\x9A` R`@\x90\x81\x90 \x80T`\x01`\x01`\xC0\x1B\x03\x19\x16\x90UQ\x7F!\n\x11\x18\xA8i$ab\x80N*\x7F!\xEF\x80\x8E\xBD\x93\xF4\xBE~\xD5\x12\x01O\xE2\x9Az\x8B\xE0.\x90a\x0E\x96\x90\x88\x90a(\x18V[`@Q\x80\x91\x03\x90\xA1_\x81\x81R`\x9B` R`@\x81 \x90\x81\x81a\x0E\xB8\x82\x82a!\xFBV[PPPP\x7F\xAF \x9F\x19\xAC\0\xE8\xCC\xB4S\x9E\x96\xD4\x14\x1C\xDC\x96\xFE\xA4y\xD2X\xD9\x99\x100|se\xE6\x87Y\x86`@Qa\x0E\xEB\x91\x90a(\x18V[`@Q\x80\x91\x03\x90\xA1a\x0E\xFE`\x97\x82a\x1D\xC8V[P\x7FO\xFD\xFD\xD5\x9E\x9E\x1E<0\x16\x08x\x8Fx\xDDE\x8Ea\xCB\x8C\x04\\\xA9+b\xA7\xB4\x84\xC8\x08$\xFB\x86`@Qa\x0F.\x91\x90a(\x18V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[a\x0FFa\x18\xDAV[a\x0C\xC6_a\x1D\xD3V[_`\x99_a\x0F\\\x84a\x1B\x0EV[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[``\x80_a\x0F\x89`\x97a\x1CpV[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xA4Wa\x0F\xA4a#JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\xE8W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0F\xC2W\x90P[P\x90P_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\x04Wa\x10\x04a#JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x107W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10\"W\x90P[P\x90P_[\x83\x81\x10\x15a\x10\xDCW_a\x10P`\x97\x83a\x1CyV[\x90P_a\x10\x8C\x82`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x90P\x80\x85\x84\x81Q\x81\x10a\x10\xA1Wa\x10\xA1a'\xD3V[` \x02` \x01\x01\x81\x90RPa\x10\xB5\x81a\x08\xF8V[\x84\x84\x81Q\x81\x10a\x10\xC7Wa\x10\xC7a'\xD3V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x10<V[P\x90\x94\x90\x93P\x91PPV[``\x80_a\x10\xF5`\x9Ca\x1E$V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x10Wa\x11\x10a#JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x119W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11UWa\x11Ua#JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11~W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x10\xDCW_\x80a\x11\x98`\x9C\x84a\x1E.V[\x91P\x91P\x81\x85\x84\x81Q\x81\x10a\x11\xAFWa\x11\xAFa'\xD3V[` \x02` \x01\x01\x81\x81RPP\x80\x84\x84\x81Q\x81\x10a\x11\xCEWa\x11\xCEa'\xD3V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPP`\x01\x01a\x11\x83V[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12\x0EWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12'WP0;\x15\x80\x15a\x12'WP_T`\xFF\x16`\x01\x14[a\x12\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12\xB0W_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12\xB9\x83a\x1D\xD3V[a\x12\xC2\x82a\x1A!V[\x80\x15a\x13\x07W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[``_a\x13\x19`\x97a\x1CpV[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x134Wa\x134a#JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13xW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13RW\x90P[P\x90P_[\x82\x81\x10\x15a\x13\xF7W_a\x13\x91`\x97\x83a\x1CyV[\x90P_a\x13\xCD\x82`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x90P\x80\x84\x84\x81Q\x81\x10a\x13\xE2Wa\x13\xE2a'\xD3V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x13}V[P\x92\x91PPV[a\x14\x06a\x18\xDAV[`\x04a\x14\x11\x81a\x194V[_[\x82\x81\x10\x15a\x14\xA4W_\x84\x84\x83\x81\x81\x10a\x14.Wa\x14.a'\xD3V[\x90P` \x02\x015\x90Pa\x14K\x81`\x9Ca\x1EK\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x14hW`@Qc\xB3\xF9+\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x81\x81R\x7Fh$\xD3`\x84\xEC\xF2\xCD\x81\x9B\x13|\xB5\xD87\xCCns\xAF\xCE\x1E\x0E4\x8C\x9F\xDE\xCA\xA8\x1D\x03A\xE5\x90` \x01`@Q\x80\x91\x03\x90\xA1P`\x01\x01a\x14\x13V[PPPPV[a\x14\xB2a\x18\xDAV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x15\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x12\x86V[a\x15 \x81a\x1D\xD3V[PV[`\x03a\x15.\x81a\x194V[a\x15;` \x85\x01\x85a'DV[a\x15D\x81a\x1A^V[a\x15aW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x85\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x15\xAF\x90\x84\x90`\x04\x01a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xCAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xEE\x91\x90a(&V[a\x16\x0BW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85a\x16!a\x07\x0Fa\x07\n6\x84\x90\x03\x84\x01\x84a$\x03V[a\x16>W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C{a\x16P6\x89\x90\x03\x89\x01\x89a$\x03V[\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x1EV\x92PPPV[a\x16\x90a\x1FAV[`fT\x80\x19\x82\x19\x81\x16\x14a\x16\xB7W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[_a\x16\xFF\x81a\x194V[a\x17\x0C` \x87\x01\x87a'DV[a\x17\x15\x81a\x1A^V[a\x172W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x87\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x17\x80\x90\x84\x90`\x04\x01a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x9BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xBF\x91\x90a(&V[a\x17\xDCW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17\xF9a\x17\xF1a\x07\n6\x8B\x90\x03\x8B\x01\x8Ba$\x03V[`\x97\x90a\x1F\xF2V[a\x18\x16W`@Qc\x18\x83F\x15`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7FO\xB6\xEF\xEC}\xD6\x006\xCE:z\xF8\xD5\xC4\x84%\x01\x9D\xAA\x0F\xB6\x1E\xB4q\xA9f\xA7\xAC,o\xA6\xA6\x88`@Qa\x18E\x91\x90a(\x18V[`@Q\x80\x91\x03\x90\xA1a\x18ea\x18_6\x8A\x90\x03\x8A\x01\x8Aa$\x03V[\x88a\x1BqV[a\x18\x86a\x18w6\x8A\x90\x03\x8A\x01\x8Aa$\x03V[a\x08\xF36\x89\x90\x03\x89\x01\x89a$\x03V[a\x18\xD0a\x18\x986\x8A\x90\x03\x8A\x01\x8Aa$\x03V[\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x1C\x8F\x92PPPV[PPPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x12\x86V[`fT`\x01`\xFF\x83\x16\x1B\x90\x81\x16\x03a\x15 W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x19t\x84\x84`\x01`\x01`\xA0\x1B\x03\x85\x16a\x1F\xFDV[\x90P[\x93\x92PPPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xE0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x04\x91\x90a(&V[a\x0C\xC6W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1A\xE4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x08\x91\x90a(&V[\x92\x91PPV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a\x1BY\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1B\x08\x90a)yV[\x80`\x99_a\x1B~\x85a\x1B\x0EV[\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x7F\x7F|\xCA\xFD\x92\xD2\x0F\xDB9\xDE\xE1\x84\xA0\xDC\xE0\x02\xA9\xDAB\x0E\xD0\xDE\xF4a\xF2\xA0'\xAB\xC9\xB3\xF6\xDF\x82\x82`@Qa\x1B\xDF\x92\x91\x90a)\x9CV[`@Q\x80\x91\x03\x90\xA1PPV[\x80`\x9A_a\x1B\xF8\x85a\x1B\x0EV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x83Q\x81T\x94\x90\x93\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x92\x90\x92\x17\x90UQ\x7F1G\x84n\xE5&\0\x90\0g\x1C 8\x0B\x85jc3Ei\x13\0\xF8%\x85\xF9\x004q\\\xF0\xE2\x90a\x1B\xDF\x90\x84\x90\x84\x90a)\xC2V[_a\x1B\x08\x82T\x90V[_a\x19w\x83\x83a \x19V[_a\x19w\x83\x83a ?V[_\x81Q\x11a\x1C\xB0W`@Qc\x861\xA0u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x1C\xBA\x83a\x1B\x0EV[\x90P_[\x82Q\x81\x10\x15a\x14\xA4W_\x83\x82\x81Q\x81\x10a\x1C\xDAWa\x1C\xDAa'\xD3V[` \x02` \x01\x01Q\x90Pa\x1C\xF8\x81`\x9Ca\x1C\x84\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1D\x15W`@Qc\xB3\xF9+\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x81R`\x9B` R`@\x90 a\x1D,\x90\x82a\x1F\xF2V[a\x1DIW`@Qc\x96\xD8\x1A\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7FW\xA1\xFC\xB3\xD9\xCDDv\x95\xC4o \x94K\xA5b\xD9Ty\x89\xDC\xDD\xEA\n\xFB\x11\x91\x15\x06\x0C\x7F\x0B\x85\x82`@Qa\x1Dz\x92\x91\x90a)\xDDV[`@Q\x80\x91\x03\x90\xA1P`\x01\x01a\x1C\xBEV[``_a\x1D\x97\x83a VV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[_a\x19w\x83\x83a }V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[_a\x1B\x08\x82a!`V[_\x80\x80\x80a\x1E<\x86\x86a!jV[\x90\x94P\x92PPP[\x92P\x92\x90PV[_a\x19w\x83\x83a!\x93V[_a\x1E`\x83a\x1B\x0EV[\x90P_[\x82Q\x81\x10\x15a\x1F\x0CW_\x83\x82\x81Q\x81\x10a\x1E\x80Wa\x1E\x80a'\xD3V[` \x02` \x01\x01Q\x90Pa\x1E\xAD\x81`\x9B_\x86\x81R` \x01\x90\x81R` \x01_ a\x1D\xC8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1E\xCAW`@Qc\xABl\xCE\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7FI\x99U\xD88\xE6\xF0\xCA1\xE8:\xDF\x81\xD1\x91\xCF\xE6\xCD\x8F\xE2R\xBF\x82lu\xC9\xA8\x0B\xA0w\xE2^\x85\x82`@Qa\x1E\xFB\x92\x91\x90a)\xDDV[`@Q\x80\x91\x03\x90\xA1P`\x01\x01a\x1EdV[P_\x81\x81R`\x9B` R`@\x81 a\x1F#\x90a\x1CpV[\x11a\x13\x07W`@QcCb\x9F{`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x9DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xC1\x91\x90a)\xF8V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xC6W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x19w\x83\x83a!\xAFV[_\x82\x81R`\x02\x84\x01` R`@\x81 \x82\x90Ua\x19t\x84\x84a\x1F\xF2V[_\x82_\x01\x82\x81T\x81\x10a .Wa .a'\xD3V[\x90_R` _ \x01T\x90P\x92\x91PPV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x19wV[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x1B\x08W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a!WW_a \x9F`\x01\x83a*\x13V[\x85T\x90\x91P_\x90a \xB2\x90`\x01\x90a*\x13V[\x90P\x81\x81\x14a!\x11W_\x86_\x01\x82\x81T\x81\x10a \xD0Wa \xD0a'\xD3V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a \xF0Wa \xF0a'\xD3V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a!\"Wa!\"a*&V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x1B\x08V[_\x91PPa\x1B\x08V[_a\x1B\x08\x82a\x1CpV[_\x80\x80a!w\x85\x85a\x1CyV[_\x81\x81R`\x02\x96\x90\x96\x01` R`@\x90\x95 T\x94\x95\x93PPPPV[_\x81\x81R`\x02\x83\x01` R`@\x81 \x81\x90Ua\x19w\x83\x83a\x1D\xC8V[_\x81\x81R`\x01\x83\x01` R`@\x81 Ta!\xF4WP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x1B\x08V[P_a\x1B\x08V[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a\x15 \x91\x90[\x80\x82\x11\x15a\"%W_\x81U`\x01\x01a\"\x12V[P\x90V[__\x83`\x1F\x84\x01\x12a\"9W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\"OW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1EDW__\xFD[____`@\x85\x87\x03\x12\x15a\"|W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x91W__\xFD[a\"\x9D\x87\x82\x88\x01a\")V[\x90\x95P\x93PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xBBW__\xFD[a\"\xC7\x87\x82\x88\x01a\")V[\x95\x98\x94\x97P\x95PPPPV[_` \x82\x84\x03\x12\x15a\"\xE3W__\xFD[P5\x91\x90PV[_`@\x82\x84\x03\x12\x15a\"\xFAW__\xFD[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15 W__\xFD[__``\x83\x85\x03\x12\x15a#%W__\xFD[a#/\x84\x84a\"\xEAV[\x91P`@\x83\x015a#?\x81a#\0V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a#\x86Wa#\x86a#JV[`@R\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a#\xA1W__\xFD[\x91\x90PV[_`@\x82\x84\x03\x12\x15a#\xB6W__\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a#\xD8Wa#\xD8a#JV[`@R\x90P\x80\x825a#\xE9\x81a#\0V[\x81Ra#\xF7` \x84\x01a#\x8EV[` \x82\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15a$\x13W__\xFD[a\x19w\x83\x83a#\xA6V[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`@\x81\x01a\x1B\x08\x82\x84a$\x1DV[__`\x80\x83\x85\x03\x12\x15a$ZW__\xFD[a$d\x84\x84a\"\xEAV[\x91Pa$s\x84`@\x85\x01a\"\xEAV[\x90P\x92P\x92\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a$\xACW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a$\x8EV[P\x93\x94\x93PPPPV[` \x81R_a\x19w` \x83\x01\x84a$|V[_`@\x82\x84\x03\x12\x15a$\xD8W__\xFD[a\x19w\x83\x83a\"\xEAV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x19w` \x83\x01\x84a$\xE2V[___``\x84\x86\x03\x12\x15a%4W__\xFD[a%>\x85\x85a\"\xEAV[\x92P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a%XW__\xFD[a%d\x86\x82\x87\x01a\")V[\x94\x97\x90\x96P\x93\x94PPPPV[_` \x82\x84\x03\x12\x15a%\x81W__\xFD[\x815`\xFF\x81\x16\x81\x14a\x19wW__\xFD[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a$\xACWa%\xB6\x86\x83Qa$\x1DV[`@\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01a%\xA3V[`@\x81R_a%\xDE`@\x83\x01\x85a%\x91V[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x87\x01_[\x83\x81\x10\x15a&[W\x85\x83\x03`\x1F\x19\x01\x85R\x81Q\x80Q\x80\x85R` \x91\x82\x01\x91\x85\x01\x90_[\x81\x81\x10\x15a&BW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a&$V[PP` \x96\x87\x01\x96\x90\x94P\x92\x90\x92\x01\x91P`\x01\x01a&\x01V[P\x90\x98\x97PPPPPPPPV[`@\x81R_a&{`@\x83\x01\x85a$|V[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x86\x01\x92P_[\x81\x81\x10\x15a&\xBEW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a&\x97V[P\x90\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15a&\xDBW__\xFD[\x825a&\xE6\x81a#\0V[\x94` \x93\x90\x93\x015\x93PPPV[` \x81R_a\x19w` \x83\x01\x84a%\x91V[__` \x83\x85\x03\x12\x15a'\x17W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a',W__\xFD[a'8\x85\x82\x86\x01a\")V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15a'TW__\xFD[\x815a\x19w\x81a#\0V[_____`\xC0\x86\x88\x03\x12\x15a'sW__\xFD[a'}\x87\x87a\"\xEAV[\x94P`@\x86\x015a'\x8D\x81a#\0V[\x93Pa'\x9C\x87``\x88\x01a\"\xEAV[\x92P`\xA0\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xB6W__\xFD[a'\xC2\x88\x82\x89\x01a\")V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x805a'\xF2\x81a#\0V[`\x01`\x01`\xA0\x1B\x03\x16\x82Rc\xFF\xFF\xFF\xFFa(\x0E` \x83\x01a#\x8EV[\x16` \x83\x01RPPV[`@\x81\x01a\x1B\x08\x82\x84a'\xE7V[_` \x82\x84\x03\x12\x15a(6W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x19wW__\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01a(jWa(ja(EV[P`\x01\x01\x90V[_` \x82\x84\x03\x12\x15a(\x81W__\xFD[\x81Q`\x03\x81\x10a\x19wW__\xFD[_` \x82\x84\x03\x12\x15a(\x9FW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xB4W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a(\xC4W__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xDDWa(\xDDa#JV[a(\xF0`\x1F\x82\x01`\x1F\x19\x16` \x01a#^V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a)\x04W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[a)+\x81\x86a'\xE7V[_`\x03\x85\x10a)HWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x84`@\x83\x01Ra)[``\x83\x01\x85a$\x1DV[`\xC0`\xA0\x83\x01Ra)o`\xC0\x83\x01\x84a$\xE2V[\x96\x95PPPPPPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\"\xFAW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[``\x81\x01a)\xAA\x82\x85a$\x1DV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`@\x91\x90\x91\x01R\x91\x90PV[`\x80\x81\x01a)\xD0\x82\x85a$\x1DV[a\x19w`@\x83\x01\x84a$\x1DV[``\x81\x01a)\xEB\x82\x85a$\x1DV[\x82`@\x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a*\x08W__\xFD[\x81Qa\x19w\x81a#\0V[\x81\x81\x03\x81\x81\x11\x15a\x1B\x08Wa\x1B\x08a(EV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 y\xCB\xE8\xFB\xBCt\x92\x8B@\xBF\xF8\x98\x15\x93\xEFg\xCF\x90\xBA\xC6\xE4q\xCAq\x82\xFCW3\xD7\xDA\x90\xC2dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106101c6575f3560e01c80636c55a37f116100fe578063ca8aa7c71161009e578063f2fde38b1161006e578063f2fde38b14610442578063f3e9f5d414610455578063fabc1cbc14610468578063fe596dee1461047b575f5ffd5b8063ca8aa7c7146103e0578063cd6dc68714610407578063d09b978b1461041a578063dfbd9dfd1461042f575f5ffd5b8063886f1195116100d9578063886f11951461037c5780638da5cb5b146103a3578063bfda3b3d146103b4578063c4bffe2b146103ca575f5ffd5b80636c55a37f1461034e578063715018a61461036157806375e4b53914610369575f5ffd5b806341ee6d0e1161016957806354fd4d501161014457806354fd4d50146102fa578063595c6a67146103025780635ac86ab71461030a5780635c975abb1461033d575f5ffd5b806341ee6d0e146102a05780634657e26a146102c057806349be7d6f146102e7575f5ffd5b806321fa7fdc116101a457806321fa7fdc14610205578063277e1e621461022e5780633c75fddf146102415780633ec45c7e14610261575f5ffd5b806304e98be3146101ca578063136439dd146101df5780631ca9142a146101f2575b5f5ffd5b6101dd6101d8366004612269565b61048e565b005b6101dd6101ed3660046122d3565b6105d2565b6101dd610200366004612314565b61060c565b610218610213366004612403565b610757565b604051610225919061243b565b60405180910390f35b6101dd61023c366004612449565b6107b7565b61025461024f366004612403565b6108f8565b60405161022591906124b6565b6102887f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610225565b6102b36102ae3660046124c8565b6109cf565b6040516102259190612510565b6102887f000000000000000000000000000000000000000000000000000000000000000081565b6101dd6102f5366004612522565b610b16565b6102b3610c84565b6101dd610cb4565b61032d610318366004612571565b606654600160ff9092169190911b9081161490565b6040519015158152602001610225565b606654604051908152602001610225565b6101dd61035c3660046124c8565b610cc8565b6101dd610f3e565b610288610377366004612403565b610f4f565b6102887f000000000000000000000000000000000000000000000000000000000000000081565b6033546001600160a01b0316610288565b6103bc610f7b565b6040516102259291906125cc565b6103d26110e7565b604051610225929190612669565b6102887f000000000000000000000000000000000000000000000000000000000000000081565b6101dd6104153660046126ca565b6111f0565b61042261130c565b60405161022591906126f4565b6101dd61043d366004612706565b6113fe565b6101dd610450366004612744565b6114aa565b6101dd610463366004612522565b611523565b6101dd6104763660046122d3565b611688565b6101dd61048936600461275f565b6116f5565b6104966118da565b60046104a181611934565b8382146104c15760405163512509d360e11b815260040160405180910390fd5b5f5b848110156105ca575f8686838181106104de576104de6127d3565b905060200201359050805f0361050757604051633d23e4d160e11b815260040160405180910390fd5b61053b8186868581811061051d5761051d6127d3565b90506020020160208101906105329190612744565b609c919061195f565b610558576040516324bf631b60e11b815260040160405180910390fd5b7f7a0a76d85b582b17996dd7371a407aa7a79b870db8539247fba315c7b6beff628186868581811061058c5761058c6127d3565b90506020020160208101906105a19190612744565b604080519283526001600160a01b0390911660208301520160405180910390a1506001016104c3565b505050505050565b6105da61197e565b60665481811681146105ff5760405163c61dca5d60e01b815260040160405180910390fd5b61060882611a21565b5050565b600161061781611934565b6106246020840184612744565b61062d81611a5e565b61064a5760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815284906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890610698908490600401612818565b602060405180830381865afa1580156106b3573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106d79190612826565b6106f457604051631fb1705560e21b815260040160405180910390fd5b8461072261070f61070a36849003840184612403565b611b0e565b5f90815260986020526040902054151590565b61073f57604051634d2baea960e11b815260040160405180910390fd5b6105ca61075136889003880188612403565b86611b71565b604080518082019091525f8082526020820152609a5f61077684611b0e565b815260208082019290925260409081015f208151808301909252546001600160a01b0381168252600160a01b900463ffffffff169181019190915292915050565b60026107c281611934565b6107cf6020840184612744565b6107d881611a5e565b6107f55760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815284906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890610843908490600401612818565b602060405180830381865afa15801561085e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906108829190612826565b61089f57604051631fb1705560e21b815260040160405180910390fd5b846108b561070f61070a36849003840184612403565b6108d257604051634d2baea960e11b815260040160405180910390fd5b6105ca6108e436889003880188612403565b6108f336889003880188612403565b611beb565b60605f609b5f61090785611b0e565b81526020019081526020015f2090505f61092082611c70565b90505f816001600160401b0381111561093b5761093b61234a565b604051908082528060200260200182016040528015610964578160200160208202803683370190505b5090505f805b838110156109c4575f61097d8683611c79565b905061098a609c82611c84565b156109bb57808484815181106109a2576109a26127d3565b6020908102919091010152826109b781612859565b9350505b5060010161096a565b508152949350505050565b6060817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316637cffe48c846040518263ffffffff1660e01b8152600401610a1e9190612818565b602060405180830381865afa158015610a39573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a5d9190612871565b610a6f61021336869003860186612403565b610a8161037736879003870187612403565b6001600160a01b03166341ee6d0e866040518263ffffffff1660e01b8152600401610aac9190612818565b5f60405180830381865afa158015610ac6573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610aed919081019061288f565b604051602001610b009493929190612921565b6040516020818303038152906040529050919050565b6003610b2181611934565b610b2e6020850185612744565b610b3781611a5e565b610b545760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815285906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890610ba2908490600401612818565b602060405180830381865afa158015610bbd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610be19190612826565b610bfe57604051631fb1705560e21b815260040160405180910390fd5b85610c1461070f61070a36849003840184612403565b610c3157604051634d2baea960e11b815260040160405180910390fd5b610c7b610c4336899003890189612403565b8787808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250611c8f92505050565b50505050505050565b6060610caf7f0000000000000000000000000000000000000000000000000000000000000000611d8b565b905090565b610cbc61197e565b610cc65f19611a21565b565b5f610cd281611934565b610cdf6020830183612744565b610ce881611a5e565b610d055760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815283906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890610d53908490600401612818565b602060405180830381865afa158015610d6e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610d929190612826565b610daf57604051631fb1705560e21b815260040160405180910390fd5b83610dc561070f61070a36849003840184612403565b610de257604051634d2baea960e11b815260040160405180910390fd5b5f610df561070a36889003880188612403565b5f818152609960205260409081902080546001600160a01b0319169055519091507fd7811913efd5d98fc7ea0d1fdd022b3d31987815360842d05b1d1cf55578d16a90610e43908890612818565b60405180910390a15f818152609a60205260409081902080546001600160c01b0319169055517f210a1118a869246162804e2a7f21ef808ebd93f4be7ed512014fe29a7a8be02e90610e96908890612818565b60405180910390a15f818152609b60205260408120908181610eb882826121fb565b505050507faf209f19ac00e8ccb4539e96d4141cdc96fea479d258d99910307c7365e6875986604051610eeb9190612818565b60405180910390a1610efe609782611dc8565b507f4ffdfdd59e9e1e3c301608788f78dd458e61cb8c045ca92b62a7b484c80824fb86604051610f2e9190612818565b60405180910390a1505050505050565b610f466118da565b610cc65f611dd3565b5f60995f610f5c84611b0e565b815260208101919091526040015f20546001600160a01b031692915050565b6060805f610f896097611c70565b90505f816001600160401b03811115610fa457610fa461234a565b604051908082528060200260200182016040528015610fe857816020015b604080518082019091525f8082526020820152815260200190600190039081610fc25790505b5090505f826001600160401b038111156110045761100461234a565b60405190808252806020026020018201604052801561103757816020015b60608152602001906001900390816110225790505b5090505f5b838110156110dc575f611050609783611c79565b90505f61108c82604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b9050808584815181106110a1576110a16127d3565b60200260200101819052506110b5816108f8565b8484815181106110c7576110c76127d3565b6020908102919091010152505060010161103c565b509094909350915050565b6060805f6110f5609c611e24565b90505f816001600160401b038111156111105761111061234a565b604051908082528060200260200182016040528015611139578160200160208202803683370190505b5090505f826001600160401b038111156111555761115561234a565b60405190808252806020026020018201604052801561117e578160200160208202803683370190505b5090505f5b838110156110dc575f80611198609c84611e2e565b91509150818584815181106111af576111af6127d3565b602002602001018181525050808484815181106111ce576111ce6127d3565b6001600160a01b03909216602092830291909101909101525050600101611183565b5f54610100900460ff161580801561120e57505f54600160ff909116105b806112275750303b15801561122757505f5460ff166001145b61128f5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff1916600117905580156112b0575f805461ff0019166101001790555b6112b983611dd3565b6112c282611a21565b8015611307575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b505050565b60605f6113196097611c70565b90505f816001600160401b038111156113345761133461234a565b60405190808252806020026020018201604052801561137857816020015b604080518082019091525f80825260208201528152602001906001900390816113525790505b5090505f5b828110156113f7575f611391609783611c79565b90505f6113cd82604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b9050808484815181106113e2576113e26127d3565b6020908102919091010152505060010161137d565b5092915050565b6114066118da565b600461141181611934565b5f5b828110156114a4575f84848381811061142e5761142e6127d3565b90506020020135905061144b81609c611e4b90919063ffffffff16565b6114685760405163b3f92ba160e01b815260040160405180910390fd5b6040518181527f6824d36084ecf2cd819b137cb5d837cc6e73afce1e0e348c9fdecaa81d0341e59060200160405180910390a150600101611413565b50505050565b6114b26118da565b6001600160a01b0381166115175760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401611286565b61152081611dd3565b50565b600361152e81611934565b61153b6020850185612744565b61154481611a5e565b6115615760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815285906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc758906115af908490600401612818565b602060405180830381865afa1580156115ca573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115ee9190612826565b61160b57604051631fb1705560e21b815260040160405180910390fd5b8561162161070f61070a36849003840184612403565b61163e57604051634d2baea960e11b815260040160405180910390fd5b610c7b61165036899003890189612403565b8787808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250611e5692505050565b611690611f41565b606654801982198116146116b75760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c9060200160405180910390a25050565b5f6116ff81611934565b61170c6020870187612744565b61171581611a5e565b6117325760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815287906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890611780908490600401612818565b602060405180830381865afa15801561179b573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117bf9190612826565b6117dc57604051631fb1705560e21b815260040160405180910390fd5b6117f96117f161070a368b90038b018b612403565b609790611ff2565b61181657604051631883461560e01b815260040160405180910390fd5b7f4fb6efec7dd60036ce3a7af8d5c48425019daa0fb61eb471a966a7ac2c6fa6a6886040516118459190612818565b60405180910390a161186561185f368a90038a018a612403565b88611b71565b611886611877368a90038a018a612403565b6108f336899003890189612403565b6118d0611898368a90038a018a612403565b8686808060200260200160405190810160405280939291908181526020018383602002808284375f92019190915250611c8f92505050565b5050505050505050565b6033546001600160a01b03163314610cc65760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401611286565b606654600160ff83161b908116036115205760405163840a48d560e01b815260040160405180910390fd5b5f61197484846001600160a01b038516611ffd565b90505b9392505050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa1580156119e0573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a049190612826565b610cc657604051631d77d47760e21b815260040160405180910390fd5b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af1158015611ae4573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b089190612826565b92915050565b5f815f0151826020015163ffffffff16604051602001611b5992919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b604051602081830303815290604052611b0890612979565b8060995f611b7e85611b0e565b81526020019081526020015f205f6101000a8154816001600160a01b0302191690836001600160a01b031602179055507f7f7ccafd92d20fdb39dee184a0dce002a9da420ed0def461f2a027abc9b3f6df8282604051611bdf92919061299c565b60405180910390a15050565b80609a5f611bf885611b0e565b815260208082019290925260409081015f2083518154949093015163ffffffff16600160a01b026001600160c01b03199094166001600160a01b0390931692909217929092179055517f3147846ee526009000671c20380b856a633345691300f82585f90034715cf0e290611bdf90849084906129c2565b5f611b08825490565b5f6119778383612019565b5f611977838361203f565b5f815111611cb057604051638631a07560e01b815260040160405180910390fd5b5f611cba83611b0e565b90505f5b82518110156114a4575f838281518110611cda57611cda6127d3565b60200260200101519050611cf881609c611c8490919063ffffffff16565b611d155760405163b3f92ba160e01b815260040160405180910390fd5b5f838152609b60205260409020611d2c9082611ff2565b611d49576040516396d81ac960e01b815260040160405180910390fd5b7f57a1fcb3d9cd447695c46f20944ba562d9547989dcddea0afb119115060c7f0b8582604051611d7a9291906129dd565b60405180910390a150600101611cbe565b60605f611d9783612056565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b5f611977838361207d565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b5f611b0882612160565b5f808080611e3c868661216a565b909450925050505b9250929050565b5f6119778383612193565b5f611e6083611b0e565b90505f5b8251811015611f0c575f838281518110611e8057611e806127d3565b60200260200101519050611ead81609b5f8681526020019081526020015f20611dc890919063ffffffff16565b611eca5760405163ab6cce0760e01b815260040160405180910390fd5b7f499955d838e6f0ca31e83adf81d191cfe6cd8fe252bf826c75c9a80ba077e25e8582604051611efb9291906129dd565b60405180910390a150600101611e64565b505f818152609b60205260408120611f2390611c70565b11611307576040516343629f7b60e01b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611f9d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611fc191906129f8565b6001600160a01b0316336001600160a01b031614610cc65760405163794821ff60e01b815260040160405180910390fd5b5f61197783836121af565b5f82815260028401602052604081208290556119748484611ff2565b5f825f01828154811061202e5761202e6127d3565b905f5260205f200154905092915050565b5f8181526001830160205260408120541515611977565b5f60ff8216601f811115611b0857604051632cd44ac360e21b815260040160405180910390fd5b5f8181526001830160205260408120548015612157575f61209f600183612a13565b85549091505f906120b290600190612a13565b9050818114612111575f865f0182815481106120d0576120d06127d3565b905f5260205f200154905080875f0184815481106120f0576120f06127d3565b5f918252602080832090910192909255918252600188019052604090208390555b855486908061212257612122612a26565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050611b08565b5f915050611b08565b5f611b0882611c70565b5f80806121778585611c79565b5f81815260029690960160205260409095205494959350505050565b5f81815260028301602052604081208190556119778383611dc8565b5f8181526001830160205260408120546121f457508154600181810184555f848152602080822090930184905584548482528286019093526040902091909155611b08565b505f611b08565b5080545f8255905f5260205f209081019061152091905b80821115612225575f8155600101612212565b5090565b5f5f83601f840112612239575f5ffd5b5081356001600160401b0381111561224f575f5ffd5b6020830191508360208260051b8501011115611e44575f5ffd5b5f5f5f5f6040858703121561227c575f5ffd5b84356001600160401b03811115612291575f5ffd5b61229d87828801612229565b90955093505060208501356001600160401b038111156122bb575f5ffd5b6122c787828801612229565b95989497509550505050565b5f602082840312156122e3575f5ffd5b5035919050565b5f604082840312156122fa575f5ffd5b50919050565b6001600160a01b0381168114611520575f5ffd5b5f5f60608385031215612325575f5ffd5b61232f84846122ea565b9150604083013561233f81612300565b809150509250929050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b03811182821017156123865761238661234a565b604052919050565b803563ffffffff811681146123a1575f5ffd5b919050565b5f604082840312156123b6575f5ffd5b604080519081016001600160401b03811182821017156123d8576123d861234a565b60405290508082356123e981612300565b81526123f76020840161238e565b60208201525092915050565b5f60408284031215612413575f5ffd5b61197783836123a6565b80516001600160a01b0316825260209081015163ffffffff16910152565b60408101611b08828461241d565b5f5f6080838503121561245a575f5ffd5b61246484846122ea565b915061247384604085016122ea565b90509250929050565b5f8151808452602084019350602083015f5b828110156124ac57815186526020958601959091019060010161248e565b5093949350505050565b602081525f611977602083018461247c565b5f604082840312156124d8575f5ffd5b61197783836122ea565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f61197760208301846124e2565b5f5f5f60608486031215612534575f5ffd5b61253e85856122ea565b925060408401356001600160401b03811115612558575f5ffd5b61256486828701612229565b9497909650939450505050565b5f60208284031215612581575f5ffd5b813560ff81168114611977575f5ffd5b5f8151808452602084019350602083015f5b828110156124ac576125b686835161241d565b60409590950194602091909101906001016125a3565b604081525f6125de6040830185612591565b828103602084015280845180835260208301915060208160051b840101602087015f5b8381101561265b57858303601f19018552815180518085526020918201918501905f5b81811015612642578351835260209384019390920191600101612624565b5050602096870196909450929092019150600101612601565b509098975050505050505050565b604081525f61267b604083018561247c565b82810360208401528084518083526020830191506020860192505f5b818110156126be5783516001600160a01b0316835260209384019390920191600101612697565b50909695505050505050565b5f5f604083850312156126db575f5ffd5b82356126e681612300565b946020939093013593505050565b602081525f6119776020830184612591565b5f5f60208385031215612717575f5ffd5b82356001600160401b0381111561272c575f5ffd5b61273885828601612229565b90969095509350505050565b5f60208284031215612754575f5ffd5b813561197781612300565b5f5f5f5f5f60c08688031215612773575f5ffd5b61277d87876122ea565b9450604086013561278d81612300565b935061279c87606088016122ea565b925060a08601356001600160401b038111156127b6575f5ffd5b6127c288828901612229565b969995985093965092949392505050565b634e487b7160e01b5f52603260045260245ffd5b80356127f281612300565b6001600160a01b0316825263ffffffff61280e6020830161238e565b1660208301525050565b60408101611b0882846127e7565b5f60208284031215612836575f5ffd5b81518015158114611977575f5ffd5b634e487b7160e01b5f52601160045260245ffd5b5f6001820161286a5761286a612845565b5060010190565b5f60208284031215612881575f5ffd5b815160038110611977575f5ffd5b5f6020828403121561289f575f5ffd5b81516001600160401b038111156128b4575f5ffd5b8201601f810184136128c4575f5ffd5b80516001600160401b038111156128dd576128dd61234a565b6128f0601f8201601f191660200161235e565b818152856020838501011115612904575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b61292b81866127e7565b5f6003851061294857634e487b7160e01b5f52602160045260245ffd5b84604083015261295b606083018561241d565b60c060a083015261296f60c08301846124e2565b9695505050505050565b805160208083015191908110156122fa575f1960209190910360031b1b16919050565b606081016129aa828561241d565b6001600160a01b039290921660409190910152919050565b608081016129d0828561241d565b611977604083018461241d565b606081016129eb828561241d565b8260408301529392505050565b5f60208284031215612a08575f5ffd5b815161197781612300565b81810381811115611b0857611b08612845565b634e487b7160e01b5f52603160045260245ffdfea264697066735822122079cbe8fbbc74928b40bff8981593ef67cf90bac6e471ca7182fc5733d7da90c264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xC6W_5`\xE0\x1C\x80clU\xA3\x7F\x11a\0\xFEW\x80c\xCA\x8A\xA7\xC7\x11a\0\x9EW\x80c\xF2\xFD\xE3\x8B\x11a\0nW\x80c\xF2\xFD\xE3\x8B\x14a\x04BW\x80c\xF3\xE9\xF5\xD4\x14a\x04UW\x80c\xFA\xBC\x1C\xBC\x14a\x04hW\x80c\xFEYm\xEE\x14a\x04{W__\xFD[\x80c\xCA\x8A\xA7\xC7\x14a\x03\xE0W\x80c\xCDm\xC6\x87\x14a\x04\x07W\x80c\xD0\x9B\x97\x8B\x14a\x04\x1AW\x80c\xDF\xBD\x9D\xFD\x14a\x04/W__\xFD[\x80c\x88o\x11\x95\x11a\0\xD9W\x80c\x88o\x11\x95\x14a\x03|W\x80c\x8D\xA5\xCB[\x14a\x03\xA3W\x80c\xBF\xDA;=\x14a\x03\xB4W\x80c\xC4\xBF\xFE+\x14a\x03\xCAW__\xFD[\x80clU\xA3\x7F\x14a\x03NW\x80cqP\x18\xA6\x14a\x03aW\x80cu\xE4\xB59\x14a\x03iW__\xFD[\x80cA\xEEm\x0E\x11a\x01iW\x80cT\xFDMP\x11a\x01DW\x80cT\xFDMP\x14a\x02\xFAW\x80cY\\jg\x14a\x03\x02W\x80cZ\xC8j\xB7\x14a\x03\nW\x80c\\\x97Z\xBB\x14a\x03=W__\xFD[\x80cA\xEEm\x0E\x14a\x02\xA0W\x80cFW\xE2j\x14a\x02\xC0W\x80cI\xBE}o\x14a\x02\xE7W__\xFD[\x80c!\xFA\x7F\xDC\x11a\x01\xA4W\x80c!\xFA\x7F\xDC\x14a\x02\x05W\x80c'~\x1Eb\x14a\x02.W\x80c<u\xFD\xDF\x14a\x02AW\x80c>\xC4\\~\x14a\x02aW__\xFD[\x80c\x04\xE9\x8B\xE3\x14a\x01\xCAW\x80c\x13d9\xDD\x14a\x01\xDFW\x80c\x1C\xA9\x14*\x14a\x01\xF2W[__\xFD[a\x01\xDDa\x01\xD86`\x04a\"iV[a\x04\x8EV[\0[a\x01\xDDa\x01\xED6`\x04a\"\xD3V[a\x05\xD2V[a\x01\xDDa\x02\x006`\x04a#\x14V[a\x06\x0CV[a\x02\x18a\x02\x136`\x04a$\x03V[a\x07WV[`@Qa\x02%\x91\x90a$;V[`@Q\x80\x91\x03\x90\xF3[a\x01\xDDa\x02<6`\x04a$IV[a\x07\xB7V[a\x02Ta\x02O6`\x04a$\x03V[a\x08\xF8V[`@Qa\x02%\x91\x90a$\xB6V[a\x02\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02%V[a\x02\xB3a\x02\xAE6`\x04a$\xC8V[a\t\xCFV[`@Qa\x02%\x91\x90a%\x10V[a\x02\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xDDa\x02\xF56`\x04a%\"V[a\x0B\x16V[a\x02\xB3a\x0C\x84V[a\x01\xDDa\x0C\xB4V[a\x03-a\x03\x186`\x04a%qV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x02%V[`fT`@Q\x90\x81R` \x01a\x02%V[a\x01\xDDa\x03\\6`\x04a$\xC8V[a\x0C\xC8V[a\x01\xDDa\x0F>V[a\x02\x88a\x03w6`\x04a$\x03V[a\x0FOV[a\x02\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\x88V[a\x03\xBCa\x0F{V[`@Qa\x02%\x92\x91\x90a%\xCCV[a\x03\xD2a\x10\xE7V[`@Qa\x02%\x92\x91\x90a&iV[a\x02\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xDDa\x04\x156`\x04a&\xCAV[a\x11\xF0V[a\x04\"a\x13\x0CV[`@Qa\x02%\x91\x90a&\xF4V[a\x01\xDDa\x04=6`\x04a'\x06V[a\x13\xFEV[a\x01\xDDa\x04P6`\x04a'DV[a\x14\xAAV[a\x01\xDDa\x04c6`\x04a%\"V[a\x15#V[a\x01\xDDa\x04v6`\x04a\"\xD3V[a\x16\x88V[a\x01\xDDa\x04\x896`\x04a'_V[a\x16\xF5V[a\x04\x96a\x18\xDAV[`\x04a\x04\xA1\x81a\x194V[\x83\x82\x14a\x04\xC1W`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x05\xCAW_\x86\x86\x83\x81\x81\x10a\x04\xDEWa\x04\xDEa'\xD3V[\x90P` \x02\x015\x90P\x80_\x03a\x05\x07W`@Qc=#\xE4\xD1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05;\x81\x86\x86\x85\x81\x81\x10a\x05\x1DWa\x05\x1Da'\xD3V[\x90P` \x02\x01` \x81\x01\x90a\x052\x91\x90a'DV[`\x9C\x91\x90a\x19_V[a\x05XW`@Qc$\xBFc\x1B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7Fz\nv\xD8[X+\x17\x99m\xD77\x1A@z\xA7\xA7\x9B\x87\r\xB8S\x92G\xFB\xA3\x15\xC7\xB6\xBE\xFFb\x81\x86\x86\x85\x81\x81\x10a\x05\x8CWa\x05\x8Ca'\xD3V[\x90P` \x02\x01` \x81\x01\x90a\x05\xA1\x91\x90a'DV[`@\x80Q\x92\x83R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1P`\x01\x01a\x04\xC3V[PPPPPPV[a\x05\xDAa\x19~V[`fT\x81\x81\x16\x81\x14a\x05\xFFW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06\x08\x82a\x1A!V[PPV[`\x01a\x06\x17\x81a\x194V[a\x06$` \x84\x01\x84a'DV[a\x06-\x81a\x1A^V[a\x06JW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x84\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x06\x98\x90\x84\x90`\x04\x01a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xB3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xD7\x91\x90a(&V[a\x06\xF4W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84a\x07\"a\x07\x0Fa\x07\n6\x84\x90\x03\x84\x01\x84a$\x03V[a\x1B\x0EV[_\x90\x81R`\x98` R`@\x90 T\x15\x15\x90V[a\x07?W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xCAa\x07Q6\x88\x90\x03\x88\x01\x88a$\x03V[\x86a\x1BqV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x9A_a\x07v\x84a\x1B\x0EV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q\x80\x83\x01\x90\x92RT`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x92\x91PPV[`\x02a\x07\xC2\x81a\x194V[a\x07\xCF` \x84\x01\x84a'DV[a\x07\xD8\x81a\x1A^V[a\x07\xF5W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x84\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x08C\x90\x84\x90`\x04\x01a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08^W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x82\x91\x90a(&V[a\x08\x9FW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84a\x08\xB5a\x07\x0Fa\x07\n6\x84\x90\x03\x84\x01\x84a$\x03V[a\x08\xD2W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xCAa\x08\xE46\x88\x90\x03\x88\x01\x88a$\x03V[a\x08\xF36\x88\x90\x03\x88\x01\x88a$\x03V[a\x1B\xEBV[``_`\x9B_a\t\x07\x85a\x1B\x0EV[\x81R` \x01\x90\x81R` \x01_ \x90P_a\t \x82a\x1CpV[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\t;Wa\t;a#JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\tdW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x83\x81\x10\x15a\t\xC4W_a\t}\x86\x83a\x1CyV[\x90Pa\t\x8A`\x9C\x82a\x1C\x84V[\x15a\t\xBBW\x80\x84\x84\x81Q\x81\x10a\t\xA2Wa\t\xA2a'\xD3V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x82a\t\xB7\x81a(YV[\x93PP[P`\x01\x01a\tjV[P\x81R\x94\x93PPPPV[``\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c|\xFF\xE4\x8C\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\x1E\x91\x90a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n]\x91\x90a(qV[a\noa\x02\x136\x86\x90\x03\x86\x01\x86a$\x03V[a\n\x81a\x03w6\x87\x90\x03\x87\x01\x87a$\x03V[`\x01`\x01`\xA0\x1B\x03\x16cA\xEEm\x0E\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\n\xAC\x91\x90a(\x18V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xC6W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\xED\x91\x90\x81\x01\x90a(\x8FV[`@Q` \x01a\x0B\0\x94\x93\x92\x91\x90a)!V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\x03a\x0B!\x81a\x194V[a\x0B.` \x85\x01\x85a'DV[a\x0B7\x81a\x1A^V[a\x0BTW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x85\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x0B\xA2\x90\x84\x90`\x04\x01a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xBDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xE1\x91\x90a(&V[a\x0B\xFEW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85a\x0C\x14a\x07\x0Fa\x07\n6\x84\x90\x03\x84\x01\x84a$\x03V[a\x0C1W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C{a\x0CC6\x89\x90\x03\x89\x01\x89a$\x03V[\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x1C\x8F\x92PPPV[PPPPPPPV[``a\x0C\xAF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x8BV[\x90P\x90V[a\x0C\xBCa\x19~V[a\x0C\xC6_\x19a\x1A!V[V[_a\x0C\xD2\x81a\x194V[a\x0C\xDF` \x83\x01\x83a'DV[a\x0C\xE8\x81a\x1A^V[a\r\x05W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x83\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\rS\x90\x84\x90`\x04\x01a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rnW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x92\x91\x90a(&V[a\r\xAFW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a\r\xC5a\x07\x0Fa\x07\n6\x84\x90\x03\x84\x01\x84a$\x03V[a\r\xE2W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\r\xF5a\x07\n6\x88\x90\x03\x88\x01\x88a$\x03V[_\x81\x81R`\x99` R`@\x90\x81\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UQ\x90\x91P\x7F\xD7\x81\x19\x13\xEF\xD5\xD9\x8F\xC7\xEA\r\x1F\xDD\x02+=1\x98x\x156\x08B\xD0[\x1D\x1C\xF5Ux\xD1j\x90a\x0EC\x90\x88\x90a(\x18V[`@Q\x80\x91\x03\x90\xA1_\x81\x81R`\x9A` R`@\x90\x81\x90 \x80T`\x01`\x01`\xC0\x1B\x03\x19\x16\x90UQ\x7F!\n\x11\x18\xA8i$ab\x80N*\x7F!\xEF\x80\x8E\xBD\x93\xF4\xBE~\xD5\x12\x01O\xE2\x9Az\x8B\xE0.\x90a\x0E\x96\x90\x88\x90a(\x18V[`@Q\x80\x91\x03\x90\xA1_\x81\x81R`\x9B` R`@\x81 \x90\x81\x81a\x0E\xB8\x82\x82a!\xFBV[PPPP\x7F\xAF \x9F\x19\xAC\0\xE8\xCC\xB4S\x9E\x96\xD4\x14\x1C\xDC\x96\xFE\xA4y\xD2X\xD9\x99\x100|se\xE6\x87Y\x86`@Qa\x0E\xEB\x91\x90a(\x18V[`@Q\x80\x91\x03\x90\xA1a\x0E\xFE`\x97\x82a\x1D\xC8V[P\x7FO\xFD\xFD\xD5\x9E\x9E\x1E<0\x16\x08x\x8Fx\xDDE\x8Ea\xCB\x8C\x04\\\xA9+b\xA7\xB4\x84\xC8\x08$\xFB\x86`@Qa\x0F.\x91\x90a(\x18V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[a\x0FFa\x18\xDAV[a\x0C\xC6_a\x1D\xD3V[_`\x99_a\x0F\\\x84a\x1B\x0EV[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[``\x80_a\x0F\x89`\x97a\x1CpV[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0F\xA4Wa\x0F\xA4a#JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\xE8W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0F\xC2W\x90P[P\x90P_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10\x04Wa\x10\x04a#JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x107W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x10\"W\x90P[P\x90P_[\x83\x81\x10\x15a\x10\xDCW_a\x10P`\x97\x83a\x1CyV[\x90P_a\x10\x8C\x82`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x90P\x80\x85\x84\x81Q\x81\x10a\x10\xA1Wa\x10\xA1a'\xD3V[` \x02` \x01\x01\x81\x90RPa\x10\xB5\x81a\x08\xF8V[\x84\x84\x81Q\x81\x10a\x10\xC7Wa\x10\xC7a'\xD3V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x10<V[P\x90\x94\x90\x93P\x91PPV[``\x80_a\x10\xF5`\x9Ca\x1E$V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x10Wa\x11\x10a#JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x119W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11UWa\x11Ua#JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11~W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x10\xDCW_\x80a\x11\x98`\x9C\x84a\x1E.V[\x91P\x91P\x81\x85\x84\x81Q\x81\x10a\x11\xAFWa\x11\xAFa'\xD3V[` \x02` \x01\x01\x81\x81RPP\x80\x84\x84\x81Q\x81\x10a\x11\xCEWa\x11\xCEa'\xD3V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPP`\x01\x01a\x11\x83V[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x12\x0EWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x12'WP0;\x15\x80\x15a\x12'WP_T`\xFF\x16`\x01\x14[a\x12\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x12\xB0W_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x12\xB9\x83a\x1D\xD3V[a\x12\xC2\x82a\x1A!V[\x80\x15a\x13\x07W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPV[``_a\x13\x19`\x97a\x1CpV[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x134Wa\x134a#JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13xW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13RW\x90P[P\x90P_[\x82\x81\x10\x15a\x13\xF7W_a\x13\x91`\x97\x83a\x1CyV[\x90P_a\x13\xCD\x82`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x90P\x80\x84\x84\x81Q\x81\x10a\x13\xE2Wa\x13\xE2a'\xD3V[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x13}V[P\x92\x91PPV[a\x14\x06a\x18\xDAV[`\x04a\x14\x11\x81a\x194V[_[\x82\x81\x10\x15a\x14\xA4W_\x84\x84\x83\x81\x81\x10a\x14.Wa\x14.a'\xD3V[\x90P` \x02\x015\x90Pa\x14K\x81`\x9Ca\x1EK\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x14hW`@Qc\xB3\xF9+\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x81\x81R\x7Fh$\xD3`\x84\xEC\xF2\xCD\x81\x9B\x13|\xB5\xD87\xCCns\xAF\xCE\x1E\x0E4\x8C\x9F\xDE\xCA\xA8\x1D\x03A\xE5\x90` \x01`@Q\x80\x91\x03\x90\xA1P`\x01\x01a\x14\x13V[PPPPV[a\x14\xB2a\x18\xDAV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x15\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x12\x86V[a\x15 \x81a\x1D\xD3V[PV[`\x03a\x15.\x81a\x194V[a\x15;` \x85\x01\x85a'DV[a\x15D\x81a\x1A^V[a\x15aW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x85\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x15\xAF\x90\x84\x90`\x04\x01a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xCAW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xEE\x91\x90a(&V[a\x16\x0BW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85a\x16!a\x07\x0Fa\x07\n6\x84\x90\x03\x84\x01\x84a$\x03V[a\x16>W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C{a\x16P6\x89\x90\x03\x89\x01\x89a$\x03V[\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x1EV\x92PPPV[a\x16\x90a\x1FAV[`fT\x80\x19\x82\x19\x81\x16\x14a\x16\xB7W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[_a\x16\xFF\x81a\x194V[a\x17\x0C` \x87\x01\x87a'DV[a\x17\x15\x81a\x1A^V[a\x172W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x87\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x17\x80\x90\x84\x90`\x04\x01a(\x18V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17\x9BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xBF\x91\x90a(&V[a\x17\xDCW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x17\xF9a\x17\xF1a\x07\n6\x8B\x90\x03\x8B\x01\x8Ba$\x03V[`\x97\x90a\x1F\xF2V[a\x18\x16W`@Qc\x18\x83F\x15`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7FO\xB6\xEF\xEC}\xD6\x006\xCE:z\xF8\xD5\xC4\x84%\x01\x9D\xAA\x0F\xB6\x1E\xB4q\xA9f\xA7\xAC,o\xA6\xA6\x88`@Qa\x18E\x91\x90a(\x18V[`@Q\x80\x91\x03\x90\xA1a\x18ea\x18_6\x8A\x90\x03\x8A\x01\x8Aa$\x03V[\x88a\x1BqV[a\x18\x86a\x18w6\x8A\x90\x03\x8A\x01\x8Aa$\x03V[a\x08\xF36\x89\x90\x03\x89\x01\x89a$\x03V[a\x18\xD0a\x18\x986\x8A\x90\x03\x8A\x01\x8Aa$\x03V[\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847_\x92\x01\x91\x90\x91RPa\x1C\x8F\x92PPPV[PPPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x12\x86V[`fT`\x01`\xFF\x83\x16\x1B\x90\x81\x16\x03a\x15 W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x19t\x84\x84`\x01`\x01`\xA0\x1B\x03\x85\x16a\x1F\xFDV[\x90P[\x93\x92PPPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xE0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x04\x91\x90a(&V[a\x0C\xC6W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1A\xE4W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x08\x91\x90a(&V[\x92\x91PPV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a\x1BY\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1B\x08\x90a)yV[\x80`\x99_a\x1B~\x85a\x1B\x0EV[\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x7F\x7F|\xCA\xFD\x92\xD2\x0F\xDB9\xDE\xE1\x84\xA0\xDC\xE0\x02\xA9\xDAB\x0E\xD0\xDE\xF4a\xF2\xA0'\xAB\xC9\xB3\xF6\xDF\x82\x82`@Qa\x1B\xDF\x92\x91\x90a)\x9CV[`@Q\x80\x91\x03\x90\xA1PPV[\x80`\x9A_a\x1B\xF8\x85a\x1B\x0EV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x83Q\x81T\x94\x90\x93\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x92\x90\x92\x17\x90UQ\x7F1G\x84n\xE5&\0\x90\0g\x1C 8\x0B\x85jc3Ei\x13\0\xF8%\x85\xF9\x004q\\\xF0\xE2\x90a\x1B\xDF\x90\x84\x90\x84\x90a)\xC2V[_a\x1B\x08\x82T\x90V[_a\x19w\x83\x83a \x19V[_a\x19w\x83\x83a ?V[_\x81Q\x11a\x1C\xB0W`@Qc\x861\xA0u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x1C\xBA\x83a\x1B\x0EV[\x90P_[\x82Q\x81\x10\x15a\x14\xA4W_\x83\x82\x81Q\x81\x10a\x1C\xDAWa\x1C\xDAa'\xD3V[` \x02` \x01\x01Q\x90Pa\x1C\xF8\x81`\x9Ca\x1C\x84\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1D\x15W`@Qc\xB3\xF9+\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x83\x81R`\x9B` R`@\x90 a\x1D,\x90\x82a\x1F\xF2V[a\x1DIW`@Qc\x96\xD8\x1A\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7FW\xA1\xFC\xB3\xD9\xCDDv\x95\xC4o \x94K\xA5b\xD9Ty\x89\xDC\xDD\xEA\n\xFB\x11\x91\x15\x06\x0C\x7F\x0B\x85\x82`@Qa\x1Dz\x92\x91\x90a)\xDDV[`@Q\x80\x91\x03\x90\xA1P`\x01\x01a\x1C\xBEV[``_a\x1D\x97\x83a VV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[_a\x19w\x83\x83a }V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[_a\x1B\x08\x82a!`V[_\x80\x80\x80a\x1E<\x86\x86a!jV[\x90\x94P\x92PPP[\x92P\x92\x90PV[_a\x19w\x83\x83a!\x93V[_a\x1E`\x83a\x1B\x0EV[\x90P_[\x82Q\x81\x10\x15a\x1F\x0CW_\x83\x82\x81Q\x81\x10a\x1E\x80Wa\x1E\x80a'\xD3V[` \x02` \x01\x01Q\x90Pa\x1E\xAD\x81`\x9B_\x86\x81R` \x01\x90\x81R` \x01_ a\x1D\xC8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1E\xCAW`@Qc\xABl\xCE\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7FI\x99U\xD88\xE6\xF0\xCA1\xE8:\xDF\x81\xD1\x91\xCF\xE6\xCD\x8F\xE2R\xBF\x82lu\xC9\xA8\x0B\xA0w\xE2^\x85\x82`@Qa\x1E\xFB\x92\x91\x90a)\xDDV[`@Q\x80\x91\x03\x90\xA1P`\x01\x01a\x1EdV[P_\x81\x81R`\x9B` R`@\x81 a\x1F#\x90a\x1CpV[\x11a\x13\x07W`@QcCb\x9F{`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\x9DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xC1\x91\x90a)\xF8V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xC6W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x19w\x83\x83a!\xAFV[_\x82\x81R`\x02\x84\x01` R`@\x81 \x82\x90Ua\x19t\x84\x84a\x1F\xF2V[_\x82_\x01\x82\x81T\x81\x10a .Wa .a'\xD3V[\x90_R` _ \x01T\x90P\x92\x91PPV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x19wV[_`\xFF\x82\x16`\x1F\x81\x11\x15a\x1B\x08W`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a!WW_a \x9F`\x01\x83a*\x13V[\x85T\x90\x91P_\x90a \xB2\x90`\x01\x90a*\x13V[\x90P\x81\x81\x14a!\x11W_\x86_\x01\x82\x81T\x81\x10a \xD0Wa \xD0a'\xD3V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a \xF0Wa \xF0a'\xD3V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a!\"Wa!\"a*&V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x1B\x08V[_\x91PPa\x1B\x08V[_a\x1B\x08\x82a\x1CpV[_\x80\x80a!w\x85\x85a\x1CyV[_\x81\x81R`\x02\x96\x90\x96\x01` R`@\x90\x95 T\x94\x95\x93PPPPV[_\x81\x81R`\x02\x83\x01` R`@\x81 \x81\x90Ua\x19w\x83\x83a\x1D\xC8V[_\x81\x81R`\x01\x83\x01` R`@\x81 Ta!\xF4WP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x1B\x08V[P_a\x1B\x08V[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a\x15 \x91\x90[\x80\x82\x11\x15a\"%W_\x81U`\x01\x01a\"\x12V[P\x90V[__\x83`\x1F\x84\x01\x12a\"9W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\"OW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1EDW__\xFD[____`@\x85\x87\x03\x12\x15a\"|W__\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x91W__\xFD[a\"\x9D\x87\x82\x88\x01a\")V[\x90\x95P\x93PP` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xBBW__\xFD[a\"\xC7\x87\x82\x88\x01a\")V[\x95\x98\x94\x97P\x95PPPPV[_` \x82\x84\x03\x12\x15a\"\xE3W__\xFD[P5\x91\x90PV[_`@\x82\x84\x03\x12\x15a\"\xFAW__\xFD[P\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15 W__\xFD[__``\x83\x85\x03\x12\x15a#%W__\xFD[a#/\x84\x84a\"\xEAV[\x91P`@\x83\x015a#?\x81a#\0V[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a#\x86Wa#\x86a#JV[`@R\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a#\xA1W__\xFD[\x91\x90PV[_`@\x82\x84\x03\x12\x15a#\xB6W__\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a#\xD8Wa#\xD8a#JV[`@R\x90P\x80\x825a#\xE9\x81a#\0V[\x81Ra#\xF7` \x84\x01a#\x8EV[` \x82\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15a$\x13W__\xFD[a\x19w\x83\x83a#\xA6V[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`@\x81\x01a\x1B\x08\x82\x84a$\x1DV[__`\x80\x83\x85\x03\x12\x15a$ZW__\xFD[a$d\x84\x84a\"\xEAV[\x91Pa$s\x84`@\x85\x01a\"\xEAV[\x90P\x92P\x92\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a$\xACW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a$\x8EV[P\x93\x94\x93PPPPV[` \x81R_a\x19w` \x83\x01\x84a$|V[_`@\x82\x84\x03\x12\x15a$\xD8W__\xFD[a\x19w\x83\x83a\"\xEAV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x19w` \x83\x01\x84a$\xE2V[___``\x84\x86\x03\x12\x15a%4W__\xFD[a%>\x85\x85a\"\xEAV[\x92P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a%XW__\xFD[a%d\x86\x82\x87\x01a\")V[\x94\x97\x90\x96P\x93\x94PPPPV[_` \x82\x84\x03\x12\x15a%\x81W__\xFD[\x815`\xFF\x81\x16\x81\x14a\x19wW__\xFD[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a$\xACWa%\xB6\x86\x83Qa$\x1DV[`@\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01a%\xA3V[`@\x81R_a%\xDE`@\x83\x01\x85a%\x91V[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x81`\x05\x1B\x84\x01\x01` \x87\x01_[\x83\x81\x10\x15a&[W\x85\x83\x03`\x1F\x19\x01\x85R\x81Q\x80Q\x80\x85R` \x91\x82\x01\x91\x85\x01\x90_[\x81\x81\x10\x15a&BW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a&$V[PP` \x96\x87\x01\x96\x90\x94P\x92\x90\x92\x01\x91P`\x01\x01a&\x01V[P\x90\x98\x97PPPPPPPPV[`@\x81R_a&{`@\x83\x01\x85a$|V[\x82\x81\x03` \x84\x01R\x80\x84Q\x80\x83R` \x83\x01\x91P` \x86\x01\x92P_[\x81\x81\x10\x15a&\xBEW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a&\x97V[P\x90\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15a&\xDBW__\xFD[\x825a&\xE6\x81a#\0V[\x94` \x93\x90\x93\x015\x93PPPV[` \x81R_a\x19w` \x83\x01\x84a%\x91V[__` \x83\x85\x03\x12\x15a'\x17W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a',W__\xFD[a'8\x85\x82\x86\x01a\")V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15a'TW__\xFD[\x815a\x19w\x81a#\0V[_____`\xC0\x86\x88\x03\x12\x15a'sW__\xFD[a'}\x87\x87a\"\xEAV[\x94P`@\x86\x015a'\x8D\x81a#\0V[\x93Pa'\x9C\x87``\x88\x01a\"\xEAV[\x92P`\xA0\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xB6W__\xFD[a'\xC2\x88\x82\x89\x01a\")V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x805a'\xF2\x81a#\0V[`\x01`\x01`\xA0\x1B\x03\x16\x82Rc\xFF\xFF\xFF\xFFa(\x0E` \x83\x01a#\x8EV[\x16` \x83\x01RPPV[`@\x81\x01a\x1B\x08\x82\x84a'\xE7V[_` \x82\x84\x03\x12\x15a(6W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x19wW__\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01a(jWa(ja(EV[P`\x01\x01\x90V[_` \x82\x84\x03\x12\x15a(\x81W__\xFD[\x81Q`\x03\x81\x10a\x19wW__\xFD[_` \x82\x84\x03\x12\x15a(\x9FW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xB4W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a(\xC4W__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xDDWa(\xDDa#JV[a(\xF0`\x1F\x82\x01`\x1F\x19\x16` \x01a#^V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a)\x04W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[a)+\x81\x86a'\xE7V[_`\x03\x85\x10a)HWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x84`@\x83\x01Ra)[``\x83\x01\x85a$\x1DV[`\xC0`\xA0\x83\x01Ra)o`\xC0\x83\x01\x84a$\xE2V[\x96\x95PPPPPPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\"\xFAW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[``\x81\x01a)\xAA\x82\x85a$\x1DV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`@\x91\x90\x91\x01R\x91\x90PV[`\x80\x81\x01a)\xD0\x82\x85a$\x1DV[a\x19w`@\x83\x01\x84a$\x1DV[``\x81\x01a)\xEB\x82\x85a$\x1DV[\x82`@\x83\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a*\x08W__\xFD[\x81Qa\x19w\x81a#\0V[\x81\x81\x03\x81\x81\x11\x15a\x1B\x08Wa\x1B\x08a(EV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 y\xCB\xE8\xFB\xBCt\x92\x8B@\xBF\xF8\x98\x15\x93\xEFg\xCF\x90\xBA\xC6\xE4q\xCAq\x82\xFCW3\xD7\xDA\x90\xC2dsolcC\0\x08\x1B\x003",
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
    /**Custom error with signature `RequireAtLeastOneTransportDestination()` and selector `0x43629f7b`.
    ```solidity
    error RequireAtLeastOneTransportDestination();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RequireAtLeastOneTransportDestination;
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
        impl ::core::convert::From<RequireAtLeastOneTransportDestination> for UnderlyingRustTuple<'_> {
            fn from(value: RequireAtLeastOneTransportDestination) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RequireAtLeastOneTransportDestination {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RequireAtLeastOneTransportDestination {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RequireAtLeastOneTransportDestination()";
            const SELECTOR: [u8; 4] = [67u8, 98u8, 159u8, 123u8];
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
    /**Custom error with signature `TransportDestinationAlreadyAdded()` and selector `0x96d81ac9`.
    ```solidity
    error TransportDestinationAlreadyAdded();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TransportDestinationAlreadyAdded;
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
        impl ::core::convert::From<TransportDestinationAlreadyAdded> for UnderlyingRustTuple<'_> {
            fn from(value: TransportDestinationAlreadyAdded) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TransportDestinationAlreadyAdded {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TransportDestinationAlreadyAdded {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TransportDestinationAlreadyAdded()";
            const SELECTOR: [u8; 4] = [150u8, 216u8, 26u8, 201u8];
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
    /**Custom error with signature `TransportDestinationNotFound()` and selector `0xab6cce07`.
    ```solidity
    error TransportDestinationNotFound();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TransportDestinationNotFound;
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
        impl ::core::convert::From<TransportDestinationNotFound> for UnderlyingRustTuple<'_> {
            fn from(value: TransportDestinationNotFound) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TransportDestinationNotFound {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TransportDestinationNotFound {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TransportDestinationNotFound()";
            const SELECTOR: [u8; 4] = [171u8, 108u8, 206u8, 7u8];
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
    /**Event with signature `TransportDestinationChainAdded((address,uint32),uint256)` and selector `0x57a1fcb3d9cd447695c46f20944ba562d9547989dcddea0afb119115060c7f0b`.
    ```solidity
    event TransportDestinationChainAdded(OperatorSet operatorSet, uint256 chainID);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransportDestinationChainAdded {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for TransportDestinationChainAdded {
            type DataTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<256>);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "TransportDestinationChainAdded((address,uint32),uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    87u8, 161u8, 252u8, 179u8, 217u8, 205u8, 68u8, 118u8, 149u8, 196u8, 111u8,
                    32u8, 148u8, 75u8, 165u8, 98u8, 217u8, 84u8, 121u8, 137u8, 220u8, 221u8, 234u8,
                    10u8, 251u8, 17u8, 145u8, 21u8, 6u8, 12u8, 127u8, 11u8,
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
                    chainID: data.1,
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
        impl alloy_sol_types::private::IntoLogData for TransportDestinationChainAdded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransportDestinationChainAdded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransportDestinationChainAdded) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TransportDestinationChainRemoved((address,uint32),uint256)` and selector `0x499955d838e6f0ca31e83adf81d191cfe6cd8fe252bf826c75c9a80ba077e25e`.
    ```solidity
    event TransportDestinationChainRemoved(OperatorSet operatorSet, uint256 chainID);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransportDestinationChainRemoved {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for TransportDestinationChainRemoved {
            type DataTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<256>);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "TransportDestinationChainRemoved((address,uint32),uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    73u8, 153u8, 85u8, 216u8, 56u8, 230u8, 240u8, 202u8, 49u8, 232u8, 58u8, 223u8,
                    129u8, 209u8, 145u8, 207u8, 230u8, 205u8, 143u8, 226u8, 82u8, 191u8, 130u8,
                    108u8, 117u8, 201u8, 168u8, 11u8, 160u8, 119u8, 226u8, 94u8,
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
                    chainID: data.1,
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
        impl alloy_sol_types::private::IntoLogData for TransportDestinationChainRemoved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransportDestinationChainRemoved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransportDestinationChainRemoved) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Event with signature `TransportDestinationsRemoved((address,uint32))` and selector `0xaf209f19ac00e8ccb4539e96d4141cdc96fea479d258d99910307c7365e68759`.
    ```solidity
    event TransportDestinationsRemoved(OperatorSet operatorSet);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TransportDestinationsRemoved {
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
        impl alloy_sol_types::SolEvent for TransportDestinationsRemoved {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TransportDestinationsRemoved((address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    175u8, 32u8, 159u8, 25u8, 172u8, 0u8, 232u8, 204u8, 180u8, 83u8, 158u8, 150u8,
                    212u8, 20u8, 28u8, 220u8, 150u8, 254u8, 164u8, 121u8, 210u8, 88u8, 217u8,
                    153u8, 16u8, 48u8, 124u8, 115u8, 101u8, 230u8, 135u8, 89u8,
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
        impl alloy_sol_types::private::IntoLogData for TransportDestinationsRemoved {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TransportDestinationsRemoved> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TransportDestinationsRemoved) -> alloy_sol_types::private::LogData {
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
    constructor(address _allocationManager, address _keyRegistrar, address _permissionController, address _pauserRegistry, string _version);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _keyRegistrar: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _permissionController: alloy::sol_types::private::Address,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                        value._allocationManager,
                        value._keyRegistrar,
                        value._permissionController,
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
                        _allocationManager: tuple.0,
                        _keyRegistrar: tuple.1,
                        _permissionController: tuple.2,
                        _pauserRegistry: tuple.3,
                        _version: tuple.4,
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
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._keyRegistrar,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._permissionController,
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
    /**Function with signature `addTransportDestinations((address,uint32),uint256[])` and selector `0x49be7d6f`.
    ```solidity
    function addTransportDestinations(OperatorSet memory operatorSet, uint256[] memory chainIDs) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addTransportDestinationsCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub chainIDs:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
    }
    ///Container type for the return parameters of the [`addTransportDestinations((address,uint32),uint256[])`](addTransportDestinationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addTransportDestinationsReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<addTransportDestinationsCall> for UnderlyingRustTuple<'_> {
                fn from(value: addTransportDestinationsCall) -> Self {
                    (value.operatorSet, value.chainIDs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addTransportDestinationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        chainIDs: tuple.1,
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
            impl ::core::convert::From<addTransportDestinationsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addTransportDestinationsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addTransportDestinationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl addTransportDestinationsReturn {
            fn _tokenize(
                &self,
            ) -> <addTransportDestinationsCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addTransportDestinationsCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addTransportDestinationsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addTransportDestinations((address,uint32),uint256[])";
            const SELECTOR: [u8; 4] = [73u8, 190u8, 125u8, 111u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainIDs),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                addTransportDestinationsReturn::_tokenize(ret)
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
    /**Function with signature `createGenerationReservation((address,uint32),address,(address,uint32),uint256[])` and selector `0xfe596dee`.
    ```solidity
    function createGenerationReservation(OperatorSet memory operatorSet, address operatorTableCalculator, ICrossChainRegistryTypes.OperatorSetConfig memory config, uint256[] memory chainIDs) external;
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
        #[allow(missing_docs)]
        pub chainIDs:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
    }
    ///Container type for the return parameters of the [`createGenerationReservation((address,uint32),address,(address,uint32),uint256[])`](createGenerationReservationCall) function.
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                <ICrossChainRegistryTypes::OperatorSetConfig as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createGenerationReservationCall> for UnderlyingRustTuple<'_> {
                fn from(value: createGenerationReservationCall) -> Self {
                    (
                        value.operatorSet,
                        value.operatorTableCalculator,
                        value.config,
                        value.chainIDs,
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
                        chainIDs: tuple.3,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createGenerationReservationReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "createGenerationReservation((address,uint32),address,(address,uint32),uint256[])";
            const SELECTOR: [u8; 4] = [254u8, 89u8, 109u8, 238u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainIDs),
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
    /**Function with signature `getActiveTransportReservations()` and selector `0xbfda3b3d`.
    ```solidity
    function getActiveTransportReservations() external view returns (OperatorSet[] memory, uint256[][] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveTransportReservationsCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getActiveTransportReservations()`](getActiveTransportReservationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveTransportReservationsReturn {
        #[allow(missing_docs)]
        pub _0:
            alloy::sol_types::private::Vec<<OperatorSet as alloy::sol_types::SolType>::RustType>,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getActiveTransportReservationsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveTransportReservationsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveTransportReservationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <OperatorSet as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        alloy::sol_types::private::primitives::aliases::U256,
                    >,
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
            impl ::core::convert::From<getActiveTransportReservationsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getActiveTransportReservationsReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getActiveTransportReservationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        impl getActiveTransportReservationsReturn {
            fn _tokenize(
                &self,
            ) -> <getActiveTransportReservationsCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                (
                    <alloy::sol_types::sol_data::Array<
                        OperatorSet,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<256>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self._1),
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActiveTransportReservationsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getActiveTransportReservationsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getActiveTransportReservations()";
            const SELECTOR: [u8; 4] = [191u8, 218u8, 59u8, 61u8];
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
                getActiveTransportReservationsReturn::_tokenize(ret)
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
    /**Function with signature `getTransportDestinations((address,uint32))` and selector `0x3c75fddf`.
    ```solidity
    function getTransportDestinations(OperatorSet memory operatorSet) external view returns (uint256[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTransportDestinationsCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`getTransportDestinations((address,uint32))`](getTransportDestinationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTransportDestinationsReturn {
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
            impl ::core::convert::From<getTransportDestinationsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTransportDestinationsCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTransportDestinationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
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
            impl ::core::convert::From<getTransportDestinationsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTransportDestinationsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTransportDestinationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTransportDestinationsCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTransportDestinations((address,uint32))";
            const SELECTOR: [u8; 4] = [60u8, 117u8, 253u8, 223u8];
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::SolType>::tokenize(ret),)
            }
            #[inline]
            fn abi_decode_returns(data: &[u8]) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(data).map(
                    |r| {
                        let r: getTransportDestinationsReturn = r.into();
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
                    let r: getTransportDestinationsReturn = r.into();
                    r._0
                })
            }
        }
    };
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    /**Function with signature `initialize(address,uint256)` and selector `0xcd6dc687`.
    ```solidity
    function initialize(address initialOwner, uint256 initialPausedStatus) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub initialOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`initialize(address,uint256)`](initializeCall) function.
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.initialOwner, value.initialPausedStatus)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        initialPausedStatus: tuple.1,
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
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,uint256)";
            const SELECTOR: [u8; 4] = [205u8, 109u8, 198u8, 135u8];
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
                        &self.initialOwner,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.initialPausedStatus,
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
    /**Function with signature `keyRegistrar()` and selector `0x3ec45c7e`.
    ```solidity
    function keyRegistrar() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keyRegistrarCall;
    #[derive(serde::Serialize, serde::Deserialize, Default, Debug, PartialEq, Eq, Hash)]
    ///Container type for the return parameters of the [`keyRegistrar()`](keyRegistrarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct keyRegistrarReturn {
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
            impl ::core::convert::From<keyRegistrarCall> for UnderlyingRustTuple<'_> {
                fn from(value: keyRegistrarCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keyRegistrarCall {
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
            impl ::core::convert::From<keyRegistrarReturn> for UnderlyingRustTuple<'_> {
                fn from(value: keyRegistrarReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for keyRegistrarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for keyRegistrarCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = alloy::sol_types::private::Address;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "keyRegistrar()";
            const SELECTOR: [u8; 4] = [62u8, 196u8, 92u8, 126u8];
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
                        let r: keyRegistrarReturn = r.into();
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
                    let r: keyRegistrarReturn = r.into();
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
    /**Function with signature `removeTransportDestinations((address,uint32),uint256[])` and selector `0xf3e9f5d4`.
    ```solidity
    function removeTransportDestinations(OperatorSet memory operatorSet, uint256[] memory chainIDs) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeTransportDestinationsCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub chainIDs:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
    }
    ///Container type for the return parameters of the [`removeTransportDestinations((address,uint32),uint256[])`](removeTransportDestinationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeTransportDestinationsReturn {}
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<removeTransportDestinationsCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeTransportDestinationsCall) -> Self {
                    (value.operatorSet, value.chainIDs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeTransportDestinationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        chainIDs: tuple.1,
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
            impl ::core::convert::From<removeTransportDestinationsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeTransportDestinationsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeTransportDestinationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        impl removeTransportDestinationsReturn {
            fn _tokenize(
                &self,
            ) -> <removeTransportDestinationsCall as alloy_sol_types::SolCall>::ReturnToken<'_>
            {
                ()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeTransportDestinationsCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeTransportDestinationsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "removeTransportDestinations((address,uint32),uint256[])";
            const SELECTOR: [u8; 4] = [243u8, 233u8, 245u8, 212u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.chainIDs),
                )
            }
            #[inline]
            fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                removeTransportDestinationsReturn::_tokenize(ret)
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
    ///Container for all the [`CrossChainRegistry`](self) function calls.
    #[derive(serde::Serialize, serde::Deserialize)]
    pub enum CrossChainRegistryCalls {
        #[allow(missing_docs)]
        addChainIDsToWhitelist(addChainIDsToWhitelistCall),
        #[allow(missing_docs)]
        addTransportDestinations(addTransportDestinationsCall),
        #[allow(missing_docs)]
        allocationManager(allocationManagerCall),
        #[allow(missing_docs)]
        calculateOperatorTableBytes(calculateOperatorTableBytesCall),
        #[allow(missing_docs)]
        createGenerationReservation(createGenerationReservationCall),
        #[allow(missing_docs)]
        getActiveGenerationReservations(getActiveGenerationReservationsCall),
        #[allow(missing_docs)]
        getActiveTransportReservations(getActiveTransportReservationsCall),
        #[allow(missing_docs)]
        getOperatorSetConfig(getOperatorSetConfigCall),
        #[allow(missing_docs)]
        getOperatorTableCalculator(getOperatorTableCalculatorCall),
        #[allow(missing_docs)]
        getSupportedChains(getSupportedChainsCall),
        #[allow(missing_docs)]
        getTransportDestinations(getTransportDestinationsCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        keyRegistrar(keyRegistrarCall),
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
        permissionController(permissionControllerCall),
        #[allow(missing_docs)]
        removeChainIDsFromWhitelist(removeChainIDsFromWhitelistCall),
        #[allow(missing_docs)]
        removeGenerationReservation(removeGenerationReservationCall),
        #[allow(missing_docs)]
        removeTransportDestinations(removeTransportDestinationsCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        setOperatorSetConfig(setOperatorSetConfigCall),
        #[allow(missing_docs)]
        setOperatorTableCalculator(setOperatorTableCalculatorCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
        version(versionCall),
    }
    #[automatically_derived]
    impl CrossChainRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 233u8, 139u8, 227u8],
            [19u8, 100u8, 57u8, 221u8],
            [28u8, 169u8, 20u8, 42u8],
            [33u8, 250u8, 127u8, 220u8],
            [39u8, 126u8, 30u8, 98u8],
            [60u8, 117u8, 253u8, 223u8],
            [62u8, 196u8, 92u8, 126u8],
            [65u8, 238u8, 109u8, 14u8],
            [70u8, 87u8, 226u8, 106u8],
            [73u8, 190u8, 125u8, 111u8],
            [84u8, 253u8, 77u8, 80u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [108u8, 85u8, 163u8, 127u8],
            [113u8, 80u8, 24u8, 166u8],
            [117u8, 228u8, 181u8, 57u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [191u8, 218u8, 59u8, 61u8],
            [196u8, 191u8, 254u8, 43u8],
            [202u8, 138u8, 167u8, 199u8],
            [205u8, 109u8, 198u8, 135u8],
            [208u8, 155u8, 151u8, 139u8],
            [223u8, 189u8, 157u8, 253u8],
            [242u8, 253u8, 227u8, 139u8],
            [243u8, 233u8, 245u8, 212u8],
            [250u8, 188u8, 28u8, 188u8],
            [254u8, 89u8, 109u8, 238u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for CrossChainRegistryCalls {
        const NAME: &'static str = "CrossChainRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addChainIDsToWhitelist(_) => {
                    <addChainIDsToWhitelistCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addTransportDestinations(_) => {
                    <addTransportDestinationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateOperatorTableBytes(_) => {
                    <calculateOperatorTableBytesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createGenerationReservation(_) => {
                    <createGenerationReservationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActiveGenerationReservations(_) => {
                    <getActiveGenerationReservationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getActiveTransportReservations(_) => {
                    <getActiveTransportReservationsCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::getTransportDestinations(_) => {
                    <getTransportDestinationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::keyRegistrar(_) => <keyRegistrarCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::permissionController(_) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeChainIDsFromWhitelist(_) => {
                    <removeChainIDsFromWhitelistCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeGenerationReservation(_) => {
                    <removeGenerationReservationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeTransportDestinations(_) => {
                    <removeTransportDestinationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperatorSetConfig(_) => {
                    <setOperatorSetConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperatorTableCalculator(_) => {
                    <setOperatorTableCalculatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
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
                -> alloy_sol_types::Result<CrossChainRegistryCalls>] = &[
                {
                    fn addChainIDsToWhitelist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <addChainIDsToWhitelistCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(CrossChainRegistryCalls::addChainIDsToWhitelist)
                    }
                    addChainIDsToWhitelist
                },
                {
                    fn pause(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::pause)
                    }
                    pause
                },
                {
                    fn setOperatorTableCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <setOperatorTableCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CrossChainRegistryCalls::setOperatorTableCalculator)
                    }
                    setOperatorTableCalculator
                },
                {
                    fn getOperatorSetConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getOperatorSetConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::getOperatorSetConfig)
                    }
                    getOperatorSetConfig
                },
                {
                    fn setOperatorSetConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <setOperatorSetConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::setOperatorSetConfig)
                    }
                    setOperatorSetConfig
                },
                {
                    fn getTransportDestinations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getTransportDestinationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(CrossChainRegistryCalls::getTransportDestinations)
                    }
                    getTransportDestinations
                },
                {
                    fn keyRegistrar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <keyRegistrarCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::keyRegistrar)
                    }
                    keyRegistrar
                },
                {
                    fn calculateOperatorTableBytes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <calculateOperatorTableBytesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CrossChainRegistryCalls::calculateOperatorTableBytes)
                    }
                    calculateOperatorTableBytes
                },
                {
                    fn permissionController(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <permissionControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::permissionController)
                    }
                    permissionController
                },
                {
                    fn addTransportDestinations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <addTransportDestinationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(CrossChainRegistryCalls::addTransportDestinations)
                    }
                    addTransportDestinations
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::version)
                    }
                    version
                },
                {
                    fn pauseAll(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn removeGenerationReservation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <removeGenerationReservationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CrossChainRegistryCalls::removeGenerationReservation)
                    }
                    removeGenerationReservation
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn getOperatorTableCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getOperatorTableCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CrossChainRegistryCalls::getOperatorTableCalculator)
                    }
                    getOperatorTableCalculator
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::owner)
                    }
                    owner
                },
                {
                    fn getActiveTransportReservations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getActiveTransportReservationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CrossChainRegistryCalls::getActiveTransportReservations)
                    }
                    getActiveTransportReservations
                },
                {
                    fn getSupportedChains(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getSupportedChainsCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::getSupportedChains)
                    }
                    getSupportedChains
                },
                {
                    fn allocationManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initialize(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getActiveGenerationReservations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getActiveGenerationReservationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                CrossChainRegistryCalls::getActiveGenerationReservations,
                            )
                    }
                    getActiveGenerationReservations
                },
                {
                    fn removeChainIDsFromWhitelist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <removeChainIDsFromWhitelistCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CrossChainRegistryCalls::removeChainIDsFromWhitelist)
                    }
                    removeChainIDsFromWhitelist
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn removeTransportDestinations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <removeTransportDestinationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CrossChainRegistryCalls::removeTransportDestinations)
                    }
                    removeTransportDestinations
                },
                {
                    fn unpause(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::unpause)
                    }
                    unpause
                },
                {
                    fn createGenerationReservation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <createGenerationReservationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CrossChainRegistryCalls::createGenerationReservation)
                    }
                    createGenerationReservation
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
                CrossChainRegistryCalls,
            >] = &[
                {
                    fn addChainIDsToWhitelist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <addChainIDsToWhitelistCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::addChainIDsToWhitelist)
                    }
                    addChainIDsToWhitelist
                },
                {
                    fn pause(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryCalls::pause)
                    }
                    pause
                },
                {
                    fn setOperatorTableCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <setOperatorTableCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::setOperatorTableCalculator)
                    }
                    setOperatorTableCalculator
                },
                {
                    fn getOperatorSetConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getOperatorSetConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::getOperatorSetConfig)
                    }
                    getOperatorSetConfig
                },
                {
                    fn setOperatorSetConfig(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <setOperatorSetConfigCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::setOperatorSetConfig)
                    }
                    setOperatorSetConfig
                },
                {
                    fn getTransportDestinations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getTransportDestinationsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::getTransportDestinations)
                    }
                    getTransportDestinations
                },
                {
                    fn keyRegistrar(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <keyRegistrarCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(CrossChainRegistryCalls::keyRegistrar)
                    }
                    keyRegistrar
                },
                {
                    fn calculateOperatorTableBytes(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <calculateOperatorTableBytesCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::calculateOperatorTableBytes)
                    }
                    calculateOperatorTableBytes
                },
                {
                    fn permissionController(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <permissionControllerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::permissionController)
                    }
                    permissionController
                },
                {
                    fn addTransportDestinations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <addTransportDestinationsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::addTransportDestinations)
                    }
                    addTransportDestinations
                },
                {
                    fn version(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <versionCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryCalls::version)
                    }
                    version
                },
                {
                    fn pauseAll(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn removeGenerationReservation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <removeGenerationReservationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::removeGenerationReservation)
                    }
                    removeGenerationReservation
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn getOperatorTableCalculator(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getOperatorTableCalculatorCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::getOperatorTableCalculator)
                    }
                    getOperatorTableCalculator
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                            data,
                        )
                        .map(CrossChainRegistryCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryCalls::owner)
                    }
                    owner
                },
                {
                    fn getActiveTransportReservations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getActiveTransportReservationsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::getActiveTransportReservations)
                    }
                    getActiveTransportReservations
                },
                {
                    fn getSupportedChains(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getSupportedChainsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::getSupportedChains)
                    }
                    getSupportedChains
                },
                {
                    fn allocationManager(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initialize(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getActiveGenerationReservations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getActiveGenerationReservationsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                CrossChainRegistryCalls::getActiveGenerationReservations,
                            )
                    }
                    getActiveGenerationReservations
                },
                {
                    fn removeChainIDsFromWhitelist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <removeChainIDsFromWhitelistCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::removeChainIDsFromWhitelist)
                    }
                    removeChainIDsFromWhitelist
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn removeTransportDestinations(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <removeTransportDestinationsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::removeTransportDestinations)
                    }
                    removeTransportDestinations
                },
                {
                    fn unpause(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryCalls::unpause)
                    }
                    unpause
                },
                {
                    fn createGenerationReservation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <createGenerationReservationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::createGenerationReservation)
                    }
                    createGenerationReservation
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
                Self::addTransportDestinations(inner) => {
                    <addTransportDestinationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getActiveGenerationReservations(inner) => {
                    <getActiveGenerationReservationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getActiveTransportReservations(inner) => {
                    <getActiveTransportReservationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getTransportDestinations(inner) => {
                    <getTransportDestinationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::keyRegistrar(inner) => {
                    <keyRegistrarCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::removeTransportDestinations(inner) => {
                    <removeTransportDestinationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::addTransportDestinations(inner) => {
                    <addTransportDestinationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getActiveGenerationReservations(inner) => {
                    <getActiveGenerationReservationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getActiveTransportReservations(inner) => {
                    <getActiveTransportReservationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getTransportDestinations(inner) => {
                    <getTransportDestinationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::keyRegistrar(inner) => {
                    <keyRegistrarCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::removeTransportDestinations(inner) => {
                    <removeTransportDestinationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::version(inner) => {
                    <versionCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`CrossChainRegistry`](self) custom errors.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum CrossChainRegistryErrors {
        #[allow(missing_docs)]
        ArrayLengthMismatch(ArrayLengthMismatch),
        #[allow(missing_docs)]
        ChainIDAlreadyWhitelisted(ChainIDAlreadyWhitelisted),
        #[allow(missing_docs)]
        ChainIDNotWhitelisted(ChainIDNotWhitelisted),
        #[allow(missing_docs)]
        CurrentlyPaused(CurrentlyPaused),
        #[allow(missing_docs)]
        EmptyChainIDsArray(EmptyChainIDsArray),
        #[allow(missing_docs)]
        GenerationReservationAlreadyExists(GenerationReservationAlreadyExists),
        #[allow(missing_docs)]
        GenerationReservationDoesNotExist(GenerationReservationDoesNotExist),
        #[allow(missing_docs)]
        InputAddressZero(InputAddressZero),
        #[allow(missing_docs)]
        InvalidChainId(InvalidChainId),
        #[allow(missing_docs)]
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        #[allow(missing_docs)]
        InvalidOperatorSet(InvalidOperatorSet),
        #[allow(missing_docs)]
        InvalidPermissions(InvalidPermissions),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        OnlyPauser(OnlyPauser),
        #[allow(missing_docs)]
        OnlyUnpauser(OnlyUnpauser),
        #[allow(missing_docs)]
        RequireAtLeastOneTransportDestination(RequireAtLeastOneTransportDestination),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
        #[allow(missing_docs)]
        TransportDestinationAlreadyAdded(TransportDestinationAlreadyAdded),
        #[allow(missing_docs)]
        TransportDestinationNotFound(TransportDestinationNotFound),
    }
    #[automatically_derived]
    impl CrossChainRegistryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [24u8, 131u8, 70u8, 21u8],
            [48u8, 90u8, 39u8, 169u8],
            [67u8, 98u8, 159u8, 123u8],
            [73u8, 126u8, 198u8, 54u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [122u8, 71u8, 201u8, 162u8],
            [126u8, 197u8, 193u8, 84u8],
            [132u8, 10u8, 72u8, 213u8],
            [134u8, 49u8, 160u8, 117u8],
            [147u8, 45u8, 148u8, 247u8],
            [150u8, 216u8, 26u8, 201u8],
            [154u8, 87u8, 93u8, 82u8],
            [162u8, 74u8, 19u8, 166u8],
            [171u8, 108u8, 206u8, 7u8],
            [179u8, 81u8, 43u8, 12u8],
            [179u8, 249u8, 43u8, 161u8],
            [198u8, 29u8, 202u8, 93u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for CrossChainRegistryErrors {
        const NAME: &'static str = "CrossChainRegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 19usize;
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
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
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
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidChainId(_) => <InvalidChainId as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOperatorSet(_) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPermissions(_) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
                Self::RequireAtLeastOneTransportDestination(_) => {
                    <RequireAtLeastOneTransportDestination as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StringTooLong(_) => <StringTooLong as alloy_sol_types::SolError>::SELECTOR,
                Self::TransportDestinationAlreadyAdded(_) => {
                    <TransportDestinationAlreadyAdded as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TransportDestinationNotFound(_) => {
                    <TransportDestinationNotFound as alloy_sol_types::SolError>::SELECTOR
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
                -> alloy_sol_types::Result<CrossChainRegistryErrors>] = &[
                {
                    fn GenerationReservationAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <GenerationReservationAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                CrossChainRegistryErrors::GenerationReservationAlreadyExists,
                            )
                    }
                    GenerationReservationAlreadyExists
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn RequireAtLeastOneTransportDestination(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <RequireAtLeastOneTransportDestination as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                CrossChainRegistryErrors::RequireAtLeastOneTransportDestination,
                            )
                    }
                    RequireAtLeastOneTransportDestination
                },
                {
                    fn ChainIDAlreadyWhitelisted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <ChainIDAlreadyWhitelisted as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(CrossChainRegistryErrors::ChainIDAlreadyWhitelisted)
                    }
                    ChainIDAlreadyWhitelisted
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn InvalidChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidChainId as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::InvalidChainId)
                    }
                    InvalidChainId
                },
                {
                    fn InvalidOperatorSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidOperatorSet as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::InvalidOperatorSet)
                    }
                    InvalidOperatorSet
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn EmptyChainIDsArray(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <EmptyChainIDsArray as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::EmptyChainIDsArray)
                    }
                    EmptyChainIDsArray
                },
                {
                    fn InvalidPermissions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidPermissions as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::InvalidPermissions)
                    }
                    InvalidPermissions
                },
                {
                    fn TransportDestinationAlreadyAdded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <TransportDestinationAlreadyAdded as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                CrossChainRegistryErrors::TransportDestinationAlreadyAdded,
                            )
                    }
                    TransportDestinationAlreadyAdded
                },
                {
                    fn GenerationReservationDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <GenerationReservationDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                            )
                            .map(
                                CrossChainRegistryErrors::GenerationReservationDoesNotExist,
                            )
                    }
                    GenerationReservationDoesNotExist
                },
                {
                    fn ArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::ArrayLengthMismatch)
                    }
                    ArrayLengthMismatch
                },
                {
                    fn TransportDestinationNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <TransportDestinationNotFound as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(CrossChainRegistryErrors::TransportDestinationNotFound)
                    }
                    TransportDestinationNotFound
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn ChainIDNotWhitelisted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <ChainIDNotWhitelisted as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::ChainIDNotWhitelisted)
                    }
                    ChainIDNotWhitelisted
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
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
                CrossChainRegistryErrors,
            >] = &[
                {
                    fn GenerationReservationAlreadyExists(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <GenerationReservationAlreadyExists as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                CrossChainRegistryErrors::GenerationReservationAlreadyExists,
                            )
                    }
                    GenerationReservationAlreadyExists
                },
                {
                    fn StringTooLong(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <StringTooLong as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryErrors::StringTooLong)
                    }
                    StringTooLong
                },
                {
                    fn RequireAtLeastOneTransportDestination(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <RequireAtLeastOneTransportDestination as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                CrossChainRegistryErrors::RequireAtLeastOneTransportDestination,
                            )
                    }
                    RequireAtLeastOneTransportDestination
                },
                {
                    fn ChainIDAlreadyWhitelisted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <ChainIDAlreadyWhitelisted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryErrors::ChainIDAlreadyWhitelisted)
                    }
                    ChainIDAlreadyWhitelisted
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(CrossChainRegistryErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn InvalidChainId(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidChainId as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryErrors::InvalidChainId)
                    }
                    InvalidChainId
                },
                {
                    fn InvalidOperatorSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidOperatorSet as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(CrossChainRegistryErrors::InvalidOperatorSet)
                    }
                    InvalidOperatorSet
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(CrossChainRegistryErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn EmptyChainIDsArray(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <EmptyChainIDsArray as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(CrossChainRegistryErrors::EmptyChainIDsArray)
                    }
                    EmptyChainIDsArray
                },
                {
                    fn InvalidPermissions(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidPermissions as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(CrossChainRegistryErrors::InvalidPermissions)
                    }
                    InvalidPermissions
                },
                {
                    fn TransportDestinationAlreadyAdded(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <TransportDestinationAlreadyAdded as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                CrossChainRegistryErrors::TransportDestinationAlreadyAdded,
                            )
                    }
                    TransportDestinationAlreadyAdded
                },
                {
                    fn GenerationReservationDoesNotExist(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <GenerationReservationDoesNotExist as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                CrossChainRegistryErrors::GenerationReservationDoesNotExist,
                            )
                    }
                    GenerationReservationDoesNotExist
                },
                {
                    fn ArrayLengthMismatch(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <ArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(CrossChainRegistryErrors::ArrayLengthMismatch)
                    }
                    ArrayLengthMismatch
                },
                {
                    fn TransportDestinationNotFound(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <TransportDestinationNotFound as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryErrors::TransportDestinationNotFound)
                    }
                    TransportDestinationNotFound
                },
                {
                    fn InvalidShortString(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidShortString as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(CrossChainRegistryErrors::InvalidShortString)
                    }
                    InvalidShortString
                },
                {
                    fn ChainIDNotWhitelisted(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <ChainIDNotWhitelisted as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryErrors::ChainIDNotWhitelisted)
                    }
                    ChainIDNotWhitelisted
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
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
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidChainId(inner) => {
                    <InvalidChainId as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidOperatorSet(inner) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidPermissions(inner) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::RequireAtLeastOneTransportDestination(inner) => {
                    <RequireAtLeastOneTransportDestination as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::TransportDestinationAlreadyAdded(inner) => {
                    <TransportDestinationAlreadyAdded as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TransportDestinationNotFound(inner) => {
                    <TransportDestinationNotFound as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
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
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidPermissions(inner) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RequireAtLeastOneTransportDestination(inner) => {
                    <RequireAtLeastOneTransportDestination as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::TransportDestinationAlreadyAdded(inner) => {
                    <TransportDestinationAlreadyAdded as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TransportDestinationNotFound(inner) => {
                    <TransportDestinationNotFound as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`CrossChainRegistry`](self) events.
    #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Hash)]
    pub enum CrossChainRegistryEvents {
        #[allow(missing_docs)]
        ChainIDAddedToWhitelist(ChainIDAddedToWhitelist),
        #[allow(missing_docs)]
        ChainIDRemovedFromWhitelist(ChainIDRemovedFromWhitelist),
        #[allow(missing_docs)]
        GenerationReservationCreated(GenerationReservationCreated),
        #[allow(missing_docs)]
        GenerationReservationRemoved(GenerationReservationRemoved),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        OperatorSetConfigRemoved(OperatorSetConfigRemoved),
        #[allow(missing_docs)]
        OperatorSetConfigSet(OperatorSetConfigSet),
        #[allow(missing_docs)]
        OperatorTableCalculatorRemoved(OperatorTableCalculatorRemoved),
        #[allow(missing_docs)]
        OperatorTableCalculatorSet(OperatorTableCalculatorSet),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        TransportDestinationChainAdded(TransportDestinationChainAdded),
        #[allow(missing_docs)]
        TransportDestinationChainRemoved(TransportDestinationChainRemoved),
        #[allow(missing_docs)]
        TransportDestinationsRemoved(TransportDestinationsRemoved),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl CrossChainRegistryEvents {
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
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                73u8, 153u8, 85u8, 216u8, 56u8, 230u8, 240u8, 202u8, 49u8, 232u8, 58u8, 223u8,
                129u8, 209u8, 145u8, 207u8, 230u8, 205u8, 143u8, 226u8, 82u8, 191u8, 130u8, 108u8,
                117u8, 201u8, 168u8, 11u8, 160u8, 119u8, 226u8, 94u8,
            ],
            [
                79u8, 182u8, 239u8, 236u8, 125u8, 214u8, 0u8, 54u8, 206u8, 58u8, 122u8, 248u8,
                213u8, 196u8, 132u8, 37u8, 1u8, 157u8, 170u8, 15u8, 182u8, 30u8, 180u8, 113u8,
                169u8, 102u8, 167u8, 172u8, 44u8, 111u8, 166u8, 166u8,
            ],
            [
                79u8, 253u8, 253u8, 213u8, 158u8, 158u8, 30u8, 60u8, 48u8, 22u8, 8u8, 120u8, 143u8,
                120u8, 221u8, 69u8, 142u8, 97u8, 203u8, 140u8, 4u8, 92u8, 169u8, 43u8, 98u8, 167u8,
                180u8, 132u8, 200u8, 8u8, 36u8, 251u8,
            ],
            [
                87u8, 161u8, 252u8, 179u8, 217u8, 205u8, 68u8, 118u8, 149u8, 196u8, 111u8, 32u8,
                148u8, 75u8, 165u8, 98u8, 217u8, 84u8, 121u8, 137u8, 220u8, 221u8, 234u8, 10u8,
                251u8, 17u8, 145u8, 21u8, 6u8, 12u8, 127u8, 11u8,
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
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                127u8, 124u8, 202u8, 253u8, 146u8, 210u8, 15u8, 219u8, 57u8, 222u8, 225u8, 132u8,
                160u8, 220u8, 224u8, 2u8, 169u8, 218u8, 66u8, 14u8, 208u8, 222u8, 244u8, 97u8,
                242u8, 160u8, 39u8, 171u8, 201u8, 179u8, 246u8, 223u8,
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
                175u8, 32u8, 159u8, 25u8, 172u8, 0u8, 232u8, 204u8, 180u8, 83u8, 158u8, 150u8,
                212u8, 20u8, 28u8, 220u8, 150u8, 254u8, 164u8, 121u8, 210u8, 88u8, 217u8, 153u8,
                16u8, 48u8, 124u8, 115u8, 101u8, 230u8, 135u8, 89u8,
            ],
            [
                215u8, 129u8, 25u8, 19u8, 239u8, 213u8, 217u8, 143u8, 199u8, 234u8, 13u8, 31u8,
                221u8, 2u8, 43u8, 61u8, 49u8, 152u8, 120u8, 21u8, 54u8, 8u8, 66u8, 208u8, 91u8,
                29u8, 28u8, 245u8, 85u8, 120u8, 209u8, 106u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for CrossChainRegistryEvents {
        const NAME: &'static str = "CrossChainRegistryEvents";
        const COUNT: usize = 15usize;
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
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Initialized)
                }
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
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<Paused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Paused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Paused)
                }
                Some(
                    <TransportDestinationChainAdded as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <TransportDestinationChainAdded as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data,
                )
                .map(Self::TransportDestinationChainAdded),
                Some(
                    <TransportDestinationChainRemoved as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TransportDestinationChainRemoved as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::TransportDestinationChainRemoved)
                }
                Some(
                    <TransportDestinationsRemoved as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <TransportDestinationsRemoved as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data,
                )
                .map(Self::TransportDestinationsRemoved),
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(topics, data)
                        .map(Self::Unpaused)
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
    impl alloy_sol_types::private::IntoLogData for CrossChainRegistryEvents {
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
                Self::Initialized(inner) => {
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
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::TransportDestinationChainAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransportDestinationChainRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TransportDestinationsRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
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
                Self::Initialized(inner) => {
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
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::TransportDestinationChainAdded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransportDestinationChainRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TransportDestinationsRemoved(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`CrossChainRegistry`](self) contract instance.

    See the [wrapper's documentation](`CrossChainRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        P: alloy_contract::private::Provider<N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> CrossChainRegistryInstance<P, N> {
        CrossChainRegistryInstance::<P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

    Returns a new instance of the contract, if the deployment was successful.

    For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>(
        provider: P,
        _allocationManager: alloy::sol_types::private::Address,
        _keyRegistrar: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _version: alloy::sol_types::private::String,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<CrossChainRegistryInstance<P, N>>>
    {
        CrossChainRegistryInstance::<P, N>::deploy(
            provider,
            _allocationManager,
            _keyRegistrar,
            _permissionController,
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
        _allocationManager: alloy::sol_types::private::Address,
        _keyRegistrar: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _version: alloy::sol_types::private::String,
    ) -> alloy_contract::RawCallBuilder<P, N> {
        CrossChainRegistryInstance::<P, N>::deploy_builder(
            provider,
            _allocationManager,
            _keyRegistrar,
            _permissionController,
            _pauserRegistry,
            _version,
        )
    }
    /**A [`CrossChainRegistry`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`CrossChainRegistry`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct CrossChainRegistryInstance<P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network: ::core::marker::PhantomData<N>,
    }
    #[automatically_derived]
    impl<P, N> ::core::fmt::Debug for CrossChainRegistryInstance<P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("CrossChainRegistryInstance")
                .field(&self.address)
                .finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        CrossChainRegistryInstance<P, N>
    {
        /**Creates a new wrapper around an on-chain [`CrossChainRegistry`](self) contract instance.

        See the [wrapper's documentation](`CrossChainRegistryInstance`) for more details.*/
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
            _allocationManager: alloy::sol_types::private::Address,
            _keyRegistrar: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::Result<CrossChainRegistryInstance<P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _allocationManager,
                _keyRegistrar,
                _permissionController,
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
            _allocationManager: alloy::sol_types::private::Address,
            _keyRegistrar: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _version: alloy::sol_types::private::String,
        ) -> alloy_contract::RawCallBuilder<P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _allocationManager,
                        _keyRegistrar,
                        _permissionController,
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
    impl<P: ::core::clone::Clone, N> CrossChainRegistryInstance<&P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> CrossChainRegistryInstance<P, N> {
            CrossChainRegistryInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        CrossChainRegistryInstance<P, N>
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
        ///Creates a new call builder for the [`addTransportDestinations`] function.
        pub fn addTransportDestinations(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            chainIDs: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, addTransportDestinationsCall, N> {
            self.call_builder(&addTransportDestinationsCall {
                operatorSet,
                chainIDs,
            })
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall)
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
            chainIDs: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, createGenerationReservationCall, N> {
            self.call_builder(&createGenerationReservationCall {
                operatorSet,
                operatorTableCalculator,
                config,
                chainIDs,
            })
        }
        ///Creates a new call builder for the [`getActiveGenerationReservations`] function.
        pub fn getActiveGenerationReservations(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getActiveGenerationReservationsCall, N> {
            self.call_builder(&getActiveGenerationReservationsCall)
        }
        ///Creates a new call builder for the [`getActiveTransportReservations`] function.
        pub fn getActiveTransportReservations(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, getActiveTransportReservationsCall, N> {
            self.call_builder(&getActiveTransportReservationsCall)
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
        ///Creates a new call builder for the [`getTransportDestinations`] function.
        pub fn getTransportDestinations(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<&P, getTransportDestinationsCall, N> {
            self.call_builder(&getTransportDestinationsCall { operatorSet })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                initialPausedStatus,
            })
        }
        ///Creates a new call builder for the [`keyRegistrar`] function.
        pub fn keyRegistrar(&self) -> alloy_contract::SolCallBuilder<&P, keyRegistrarCall, N> {
            self.call_builder(&keyRegistrarCall)
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
        ///Creates a new call builder for the [`permissionController`] function.
        pub fn permissionController(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, permissionControllerCall, N> {
            self.call_builder(&permissionControllerCall)
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
        ///Creates a new call builder for the [`removeTransportDestinations`] function.
        pub fn removeTransportDestinations(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            chainIDs: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<&P, removeTransportDestinationsCall, N> {
            self.call_builder(&removeTransportDestinationsCall {
                operatorSet,
                chainIDs,
            })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<&P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall)
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
        ///Creates a new call builder for the [`version`] function.
        pub fn version(&self) -> alloy_contract::SolCallBuilder<&P, versionCall, N> {
            self.call_builder(&versionCall)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<P: alloy_contract::private::Provider<N>, N: alloy_contract::private::Network>
        CrossChainRegistryInstance<P, N>
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
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<&P, Initialized, N> {
            self.event_filter::<Initialized>()
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
        ///Creates a new event filter for the [`TransportDestinationChainAdded`] event.
        pub fn TransportDestinationChainAdded_filter(
            &self,
        ) -> alloy_contract::Event<&P, TransportDestinationChainAdded, N> {
            self.event_filter::<TransportDestinationChainAdded>()
        }
        ///Creates a new event filter for the [`TransportDestinationChainRemoved`] event.
        pub fn TransportDestinationChainRemoved_filter(
            &self,
        ) -> alloy_contract::Event<&P, TransportDestinationChainRemoved, N> {
            self.event_filter::<TransportDestinationChainRemoved>()
        }
        ///Creates a new event filter for the [`TransportDestinationsRemoved`] event.
        pub fn TransportDestinationsRemoved_filter(
            &self,
        ) -> alloy_contract::Event<&P, TransportDestinationsRemoved, N> {
            self.event_filter::<TransportDestinationsRemoved>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<&P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
