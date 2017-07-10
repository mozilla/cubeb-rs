// Copyright © 2017 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.

extern crate cubeb_core;

pub mod ffi;
mod traits;

pub use traits::{Context, Stream};
