#![allow(bad_style, unused_macros)]

extern crate cubeb_sys;

use cubeb_sys::*;
use std::os::raw::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
