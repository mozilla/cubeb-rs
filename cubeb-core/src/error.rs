use ErrorCode;
use ffi;
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
            ffi::CUBEB_ERROR => ErrorCode::Error,
            ffi::CUBEB_ERROR_INVALID_FORMAT => ErrorCode::InvalidFormat,
            ffi::CUBEB_ERROR_INVALID_PARAMETER => ErrorCode::InvalidParameter,
            ffi::CUBEB_ERROR_NOT_SUPPORTED => ErrorCode::NotSupported,
            ffi::CUBEB_ERROR_DEVICE_UNAVAILABLE => ErrorCode::DeviceUnavailable,
            _ => super::ErrorCode::Error,
        }
    }

    pub fn raw_code(&self) -> ffi::cubeb_error_code {
        macro_rules! check(($($e:ident,)*) => (
            $(if self.code == ffi::$e as c_int { ffi::$e }) else *
            else {
                ffi::CUBEB_ERROR
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
            ffi::CUBEB_ERROR => "Error",
            ffi::CUBEB_ERROR_INVALID_FORMAT => "Invalid format",
            ffi::CUBEB_ERROR_INVALID_PARAMETER => "Invalid parameter",
            ffi::CUBEB_ERROR_NOT_SUPPORTED => "Not supported",
            ffi::CUBEB_ERROR_DEVICE_UNAVAILABLE => "Device unavailable",
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
        unsafe { Error::from_raw(ffi::CUBEB_ERROR) }
    }
}

impl From<Error> for i32 {
    fn from(e: Error) -> i32 {
        -e.code
    }
}
