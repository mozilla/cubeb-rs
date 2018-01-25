// Copyright © 2017-2018 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.

use {ChannelLayout, SampleFormat, StreamParams, StreamPrefs};
use ffi;

///
#[derive(Debug)]
pub struct StreamParamsBuilder(ffi::cubeb_stream_params);

impl Default for StreamParamsBuilder {
    fn default() -> Self {
        let mut r = ffi::cubeb_stream_params::default();
        r.format = ffi::CUBEB_SAMPLE_S16NE;
        StreamParamsBuilder(r)
    }
}

impl StreamParamsBuilder {
    pub fn new() -> Self { Default::default() }

    pub fn format(mut self, format: SampleFormat) -> Self {
        self.0.format = format.into();
        self
    }

    pub fn rate(mut self, rate: u32) -> Self {
        self.0.rate = rate;
        self
    }

    pub fn channels(mut self, channels: u32) -> Self {
        self.0.channels = channels;
        self
    }

    pub fn layout(mut self, layout: ChannelLayout) -> Self {
        self.0.layout = layout.into();
        self
    }

    pub fn prefs(mut self, prefs: StreamPrefs) -> Self {
        self.0.prefs = prefs.bits();
        self
    }

    pub fn take(&self) -> StreamParams { StreamParams::from(self.0) }
}

#[cfg(test)]
mod tests {
    use {ffi, StreamParamsBuilder, StreamPrefs};
    use SampleFormat;
    use foreign_types::ForeignType;

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
            .format(SampleFormat::S16NE)
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
            .format(SampleFormat::Float32NE)
            .take();
        assert_eq!(
            params.format(),
            if cfg!(target_endian = "little") {
                SampleFormat::Float32LE
            } else {
                SampleFormat::Float32BE
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
        let raw = unsafe { &*params.as_ptr() };
        assert_eq!(raw.channels, 2);
    }

    #[test]
    fn stream_params_builder_to_raw_format() {
        macro_rules! check(
            ($($real:ident => $raw:ident),*) => (
                $(let params = super::StreamParamsBuilder::new()
                  .format(SampleFormat::$real)
                  .take();
                  let raw = unsafe { &*params.as_ptr() };
                  assert_eq!(raw.format, ffi::$raw);
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
            .format(SampleFormat::S16NE)
            .take();
        let raw = unsafe { &*params.as_ptr() };
        assert_eq!(
            raw.format,
            if cfg!(target_endian = "little") {
                ffi::CUBEB_SAMPLE_S16LE
            } else {
                ffi::CUBEB_SAMPLE_S16BE
            }
        );

        let params = StreamParamsBuilder::new()
            .format(SampleFormat::Float32NE)
            .take();
        let raw = unsafe { &*params.as_ptr() };
        assert_eq!(
            raw.format,
            if cfg!(target_endian = "little") {
                ffi::CUBEB_SAMPLE_FLOAT32LE
            } else {
                ffi::CUBEB_SAMPLE_FLOAT32BE
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
                  let raw = unsafe { &*params.as_ptr() };
                  assert_eq!(raw.layout, ffi::$raw);
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
        let raw = unsafe { &*params.as_ptr() };
        assert_eq!(raw.rate, 44100);
    }

    #[test]
    fn stream_params_builder_prefs_default() {
        let params = StreamParamsBuilder::new().take();
        assert_eq!(params.prefs(), StreamPrefs::NONE);
    }

    #[test]
    fn stream_params_builder_prefs() {
        let params = StreamParamsBuilder::new()
            .prefs(StreamPrefs::LOOPBACK)
            .take();
        assert_eq!(params.prefs(), StreamPrefs::LOOPBACK);
    }
}
