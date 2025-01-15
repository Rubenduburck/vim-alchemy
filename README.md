
### VIM-ALCHEMY (WIP)

> Convert stuff into other stuff

## Description

Vim-alchemy is your one stop shop for manipulating data in vim.
Convert data, hash stuff, reverse arrays, pad bytes, etc.

## Installation

```lua
{
    "rubenduburck/vim-alchemy",
    event = "VeryLazy",
    opts = {},
}
```


## Usage

### Convert
```vim
    :Alch classify_and_convert {optional:output_encoding}
    :Alch convert {input_encoding} {optional:output_encoding}
```

Converts visual selection from input to output encoding.
classify_and_convert will guess the input encoding.
Set `input_encoding` to `auto` to automatically detect the input encoding.
Set `input_encoding` to `select` to select the encoding from a list of options.
Set `output_encoding` to `select` to select the encoding from a list of options.

#### Auto-detection
The plugin will do its best to figure out what your input is.
Sometimes it needs a little help, e.g. if you have hex bytes without the 0x prefix.
Simply highlight the text you want to convert and run the command with the desired encoding.
Encoding can be any of the following:
* ```hex```
* ```bytes```
* ```int```
* ```bin```
* ```base{2-36, 58, 64}```
* ```utf{8, 16}```(experimental)

Add more defaults to the config

#### Examples

Convert between bases:
```vim
" with 1000000000000000000 highlighted
:Alch classify_and_convert hex 
:Alch convert int hex
" output: 0xde0b6b3a7640000
```

```vim
" with 0b110111100000101101011001110101001110010000000000 highlighted
:Alch classify_and_convert base64
:Alch convert bin base64
" output 3gtZ1OQA
```

Handles arbitrary size btw:
```vim
" with 123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890 highlighted
:Alch classify_and_convert hex
:Alch convert int hex
" output: 0x79dd55ae1cb75eea2dcc2a04430c813e5739185d3559085ef523a55a7ce2c5392c7d287cbdd892ae321dfd37238836d35ba5ceea2a01788a9935e243d1161ef7bf14baccff196ce3f0ad2
```

```vim
" with 0xde0b6b3a7640000 highlighted
:Alch convert hex bytes 
:Alch classify_and_convert bytes
" replaces with [0x0, 0x0, 0x64, 0xa7, 0xb3, 0xb6, 0xe0, 0xd]
```

### Hashing

Available hashers:
* ```sha2-{224, 256, 384, 512}```
* ```sha3-{224, 256, 384, 512}```
* ```keccak-{224, 256, 384, 512}```
* ```blake2-{256, 512}```

```vim
    :Alch classify_and_hash {optional:hasher}
    :Alch hash {optional:input_encoding} {optional:hasher}
```

Hashes visual selection with the specified hasher.
classify_and_hash will guess the hasher.
Set `hasher` to `select` to select the hasher from a list of options.
Set `input_encoding` to `auto` to automatically detect the input encoding.
Set `input_encoding` to `select` to select the encoding from a list of options.

#### Examples

Ethereum function signature:
```vim
" with Swap(address,uint256,uint256,uint256,uint256,address) highlighted
:Alch hash ascii keccak256
:Alch classify_and_hash keccak256
" replaces with 0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822
```

Solana anchor function signature:
```vim
" with global:initialize highlighted
:Alch hash ascii sha2
:Alch classify_and_hash sha2
" replaces with 0xafaf6d1f0d989bedd46a95073281adc21bb5e0e1d773b2fbbd7ab504cdd4aa30
```

### Manipulate Arrays

#### Examples
```vim
" with [0x0, 0x0, 0x64, 0xa7, 0xb3, 0xb6, 0xe0, 0xd] highlighted
:Alch pad_left 32 
" replaces with [0x00, 0x00, 0x64, 0xa7, 0xb3, 0xb6, 0xe0, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
```

```vim
" with [0x00, 0x00, 0x64, 0xa7, 0xb3, 0xb6, 0xe0, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ] highlighted
:Alch chunk 4 
" replaces with [0xde0b6b3a7640000, 0x0, 0x0, 0x0]
```

```vim
" with [0xde0b6b3a7640000, 0x0, 0x0, 0x0] highlighted
:Alch reverse
" replaces with [0x0, 0x0, 0x0, 0xde0b6b3a7640000]
```

## License
[MIT](https://choosealicense.com/licenses/mit/)

## Roadmap

## Known Issues
* Random is kinda slow
* Various tools don't work line wise
