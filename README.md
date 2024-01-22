# Depth Analyzer
> Program that analyzes an image processed by AI depth-detection models.

## Installation
```sh
cargo install depth_analyzer
```

## Usage
```sh
depth-analyzer /path/to/image.[jpg | png | webp]
```

### Options
- `-h`, `--help` Displays help menu.  
- `-v`, `--version` Displays current version.  
- `-c`, `--color` **[ RED | WHITE ]** Specifies which color to use as an indicator for proximity.  
- `-t`, `--threshold` **[0 .. 255 ]** Specifies the value a pixel must have in order to be considered to be of the proximity color.  


## Possible Results
The user can expect one of the following instructions (shown here in order of precedence):
- `FORWARD` - the *center* path is unobstructed
- `RIGHT` - the *center* path is obstructed, but the *right* is not
- `LEFT` - the *left* path is the only unobstructed path
- `STOP` - all paths are obstructed  
