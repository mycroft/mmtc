# mmtc

[![release](https://img.shields.io/github/v/release/figsoda/mmtc?logo=github&style=flat-square)](https://github.com/figsoda/mmtc/releases)
[![version](https://img.shields.io/crates/v/mmtc?logo=rust&style=flat-square)][Crate]
[![dependencies](https://img.shields.io/librariesio/release/cargo/mmtc?style=flat-square)](https://libraries.io/cargo/mmtc)
[![license](https://img.shields.io/badge/license-MPL--2.0-blue?style=flat-square)](https://www.mozilla.org/en-US/MPL/2.0)
[![ci](https://img.shields.io/github/workflow/status/figsoda/mmtc/ci?label=ci&logo=github-actions&style=flat-square)](https://github.com/figsoda/mmtc/actions?query=workflow:ci)

Minimal mpd terminal client that aims to be simple yet highly configurable

- [Installation](#Installation)
- [Building from source](#Building-from-source)
- [Key bindings](#Key-bindings)
- [Configuration.md]
- [CHANGELOG.md]

## Installation

The latest precompiled binaries are available on [github](https://github.com/figsoda/mmtc/releases/latest).

Alternatively you can install mmtc from [crates.io][Crate] with cargo. This requires the nightly toolchain of Rust.

```shell
cargo +nightly install mmtc
```


## Building from source

This requires the nightly toolchain of Rust.

```shell
cargo +nightly build --release
```


## Usage

```
mmtc [FLAGS] [OPTIONS]
```

flag | description
-|-
--cycle | Cycle through the queue
-h, --help | Prints help information
--no-cycle | Don't cycle through the queue
-V, --version | Prints version information

option | description
-|-
--address \<ADDRESS> | Specify the address of the mpd server
-c, --config \<FILE> | Specify the config file
--jump-lines \<NUMBER> | The number of lines to jump
--seek-secs \<NUMBER> | The time to seek in seconds
--ups \<NUMBER> | The amount of status updates per second


## Key bindings

Key | Action
-|-
`q` | quit mmtc
`r` | toggle repeat
`R` | toggle random
`s` | toggle single
`S` | toggle oneshot
`c` | toggle consume
`p` | toggle pause
`;` | stop
`h`, `Left` | seek backwards
`l`, `Right` | seek forwards
`H` | previous song
`L` | next song
`Enter` | play selected song or quit searching mode if in searching mode
`Space` | select current song or the first song in the queue
`j`, `Down`, `ScrollDown` | go down in the queue
`k`, `Up`, `ScrollUp` | go up in the queue
`J`, `PageDown` | jump down in the queue
`K`, `PageUp` | jump up in the queue
`/` | enter searching mode
`Escape` | quit searching mode and empty query


## Configuration

See [Configuration.md]


## Changelog

See [CHANGELOG.md]


[Crate]: https://crates.io/crates/mmtc
[Configuration.md]: https://github.com/figsoda/mmtc/blob/main/Configuration.md
[CHANGELOG.md]: https://github.com/figsoda/mmtc/blob/main/CHANGELOG.md
