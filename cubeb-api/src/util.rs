use {Error, ffi};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[doc(hidden)]
pub trait IsNullPtr {
    fn is_ptr_null(&self) -> bool;
}

impl<T> IsNullPtr for *const T {
    fn is_ptr_null(&self) -> bool {
        self.is_null()
    }
}

impl<T> IsNullPtr for *mut T {
    fn is_ptr_null(&self) -> bool {
        self.is_null()
    }
}

#[doc(hidden)]
pub trait Binding: Sized {
    type Raw;

    unsafe fn from_raw(raw: Self::Raw) -> Self;
    fn raw(&self) -> Self::Raw;

    unsafe fn from_raw_opt<T>(raw: T) -> Option<Self>
    where
        T: Copy + IsNullPtr,
        Self: Binding<Raw = T>,
    {
        if raw.is_ptr_null() {
            None
        } else {
            Some(Binding::from_raw(raw))
        }
    }
}

/// A class of types that can be converted to C strings.
///
/// These types are represented internally as byte slices and it is quite rare
/// for them to contain an interior 0 byte.
pub trait IntoCString {
    /// Consume this container, converting it into a CString
    fn into_c_string(self) -> Result<CString, Error>;
}

impl<'a, T: IntoCString + Clone> IntoCString for &'a T {
    fn into_c_string(self) -> Result<CString, Error> {
        self.clone().into_c_string()
    }
}

impl<'a> IntoCString for &'a str {
    fn into_c_string(self) -> Result<CString, Error> {
        match CString::new(self) {
            Ok(s) => Ok(s),
            Err(_) => Err(unsafe { Error::from_raw(ffi::CUBEB_ERROR) }),
        }
    }
}

impl IntoCString for String {
    fn into_c_string(self) -> Result<CString, Error> {
        match CString::new(self.into_bytes()) {
            Ok(s) => Ok(s),
            Err(_) => Err(unsafe { Error::from_raw(ffi::CUBEB_ERROR) }),
        }
    }
}

impl IntoCString for CString {
    fn into_c_string(self) -> Result<CString, Error> {
        Ok(self)
    }
}

pub unsafe fn opt_bytes<T>(_anchor: &T, c: *const c_char) -> Option<&[u8]> {
    if c.is_null() {
        None
    } else {
        Some(CStr::from_ptr(c).to_bytes())
    }
}

pub fn opt_cstr<T>(o: Option<T>) -> Result<Option<CString>, Error>
where
    T: IntoCString,
{
    match o {
        Some(s) => s.into_c_string().map(Some),
        None => Ok(None),
    }
}
