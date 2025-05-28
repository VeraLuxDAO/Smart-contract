/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/veralux.json`.
 */
export type Veralux = {
  "address": "B7ey9pqVRrtFWDWLWuDr9VxD1VWdn1YviVg74vw4cPwq",
  "metadata": {
    "name": "veralux",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addWhitelist",
      "discriminator": [
        215,
        46,
        143,
        176,
        108,
        113,
        24,
        1
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "global",
          "writable": true
        },
        {
          "name": "multisig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  117,
                  108,
                  116,
                  105,
                  115,
                  105,
                  103,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "payer"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "whitelist",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "buyPresale",
      "docs": [
        "Preslae"
      ],
      "discriminator": [
        113,
        18,
        193,
        68,
        35,
        36,
        215,
        8
      ],
      "accounts": [
        {
          "name": "buyer",
          "writable": true,
          "signer": true
        },
        {
          "name": "global",
          "writable": true
        },
        {
          "name": "presalePurchase",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  45,
                  112,
                  117,
                  114,
                  99,
                  104,
                  97,
                  115,
                  101,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "buyer"
              }
            ]
          }
        },
        {
          "name": "presaleVesting",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  115,
                  97,
                  108,
                  101,
                  45,
                  118,
                  101,
                  115,
                  116,
                  105,
                  110,
                  103,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "buyer"
              }
            ]
          }
        },
        {
          "name": "buyerUsdcAccount",
          "writable": true
        },
        {
          "name": "adminUsdcAccount",
          "writable": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "usdcAmount",
          "type": "u64"
        },
        {
          "name": "kycVerified",
          "type": "bool"
        }
      ]
    },
    {
      "name": "confirmMultisig",
      "discriminator": [
        165,
        28,
        156,
        38,
        215,
        147,
        111,
        196
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "global",
          "writable": true
        },
        {
          "name": "multisig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  117,
                  108,
                  116,
                  105,
                  115,
                  105,
                  103,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "pendingMultisig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  101,
                  110,
                  100,
                  105,
                  110,
                  103,
                  45,
                  109,
                  117,
                  108,
                  116,
                  105,
                  115,
                  105,
                  103,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "executeProposal",
      "discriminator": [
        186,
        60,
        116,
        133,
        108,
        128,
        111,
        28
      ],
      "accounts": [
        {
          "name": "global",
          "writable": true
        },
        {
          "name": "proposal",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "initGlobal",
      "docs": [
        "Global"
      ],
      "discriminator": [
        44,
        238,
        77,
        253,
        76,
        182,
        192,
        162
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "global",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  103,
                  108,
                  111,
                  98,
                  97,
                  108,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "payer"
              }
            ]
          }
        },
        {
          "name": "multisig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  117,
                  108,
                  116,
                  105,
                  115,
                  105,
                  103,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "payer"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "initMultisig",
      "docs": [
        "Multisig"
      ],
      "discriminator": [
        119,
        130,
        22,
        116,
        114,
        61,
        124,
        66
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "global",
          "writable": true
        },
        {
          "name": "multisig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  117,
                  108,
                  116,
                  105,
                  115,
                  105,
                  103,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "pendingMultisig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  101,
                  110,
                  100,
                  105,
                  110,
                  103,
                  45,
                  109,
                  117,
                  108,
                  116,
                  105,
                  115,
                  105,
                  103,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "owners",
          "type": {
            "vec": "pubkey"
          }
        },
        {
          "name": "threshold",
          "type": "u8"
        }
      ]
    },
    {
      "name": "startPresale",
      "discriminator": [
        57,
        19,
        73,
        191,
        195,
        254,
        45,
        223
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "global",
          "writable": true
        },
        {
          "name": "multisig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  117,
                  108,
                  116,
                  105,
                  115,
                  105,
                  103,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "payer"
              }
            ]
          }
        }
      ],
      "args": []
    },
    {
      "name": "submitProposal",
      "docs": [
        "Governance"
      ],
      "discriminator": [
        224,
        38,
        210,
        52,
        167,
        150,
        221,
        150
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true
        },
        {
          "name": "global",
          "writable": true
        },
        {
          "name": "multisig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  117,
                  108,
                  116,
                  105,
                  115,
                  105,
                  103,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              }
            ]
          }
        },
        {
          "name": "proposal",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  111,
                  112,
                  111,
                  115,
                  97,
                  108,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "signer"
              },
              {
                "kind": "account",
                "path": "global.proposal_count",
                "account": "globalState"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "ix",
          "type": {
            "defined": {
              "name": "proposalIx"
            }
          }
        }
      ]
    },
    {
      "name": "updateGlobal",
      "discriminator": [
        90,
        152,
        240,
        21,
        199,
        38,
        72,
        20
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "global",
          "writable": true
        },
        {
          "name": "multisig",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  117,
                  108,
                  116,
                  105,
                  115,
                  105,
                  103,
                  45,
                  97,
                  117,
                  116,
                  104,
                  111,
                  114,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "payer"
              }
            ]
          }
        }
      ],
      "args": [
        {
          "name": "ix",
          "type": {
            "defined": {
              "name": "globalIx"
            }
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "globalState",
      "discriminator": [
        163,
        46,
        74,
        168,
        216,
        123,
        133,
        98
      ]
    },
    {
      "name": "multisigState",
      "discriminator": [
        63,
        19,
        203,
        218,
        31,
        61,
        159,
        8
      ]
    },
    {
      "name": "pendingMultisigState",
      "discriminator": [
        183,
        153,
        92,
        79,
        10,
        81,
        200,
        157
      ]
    },
    {
      "name": "presalePurchase",
      "discriminator": [
        34,
        110,
        127,
        240,
        83,
        219,
        152,
        227
      ]
    },
    {
      "name": "presaleVesting",
      "discriminator": [
        97,
        146,
        20,
        99,
        2,
        184,
        221,
        191
      ]
    },
    {
      "name": "proposalState",
      "discriminator": [
        251,
        209,
        129,
        206,
        222,
        199,
        248,
        29
      ]
    }
  ],
  "events": [
    {
      "name": "globalUpdatedEvent",
      "discriminator": [
        238,
        115,
        218,
        227,
        170,
        114,
        63,
        209
      ]
    },
    {
      "name": "multisigConfirmedEvent",
      "discriminator": [
        18,
        227,
        108,
        78,
        71,
        196,
        227,
        224
      ]
    },
    {
      "name": "multisigPendingEvent",
      "discriminator": [
        56,
        87,
        61,
        145,
        139,
        70,
        25,
        35
      ]
    },
    {
      "name": "presalePurchaseEvent",
      "discriminator": [
        30,
        12,
        195,
        203,
        44,
        187,
        164,
        135
      ]
    },
    {
      "name": "proposalSubmittedEvent",
      "discriminator": [
        210,
        156,
        179,
        54,
        84,
        52,
        115,
        4
      ]
    },
    {
      "name": "startedPresaleEvent",
      "discriminator": [
        173,
        101,
        34,
        163,
        102,
        102,
        69,
        72
      ]
    },
    {
      "name": "whitelistAddedEvent",
      "discriminator": [
        123,
        13,
        187,
        218,
        198,
        10,
        192,
        230
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "paused",
      "msg": "Contract is paused"
    },
    {
      "code": 6001,
      "name": "invalidAdmin",
      "msg": "Invalid global admin"
    },
    {
      "code": 6002,
      "name": "whitelistFull",
      "msg": "The whitelist list is full"
    },
    {
      "code": 6003,
      "name": "alreadyWhitelisted",
      "msg": "This memeber already in whitelist"
    },
    {
      "code": 6004,
      "name": "alreadyUpdatedGlobal",
      "msg": "Couldn't update global"
    },
    {
      "code": 6005,
      "name": "presaleNotActive",
      "msg": "Presale is not active"
    },
    {
      "code": 6006,
      "name": "invalidPurchaseAmount",
      "msg": "Presale purchase amount must be greater than zero."
    },
    {
      "code": 6007,
      "name": "descriptionTooLong",
      "msg": "Description too long"
    },
    {
      "code": 6008,
      "name": "tooManyProposalValues",
      "msg": "Too many proposal values"
    },
    {
      "code": 6009,
      "name": "proposalAlreadyExecuted",
      "msg": "Proposal has already been executed"
    },
    {
      "code": 6010,
      "name": "votingPeriodNotEnded",
      "msg": "Voting period has not ended"
    },
    {
      "code": 6011,
      "name": "noticePeriodNotMet",
      "msg": "Notice period for proposal execution not met"
    },
    {
      "code": 6012,
      "name": "reentrancyGuardTriggered",
      "msg": "Reentrancy guard triggered: Operation already in progress"
    },
    {
      "code": 6013,
      "name": "unauthorizedMultisig",
      "msg": "Unauthorized: Multisig admin is not the first owner"
    },
    {
      "code": 6014,
      "name": "insufficientSigners",
      "msg": "Unauthorized: Insufficient signers for multisig operation"
    },
    {
      "code": 6015,
      "name": "signerNotOwner",
      "msg": "Unauthorized: Signer is not a multisig owner"
    },
    {
      "code": 6016,
      "name": "buyerNotOwner",
      "msg": "Unauthorized: Buyer is not a account owner"
    },
    {
      "code": 6017,
      "name": "notAdminUsdtOwner",
      "msg": "Unauthorized: USDT account owner is not a admin"
    },
    {
      "code": 6018,
      "name": "timeLockNotMet",
      "msg": "Time lock requirement not met"
    },
    {
      "code": 6019,
      "name": "invalidThreshold",
      "msg": "Invalid threshold"
    },
    {
      "code": 6020,
      "name": "tooFewOwners",
      "msg": "Too few owners in multisig"
    },
    {
      "code": 6021,
      "name": "notEnoughUsdtBuyer",
      "msg": "Not enough usdt amount in buyer"
    },
    {
      "code": 6022,
      "name": "presaleSupplyExceeded",
      "msg": "Presale supply exceeded"
    },
    {
      "code": 6023,
      "name": "presaleMaxPerWalletExceeded",
      "msg": "Presale maximum per wallet exceeded"
    },
    {
      "code": 6024,
      "name": "invalidAuthority",
      "msg": "Invalid authority"
    },
    {
      "code": 6025,
      "name": "presaleAlreadyInitialized",
      "msg": "Presale already initialized"
    },
    {
      "code": 6026,
      "name": "invalidTreasury",
      "msg": "Invalid treasury"
    },
    {
      "code": 6027,
      "name": "arithmeticOverflow",
      "msg": "Arithmetic overflow"
    },
    {
      "code": 6028,
      "name": "kycRequired",
      "msg": "KYC required"
    },
    {
      "code": 6029,
      "name": "invalidUser",
      "msg": "Invalid user"
    },
    {
      "code": 6030,
      "name": "presaleNotStarted",
      "msg": "Presale not started"
    },
    {
      "code": 6031,
      "name": "presaleAlreadyClaimed",
      "msg": "Presale already claimed"
    },
    {
      "code": 6032,
      "name": "nothingToClaim",
      "msg": "Nothing to claim"
    },
    {
      "code": 6033,
      "name": "invalidUserTokenAccount",
      "msg": "Invalid user token account"
    },
    {
      "code": 6034,
      "name": "presaleEnded",
      "msg": "Presale ended"
    },
    {
      "code": 6035,
      "name": "invalidTreasuryTokenAccount",
      "msg": "Invalid treasury token account"
    },
    {
      "code": 6036,
      "name": "invalidOwnersCount",
      "msg": "Invalid owners count"
    },
    {
      "code": 6037,
      "name": "invalidMultisigAdmin",
      "msg": "Invalid multisig admin"
    },
    {
      "code": 6038,
      "name": "stakingAmountZero",
      "msg": "Staking amount is zero"
    },
    {
      "code": 6039,
      "name": "invalidStakingTier",
      "msg": "Invalid staking tier"
    },
    {
      "code": 6040,
      "name": "unauthorized",
      "msg": "unauthorized"
    }
  ],
  "types": [
    {
      "name": "globalIx",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "teamWallet",
            "type": "pubkey"
          },
          {
            "name": "treasuryWallet",
            "type": "pubkey"
          },
          {
            "name": "lpWallet",
            "type": "pubkey"
          },
          {
            "name": "charityWallet",
            "type": "pubkey"
          },
          {
            "name": "launchTimestamp",
            "type": "i64"
          },
          {
            "name": "initialOwners",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "threshold",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "globalState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "adminWallet",
            "type": "pubkey"
          },
          {
            "name": "teamWallet",
            "type": "pubkey"
          },
          {
            "name": "treasuryWallet",
            "type": "pubkey"
          },
          {
            "name": "lpWallet",
            "type": "pubkey"
          },
          {
            "name": "charityWallet",
            "type": "pubkey"
          },
          {
            "name": "launchTimestamp",
            "type": "i64"
          },
          {
            "name": "totalPublicPresaleSold",
            "type": "u64"
          },
          {
            "name": "totalPrivatePresaleSold",
            "type": "u64"
          },
          {
            "name": "presaleActive",
            "type": "bool"
          },
          {
            "name": "proposalCount",
            "type": "u32"
          },
          {
            "name": "totalBotingPower",
            "type": "u64"
          },
          {
            "name": "taxRate",
            "type": "u64"
          },
          {
            "name": "whitelist",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "pauseReason",
            "type": "bytes"
          },
          {
            "name": "isProcessing",
            "type": "bool"
          },
          {
            "name": "paused",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "globalUpdatedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "launchTimestamp",
            "type": "i64"
          },
          {
            "name": "threshold",
            "type": "u8"
          },
          {
            "name": "initialOwners",
            "type": {
              "vec": "pubkey"
            }
          }
        ]
      }
    },
    {
      "name": "multisigConfirmedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "threshold",
            "type": "u8"
          },
          {
            "name": "ownerCount",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "multisigPendingEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "initiationTime",
            "type": "i64"
          },
          {
            "name": "threshold",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "multisigState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owners",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "threshold",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "pendingMultisigState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "newOwners",
            "type": {
              "vec": "pubkey"
            }
          },
          {
            "name": "newThreshold",
            "type": "u8"
          },
          {
            "name": "initiationTime",
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "presalePurchase",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "pubkey"
          },
          {
            "name": "totalPurchased",
            "type": "u64"
          },
          {
            "name": "totalPrivatePurchased",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "presalePurchaseEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "buyer",
            "type": "pubkey"
          },
          {
            "name": "usdcAmount",
            "type": "u64"
          },
          {
            "name": "tokenAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "presaleVesting",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalAmount",
            "type": "u64"
          },
          {
            "name": "claimedAmount",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "proposalIx",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposalValues",
            "type": {
              "vec": "u64"
            }
          },
          {
            "name": "description",
            "type": "bytes"
          },
          {
            "name": "proposalType",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "proposalState",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "u32"
          },
          {
            "name": "startTime",
            "type": "i64"
          },
          {
            "name": "endTime",
            "type": "i64"
          },
          {
            "name": "executionTime",
            "type": "i64"
          },
          {
            "name": "votesFor",
            "type": "u64"
          },
          {
            "name": "votesAgainst",
            "type": "u64"
          },
          {
            "name": "proposalValues",
            "type": {
              "vec": "u64"
            }
          },
          {
            "name": "description",
            "type": "bytes"
          },
          {
            "name": "proposalType",
            "type": "u8"
          },
          {
            "name": "status",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "proposalSubmittedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposalId",
            "type": "u32"
          },
          {
            "name": "proposalType",
            "type": "u8"
          },
          {
            "name": "description",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "startedPresaleEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "startedPresale",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "whitelistAddedEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "address",
            "type": "pubkey"
          },
          {
            "name": "totalWhitelisted",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
