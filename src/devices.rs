//! Bindings to libcubeb's raw `cubeb_device_collection` type

use {DeviceFormat, DeviceId, DevicePref, DeviceState, DeviceType, raw};
use std::marker;
use std::ops::Range;
use std::str;
use util::{Binding, opt_bytes};

/// Audio device description
pub struct Device<'a> {
    raw: *const raw::cubeb_device,
    _marker: marker::PhantomData<&'a raw::cubeb_device>
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
    type Raw = *const raw::cubeb_device;

    unsafe fn from_raw(raw: *const raw::cubeb_device) -> Device<'a> {
        Device {
            raw: raw,
            _marker: marker::PhantomData
        }
    }
    fn raw(&self) -> *const raw::cubeb_device {
        self.raw
    }
}


/// A collection of device info used by libcubeb
///
#[derive(Clone, Copy)]
pub struct DeviceCollection {
    raw: raw::cubeb_device_collection
}

/// A forward iterator over the collection.
pub struct Iter<'a> {
    range: Range<usize>,
    coll: &'a DeviceCollection
}

impl DeviceCollection {
    pub fn get<'coll>(&'coll self, i: usize) -> Option<DeviceInfo<'coll>> {
        if i < self.len() {
            unsafe {
                let ptr = self.raw.device.offset(i as isize);
                Some(DeviceInfo {
                    raw: ptr,
                    _marker: marker::PhantomData
                })
            }
        } else {
            None
        }
    }

    /// Returns an iterator over the DeviceInfo contained within this array.
    pub fn iter(&self) -> Iter {
        Iter {
            range: 0..self.len(),
            coll: self
        }
    }

    /// Returns the number of strings in this collection.
    pub fn len(&self) -> usize {
        self.raw.count
    }

    /// Return `true` if this array is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn raw(&mut self) -> *mut raw::cubeb_device_collection {
        &mut self.raw
    }
}


impl<'a> Iterator for Iter<'a> {
    type Item = DeviceInfo<'a>;
    fn next(&mut self) -> Option<DeviceInfo<'a>> {
        self.range.next().map(|i| self.coll.get(i).unwrap())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<'a> ExactSizeIterator for Iter<'a> {}


/// This structure holds the characteristicsc of an input or output
/// audio device. It is obtained using `enumerate_devices`, which
/// returns these structures via `device_collection` and must be
/// destroyed via `device_collection_destroy`.
pub struct DeviceInfo<'coll> {
    raw: *const raw::cubeb_device_info,
    _marker: marker::PhantomData<&'coll DeviceCollection>
}

impl<'coll> DeviceInfo<'coll> {
    /// Device identifier handle.
    pub fn devid(&self) -> DeviceId {
        unsafe { Binding::from_raw((*self.raw).devid) }
    }

    /// Device identifier which might be presented in a UI.
    pub fn device_id(&self) -> Option<&str> {
        self.device_id_bytes().and_then(|s| str::from_utf8(s).ok())
    }
    fn device_id_bytes(&self) -> Option<&[u8]> {
        unsafe { opt_bytes(self, (*self.raw).device_id) }
    }

    /// Friendly device name which might be presented in a UI.
    pub fn friendly_name(&self) -> Option<&str> {
        self.friendly_name_bytes().and_then(
            |s| str::from_utf8(s).ok()
        )
    }

    fn friendly_name_bytes(&self) -> Option<&[u8]> {
        unsafe { opt_bytes(self, (*self.raw).friendly_name) }
    }

    /// Two devices have the same group identifier if they belong to
    /// the same physical device; for example a headset and
    /// microphone.
    pub fn group_id(&self) -> Option<&str> {
        self.group_id_bytes().and_then(|s| str::from_utf8(s).ok())
    }

    fn group_id_bytes(&self) -> Option<&[u8]> {
        unsafe { opt_bytes(self, (*self.raw).group_id) }
    }

    /// Optional vendor name, may be NULL.
    pub fn vendor_name(&self) -> Option<&str> {
        self.vendor_name_bytes().and_then(
            |s| str::from_utf8(s).ok()
        )
    }

    fn vendor_name_bytes(&self) -> Option<&[u8]> {
        unsafe { opt_bytes(self, (*self.raw).vendor_name) }
    }

    /// Type of device (Input/Output).
    pub fn device_type(&self) -> DeviceType {
        DeviceType::from_bits_truncate(unsafe { (*self.raw).device_type })
    }

    /// State of device disabled/enabled/unplugged.
    pub fn state(&self) -> DeviceState {
        let state = unsafe { (*self.raw).state };
        macro_rules! check( ($($raw:ident => $real:ident),*) => (
            $(if state == raw::$raw {
                super::DeviceState::$real
            }) else *
            else {
                panic!("unknown device state: {}", state)
            }
        ));

        check!(CUBEB_DEVICE_STATE_DISABLED => Disabled ,
               CUBEB_DEVICE_STATE_UNPLUGGED => Unplugged,
               CUBEB_DEVICE_STATE_ENABLED => Enabled)
    }

    /// Preferred device.
    pub fn preferred(&self) -> DevicePref {
        DevicePref::from_bits(unsafe { (*self.raw).preferred }).unwrap()
    }

    /// Sample format supported.
    pub fn format(&self) -> DeviceFormat {
        DeviceFormat::from_bits(unsafe { (*self.raw).format }).unwrap()
    }

    /// The default sample format for this device.
    pub fn default_format(&self) -> DeviceFormat {
        DeviceFormat::from_bits(unsafe { (*self.raw).default_format }).unwrap()
    }

    /// Channels.
    pub fn max_channels(&self) -> u32 {
        unsafe { (*self.raw).max_channels }
    }

    /// Default/Preferred sample rate.
    pub fn default_rate(&self) -> u32 {
        unsafe { (*self.raw).default_rate }
    }

    /// Maximum sample rate supported.
    pub fn max_rate(&self) -> u32 {
        unsafe { (*self.raw).max_rate }
    }

    /// Minimum sample rate supported.
    pub fn min_rate(&self) -> u32 {
        unsafe { (*self.raw).min_rate }
    }

    /// Lowest possible latency in frames.
    pub fn latency_lo(&self) -> u32 {
        unsafe { (*self.raw).latency_lo }
    }

    /// Higest possible latency in frames.
    pub fn latency_hi(&self) -> u32 {
        unsafe { (*self.raw).latency_hi }
    }
}

impl<'coll> Binding for DeviceInfo<'coll> {
    type Raw = *const raw::cubeb_device_info;
    unsafe fn from_raw(raw: *const raw::cubeb_device_info) -> Self {
        DeviceInfo {
            raw: raw,
            _marker: marker::PhantomData
        }
    }
    fn raw(&self) -> Self::Raw {
        self.raw
    }
}
