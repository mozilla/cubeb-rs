// Copyright © 2017 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.

#[macro_use]
extern crate bitflags;

pub mod ffi;
pub mod binding;
mod error;
mod util;

use binding::Binding;
pub use error::Error;
use std::{marker, ptr, str};
use util::opt_bytes;

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

/// Stream format initialization parameters.
#[derive(Clone, Copy)]
pub struct StreamParams {
    raw: ffi::cubeb_stream_params
}

impl StreamParams {
    pub fn format(&self) -> SampleFormat {
        macro_rules! check( ($($raw:ident => $real:ident),*) => (
            $(if self.raw.format == ffi::$raw {
                SampleFormat::$real
            }) else *
            else {
                panic!("unknown sample format: {}", self.raw.format)
            }
        ) );

        check!(
            CUBEB_SAMPLE_S16LE => S16LE,
            CUBEB_SAMPLE_S16BE => S16BE,
            CUBEB_SAMPLE_FLOAT32LE => Float32LE,
            CUBEB_SAMPLE_FLOAT32BE => Float32BE
        )
    }

    pub fn rate(&self) -> u32 {
        self.raw.rate as u32
    }

    pub fn channels(&self) -> u32 {
        self.raw.channels as u32
    }

    pub fn layout(&self) -> ChannelLayout {
        macro_rules! check( ($($raw:ident => $real:ident),*) => (
            $(if self.raw.layout == ffi::$raw {
                ChannelLayout::$real
            }) else *
            else {
                panic!("unknown channel layout: {}", self.raw.layout)
            }
        ) );

        check!(CUBEB_LAYOUT_UNDEFINED => Undefined,
               CUBEB_LAYOUT_DUAL_MONO => DualMono,
               CUBEB_LAYOUT_DUAL_MONO_LFE => DualMonoLfe,
               CUBEB_LAYOUT_MONO => Mono,
               CUBEB_LAYOUT_MONO_LFE => MonoLfe,
               CUBEB_LAYOUT_STEREO => Stereo,
               CUBEB_LAYOUT_STEREO_LFE => StereoLfe,
               CUBEB_LAYOUT_3F => Layout3F,
               CUBEB_LAYOUT_3F_LFE => Layout3FLfe,
               CUBEB_LAYOUT_2F1 => Layout2F1,
               CUBEB_LAYOUT_2F1_LFE => Layout2F1Lfe,
               CUBEB_LAYOUT_3F1 => Layout3F1,
               CUBEB_LAYOUT_3F1_LFE => Layout3F1Lfe,
               CUBEB_LAYOUT_2F2 => Layout2F2,
               CUBEB_LAYOUT_2F2_LFE => Layout2F2Lfe,
               CUBEB_LAYOUT_3F2 => Layout3F2,
               CUBEB_LAYOUT_3F2_LFE => Layout3F2Lfe,
               CUBEB_LAYOUT_3F3R_LFE => Layout3F3RLfe,
               CUBEB_LAYOUT_3F4_LFE => Layout3F4Lfe)
    }

    #[cfg(target_os = "android")]
    pub fn stream_type(&self) -> StreamType {
        macro_rules! check( ($($raw:ident => $real:ident),*) => (
            $(if self.raw.stream_type == raw::$raw {
                super::StreamType::$real
            }) else *
            else {
                panic!("unknown stream type: {}", self.raw.stream_type)
            }
        ) );

        check!(CUBEB_STREAM_TYPE_VOICE_CALL => VoiceCall,
               CUBEB_STREAM_TYPE_SYSTEM => System,
               CUBEB_STREAM_TYPE_RING => Ring,
               CUBEB_STREAM_TYPE_MUSIC => Music,
               CUBEB_STREAM_TYPE_ALARM => Alarm,
               CUBEB_STREAM_TYPE_NOTIFICATION => Notification,
               CUBEB_STREAM_TYPE_BLUETOOTH_SCO => BluetoothSco,
               CUBEB_STREAM_TYPE_SYSTEM_ENFORCED => SystemEnforced,
               CUBEB_STREAM_TYPE_DTMF => Dtmf,
               CUBEB_STREAM_TYPE_TTS => Tts,
               CUBEB_STREAM_TYPE_FM => Fm)
    }
}

impl Binding for StreamParams {
    type Raw = *const ffi::cubeb_stream_params;
    unsafe fn from_raw(raw: *const ffi::cubeb_stream_params) -> Self {
        Self {
            raw: *raw
        }
    }
    fn raw(&self) -> Self::Raw {
        &self.raw as Self::Raw
    }
}

/// Audio device description
pub struct Device<'a> {
    raw: *const ffi::cubeb_device,
    _marker: marker::PhantomData<&'a ffi::cubeb_device>
}

impl<'a> Device<'a> {
    /// Gets the output device name.
    ///
    /// May return `None` if there is no output device.
    pub fn output_name(&self) -> Option<&str> {
        self.output_name_bytes().map(|b| str::from_utf8(b).unwrap())
    }

    fn output_name_bytes(&self) -> Option<&[u8]> {
        unsafe { opt_bytes(self, (*self.raw).output_name) }
    }

