### ALCHEMY

> Convert stuff into other stuff

## Description

Alchemy is a powerful data manipulation tool for Vim/Neovim. It provides a CLI tool for encoding, decoding, hashing, and various data transformations, integrated seamlessly with your editor through a Lua plugin.

## Architecture

Alchemy consists of two parts:
1. **CLI Tool**: A standalone binary written in Rust that handles all data transformations
2. **Neovim Plugin**: A Lua plugin that provides editor integration and calls the CLI

## Installation

### With lazy.nvim

```lua
{
    "rubenduburck/alchemy",
    event = "VeryLazy",
    build = "make install",  -- Automatically downloads the correct binary for your platform
    opts = {
        -- Optional: disable default keymaps (default: true)
        default_keymaps = false
    },
    config = function(_, opts)
        require("alchemy").setup(opts)
    end,
}
```

### Manual Installation

1. **Automatic (recommended)**: `make install` - Downloads the correct binary for your platform
2. **Manual**: Download from [releases](https://github.com/rubenduburck/alchemy/releases) and place in PATH
3. **Build from source**: `make build` - Requires Rust toolchain

### Plugin Setup

```lua
-- Basic setup (includes default keymaps and auto-detects binary)
require("alchemy").setup()

-- Setup without keymaps
require("alchemy").setup({
    default_keymaps = false
})

-- Setup with custom CLI binary path
require("alchemy").setup({
    cli = { bin = "/path/to/alchemy" }
})
```

The plugin automatically looks for the `alchemy` binary in:
1. `./bin/alchemy` (downloaded by `make install`)
2. Your PATH
3. Common installation locations

## CLI Usage

The CLI tool can be used standalone for data manipulation:

```bash
# Classify input encoding
alchemy classify "0x1234"

# Convert between encodings
alchemy convert --from hex --to base64 "0x1234"

# Classify and convert automatically
alchemy convert --to base64 "0x1234"

# Hash data
alchemy hash sha256 "0x1234"
alchemy chunk-array -c 2 "[1,2,3,4,5,6]"
alchemy reverse-array -d 1 "[1,2,3,4]"
alchemy rotate-array -r 2 "[1,2,3,4]"

# Generate data
alchemy generate -e hex -b 32
alchemy random -e base64 -b 16

# Padding
alchemy pad-left -p 32 "0x1234"
alchemy pad-right -p 32 "0x1234"
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
        bin = "alchemy", -- Path to CLI binary
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
git clone https://github.com/rubenduburck/alchemy
cd alchemy

# Build the CLI tool
cargo build --release

# The binary will be at target/release/alchemy
```

### Build Requirements

This project uses the `rug` library for arbitrary precision arithmetic, which requires:
- GMP (GNU Multiple Precision Arithmetic Library)
- MPFR (Multiple Precision Floating-Point Reliable Library)
- C compiler with GNU17 support

**Important**: The project is configured to compile with `-std=gnu17` to ensure compatibility with the rug dependency.

#### Installing dependencies:

**Ubuntu/Debian:**
```bash
sudo apt-get install libgmp-dev libmpfr-dev libmpc-dev
```

**macOS:**
```bash
brew install gmp mpfr
```

**Windows:**
Not supported due to GMP/MPFR dependency requirements.

### Cross-compilation

To build for different architectures:

```bash
# Build for specific target
make build-target TARGET=aarch64-unknown-linux-gnu

# Available targets:
# - x86_64-unknown-linux-gnu
# - aarch64-unknown-linux-gnu
# - x86_64-apple-darwin
# - aarch64-apple-darwin
```

### Supported Architectures

Pre-built binaries are available for:
- **Linux**: x86_64, aarch64
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)

**Windows is not supported** due to the `rug` dependency requiring GMP/MPFR libraries which are difficult to build on Windows with MSVC.

**Note**: Architecture support is limited by the `rug` dependency. If rug doesn't support a particular architecture, we cannot provide builds for it.

### Running tests

```bash
cargo test
```

## License

MIT