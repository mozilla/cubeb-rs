// Copyright © 2017 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.

use cubeb_core::{ChannelLayout, DeviceId, DeviceType, Result, StreamParams};
use cubeb_core::ffi;
use std::ffi::CStr;
use std::os::raw::c_void;

pub trait Context {
    fn init(context_name: Option<&CStr>) -> Result<*mut ffi::cubeb>;
    fn backend_id(&mut self) -> &'static CStr;
    fn max_channel_count(&mut self) -> Result<u32>;
    fn min_latency(&mut self, params: &StreamParams) -> Result<u32>;
    fn preferred_sample_rate(&mut self) -> Result<u32>;
    fn preferred_channel_layout(&mut self) -> Result<ChannelLayout>;
    fn enumerate_devices(
        &mut self,
        devtype: DeviceType,
    ) -> Result<ffi::cubeb_device_collection>;
    fn device_collection_destroy(
        &mut self,
        collection: *mut ffi::cubeb_device_collection,
    );
    fn stream_init(
        &mut self,
        stream_name: Option<&CStr>,
        input_device: DeviceId,
        input_stream_params: Option<&ffi::cubeb_stream_params>,
        output_device: DeviceId,
        output_stream_params: Option<&ffi::cubeb_stream_params>,
        latency_frames: u32,
        data_callback: ffi::cubeb_data_callback,
        state_callback: ffi::cubeb_state_callback,
        user_ptr: *mut c_void,
    ) -> Result<*mut ffi::cubeb_stream>;
    fn register_device_collection_changed(
        &mut self,
        devtype: DeviceType,
        cb: ffi::cubeb_device_collection_changed_callback,
        user_ptr: *mut c_void,
    ) -> Result<()>;
}

pub trait Stream {
    fn start(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn position(&self) -> Result<u64>;
    fn latency(&self) -> Result<u32>;
    fn set_volume(&mut self, volume: f32) -> Result<()>;
    fn set_panning(&mut self, panning: f32) -> Result<()>;
    fn current_device(&mut self) -> Result<*const ffi::cubeb_device>;
    fn device_destroy(&mut self, device: *const ffi::cubeb_device) -> Result<()>;
    fn register_device_changed_callback(
        &mut self,
        device_changed_callback: ffi::cubeb_device_changed_callback,
    ) -> Result<()>;
}
