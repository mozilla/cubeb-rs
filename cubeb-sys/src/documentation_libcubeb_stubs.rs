use std::os::raw::{c_char, c_float, c_int, c_uint, c_void};

// Documentation-only stubs for libcubeb functions.

#[no_mangle]
pub fn cubeb_stream_destroy(stream: *mut cubeb_stream) {}
#[no_mangle]
pub fn cubeb_stream_start(stream: *mut cubeb_stream) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_stream_stop(stream: *mut cubeb_stream) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_stream_get_position(stream: *mut cubeb_stream, position: *mut u64) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_stream_get_latency(stream: *mut cubeb_stream, latency: *mut c_uint) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_stream_get_input_latency(stream: *mut cubeb_stream, latency: *mut c_uint) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_stream_set_volume(stream: *mut cubeb_stream, volume: c_float) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_stream_set_name(stream: *mut cubeb_stream, name: *const c_char) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_stream_get_current_device(
    stream: *mut cubeb_stream,
    device: *mut *mut cubeb_device,
) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_stream_set_input_mute(stream: *mut cubeb_stream, mute: c_int) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_stream_set_input_processing_params(
    stream: *mut cubeb_stream,
    params: cubeb_input_processing_params,
) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_stream_device_destroy(stream: *mut cubeb_stream, devices: *mut cubeb_device) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_stream_register_device_changed_callback(
    stream: *mut cubeb_stream,
    device_changed_callback: cubeb_device_changed_callback,
) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_stream_user_ptr(stream: *mut cubeb_stream) -> *mut c_void {
    0
}

#[no_mangle]
pub fn cubeb_audio_dump_init(session: *mut cubeb_audio_dump_session_t) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_audio_dump_shutdown(session: cubeb_audio_dump_session_t) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_audio_dump_stream_init(
    session: cubeb_audio_dump_session_t,
    stream: *mut cubeb_audio_dump_stream_t,
    stream_params: cubeb_stream_params,
    name: *const c_char,
) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_audio_dump_stream_shutdown(
    session: cubeb_audio_dump_session_t,
    stream: cubeb_audio_dump_stream_t,
) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_audio_dump_start(session: cubeb_audio_dump_session_t) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_audio_dump_stop(session: cubeb_audio_dump_session_t) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_audio_dump_write(
    stream: cubeb_audio_dump_stream_t,
    audio_samples: *mut c_void,
    count: u32,
) -> c_int {
    0
}

#[no_mangle]
pub fn cubeb_set_log_callback(
    log_level: cubeb_log_level,
    log_callback: cubeb_log_callback,
) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_log_get_callback() -> cubeb_log_callback {
    None
}
#[no_mangle]
pub fn cubeb_log_get_level() -> cubeb_log_level {
    cubeb_log_level::CUBEB_LOG_DISABLED
}
#[no_mangle]
pub fn cubeb_async_log_reset_threads(_: c_void) {}
#[no_mangle]
pub fn cubeb_async_log(msg: *const c_char, ...) {}

#[no_mangle]
pub fn cubeb_resampler_create(
    stream: *mut cubeb_stream,
    input_params: *mut cubeb_stream_params,
    output_params: *mut cubeb_stream_params,
    target_rate: c_uint,
    callback: cubeb_data_callback,
    user_ptr: *mut c_void,
    quality: cubeb_resampler_quality,
    reclock: cubeb_resampler_reclock,
) -> *mut cubeb_resampler {
    std::ptr::null
}

#[no_mangle]
pub fn cubeb_resampler_fill(
    resampler: *mut cubeb_resampler,
    input_buffer: *mut c_void,
    input_frame_count: *mut c_long,
    output_buffer: *mut c_void,
    output_frames_needed: c_long,
) -> c_long {
    0
}

#[no_mangle]
pub fn cubeb_resampler_destroy(resampler: *mut cubeb_resampler) {}
#[no_mangle]
pub fn cubeb_resampler_latency(resampler: *mut cubeb_resampler) -> c_long {
    0
}

#[no_mangle]
pub fn cubeb_mixer_create(
    format: cubeb_sample_format,
    in_channels: u32,
    in_layout: cubeb_channel_layout,
    out_channels: u32,
    out_layout: cubeb_channel_layout,
) -> *mut cubeb_mixer {
    std::ptr::null
}
#[no_mangle]
pub fn cubeb_mixer_destroy(mixer: *mut cubeb_mixer) {}
#[no_mangle]
pub fn cubeb_mixer_mix(
    mixer: *mut cubeb_mixer,
    frames: usize,
    input_buffer: *const c_void,
    input_buffer_length: usize,
    output_buffer: *mut c_void,
    output_buffer_length: usize,
) -> c_int {
    0
}

#[no_mangle]
pub fn cubeb_channel_layout_nb_channels(channel_layout: cubeb_channel_layout) -> c_uint {
    0
}

#[no_mangle]
pub fn cubeb_init(
    context: *mut *mut cubeb,
    context_name: *const c_char,
    backend_name: *const c_char,
) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_get_backend_id(context: *mut cubeb) -> *const c_char {
    std::ptr::null
}
#[no_mangle]
pub fn cubeb_get_max_channel_count(context: *mut cubeb, max_channels: *mut c_uint) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_get_min_latency(
    context: *mut cubeb,
    params: *mut cubeb_stream_params,
    latency_frames: *mut c_uint,
) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_get_preferred_sample_rate(context: *mut cubeb, rate: *mut c_uint) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_get_supported_input_processing_params(
    context: *mut cubeb,
    params: *mut cubeb_input_processing_params,
) -> c_int {
    0
}
#[no_mangle]
pub fn cubeb_destroy(context: *mut cubeb) {}
#[no_mangle]
pub fn cubeb_stream_init(
    context: *mut cubeb,
    stream: *mut *mut cubeb_stream,
    stream_name: *const c_char,
    input_device: cubeb_devid,
    input_stream_params: *mut cubeb_stream_params,
    output_device: cubeb_devid,
    output_stream_params: *mut cubeb_stream_params,
    latency_frames: c_uint,
    data_callback: cubeb_data_callback,
    state_callback: cubeb_state_callback,
    user_ptr: *mut c_void,
) -> c_int {
    0
}
