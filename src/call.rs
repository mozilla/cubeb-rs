use Error;
use std::os::raw::c_int;

macro_rules! call {
    (raw::$p:ident ($($e:expr),*)) => (
        raw::$p($(::call::convert(&$e)),*)
    )
}

macro_rules! try_call {
    (raw::$p:ident ($($e:expr),*)) => ({
        match ::call::try(raw::$p($(::call::convert(&$e)),*)) {
            Ok(o) => o,
            Err(e) => { return Err(e) }
        }
    })
}

pub fn try(ret: c_int) -> Result<c_int, Error> {
    match ret {
        n if n < 0 => Err(unsafe { Error::from_raw(n) }),
        n => Ok(n),
    }
}

#[doc(hidden)]
pub trait Convert<T> {
    fn convert(&self) -> T;
}

pub fn convert<T, U>(u: &U) -> T
where
    U: Convert<T>,
{
    u.convert()
}

mod impls {
    use {ChannelLayout, LogLevel, SampleFormat, State, raw};
    #[cfg(target_os = "android")]
    use StreamType;
    use call::Convert;
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::ptr;

    impl<T: Copy> Convert<T> for T {
        fn convert(&self) -> T {
            *self
        }
    }

    impl<'a, T> Convert<*const T> for &'a T {
        fn convert(&self) -> *const T {
            &**self as *const _
        }
    }

    impl<'a, T> Convert<*mut T> for &'a mut T {
        fn convert(&self) -> *mut T {
            &**self as *const _ as *mut _
        }
    }

    impl<T> Convert<*const T> for *mut T {
        fn convert(&self) -> *const T {
            *self as *const T
        }
    }

    impl Convert<*const c_char> for CString {
        fn convert(&self) -> *const c_char {
            self.as_ptr()
        }
    }

    impl<T, U: Convert<*const T>> Convert<*const T> for Option<U> {
        fn convert(&self) -> *const T {
            self.as_ref().map(|s| s.convert()).unwrap_or(ptr::null())
        }
    }

    impl Convert<raw::cubeb_sample_format> for SampleFormat {
        #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
        fn convert(&self) -> raw::cubeb_sample_format {
            match *self {
                SampleFormat::S16LE => raw::CUBEB_SAMPLE_S16LE,
                SampleFormat::S16BE => raw::CUBEB_SAMPLE_S16BE,
                SampleFormat::S16NE => raw::CUBEB_SAMPLE_S16NE,
                SampleFormat::Float32LE => raw::CUBEB_SAMPLE_FLOAT32LE,
                SampleFormat::Float32BE => raw::CUBEB_SAMPLE_FLOAT32BE,
                SampleFormat::Float32NE => raw::CUBEB_SAMPLE_FLOAT32NE,
            }
        }
    }

    #[cfg(target_os = "android")]
    impl Convert<raw::cubeb_stream_type> for StreamType {
        fn convert(&self) -> raw::cubeb_stream_type {
            match *self {
                StreamType::VoiceCall => raw::CUBEB_STREAM_TYPE_VOICE_CALL,
                StreamType::System => raw::CUBEB_STREAM_TYPE_SYSTEM,
                StreamType::Ring => raw::CUBEB_STREAM_TYPE_RING,
                StreamType::Music => raw::CUBEB_STREAM_TYPE_MUSIC,
                StreamType::Alarm => raw::CUBEB_STREAM_TYPE_ALARM,
                StreamType::Notification => raw::CUBEB_STREAM_TYPE_NOTIFICATION,
                StreamType::BluetoothSco => raw::CUBEB_STREAM_TYPE_BLUETOOTH_SCO,
                StreamType::SystemEnforced => raw::CUBEB_STREAM_TYPE_SYSTEM_ENFORCED,
                StreamType::Dtmf => raw::CUBEB_STREAM_TYPE_DTMF,
                StreamType::Tts => raw::CUBEB_STREAM_TYPE_TTS,
                StreamType::Fm => raw::CUBEB_STREAM_TYPE_FM,
            }
        }
    }

    impl Convert<raw::cubeb_log_level> for LogLevel {
        fn convert(&self) -> raw::cubeb_log_level {
            match *self {
                LogLevel::Disabled => raw::CUBEB_LOG_DISABLED,
                LogLevel::Normal => raw::CUBEB_LOG_NORMAL,
                LogLevel::Verbose => raw::CUBEB_LOG_VERBOSE,
            }
        }
    }


    impl Convert<raw::cubeb_channel_layout> for ChannelLayout {
        fn convert(&self) -> raw::cubeb_channel_layout {
            match *self {
                ChannelLayout::Undefined => raw::CUBEB_LAYOUT_UNDEFINED,
                ChannelLayout::DualMono => raw::CUBEB_LAYOUT_DUAL_MONO,
                ChannelLayout::DualMonoLfe => raw::CUBEB_LAYOUT_DUAL_MONO_LFE,
                ChannelLayout::Mono => raw::CUBEB_LAYOUT_MONO,
                ChannelLayout::MonoLfe => raw::CUBEB_LAYOUT_MONO_LFE,
                ChannelLayout::Stereo => raw::CUBEB_LAYOUT_STEREO,
                ChannelLayout::StereoLfe => raw::CUBEB_LAYOUT_STEREO_LFE,
                ChannelLayout::Layout3F => raw::CUBEB_LAYOUT_3F,
                ChannelLayout::Layout3FLfe => raw::CUBEB_LAYOUT_3F_LFE,
                ChannelLayout::Layout2F1 => raw::CUBEB_LAYOUT_2F1,
                ChannelLayout::Layout2F1Lfe => raw::CUBEB_LAYOUT_2F1_LFE,
                ChannelLayout::Layout3F1 => raw::CUBEB_LAYOUT_3F1,
                ChannelLayout::Layout3F1Lfe => raw::CUBEB_LAYOUT_3F1_LFE,
                ChannelLayout::Layout2F2 => raw::CUBEB_LAYOUT_2F2,
                ChannelLayout::Layout2F2Lfe => raw::CUBEB_LAYOUT_2F2_LFE,
                ChannelLayout::Layout3F2 => raw::CUBEB_LAYOUT_3F2,
                ChannelLayout::Layout3F2Lfe => raw::CUBEB_LAYOUT_3F2_LFE,
                ChannelLayout::Layout3F3RLfe => raw::CUBEB_LAYOUT_3F3R_LFE,
                ChannelLayout::Layout3F4Lfe => raw::CUBEB_LAYOUT_3F4_LFE,
            }
        }
    }

    impl Convert<raw::cubeb_state> for State {
        fn convert(&self) -> raw::cubeb_state {
            {
                match *self {
                    State::Started => raw::CUBEB_STATE_STARTED,
                    State::Stopped => raw::CUBEB_STATE_STOPPED,
                    State::Drained => raw::CUBEB_STATE_DRAINED,
                    State::Error => raw::CUBEB_STATE_ERROR,
                }
            }
        }
    }
}
