use {ChannelLayout, DeviceCollection, DeviceType, Result};
use {Stream, StreamInitOptions, StreamParams};
use raw;
use std::{ptr, str};
use std::ffi::CString;
use stream::{StreamCallback, stream_init};
use util::{Binding, opt_bytes, opt_cstr};

pub struct Context {
    raw: *mut raw::cubeb
}

impl Context {
    pub fn init(context_name: &str, backend_name: Option<&str>) -> Result<Context> {
        let mut context: *mut raw::cubeb = ptr::null_mut();
        let context_name = try!(CString::new(context_name));
        let backend_name = try!(opt_cstr(backend_name));
        unsafe {
            try_call!(raw::cubeb_init(&mut context, context_name, backend_name));
            Ok(Binding::from_raw(context))
        }
    }

    pub fn backend_id(&self) -> &str {
        str::from_utf8(self.backend_id_bytes()).unwrap()
    }
    pub fn backend_id_bytes(&self) -> &[u8] {
        unsafe { opt_bytes(self, call!(raw::cubeb_get_backend_id(self.raw))).unwrap() }
    }

    pub fn max_channel_count(&self) -> Result<u32> {
        let mut channel_count = 0u32;
        unsafe {
            try_call!(raw::cubeb_get_max_channel_count(
                self.raw,
                &mut channel_count
            ));
        }
        Ok(channel_count)
    }

    pub fn min_latency(&self, params: &StreamParams) -> Result<u32> {
        let mut latency = 0u32;
        unsafe {
            try_call!(raw::cubeb_get_min_latency(
                self.raw,
                params.raw(),
                &mut latency
            ));
        }
        Ok(latency)
    }

    pub fn preferred_sample_rate(&self) -> Result<u32> {
        let mut rate = 0u32;
        unsafe {
            try_call!(raw::cubeb_get_preferred_sample_rate(self.raw, &mut rate));
        }
        Ok(rate)
    }

    pub fn preferred_channel_layout(&self) -> Result<ChannelLayout> {
        let mut layout: raw::cubeb_channel_layout = raw::CUBEB_LAYOUT_UNDEFINED;
        unsafe {
            try_call!(raw::cubeb_get_preferred_channel_layout(
                self.raw,
                &mut layout
            ));
        }
        macro_rules! check( ($($raw:ident => $real:ident),*) => (
            $(if layout == raw::$raw {
                Ok(super::ChannelLayout::$real)
            }) else *
            else {
                panic!("unknown channel layout: {}", layout)
            }
        ));

        check!(
            CUBEB_LAYOUT_UNDEFINED => Undefined,
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
            CUBEB_LAYOUT_3F4_LFE => Layout3F4Lfe
        )
    }

    /// Initialize a stream associated with the supplied application context.
    pub fn stream_init<'ctx, CB>(&'ctx self, opts: &StreamInitOptions, cb: CB) -> Result<Stream<'ctx, CB>>
    where
        CB: StreamCallback,
    {
        stream_init(self, opts, cb)
    }

    pub fn enumerate_devices(&self, devtype: DeviceType, collection: &mut DeviceCollection) -> Result<()> {
        unsafe {
            try_call!(raw::cubeb_enumerate_devices(
                self.raw,
                devtype.bits(),
                collection.raw()
            ));
        }
        Ok(())
    }

    pub fn cubeb_device_collection_destroy(&self, collection: &mut DeviceCollection) -> Result<()> {
        unsafe {
            try_call!(raw::cubeb_device_collection_destroy(
                self.raw,
                collection.raw()
            ));
        }
        Ok(())
    }

    /*
    pub fn register_device_collection_changed(
        &self,
        devtype: DeviceType,
        callback: &mut DeviceCollectionChangedCb,
        user_ptr: *mut c_void,
    ) -> Result<()> {
        unsafe {
            try_call!(raw::cubeb_register_device_collection_changed(self.raw, devtype, cb));
        }

        Ok(())
    }
*/
}

impl Binding for Context {
    type Raw = *mut raw::cubeb;
    unsafe fn from_raw(raw: *mut raw::cubeb) -> Self {
        Self {
            raw: raw
        }
    }
    fn raw(&self) -> Self::Raw {
        self.raw
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { raw::cubeb_destroy(self.raw) }
    }
}
