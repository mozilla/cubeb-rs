#![allow(bad_style, unused_macros, unused_imports, clippy::all)]

extern crate cubeb_sys;

use cubeb_sys::*;
use std::os::raw::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
