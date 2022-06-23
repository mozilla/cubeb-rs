// Copyright © 2017-2018 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.


#[macro_export]
macro_rules! cubeb_log_internal {
    ($use_async: ident, $level: expr, $msg: expr) => {
        #[allow(unused_unsafe)]
        unsafe {
            if $level <= $crate::ffi::g_cubeb_log_level.into() {
                cubeb_log_internal!($use_async, __INTERNAL__ $msg);
            }
        }
    };
    ($use_async: ident, $level: expr, $fmt: expr, $($arg: expr),+) => {
        #[allow(unused_unsafe)]
        unsafe {
            if $level <= $crate::ffi::g_cubeb_log_level.into() {
                cubeb_log_internal!($use_async, __INTERNAL__ format!($fmt, $($arg),*));
            }
        }
    };
    ($use_async: ident, __INTERNAL__ $msg: expr) => {
        fn get_cstr<'a>(buf: &'a mut [u8; 1024], file: &str, line: u32, msg: &str) -> &'a std::ffi::CStr {
            use std::io::Write;
            let filename = std::path::Path::new(file)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            // 2 for ':', 1 for ' ', 1 for '\n', and 1 for converting `line!()` to number of digits
            let len = filename.len() + ((line as f32).log10().trunc() as usize) + msg.len() + 5 + 5;
            debug_assert!(len < buf.len(), "log will be truncated");
            let _ = write!(&mut buf[..], "{}:{}: {}\n", filename, line, msg);
            let last = std::cmp::min(len, buf.len() - 1);
            buf[last] = 0;
            let cstr = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(&buf[..=last]) };
            cstr
        }

        match($use_async) {
            false => {
                if let Some(log_callback) = $crate::ffi::g_cubeb_log_callback {
                    let mut buf = [0u8; 1024];
                    log_callback(get_cstr(&mut buf, file!(), line!(), &$msg).as_ptr());
                }
            }
            true => {
                {   // Extra braces here to limit buf's lifetime
                    let mut buf = [0u8; 1024];
                    $crate::ffi::cubeb_async_log(get_cstr(&mut buf, file!(), line!(), &$msg).as_ptr());
                }
            }
        }
    }
}

#[macro_export]
macro_rules! cubeb_logv {
    ($msg: expr) => (cubeb_log_internal!(false, $crate::LogLevel::Verbose, $msg));
    ($fmt: expr, $($arg: expr),+) => (cubeb_log_internal!(false, $crate::LogLevel::Verbose, $fmt, $($arg),*));
}

#[macro_export]
macro_rules! cubeb_log {
    ($msg: expr) => (cubeb_log_internal!(false, $crate::LogLevel::Normal, $msg));
    ($fmt: expr, $($arg: expr),+) => (cubeb_log_internal!(false, $crate::LogLevel::Normal, $fmt, $($arg),*));
}

#[macro_export]
macro_rules! cubeb_alogv {
    ($msg: expr) => (cubeb_log_internal!(true, $crate::LogLevel::Verbose, $msg));
    ($fmt: expr, $($arg: expr),+) => (cubeb_log_internal!(true, $crate::LogLevel::Verbose, $fmt, $($arg),*));
}

#[macro_export]
macro_rules! cubeb_alog {
    ($msg: expr) => (cubeb_log_internal!(true, $crate::LogLevel::Normal, $msg));
    ($fmt: expr, $($arg: expr),+) => (cubeb_log_internal!(true, $crate::LogLevel::Verbose, $fmt, $($arg),*));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_normal_logging_sync() {
        cubeb_log!("This is synchronous log output at normal level");
        cubeb_log!("{} Formatted log", 1);
        cubeb_log!("{} Formatted {} log {}", 1, 2, 3);
    }

    #[test]
    fn test_verbose_logging_sync() {
        cubeb_logv!("This is synchronous log output at verbose level");
        cubeb_logv!("{} Formatted log", 1);
        cubeb_logv!("{} Formatted {} log {}", 1, 2, 3);
    }

    #[test]
    fn test_normal_logging_async() {
        cubeb_alog!("This is asynchronous log output at normal level");
        cubeb_alog!("{} Formatted log", 1);
        cubeb_alog!("{} Formatted {} log {}", 1, 2, 3);
    }

    #[test]
    fn test_verbose_logging_async() {
        cubeb_alogv!("This is asynchronous log output at verbose level");
        cubeb_alogv!("{} Formatted log", 1);
        cubeb_alogv!("{} Formatted {} log {}", 1, 2, 3);
    }
}
