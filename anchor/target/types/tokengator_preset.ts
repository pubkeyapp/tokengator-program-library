export type TokengatorPreset = {
  "version": "0.1.0",
  "name": "tokengator_preset",
  "instructions": [
    {
      "name": "createPreset",
      "accounts": [
        {
          "name": "preset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenExtensionsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "CreatePresetArgs"
          }
        }
      ]
    },
    {
      "name": "addPresetAuthority",
      "accounts": [
        {
          "name": "preset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "AddPresetAuthorityArgs"
          }
        }
      ]
    },
    {
      "name": "removePresetAuthority",
      "accounts": [
        {
          "name": "preset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "RemovePresetAuthorityArgs"
          }
        }
      ]
    },
    {
      "name": "mintPreset",
      "accounts": [
        {
          "name": "preset",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authorityTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "removePreset",
      "accounts": [
        {
          "name": "preset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "preset",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "imageUrl",
            "type": "string"
          },
          {
            "name": "feePayer",
            "type": "publicKey"
          },
          {
            "name": "authorities",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "minterConfig",
            "type": {
              "defined": "MinterConfig"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "AddPresetAuthorityArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "newAuthority",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "CreatePresetArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "imageUrl",
            "type": "string"
          },
          {
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "metadataConfig",
            "type": {
              "option": {
                "defined": "MinterMetadataConfig"
              }
            }
          },
          {
            "name": "interestConfig",
            "type": {
              "option": {
                "defined": "MinterInterestConfig"
              }
            }
          },
          {
            "name": "transferFeeConfig",
            "type": {
              "option": {
                "defined": "MinterTransferFeeConfig"
              }
            }
          }
        ]
      }
    },
    {
      "name": "RemovePresetAuthorityArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authorityToRemove",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "MinterMetadataConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "imageUrl",
            "type": "string"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "metadata",
            "type": {
              "option": {
                "vec": {
                  "array": [
                    "string",
                    2
                  ]
                }
              }
            }
          },
          {
            "name": "uri",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "MinterInterestConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rate",
            "type": "i16"
          }
        ]
      }
    },
    {
      "name": "MinterTransferFeeConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "transferFeeBasisPoints",
            "type": "u16"
          },
          {
            "name": "maxFeeRate",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "MinterConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "metadataConfig",
            "type": {
              "option": {
                "defined": "MinterMetadataConfig"
              }
            }
          },
          {
            "name": "interestConfig",
            "type": {
              "option": {
                "defined": "MinterInterestConfig"
              }
            }
          },
          {
            "name": "transferFeeConfig",
            "type": {
              "option": {
                "defined": "MinterTransferFeeConfig"
              }
            }
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidAccountOwner",
      "msg": "Account not owned by program"
    },
    {
      "code": 6001,
      "name": "InvalidFeePayer",
      "msg": "Invalid Fee payer"
    },
    {
      "code": 6002,
      "name": "UnAuthorized",
      "msg": "Account unauthorized to perform this action"
    },
    {
      "code": 6003,
      "name": "AuthorityAlreadyExists",
      "msg": "Authority already exists"
    },
    {
      "code": 6004,
      "name": "AuthorityNonExistant",
      "msg": "Authority does not exist"
    },
    {
      "code": 6005,
      "name": "CannotRemoveSoloAuthority",
      "msg": "Cannot remove last remaining authority"
    },
    {
      "code": 6006,
      "name": "InvalidPresetName",
      "msg": "Invalid preset name"
    },
    {
      "code": 6007,
      "name": "InvalidPresetDescription",
      "msg": "Invalid preset description"
    },
    {
      "code": 6008,
      "name": "InvalidPresetImageURL",
      "msg": "Invalid Image Url"
    },
    {
      "code": 6009,
      "name": "MaxSizeReached",
      "msg": "Array reached max size"
    },
    {
      "code": 6010,
      "name": "InvalidMint",
      "msg": "Invalid mint account passed"
    },
    {
      "code": 6011,
      "name": "InvalidTokenProgram",
      "msg": "Token extensions program required"
    },
    {
      "code": 6012,
      "name": "CannotRemoveNonZeroSupplyPreset",
      "msg": "Cannot remove preset of non-zero supply"
    }
  ]
};

