[![crates.io](https://img.shields.io/crates/v/nu_plugin_compress.svg)](https://crates.io/crates/nu_plugin_compress)
[![docs.rs](https://docs.rs/nu_plugin_compress/badge.svg)](https://docs.rs/nu_plugin_compress)

## nu_plugin_compress
A nushell plugin for compression and decompression, supporting zstd, gzip, bzip2, and xz. Require nushell >= `0.102.0`.

## Status

Supported compression formats include:

|Type|Compress Command|Decompress Command|
|--|--|--|
|gzip|to gz|from gz|
|zstd|to zst|from zst|
|xz|to xz|from xz|
|bzip2|to bz2|from bz2|


### Installation
```shell
cargo install nu_plugin_compress
plugin add ~/.cargo/bin/nu_plugin_compress
plugin use compress
```

### Usage
```shell
# compress and save
"hello" | to gz | save hello.gz
# compress with level 7, Default level is 3
"hello" | to xz -l 7
# decompress and print
open hello.gz | decode
# archive and compress
tar cvf - hello.txt | to gz | save hello.tar.gz
# decompress and unarchive
open hello.tar.gz | tar xvf -
```

### Example
```shell
> open hello.gz
Length: unknown (stream) | printable whitespace ascii_other non_ascii
00000000:   68 65 6c 6c  6f                                      hello
```