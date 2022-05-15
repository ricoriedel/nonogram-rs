# nonogram-rs
A fast and lightweight nonogram solving library.

Features:
* Solving regular nonograms
* Solving colored nonograms
* Arbitrary color type
* De/serializing a layout (requires `serde` feature)
* De/serializing a nonogram (requires `serde` feature)

Examples on how to use this library can be found in the `tests/` directory.

## Command line interface
This library includes a CLI called `nonosolver` as binary target.

```shell
cargo build --bin nonosolver --all-features
```
```shell
# Solve
cat layout.json | ./nonosolver solve > result.json

# Show
cat result.json | ./nonoslver show

# Both in one line
cat layout.json | ./nonoslver solve | ./nonosolver show
```
#### Example layout
```json
{
  "cols": [
    [["y", 1], ["y", 1]],
    [["r", 3]],
    [["r", 1]]
  ],
  "rows": [
    [["y", 1], ["r", 1]],
    [["r", 1]],
    [["y", 1], ["r", 2]]
  ]
}
```

#### Example nonogram
```json
{
  "rows":[
    [{"Box":{"color":"y"}}, {"Box":{"color":"r"}}, "Space"],
    ["Space",               {"Box":{"color":"r"}}, "Space"],
    [{"Box":{"color":"y"}}, {"Box":{"color":"r"}}, {"Box":{"color":"r"}}]
  ]
}
```

#### Result
```
🟨🟥  
  🟥  
🟨🟥🟥
```

#### Colors
The following colors are supported by the CLI.
Lowercase letters are dark and uppercase letters are bright colors.
Note that the library supports arbitrary color types.

| Key | Dark color   | Key | Bright color     |
|:---:|:-------------|:---:|:-----------------|
|  d  | dark (black) |  D  | dark (dark gray) |
|  r  | red          |  R  | red              |
|  g  | green        |  G  | green            |
|  y  | yellow       |  Y  | yellow           |
|  b  | blue         |  B  | blue             |
|  m  | magenta      |  M  | magenta          |
|  c  | cyan         |  C  | cyan             |
|  w  | white (gray) |  W  | white            |

## Algorithm
The algorithm is explained in detail in a [separate document](ALGORITHM.md).

## License
This software (including the complete source code) is licensed under the [GPLv3](LICENSE).