    /// Gets the input device name.
    ///
    /// May return `None` if there is no input device.
    pub fn input_name(&self) -> Option<&str> {
        self.input_name_bytes().map(|b| str::from_utf8(b).unwrap())
    }

    fn input_name_bytes(&self) -> Option<&[u8]> {
        unsafe { opt_bytes(self, (*self.raw).input_name) }
    }
}

impl<'a> Binding for Device<'a> {
    type Raw = *const ffi::cubeb_device;

    unsafe fn from_raw(raw: *const ffi::cubeb_device) -> Device<'a> {
        Device {
            raw: raw,
            _marker: marker::PhantomData
        }
    }
    fn raw(&self) -> *const ffi::cubeb_device {
        self.raw
    }
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

#[cfg(test)]
mod tests {
    use binding::Binding;
    use std::mem;

    #[test]
    fn stream_params_raw_channels() {
        let mut raw: super::ffi::cubeb_stream_params = unsafe { mem::zeroed() };
        raw.channels = 2;
        let params = unsafe { super::StreamParams::from_raw(&raw as *const _) };
        assert_eq!(params.channels(), 2);
    }

    #[test]
    fn stream_params_raw_format() {
        let mut raw: super::ffi::cubeb_stream_params = unsafe { mem::zeroed() };
        macro_rules! check(
            ($($raw:ident => $real:ident),*) => (
                $(raw.format = super::ffi::$raw;
                  let params = unsafe {
                      super::StreamParams::from_raw(&raw as *const _)
                  };
                  assert_eq!(params.format(), super::SampleFormat::$real);
                )*
            ) );

        check!(CUBEB_SAMPLE_S16LE => S16LE,
               CUBEB_SAMPLE_S16BE => S16BE,
               CUBEB_SAMPLE_FLOAT32LE => Float32LE,
               CUBEB_SAMPLE_FLOAT32BE => Float32BE);
    }

    #[test]
    fn stream_params_raw_format_native_endian() {
        let mut raw: super::ffi::cubeb_stream_params = unsafe { mem::zeroed() };
        raw.format = super::ffi::CUBEB_SAMPLE_S16NE;
        let params = unsafe { super::StreamParams::from_raw(&raw as *const _) };
        assert_eq!(
            params.format(),
            if cfg!(target_endian = "little") {
                super::SampleFormat::S16LE
            } else {
                super::SampleFormat::S16BE
            }
        );

        raw.format = super::ffi::CUBEB_SAMPLE_FLOAT32NE;
        let params = unsafe { super::StreamParams::from_raw(&raw as *const _) };
        assert_eq!(
            params.format(),
            if cfg!(target_endian = "little") {
                super::SampleFormat::Float32LE
            } else {
                super::SampleFormat::Float32BE
            }
        );
    }

    #[test]
    fn stream_params_raw_layout() {
        let mut raw: super::ffi::cubeb_stream_params = unsafe { mem::zeroed() };
        macro_rules! check(
            ($($raw:ident => $real:ident),*) => (
                $(raw.layout = super::ffi::$raw;
                  let params = unsafe {
                      super::StreamParams::from_raw(&raw as *const _)
                  };
                  assert_eq!(params.layout(), super::ChannelLayout::$real);
                )*
            ) );

        check!(CUBEB_LAYOUT_UNDEFINED => Undefined,
               CUBEB_LAYOUT_DUAL_MONO => DualMono,
               CUBEB_LAYOUT_DUAL_MONO_LFE => DualMonoLfe,
               CUBEB_LAYOUT_MONO => Mono,
               CUBEB_LAYOUT_MONO_LFE => MonoLfe,
               CUBEB_LAYOUT_STEREO => Stereo,
               CUBEB_LAYOUT_STEREO_LFE => StereoLfe,
               CUBEB_LAYOUT_3F => Layout3F,
               CUBEB_LAYOUT_3F_LFE => Layout3FLfe,
               CUBEB_LAYOUT_2F1 => Layout2F1,
               CUBEB_LAYOUT_2F1_LFE => Layout2F1Lfe,
               CUBEB_LAYOUT_3F1 => Layout3F1,
               CUBEB_LAYOUT_3F1_LFE => Layout3F1Lfe,
               CUBEB_LAYOUT_2F2 => Layout2F2,
               CUBEB_LAYOUT_2F2_LFE => Layout2F2Lfe,
               CUBEB_LAYOUT_3F2 => Layout3F2,
               CUBEB_LAYOUT_3F2_LFE => Layout3F2Lfe,
               CUBEB_LAYOUT_3F3R_LFE => Layout3F3RLfe,
               CUBEB_LAYOUT_3F4_LFE => Layout3F4Lfe);
    }

    #[test]
    fn stream_params_raw_rate() {
        let mut raw: super::ffi::cubeb_stream_params = unsafe { mem::zeroed() };
        raw.rate = 44100;
        let params = unsafe { super::StreamParams::from_raw(&raw as *const _) };
        assert_eq!(params.rate(), 44100);
    }

}
