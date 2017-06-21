#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_float, c_int, c_long, c_uint, c_void};

macro_rules! cubeb_enum {
    (pub enum $name:ident { $($variants:tt)* }) => {
        #[cfg(target_env = "msvc")]
        pub type $name = i32;
        #[cfg(not(target_env = "msvc"))]
        pub type $name = u32;
        cubeb_enum!(gen, $name, 0, $($variants)*);
    };
    (pub enum $name:ident: $t:ty { $($variants:tt)* }) => {
        pub type $name = $t;
        cubeb_enum!(gen, $name, 0, $($variants)*);
    };
    (gen, $name:ident, $val:expr, $variant:ident, $($rest:tt)*) => {
        pub const $variant: $name = $val;
        cubeb_enum!(gen, $name, $val+1, $($rest)*);
    };
    (gen, $name:ident, $val:expr, $variant:ident = $e:expr, $($rest:tt)*) => {
        pub const $variant: $name = $e;
        cubeb_enum!(gen, $name, $e+1, $($rest)*);
    };
    (gen, $name:ident, $val:expr, ) => {}
}

pub enum cubeb {}
pub enum cubeb_stream {}

cubeb_enum! {
    pub enum cubeb_sample_format {
        CUBEB_SAMPLE_S16LE,
        CUBEB_SAMPLE_S16BE,
        CUBEB_SAMPLE_FLOAT32LE,
        CUBEB_SAMPLE_FLOAT32BE,
    }
}

#[cfg(target_endian = "big")]
pub const CUBEB_SAMPLE_S16NE: cubeb_sample_format = CUBEB_SAMPLE_S16BE;
#[cfg(target_endian = "big")]
pub const CUBEB_SAMPLE_FLOAT32NE: cubeb_sample_format = CUBEB_SAMPLE_FLOAT32BE;
#[cfg(target_endian = "little")]
pub const CUBEB_SAMPLE_S16NE: cubeb_sample_format = CUBEB_SAMPLE_S16LE;
#[cfg(target_endian = "little")]
pub const CUBEB_SAMPLE_FLOAT32NE: cubeb_sample_format = CUBEB_SAMPLE_FLOAT32LE;

#[cfg(target_os = "android")]
cubeb_enum! {
    pub enum cubeb_stream_type: c_int {
        CUBEB_STREAM_TYPE_VOICE_CALL = 0,
        CUBEB_STREAM_TYPE_SYSTEM = 1,
        CUBEB_STREAM_TYPE_RING = 2,
        CUBEB_STREAM_TYPE_MUSIC = 3,
        CUBEB_STREAM_TYPE_ALARM = 4,
        CUBEB_STREAM_TYPE_NOTIFICATION = 5,
        CUBEB_STREAM_TYPE_BLUETOOTH_SCO = 6,
        CUBEB_STREAM_TYPE_SYSTEM_ENFORCED = 7,
        CUBEB_STREAM_TYPE_DTMF = 8,
        CUBEB_STREAM_TYPE_TTS = 9,
        CUBEB_STREAM_TYPE_FM = 10,

        CUBEB_STREAM_TYPE_MAX
    }
}

pub type cubeb_devid = *const c_void;

cubeb_enum! {
    pub enum cubeb_log_level: c_int {
        CUBEB_LOG_DISABLED = 0,
        CUBEB_LOG_NORMAL = 1,
        CUBEB_LOG_VERBOSE = 2,
    }
}


