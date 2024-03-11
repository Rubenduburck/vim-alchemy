
### VIM-ALCHEMY (WIP)

> Convert stuff into other stuff

## Description

Vim-alchemy is a personal plugin that does some stuff that I would otherwise waste a minute on.

Stuff like:
```vim
" with 1000000000000000000 highlighted
:Alch hex 
" output: 0xde0b6b3a7640000
```

```vim
" with 0xde0b6b3a7640000 highlighted
:Alch bytes 
" replaces with [0x0, 0x0, 0x64, 0xa7, 0xb3, 0xb6, 0xe0, 0xd]
```

```vim
" with [0x0, 0x0, 0x64, 0xa7, 0xb3, 0xb6, 0xe0, 0xd] highlighted
:AlchPadLeft 32 
" replaces with [0x00, 0x00, 0x64, 0xa7, 0xb3, 0xb6, 0xe0, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
```

```vim
" with [0x00, 0x00, 0x64, 0xa7, 0xb3, 0xb6, 0xe0, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ] highlighted
:AlchChunk 4 
" replaces with [0xde0b6b3a7640000, 0x0, 0x0, 0x0]
```

```vim
" with [0xde0b6b3a7640000, 0x0, 0x0, 0x0] highlighted
:AlchReverse 
" replaces with [0x0, 0x0, 0x0, 0xde0b6b3a7640000]
```

And some very niche stuff like:
```vim
" with Swap(address,uint256,uint256,uint256,uint256,address) highlighted
:AlchHash keccak256
" replaces with 0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822
```

etc.

## Installation

```lua
{
	"rubenduburck/vim-alchemy",
    event = "VeryLazy",
    opts = {},
}
```

## Design
I don't like thinking, especially about:
1. Pre-formatting my input
2. Explicitly telling the plugin what I want it to do
3. Arbitrary limits on the input/output

Therefore I chose this three step classification - decoding - encoding pipeline

### Classification

Currently, the plugin will try to classify the following input types:
* Decimal
* Hexadecimal
* Binary
* Base64
* Base58
* Arrays of the above

Classification is done using a regex and a simple difference score, lower is better.
If two classifications have the same score, the one with the lowest base is chosen.
However, this would be annoying, since a decimal number would always default its lowest digit's base.
E.g., ```1234``` would be classified as base4 under these rules, which probably isn't what the user wants.
For this reason, there is an arbitrary preference given to decimal, then hex, because that's usually what I'm dealing with.

For arrays, the plugin will try to classify anything with "brackets" and "separators" as an array.
Currently, "brackets" are limited to ```[, ], {, }, (, ), <, >```, and "separators" are limited to ```,``` only.
Also nested arrays are fine.

In general just throw something in and see what happens.

### Decoding

Once the input is classified, the plugin will try to decode it into an internal little endian byte array.
For base 2 to 36, I use the ```rug``` crate, which uses GNU MP for arbitrary precision arithmetic. 
For base 64, I use the ```base64``` crate.
For base 58, I use the ```bs58``` crate.
For arrays, I try to decode each element individually.

Decoding **never fails**, but the string extracted for decoding might not be what the user intended.

### Encoding

Once the input is decoded, the plugin will try to encode it into the desired output.
Some keywords are matched to common encodings, namely:
* ```hex``` -> base 16
* ```bytes``` -> array of bytes
* ```int``` -> decimal
* ```bin``` -> binary
* ```baseN``` -> base N, where N is a number between 2 and 62
* ```utfN``` -> utf N, where N is 8 or 16

keywords can be enclosed in brackets, e.g., ```[hex]```, to force the output to be an array of the desired encoding.
If for a given encoding operation, the encoder runs out of encodings in an array, it will loop the array.
E.g. if ```[1, 2, 3]``` is encoded to [hex], the output will be ```[0x1, 0x2, 0x3]```.
However, if ```[1, 2, 3]``` is encoded to ```[hex, int]```, the output will be ```[0x1, 2, 0x3]```.

### Hashing

I need a quick hash sometimes when dealing with Ethereum, so I added some simple hashing functionality.
e.g.
```vim
" with Swap(address,uint256,uint256,uint256,uint256,address) highlighted
:AlchHash keccak256
" replaces with 0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822
```
Default is ```keccak256```, but other hashers are supported, including:
* ```sha2-{224, 256, 384, 512}```
* ```sha3-{224, 256, 384, 512}```
* ```keccak-{224, 256, 384, 512}```


### Misc tools

It turns out that there's many situations where you need slightly different encodings of the same data.
E.g., you might convert 1234 to 0x4d2, but you actually needed 0x000004d2.
To handle this, I added some tools to easily pre/post process the data.
These tools are:
* ```AlchPadLeft N``` - pad the input to the left with "zeroes" until N bytes long.
* ```AlchPadRight N``` - pad the input to the right with "zeroes" until N bytes long.
* ```AlchChunk N``` - chunk the input into an array of N chunks
* ```AlchReverse``` - reverse the input
* ```AlchRotate N``` - rotate the input N bytes to the left
* ```AlchFlatten N``` - flatten the input to depth N
* ```AlchGenerate encoding N``` - generate N "zero" bytes of the encoding
* ```AlchRandom encoding N``` - generate N random bytes of the encoding


## License
[MIT](https://choosealicense.com/licenses/mit/)

## Roadmap
* Pick classification from list

## Known Issues
