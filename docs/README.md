# nib

[![Build Status](https://dev.azure.com/afnanenayet/nib/_apis/build/status/afnanenayet.nib?branchName=master)](https://dev.azure.com/afnanenayet/nib/_build/latest?definitionId=7&branchName=master)

[![asciicast](https://asciinema.org/a/g7uq7pnr4FrXGdBBMkpgeV8IZ.svg)](https://asciinema.org/a/g7uq7pnr4FrXGdBBMkpgeV8IZ)

## Summary

`nib` is a renderer created for research purposes. It is designed to be as
performant as possible without compromising the ability for a user to hack
around with the source code, especially with regards to implementing different
sampling strategies. It is written in Rust and uses Rayon for parallelization,
which calculates the value of each pixel as a unit of work.

## Usage

You can find the available command line flags and usage information with

```sh
nib --help
```

You can find example scene files in the `data/` folder.

A minimal scenes looks like this:

```json
{
    "objects": [],
    "acceleration_structure": "ObjectList",
    "camera": {
        "Pinhole": {
            "origin": {
                "x": 0.0,
                "y": 0.0,
                "z": 0.0
            },
            "horizontal": {
                "x": 4.0,
                "y": 0.0,
                "z": 0.0
            },
            "vertical": {
                "x": 0.0,
                "y": 2.0,
                "z": 0.0
            },
            "lower_left": {
                "x": -2.0,
                "y": -1.0,
                "z": -1.0
            }
        }
    },
    "background": [
        0,
        0,
        0
    ],
    "samples_per_pixel": 100
}
```

You can use JSON, YAML, or RON (I use serde for serialization support). For
now, the scene specification is subject to change as I develop the renderer.
