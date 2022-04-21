# nonogram-rs
A fast and lightweight library to solve nonograms.

This library is based on self designed algorithm which is explained in a separate document [here](doc/Algorithm.md).

## Command Line Interface
### Compile
```shell
cargo build --release --bin nonosolver --all-features
```
### Solve
```shell
cat 'layout.json' | nonosolver
```
```
Size: 3x3

████  
██  ██
██  ██
```
### Solve (output as json)
```shell
cat 'layout.json' | nonosolver --out-format 'json'
```
```
{"rows":[[1,1,2],[1,2,1],[1,2,1]]}
```