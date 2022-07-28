// Copyright © 2017-2018 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.

use ffi;

/// Sample format enumeration
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum SampleFormat {
    /// 16bit signed integer little endian
    S16LE,
    /// 16bit signed integer big endian
    S16BE,
    /// 32bit float little endian
    Float32LE,
    /// 32bit float big endian
    Float32BE,
    // Maps to the platform native endian
    /// 16bit signed integer native endian - maps to flatform specific type
    S16NE,
    /// 32bit float native endian - maps to flatform specific type
    Float32NE,
}

impl From<ffi::cubeb_sample_format> for SampleFormat {
    fn from(x: ffi::cubeb_sample_format) -> SampleFormat {
        match x {
            ffi::CUBEB_SAMPLE_S16LE => SampleFormat::S16LE,
            ffi::CUBEB_SAMPLE_S16BE => SampleFormat::S16BE,
            ffi::CUBEB_SAMPLE_FLOAT32LE => SampleFormat::Float32LE,
            ffi::CUBEB_SAMPLE_FLOAT32BE => SampleFormat::Float32BE,
            // TODO: Implement TryFrom
            _ => SampleFormat::S16NE,
        }
    }
}

impl From<SampleFormat> for ffi::cubeb_sample_format {
    fn from(x: SampleFormat) -> Self {
        use SampleFormat::*;
        match x {
            S16LE => ffi::CUBEB_SAMPLE_S16LE,
            S16BE => ffi::CUBEB_SAMPLE_S16BE,
            Float32LE => ffi::CUBEB_SAMPLE_FLOAT32LE,
            Float32BE => ffi::CUBEB_SAMPLE_FLOAT32BE,
            S16NE => ffi::CUBEB_SAMPLE_S16NE,
            Float32NE => ffi::CUBEB_SAMPLE_FLOAT32NE,
        }
    }
}
