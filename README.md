# Depth Analyzer
> Program that analyzes an image processed by [MiDaS AI](https://github.com/isl-org/MiDaS/tree/master).

## Installation
```sh
cargo install depth_analyzer
```

## Usage
```sh
depth-analyzer /path/to/image.[jpg | png | webp]
```
  
The user can expect one of the following instructions:  
- `STOP` - all paths are obstructed  
- `FORWARD` - the *center* path is unobstructed
- `RIGHT` - the *center* path is obstructed, but the *right* is not
- `LEFT` - the *left* path is the only unobstructed path
