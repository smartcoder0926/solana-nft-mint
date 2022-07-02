export type WalletNftMint = {
  "version": "0.1.0",
  "name": "wallet_nft_mint",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "initializer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nonceMinting",
          "type": "u8"
        },
        {
          "name": "authorizedCreator",
          "type": "publicKey"
        },
        {
          "name": "maxSupply",
          "type": "u64"
        },
        {
          "name": "ogMax",
          "type": "u64"
        },
        {
          "name": "wlMax",
          "type": "u64"
        },
        {
          "name": "publicMax",
          "type": "u64"
        },
        {
          "name": "ogPrice",
          "type": "u64"
        },
        {
          "name": "wlPrice",
          "type": "u64"
        },
        {
          "name": "publicPrice",
          "type": "u64"
        }
      ]
    },
    {
      "name": "addWlList",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "user",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "removeWlList",
      "accounts": [
        {
          "name": "initializer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updatePrice",
      "accounts": [
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "nonceMinting",
          "type": "u8"
        },
        {
          "name": "newOgPrice",
          "type": "u64"
        },
        {
          "name": "newWlPrice",
          "type": "u64"
        },
        {
          "name": "newPublicPrice",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateAmount",
      "accounts": [
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "nonceMinting",
          "type": "u8"
        },
        {
          "name": "newOgAmount",
          "type": "u64"
        },
        {
          "name": "newWlAmount",
          "type": "u64"
        },
        {
          "name": "newPublicAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "setStage",
      "accounts": [
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "nonceMinting",
          "type": "u8"
        },
        {
          "name": "newStage",
          "type": "i8"
        }
      ]
    },
    {
      "name": "setUri",
      "accounts": [
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "nonceMinting",
          "type": "u8"
        },
        {
          "name": "newUri",
          "type": "string"
        }
      ]
    },
    {
      "name": "mintNft",
      "accounts": [
        {
          "name": "mintAuthority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userMintingCounterAccount",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "masterEdition",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "creatorKey",
          "type": "publicKey"
        },
        {
          "name": "title",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "mintingAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "adminKey",
            "type": "publicKey"
          },
          {
            "name": "auryVault",
            "type": "publicKey"
          },
          {
            "name": "authorizedCreator",
            "type": "publicKey"
          },
          {
            "name": "maxSupply",
            "type": "u64"
          },
          {
            "name": "ogMax",
            "type": "u64"
          },
          {
            "name": "wlMax",
            "type": "u64"
          },
          {
            "name": "publicMax",
            "type": "u64"
          },
          {
            "name": "ogPrice",
            "type": "u64"
          },
          {
            "name": "wlPrice",
            "type": "u64"
          },
          {
            "name": "publicPrice",
            "type": "u64"
          },
          {
            "name": "curNum",
            "type": "u64"
          },
          {
            "name": "freezeProgram",
            "type": "bool"
          },
          {
            "name": "curStage",
            "type": "i8"
          },
          {
            "name": "baseUri",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "userMintingAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "curNum",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "whiteList",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "publicKey"
          },
          {
            "name": "mintingAccount",
            "type": "publicKey"
          },
          {
            "name": "initializer",
            "type": "publicKey"
          },
          {
            "name": "count",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "MintError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "NotAllowed"
          },
          {
            "name": "NotActive"
          },
          {
            "name": "InvalidStage"
          },
          {
            "name": "InsufficientFunds"
          }
        ]
      }
    }
  ]
};

export const IDL: WalletNftMint = {
  "version": "0.1.0",
  "name": "wallet_nft_mint",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "initializer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "nonceMinting",
          "type": "u8"
        },
        {
          "name": "authorizedCreator",
          "type": "publicKey"
        },
        {
          "name": "maxSupply",
          "type": "u64"
        },
        {
          "name": "ogMax",
          "type": "u64"
        },
        {
          "name": "wlMax",
          "type": "u64"
        },
        {
          "name": "publicMax",
          "type": "u64"
        },
        {
          "name": "ogPrice",
          "type": "u64"
        },
        {
          "name": "wlPrice",
          "type": "u64"
        },
        {
          "name": "publicPrice",
          "type": "u64"
        }
      ]
    },
    {
      "name": "addWlList",
      "accounts": [
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "user",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "removeWlList",
      "accounts": [
        {
          "name": "initializer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "updatePrice",
      "accounts": [
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "nonceMinting",
          "type": "u8"
        },
        {
          "name": "newOgPrice",
          "type": "u64"
        },
        {
          "name": "newWlPrice",
          "type": "u64"
        },
        {
          "name": "newPublicPrice",
          "type": "u64"
        }
      ]
    },
    {
      "name": "updateAmount",
      "accounts": [
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "nonceMinting",
          "type": "u8"
        },
        {
          "name": "newOgAmount",
          "type": "u64"
        },
        {
          "name": "newWlAmount",
          "type": "u64"
        },
        {
          "name": "newPublicAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "setStage",
      "accounts": [
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "nonceMinting",
          "type": "u8"
        },
        {
          "name": "newStage",
          "type": "i8"
        }
      ]
    },
    {
      "name": "setUri",
      "accounts": [
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "nonceMinting",
          "type": "u8"
        },
        {
          "name": "newUri",
          "type": "string"
        }
      ]
    },
    {
      "name": "mintNft",
      "accounts": [
        {
          "name": "mintAuthority",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "metadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenMetadataProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mintingAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "whitelist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userMintingCounterAccount",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "masterEdition",
          "isMut": true,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "creatorKey",
          "type": "publicKey"
        },
        {
          "name": "title",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "mintingAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "adminKey",
            "type": "publicKey"
          },
          {
            "name": "auryVault",
            "type": "publicKey"
          },
          {
            "name": "authorizedCreator",
            "type": "publicKey"
          },
          {
            "name": "maxSupply",
            "type": "u64"
          },
          {
            "name": "ogMax",
            "type": "u64"
          },
          {
            "name": "wlMax",
            "type": "u64"
          },
          {
            "name": "publicMax",
            "type": "u64"
          },
          {
            "name": "ogPrice",
            "type": "u64"
          },
          {
            "name": "wlPrice",
            "type": "u64"
          },
          {
            "name": "publicPrice",
            "type": "u64"
          },
          {
            "name": "curNum",
            "type": "u64"
          },
          {
            "name": "freezeProgram",
            "type": "bool"
          },
          {
            "name": "curStage",
            "type": "i8"
          },
          {
            "name": "baseUri",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "userMintingAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "curNum",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "whiteList",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "user",
            "type": "publicKey"
          },
          {
            "name": "mintingAccount",
            "type": "publicKey"
          },
          {
            "name": "initializer",
            "type": "publicKey"
          },
          {
            "name": "count",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "MintError",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "NotAllowed"
          },
          {
            "name": "NotActive"
          },
          {
            "name": "InvalidStage"
          },
          {
            "name": "InsufficientFunds"
          }
        ]
      }
    }
  ]
};
