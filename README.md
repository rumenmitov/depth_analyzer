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
<details>
<summary> <code>-h</code>, <code>--help</code> </summary>
  
Displays help menu.  
  
</details>

<details>
<summary> <code>-v</code>, <code>--version</code> </summary>
  
Displays current version.  

</details>

<details>
<summary> <code>-c</code>, <code>--color</code> <b>[ RED | WHITE ]</b> </summary>
  
Specifies which color to use as an indicator for proximity.  

</details>

<details>
<summary> <code>-t</code>, <code>--threshold</code> <b>[0 .. 255 ]</b> </summary>
  
Specifies the value a pixel must have in order to be considered to be of the proximity color.  
  
</details>

## Possible Results
The user can expect one of the following instructions (shown here in order of precedence):
- `FORWARD` - the *center* path is unobstructed
- `RIGHT` - the *center* path is obstructed, but the *right* is not
- `LEFT` - the *left* path is the only unobstructed path
- `STOP` - all paths are obstructed  

***
![GitHub License](https://img.shields.io/github/license/rumenmitov/depth_analyzer)  
![Downloads on crates.io](https://img.shields.io/crates/dr/depth_analyzer?style=social&logo=rust&logoColor=orange)  
