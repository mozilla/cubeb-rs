# cubeb-rs [![ISC License](https://img.shields.io/crates/l/cubeb.svg)](https://github.com/djg/cubeb-rs/blob/master/LICENSE)

A cross-platform audio library in Rust.

## Features

Provides access to the following:

- Multiple audio backends across multiple platforms. See
  [here](https://github.com/mozilla/cubeb/wiki/Backend-Support) for details.
- Enumeration of available audio devices.
- Opening input, output and duplex audio streams with control over latency,
  sample rate, channel layout, state transitions, data handling and more.

## Goals

Currently, **cubeb-rs** is based on a set of bindings to the original [**cubeb**
C++ library](https://github.com/mozilla/cubeb) most notable for its use as the
audio backend within
[Gecko](https://github.com/mozilla/gecko-dev/search?q=cubeb&unscoped_q=cubeb),
Mozilla's browser engine. The long-term goal for **cubeb-rs** is to become
independent of the C++ library and provide a pure-Rust implementation down to
the level of the platform API eventually replacing the original within Gecko
where possible.

In order to achieve this goal **cubeb-rs** is structured in a manner that
supports backend implementations in both pure-Rust and via bindings to the C++
implementation, allowing for progressive replacement. So far, pure-Rust
implementations exist for:

- CoreAudio https://github.com/mozilla/cubeb-coreaudio-rs
- PulseAudio https://github.com/mozilla/cubeb-pulse-rs

The plan is to consolidate all **cubeb**-related projects (including backend
implementations) under a single repository
[here](https://github.com/mozilla/cubeb) in the near future.

While **cubeb** is primarily renown for its use within Gecko, contributions and
use from projects outside of Gecko is very welcome.

## Crates

The following crates are included within this repository:

| Crate | Links | Description |
| --- | --- | --- |
| `cubeb` | [![crates.io](https://img.shields.io/crates/v/cubeb.svg)](https://crates.io/crates/cubeb) [![docs.rs](https://docs.rs/cubeb/badge.svg)](https://docs.rs/cubeb) | The top-level user API for **cubeb-rs**. See the `cubeb-api` subdirectory. Depends on `cubeb-core`. |
| `cubeb-core` | [![crates.io](https://img.shields.io/crates/v/cubeb-core.svg)](https://crates.io/crates/cubeb-core) [![docs.rs](https://docs.rs/cubeb-core/badge.svg)](https://docs.rs/cubeb-core) | Common types and definitions for cubeb rust and C bindings. Not intended for direct use. Depends on `cubeb-sys`. |
| `cubeb-sys` | [![crates.io](https://img.shields.io/crates/v/cubeb-sys.svg)](https://crates.io/crates/cubeb-sys) [![docs.rs](https://docs.rs/cubeb-sys/badge.svg)](https://docs.rs/cubeb-sys) | Native bindings to the cubeb C++ library. Requires `pkg-config` and `cmake` |
| `cubeb-backend` | [![crates.io](https://img.shields.io/crates/v/cubeb-backend.svg)](https://crates.io/crates/cubeb-backend) [![docs.rs](https://docs.rs/cubeb-backend/badge.svg)](https://docs.rs/cubeb-backend) | Bindings to libcubeb internals to facilitate implementing cubeb backends in Rust. Depends on `cubeb-core`. |
