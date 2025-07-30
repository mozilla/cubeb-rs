//! # libcubeb bindings for rust
//!
//! This library contains bindings to the [cubeb][1] C library which
//! is used to interact with system audio.  The library itself is a
//! work in progress and is likely lacking documentation and test.
//!
//! [1]: https://github.com/mozilla/cubeb/
//!
//! The cubeb-rs library exposes the user API of libcubeb.  It doesn't
//! expose the internal interfaces, so isn't suitable for extending
//! libcubeb. See [cubeb-pulse-rs][2] for an example of extending
//! libcubeb via implementing a cubeb backend in rust.
//!
//! [2]: https://github.com/mozilla/cubeb-pulse-rs/
//!
//! To get started, have a look at the [`StreamBuilder`]

// Copyright © 2017-2018 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.

extern crate cubeb_core;

mod context;
mod device_collection;
mod entry;
mod frame;
mod sample;
mod stream;

pub use context::Context;
pub use device_collection::DeviceCollection;
pub use entry::init;
// Re-export cubeb_core types
pub use cubeb_core::{
    ffi, ChannelLayout, Device, DeviceFormat, DeviceId, DeviceInfo, DeviceRef, DeviceState,
    DeviceType, Error, LogLevel, Result, SampleFormat, State, StreamParams, StreamParamsBuilder,
    StreamParamsRef, StreamPrefs, StreamRef,
};
pub use frame::*;
pub use sample::*;
pub use stream::*;
