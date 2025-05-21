/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/veralux.json`.
 */
export type Veralux = {
  "address": "FVrTj1gzeyFBMnQsMXTQUYvtXqU8m6cJtdk3VFeSfHf1",
  "metadata": {
    "name": "veralux",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
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
          "name": "veraluxTokenMint",
          "writable": true
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
      "name": "descriptionTooLong",
      "msg": "Description too long"
    },
    {
      "code": 6003,
      "name": "tooManyProposalValues",
      "msg": "Too many proposal values"
    },
    {
      "code": 6004,
      "name": "reentrancyGuardTriggered",
      "msg": "Reentrancy guard triggered: Operation already in progress"
    },
    {
      "code": 6005,
      "name": "unauthorizedMultisig",
      "msg": "Unauthorized: Multisig admin is not the first owner"
    },
    {
      "code": 6006,
      "name": "insufficientSigners",
      "msg": "Unauthorized: Insufficient signers for multisig operation"
    },
    {
      "code": 6007,
      "name": "signerNotOwner",
      "msg": "Unauthorized: Signer is not a multisig owner"
    },
    {
      "code": 6008,
      "name": "timeLockNotMet",
      "msg": "Time lock requirement not met"
    },
    {
      "code": 6009,
      "name": "invalidThreshold",
      "msg": "Invalid threshold"
    },
    {
      "code": 6010,
      "name": "tooFewOwners",
      "msg": "Too few owners in multisig"
    },
    {
      "code": 6011,
      "name": "invalidAuthority",
      "msg": "Invalid authority"
    },
    {
      "code": 6012,
      "name": "presaleAlreadyInitialized",
      "msg": "Presale already initialized"
    },
    {
      "code": 6013,
      "name": "presaleNotActive",
      "msg": "Presale not active"
    },
    {
      "code": 6014,
      "name": "invalidTreasury",
      "msg": "Invalid treasury"
    },
    {
      "code": 6015,
      "name": "arithmeticOverflow",
      "msg": "Arithmetic overflow"
    },
    {
      "code": 6016,
      "name": "presaleSupplyExceeded",
      "msg": "Presale supply exceeded"
    },
    {
      "code": 6017,
      "name": "kycRequired",
      "msg": "KYC required"
    },
    {
      "code": 6018,
      "name": "invalidUser",
      "msg": "Invalid user"
    },
    {
      "code": 6019,
      "name": "presaleNotStarted",
      "msg": "Presale not started"
    },
    {
      "code": 6020,
      "name": "presaleAlreadyClaimed",
      "msg": "Presale already claimed"
    },
    {
      "code": 6021,
      "name": "nothingToClaim",
      "msg": "Nothing to claim"
    },
    {
      "code": 6022,
      "name": "invalidUserTokenAccount",
      "msg": "Invalid user token account"
    },
    {
      "code": 6023,
      "name": "presaleEnded",
      "msg": "Presale ended"
    },
    {
      "code": 6024,
      "name": "invalidTreasuryTokenAccount",
      "msg": "Invalid treasury token account"
    },
    {
      "code": 6025,
      "name": "invalidOwnersCount",
      "msg": "Invalid owners count"
    },
    {
      "code": 6026,
      "name": "invalidMultisigAdmin",
      "msg": "Invalid multisig admin"
    },
    {
      "code": 6027,
      "name": "stakingAmountZero",
      "msg": "Staking amount is zero"
    },
    {
      "code": 6028,
      "name": "invalidStakingTier",
      "msg": "Invalid staking tier"
    },
    {
      "code": 6029,
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
            "name": "launchTimestamp",
            "type": "i64"
          },
          {
            "name": "totalPresaleSold",
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
            "name": "taxRate",
            "type": "u64"
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
    }
  ]
};
