/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/veralux.json`.
 */
export type Veralux = {
  "address": "xikFFfUa48SD9UZcHjRmkaSKuso7WXQXyxRP4t5EEZY",
  "metadata": {
    "name": "veralux",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "buyPresale",
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
          "name": "purchaser",
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
                  101
                ]
              },
              {
                "kind": "account",
                "path": "purchaser"
              }
            ]
          }
        },
        {
          "name": "purchaserUsdt",
          "writable": true
        },
        {
          "name": "treasuryUsdt",
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
          "name": "usdtAmount",
          "type": "u64"
        },
        {
          "name": "kycVerified",
          "type": "bool"
        }
      ]
    },
    {
      "name": "claimPresale",
      "discriminator": [
        82,
        240,
        122,
        5,
        109,
        66,
        86,
        190
      ],
      "accounts": [
        {
          "name": "user",
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
                  101
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "treasuryTokenAccount",
          "writable": true
        },
        {
          "name": "treasuryAuthority",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  114,
                  101,
                  97,
                  115,
                  117,
                  114,
                  121
                ]
              },
              {
                "kind": "account",
                "path": "global.treasury_wallet",
                "account": "globalState"
              }
            ]
          }
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
      "args": []
    },
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
                  108
                ]
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
      "args": [
        {
          "name": "treasuryWallet",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "initPresale",
      "discriminator": [
        172,
        248,
        47,
        226,
        223,
        52,
        94,
        217
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
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "tokenPriceInUsdt",
          "type": "u64"
        },
        {
          "name": "maxPerWallet",
          "type": "u64"
        },
        {
          "name": "totalPresaleCap",
          "type": "u64"
        },
        {
          "name": "launchTimestamp",
          "type": "i64"
        },
        {
          "name": "vesting",
          "type": {
            "defined": {
              "name": "presaleVestingScheduel"
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
        }
      ],
      "args": [
        {
          "name": "treasuryUsdtAccount",
          "type": {
            "option": "pubkey"
          }
        },
        {
          "name": "treasuryWallet",
          "type": {
            "option": "pubkey"
          }
        }
      ]
    },
    {
      "name": "updatePresale",
      "discriminator": [
        9,
        223,
        20,
        184,
        183,
        199,
        90,
        226
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
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "tokenPriceInUsdt",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "maxPerWallet",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "totalPresaleCap",
          "type": {
            "option": "u64"
          }
        },
        {
          "name": "launchTimestamp",
          "type": {
            "option": "i64"
          }
        },
        {
          "name": "newVesting",
          "type": {
            "option": {
              "defined": {
                "name": "presaleVestingScheduel"
              }
            }
          }
        },
        {
          "name": "presaleActive",
          "type": {
            "option": "bool"
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
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "invalidAuthority",
      "msg": "Invalid authority"
    },
    {
      "code": 6001,
      "name": "invalidAdmin",
      "msg": "Invalid admin"
    },
    {
      "code": 6002,
      "name": "presaleAlreadyInitialized",
      "msg": "Presale already initialized"
    },
    {
      "code": 6003,
      "name": "presaleNotActive",
      "msg": "Presale not active"
    },
    {
      "code": 6004,
      "name": "invalidTreasury",
      "msg": "Invalid treasury"
    },
    {
      "code": 6005,
      "name": "arithmeticOverflow",
      "msg": "Arithmetic overflow"
    },
    {
      "code": 6006,
      "name": "presaleSupplyExceeded",
      "msg": "Presale supply exceeded"
    },
    {
      "code": 6007,
      "name": "kycRequired",
      "msg": "KYC required"
    },
    {
      "code": 6008,
      "name": "invalidUser",
      "msg": "Invalid user"
    },
    {
      "code": 6009,
      "name": "presaleNotStarted",
      "msg": "Presale not started"
    },
    {
      "code": 6010,
      "name": "presaleAlreadyClaimed",
      "msg": "Presale already claimed"
    },
    {
      "code": 6011,
      "name": "nothingToClaim",
      "msg": "Nothing to claim"
    },
    {
      "code": 6012,
      "name": "invalidUserTokenAccount",
      "msg": "Invalid user token account"
    },
    {
      "code": 6013,
      "name": "presaleEnded",
      "msg": "Presale ended"
    },
    {
      "code": 6014,
      "name": "invalidTreasuryTokenAccount",
      "msg": "Invalid treasury token account"
    }
  ],
  "types": [
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
            "name": "presaleTotalSold",
            "type": "u64"
          },
          {
            "name": "tokenPriceInUsdt",
            "type": "u64"
          },
          {
            "name": "maxPerWallet",
            "type": "u64"
          },
          {
            "name": "totalPresaleCap",
            "type": "u64"
          },
          {
            "name": "presaleVesting",
            "type": {
              "defined": {
                "name": "presaleVestingScheduel"
              }
            }
          },
          {
            "name": "treasuryUsdtAccount",
            "type": "pubkey"
          },
          {
            "name": "treasuryWallet",
            "type": "pubkey"
          },
          {
            "name": "presaleActive",
            "type": "bool"
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
            "name": "user",
            "type": "pubkey"
          },
          {
            "name": "totalPurchased",
            "type": "u64"
          },
          {
            "name": "totalClaimed",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "presaleVestingScheduel",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "initialUnlockPct",
            "type": "u8"
          },
          {
            "name": "weeklyUnlockPct",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
