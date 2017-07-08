//! Bindings to libcubeb's raw `cubeb_device_collection` type

use {Binding, Context};
use cubeb_core::{DeviceFormat, DeviceId, DevicePref, DeviceState, DeviceType, Result};
use ffi;
use std::{ptr, slice, str};
use std::ops::Deref;
use sys;
use util::opt_bytes;

/// This structure holds the characteristicsc of an input or output
/// audio device. It is obtained using `enumerate_devices`, which
/// returns these structures via `device_collection` and must be
/// destroyed via `device_collection_destroy`.
pub struct DeviceInfo {
    raw: ffi::cubeb_device_info
}

//impl<'coll, 'ctx> DeviceInfo<'coll, 'ctx> {
impl DeviceInfo {
    /// Device identifier handle.
    pub fn devid(&self) -> DeviceId {
        unsafe { Binding::from_raw(self.raw.devid) }
    }

    /// Device identifier which might be presented in a UI.
    pub fn device_id(&self) -> Option<&str> {
        self.device_id_bytes().and_then(|s| str::from_utf8(s).ok())
    }

    fn device_id_bytes(&self) -> Option<&[u8]> {
        unsafe { opt_bytes(self, self.raw.device_id) }
    }

    /// Friendly device name which might be presented in a UI.
    pub fn friendly_name(&self) -> Option<&str> {
        self.friendly_name_bytes().and_then(
            |s| str::from_utf8(s).ok()
        )
    }

    fn friendly_name_bytes(&self) -> Option<&[u8]> {
        unsafe { opt_bytes(self, self.raw.friendly_name) }
    }

    /// Two devices have the same group identifier if they belong to
    /// the same physical device; for example a headset and
    /// microphone.
    pub fn group_id(&self) -> Option<&str> {
        self.group_id_bytes().and_then(|s| str::from_utf8(s).ok())
    }

    fn group_id_bytes(&self) -> Option<&[u8]> {
        unsafe { opt_bytes(self, self.raw.group_id) }
    }

    /// Optional vendor name, may be NULL.
    pub fn vendor_name(&self) -> Option<&str> {
        self.vendor_name_bytes().and_then(
            |s| str::from_utf8(s).ok()
        )
    }

    fn vendor_name_bytes(&self) -> Option<&[u8]> {
        unsafe { opt_bytes(self, self.raw.vendor_name) }
    }

    /// Type of device (Input/Output).
    pub fn device_type(&self) -> DeviceType {
        DeviceType::from_bits_truncate(self.raw.device_type)
    }

    /// State of device disabled/enabled/unplugged.
    pub fn state(&self) -> DeviceState {
        let state = self.raw.state;
        macro_rules! check( ($($raw:ident => $real:ident),*) => (
            $(if state == ffi::$raw {
                DeviceState::$real
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
        DevicePref::from_bits(self.raw.preferred).unwrap()
    }

    /// Sample format supported.
    pub fn format(&self) -> DeviceFormat {
        DeviceFormat::from_bits(self.raw.format).unwrap()
    }

    /// The default sample format for this device.
    pub fn default_format(&self) -> DeviceFormat {
        DeviceFormat::from_bits(self.raw.default_format).unwrap()
    }

    /// Channels.
    pub fn max_channels(&self) -> u32 {
        self.raw.max_channels
    }

    /// Default/Preferred sample rate.
    pub fn default_rate(&self) -> u32 {
        self.raw.default_rate
    }

    /// Maximum sample rate supported.
    pub fn max_rate(&self) -> u32 {
        self.raw.max_rate
    }

    /// Minimum sample rate supported.
    pub fn min_rate(&self) -> u32 {
        self.raw.min_rate
    }

    /// Lowest possible latency in frames.
    pub fn latency_lo(&self) -> u32 {
        self.raw.latency_lo
    }

    /// Higest possible latency in frames.
    pub fn latency_hi(&self) -> u32 {
        self.raw.latency_hi
    }
}

/// A collection of `DeviceInfo` used by libcubeb
pub struct DeviceCollection<'coll, 'ctx> {
    coll: &'coll [DeviceInfo],
    ctx: &'ctx Context
}

impl<'coll, 'ctx> DeviceCollection<'coll, 'ctx> {
    fn new(ctx: &'ctx Context, devtype: DeviceType) -> Result<DeviceCollection> {
        let mut coll = ffi::cubeb_device_collection {
            device: ptr::null(),
            count: 0
        };
        let devices = unsafe {
            try_call!(sys::cubeb_enumerate_devices(
                ctx.raw(),
                devtype.bits(),
                &mut coll
            ));
            slice::from_raw_parts(coll.device as *const _, coll.count)
        };
        Ok(DeviceCollection {
            coll: devices,
            ctx: ctx
        })
    }
}

impl<'coll, 'ctx> Deref for DeviceCollection<'coll, 'ctx> {
    type Target = [DeviceInfo];
    fn deref(&self) -> &[DeviceInfo] {
        self.coll
    }
}

impl<'coll, 'ctx> Drop for DeviceCollection<'coll, 'ctx> {
    fn drop(&mut self) {
        let mut coll = ffi::cubeb_device_collection {
            device: self.coll.as_ptr() as *const _,
            count: self.coll.len()
        };
        unsafe {
            call!(sys::cubeb_device_collection_destroy(
                self.ctx.raw(),
                &mut coll
            ));
        }
    }
}

pub fn enumerate(ctx: &Context, devtype: DeviceType) -> Result<DeviceCollection> {
    DeviceCollection::new(ctx, devtype)
}
