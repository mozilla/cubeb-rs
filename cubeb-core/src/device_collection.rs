// Copyright © 2017-2018 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.

use ffi;
use std::mem::MaybeUninit;
use {ContextRef, DeviceInfo, DeviceType, Error, Result};

/// A collection of `DeviceInfo` used by libcubeb
#[derive(Debug)]
pub struct DeviceCollection<'ctx> {
    inner: &'ctx mut [DeviceInfo],
    ctx: &'ctx ContextRef,
}

impl DeviceCollection<'_> {
    pub(crate) fn new(ctx: &ContextRef, devtype: DeviceType) -> Result<DeviceCollection<'_>> {
        let mut coll = MaybeUninit::uninit();
        unsafe {
            Error::wrap(ffi::cubeb_enumerate_devices(
                ctx.as_ptr(),
                devtype.bits(),
                coll.as_mut_ptr(),
            ))?;
        }

        // SAFETY: It is the responsibility of the cubeb_enumerate_devices to initialize the
        // device collection struct with a valid array of device infos.
        let inner = unsafe {
            let coll = coll.assume_init();
            std::slice::from_raw_parts_mut(coll.device as *mut _, coll.count)
        };
        Ok(DeviceCollection { inner, ctx })
    }
}

impl Drop for DeviceCollection<'_> {
    fn drop(&mut self) {
        let mut coll = ffi::cubeb_device_collection {
            device: self.inner.as_mut_ptr() as *mut _,
            count: self.inner.len(),
        };
        unsafe {
            // This drops the self.inner, do not interact with it past this point
            let res = ffi::cubeb_device_collection_destroy(self.ctx.as_ptr(), &mut coll);
            debug_assert!(res == 0)
        }
    }
}

impl ::std::ops::Deref for DeviceCollection<'_> {
    type Target = [DeviceInfo];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
