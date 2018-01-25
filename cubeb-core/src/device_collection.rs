// Copyright © 2017-2018 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.

use DeviceInfo;
use ffi;
use std::{ops, slice};

/// A collection of `DeviceInfo` used by libcubeb
ffi_type_stack! {
    type CType = ffi::cubeb_device_collection;
    #[derive(Debug)]
    pub struct DeviceCollection;
    pub struct DeviceCollectionRef;
}

impl ops::Deref for DeviceCollectionRef {
    type Target = [DeviceInfo];
    fn deref(&self) -> &[DeviceInfo] {
        unsafe {
            let coll: &ffi::cubeb_device_collection = &*self.as_ptr();
            slice::from_raw_parts(coll.device as *const DeviceInfo, coll.count)
        }
    }
}
