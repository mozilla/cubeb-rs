//! Stream Functions
//!
//! # Example
//! ```no run
//! extern crate cubeb
//!
//! struct SquareWave {
//!     phase_inc: f32,
//!     phase: f32,
//!     volume: f32
//! }
//!
//! impl cubeb::StreamCallbacks for SquareWave {
//! }
//! ```

use {ChannelLayout, Context, Device, DeviceId, Error, Frame, Result, SampleFormat, State, raw};
use std::{marker, ptr, str};
use std::ffi::CString;
use std::os::raw::{c_long, c_void};
use util::{Binding, IntoCString};

/// An extension trait which allows the implementation of converting
/// void* buffers from libcubeb-sys into rust slices of the appropriate
/// type.
pub trait SampleType: Send + Copy {
    /// Type of the sample
    fn format() -> SampleFormat;
    /// Map f32 in range [-1,1] to sample type
    fn from_float(f32) -> Self;
}

impl SampleType for i16 {
    fn format() -> SampleFormat {
        SampleFormat::S16NE
    }
    fn from_float(x: f32) -> i16 {
        (x * i16::max_value() as f32) as i16
    }
}

impl SampleType for f32 {
    fn format() -> SampleFormat {
        SampleFormat::Float32NE
    }
    fn from_float(x: f32) -> f32 {
        x
    }
}

pub trait StreamCallback: Send + 'static
where
    Self::Frame: Frame,
{
    type Frame;

    // This should return a Result<usize,Error>
    fn data_callback(&mut self, &[Self::Frame], &mut [Self::Frame]) -> isize;
    fn state_callback(&mut self, state: State);
}

///
#[derive(Clone, Copy)]
pub struct StreamParams {
    raw: raw::cubeb_stream_params
}

