// Real output of `substrate-data` 0.2.1 (feature `json`): `ShapeDoc::of(&T::shape())` and the value written by `T::write()`,
// both through `json::to_json`, for a Transform-like struct with f32 arrays, a plain enum, a tagged enum, a list, a map,
// an optional and a bool. Pinned here so the parser is tested against what the crate writes, not what we assume.
import type { DataValue } from "./dataValue";

export const transformShape: unknown = {
  "defs": {
    "Light": {
      "kind": "enum",
      "tag": "kind",
      "variants": [
        {
          "fields": [],
          "name": "off"
        },
        {
          "fields": [
            {
              "name": "radius",
              "required": true,
              "shape": {
                "bits": 32,
                "kind": "float"
              }
            },
            {
              "name": "color",
              "required": true,
              "shape": {
                "kind": "list",
                "of": {
                  "bits": 32,
                  "kind": "float"
                }
              }
            }
          ],
          "name": "point"
        },
        {
          "fields": [
            {
              "name": "angle",
              "required": true,
              "shape": {
                "bits": 32,
                "kind": "float"
              }
            }
          ],
          "name": "spot"
        }
      ]
    },
    "Mode": {
      "doc": "A mode.",
      "kind": "enum",
      "variants": [
        {
          "name": "fast"
        },
        {
          "name": "slow"
        }
      ]
    },
    "Transform": {
      "doc": "Position in space.",
      "fields": [
        {
          "doc": "Where.",
          "name": "translation",
          "required": true,
          "shape": {
            "kind": "list",
            "of": {
              "bits": 32,
              "kind": "float"
            }
          }
        },
        {
          "name": "rotation",
          "required": true,
          "shape": {
            "kind": "list",
            "of": {
              "bits": 32,
              "kind": "float"
            }
          }
        },
        {
          "name": "scale",
          "range": [
            0.0,
            10.0
          ],
          "required": false,
          "shape": {
            "kind": "list",
            "of": {
              "bits": 32,
              "kind": "float"
            }
          }
        },
        {
          "name": "mode",
          "required": true,
          "shape": {
            "kind": "def",
            "name": "Mode"
          }
        },
        {
          "name": "light",
          "required": true,
          "shape": {
            "kind": "def",
            "name": "Light"
          }
        },
        {
          "name": "lights",
          "required": true,
          "shape": {
            "kind": "list",
            "of": {
              "kind": "def",
              "name": "Light"
            }
          }
        },
        {
          "name": "tags",
          "required": true,
          "shape": {
            "kind": "map",
            "of": {
              "kind": "int",
              "max": 2147483647,
              "min": -2147483648
            }
          }
        },
        {
          "name": "parent",
          "required": false,
          "shape": {
            "kind": "optional",
            "of": {
              "kind": "string"
            }
          }
        },
        {
          "name": "visible",
          "required": true,
          "shape": {
            "kind": "bool"
          }
        },
        {
          "name": "count",
          "required": true,
          "shape": {
            "kind": "int",
            "max": 255,
            "min": 0
          }
        }
      ],
      "kind": "struct"
    }
  },
  "root": {
    "kind": "def",
    "name": "Transform"
  }
};

export const transformValue: DataValue = {
  "count": 7,
  "light": {
    "color": [
      1.0,
      0.5,
      0.0
    ],
    "kind": "point",
    "radius": 2.5
  },
  "lights": [
    {
      "kind": "off"
    },
    {
      "angle": 0.5,
      "kind": "spot"
    }
  ],
  "mode": "fast",
  "rotation": [
    0.0,
    0.0,
    0.0,
    1.0
  ],
  "scale": [
    1.0,
    1.0,
    1.0
  ],
  "tags": {
    "a": 1
  },
  "translation": [
    1.0,
    2.0,
    3.0
  ],
  "visible": true
};