cubeb_enum! {
    pub enum cubeb_channel_layout: c_int {
        CUBEB_LAYOUT_UNDEFINED,
        CUBEB_LAYOUT_DUAL_MONO,
        CUBEB_LAYOUT_DUAL_MONO_LFE,
        CUBEB_LAYOUT_MONO,
        CUBEB_LAYOUT_MONO_LFE,
        CUBEB_LAYOUT_STEREO,
        CUBEB_LAYOUT_STEREO_LFE,
        CUBEB_LAYOUT_3F,
        CUBEB_LAYOUT_3F_LFE,
        CUBEB_LAYOUT_2F1,
        CUBEB_LAYOUT_2F1_LFE,
        CUBEB_LAYOUT_3F1,
        CUBEB_LAYOUT_3F1_LFE,
        CUBEB_LAYOUT_2F2,
        CUBEB_LAYOUT_2F2_LFE,
        CUBEB_LAYOUT_3F2,
        CUBEB_LAYOUT_3F2_LFE,
        CUBEB_LAYOUT_3F3R_LFE,
        CUBEB_LAYOUT_3F4_LFE,
        CUBEB_LAYOUT_MAX,
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct cubeb_stream_params {
    pub format: cubeb_sample_format,
    pub rate: c_uint,
    pub channels: c_uint,
    pub layout: cubeb_channel_layout,
    #[cfg(target_os = "android")]
    pub stream_type: cubeb_stream_type
}

#[repr(C)]
pub struct cubeb_device {
    pub output_name: *const c_char,
    pub input_name: *const c_char
}

cubeb_enum! {
    pub enum cubeb_state: c_int {
        CUBEB_STATE_STARTED,
        CUBEB_STATE_STOPPED,
        CUBEB_STATE_DRAINED,
        CUBEB_STATE_ERROR,
    }
}

cubeb_enum! {
    pub enum cubeb_error_code: c_int {
        CUBEB_OK = 0,
        CUBEB_ERROR = -1,
        CUBEB_ERROR_INVALID_FORMAT = -2,
        CUBEB_ERROR_INVALID_PARAMETER = -3,
        CUBEB_ERROR_NOT_SUPPORTED = -4,
        CUBEB_ERROR_DEVICE_UNAVAILABLE = -5,
    }
}

cubeb_enum! {
    pub enum cubeb_device_type {
        CUBEB_DEVICE_TYPE_UNKNOWN,
        CUBEB_DEVICE_TYPE_INPUT,
        CUBEB_DEVICE_TYPE_OUTPUT,
    }
}

cubeb_enum! {
    pub enum cubeb_device_state {
        CUBEB_DEVICE_STATE_DISABLED,
        CUBEB_DEVICE_STATE_UNPLUGGED,
        CUBEB_DEVICE_STATE_ENABLED,
    }
}

cubeb_enum! {
    pub enum cubeb_device_fmt {
        CUBEB_DEVICE_FMT_S16LE          = 0x0010,
        CUBEB_DEVICE_FMT_S16BE          = 0x0020,
        CUBEB_DEVICE_FMT_F32LE          = 0x1000,
        CUBEB_DEVICE_FMT_F32BE          = 0x2000,
    }
}

#[cfg(target_endian = "big")]
pub const CUBEB_DEVICE_FMT_S16NE: cubeb_device_fmt = CUBEB_DEVICE_FMT_S16BE;
#[cfg(target_endian = "big")]
pub const CUBEB_DEVICE_FMT_F32NE: cubeb_device_fmt = CUBEB_DEVICE_FMT_F32BE;
#[cfg(target_endian = "little")]
pub const CUBEB_DEVICE_FMT_S16NE: cubeb_device_fmt = CUBEB_DEVICE_FMT_S16LE;
#[cfg(target_endian = "little")]
pub const CUBEB_DEVICE_FMT_F32NE: cubeb_device_fmt = CUBEB_DEVICE_FMT_F32LE;

pub const CUBEB_DEVICE_FMT_S16_MASK: cubeb_device_fmt = (CUBEB_DEVICE_FMT_S16LE | CUBEB_DEVICE_FMT_S16BE);
pub const CUBEB_DEVICE_FMT_F32_MASK: cubeb_device_fmt = (CUBEB_DEVICE_FMT_F32LE | CUBEB_DEVICE_FMT_F32BE);
pub const CUBEB_DEVICE_FMT_ALL: cubeb_device_fmt = (CUBEB_DEVICE_FMT_S16_MASK | CUBEB_DEVICE_FMT_F32_MASK);

cubeb_enum! {
    pub enum cubeb_device_pref  {
        CUBEB_DEVICE_PREF_NONE          = 0x00,
        CUBEB_DEVICE_PREF_MULTIMEDIA    = 0x01,
        CUBEB_DEVICE_PREF_VOICE         = 0x02,
        CUBEB_DEVICE_PREF_NOTIFICATION  = 0x04,
        CUBEB_DEVICE_PREF_ALL           = 0x0F,
    }
}

#[repr(C)]
pub struct cubeb_device_info {
    pub devid: cubeb_devid,
    pub device_id: *const c_char,
    pub friendly_name: *const c_char,
    pub group_id: *const c_char,
    pub vendor_name: *const c_char,

    pub device_type: cubeb_device_type,
    pub state: cubeb_device_state,
    pub preferred: cubeb_device_pref,

    pub format: cubeb_device_fmt,
    pub default_format: cubeb_device_fmt,
    pub max_channels: c_uint,
    pub default_rate: c_uint,
    pub max_rate: c_uint,
    pub min_rate: c_uint,

    pub latency_lo: c_uint,
    pub latency_hi: c_uint
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct cubeb_device_collection {
    pub device: *const cubeb_device_info,
    pub count: usize
}

pub type cubeb_data_callback = extern "C" fn(*mut cubeb_stream,
                                             *mut c_void,
                                             *const c_void,
                                             *mut c_void,
                                             c_long)
                                             -> c_long;

pub type cubeb_state_callback = extern "C" fn(*mut cubeb_stream, *mut c_void, cubeb_state);
pub type cubeb_device_changed_callback = extern "C" fn(*mut c_void);
pub type cubeb_device_collection_changed_callback = extern "C" fn(*mut cubeb, *mut c_void);
pub type cubeb_log_callback = extern "C" fn(*const c_char, ...);

extern "C" {
    pub fn cubeb_init(context: *mut *mut cubeb, context_name: *const c_char, backend_name: *const c_char) -> c_int;
    pub fn cubeb_get_backend_id(context: *mut cubeb) -> *const c_char;
    pub fn cubeb_get_max_channel_count(context: *mut cubeb, max_channels: *mut c_uint) -> c_int;
    pub fn cubeb_get_min_latency(
        context: *mut cubeb,
        params: *const cubeb_stream_params,
        latency_frames: *mut c_uint,
    ) -> c_int;
    pub fn cubeb_get_preferred_sample_rate(context: *mut cubeb, rate: *mut c_uint) -> c_int;
    pub fn cubeb_get_preferred_channel_layout(context: *mut cubeb, layout: *mut cubeb_channel_layout) -> c_int;
    pub fn cubeb_destroy(context: *mut cubeb);
    pub fn cubeb_stream_init(
        context: *mut cubeb,
        stream: *mut *mut cubeb_stream,
        stream_name: *const c_char,
        input_device: cubeb_devid,
        input_stream_params: *const cubeb_stream_params,
        output_device: cubeb_devid,
        output_stream_params: *const cubeb_stream_params,
        latency_frames: c_uint,
        data_callback: cubeb_data_callback,
        state_callback: cubeb_state_callback,
        user_ptr: *mut c_void,
    ) -> c_int;
    pub fn cubeb_stream_destroy(stream: *mut cubeb_stream);
    pub fn cubeb_stream_start(stream: *mut cubeb_stream) -> c_int;
    pub fn cubeb_stream_stop(stream: *mut cubeb_stream) -> c_int;
    pub fn cubeb_stream_get_position(stream: *mut cubeb_stream, position: *mut u64) -> c_int;
    pub fn cubeb_stream_get_latency(stream: *mut cubeb_stream, latency: *mut c_uint) -> c_int;
    pub fn cubeb_stream_set_volume(stream: *mut cubeb_stream, volume: c_float) -> c_int;
    pub fn cubeb_stream_set_panning(stream: *mut cubeb_stream, panning: c_float) -> c_int;
    pub fn cubeb_stream_get_current_device(stream: *mut cubeb_stream, device: *mut *const cubeb_device) -> c_int;
    pub fn cubeb_stream_device_destroy(stream: *mut cubeb_stream, devices: *const cubeb_device) -> c_int;
    pub fn cubeb_stream_register_device_changed_callback(
        stream: *mut cubeb_stream,
        device_changed_callback: cubeb_device_changed_callback,
    ) -> c_int;
    pub fn cubeb_enumerate_devices(
        context: *mut cubeb,
        devtype: cubeb_device_type,
        collection: *mut cubeb_device_collection,
    ) -> c_int;
    pub fn cubeb_device_collection_destroy(context: *mut cubeb, collection: *mut cubeb_device_collection) -> c_int;
    pub fn cubeb_register_device_collection_changed(
        context: *mut cubeb,
        devtype: cubeb_device_type,
        callback: cubeb_device_collection_changed_callback,
        user_ptr: *mut c_void,
    ) -> c_int;
    pub fn cubeb_set_log_callback(log_level: cubeb_log_level, log_callback: cubeb_log_callback) -> c_int;
}
