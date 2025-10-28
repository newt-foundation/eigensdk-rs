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
    error InvalidEndIndex();
    error InvalidNewPausedStatus();
    error InvalidOperatorSet();
    error InvalidPermissions();
    error InvalidRange();
    error InvalidShortString();
    error InvalidStalenessPeriod();
    error InvalidTableUpdateCadence();
    error KeyTypeNotSet();
    error OnlyPauser();
    error OnlyUnpauser();
    error StringTooLong(string str);

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
    event TableUpdateCadenceSet(uint32 tableUpdateCadence);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _allocationManager, address _keyRegistrar, address _permissionController, address _pauserRegistry, string _version);

    function addChainIDsToWhitelist(uint256[] memory chainIDs, address[] memory operatorTableUpdaters) external;
    function allocationManager() external view returns (address);
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
    function initialize(address initialOwner, uint32 initialTableUpdateCadence, uint256 initialPausedStatus) external;
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
    function renounceOwnership() external;
    function setOperatorSetConfig(OperatorSet memory operatorSet, ICrossChainRegistryTypes.OperatorSetConfig memory config) external;
    function setOperatorTableCalculator(OperatorSet memory operatorSet, address operatorTableCalculator) external;
    function setTableUpdateCadence(uint32 tableUpdateCadence) external;
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
    "name": "initialize",
    "inputs": [
      {
        "name": "initialOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "initialTableUpdateCadence",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "InvalidEndIndex",
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
    "name": "InvalidRange",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidShortString",
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
    ///0x610120604052348015610010575f5ffd5b5060405161281438038061281483398101604081905261002f916101c1565b80838686856001600160a01b03811661005b576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b0390811660805291821660a052811660c0521660e05261008181610098565b610100525061008e6100de565b5050505050610319565b5f5f829050601f815111156100cb578260405163305a27a960e01b81526004016100c291906102be565b60405180910390fd5b80516100d6826102f3565b179392505050565b5f54610100900460ff16156101455760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b60648201526084016100c2565b5f5460ff90811614610194575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146101aa575f5ffd5b50565b634e487b7160e01b5f52604160045260245ffd5b5f5f5f5f5f60a086880312156101d5575f5ffd5b85516101e081610196565b60208701519095506101f181610196565b604087015190945061020281610196565b606087015190935061021381610196565b60808701519092506001600160401b0381111561022e575f5ffd5b8601601f8101881361023e575f5ffd5b80516001600160401b03811115610257576102576101ad565b604051601f8201601f19908116603f011681016001600160401b0381118282101715610285576102856101ad565b6040528181528282016020018a101561029c575f5ffd5b8160208401602083015e5f602083830101528093505050509295509295909350565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b80516020808301519190811015610313575f198160200360031b1b821691505b50919050565b60805160a05160c05160e0516101005161247f6103955f395f610b7201525f81816102e601526117d701525f818161028701528181610a29015261112201525f81816103ee015281816107920152818161092201528181610c05015261107901525f81816103800152818161170b0152611a9b015261247f5ff3fe608060405234801561000f575f5ffd5b50600436106101d1575f3560e01c8063715018a6116100fe578063ca8aa7c71161009e578063d9a6729e1161006e578063d9a6729e1461044b578063dfbd9dfd1461045e578063f2fde38b14610471578063fabc1cbc14610484575f5ffd5b8063ca8aa7c7146103e9578063d09b978b14610410578063d504491114610425578063d6db9e2514610438575f5ffd5b80638da5cb5b116100d95780638da5cb5b146103a2578063ac505f4b146103b3578063b186a60e146103cb578063c4bffe2b146103d3575f5ffd5b8063715018a61461036057806375e4b53914610368578063886f11951461037b575f5ffd5b80633ec45c7e11610174578063595c6a6711610144578063595c6a67146103105780635ac86ab7146103185780635c975abb1461033b5780636c55a37f1461034d575f5ffd5b80633ec45c7e1461028257806341ee6d0e146102c15780634657e26a146102e157806354fd4d5014610308575f5ffd5b80631ca9142a116101af5780631ca9142a1461021057806321fa7fdc14610223578063277e1e621461024c57806336b200de1461025f575f5ffd5b806304e98be3146101d55780630f19aaef146101ea578063136439dd146101fd575b5f5ffd5b6101e86101e3366004611d72565b610497565b005b6101e86101f8366004611e0a565b6105db565b6101e861020b366004611e46565b610701565b6101e861021e366004611e73565b61073b565b610236610231366004611f4c565b61086b565b6040516102439190611f84565b60405180910390f35b6101e861025a366004611f92565b6108cb565b61027261026d366004611f4c565b610a09565b6040519015158152602001610243565b6102a97f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610243565b6102d46102cf366004611fc5565b610a24565b604051610243919061200d565b6102a97f000000000000000000000000000000000000000000000000000000000000000081565b6102d4610b6b565b6101e8610b9b565b61027261032636600461201f565b606654600160ff9092169190911b9081161490565b6066545b604051908152602001610243565b6101e861035b366004611fc5565b610baf565b6101e8610dd2565b6102a9610376366004611f4c565b610de3565b6102a97f000000000000000000000000000000000000000000000000000000000000000081565b6033546001600160a01b03166102a9565b609e5460405163ffffffff9091168152602001610243565b61033f610e0f565b6103db610e1a565b60405161024392919061203f565b6102a97f000000000000000000000000000000000000000000000000000000000000000081565b610418610f30565b60405161024391906120c8565b6101e8610433366004612115565b611023565b6101e8610446366004612159565b611269565b610418610459366004612172565b61127d565b6101e861046c366004612192565b6113c7565b6101e861047f3660046121d1565b61146d565b6101e8610492366004611e46565b6114e3565b61049f611550565b60036104aa816115aa565b8382146104ca5760405163512509d360e11b815260040160405180910390fd5b5f5b848110156105d3575f8686838181106104e7576104e76121ec565b905060200201359050805f0361051057604051633d23e4d160e11b815260040160405180910390fd5b61054481868685818110610526576105266121ec565b905060200201602081019061053b91906121d1565b609b91906115d5565b610561576040516324bf631b60e11b815260040160405180910390fd5b7f7a0a76d85b582b17996dd7371a407aa7a79b870db8539247fba315c7b6beff6281868685818110610595576105956121ec565b90506020020160208101906105aa91906121d1565b604080519283526001600160a01b0390911660208301520160405180910390a1506001016104cc565b505050505050565b5f54610100900460ff16158080156105f957505f54600160ff909116105b806106125750303b15801561061257505f5460ff166001145b61067a5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff19166001179055801561069b575f805461ff0019166101001790555b6106a4846115f4565b6106ad83611645565b6106b6826116b9565b80156106fb575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b6107096116f6565b606654818116811461072e5760405163c61dca5d60e01b815260040160405180910390fd5b610737826116b9565b5050565b6001610746816115aa565b61075360208401846121d1565b61075c81611799565b6107795760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815284906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc758906107c7908490600401612231565b602060405180830381865afa1580156107e2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610806919061223f565b61082357604051631fb1705560e21b815260040160405180910390fd5b8461083661026d36839003830183611f4c565b61085357604051634d2baea960e11b815260040160405180910390fd5b6105d361086536889003880188611f4c565b86611843565b604080518082019091525f8082526020820152609a5f61088a846118bd565b815260208082019290925260409081015f208151808301909252546001600160a01b0381168252600160a01b900463ffffffff169181019190915292915050565b60026108d6816115aa565b6108e360208401846121d1565b6108ec81611799565b6109095760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815284906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890610957908490600401612231565b602060405180830381865afa158015610972573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610996919061223f565b6109b357604051631fb1705560e21b815260040160405180910390fd5b846109c661026d36839003830183611f4c565b6109e357604051634d2baea960e11b815260040160405180910390fd5b6105d36109f536889003880188611f4c565b610a0436889003880188611f4c565b611920565b5f610a1e610a16836118bd565b6097906119e9565b92915050565b6060817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316637cffe48c846040518263ffffffff1660e01b8152600401610a739190612231565b602060405180830381865afa158015610a8e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ab2919061225e565b610ac461023136869003860186611f4c565b610ad661037636879003870187611f4c565b6001600160a01b03166341ee6d0e866040518263ffffffff1660e01b8152600401610b019190612231565b5f60405180830381865afa158015610b1b573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610b42919081019061227c565b604051602001610b559493929190612324565b6040516020818303038152906040529050919050565b6060610b967f0000000000000000000000000000000000000000000000000000000000000000611a00565b905090565b610ba36116f6565b610bad5f196116b9565b565b5f610bb9816115aa565b610bc660208301836121d1565b610bcf81611799565b610bec5760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815283906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890610c3a908490600401612231565b602060405180830381865afa158015610c55573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c79919061223f565b610c9657604051631fb1705560e21b815260040160405180910390fd5b83610ca961026d36839003830183611f4c565b610cc657604051634d2baea960e11b815260040160405180910390fd5b5f610cde610cd936889003880188611f4c565b6118bd565b5f818152609960205260409081902080546001600160a01b0319169055519091507fd7811913efd5d98fc7ea0d1fdd022b3d31987815360842d05b1d1cf55578d16a90610d2c908890612231565b60405180910390a15f818152609a60205260409081902080546001600160c01b0319169055517f210a1118a869246162804e2a7f21ef808ebd93f4be7ed512014fe29a7a8be02e90610d7f908890612231565b60405180910390a1610d92609782611a3d565b507f4ffdfdd59e9e1e3c301608788f78dd458e61cb8c045ca92b62a7b484c80824fb86604051610dc29190612231565b60405180910390a1505050505050565b610dda611550565b610bad5f6115f4565b5f60995f610df0846118bd565b815260208101919091526040015f20546001600160a01b031692915050565b5f610b966097611a48565b6060805f610e28609b611a51565b90505f8167ffffffffffffffff811115610e4457610e44611ea9565b604051908082528060200260200182016040528015610e6d578160200160208202803683370190505b5090505f8267ffffffffffffffff811115610e8a57610e8a611ea9565b604051908082528060200260200182016040528015610eb3578160200160208202803683370190505b5090505f5b83811015610f25575f80610ecd609b84611a5b565b9150915081858481518110610ee457610ee46121ec565b60200260200101818152505080848481518110610f0357610f036121ec565b6001600160a01b03909216602092830291909101909101525050600101610eb8565b509094909350915050565b60605f610f3d6097611a48565b90505f8167ffffffffffffffff811115610f5957610f59611ea9565b604051908082528060200260200182016040528015610f9d57816020015b604080518082019091525f8082526020820152815260200190600190039081610f775790505b5090505f5b8281101561101c575f610fb6609783611a78565b90505f610ff282604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b905080848481518110611007576110076121ec565b60209081029190910101525050600101610fa2565b5092915050565b5f61102d816115aa565b61103a60208501856121d1565b61104381611799565b6110605760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815285906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc758906110ae908490600401612231565b602060405180830381865afa1580156110c9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110ed919061223f565b61110a57604051631fb1705560e21b815260040160405180910390fd5b5f604051631f3ff92360e21b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690637cffe48c90611157908a90600401612231565b602060405180830381865afa158015611172573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611196919061225e565b60028111156111a7576111a7612310565b036111c55760405163e57cacbd60e01b815260040160405180910390fd5b6111e26111da610cd936899003890189611f4c565b609790611a83565b6111ff57604051631883461560e01b815260040160405180910390fd5b7f4fb6efec7dd60036ce3a7af8d5c48425019daa0fb61eb471a966a7ac2c6fa6a68660405161122e9190612231565b60405180910390a161124861086536889003880188611f4c565b6105d361125a36889003880188611f4c565b610a0436879003870187611f4c565b611271611550565b61127a81611645565b50565b6060818311156112a05760405163561ce9bb60e01b815260040160405180910390fd5b6112aa6097611a48565b8211156112ca576040516302da361360e61b815260040160405180910390fd5b5f6112d58484612390565b90505f8167ffffffffffffffff8111156112f1576112f1611ea9565b60405190808252806020026020018201604052801561133557816020015b604080518082019091525f808252602082015281526020019060019003908161130f5790505b5090505f5b828110156113be575f61135861135083896123a3565b609790611a78565b90505f61139482604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b9050808484815181106113a9576113a96121ec565b6020908102919091010152505060010161133a565b50949350505050565b6113cf611550565b60036113da816115aa565b5f5b828110156106fb575f8484838181106113f7576113f76121ec565b90506020020135905061141481609b611a8e90919063ffffffff16565b6114315760405163b3f92ba160e01b815260040160405180910390fd5b6040518181527f6824d36084ecf2cd819b137cb5d837cc6e73afce1e0e348c9fdecaa81d0341e59060200160405180910390a1506001016113dc565b611475611550565b6001600160a01b0381166114da5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610671565b61127a816115f4565b6114eb611a99565b606654801982198116146115125760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c9060200160405180910390a25050565b6033546001600160a01b03163314610bad5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610671565b606654600160ff83161b9081160361127a5760405163840a48d560e01b815260040160405180910390fd5b5f6115ea84846001600160a01b038516611b4a565b90505b9392505050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b5f8163ffffffff161161166b576040516316d98e1b60e31b815260040160405180910390fd5b609e805463ffffffff191663ffffffff83169081179091556040519081527f4fbcd0cca70015b33db8af4aa4f2bd6fd6c1efa9460b8e2333f252c1467a63279060200160405180910390a150565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015611758573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061177c919061223f565b610bad57604051631d77d47760e21b815260040160405180910390fd5b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af115801561181f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a1e919061223f565b8060995f611850856118bd565b81526020019081526020015f205f6101000a8154816001600160a01b0302191690836001600160a01b031602179055507f7f7ccafd92d20fdb39dee184a0dce002a9da420ed0def461f2a027abc9b3f6df82826040516118b19291906123b6565b60405180910390a15050565b5f815f0151826020015163ffffffff1660405160200161190892919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b604051602081830303815290604052610a1e906123dc565b602081015163ffffffff1615806119475750609e54602082015163ffffffff918216911610155b61196457604051632e46483160e11b815260040160405180910390fd5b80609a5f611971856118bd565b815260208082019290925260409081015f2083518154949093015163ffffffff16600160a01b026001600160c01b03199094166001600160a01b0390931692909217929092179055517f3147846ee526009000671c20380b856a633345691300f82585f90034715cf0e2906118b190849084906123ff565b5f81815260018301602052604081205415156115ed565b60605f611a0c83611b66565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b5f6115ed8383611b8d565b5f610a1e825490565b5f610a1e82611c70565b5f808080611a698686611c7a565b909450925050505b9250929050565b5f6115ed8383611ca3565b5f6115ed8383611cc9565b5f6115ed8383611d15565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611af5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b19919061241a565b6001600160a01b0316336001600160a01b031614610bad5760405163794821ff60e01b815260040160405180910390fd5b5f82815260028401602052604081208290556115ea8484611a83565b5f60ff8216601f811115610a1e57604051632cd44ac360e21b815260040160405180910390fd5b5f8181526001830160205260408120548015611c67575f611baf600183612390565b85549091505f90611bc290600190612390565b9050818114611c21575f865f018281548110611be057611be06121ec565b905f5260205f200154905080875f018481548110611c0057611c006121ec565b5f918252602080832090910192909255918252600188019052604090208390555b8554869080611c3257611c32612435565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050610a1e565b5f915050610a1e565b5f610a1e82611a48565b5f8080611c878585611a78565b5f81815260029690960160205260409095205494959350505050565b5f825f018281548110611cb857611cb86121ec565b905f5260205f200154905092915050565b5f818152600183016020526040812054611d0e57508154600181810184555f848152602080822090930184905584548482528286019093526040902091909155610a1e565b505f610a1e565b5f81815260028301602052604081208190556115ed8383611a3d565b5f5f83601f840112611d41575f5ffd5b50813567ffffffffffffffff811115611d58575f5ffd5b6020830191508360208260051b8501011115611a71575f5ffd5b5f5f5f5f60408587031215611d85575f5ffd5b843567ffffffffffffffff811115611d9b575f5ffd5b611da787828801611d31565b909550935050602085013567ffffffffffffffff811115611dc6575f5ffd5b611dd287828801611d31565b95989497509550505050565b6001600160a01b038116811461127a575f5ffd5b803563ffffffff81168114611e05575f5ffd5b919050565b5f5f5f60608486031215611e1c575f5ffd5b8335611e2781611dde565b9250611e3560208501611df2565b929592945050506040919091013590565b5f60208284031215611e56575f5ffd5b5035919050565b5f60408284031215611e6d575f5ffd5b50919050565b5f5f60608385031215611e84575f5ffd5b611e8e8484611e5d565b91506040830135611e9e81611dde565b809150509250929050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715611ee657611ee6611ea9565b604052919050565b5f60408284031215611efe575f5ffd5b6040805190810167ffffffffffffffff81118282101715611f2157611f21611ea9565b6040529050808235611f3281611dde565b8152611f4060208401611df2565b60208201525092915050565b5f60408284031215611f5c575f5ffd5b6115ed8383611eee565b80516001600160a01b0316825260209081015163ffffffff16910152565b60408101610a1e8284611f66565b5f5f60808385031215611fa3575f5ffd5b611fad8484611e5d565b9150611fbc8460408501611e5d565b90509250929050565b5f60408284031215611fd5575f5ffd5b6115ed8383611e5d565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f6115ed6020830184611fdf565b5f6020828403121561202f575f5ffd5b813560ff811681146115ed575f5ffd5b604080825283519082018190525f9060208501906060840190835b8181101561207857835183526020938401939092019160010161205a565b5050838103602080860191909152855180835291810192508501905f5b818110156120bc5782516001600160a01b0316845260209384019390920191600101612095565b50919695505050505050565b602080825282518282018190525f918401906040840190835b8181101561210a576120f4838551611f66565b60209390930192604092909201916001016120e1565b509095945050505050565b5f5f5f60a08486031215612127575f5ffd5b6121318585611e5d565b9250604084013561214181611dde565b91506121508560608601611e5d565b90509250925092565b5f60208284031215612169575f5ffd5b6115ed82611df2565b5f5f60408385031215612183575f5ffd5b50508035926020909101359150565b5f5f602083850312156121a3575f5ffd5b823567ffffffffffffffff8111156121b9575f5ffd5b6121c585828601611d31565b90969095509350505050565b5f602082840312156121e1575f5ffd5b81356115ed81611dde565b634e487b7160e01b5f52603260045260245ffd5b803561220b81611dde565b6001600160a01b0316825263ffffffff61222760208301611df2565b1660208301525050565b60408101610a1e8284612200565b5f6020828403121561224f575f5ffd5b815180151581146115ed575f5ffd5b5f6020828403121561226e575f5ffd5b8151600381106115ed575f5ffd5b5f6020828403121561228c575f5ffd5b815167ffffffffffffffff8111156122a2575f5ffd5b8201601f810184136122b2575f5ffd5b805167ffffffffffffffff8111156122cc576122cc611ea9565b6122df601f8201601f1916602001611ebd565b8181528560208385010111156122f3575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b634e487b7160e01b5f52602160045260245ffd5b61232e8186612200565b5f6003851061234b57634e487b7160e01b5f52602160045260245ffd5b84604083015261235e6060830185611f66565b60c060a083015261237260c0830184611fdf565b9695505050505050565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610a1e57610a1e61237c565b80820180821115610a1e57610a1e61237c565b606081016123c48285611f66565b6001600160a01b039290921660409190910152919050565b80516020808301519190811015611e6d575f1960209190910360031b1b16919050565b6080810161240d8285611f66565b6115ed6040830184611f66565b5f6020828403121561242a575f5ffd5b81516115ed81611dde565b634e487b7160e01b5f52603160045260245ffdfea26469706673582212209eee65670a0acd546a18377ada41e4f6518f10b5c3d618724117493c69a324be64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01 `@R4\x80\x15a\0\x10W__\xFD[P`@Qa(\x148\x03\x80a(\x14\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xC1V[\x80\x83\x86\x86\x85`\x01`\x01`\xA0\x1B\x03\x81\x16a\0[W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80R\x91\x82\x16`\xA0R\x81\x16`\xC0R\x16`\xE0Ra\0\x81\x81a\0\x98V[a\x01\0RPa\0\x8Ea\0\xDEV[PPPPPa\x03\x19V[__\x82\x90P`\x1F\x81Q\x11\x15a\0\xCBW\x82`@Qc0Z'\xA9`\xE0\x1B\x81R`\x04\x01a\0\xC2\x91\x90a\x02\xBEV[`@Q\x80\x91\x03\x90\xFD[\x80Qa\0\xD6\x82a\x02\xF3V[\x17\x93\x92PPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x01EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01a\0\xC2V[_T`\xFF\x90\x81\x16\x14a\x01\x94W_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xAAW__\xFD[PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_____`\xA0\x86\x88\x03\x12\x15a\x01\xD5W__\xFD[\x85Qa\x01\xE0\x81a\x01\x96V[` \x87\x01Q\x90\x95Pa\x01\xF1\x81a\x01\x96V[`@\x87\x01Q\x90\x94Pa\x02\x02\x81a\x01\x96V[``\x87\x01Q\x90\x93Pa\x02\x13\x81a\x01\x96V[`\x80\x87\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02.W__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\x02>W__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02WWa\x02Wa\x01\xADV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02\x85Wa\x02\x85a\x01\xADV[`@R\x81\x81R\x82\x82\x01` \x01\x8A\x10\x15a\x02\x9CW__\xFD[\x81` \x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x93PPPP\x92\x95P\x92\x95\x90\x93PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x03\x13W_\x19\x81` \x03`\x03\x1B\x1B\x82\x16\x91P[P\x91\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa$\x7Fa\x03\x95_9_a\x0Br\x01R_\x81\x81a\x02\xE6\x01Ra\x17\xD7\x01R_\x81\x81a\x02\x87\x01R\x81\x81a\n)\x01Ra\x11\"\x01R_\x81\x81a\x03\xEE\x01R\x81\x81a\x07\x92\x01R\x81\x81a\t\"\x01R\x81\x81a\x0C\x05\x01Ra\x10y\x01R_\x81\x81a\x03\x80\x01R\x81\x81a\x17\x0B\x01Ra\x1A\x9B\x01Ra$\x7F_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xD1W_5`\xE0\x1C\x80cqP\x18\xA6\x11a\0\xFEW\x80c\xCA\x8A\xA7\xC7\x11a\0\x9EW\x80c\xD9\xA6r\x9E\x11a\0nW\x80c\xD9\xA6r\x9E\x14a\x04KW\x80c\xDF\xBD\x9D\xFD\x14a\x04^W\x80c\xF2\xFD\xE3\x8B\x14a\x04qW\x80c\xFA\xBC\x1C\xBC\x14a\x04\x84W__\xFD[\x80c\xCA\x8A\xA7\xC7\x14a\x03\xE9W\x80c\xD0\x9B\x97\x8B\x14a\x04\x10W\x80c\xD5\x04I\x11\x14a\x04%W\x80c\xD6\xDB\x9E%\x14a\x048W__\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xD9W\x80c\x8D\xA5\xCB[\x14a\x03\xA2W\x80c\xACP_K\x14a\x03\xB3W\x80c\xB1\x86\xA6\x0E\x14a\x03\xCBW\x80c\xC4\xBF\xFE+\x14a\x03\xD3W__\xFD[\x80cqP\x18\xA6\x14a\x03`W\x80cu\xE4\xB59\x14a\x03hW\x80c\x88o\x11\x95\x14a\x03{W__\xFD[\x80c>\xC4\\~\x11a\x01tW\x80cY\\jg\x11a\x01DW\x80cY\\jg\x14a\x03\x10W\x80cZ\xC8j\xB7\x14a\x03\x18W\x80c\\\x97Z\xBB\x14a\x03;W\x80clU\xA3\x7F\x14a\x03MW__\xFD[\x80c>\xC4\\~\x14a\x02\x82W\x80cA\xEEm\x0E\x14a\x02\xC1W\x80cFW\xE2j\x14a\x02\xE1W\x80cT\xFDMP\x14a\x03\x08W__\xFD[\x80c\x1C\xA9\x14*\x11a\x01\xAFW\x80c\x1C\xA9\x14*\x14a\x02\x10W\x80c!\xFA\x7F\xDC\x14a\x02#W\x80c'~\x1Eb\x14a\x02LW\x80c6\xB2\0\xDE\x14a\x02_W__\xFD[\x80c\x04\xE9\x8B\xE3\x14a\x01\xD5W\x80c\x0F\x19\xAA\xEF\x14a\x01\xEAW\x80c\x13d9\xDD\x14a\x01\xFDW[__\xFD[a\x01\xE8a\x01\xE36`\x04a\x1DrV[a\x04\x97V[\0[a\x01\xE8a\x01\xF86`\x04a\x1E\nV[a\x05\xDBV[a\x01\xE8a\x02\x0B6`\x04a\x1EFV[a\x07\x01V[a\x01\xE8a\x02\x1E6`\x04a\x1EsV[a\x07;V[a\x026a\x0216`\x04a\x1FLV[a\x08kV[`@Qa\x02C\x91\x90a\x1F\x84V[`@Q\x80\x91\x03\x90\xF3[a\x01\xE8a\x02Z6`\x04a\x1F\x92V[a\x08\xCBV[a\x02ra\x02m6`\x04a\x1FLV[a\n\tV[`@Q\x90\x15\x15\x81R` \x01a\x02CV[a\x02\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02CV[a\x02\xD4a\x02\xCF6`\x04a\x1F\xC5V[a\n$V[`@Qa\x02C\x91\x90a \rV[a\x02\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xD4a\x0BkV[a\x01\xE8a\x0B\x9BV[a\x02ra\x03&6`\x04a \x1FV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fT[`@Q\x90\x81R` \x01a\x02CV[a\x01\xE8a\x03[6`\x04a\x1F\xC5V[a\x0B\xAFV[a\x01\xE8a\r\xD2V[a\x02\xA9a\x03v6`\x04a\x1FLV[a\r\xE3V[a\x02\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xA9V[`\x9ET`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02CV[a\x03?a\x0E\x0FV[a\x03\xDBa\x0E\x1AV[`@Qa\x02C\x92\x91\x90a ?V[a\x02\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\x18a\x0F0V[`@Qa\x02C\x91\x90a \xC8V[a\x01\xE8a\x0436`\x04a!\x15V[a\x10#V[a\x01\xE8a\x04F6`\x04a!YV[a\x12iV[a\x04\x18a\x04Y6`\x04a!rV[a\x12}V[a\x01\xE8a\x04l6`\x04a!\x92V[a\x13\xC7V[a\x01\xE8a\x04\x7F6`\x04a!\xD1V[a\x14mV[a\x01\xE8a\x04\x926`\x04a\x1EFV[a\x14\xE3V[a\x04\x9Fa\x15PV[`\x03a\x04\xAA\x81a\x15\xAAV[\x83\x82\x14a\x04\xCAW`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x05\xD3W_\x86\x86\x83\x81\x81\x10a\x04\xE7Wa\x04\xE7a!\xECV[\x90P` \x02\x015\x90P\x80_\x03a\x05\x10W`@Qc=#\xE4\xD1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05D\x81\x86\x86\x85\x81\x81\x10a\x05&Wa\x05&a!\xECV[\x90P` \x02\x01` \x81\x01\x90a\x05;\x91\x90a!\xD1V[`\x9B\x91\x90a\x15\xD5V[a\x05aW`@Qc$\xBFc\x1B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7Fz\nv\xD8[X+\x17\x99m\xD77\x1A@z\xA7\xA7\x9B\x87\r\xB8S\x92G\xFB\xA3\x15\xC7\xB6\xBE\xFFb\x81\x86\x86\x85\x81\x81\x10a\x05\x95Wa\x05\x95a!\xECV[\x90P` \x02\x01` \x81\x01\x90a\x05\xAA\x91\x90a!\xD1V[`@\x80Q\x92\x83R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1P`\x01\x01a\x04\xCCV[PPPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x05\xF9WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x06\x12WP0;\x15\x80\x15a\x06\x12WP_T`\xFF\x16`\x01\x14[a\x06zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x06\x9BW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x06\xA4\x84a\x15\xF4V[a\x06\xAD\x83a\x16EV[a\x06\xB6\x82a\x16\xB9V[\x80\x15a\x06\xFBW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[a\x07\ta\x16\xF6V[`fT\x81\x81\x16\x81\x14a\x07.W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x077\x82a\x16\xB9V[PPV[`\x01a\x07F\x81a\x15\xAAV[a\x07S` \x84\x01\x84a!\xD1V[a\x07\\\x81a\x17\x99V[a\x07yW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x84\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x07\xC7\x90\x84\x90`\x04\x01a\"1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x06\x91\x90a\"?V[a\x08#W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84a\x086a\x02m6\x83\x90\x03\x83\x01\x83a\x1FLV[a\x08SW`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xD3a\x08e6\x88\x90\x03\x88\x01\x88a\x1FLV[\x86a\x18CV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x9A_a\x08\x8A\x84a\x18\xBDV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q\x80\x83\x01\x90\x92RT`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x92\x91PPV[`\x02a\x08\xD6\x81a\x15\xAAV[a\x08\xE3` \x84\x01\x84a!\xD1V[a\x08\xEC\x81a\x17\x99V[a\t\tW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x84\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\tW\x90\x84\x90`\x04\x01a\"1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\trW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x96\x91\x90a\"?V[a\t\xB3W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84a\t\xC6a\x02m6\x83\x90\x03\x83\x01\x83a\x1FLV[a\t\xE3W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xD3a\t\xF56\x88\x90\x03\x88\x01\x88a\x1FLV[a\n\x046\x88\x90\x03\x88\x01\x88a\x1FLV[a\x19 V[_a\n\x1Ea\n\x16\x83a\x18\xBDV[`\x97\x90a\x19\xE9V[\x92\x91PPV[``\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c|\xFF\xE4\x8C\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\ns\x91\x90a\"1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xB2\x91\x90a\"^V[a\n\xC4a\x0216\x86\x90\x03\x86\x01\x86a\x1FLV[a\n\xD6a\x03v6\x87\x90\x03\x87\x01\x87a\x1FLV[`\x01`\x01`\xA0\x1B\x03\x16cA\xEEm\x0E\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x01\x91\x90a\"1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x1BW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0BB\x91\x90\x81\x01\x90a\"|V[`@Q` \x01a\x0BU\x94\x93\x92\x91\x90a#$V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[``a\x0B\x96\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\0V[\x90P\x90V[a\x0B\xA3a\x16\xF6V[a\x0B\xAD_\x19a\x16\xB9V[V[_a\x0B\xB9\x81a\x15\xAAV[a\x0B\xC6` \x83\x01\x83a!\xD1V[a\x0B\xCF\x81a\x17\x99V[a\x0B\xECW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x83\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x0C:\x90\x84\x90`\x04\x01a\"1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CUW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cy\x91\x90a\"?V[a\x0C\x96W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a\x0C\xA9a\x02m6\x83\x90\x03\x83\x01\x83a\x1FLV[a\x0C\xC6W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\xDEa\x0C\xD96\x88\x90\x03\x88\x01\x88a\x1FLV[a\x18\xBDV[_\x81\x81R`\x99` R`@\x90\x81\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UQ\x90\x91P\x7F\xD7\x81\x19\x13\xEF\xD5\xD9\x8F\xC7\xEA\r\x1F\xDD\x02+=1\x98x\x156\x08B\xD0[\x1D\x1C\xF5Ux\xD1j\x90a\r,\x90\x88\x90a\"1V[`@Q\x80\x91\x03\x90\xA1_\x81\x81R`\x9A` R`@\x90\x81\x90 \x80T`\x01`\x01`\xC0\x1B\x03\x19\x16\x90UQ\x7F!\n\x11\x18\xA8i$ab\x80N*\x7F!\xEF\x80\x8E\xBD\x93\xF4\xBE~\xD5\x12\x01O\xE2\x9Az\x8B\xE0.\x90a\r\x7F\x90\x88\x90a\"1V[`@Q\x80\x91\x03\x90\xA1a\r\x92`\x97\x82a\x1A=V[P\x7FO\xFD\xFD\xD5\x9E\x9E\x1E<0\x16\x08x\x8Fx\xDDE\x8Ea\xCB\x8C\x04\\\xA9+b\xA7\xB4\x84\xC8\x08$\xFB\x86`@Qa\r\xC2\x91\x90a\"1V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[a\r\xDAa\x15PV[a\x0B\xAD_a\x15\xF4V[_`\x99_a\r\xF0\x84a\x18\xBDV[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[_a\x0B\x96`\x97a\x1AHV[``\x80_a\x0E(`\x9Ba\x1AQV[\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EDWa\x0EDa\x1E\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0EmW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x8AWa\x0E\x8Aa\x1E\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xB3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x0F%W_\x80a\x0E\xCD`\x9B\x84a\x1A[V[\x91P\x91P\x81\x85\x84\x81Q\x81\x10a\x0E\xE4Wa\x0E\xE4a!\xECV[` \x02` \x01\x01\x81\x81RPP\x80\x84\x84\x81Q\x81\x10a\x0F\x03Wa\x0F\x03a!\xECV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPP`\x01\x01a\x0E\xB8V[P\x90\x94\x90\x93P\x91PPV[``_a\x0F=`\x97a\x1AHV[\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FYWa\x0FYa\x1E\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x9DW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0FwW\x90P[P\x90P_[\x82\x81\x10\x15a\x10\x1CW_a\x0F\xB6`\x97\x83a\x1AxV[\x90P_a\x0F\xF2\x82`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x90P\x80\x84\x84\x81Q\x81\x10a\x10\x07Wa\x10\x07a!\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x0F\xA2V[P\x92\x91PPV[_a\x10-\x81a\x15\xAAV[a\x10:` \x85\x01\x85a!\xD1V[a\x10C\x81a\x17\x99V[a\x10`W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x85\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x10\xAE\x90\x84\x90`\x04\x01a\"1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xED\x91\x90a\"?V[a\x11\nW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Qc\x1F?\xF9#`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c|\xFF\xE4\x8C\x90a\x11W\x90\x8A\x90`\x04\x01a\"1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x96\x91\x90a\"^V[`\x02\x81\x11\x15a\x11\xA7Wa\x11\xA7a#\x10V[\x03a\x11\xC5W`@Qc\xE5|\xAC\xBD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\xE2a\x11\xDAa\x0C\xD96\x89\x90\x03\x89\x01\x89a\x1FLV[`\x97\x90a\x1A\x83V[a\x11\xFFW`@Qc\x18\x83F\x15`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7FO\xB6\xEF\xEC}\xD6\x006\xCE:z\xF8\xD5\xC4\x84%\x01\x9D\xAA\x0F\xB6\x1E\xB4q\xA9f\xA7\xAC,o\xA6\xA6\x86`@Qa\x12.\x91\x90a\"1V[`@Q\x80\x91\x03\x90\xA1a\x12Ha\x08e6\x88\x90\x03\x88\x01\x88a\x1FLV[a\x05\xD3a\x12Z6\x88\x90\x03\x88\x01\x88a\x1FLV[a\n\x046\x87\x90\x03\x87\x01\x87a\x1FLV[a\x12qa\x15PV[a\x12z\x81a\x16EV[PV[``\x81\x83\x11\x15a\x12\xA0W`@QcV\x1C\xE9\xBB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xAA`\x97a\x1AHV[\x82\x11\x15a\x12\xCAW`@Qc\x02\xDA6\x13`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x12\xD5\x84\x84a#\x90V[\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xF1Wa\x12\xF1a\x1E\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x135W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13\x0FW\x90P[P\x90P_[\x82\x81\x10\x15a\x13\xBEW_a\x13Xa\x13P\x83\x89a#\xA3V[`\x97\x90a\x1AxV[\x90P_a\x13\x94\x82`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x90P\x80\x84\x84\x81Q\x81\x10a\x13\xA9Wa\x13\xA9a!\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x13:V[P\x94\x93PPPPV[a\x13\xCFa\x15PV[`\x03a\x13\xDA\x81a\x15\xAAV[_[\x82\x81\x10\x15a\x06\xFBW_\x84\x84\x83\x81\x81\x10a\x13\xF7Wa\x13\xF7a!\xECV[\x90P` \x02\x015\x90Pa\x14\x14\x81`\x9Ba\x1A\x8E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x141W`@Qc\xB3\xF9+\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x81\x81R\x7Fh$\xD3`\x84\xEC\xF2\xCD\x81\x9B\x13|\xB5\xD87\xCCns\xAF\xCE\x1E\x0E4\x8C\x9F\xDE\xCA\xA8\x1D\x03A\xE5\x90` \x01`@Q\x80\x91\x03\x90\xA1P`\x01\x01a\x13\xDCV[a\x14ua\x15PV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x14\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06qV[a\x12z\x81a\x15\xF4V[a\x14\xEBa\x1A\x99V[`fT\x80\x19\x82\x19\x81\x16\x14a\x15\x12W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06qV[`fT`\x01`\xFF\x83\x16\x1B\x90\x81\x16\x03a\x12zW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x15\xEA\x84\x84`\x01`\x01`\xA0\x1B\x03\x85\x16a\x1BJV[\x90P[\x93\x92PPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[_\x81c\xFF\xFF\xFF\xFF\x16\x11a\x16kW`@Qc\x16\xD9\x8E\x1B`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x9E\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7FO\xBC\xD0\xCC\xA7\0\x15\xB3=\xB8\xAFJ\xA4\xF2\xBDo\xD6\xC1\xEF\xA9F\x0B\x8E#3\xF2R\xC1Fzc'\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17XW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17|\x91\x90a\"?V[a\x0B\xADW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18\x1FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x1E\x91\x90a\"?V[\x80`\x99_a\x18P\x85a\x18\xBDV[\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x7F\x7F|\xCA\xFD\x92\xD2\x0F\xDB9\xDE\xE1\x84\xA0\xDC\xE0\x02\xA9\xDAB\x0E\xD0\xDE\xF4a\xF2\xA0'\xAB\xC9\xB3\xF6\xDF\x82\x82`@Qa\x18\xB1\x92\x91\x90a#\xB6V[`@Q\x80\x91\x03\x90\xA1PPV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a\x19\x08\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\n\x1E\x90a#\xDCV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x19GWP`\x9ET` \x82\x01Qc\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x10\x15[a\x19dW`@Qc.FH1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x9A_a\x19q\x85a\x18\xBDV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x83Q\x81T\x94\x90\x93\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x92\x90\x92\x17\x90UQ\x7F1G\x84n\xE5&\0\x90\0g\x1C 8\x0B\x85jc3Ei\x13\0\xF8%\x85\xF9\x004q\\\xF0\xE2\x90a\x18\xB1\x90\x84\x90\x84\x90a#\xFFV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x15\xEDV[``_a\x1A\x0C\x83a\x1BfV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[_a\x15\xED\x83\x83a\x1B\x8DV[_a\n\x1E\x82T\x90V[_a\n\x1E\x82a\x1CpV[_\x80\x80\x80a\x1Ai\x86\x86a\x1CzV[\x90\x94P\x92PPP[\x92P\x92\x90PV[_a\x15\xED\x83\x83a\x1C\xA3V[_a\x15\xED\x83\x83a\x1C\xC9V[_a\x15\xED\x83\x83a\x1D\x15V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x19\x91\x90a$\x1AV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xADW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x81R`\x02\x84\x01` R`@\x81 \x82\x90Ua\x15\xEA\x84\x84a\x1A\x83V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\n\x1EW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x1CgW_a\x1B\xAF`\x01\x83a#\x90V[\x85T\x90\x91P_\x90a\x1B\xC2\x90`\x01\x90a#\x90V[\x90P\x81\x81\x14a\x1C!W_\x86_\x01\x82\x81T\x81\x10a\x1B\xE0Wa\x1B\xE0a!\xECV[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a\x1C\0Wa\x1C\0a!\xECV[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x1C2Wa\x1C2a$5V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\n\x1EV[_\x91PPa\n\x1EV[_a\n\x1E\x82a\x1AHV[_\x80\x80a\x1C\x87\x85\x85a\x1AxV[_\x81\x81R`\x02\x96\x90\x96\x01` R`@\x90\x95 T\x94\x95\x93PPPPV[_\x82_\x01\x82\x81T\x81\x10a\x1C\xB8Wa\x1C\xB8a!\xECV[\x90_R` _ \x01T\x90P\x92\x91PPV[_\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x1D\x0EWP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\n\x1EV[P_a\n\x1EV[_\x81\x81R`\x02\x83\x01` R`@\x81 \x81\x90Ua\x15\xED\x83\x83a\x1A=V[__\x83`\x1F\x84\x01\x12a\x1DAW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DXW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1AqW__\xFD[____`@\x85\x87\x03\x12\x15a\x1D\x85W__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x9BW__\xFD[a\x1D\xA7\x87\x82\x88\x01a\x1D1V[\x90\x95P\x93PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xC6W__\xFD[a\x1D\xD2\x87\x82\x88\x01a\x1D1V[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12zW__\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1E\x05W__\xFD[\x91\x90PV[___``\x84\x86\x03\x12\x15a\x1E\x1CW__\xFD[\x835a\x1E'\x81a\x1D\xDEV[\x92Pa\x1E5` \x85\x01a\x1D\xF2V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a\x1EVW__\xFD[P5\x91\x90PV[_`@\x82\x84\x03\x12\x15a\x1EmW__\xFD[P\x91\x90PV[__``\x83\x85\x03\x12\x15a\x1E\x84W__\xFD[a\x1E\x8E\x84\x84a\x1E]V[\x91P`@\x83\x015a\x1E\x9E\x81a\x1D\xDEV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\xE6Wa\x1E\xE6a\x1E\xA9V[`@R\x91\x90PV[_`@\x82\x84\x03\x12\x15a\x1E\xFEW__\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1F!Wa\x1F!a\x1E\xA9V[`@R\x90P\x80\x825a\x1F2\x81a\x1D\xDEV[\x81Ra\x1F@` \x84\x01a\x1D\xF2V[` \x82\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15a\x1F\\W__\xFD[a\x15\xED\x83\x83a\x1E\xEEV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`@\x81\x01a\n\x1E\x82\x84a\x1FfV[__`\x80\x83\x85\x03\x12\x15a\x1F\xA3W__\xFD[a\x1F\xAD\x84\x84a\x1E]V[\x91Pa\x1F\xBC\x84`@\x85\x01a\x1E]V[\x90P\x92P\x92\x90PV[_`@\x82\x84\x03\x12\x15a\x1F\xD5W__\xFD[a\x15\xED\x83\x83a\x1E]V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x15\xED` \x83\x01\x84a\x1F\xDFV[_` \x82\x84\x03\x12\x15a /W__\xFD[\x815`\xFF\x81\x16\x81\x14a\x15\xEDW__\xFD[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a xW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a ZV[PP\x83\x81\x03` \x80\x86\x01\x91\x90\x91R\x85Q\x80\x83R\x91\x81\x01\x92P\x85\x01\x90_[\x81\x81\x10\x15a \xBCW\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a \x95V[P\x91\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!\nWa \xF4\x83\x85Qa\x1FfV[` \x93\x90\x93\x01\x92`@\x92\x90\x92\x01\x91`\x01\x01a \xE1V[P\x90\x95\x94PPPPPV[___`\xA0\x84\x86\x03\x12\x15a!'W__\xFD[a!1\x85\x85a\x1E]V[\x92P`@\x84\x015a!A\x81a\x1D\xDEV[\x91Pa!P\x85``\x86\x01a\x1E]V[\x90P\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a!iW__\xFD[a\x15\xED\x82a\x1D\xF2V[__`@\x83\x85\x03\x12\x15a!\x83W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[__` \x83\x85\x03\x12\x15a!\xA3W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xB9W__\xFD[a!\xC5\x85\x82\x86\x01a\x1D1V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15a!\xE1W__\xFD[\x815a\x15\xED\x81a\x1D\xDEV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x805a\"\x0B\x81a\x1D\xDEV[`\x01`\x01`\xA0\x1B\x03\x16\x82Rc\xFF\xFF\xFF\xFFa\"'` \x83\x01a\x1D\xF2V[\x16` \x83\x01RPPV[`@\x81\x01a\n\x1E\x82\x84a\"\0V[_` \x82\x84\x03\x12\x15a\"OW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x15\xEDW__\xFD[_` \x82\x84\x03\x12\x15a\"nW__\xFD[\x81Q`\x03\x81\x10a\x15\xEDW__\xFD[_` \x82\x84\x03\x12\x15a\"\x8CW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xA2W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\"\xB2W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xCCWa\"\xCCa\x1E\xA9V[a\"\xDF`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1E\xBDV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\"\xF3W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[a#.\x81\x86a\"\0V[_`\x03\x85\x10a#KWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x84`@\x83\x01Ra#^``\x83\x01\x85a\x1FfV[`\xC0`\xA0\x83\x01Ra#r`\xC0\x83\x01\x84a\x1F\xDFV[\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\n\x1EWa\n\x1Ea#|V[\x80\x82\x01\x80\x82\x11\x15a\n\x1EWa\n\x1Ea#|V[``\x81\x01a#\xC4\x82\x85a\x1FfV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`@\x91\x90\x91\x01R\x91\x90PV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x1EmW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\x80\x81\x01a$\r\x82\x85a\x1FfV[a\x15\xED`@\x83\x01\x84a\x1FfV[_` \x82\x84\x03\x12\x15a$*W__\xFD[\x81Qa\x15\xED\x81a\x1D\xDEV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x9E\xEEeg\n\n\xCDTj\x187z\xDAA\xE4\xF6Q\x8F\x10\xB5\xC3\xD6\x18rA\x17I<i\xA3$\xBEdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106101d1575f3560e01c8063715018a6116100fe578063ca8aa7c71161009e578063d9a6729e1161006e578063d9a6729e1461044b578063dfbd9dfd1461045e578063f2fde38b14610471578063fabc1cbc14610484575f5ffd5b8063ca8aa7c7146103e9578063d09b978b14610410578063d504491114610425578063d6db9e2514610438575f5ffd5b80638da5cb5b116100d95780638da5cb5b146103a2578063ac505f4b146103b3578063b186a60e146103cb578063c4bffe2b146103d3575f5ffd5b8063715018a61461036057806375e4b53914610368578063886f11951461037b575f5ffd5b80633ec45c7e11610174578063595c6a6711610144578063595c6a67146103105780635ac86ab7146103185780635c975abb1461033b5780636c55a37f1461034d575f5ffd5b80633ec45c7e1461028257806341ee6d0e146102c15780634657e26a146102e157806354fd4d5014610308575f5ffd5b80631ca9142a116101af5780631ca9142a1461021057806321fa7fdc14610223578063277e1e621461024c57806336b200de1461025f575f5ffd5b806304e98be3146101d55780630f19aaef146101ea578063136439dd146101fd575b5f5ffd5b6101e86101e3366004611d72565b610497565b005b6101e86101f8366004611e0a565b6105db565b6101e861020b366004611e46565b610701565b6101e861021e366004611e73565b61073b565b610236610231366004611f4c565b61086b565b6040516102439190611f84565b60405180910390f35b6101e861025a366004611f92565b6108cb565b61027261026d366004611f4c565b610a09565b6040519015158152602001610243565b6102a97f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610243565b6102d46102cf366004611fc5565b610a24565b604051610243919061200d565b6102a97f000000000000000000000000000000000000000000000000000000000000000081565b6102d4610b6b565b6101e8610b9b565b61027261032636600461201f565b606654600160ff9092169190911b9081161490565b6066545b604051908152602001610243565b6101e861035b366004611fc5565b610baf565b6101e8610dd2565b6102a9610376366004611f4c565b610de3565b6102a97f000000000000000000000000000000000000000000000000000000000000000081565b6033546001600160a01b03166102a9565b609e5460405163ffffffff9091168152602001610243565b61033f610e0f565b6103db610e1a565b60405161024392919061203f565b6102a97f000000000000000000000000000000000000000000000000000000000000000081565b610418610f30565b60405161024391906120c8565b6101e8610433366004612115565b611023565b6101e8610446366004612159565b611269565b610418610459366004612172565b61127d565b6101e861046c366004612192565b6113c7565b6101e861047f3660046121d1565b61146d565b6101e8610492366004611e46565b6114e3565b61049f611550565b60036104aa816115aa565b8382146104ca5760405163512509d360e11b815260040160405180910390fd5b5f5b848110156105d3575f8686838181106104e7576104e76121ec565b905060200201359050805f0361051057604051633d23e4d160e11b815260040160405180910390fd5b61054481868685818110610526576105266121ec565b905060200201602081019061053b91906121d1565b609b91906115d5565b610561576040516324bf631b60e11b815260040160405180910390fd5b7f7a0a76d85b582b17996dd7371a407aa7a79b870db8539247fba315c7b6beff6281868685818110610595576105956121ec565b90506020020160208101906105aa91906121d1565b604080519283526001600160a01b0390911660208301520160405180910390a1506001016104cc565b505050505050565b5f54610100900460ff16158080156105f957505f54600160ff909116105b806106125750303b15801561061257505f5460ff166001145b61067a5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff19166001179055801561069b575f805461ff0019166101001790555b6106a4846115f4565b6106ad83611645565b6106b6826116b9565b80156106fb575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b6107096116f6565b606654818116811461072e5760405163c61dca5d60e01b815260040160405180910390fd5b610737826116b9565b5050565b6001610746816115aa565b61075360208401846121d1565b61075c81611799565b6107795760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815284906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc758906107c7908490600401612231565b602060405180830381865afa1580156107e2573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610806919061223f565b61082357604051631fb1705560e21b815260040160405180910390fd5b8461083661026d36839003830183611f4c565b61085357604051634d2baea960e11b815260040160405180910390fd5b6105d361086536889003880188611f4c565b86611843565b604080518082019091525f8082526020820152609a5f61088a846118bd565b815260208082019290925260409081015f208151808301909252546001600160a01b0381168252600160a01b900463ffffffff169181019190915292915050565b60026108d6816115aa565b6108e360208401846121d1565b6108ec81611799565b6109095760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815284906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890610957908490600401612231565b602060405180830381865afa158015610972573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610996919061223f565b6109b357604051631fb1705560e21b815260040160405180910390fd5b846109c661026d36839003830183611f4c565b6109e357604051634d2baea960e11b815260040160405180910390fd5b6105d36109f536889003880188611f4c565b610a0436889003880188611f4c565b611920565b5f610a1e610a16836118bd565b6097906119e9565b92915050565b6060817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316637cffe48c846040518263ffffffff1660e01b8152600401610a739190612231565b602060405180830381865afa158015610a8e573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ab2919061225e565b610ac461023136869003860186611f4c565b610ad661037636879003870187611f4c565b6001600160a01b03166341ee6d0e866040518263ffffffff1660e01b8152600401610b019190612231565b5f60405180830381865afa158015610b1b573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610b42919081019061227c565b604051602001610b559493929190612324565b6040516020818303038152906040529050919050565b6060610b967f0000000000000000000000000000000000000000000000000000000000000000611a00565b905090565b610ba36116f6565b610bad5f196116b9565b565b5f610bb9816115aa565b610bc660208301836121d1565b610bcf81611799565b610bec5760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815283906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc75890610c3a908490600401612231565b602060405180830381865afa158015610c55573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c79919061223f565b610c9657604051631fb1705560e21b815260040160405180910390fd5b83610ca961026d36839003830183611f4c565b610cc657604051634d2baea960e11b815260040160405180910390fd5b5f610cde610cd936889003880188611f4c565b6118bd565b5f818152609960205260409081902080546001600160a01b0319169055519091507fd7811913efd5d98fc7ea0d1fdd022b3d31987815360842d05b1d1cf55578d16a90610d2c908890612231565b60405180910390a15f818152609a60205260409081902080546001600160c01b0319169055517f210a1118a869246162804e2a7f21ef808ebd93f4be7ed512014fe29a7a8be02e90610d7f908890612231565b60405180910390a1610d92609782611a3d565b507f4ffdfdd59e9e1e3c301608788f78dd458e61cb8c045ca92b62a7b484c80824fb86604051610dc29190612231565b60405180910390a1505050505050565b610dda611550565b610bad5f6115f4565b5f60995f610df0846118bd565b815260208101919091526040015f20546001600160a01b031692915050565b5f610b966097611a48565b6060805f610e28609b611a51565b90505f8167ffffffffffffffff811115610e4457610e44611ea9565b604051908082528060200260200182016040528015610e6d578160200160208202803683370190505b5090505f8267ffffffffffffffff811115610e8a57610e8a611ea9565b604051908082528060200260200182016040528015610eb3578160200160208202803683370190505b5090505f5b83811015610f25575f80610ecd609b84611a5b565b9150915081858481518110610ee457610ee46121ec565b60200260200101818152505080848481518110610f0357610f036121ec565b6001600160a01b03909216602092830291909101909101525050600101610eb8565b509094909350915050565b60605f610f3d6097611a48565b90505f8167ffffffffffffffff811115610f5957610f59611ea9565b604051908082528060200260200182016040528015610f9d57816020015b604080518082019091525f8082526020820152815260200190600190039081610f775790505b5090505f5b8281101561101c575f610fb6609783611a78565b90505f610ff282604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b905080848481518110611007576110076121ec565b60209081029190910101525050600101610fa2565b5092915050565b5f61102d816115aa565b61103a60208501856121d1565b61104381611799565b6110605760405163932d94f760e01b815260040160405180910390fd5b6040516304c1b8eb60e31b815285906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063260dc758906110ae908490600401612231565b602060405180830381865afa1580156110c9573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906110ed919061223f565b61110a57604051631fb1705560e21b815260040160405180910390fd5b5f604051631f3ff92360e21b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690637cffe48c90611157908a90600401612231565b602060405180830381865afa158015611172573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611196919061225e565b60028111156111a7576111a7612310565b036111c55760405163e57cacbd60e01b815260040160405180910390fd5b6111e26111da610cd936899003890189611f4c565b609790611a83565b6111ff57604051631883461560e01b815260040160405180910390fd5b7f4fb6efec7dd60036ce3a7af8d5c48425019daa0fb61eb471a966a7ac2c6fa6a68660405161122e9190612231565b60405180910390a161124861086536889003880188611f4c565b6105d361125a36889003880188611f4c565b610a0436879003870187611f4c565b611271611550565b61127a81611645565b50565b6060818311156112a05760405163561ce9bb60e01b815260040160405180910390fd5b6112aa6097611a48565b8211156112ca576040516302da361360e61b815260040160405180910390fd5b5f6112d58484612390565b90505f8167ffffffffffffffff8111156112f1576112f1611ea9565b60405190808252806020026020018201604052801561133557816020015b604080518082019091525f808252602082015281526020019060019003908161130f5790505b5090505f5b828110156113be575f61135861135083896123a3565b609790611a78565b90505f61139482604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b9050808484815181106113a9576113a96121ec565b6020908102919091010152505060010161133a565b50949350505050565b6113cf611550565b60036113da816115aa565b5f5b828110156106fb575f8484838181106113f7576113f76121ec565b90506020020135905061141481609b611a8e90919063ffffffff16565b6114315760405163b3f92ba160e01b815260040160405180910390fd5b6040518181527f6824d36084ecf2cd819b137cb5d837cc6e73afce1e0e348c9fdecaa81d0341e59060200160405180910390a1506001016113dc565b611475611550565b6001600160a01b0381166114da5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610671565b61127a816115f4565b6114eb611a99565b606654801982198116146115125760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c9060200160405180910390a25050565b6033546001600160a01b03163314610bad5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610671565b606654600160ff83161b9081160361127a5760405163840a48d560e01b815260040160405180910390fd5b5f6115ea84846001600160a01b038516611b4a565b90505b9392505050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b5f8163ffffffff161161166b576040516316d98e1b60e31b815260040160405180910390fd5b609e805463ffffffff191663ffffffff83169081179091556040519081527f4fbcd0cca70015b33db8af4aa4f2bd6fd6c1efa9460b8e2333f252c1467a63279060200160405180910390a150565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015611758573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061177c919061223f565b610bad57604051631d77d47760e21b815260040160405180910390fd5b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af115801561181f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a1e919061223f565b8060995f611850856118bd565b81526020019081526020015f205f6101000a8154816001600160a01b0302191690836001600160a01b031602179055507f7f7ccafd92d20fdb39dee184a0dce002a9da420ed0def461f2a027abc9b3f6df82826040516118b19291906123b6565b60405180910390a15050565b5f815f0151826020015163ffffffff1660405160200161190892919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b604051602081830303815290604052610a1e906123dc565b602081015163ffffffff1615806119475750609e54602082015163ffffffff918216911610155b61196457604051632e46483160e11b815260040160405180910390fd5b80609a5f611971856118bd565b815260208082019290925260409081015f2083518154949093015163ffffffff16600160a01b026001600160c01b03199094166001600160a01b0390931692909217929092179055517f3147846ee526009000671c20380b856a633345691300f82585f90034715cf0e2906118b190849084906123ff565b5f81815260018301602052604081205415156115ed565b60605f611a0c83611b66565b6040805160208082528183019092529192505f91906020820181803683375050509182525060208101929092525090565b5f6115ed8383611b8d565b5f610a1e825490565b5f610a1e82611c70565b5f808080611a698686611c7a565b909450925050505b9250929050565b5f6115ed8383611ca3565b5f6115ed8383611cc9565b5f6115ed8383611d15565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611af5573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b19919061241a565b6001600160a01b0316336001600160a01b031614610bad5760405163794821ff60e01b815260040160405180910390fd5b5f82815260028401602052604081208290556115ea8484611a83565b5f60ff8216601f811115610a1e57604051632cd44ac360e21b815260040160405180910390fd5b5f8181526001830160205260408120548015611c67575f611baf600183612390565b85549091505f90611bc290600190612390565b9050818114611c21575f865f018281548110611be057611be06121ec565b905f5260205f200154905080875f018481548110611c0057611c006121ec565b5f918252602080832090910192909255918252600188019052604090208390555b8554869080611c3257611c32612435565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f905560019350505050610a1e565b5f915050610a1e565b5f610a1e82611a48565b5f8080611c878585611a78565b5f81815260029690960160205260409095205494959350505050565b5f825f018281548110611cb857611cb86121ec565b905f5260205f200154905092915050565b5f818152600183016020526040812054611d0e57508154600181810184555f848152602080822090930184905584548482528286019093526040902091909155610a1e565b505f610a1e565b5f81815260028301602052604081208190556115ed8383611a3d565b5f5f83601f840112611d41575f5ffd5b50813567ffffffffffffffff811115611d58575f5ffd5b6020830191508360208260051b8501011115611a71575f5ffd5b5f5f5f5f60408587031215611d85575f5ffd5b843567ffffffffffffffff811115611d9b575f5ffd5b611da787828801611d31565b909550935050602085013567ffffffffffffffff811115611dc6575f5ffd5b611dd287828801611d31565b95989497509550505050565b6001600160a01b038116811461127a575f5ffd5b803563ffffffff81168114611e05575f5ffd5b919050565b5f5f5f60608486031215611e1c575f5ffd5b8335611e2781611dde565b9250611e3560208501611df2565b929592945050506040919091013590565b5f60208284031215611e56575f5ffd5b5035919050565b5f60408284031215611e6d575f5ffd5b50919050565b5f5f60608385031215611e84575f5ffd5b611e8e8484611e5d565b91506040830135611e9e81611dde565b809150509250929050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715611ee657611ee6611ea9565b604052919050565b5f60408284031215611efe575f5ffd5b6040805190810167ffffffffffffffff81118282101715611f2157611f21611ea9565b6040529050808235611f3281611dde565b8152611f4060208401611df2565b60208201525092915050565b5f60408284031215611f5c575f5ffd5b6115ed8383611eee565b80516001600160a01b0316825260209081015163ffffffff16910152565b60408101610a1e8284611f66565b5f5f60808385031215611fa3575f5ffd5b611fad8484611e5d565b9150611fbc8460408501611e5d565b90509250929050565b5f60408284031215611fd5575f5ffd5b6115ed8383611e5d565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b602081525f6115ed6020830184611fdf565b5f6020828403121561202f575f5ffd5b813560ff811681146115ed575f5ffd5b604080825283519082018190525f9060208501906060840190835b8181101561207857835183526020938401939092019160010161205a565b5050838103602080860191909152855180835291810192508501905f5b818110156120bc5782516001600160a01b0316845260209384019390920191600101612095565b50919695505050505050565b602080825282518282018190525f918401906040840190835b8181101561210a576120f4838551611f66565b60209390930192604092909201916001016120e1565b509095945050505050565b5f5f5f60a08486031215612127575f5ffd5b6121318585611e5d565b9250604084013561214181611dde565b91506121508560608601611e5d565b90509250925092565b5f60208284031215612169575f5ffd5b6115ed82611df2565b5f5f60408385031215612183575f5ffd5b50508035926020909101359150565b5f5f602083850312156121a3575f5ffd5b823567ffffffffffffffff8111156121b9575f5ffd5b6121c585828601611d31565b90969095509350505050565b5f602082840312156121e1575f5ffd5b81356115ed81611dde565b634e487b7160e01b5f52603260045260245ffd5b803561220b81611dde565b6001600160a01b0316825263ffffffff61222760208301611df2565b1660208301525050565b60408101610a1e8284612200565b5f6020828403121561224f575f5ffd5b815180151581146115ed575f5ffd5b5f6020828403121561226e575f5ffd5b8151600381106115ed575f5ffd5b5f6020828403121561228c575f5ffd5b815167ffffffffffffffff8111156122a2575f5ffd5b8201601f810184136122b2575f5ffd5b805167ffffffffffffffff8111156122cc576122cc611ea9565b6122df601f8201601f1916602001611ebd565b8181528560208385010111156122f3575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b634e487b7160e01b5f52602160045260245ffd5b61232e8186612200565b5f6003851061234b57634e487b7160e01b5f52602160045260245ffd5b84604083015261235e6060830185611f66565b60c060a083015261237260c0830184611fdf565b9695505050505050565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610a1e57610a1e61237c565b80820180821115610a1e57610a1e61237c565b606081016123c48285611f66565b6001600160a01b039290921660409190910152919050565b80516020808301519190811015611e6d575f1960209190910360031b1b16919050565b6080810161240d8285611f66565b6115ed6040830184611f66565b5f6020828403121561242a575f5ffd5b81516115ed81611dde565b634e487b7160e01b5f52603160045260245ffdfea26469706673582212209eee65670a0acd546a18377ada41e4f6518f10b5c3d618724117493c69a324be64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xD1W_5`\xE0\x1C\x80cqP\x18\xA6\x11a\0\xFEW\x80c\xCA\x8A\xA7\xC7\x11a\0\x9EW\x80c\xD9\xA6r\x9E\x11a\0nW\x80c\xD9\xA6r\x9E\x14a\x04KW\x80c\xDF\xBD\x9D\xFD\x14a\x04^W\x80c\xF2\xFD\xE3\x8B\x14a\x04qW\x80c\xFA\xBC\x1C\xBC\x14a\x04\x84W__\xFD[\x80c\xCA\x8A\xA7\xC7\x14a\x03\xE9W\x80c\xD0\x9B\x97\x8B\x14a\x04\x10W\x80c\xD5\x04I\x11\x14a\x04%W\x80c\xD6\xDB\x9E%\x14a\x048W__\xFD[\x80c\x8D\xA5\xCB[\x11a\0\xD9W\x80c\x8D\xA5\xCB[\x14a\x03\xA2W\x80c\xACP_K\x14a\x03\xB3W\x80c\xB1\x86\xA6\x0E\x14a\x03\xCBW\x80c\xC4\xBF\xFE+\x14a\x03\xD3W__\xFD[\x80cqP\x18\xA6\x14a\x03`W\x80cu\xE4\xB59\x14a\x03hW\x80c\x88o\x11\x95\x14a\x03{W__\xFD[\x80c>\xC4\\~\x11a\x01tW\x80cY\\jg\x11a\x01DW\x80cY\\jg\x14a\x03\x10W\x80cZ\xC8j\xB7\x14a\x03\x18W\x80c\\\x97Z\xBB\x14a\x03;W\x80clU\xA3\x7F\x14a\x03MW__\xFD[\x80c>\xC4\\~\x14a\x02\x82W\x80cA\xEEm\x0E\x14a\x02\xC1W\x80cFW\xE2j\x14a\x02\xE1W\x80cT\xFDMP\x14a\x03\x08W__\xFD[\x80c\x1C\xA9\x14*\x11a\x01\xAFW\x80c\x1C\xA9\x14*\x14a\x02\x10W\x80c!\xFA\x7F\xDC\x14a\x02#W\x80c'~\x1Eb\x14a\x02LW\x80c6\xB2\0\xDE\x14a\x02_W__\xFD[\x80c\x04\xE9\x8B\xE3\x14a\x01\xD5W\x80c\x0F\x19\xAA\xEF\x14a\x01\xEAW\x80c\x13d9\xDD\x14a\x01\xFDW[__\xFD[a\x01\xE8a\x01\xE36`\x04a\x1DrV[a\x04\x97V[\0[a\x01\xE8a\x01\xF86`\x04a\x1E\nV[a\x05\xDBV[a\x01\xE8a\x02\x0B6`\x04a\x1EFV[a\x07\x01V[a\x01\xE8a\x02\x1E6`\x04a\x1EsV[a\x07;V[a\x026a\x0216`\x04a\x1FLV[a\x08kV[`@Qa\x02C\x91\x90a\x1F\x84V[`@Q\x80\x91\x03\x90\xF3[a\x01\xE8a\x02Z6`\x04a\x1F\x92V[a\x08\xCBV[a\x02ra\x02m6`\x04a\x1FLV[a\n\tV[`@Q\x90\x15\x15\x81R` \x01a\x02CV[a\x02\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02CV[a\x02\xD4a\x02\xCF6`\x04a\x1F\xC5V[a\n$V[`@Qa\x02C\x91\x90a \rV[a\x02\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xD4a\x0BkV[a\x01\xE8a\x0B\x9BV[a\x02ra\x03&6`\x04a \x1FV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fT[`@Q\x90\x81R` \x01a\x02CV[a\x01\xE8a\x03[6`\x04a\x1F\xC5V[a\x0B\xAFV[a\x01\xE8a\r\xD2V[a\x02\xA9a\x03v6`\x04a\x1FLV[a\r\xE3V[a\x02\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xA9V[`\x9ET`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02CV[a\x03?a\x0E\x0FV[a\x03\xDBa\x0E\x1AV[`@Qa\x02C\x92\x91\x90a ?V[a\x02\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\x18a\x0F0V[`@Qa\x02C\x91\x90a \xC8V[a\x01\xE8a\x0436`\x04a!\x15V[a\x10#V[a\x01\xE8a\x04F6`\x04a!YV[a\x12iV[a\x04\x18a\x04Y6`\x04a!rV[a\x12}V[a\x01\xE8a\x04l6`\x04a!\x92V[a\x13\xC7V[a\x01\xE8a\x04\x7F6`\x04a!\xD1V[a\x14mV[a\x01\xE8a\x04\x926`\x04a\x1EFV[a\x14\xE3V[a\x04\x9Fa\x15PV[`\x03a\x04\xAA\x81a\x15\xAAV[\x83\x82\x14a\x04\xCAW`@QcQ%\t\xD3`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x05\xD3W_\x86\x86\x83\x81\x81\x10a\x04\xE7Wa\x04\xE7a!\xECV[\x90P` \x02\x015\x90P\x80_\x03a\x05\x10W`@Qc=#\xE4\xD1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05D\x81\x86\x86\x85\x81\x81\x10a\x05&Wa\x05&a!\xECV[\x90P` \x02\x01` \x81\x01\x90a\x05;\x91\x90a!\xD1V[`\x9B\x91\x90a\x15\xD5V[a\x05aW`@Qc$\xBFc\x1B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7Fz\nv\xD8[X+\x17\x99m\xD77\x1A@z\xA7\xA7\x9B\x87\r\xB8S\x92G\xFB\xA3\x15\xC7\xB6\xBE\xFFb\x81\x86\x86\x85\x81\x81\x10a\x05\x95Wa\x05\x95a!\xECV[\x90P` \x02\x01` \x81\x01\x90a\x05\xAA\x91\x90a!\xD1V[`@\x80Q\x92\x83R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1P`\x01\x01a\x04\xCCV[PPPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x05\xF9WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x06\x12WP0;\x15\x80\x15a\x06\x12WP_T`\xFF\x16`\x01\x14[a\x06zW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x06\x9BW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x06\xA4\x84a\x15\xF4V[a\x06\xAD\x83a\x16EV[a\x06\xB6\x82a\x16\xB9V[\x80\x15a\x06\xFBW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[a\x07\ta\x16\xF6V[`fT\x81\x81\x16\x81\x14a\x07.W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x077\x82a\x16\xB9V[PPV[`\x01a\x07F\x81a\x15\xAAV[a\x07S` \x84\x01\x84a!\xD1V[a\x07\\\x81a\x17\x99V[a\x07yW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x84\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x07\xC7\x90\x84\x90`\x04\x01a\"1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xE2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x06\x91\x90a\"?V[a\x08#W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84a\x086a\x02m6\x83\x90\x03\x83\x01\x83a\x1FLV[a\x08SW`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xD3a\x08e6\x88\x90\x03\x88\x01\x88a\x1FLV[\x86a\x18CV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\x9A_a\x08\x8A\x84a\x18\xBDV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q\x80\x83\x01\x90\x92RT`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x92\x91PPV[`\x02a\x08\xD6\x81a\x15\xAAV[a\x08\xE3` \x84\x01\x84a!\xD1V[a\x08\xEC\x81a\x17\x99V[a\t\tW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x84\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\tW\x90\x84\x90`\x04\x01a\"1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\trW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x96\x91\x90a\"?V[a\t\xB3W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84a\t\xC6a\x02m6\x83\x90\x03\x83\x01\x83a\x1FLV[a\t\xE3W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xD3a\t\xF56\x88\x90\x03\x88\x01\x88a\x1FLV[a\n\x046\x88\x90\x03\x88\x01\x88a\x1FLV[a\x19 V[_a\n\x1Ea\n\x16\x83a\x18\xBDV[`\x97\x90a\x19\xE9V[\x92\x91PPV[``\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c|\xFF\xE4\x8C\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\ns\x91\x90a\"1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x8EW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xB2\x91\x90a\"^V[a\n\xC4a\x0216\x86\x90\x03\x86\x01\x86a\x1FLV[a\n\xD6a\x03v6\x87\x90\x03\x87\x01\x87a\x1FLV[`\x01`\x01`\xA0\x1B\x03\x16cA\xEEm\x0E\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x01\x91\x90a\"1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x1BW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0BB\x91\x90\x81\x01\x90a\"|V[`@Q` \x01a\x0BU\x94\x93\x92\x91\x90a#$V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[``a\x0B\x96\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1A\0V[\x90P\x90V[a\x0B\xA3a\x16\xF6V[a\x0B\xAD_\x19a\x16\xB9V[V[_a\x0B\xB9\x81a\x15\xAAV[a\x0B\xC6` \x83\x01\x83a!\xD1V[a\x0B\xCF\x81a\x17\x99V[a\x0B\xECW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x83\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x0C:\x90\x84\x90`\x04\x01a\"1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CUW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cy\x91\x90a\"?V[a\x0C\x96W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a\x0C\xA9a\x02m6\x83\x90\x03\x83\x01\x83a\x1FLV[a\x0C\xC6W`@QcM+\xAE\xA9`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\xDEa\x0C\xD96\x88\x90\x03\x88\x01\x88a\x1FLV[a\x18\xBDV[_\x81\x81R`\x99` R`@\x90\x81\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90UQ\x90\x91P\x7F\xD7\x81\x19\x13\xEF\xD5\xD9\x8F\xC7\xEA\r\x1F\xDD\x02+=1\x98x\x156\x08B\xD0[\x1D\x1C\xF5Ux\xD1j\x90a\r,\x90\x88\x90a\"1V[`@Q\x80\x91\x03\x90\xA1_\x81\x81R`\x9A` R`@\x90\x81\x90 \x80T`\x01`\x01`\xC0\x1B\x03\x19\x16\x90UQ\x7F!\n\x11\x18\xA8i$ab\x80N*\x7F!\xEF\x80\x8E\xBD\x93\xF4\xBE~\xD5\x12\x01O\xE2\x9Az\x8B\xE0.\x90a\r\x7F\x90\x88\x90a\"1V[`@Q\x80\x91\x03\x90\xA1a\r\x92`\x97\x82a\x1A=V[P\x7FO\xFD\xFD\xD5\x9E\x9E\x1E<0\x16\x08x\x8Fx\xDDE\x8Ea\xCB\x8C\x04\\\xA9+b\xA7\xB4\x84\xC8\x08$\xFB\x86`@Qa\r\xC2\x91\x90a\"1V[`@Q\x80\x91\x03\x90\xA1PPPPPPV[a\r\xDAa\x15PV[a\x0B\xAD_a\x15\xF4V[_`\x99_a\r\xF0\x84a\x18\xBDV[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[_a\x0B\x96`\x97a\x1AHV[``\x80_a\x0E(`\x9Ba\x1AQV[\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0EDWa\x0EDa\x1E\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0EmW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\x8AWa\x0E\x8Aa\x1E\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xB3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x0F%W_\x80a\x0E\xCD`\x9B\x84a\x1A[V[\x91P\x91P\x81\x85\x84\x81Q\x81\x10a\x0E\xE4Wa\x0E\xE4a!\xECV[` \x02` \x01\x01\x81\x81RPP\x80\x84\x84\x81Q\x81\x10a\x0F\x03Wa\x0F\x03a!\xECV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPP`\x01\x01a\x0E\xB8V[P\x90\x94\x90\x93P\x91PPV[``_a\x0F=`\x97a\x1AHV[\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0FYWa\x0FYa\x1E\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0F\x9DW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0FwW\x90P[P\x90P_[\x82\x81\x10\x15a\x10\x1CW_a\x0F\xB6`\x97\x83a\x1AxV[\x90P_a\x0F\xF2\x82`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x90P\x80\x84\x84\x81Q\x81\x10a\x10\x07Wa\x10\x07a!\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x0F\xA2V[P\x92\x91PPV[_a\x10-\x81a\x15\xAAV[a\x10:` \x85\x01\x85a!\xD1V[a\x10C\x81a\x17\x99V[a\x10`W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x04\xC1\xB8\xEB`\xE3\x1B\x81R\x85\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c&\r\xC7X\x90a\x10\xAE\x90\x84\x90`\x04\x01a\"1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xED\x91\x90a\"?V[a\x11\nW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Qc\x1F?\xF9#`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c|\xFF\xE4\x8C\x90a\x11W\x90\x8A\x90`\x04\x01a\"1V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11rW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x96\x91\x90a\"^V[`\x02\x81\x11\x15a\x11\xA7Wa\x11\xA7a#\x10V[\x03a\x11\xC5W`@Qc\xE5|\xAC\xBD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11\xE2a\x11\xDAa\x0C\xD96\x89\x90\x03\x89\x01\x89a\x1FLV[`\x97\x90a\x1A\x83V[a\x11\xFFW`@Qc\x18\x83F\x15`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7FO\xB6\xEF\xEC}\xD6\x006\xCE:z\xF8\xD5\xC4\x84%\x01\x9D\xAA\x0F\xB6\x1E\xB4q\xA9f\xA7\xAC,o\xA6\xA6\x86`@Qa\x12.\x91\x90a\"1V[`@Q\x80\x91\x03\x90\xA1a\x12Ha\x08e6\x88\x90\x03\x88\x01\x88a\x1FLV[a\x05\xD3a\x12Z6\x88\x90\x03\x88\x01\x88a\x1FLV[a\n\x046\x87\x90\x03\x87\x01\x87a\x1FLV[a\x12qa\x15PV[a\x12z\x81a\x16EV[PV[``\x81\x83\x11\x15a\x12\xA0W`@QcV\x1C\xE9\xBB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xAA`\x97a\x1AHV[\x82\x11\x15a\x12\xCAW`@Qc\x02\xDA6\x13`\xE6\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x12\xD5\x84\x84a#\x90V[\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x12\xF1Wa\x12\xF1a\x1E\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x135W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13\x0FW\x90P[P\x90P_[\x82\x81\x10\x15a\x13\xBEW_a\x13Xa\x13P\x83\x89a#\xA3V[`\x97\x90a\x1AxV[\x90P_a\x13\x94\x82`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x90P\x80\x84\x84\x81Q\x81\x10a\x13\xA9Wa\x13\xA9a!\xECV[` \x90\x81\x02\x91\x90\x91\x01\x01RPP`\x01\x01a\x13:V[P\x94\x93PPPPV[a\x13\xCFa\x15PV[`\x03a\x13\xDA\x81a\x15\xAAV[_[\x82\x81\x10\x15a\x06\xFBW_\x84\x84\x83\x81\x81\x10a\x13\xF7Wa\x13\xF7a!\xECV[\x90P` \x02\x015\x90Pa\x14\x14\x81`\x9Ba\x1A\x8E\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x141W`@Qc\xB3\xF9+\xA1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x81\x81R\x7Fh$\xD3`\x84\xEC\xF2\xCD\x81\x9B\x13|\xB5\xD87\xCCns\xAF\xCE\x1E\x0E4\x8C\x9F\xDE\xCA\xA8\x1D\x03A\xE5\x90` \x01`@Q\x80\x91\x03\x90\xA1P`\x01\x01a\x13\xDCV[a\x14ua\x15PV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x14\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06qV[a\x12z\x81a\x15\xF4V[a\x14\xEBa\x1A\x99V[`fT\x80\x19\x82\x19\x81\x16\x14a\x15\x12W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06qV[`fT`\x01`\xFF\x83\x16\x1B\x90\x81\x16\x03a\x12zW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x15\xEA\x84\x84`\x01`\x01`\xA0\x1B\x03\x85\x16a\x1BJV[\x90P[\x93\x92PPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[_\x81c\xFF\xFF\xFF\xFF\x16\x11a\x16kW`@Qc\x16\xD9\x8E\x1B`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x9E\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7FO\xBC\xD0\xCC\xA7\0\x15\xB3=\xB8\xAFJ\xA4\xF2\xBDo\xD6\xC1\xEF\xA9F\x0B\x8E#3\xF2R\xC1Fzc'\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17XW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17|\x91\x90a\"?V[a\x0B\xADW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x18\x1FW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x1E\x91\x90a\"?V[\x80`\x99_a\x18P\x85a\x18\xBDV[\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x7F\x7F|\xCA\xFD\x92\xD2\x0F\xDB9\xDE\xE1\x84\xA0\xDC\xE0\x02\xA9\xDAB\x0E\xD0\xDE\xF4a\xF2\xA0'\xAB\xC9\xB3\xF6\xDF\x82\x82`@Qa\x18\xB1\x92\x91\x90a#\xB6V[`@Q\x80\x91\x03\x90\xA1PPV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a\x19\x08\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\n\x1E\x90a#\xDCV[` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a\x19GWP`\x9ET` \x82\x01Qc\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x10\x15[a\x19dW`@Qc.FH1`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x9A_a\x19q\x85a\x18\xBDV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x83Q\x81T\x94\x90\x93\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x92\x90\x92\x17\x90UQ\x7F1G\x84n\xE5&\0\x90\0g\x1C 8\x0B\x85jc3Ei\x13\0\xF8%\x85\xF9\x004q\\\xF0\xE2\x90a\x18\xB1\x90\x84\x90\x84\x90a#\xFFV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x15\xEDV[``_a\x1A\x0C\x83a\x1BfV[`@\x80Q` \x80\x82R\x81\x83\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837PPP\x91\x82RP` \x81\x01\x92\x90\x92RP\x90V[_a\x15\xED\x83\x83a\x1B\x8DV[_a\n\x1E\x82T\x90V[_a\n\x1E\x82a\x1CpV[_\x80\x80\x80a\x1Ai\x86\x86a\x1CzV[\x90\x94P\x92PPP[\x92P\x92\x90PV[_a\x15\xED\x83\x83a\x1C\xA3V[_a\x15\xED\x83\x83a\x1C\xC9V[_a\x15\xED\x83\x83a\x1D\x15V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xF5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\x19\x91\x90a$\x1AV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xADW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82\x81R`\x02\x84\x01` R`@\x81 \x82\x90Ua\x15\xEA\x84\x84a\x1A\x83V[_`\xFF\x82\x16`\x1F\x81\x11\x15a\n\x1EW`@Qc,\xD4J\xC3`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a\x1CgW_a\x1B\xAF`\x01\x83a#\x90V[\x85T\x90\x91P_\x90a\x1B\xC2\x90`\x01\x90a#\x90V[\x90P\x81\x81\x14a\x1C!W_\x86_\x01\x82\x81T\x81\x10a\x1B\xE0Wa\x1B\xE0a!\xECV[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a\x1C\0Wa\x1C\0a!\xECV[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a\x1C2Wa\x1C2a$5V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\n\x1EV[_\x91PPa\n\x1EV[_a\n\x1E\x82a\x1AHV[_\x80\x80a\x1C\x87\x85\x85a\x1AxV[_\x81\x81R`\x02\x96\x90\x96\x01` R`@\x90\x95 T\x94\x95\x93PPPPV[_\x82_\x01\x82\x81T\x81\x10a\x1C\xB8Wa\x1C\xB8a!\xECV[\x90_R` _ \x01T\x90P\x92\x91PPV[_\x81\x81R`\x01\x83\x01` R`@\x81 Ta\x1D\x0EWP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\n\x1EV[P_a\n\x1EV[_\x81\x81R`\x02\x83\x01` R`@\x81 \x81\x90Ua\x15\xED\x83\x83a\x1A=V[__\x83`\x1F\x84\x01\x12a\x1DAW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DXW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1AqW__\xFD[____`@\x85\x87\x03\x12\x15a\x1D\x85W__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\x9BW__\xFD[a\x1D\xA7\x87\x82\x88\x01a\x1D1V[\x90\x95P\x93PP` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1D\xC6W__\xFD[a\x1D\xD2\x87\x82\x88\x01a\x1D1V[\x95\x98\x94\x97P\x95PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12zW__\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1E\x05W__\xFD[\x91\x90PV[___``\x84\x86\x03\x12\x15a\x1E\x1CW__\xFD[\x835a\x1E'\x81a\x1D\xDEV[\x92Pa\x1E5` \x85\x01a\x1D\xF2V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a\x1EVW__\xFD[P5\x91\x90PV[_`@\x82\x84\x03\x12\x15a\x1EmW__\xFD[P\x91\x90PV[__``\x83\x85\x03\x12\x15a\x1E\x84W__\xFD[a\x1E\x8E\x84\x84a\x1E]V[\x91P`@\x83\x015a\x1E\x9E\x81a\x1D\xDEV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1E\xE6Wa\x1E\xE6a\x1E\xA9V[`@R\x91\x90PV[_`@\x82\x84\x03\x12\x15a\x1E\xFEW__\xFD[`@\x80Q\x90\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1F!Wa\x1F!a\x1E\xA9V[`@R\x90P\x80\x825a\x1F2\x81a\x1D\xDEV[\x81Ra\x1F@` \x84\x01a\x1D\xF2V[` \x82\x01RP\x92\x91PPV[_`@\x82\x84\x03\x12\x15a\x1F\\W__\xFD[a\x15\xED\x83\x83a\x1E\xEEV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`@\x81\x01a\n\x1E\x82\x84a\x1FfV[__`\x80\x83\x85\x03\x12\x15a\x1F\xA3W__\xFD[a\x1F\xAD\x84\x84a\x1E]V[\x91Pa\x1F\xBC\x84`@\x85\x01a\x1E]V[\x90P\x92P\x92\x90PV[_`@\x82\x84\x03\x12\x15a\x1F\xD5W__\xFD[a\x15\xED\x83\x83a\x1E]V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x15\xED` \x83\x01\x84a\x1F\xDFV[_` \x82\x84\x03\x12\x15a /W__\xFD[\x815`\xFF\x81\x16\x81\x14a\x15\xEDW__\xFD[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a xW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a ZV[PP\x83\x81\x03` \x80\x86\x01\x91\x90\x91R\x85Q\x80\x83R\x91\x81\x01\x92P\x85\x01\x90_[\x81\x81\x10\x15a \xBCW\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a \x95V[P\x91\x96\x95PPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a!\nWa \xF4\x83\x85Qa\x1FfV[` \x93\x90\x93\x01\x92`@\x92\x90\x92\x01\x91`\x01\x01a \xE1V[P\x90\x95\x94PPPPPV[___`\xA0\x84\x86\x03\x12\x15a!'W__\xFD[a!1\x85\x85a\x1E]V[\x92P`@\x84\x015a!A\x81a\x1D\xDEV[\x91Pa!P\x85``\x86\x01a\x1E]V[\x90P\x92P\x92P\x92V[_` \x82\x84\x03\x12\x15a!iW__\xFD[a\x15\xED\x82a\x1D\xF2V[__`@\x83\x85\x03\x12\x15a!\x83W__\xFD[PP\x805\x92` \x90\x91\x015\x91PV[__` \x83\x85\x03\x12\x15a!\xA3W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xB9W__\xFD[a!\xC5\x85\x82\x86\x01a\x1D1V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15a!\xE1W__\xFD[\x815a\x15\xED\x81a\x1D\xDEV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x805a\"\x0B\x81a\x1D\xDEV[`\x01`\x01`\xA0\x1B\x03\x16\x82Rc\xFF\xFF\xFF\xFFa\"'` \x83\x01a\x1D\xF2V[\x16` \x83\x01RPPV[`@\x81\x01a\n\x1E\x82\x84a\"\0V[_` \x82\x84\x03\x12\x15a\"OW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x15\xEDW__\xFD[_` \x82\x84\x03\x12\x15a\"nW__\xFD[\x81Q`\x03\x81\x10a\x15\xEDW__\xFD[_` \x82\x84\x03\x12\x15a\"\x8CW__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xA2W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\"\xB2W__\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xCCWa\"\xCCa\x1E\xA9V[a\"\xDF`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1E\xBDV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a\"\xF3W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[a#.\x81\x86a\"\0V[_`\x03\x85\x10a#KWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x84`@\x83\x01Ra#^``\x83\x01\x85a\x1FfV[`\xC0`\xA0\x83\x01Ra#r`\xC0\x83\x01\x84a\x1F\xDFV[\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\n\x1EWa\n\x1Ea#|V[\x80\x82\x01\x80\x82\x11\x15a\n\x1EWa\n\x1Ea#|V[``\x81\x01a#\xC4\x82\x85a\x1FfV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`@\x91\x90\x91\x01R\x91\x90PV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x1EmW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\x80\x81\x01a$\r\x82\x85a\x1FfV[a\x15\xED`@\x83\x01\x84a\x1FfV[_` \x82\x84\x03\x12\x15a$*W__\xFD[\x81Qa\x15\xED\x81a\x1D\xDEV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD\xFE\xA2dipfsX\"\x12 \x9E\xEEeg\n\n\xCDTj\x187z\xDAA\xE4\xF6Q\x8F\x10\xB5\xC3\xD6\x18rA\x17I<i\xA3$\xBEdsolcC\0\x08\x1B\x003",
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
    /**Function with signature `initialize(address,uint32,uint256)` and selector `0x0f19aaef`.
    ```solidity
    function initialize(address initialOwner, uint32 initialTableUpdateCadence, uint256 initialPausedStatus) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub initialOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub initialTableUpdateCadence: u32,
        #[allow(missing_docs)]
        pub initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`initialize(address,uint32,uint256)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value.initialOwner,
                        value.initialTableUpdateCadence,
                        value.initialPausedStatus,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        initialTableUpdateCadence: tuple.1,
                        initialPausedStatus: tuple.2,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,uint32,uint256)";
            const SELECTOR: [u8; 4] = [15u8, 25u8, 170u8, 239u8];
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.initialTableUpdateCadence,
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
        allocationManager(allocationManagerCall),
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
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        setOperatorSetConfig(setOperatorSetConfigCall),
        #[allow(missing_docs)]
        setOperatorTableCalculator(setOperatorTableCalculatorCall),
        #[allow(missing_docs)]
        setTableUpdateCadence(setTableUpdateCadenceCall),
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
            [15u8, 25u8, 170u8, 239u8],
            [19u8, 100u8, 57u8, 221u8],
            [28u8, 169u8, 20u8, 42u8],
            [33u8, 250u8, 127u8, 220u8],
            [39u8, 126u8, 30u8, 98u8],
            [54u8, 178u8, 0u8, 222u8],
            [62u8, 196u8, 92u8, 126u8],
            [65u8, 238u8, 109u8, 14u8],
            [70u8, 87u8, 226u8, 106u8],
            [84u8, 253u8, 77u8, 80u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [108u8, 85u8, 163u8, 127u8],
            [113u8, 80u8, 24u8, 166u8],
            [117u8, 228u8, 181u8, 57u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [172u8, 80u8, 95u8, 75u8],
            [177u8, 134u8, 166u8, 14u8],
            [196u8, 191u8, 254u8, 43u8],
            [202u8, 138u8, 167u8, 199u8],
            [208u8, 155u8, 151u8, 139u8],
            [213u8, 4u8, 73u8, 17u8],
            [214u8, 219u8, 158u8, 37u8],
            [217u8, 166u8, 114u8, 158u8],
            [223u8, 189u8, 157u8, 253u8],
            [242u8, 253u8, 227u8, 139u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for CrossChainRegistryCalls {
        const NAME: &'static str = "CrossChainRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 30usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addChainIDsToWhitelist(_) => {
                    <addChainIDsToWhitelistCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::keyRegistrar(_) => {
                    <keyRegistrarCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn initialize(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::initialize)
                    }
                    initialize
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
                    fn hasActiveGenerationReservation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <hasActiveGenerationReservationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(CrossChainRegistryCalls::hasActiveGenerationReservation)
                    }
                    hasActiveGenerationReservation
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
                    fn getTableUpdateCadence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(CrossChainRegistryCalls::getTableUpdateCadence)
                    }
                    getTableUpdateCadence
                },
                {
                    fn getActiveGenerationReservationCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getActiveGenerationReservationCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                CrossChainRegistryCalls::getActiveGenerationReservationCount,
                            )
                    }
                    getActiveGenerationReservationCount
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
                {
                    fn setTableUpdateCadence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <setTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data,
                        )
                        .map(CrossChainRegistryCalls::setTableUpdateCadence)
                    }
                    setTableUpdateCadence
                },
                {
                    fn getActiveGenerationReservationsByRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getActiveGenerationReservationsByRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                            )
                            .map(
                                CrossChainRegistryCalls::getActiveGenerationReservationsByRange,
                            )
                    }
                    getActiveGenerationReservationsByRange
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
                    fn unpause(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                            .map(CrossChainRegistryCalls::unpause)
                    }
                    unpause
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
                    fn initialize(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryCalls::initialize)
                    }
                    initialize
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
                    fn hasActiveGenerationReservation(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <hasActiveGenerationReservationCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::hasActiveGenerationReservation)
                    }
                    hasActiveGenerationReservation
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
                    fn getTableUpdateCadence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::getTableUpdateCadence)
                    }
                    getTableUpdateCadence
                },
                {
                    fn getActiveGenerationReservationCount(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getActiveGenerationReservationCountCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                CrossChainRegistryCalls::getActiveGenerationReservationCount,
                            )
                    }
                    getActiveGenerationReservationCount
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
                {
                    fn setTableUpdateCadence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <setTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryCalls::setTableUpdateCadence)
                    }
                    setTableUpdateCadence
                },
                {
                    fn getActiveGenerationReservationsByRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <getActiveGenerationReservationsByRangeCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                data,
                            )
                            .map(
                                CrossChainRegistryCalls::getActiveGenerationReservationsByRange,
                            )
                    }
                    getActiveGenerationReservationsByRange
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
                    fn unpause(data: &[u8]) -> alloy_sol_types::Result<CrossChainRegistryCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryCalls::unpause)
                    }
                    unpause
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
                Self::setTableUpdateCadence(inner) => {
                    <setTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setTableUpdateCadence(inner) => {
                    <setTableUpdateCadenceCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        InvalidEndIndex(InvalidEndIndex),
        #[allow(missing_docs)]
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        #[allow(missing_docs)]
        InvalidOperatorSet(InvalidOperatorSet),
        #[allow(missing_docs)]
        InvalidPermissions(InvalidPermissions),
        #[allow(missing_docs)]
        InvalidRange(InvalidRange),
        #[allow(missing_docs)]
        InvalidShortString(InvalidShortString),
        #[allow(missing_docs)]
        InvalidStalenessPeriod(InvalidStalenessPeriod),
        #[allow(missing_docs)]
        InvalidTableUpdateCadence(InvalidTableUpdateCadence),
        #[allow(missing_docs)]
        KeyTypeNotSet(KeyTypeNotSet),
        #[allow(missing_docs)]
        OnlyPauser(OnlyPauser),
        #[allow(missing_docs)]
        OnlyUnpauser(OnlyUnpauser),
        #[allow(missing_docs)]
        StringTooLong(StringTooLong),
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
            [73u8, 126u8, 198u8, 54u8],
            [86u8, 28u8, 233u8, 187u8],
            [92u8, 140u8, 144u8, 98u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [122u8, 71u8, 201u8, 162u8],
            [126u8, 197u8, 193u8, 84u8],
            [132u8, 10u8, 72u8, 213u8],
            [134u8, 49u8, 160u8, 117u8],
            [147u8, 45u8, 148u8, 247u8],
            [154u8, 87u8, 93u8, 82u8],
            [162u8, 74u8, 19u8, 166u8],
            [179u8, 81u8, 43u8, 12u8],
            [179u8, 249u8, 43u8, 161u8],
            [182u8, 141u8, 132u8, 192u8],
            [182u8, 204u8, 112u8, 216u8],
            [198u8, 29u8, 202u8, 93u8],
            [229u8, 124u8, 172u8, 189u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for CrossChainRegistryErrors {
        const NAME: &'static str = "CrossChainRegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 21usize;
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
                Self::InvalidEndIndex(_) => {
                    <InvalidEndIndex as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOperatorSet(_) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPermissions(_) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRange(_) => <InvalidRange as alloy_sol_types::SolError>::SELECTOR,
                Self::InvalidShortString(_) => {
                    <InvalidShortString as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidStalenessPeriod(_) => {
                    <InvalidStalenessPeriod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTableUpdateCadence(_) => {
                    <InvalidTableUpdateCadence as alloy_sol_types::SolError>::SELECTOR
                }
                Self::KeyTypeNotSet(_) => <KeyTypeNotSet as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
                Self::StringTooLong(_) => <StringTooLong as alloy_sol_types::SolError>::SELECTOR,
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
                    fn InvalidRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidRange as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::InvalidRange)
                    }
                    InvalidRange
                },
                {
                    fn InvalidStalenessPeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidStalenessPeriod as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::InvalidStalenessPeriod)
                    }
                    InvalidStalenessPeriod
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
                    fn InvalidEndIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidEndIndex as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::InvalidEndIndex)
                    }
                    InvalidEndIndex
                },
                {
                    fn InvalidTableUpdateCadence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidTableUpdateCadence as alloy_sol_types::SolError>::abi_decode_raw(
                            data,
                        )
                        .map(CrossChainRegistryErrors::InvalidTableUpdateCadence)
                    }
                    InvalidTableUpdateCadence
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
                {
                    fn KeyTypeNotSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <KeyTypeNotSet as alloy_sol_types::SolError>::abi_decode_raw(data)
                            .map(CrossChainRegistryErrors::KeyTypeNotSet)
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
                    fn InvalidRange(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidRange as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryErrors::InvalidRange)
                    }
                    InvalidRange
                },
                {
                    fn InvalidStalenessPeriod(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidStalenessPeriod as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryErrors::InvalidStalenessPeriod)
                    }
                    InvalidStalenessPeriod
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
                    fn InvalidEndIndex(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidEndIndex as alloy_sol_types::SolError>::abi_decode_raw_validate(
                            data,
                        )
                        .map(CrossChainRegistryErrors::InvalidEndIndex)
                    }
                    InvalidEndIndex
                },
                {
                    fn InvalidTableUpdateCadence(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <InvalidTableUpdateCadence as alloy_sol_types::SolError>::abi_decode_raw_validate(
                                data,
                            )
                            .map(CrossChainRegistryErrors::InvalidTableUpdateCadence)
                    }
                    InvalidTableUpdateCadence
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
                {
                    fn KeyTypeNotSet(
                        data: &[u8],
                    ) -> alloy_sol_types::Result<CrossChainRegistryErrors> {
                        <KeyTypeNotSet as alloy_sol_types::SolError>::abi_decode_raw_validate(data)
                            .map(CrossChainRegistryErrors::KeyTypeNotSet)
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
                Self::InvalidEndIndex(inner) => {
                    <InvalidEndIndex as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InvalidRange(inner) => {
                    <InvalidRange as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidShortString(inner) => {
                    <InvalidShortString as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
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
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::StringTooLong(inner) => {
                    <StringTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InvalidEndIndex(inner) => {
                    <InvalidEndIndex as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidRange(inner) => {
                    <InvalidRange as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encode_raw(
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
        TableUpdateCadenceSet(TableUpdateCadenceSet),
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
                215u8, 129u8, 25u8, 19u8, 239u8, 213u8, 217u8, 143u8, 199u8, 234u8, 13u8, 31u8,
                221u8, 2u8, 43u8, 61u8, 49u8, 152u8, 120u8, 21u8, 54u8, 8u8, 66u8, 208u8, 91u8,
                29u8, 28u8, 245u8, 85u8, 120u8, 209u8, 106u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for CrossChainRegistryEvents {
        const NAME: &'static str = "CrossChainRegistryEvents";
        const COUNT: usize = 13usize;
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
                Some(<TableUpdateCadenceSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TableUpdateCadenceSet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data,
                    )
                    .map(Self::TableUpdateCadenceSet)
                }
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
                Self::TableUpdateCadenceSet(inner) => {
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
                Self::TableUpdateCadenceSet(inner) => {
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
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            initialTableUpdateCadence: u32,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<&P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                initialTableUpdateCadence,
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
        ///Creates a new call builder for the [`setTableUpdateCadence`] function.
        pub fn setTableUpdateCadence(
            &self,
            tableUpdateCadence: u32,
        ) -> alloy_contract::SolCallBuilder<&P, setTableUpdateCadenceCall, N> {
            self.call_builder(&setTableUpdateCadenceCall { tableUpdateCadence })
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
        ///Creates a new event filter for the [`TableUpdateCadenceSet`] event.
        pub fn TableUpdateCadenceSet_filter(
            &self,
        ) -> alloy_contract::Event<&P, TableUpdateCadenceSet, N> {
            self.event_filter::<TableUpdateCadenceSet>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<&P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
