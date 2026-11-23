/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/solana_task_manager.json`.
 */
export type SolanaTaskManager = {
  "address": "3T8VSbh6k1sEStgASQ3epLhCqdP2dv4WxVyAD4RMPCqj",
  "metadata": {
    "name": "solanaTaskManager",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "completeTask",
      "docs": [
        "Marca la tarea como completada. Solo el creador."
      ],
      "discriminator": [
        109,
        167,
        192,
        41,
        129,
        108,
        220,
        196
      ],
      "accounts": [
        {
          "name": "creator",
          "signer": true,
          "relations": [
            "task"
          ]
        },
        {
          "name": "task",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "createTask",
      "docs": [
        "Crea una tarea. El id es el contador actual del perfil."
      ],
      "discriminator": [
        194,
        80,
        6,
        180,
        232,
        127,
        48,
        171
      ],
      "accounts": [
        {
          "name": "creator",
          "writable": true,
          "signer": true
        },
        {
          "name": "profile",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  111,
                  102,
                  105,
                  108,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "creator"
              }
            ]
          }
        },
        {
          "name": "task",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  116,
                  97,
                  115,
                  107
                ]
              },
              {
                "kind": "account",
                "path": "creator"
              },
              {
                "kind": "account",
                "path": "profile.taskCount",
                "account": "profile"
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
          "name": "title",
          "type": "string"
        },
        {
          "name": "description",
          "type": "string"
        }
      ]
    },
    {
      "name": "deleteTask",
      "docs": [
        "Cierra la cuenta de la tarea y devuelve la renta al creador.",
        "Regla de negocio: SOLO si la tarea ya está completada."
      ],
      "discriminator": [
        112,
        220,
        10,
        109,
        3,
        168,
        46,
        73
      ],
      "accounts": [
        {
          "name": "creator",
          "writable": true,
          "signer": true,
          "relations": [
            "task"
          ]
        },
        {
          "name": "task",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "initializeProfile",
      "docs": [
        "Crea el perfil del usuario (una sola vez por wallet)."
      ],
      "discriminator": [
        32,
        145,
        77,
        213,
        58,
        39,
        251,
        234
      ],
      "accounts": [
        {
          "name": "owner",
          "writable": true,
          "signer": true
        },
        {
          "name": "profile",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  111,
                  102,
                  105,
                  108,
                  101
                ]
              },
              {
                "kind": "account",
                "path": "owner"
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
      "name": "updateTask",
      "docs": [
        "Edita título/descripción. Solo el creador y solo si no está completada."
      ],
      "discriminator": [
        100,
        51,
        124,
        168,
        211,
        208,
        42,
        228
      ],
      "accounts": [
        {
          "name": "creator",
          "writable": true,
          "signer": true,
          "relations": [
            "task"
          ]
        },
        {
          "name": "task",
          "writable": true
        }
      ],
      "args": [
        {
          "name": "title",
          "type": "string"
        },
        {
          "name": "description",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "profile",
      "discriminator": [
        184,
        101,
        165,
        188,
        95,
        63,
        127,
        188
      ]
    },
    {
      "name": "task",
      "discriminator": [
        79,
        34,
        229,
        55,
        88,
        90,
        55,
        84
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "unauthorized",
      "msg": "Solo el creador de la tarea puede realizar esta operación"
    },
    {
      "code": 6001,
      "name": "emptyTitle",
      "msg": "El título no puede estar vacío"
    },
    {
      "code": 6002,
      "name": "titleTooLong",
      "msg": "El título supera los 64 caracteres"
    },
    {
      "code": 6003,
      "name": "descriptionTooLong",
      "msg": "La descripción supera los 200 caracteres"
    },
    {
      "code": 6004,
      "name": "taskAlreadyCompleted",
      "msg": "La tarea ya está completada"
    },
    {
      "code": 6005,
      "name": "taskNotCompleted",
      "msg": "La tarea debe estar completada antes de poder eliminarla"
    }
  ],
  "types": [
    {
      "name": "profile",
      "docs": [
        "Perfil de un usuario: cuenta relacionada con todas sus tareas.",
        "PDA: [\"profile\", owner]"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "pubkey"
          },
          {
            "name": "taskCount",
            "docs": [
              "Contador monotónico: el id de la próxima tarea.",
              "También es el componente que hace única cada PDA de tarea."
            ],
            "type": "u64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "task",
      "docs": [
        "Una tarea on-chain.",
        "PDA: [\"task\", creator, id.to_le_bytes()]"
      ],
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "id",
            "type": "u64"
          },
          {
            "name": "title",
            "type": "string"
          },
          {
            "name": "description",
            "type": "string"
          },
          {
            "name": "completed",
            "type": "bool"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "constants": [
    {
      "name": "profileSeed",
      "type": "bytes",
      "value": "[112, 114, 111, 102, 105, 108, 101]"
    },
    {
      "name": "taskSeed",
      "type": "bytes",
      "value": "[116, 97, 115, 107]"
    }
  ]
};
