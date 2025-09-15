// Copyright © 2017-2018 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.
#![macro_use]

use crate::Error;
use std::os::raw::c_int;

pub fn cvt_r(ret: c_int) -> Result<(), Error> {
    Error::wrap(ret)
}

macro_rules! call {
    (ffi::$p:ident ($($e:expr),*)) => ({
        crate::call::cvt_r(ffi::$p($($e),*))
    })
}
