
### VIM-ALCHEMY (WIP)

> Convert stuff into other stuff

## Description

Vim-alchemy is a personal plugin that does some stuff that I would otherwise waste a minute on.

Stuff like:

* Converting int to hex and vice versa
```vim
" with 1000000000000000000 highlighted
:Alch hex 
" output: 0xde0b6b3a7640000
```

* Converting hex to bytes and vice versa
```vim
" with 0xde0b6b3a7640000 highlighted
:Alch bytes 
" replaces with [0x0, 0x0, 0x64, 0xa7, 0xb3, 0xb6, 0xe0, 0xd]
```

* Manipulating arrays
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

And some more niche stuff like:
```vim
" with Swap(address,uint256,uint256,uint256,uint256,address) highlighted
:AlchHash keccak256
" replaces with 0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822
```

Most commands also work linewise, so you can do stuff like:
```vim
" 123
" 456
" 789
:Alch hex
" 0x7b
" 0x1c8
" 0x315
```

## Installation

```lua
{
    "rubenduburck/vim-alchemy",
    event = "VeryLazy",
    opts = {},
}
```

## Usage

### Base Conversion

The most basic functionality is converting between bases.
```vim
" with 1000000000000000000 highlighted
:Alch hex
" replaces with 0xde0b6b3a7640000
```

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

So you converted 1234 to 0x4d2 but you actually needed 0x000004d2.
I added some misc tools to help with that.

* Padding
```vim
" with 0x4d2 highlighted
:AlchPadLeft 8
" replaces with 0x000004d2
:AlchPadRight 8
" replaces with 0x4d200000
```

* Chunking
```vim
" with [0x00, 0x00, 0x04, 0xd2] highlighted
:AlchChunk 2
" replaces with [0x0, 0xd204]
```

* Reversing and Rotating
```vim
" with 0x12345678 highlighted
:AlchReverse
" replaces with 0x78563412
:AlchRotate 3
" replaces with 0x34567812
```

* Flattening
```vim
" with [0x0, 0x0, [0x64, [0xa7, 0xb3], 0xb6], 0xe0, 0xd] highlighted
:AlchFlatten 
" replaces with [0x0, 0x0, 0x64, 0xa7, 0xb3, 0xb6, 0xe0, 0xd]
```

* Generating and Randomizing
```vim
:AlchGenerate bytes 8
" puts at cursor [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
:AlchRandom bytes 8
" puts at cursor [0x8f, 0x8a, 0x3a, 0xda, 0x97, 0xbb, 0x59, 0xf3]
```


## License
[MIT](https://choosealicense.com/licenses/mit/)

## Roadmap
* Pick classification from list
* More explicit methods to skip classification

## Known Issues
* Random is kinda slow
* Various tools don't work line wise
