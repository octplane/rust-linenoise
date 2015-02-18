# Linenoise for rust

A minimal, zero-config, BSD licensed, readline replacement.

- Original code from Antirez, cf [antirez/linenoise](https://github.com/antirez/linenoise).
- Ported to rust by your humble servant.

# How to add the dependency in Cargo

Add the following line to you project

`
linenoise-rust = "0.1.5"
`

# Usage

- example code [src/linenoise.rs](src/linenoise.rs).
- [Api documentation](http://octplane.github.io/rust-linenoise/linenoise/index.html)

# Misc

## Why is this library not named linenoise ?

The original `linenoise` library belongs to [cmr/linenoise-rs](https://github.com/cmr/linenoise-rs) for which I have sent some pull requests. However, I'm trying to maintain my own version as the stats tends to [indicate my version](https://crates.io/search?q=linenoise) is the de factor standard.

#### TODO
- Use fork from [oldium/linenoise](oldium/linenoise) (when gcc:compile_library supports -l) : [oldium/linenoise](oldium/linenoise) does not build on OSX as is,
- Split in -sys and normal library.
