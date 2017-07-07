use {ErrorCode, raw};
use std::error;
use std::ffi::NulError;
use std::fmt;
use std::os::raw::c_int;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Error {
    code: c_int
}

impl Error {
    pub unsafe fn from_raw(code: c_int) -> Error {
        Error {
            code: -code
        }
    }

    pub fn code(&self) -> ErrorCode {
        match self.raw_code() {
            raw::CUBEB_ERROR => ErrorCode::Error,
            raw::CUBEB_ERROR_INVALID_FORMAT => ErrorCode::InvalidFormat,
            raw::CUBEB_ERROR_INVALID_PARAMETER => ErrorCode::InvalidParameter,
            raw::CUBEB_ERROR_NOT_SUPPORTED => ErrorCode::NotSupported,
            raw::CUBEB_ERROR_DEVICE_UNAVAILABLE => ErrorCode::DeviceUnavailable,
            _ => super::ErrorCode::Error,
        }
    }

    pub fn raw_code(&self) -> raw::cubeb_error_code {
        macro_rules! check(($($e:ident,)*) => (
            $(if self.code == raw::$e as c_int { raw::$e }) else *
            else {
                raw::CUBEB_ERROR
            }
        ));
        check!(
            CUBEB_OK,
            CUBEB_ERROR,
            CUBEB_ERROR_INVALID_FORMAT,
            CUBEB_ERROR_INVALID_PARAMETER,
            CUBEB_ERROR_NOT_SUPPORTED,
            CUBEB_ERROR_DEVICE_UNAVAILABLE,
        )
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.code {
            raw::CUBEB_ERROR => "Error",
            raw::CUBEB_ERROR_INVALID_FORMAT => "Invalid format",
            raw::CUBEB_ERROR_INVALID_PARAMETER => "Invalid parameter",
            raw::CUBEB_ERROR_NOT_SUPPORTED => "Not supported",
            raw::CUBEB_ERROR_DEVICE_UNAVAILABLE => "Device unavailable",
            _ => panic!("Invalid cubeb error"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.description())
    }
}

impl From<NulError> for Error {
    fn from(_: NulError) -> Error {
        unsafe { Error::from_raw(raw::CUBEB_ERROR) }
    }
}