export const IDL: TokengatorPreset = {
  "version": "0.1.0",
  "name": "tokengator_preset",
  "instructions": [
    {
      "name": "createPreset",
      "accounts": [
        {
          "name": "preset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenExtensionsProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "CreatePresetArgs"
          }
        }
      ]
    },
    {
      "name": "addPresetAuthority",
      "accounts": [
        {
          "name": "preset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "AddPresetAuthorityArgs"
          }
        }
      ]
    },
    {
      "name": "removePresetAuthority",
      "accounts": [
        {
          "name": "preset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        }
      ],
      "args": [
        {
          "name": "args",
          "type": {
            "defined": "RemovePresetAuthorityArgs"
          }
        }
      ]
    },
    {
      "name": "mintPreset",
      "accounts": [
        {
          "name": "preset",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "authorityTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "removePreset",
      "accounts": [
        {
          "name": "preset",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "mint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "feePayer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "authority",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "preset",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "imageUrl",
            "type": "string"
          },
          {
            "name": "feePayer",
            "type": "publicKey"
          },
          {
            "name": "authorities",
            "type": {
              "vec": "publicKey"
            }
          },
          {
            "name": "minterConfig",
            "type": {
              "defined": "MinterConfig"
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "AddPresetAuthorityArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "newAuthority",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "CreatePresetArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "imageUrl",
            "type": "string"
          },
          {
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "metadataConfig",
            "type": {
              "option": {
                "defined": "MinterMetadataConfig"
              }
            }
          },
          {
            "name": "interestConfig",
            "type": {
              "option": {
                "defined": "MinterInterestConfig"
              }
            }
          },
          {
            "name": "transferFeeConfig",
            "type": {
              "option": {
                "defined": "MinterTransferFeeConfig"
              }
            }
          }
        ]
      }
    },
    {
      "name": "RemovePresetAuthorityArgs",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authorityToRemove",
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "MinterMetadataConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "imageUrl",
            "type": "string"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "symbol",
            "type": "string"
          },
          {
            "name": "metadata",
            "type": {
              "option": {
                "vec": {
                  "array": [
                    "string",
                    2
                  ]
                }
              }
            }
          },
          {
            "name": "uri",
            "type": "string"
          }
        ]
      }
    },
    {
      "name": "MinterInterestConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "rate",
            "type": "i16"
          }
        ]
      }
    },
    {
      "name": "MinterTransferFeeConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "transferFeeBasisPoints",
            "type": "u16"
          },
          {
            "name": "maxFeeRate",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "MinterConfig",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mint",
            "type": "publicKey"
          },
          {
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "metadataConfig",
            "type": {
              "option": {
                "defined": "MinterMetadataConfig"
              }
            }
          },
          {
            "name": "interestConfig",
            "type": {
              "option": {
                "defined": "MinterInterestConfig"
              }
            }
          },
          {
            "name": "transferFeeConfig",
            "type": {
              "option": {
                "defined": "MinterTransferFeeConfig"
              }
            }
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidAccountOwner",
      "msg": "Account not owned by program"
    },
    {
      "code": 6001,
      "name": "InvalidFeePayer",
      "msg": "Invalid Fee payer"
    },
    {
      "code": 6002,
      "name": "UnAuthorized",
      "msg": "Account unauthorized to perform this action"
    },
    {
      "code": 6003,
      "name": "AuthorityAlreadyExists",
      "msg": "Authority already exists"
    },
    {
      "code": 6004,
      "name": "AuthorityNonExistant",
      "msg": "Authority does not exist"
    },
    {
      "code": 6005,
      "name": "CannotRemoveSoloAuthority",
      "msg": "Cannot remove last remaining authority"
    },
    {
      "code": 6006,
      "name": "InvalidPresetName",
      "msg": "Invalid preset name"
    },
    {
      "code": 6007,
      "name": "InvalidPresetDescription",
      "msg": "Invalid preset description"
    },
    {
      "code": 6008,
      "name": "InvalidPresetImageURL",
      "msg": "Invalid Image Url"
    },
    {
      "code": 6009,
      "name": "MaxSizeReached",
      "msg": "Array reached max size"
    },
    {
      "code": 6010,
      "name": "InvalidMint",
      "msg": "Invalid mint account passed"
    },
    {
      "code": 6011,
      "name": "InvalidTokenProgram",
      "msg": "Token extensions program required"
    },
    {
      "code": 6012,
      "name": "CannotRemoveNonZeroSupplyPreset",
      "msg": "Cannot remove preset of non-zero supply"
    }
  ]
};