export const lightShape: unknown = {
  "defs": {
    "Light": {
      "kind": "enum",
      "tag": "kind",
      "variants": [
        {
          "fields": [],
          "name": "off"
        },
        {
          "fields": [
            {
              "name": "radius",
              "required": true,
              "shape": {
                "bits": 32,
                "kind": "float"
              }
            },
            {
              "name": "color",
              "required": true,
              "shape": {
                "kind": "list",
                "of": {
                  "bits": 32,
                  "kind": "float"
                }
              }
            }
          ],
          "name": "point"
        },
        {
          "fields": [
            {
              "name": "angle",
              "required": true,
              "shape": {
                "bits": 32,
                "kind": "float"
              }
            }
          ],
          "name": "spot"
        }
      ]
    }
  },
  "root": {
    "kind": "def",
    "name": "Light"
  }
};

export const lightValue: DataValue = {
  "angle": 1.0,
  "kind": "spot"
};

export const modeShape: unknown = {
  "defs": {
    "Mode": {
      "doc": "A mode.",
      "kind": "enum",
      "variants": [
        {
          "name": "fast"
        },
        {
          "name": "slow"
        }
      ]
    }
  },
  "root": {
    "kind": "def",
    "name": "Mode"
  }
};

export const modeValue: DataValue = "slow";

export const recursiveShape: unknown = {
  "defs": {
    "Rec": {
      "fields": [
        {
          "name": "next",
          "required": false,
          "shape": {
            "kind": "optional",
            "of": {
              "kind": "def",
              "name": "Rec"
            }
          }
        },
        {
          "name": "v",
          "required": true,
          "shape": {
            "kind": "int",
            "max": 255,
            "min": 0
          }
        }
      ],
      "kind": "struct"
    }
  },
  "root": {
    "kind": "def",
    "name": "Rec"
  }
};

export const recursiveValue: DataValue = {
  "next": {
    "v": 1
  },
  "v": 2
};

// From IDLE's editor backend (protocol samples): a `Described` component's shape and a `Component` value. Ground truth for the wire.
export const motorShape: unknown = {
  "defs": {
    "MotorForm": {
      "doc": "How a motor is stored (the `Stored` form of `Motor3`/`FixedMotor3`): a rigid motion that rotates by `rotor`, then moves the origin to `translation`. Written with `dual` instead of `translation` when rebuilding the motor from the two wouldn't give the same bits.",
      "fields": [
        {
          "doc": "The rotation: `[s, e23, e31, e12]`.",
          "name": "rotor",
          "required": true,
          "shape": {
            "kind": "list",
            "of": {
              "bits": 32,
              "kind": "float"
            }
          }
        },
        {
          "doc": "Where the origin goes, after the rotation: `[x, y, z]`; default `[0, 0, 0]`.",
          "name": "translation",
          "required": false,
          "shape": {
            "kind": "optional",
            "of": {
              "kind": "list",
              "of": {
                "bits": 32,
                "kind": "float"
              }
            }
          }
        },
        {
          "doc": "The motor's `e0` part `[e01, e02, e03, e0123]` as stored.",
          "name": "dual",
          "required": false,
          "shape": {
            "kind": "optional",
            "of": {
              "kind": "list",
              "of": {
                "bits": 32,
                "kind": "float"
              }
            }
          }
        }
      ],
      "kind": "struct"
    },
    "TransformForm": {
      "doc": "Where an entity is: rotation, then translation, as one motor \u2014 stored as `{ motor }`, identity when left out. A newtype of this crate because `ecs-core` stays file-blind and the orphan rule keeps `Schema` off `Transform` anywhere else.",
      "fields": [
        {
          "doc": "The rigid motion from the entity's frame to its parent's.",
          "name": "motor",
          "required": false,
          "shape": {
            "kind": "named",
            "name": "motor",
            "of": {
              "kind": "def",
              "name": "MotorForm"
            }
          }
        }
      ],
      "kind": "struct"
    }
  },
  "root": {
    "kind": "def",
    "name": "TransformForm"
  }
};

export const motorValue: DataValue = {
  "motor": {
    "rotor": [
      1.0,
      0.0,
      -0.0,
      0.0
    ],
    "translation": [
      -0.0,
      1.5,
      -0.0
    ]
  }
};
