//! # libcubeb bindings for rust
//!
//! This library contains bindings to the [cubeb][1] C library which
//! is used to interact with system audio.  The library itself is a
//! work in progress and is likely lacking documentation and test.
//!
//! [1]: https://github.com/kinetiknz/cubeb/
//!
//! The cubeb-rs library exposes the user API of libcubeb.  It doesn't
//! expose the internal interfaces, so isn't suitable for extending
//! libcubeb. See [cubeb-pulse-rs][2] for an example of extending
//! libcubeb via implementing a cubeb backend in rust.

#[macro_use]
extern crate bitflags;
extern crate cubeb_core;
extern crate libcubeb_sys as sys;

mod error;
#[macro_use]
mod call;
mod context;
mod dev_coll;
mod devices;
mod frame;
mod stream;
mod util;

pub use context::Context;
use cubeb_core::ffi;
pub use dev_coll::{DeviceCollection, DeviceInfo};
pub use devices::Device;
pub use error::Error;
pub use frame::{Frame, MonoFrame, StereoFrame};
use std::ptr;
pub use stream::{SampleType, Stream, StreamCallback, StreamInitOptions, StreamInitOptionsBuilder, StreamParams,
                 StreamParamsBuilder};
use util::Binding;

/// An enumeration of possible errors that can happen when working with cubeb.
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ErrorCode {
    /// GenericError
    Error,
    /// Requested format is invalid
    InvalidFormat,
    /// Requested parameter is invalid
    InvalidParameter,
    /// Requested operation is not supported
    NotSupported,
    /// Requested device is unavailable
    DeviceUnavailable
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum SampleFormat {
    S16LE,
    S16BE,
    S16NE,
    Float32LE,
    Float32BE,
    Float32NE
}

/// This maps to the underlying stream types on supported platforms, e.g. Android.
#[cfg(target_os = "android")]
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum StreamType {
    VoiceCall,
    System,
    Ring,
    Music,
    Alarm,
    Notification,
    BluetoothSco,
    SystemEnforced,
    Dtmf,
    Tts,
    Fm
}

/// Level (verbosity) of logging for a particular cubeb context.
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum LogLevel {
    /// Logging disabled
    Disabled,
    /// Logging lifetime operation (creation/destruction).
    Normal,
    /// Verbose logging of callbacks, can have performance implications.
    Verbose
}

/// SMPTE channel layout (also known as wave order)
/// DUAL-MONO      L   R
/// DUAL-MONO-LFE  L   R   LFE
/// MONO           M
/// MONO-LFE       M   LFE
/// STEREO         L   R
/// STEREO-LFE     L   R   LFE
/// 3F             L   R   C
/// 3F-LFE         L   R   C    LFE
/// 2F1            L   R   S
/// 2F1-LFE        L   R   LFE  S
/// 3F1            L   R   C    S
/// 3F1-LFE        L   R   C    LFE S
/// 2F2            L   R   LS   RS
/// 2F2-LFE        L   R   LFE  LS   RS
/// 3F2            L   R   C    LS   RS
/// 3F2-LFE        L   R   C    LFE  LS   RS
/// 3F3R-LFE       L   R   C    LFE  RC   LS   RS
/// 3F4-LFE        L   R   C    LFE  RLS  RRS  LS   RS
///
/// The abbreviation of channel name is defined in following table:
/// ---------------------------
/// Abbr | Channel name
/// ---------------------------
/// M    | Mono
/// L    | Left
/// R    | Right
/// C    | Center
/// LS   | Left Surround
/// RS   | Right Surround
/// RLS  | Rear Left Surround
/// RC   | Rear Center
/// RRS  | Rear Right Surround
/// LFE  | Low Frequency Effects
/// ---------------------------
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ChannelLayout {
    /// Indicate the speaker's layout is undefined.
    Undefined,
    DualMono,
    DualMonoLfe,
    Mono,
    MonoLfe,
    Stereo,
    StereoLfe,
    Layout3F,
    Layout3FLfe,
    Layout2F1,
    Layout2F1Lfe,
    Layout3F1,
    Layout3F1Lfe,
    Layout2F2,
    Layout2F2Lfe,
    Layout3F2,
    Layout3F2Lfe,
    Layout3F3RLfe,
    Layout3F4Lfe
}

/// Stream states signaled via state_callback.
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum State {
    /// Stream started.
    Started,
    /// Stream stopped.
    Stopped,
    /// Stream drained.
    Drained,
    /// Stream disabled due to error.
    Error
}

bitflags! {
    pub struct DeviceType: ffi::cubeb_device_type {
        const DEVICE_TYPE_UNKNOWN = ffi::CUBEB_DEVICE_TYPE_UNKNOWN as u32;
        const DEVICE_TYPE_INPUT = ffi::CUBEB_DEVICE_TYPE_INPUT as u32;
        const DEVICE_TYPE_OUTPUT = ffi::CUBEB_DEVICE_TYPE_OUTPUT as u32;
    }
}

/// The state of a device.
#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum DeviceState {
    /// The device has been disabled at the system level.
    Disabled,
    /// The device is enabled, but nothing is plugged into it.
    Unplugged,
    /// The device is enabled.
    Enabled
}

bitflags! {
    pub struct DeviceFormat: ffi::cubeb_device_fmt {
        const DEVICE_FMT_S16LE = ffi::CUBEB_DEVICE_FMT_S16LE;
        const DEVICE_FMT_S16BE = ffi::CUBEB_DEVICE_FMT_S16BE;
        const DEVICE_FMT_F32LE = ffi::CUBEB_DEVICE_FMT_F32LE;
        const DEVICE_FMT_F32BE = ffi::CUBEB_DEVICE_FMT_F32BE;
    }
}

bitflags! {
    pub struct DevicePref: ffi::cubeb_device_pref {
        const DEVICE_PREF_NONE = ffi::CUBEB_DEVICE_PREF_NONE;
        const DEVICE_PREF_MULTIMEDIA = ffi::CUBEB_DEVICE_PREF_MULTIMEDIA;
        const DEVICE_PREF_VOICE = ffi::CUBEB_DEVICE_PREF_VOICE;
        const DEVICE_PREF_NOTIFICATION = ffi::CUBEB_DEVICE_PREF_NOTIFICATION;
        const DEVICE_PREF_ALL = ffi::CUBEB_DEVICE_PREF_ALL;
    }
}

pub type DeviceChangedCb<'a> = FnMut() + 'a;
pub type DeviceCollectionChangedCb<'a> = FnMut(Context) + 'a;

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub struct DeviceId {
    raw: ffi::cubeb_devid
}

impl Binding for DeviceId {
    type Raw = ffi::cubeb_devid;

    unsafe fn from_raw(raw: Self::Raw) -> DeviceId {
        DeviceId {
            raw: raw
        }
    }
    fn raw(&self) -> Self::Raw {
        self.raw
    }
}

impl Default for DeviceId {
    fn default() -> Self {
        DeviceId {
            raw: ptr::null()
        }
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