impl StreamParams {
    pub fn format(&self) -> SampleFormat {
        macro_rules! check( ($($raw:ident => $real:ident),*) => (
            $(if self.raw.format == raw::$raw {
                super::SampleFormat::$real
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
            $(if self.raw.layout == raw::$raw {
                super::ChannelLayout::$real
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
    type Raw = *const raw::cubeb_stream_params;
    unsafe fn from_raw(raw: *const raw::cubeb_stream_params) -> Self {
        Self {
            raw: *raw
        }
    }
    fn raw(&self) -> *const raw::cubeb_stream_params {
        &self.raw as *const raw::cubeb_stream_params
    }
}

///
pub struct StreamParamsBuilder {
    format: SampleFormat,
    rate: u32,
    channels: u32,
    layout: ChannelLayout,
    #[cfg(target_os = "android")]
    stream_type: StreamType
}

impl Default for StreamParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamParamsBuilder {
    #[cfg(target_os = "android")]
    pub fn new() -> Self {
        Self {
            format: SampleFormat::S16NE,
            rate: 0,
            channels: 0,
            layout: ChannelLayout::Undefined,
            stream_type: StreamType::Music
        }
    }
    #[cfg(not(target_os = "android"))]
    pub fn new() -> Self {
        Self {
            format: SampleFormat::S16NE,
            rate: 0,
            channels: 0,
            layout: ChannelLayout::Undefined
        }
    }

    pub fn format(&mut self, format: SampleFormat) -> &mut Self {
        self.format = format;
        self
    }

    pub fn rate(&mut self, rate: u32) -> &mut Self {
        self.rate = rate;
        self
    }

    pub fn channels(&mut self, channels: u32) -> &mut Self {
        self.channels = channels;
        self
    }

    pub fn layout(&mut self, layout: ChannelLayout) -> &mut Self {
        self.layout = layout;
        self
    }

    #[cfg(target_os = "android")]
    pub fn stream_type(&mut self, stream_type: StreamType) -> &mut Self {
        self.stream_type = stream_type;
        self
    }

    pub fn take(&self) -> StreamParams {
        // Convert native endian types to matching format
        let raw_sample_format = match self.format {
            SampleFormat::S16LE => raw::CUBEB_SAMPLE_S16LE,
            SampleFormat::S16BE => raw::CUBEB_SAMPLE_S16BE,
            SampleFormat::S16NE => raw::CUBEB_SAMPLE_S16NE,
            SampleFormat::Float32LE => raw::CUBEB_SAMPLE_FLOAT32LE,
            SampleFormat::Float32BE => raw::CUBEB_SAMPLE_FLOAT32BE,
            SampleFormat::Float32NE => raw::CUBEB_SAMPLE_FLOAT32NE,
        };
        unsafe {
            Binding::from_raw(&raw::cubeb_stream_params {
                format: raw_sample_format,
                rate: self.rate,
                channels: self.channels,
                layout: self.layout as raw::cubeb_channel_layout
            } as *const _)
        }
    }
}

///
pub struct Stream<'ctx, CB>
where
    CB: StreamCallback,
{
    raw: *mut raw::cubeb_stream,
    cbs: Box<CB>,
    _marker: marker::PhantomData<&'ctx Context>
}

impl<'ctx, CB> Stream<'ctx, CB>
where
    CB: StreamCallback,
{
    fn init(context: &'ctx Context, opts: &StreamInitOptions, cb: CB) -> Result<Stream<'ctx, CB>> {
        let mut stream: *mut raw::cubeb_stream = ptr::null_mut();

        let cbs = Box::new(cb);

        unsafe {
            let input_stream_params = opts.input_stream_params
                .as_ref()
                .map(|s| s.raw())
                .unwrap_or(ptr::null());

            let output_stream_params = opts.output_stream_params
                .as_ref()
                .map(|s| s.raw())
                .unwrap_or(ptr::null());

            let user_ptr: *mut c_void = &*cbs as *const _ as *mut _;

            try_call!(raw::cubeb_stream_init(
                context.raw(),
                &mut stream,
                opts.stream_name,
                opts.input_device.raw(),
                input_stream_params,
                opts.output_device.raw(),
                output_stream_params,
                opts.latency_frames,
                data_cb_c::<CB>,
                state_cb_c::<CB>,
                user_ptr
            ));
        }

        Ok(Stream {
            raw: stream,
            cbs: cbs,
            _marker: marker::PhantomData
        })
    }

    // start playback.
    pub fn start(&self) -> Result<()> {
        unsafe {
            try_call!(raw::cubeb_stream_start(self.raw));
        }
        Ok(())
    }

    // Stop playback.
    pub fn stop(&self) -> Result<()> {
        unsafe {
            try_call!(raw::cubeb_stream_stop(self.raw));
        }
        Ok(())
    }

    // Get the current stream playback position.
    pub fn position(&self) -> Result<u64> {
        let mut position: u64 = 0;
        unsafe {
            try_call!(raw::cubeb_stream_get_position(self.raw, &mut position));
        }
        Ok(position)
    }

    pub fn latency(&self) -> Result<u32> {
        let mut latency: u32 = 0;
        unsafe {
            try_call!(raw::cubeb_stream_get_latency(self.raw, &mut latency));
        }
        Ok(latency)
    }

    pub fn set_volume(&self, volume: f32) -> Result<()> {
        unsafe {
            try_call!(raw::cubeb_stream_set_volume(self.raw, volume));
        }
        Ok(())
    }

    pub fn set_panning(&self, panning: f32) -> Result<()> {
        unsafe {
            try_call!(raw::cubeb_stream_set_panning(self.raw, panning));
        }
        Ok(())
    }

    pub fn current_device(&self) -> Result<Device> {
        let mut device_ptr: *const raw::cubeb_device = ptr::null();
        unsafe {
            try_call!(raw::cubeb_stream_get_current_device(
                self.raw,
                &mut device_ptr
            ));
            Binding::from_raw_opt(device_ptr).ok_or(Error::from_raw(raw::CUBEB_ERROR))
        }
    }

    pub fn destroy_device(&self, device: Device) -> Result<()> {
        unsafe {
            try_call!(raw::cubeb_stream_device_destroy(self.raw, device.raw()));
        }
        Ok(())
    }

    /*
    pub fn register_device_changed_callback(&self, device_changed_cb: &mut DeviceChangedCb) -> Result<(), Error> {
        unsafe { try_call!(raw::cubeb_stream_register_device_changed_callback(self.raw, ...)); }
        Ok(())
    }
*/
}

impl<'ctx, CB> Drop for Stream<'ctx, CB>
where
    CB: StreamCallback,
{
    fn drop(&mut self) {
        unsafe {
            raw::cubeb_stream_destroy(self.raw);
        }
    }
}

// C callable callbacks
extern "C" fn data_cb_c<CB: StreamCallback>(
    _: *mut raw::cubeb_stream,
    user_ptr: *mut c_void,
    input_buffer: *const c_void,
    output_buffer: *mut c_void,
    nframes: c_long,
) -> c_long {
    use std::slice::{from_raw_parts, from_raw_parts_mut};

    unsafe {
        let cbs = &mut *(user_ptr as *mut CB);
        let input: &[CB::Frame] = if input_buffer.is_null() {
            &[]
        } else {
            from_raw_parts(input_buffer as *const _, nframes as usize)
        };
        let mut output: &mut [CB::Frame] = if output_buffer.is_null() {
            &mut []
        } else {
            from_raw_parts_mut(output_buffer as *mut _, nframes as usize)
        };
        cbs.data_callback(input, output) as c_long
    }
}

extern "C" fn state_cb_c<CB: StreamCallback>(
    _: *mut raw::cubeb_stream,
    user_ptr: *mut c_void,
    state: raw::cubeb_state,
) {
    let state = match state {
        raw::CUBEB_STATE_STARTED => State::Started,
        raw::CUBEB_STATE_STOPPED => State::Stopped,
        raw::CUBEB_STATE_DRAINED => State::Drained,
        raw::CUBEB_STATE_ERROR => State::Error,
        n => panic!("unknown state: {}", n),
    };
    unsafe {
        let cbs = &mut *(user_ptr as *mut CB);
        cbs.state_callback(state);
    };
}

#[doc(hidden)]
pub fn stream_init<'ctx, CB>(context: &'ctx Context, opts: &StreamInitOptions, cb: CB) -> Result<Stream<'ctx, CB>>
where
    CB: StreamCallback,
{
    Stream::init(context, opts, cb)
}

pub struct StreamInitOptions {
    pub stream_name: Option<CString>,
    pub input_device: DeviceId,
    pub input_stream_params: Option<StreamParams>,
    pub output_device: DeviceId,
    pub output_stream_params: Option<StreamParams>,
    pub latency_frames: u32
}

impl StreamInitOptions {
    pub fn new() -> Self {
        StreamInitOptions {
            stream_name: None,
            input_device: DeviceId::default(),
            input_stream_params: None,
            output_device: DeviceId::default(),
            output_stream_params: None,
            latency_frames: 0
        }
    }
}

impl Default for StreamInitOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Structure describing options about how stream should be initialized.
pub struct StreamInitOptionsBuilder {
    opts: StreamInitOptions
}

impl Default for StreamInitOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamInitOptionsBuilder {
    pub fn new() -> Self {
        StreamInitOptionsBuilder {
            opts: Default::default()
        }
    }

    pub fn stream_name<S>(&mut self, name: S) -> &mut Self
    where
        S: IntoCString,
    {
        self.opts.stream_name = Some(name.into_c_string().unwrap());
        self
    }

    pub fn input_device(&mut self, id: DeviceId) -> &mut Self {
        self.opts.input_device = id;
        self
    }

    pub fn input_stream_param(&mut self, param: &StreamParams) -> &mut Self {
        self.opts.input_stream_params = Some(*param);
        self
    }

    pub fn output_device(&mut self, id: DeviceId) -> &mut Self {
        self.opts.output_device = id;
        self
    }

    pub fn output_stream_param(&mut self, param: &StreamParams) -> &mut Self {
        self.opts.output_stream_params = Some(*param);
        self
    }

    pub fn latency(&mut self, latency: u32) -> &mut Self {
        self.opts.latency_frames = latency;
        self
    }

    pub fn take(&mut self) -> StreamInitOptions {
        use std::mem::replace;
        replace(&mut self.opts, Default::default())
    }
}

#[cfg(test)]
mod tests {

    use StreamParamsBuilder;
    use raw;
    use std::mem;
    use util::Binding;

    #[test]
    fn stream_params_raw_channels() {
        let mut raw: raw::cubeb_stream_params = unsafe { mem::zeroed() };
        raw.channels = 2;
        let params = unsafe { super::StreamParams::from_raw(&raw as *const _) };
        assert_eq!(params.channels(), 2);
    }

    #[test]
    fn stream_params_raw_format() {
        let mut raw: raw::cubeb_stream_params = unsafe { mem::zeroed() };
        macro_rules! check(
            ($($raw:ident => $real:ident),*) => (
                $(raw.format = raw::$raw;
                  let params = unsafe { super::StreamParams::from_raw(&raw as *const _) };
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
        let mut raw: raw::cubeb_stream_params = unsafe { mem::zeroed() };
        raw.format = raw::CUBEB_SAMPLE_S16NE;
        let params = unsafe { super::StreamParams::from_raw(&raw as *const _) };
        assert_eq!(
            params.format(),
            if cfg!(target_endian = "little") {
                super::SampleFormat::S16LE
            } else {
                super::SampleFormat::S16BE
            }
        );

        raw.format = raw::CUBEB_SAMPLE_FLOAT32NE;
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
        let mut raw: raw::cubeb_stream_params = unsafe { mem::zeroed() };
        macro_rules! check(
            ($($raw:ident => $real:ident),*) => (
                $(raw.layout = raw::$raw;
                  let params = unsafe { super::StreamParams::from_raw(&raw as *const _) };
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
        let mut raw: raw::cubeb_stream_params = unsafe { mem::zeroed() };
        raw.rate = 44100;
        let params = unsafe { super::StreamParams::from_raw(&raw as *const _) };
        assert_eq!(params.rate(), 44100);
    }

    #[test]
    fn stream_params_builder_channels() {
        let params = StreamParamsBuilder::new().channels(2).take();
        assert_eq!(params.channels(), 2);
    }

    #[test]
    fn stream_params_builder_format() {
        macro_rules! check(
            ($($real:ident),*) => (
                $(let params = StreamParamsBuilder::new()
                  .format(super::SampleFormat::$real)
                  .take();
                assert_eq!(params.format(), super::SampleFormat::$real);
                )*
            ) );

        check!(S16LE, S16BE, Float32LE, Float32BE);
    }

    #[test]
    fn stream_params_builder_format_native_endian() {
        let params = StreamParamsBuilder::new()
            .format(super::SampleFormat::S16NE)
            .take();
        assert_eq!(
            params.format(),
            if cfg!(target_endian = "little") {
                super::SampleFormat::S16LE
            } else {
                super::SampleFormat::S16BE
            }
        );

        let params = StreamParamsBuilder::new()
            .format(super::SampleFormat::Float32NE)
            .take();
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
    fn stream_params_builder_layout() {
        macro_rules! check(
            ($($real:ident),*) => (
                $(let params = StreamParamsBuilder::new()
                  .layout(super::ChannelLayout::$real)
                  .take();
                assert_eq!(params.layout(), super::ChannelLayout::$real);
                )*
            ) );

        check!(
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
            Layout3F3RLfe,
            Layout3F4Lfe
        );
    }

    #[test]
    fn stream_params_builder_rate() {
        let params = StreamParamsBuilder::new().rate(44100).take();
        assert_eq!(params.rate(), 44100);
    }

    #[test]
    fn stream_params_builder_to_raw_channels() {
        let params = StreamParamsBuilder::new().channels(2).take();
        let raw = unsafe { &*params.raw() };
        assert_eq!(raw.channels, 2);
    }

    #[test]
    fn stream_params_builder_to_raw_format() {
        macro_rules! check(
            ($($real:ident => $raw:ident),*) => (
                $(let params = super::StreamParamsBuilder::new()
                  .format(super::SampleFormat::$real)
                  .take();
                  let raw = unsafe { &*params.raw() };
                  assert_eq!(raw.format, raw::$raw);
                )*
            ) );

        check!(S16LE => CUBEB_SAMPLE_S16LE,
               S16BE => CUBEB_SAMPLE_S16BE,
               Float32LE => CUBEB_SAMPLE_FLOAT32LE,
               Float32BE => CUBEB_SAMPLE_FLOAT32BE);
    }

    #[test]
    fn stream_params_builder_format_to_raw_native_endian() {
        let params = StreamParamsBuilder::new()
            .format(super::SampleFormat::S16NE)
            .take();
        let raw = unsafe { &*params.raw() };
        assert_eq!(
            raw.format,
            if cfg!(target_endian = "little") {
                raw::CUBEB_SAMPLE_S16LE
            } else {
                raw::CUBEB_SAMPLE_S16BE
            }
        );

        let params = StreamParamsBuilder::new()
            .format(super::SampleFormat::Float32NE)
            .take();
        let raw = unsafe { &*params.raw() };
        assert_eq!(
            raw.format,
            if cfg!(target_endian = "little") {
                raw::CUBEB_SAMPLE_FLOAT32LE
            } else {
                raw::CUBEB_SAMPLE_FLOAT32BE
            }
        );
    }

    #[test]
    fn stream_params_builder_to_raw_layout() {
        macro_rules! check(
            ($($real:ident => $raw:ident),*) => (
                $(let params = super::StreamParamsBuilder::new()
                  .layout(super::ChannelLayout::$real)
                  .take();
                  let raw = unsafe { &*params.raw() };
                  assert_eq!(raw.layout, raw::$raw);
                )*
            ) );

        check!(Undefined => CUBEB_LAYOUT_UNDEFINED,
               DualMono => CUBEB_LAYOUT_DUAL_MONO,
               DualMonoLfe => CUBEB_LAYOUT_DUAL_MONO_LFE,
               Mono => CUBEB_LAYOUT_MONO,
               MonoLfe => CUBEB_LAYOUT_MONO_LFE,
               Stereo => CUBEB_LAYOUT_STEREO,
               StereoLfe => CUBEB_LAYOUT_STEREO_LFE,
               Layout3F => CUBEB_LAYOUT_3F,
               Layout3FLfe => CUBEB_LAYOUT_3F_LFE,
               Layout2F1 => CUBEB_LAYOUT_2F1,
               Layout2F1Lfe => CUBEB_LAYOUT_2F1_LFE,
               Layout3F1 => CUBEB_LAYOUT_3F1,
               Layout3F1Lfe => CUBEB_LAYOUT_3F1_LFE,
               Layout2F2 => CUBEB_LAYOUT_2F2,
               Layout2F2Lfe => CUBEB_LAYOUT_2F2_LFE,
               Layout3F2 => CUBEB_LAYOUT_3F2,
               Layout3F2Lfe => CUBEB_LAYOUT_3F2_LFE,
               Layout3F3RLfe => CUBEB_LAYOUT_3F3R_LFE,
               Layout3F4Lfe => CUBEB_LAYOUT_3F4_LFE);
    }

    #[test]
    fn stream_params_builder_to_raw_rate() {
        let params = StreamParamsBuilder::new().rate(44100).take();
        let raw = unsafe { &*params.raw() };
        assert_eq!(raw.rate, 44100);
    }
}
