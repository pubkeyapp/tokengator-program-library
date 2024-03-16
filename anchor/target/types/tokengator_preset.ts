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
          "name": "authority",
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
            "name": "minterConfig",
            "type": {
              "defined": "MinterConfig"
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
            "name": "image",
            "type": {
              "option": "string"
            }
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
            "type": {
              "option": "string"
            }
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
            "type": "u64"
          },
          {
            "name": "rateAuthority",
            "type": {
              "option": "publicKey"
            }
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
            "name": "transferFeeRate",
            "type": "u64"
          },
          {
            "name": "transferFeeAccount",
            "type": "publicKey"
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
            "name": "feePayer",
            "type": "publicKey"
          },
          {
            "name": "freezeAuthority",
            "type": {
              "option": "publicKey"
            }
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
      "name": "UnAuthorized",
      "msg": "Account unauthorized to perform this action"
    },
    {
      "code": 6002,
      "name": "AuthorityAlreadyExists",
      "msg": "Authority already exists"
    },
    {
      "code": 6003,
      "name": "AuthorityNonExistant",
      "msg": "Authority does not exist"
    },
    {
      "code": 6004,
      "name": "CannotRemoveSoloAuthority",
      "msg": "Cannot remove last remaining authority"
    },
    {
      "code": 6005,
      "name": "InvalidPresetName",
      "msg": "Invalid preset name"
    },
    {
      "code": 6006,
      "name": "InvalidPresetDescription",
      "msg": "Invalid preset description"
    },
    {
      "code": 6007,
      "name": "InvalidPresetImageURL",
      "msg": "Invalid Image Url"
    },
    {
      "code": 6008,
      "name": "MaxSizeReached",
      "msg": "Array reached max size"
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
          "name": "authority",
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
            "name": "minterConfig",
            "type": {
              "defined": "MinterConfig"
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
            "name": "image",
            "type": {
              "option": "string"
            }
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
            "type": {
              "option": "string"
            }
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
            "type": "u64"
          },
          {
            "name": "rateAuthority",
            "type": {
              "option": "publicKey"
            }
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
            "name": "transferFeeRate",
            "type": "u64"
          },
          {
            "name": "transferFeeAccount",
            "type": "publicKey"
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
            "name": "feePayer",
            "type": "publicKey"
          },
          {
            "name": "freezeAuthority",
            "type": {
              "option": "publicKey"
            }
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
      "name": "UnAuthorized",
      "msg": "Account unauthorized to perform this action"
    },
    {
      "code": 6002,
      "name": "AuthorityAlreadyExists",
      "msg": "Authority already exists"
    },
    {
      "code": 6003,
      "name": "AuthorityNonExistant",
      "msg": "Authority does not exist"
    },
    {
      "code": 6004,
      "name": "CannotRemoveSoloAuthority",
      "msg": "Cannot remove last remaining authority"
    },
    {
      "code": 6005,
      "name": "InvalidPresetName",
      "msg": "Invalid preset name"
    },
    {
      "code": 6006,
      "name": "InvalidPresetDescription",
      "msg": "Invalid preset description"
    },
    {
      "code": 6007,
      "name": "InvalidPresetImageURL",
      "msg": "Invalid Image Url"
    },
    {
      "code": 6008,
      "name": "MaxSizeReached",
      "msg": "Array reached max size"
    }
  ]
};
