# nonogram-rs
A fast and lightweight library to solve nonograms.

This library is based on self designed algorithm which is explained in a separate document [here](doc/Algorithm.md).

## Command Line Interface
### Compile
```shell
cargo build --release --bin nonosolver --features="json cmd"
```
### Solve
```shell
nonosolver --in-json '{"cols": [[3], [1], [2]], "rows": [[2], [1, 1], [1, 1]]}'
# OR
nonosolver --in-file 'layout.json'
```
```
Size: 3x3

████  
██  ██
██  ██
```
### Output as json
```shell
nonosolver --in-json '{"cols": [[3], [1], [2]], "rows": [[2], [1, 1], [1, 1]]}' --out-format 'json'
```
```
{"rows":[[1,1,2],[1,2,1],[1,2,1]]}
```