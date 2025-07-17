### VIM-ALCHEMY

> Convert stuff into other stuff

## Description

Vim-alchemy is a powerful data manipulation tool for Vim/Neovim. It provides a CLI tool for encoding, decoding, hashing, and various data transformations, integrated seamlessly with your editor through a Lua plugin.

## Architecture

Vim-alchemy consists of two parts:
1. **CLI Tool**: A standalone binary written in Rust that handles all data transformations
2. **Neovim Plugin**: A Lua plugin that provides editor integration and calls the CLI

## Installation

```lua
{
    "rubenduburck/vim-alchemy",
    event = "VeryLazy",
    build = "make",
    opts = {},
}
```

## CLI Usage

The CLI tool can be used standalone for data manipulation:

```bash
# Classify input encoding
vim-alchemy classify "0x1234"

# Convert between encodings
vim-alchemy convert -i hex -o base64 "0x1234"

# Classify and convert automatically
vim-alchemy classify-and-convert -o base64 "0x1234"

# Hash data
vim-alchemy hash -a sha256 -i hex "0x1234"

# Array operations
vim-alchemy flatten-array "[[1,2],[3,4]]"
vim-alchemy chunk-array -c 2 "[1,2,3,4,5,6]"
vim-alchemy reverse-array -d 1 "[1,2,3,4]"
vim-alchemy rotate-array -r 2 "[1,2,3,4]"

# Generate data
vim-alchemy generate -e hex -b 32
vim-alchemy random -e base64 -b 16

# Padding
vim-alchemy pad-left -p 32 "0x1234"
vim-alchemy pad-right -p 32 "0x1234"
```

## Neovim Usage

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

### Auto-detection
The plugin will do its best to figure out what your input is.
Sometimes it needs a little help, e.g. if you have hex bytes without the 0x prefix.
Simply highlight the text you want to convert and run the command with the desired encoding.

## Supported Encodings

* `hex` - Hexadecimal (with or without 0x prefix)
* `bytes` - Byte arrays like [0x12, 0x34]
* `int` - Decimal integers
* `bin` - Binary (0b prefix optional)
* `base{2-36}` - Base N encoding
* `base58` - Base58 encoding
* `base64` - Base64 encoding  
* `utf8` - UTF-8 text
* `utf16` - UTF-16 text
* `ascii` - ASCII text

## Supported Hash Algorithms

* `md5`
* `sha1`
* `sha256`, `sha384`, `sha512`
* `sha3-256`, `sha3-384`, `sha3-512`
* `keccak256`, `keccak512`
* `blake2b`, `blake2s`

## Configuration

```lua
require('alchemy').setup({
    cli = {
        bin = "vim-alchemy", -- Path to CLI binary
    },
    hashers = {
        "md5", "sha256", "sha512", -- etc
    },
    input_encodings = {
        "int", "hex", "bin", "base58", "base64", -- etc
    },
    output_encodings = {
        "int", "hex", "bin", "bytes", "[int]", -- etc
    },
})
```

## Development

### Building from source

```bash
# Clone the repository
git clone https://github.com/rubenduburck/vim-alchemy
cd vim-alchemy

# Build the CLI tool
cargo build --release

# The binary will be at target/release/vim-alchemy
```

### Running tests

```bash
cargo test
```

## License

MIT