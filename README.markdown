# Linenoise for rust

A minimal, zero-config, BSD licensed, readline replacement.

- Original code from Antirez, cf [antirez/linenoise](https://github.com/antirez/linenoise).
- Ported to rust by your humble servant.

# How to add the dependency in Cargo

Add the following line to you project

`
linenoise-rust = "0.2.0"
`

# Usage

- Example code: [examples/linenoise_example.rs](examples/linenoise_example.rs).

# Misc

## Why is this library not named linenoise ?

The original `linenoise` library belongs to [cmr/linenoise-rs](https://github.com/cmr/linenoise-rs) for which I have sent some pull requests. However, I'm trying to maintain my own version as the stats tends to [indicate my version](https://crates.io/search?q=linenoise) is the de factor standard.

#### TODO
- Use fork from [oldium/linenoise](oldium/linenoise) (when gcc:compile_library supports -l) : [oldium/linenoise](oldium/linenoise) does not build on OSX as is,
- Split in -sys and normal library.


#### Contributors

Linenoise was also improved by these people. Thanks to them :)

- Dan Spencer @nukep: Fixes for stable rust
- Kalyanov Dmitry @dmitryvk: Memory Leak
- Stacy Prowell: Support for history, using a custom version of linenoise
- Chris Dawes @cmsd2: win32 and arango merge help
- Bruno Ploumhans @Technici4n: fix build under MSVC
