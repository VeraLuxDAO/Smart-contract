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
      "name": "initGlobal",
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
    }
  ],
  "events": [
    {
      "name": "globalUpdateEvent",
      "discriminator": [
        153,
        69,
        19,
        7,
        115,
        232,
        248,
        248
      ]
    },
    {
      "name": "multisigUpdatedEvent",
      "discriminator": [
        56,
        243,
        88,
        79,
        230,
        188,
        137,
        20
      ]
    },
    {
      "name": "proposalSubmitted",
      "discriminator": [
        36,
        127,
        215,
        177,
        143,
        187,
        90,
        147
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
            "type": "u64"
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
      "name": "globalUpdateEvent",
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
      "name": "multisigUpdatedEvent",
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
      "name": "proposalSubmitted",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "proposalId",
            "type": "u64"
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
