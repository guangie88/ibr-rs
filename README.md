# image-batch-resizer-rs

[![Build Status](https://travis-ci.org/guangie88/image-batch-resizer-rs.svg?branch=master)](https://travis-ci.org/guangie88/image-batch-resizer-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/0crl0n8bmx240nls/branch/master?svg=true)](https://ci.appveyor.com/project/guangie88/image-batch-resizer-rs/branch/master)
[![Crates.io](https://img.shields.io/crates/v/image-batch-resizer.svg)](https://crates.io/crates/image-batch-resizer)

Experimental image batch resizer executable in Rust.

Performs simple proportional resizing of image files in a given directory path.

## Installation

```bash
cargo install image-batch-resizer
```

This installs `ibr` into your Cargo binary directory.

## Example usage

```bash
ibr input/ -m 512 -d -vvv
```

This resizes all image files in `input/` directory:

* `-m 512`
  * to maximum width/height to 512 pixels proportionally,
* `-d`
  * and deletes the origin image files, replacing with the resized ones,
* `-vvv`
  * and prints logs at verbosity level of 3,

For more argument details, type:

```bash
ibr -h
```
