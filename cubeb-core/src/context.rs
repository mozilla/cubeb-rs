// Copyright © 2017-2018 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.

use ffi;
use std::ffi::CStr;
use std::os::raw::c_void;
use std::{ptr, str};
use Error;

use {
    DeviceCollection, DeviceId, DeviceType, InputProcessingParams, Result, Stream, StreamParamsRef,
};

macro_rules! as_ptr {
    ($e:expr) => {
        $e.map(|s| s.as_ptr()).unwrap_or(ptr::null_mut())
    };
}

#[derive(Debug)]
pub struct Context(pub(crate) *mut ffi::cubeb);

impl Context {
    pub fn init(context_name: Option<&CStr>, backend_name: Option<&CStr>) -> Result<Self> {
        let mut context: *mut ffi::cubeb = ptr::null_mut();
        let context_name = as_ptr!(context_name);
        let backend_name = as_ptr!(backend_name);
        unsafe {
            Error::wrap(ffi::cubeb_init(&mut context, context_name, backend_name))?;
            Ok(Context(context))
        }
    }

    pub fn backend_id(&self) -> &str {
        // SAFETY: The returned pointer is guaranted to be a valid, read-only C string
        unsafe { CStr::from_ptr(ffi::cubeb_get_backend_id(self.0)) }
            .to_str()
            .expect("Backend ID is not a valid UTF-8 string.")
    }

    pub fn max_channel_count(&self) -> Result<u32> {
        let mut channel_count = 0u32;
        unsafe {
            Error::wrap(ffi::cubeb_get_max_channel_count(self.0, &mut channel_count))?;
        }
        Ok(channel_count)
    }

    pub fn min_latency(&self, params: &StreamParamsRef) -> Result<u32> {
        let mut latency = 0u32;
        unsafe {
            Error::wrap(ffi::cubeb_get_min_latency(
                self.0,
                params.as_ptr(),
                &mut latency,
            ))?;
        }
        Ok(latency)
    }

    pub fn preferred_sample_rate(&self) -> Result<u32> {
        let mut rate = 0u32;
        unsafe {
            Error::wrap(ffi::cubeb_get_preferred_sample_rate(self.0, &mut rate))?;
        }
        Ok(rate)
    }

    pub fn supported_input_processing_params(&self) -> Result<InputProcessingParams> {
        let mut params = ffi::CUBEB_INPUT_PROCESSING_PARAM_NONE;
        unsafe {
            Error::wrap(ffi::cubeb_get_supported_input_processing_params(
                self.0,
                &mut params,
            ))?;
        };
        Ok(InputProcessingParams::from_bits_truncate(params))
    }

    /// # Safety
    ///
    /// This function is unsafe because it dereferences the given `data_callback`, `state_callback`, and `user_ptr` pointers.
    /// The caller should ensure those pointers are valid.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn stream_init(
        &self,
        stream_name: Option<&CStr>,
        input_device: DeviceId,
        input_stream_params: Option<&StreamParamsRef>,
        output_device: DeviceId,
        output_stream_params: Option<&StreamParamsRef>,
        latency_frames: u32,
        data_callback: ffi::cubeb_data_callback,
        state_callback: ffi::cubeb_state_callback,
        user_ptr: *mut c_void,
    ) -> Result<Stream> {
        let mut stm: *mut ffi::cubeb_stream = ptr::null_mut();

        let stream_name = as_ptr!(stream_name);
        let input_stream_params = as_ptr!(input_stream_params);
        let output_stream_params = as_ptr!(output_stream_params);

        Error::wrap(ffi::cubeb_stream_init(
            self.0,
            &mut stm,
            stream_name,
            input_device,
            input_stream_params,
            output_device,
            output_stream_params,
            latency_frames,
            data_callback,
            state_callback,
            user_ptr,
        ))?;
        Ok(Stream::from_ptr(stm))
    }

    pub fn enumerate_devices(&self, devtype: DeviceType) -> Result<DeviceCollection<'_>> {
        DeviceCollection::new(self, devtype)
    }

    /// # Safety
    ///
    /// This function is unsafe because it dereferences the given `callback` and  `user_ptr` pointers.
    /// The caller should ensure those pointers are valid.
    pub unsafe fn register_device_collection_changed(
        &self,
        devtype: DeviceType,
        callback: ffi::cubeb_device_collection_changed_callback,
        user_ptr: *mut c_void,
    ) -> Result<()> {
        Error::wrap(ffi::cubeb_register_device_collection_changed(
            self.0,
            devtype.bits(),
            callback,
            user_ptr,
        ))
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { ffi::cubeb_destroy(self.0) }
    }
}
