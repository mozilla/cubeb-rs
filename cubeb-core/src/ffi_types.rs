// Copyright © 2017-2018 Mozilla Foundation
//
// This program is made available under an ISC-style license.  See the
// accompanying file LICENSE for details.

/// A macro to define a wrapper around a POD FFI type that lives on the
/// heap.  This differs from `foreign_type!` because that macros assumes
/// that drop can be implemented just by passing the pointer to a
/// function. Cubeb's APIs are more complicated.
macro_rules! ffi_type_heap {
    (
        $(#[$impl_attr:meta])*
        type CType = $ctype:ty;
        $(fn drop = $drop:expr;)*
        $(fn clone = $clone:expr;)*
        $(#[$owned_attr:meta])*
        pub struct $owned:ident;
        $(#[$borrowed_attr:meta])*
        pub struct $borrowed:ident;
    ) => {
        $(#[$owned_attr])*
        pub struct $owned(*mut $ctype);

        $(#[$impl_attr])*
        impl ::foreign_types::ForeignType for $owned {
            type CType = $ctype;
            type Ref = $borrowed;

            #[inline]
            unsafe fn from_ptr(ptr: *mut $ctype) -> $owned {
                $owned(ptr)
            }

            #[inline]
            fn as_ptr(&self) -> *mut $ctype {
                self.0
            }
        }

        $(
            impl Drop for $owned {
                #[inline]
                fn drop(&mut self) {
                    unsafe { $drop(self.0) }
                }
            }
        )*

        $(
            impl Clone for $owned {
                #[inline]
                fn clone(&self) -> $owned {
                    unsafe {
                        let handle: *mut $ctype = $clone(self.0);
                        ::foreign_types::ForeignType::from_ptr(handle)
                    }
                }
            }

            impl ::std::borrow::ToOwned for $borrowed {
                type Owned = $owned;
                #[inline]
                fn to_owned(&self) -> $owned {
                    unsafe {
                        let handle: *mut $ctype = $clone(::foreign_types::ForeignTypeRef::as_ptr(self));
                        ::foreign_types::ForeignType::from_ptr(handle)
                    }
                }
            }
        )*

        impl ::std::ops::Deref for $owned {
            type Target = $borrowed;

            #[inline]
            fn deref(&self) -> &$borrowed {
                unsafe { ::foreign_types::ForeignTypeRef::from_ptr(self.0) }
            }
        }

        impl ::std::ops::DerefMut for $owned {
            #[inline]
            fn deref_mut(&mut self) -> &mut $borrowed {
                unsafe { ::foreign_types::ForeignTypeRef::from_ptr_mut(self.0) }
            }
        }

        impl ::std::borrow::Borrow<$borrowed> for $owned {
            #[inline]
            fn borrow(&self) -> &$borrowed {
                &**self
            }
        }

        impl ::std::convert::AsRef<$borrowed> for $owned {
            #[inline]
            fn as_ref(&self) -> &$borrowed {
                &**self
            }
        }

        $(#[$borrowed_attr])*
        pub struct $borrowed(::foreign_types::Opaque);

        $(#[$impl_attr])*
        impl ::foreign_types::ForeignTypeRef for $borrowed {
            type CType = $ctype;
        }

        impl ::std::fmt::Debug for $borrowed {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let ptr = self as *const $borrowed as usize;
                f.debug_tuple(stringify!($borrowed))
                    .field(&ptr)
                    .finish()
            }
        }
    }
}

/// A macro to define a wrapper around a POD FFI type that lives on
/// the stack.
macro_rules! ffi_type_stack {
    ($(#[$impl_attr:meta])*
     type CType = $ctype:ty;
     $(#[$owned_attr:meta])*
     pub struct $owned:ident;
     $(#[$borrowed_attr:meta])*
     pub struct $borrowed:ident;
    ) => {
        $(#[$owned_attr])*
        pub struct $owned($ctype);

        $(#[$impl_attr])*
        impl ::foreign_types::ForeignType for $owned {
            type CType = $ctype;
            type Ref = $borrowed;

            unsafe fn from_ptr(_: *mut $ctype) -> $owned {
                // $owned(*ptr)
                panic!("Not implemented.")
            }

            fn as_ptr(&self) -> *mut Self::CType {
                &self.0 as *const $ctype as *mut $ctype
            }
        }

        impl Default for $owned {
            fn default() -> $owned {
                $owned(Default::default())
            }
        }

        impl From<$ctype> for $owned {
            fn from(x: $ctype) -> $owned {
                $owned(x)
            }
        }

        impl ::std::borrow::ToOwned for $borrowed {
            type Owned = $owned;
            fn to_owned(&self) -> $owned {
                unsafe {
                    ::foreign_types::ForeignType::from_ptr(self.as_ptr())
                }
            }
        }

        impl ::std::ops::Deref for $owned {
            type Target = $borrowed;

            #[inline]
            fn deref(&self) -> &$borrowed {
                let ptr = &self.0 as *const $ctype as *mut $ctype;
                unsafe { ::foreign_types::ForeignTypeRef::from_ptr(ptr) }
            }
        }

        impl ::std::ops::DerefMut for $owned {
            #[inline]
            fn deref_mut(&mut self) -> &mut $borrowed {
                let ptr = &self.0 as *const $ctype as *mut $ctype;
                unsafe { ::foreign_types::ForeignTypeRef::from_ptr_mut(ptr) }
            }
        }

        impl ::std::borrow::Borrow<$borrowed> for $owned {
            #[inline]
            fn borrow(&self) -> &$borrowed {
                &**self
            }
        }

        impl ::std::convert::AsRef<$borrowed> for $owned {
            #[inline]
            fn as_ref(&self) -> &$borrowed {
                &**self
            }
        }

        $(#[$borrowed_attr])*
        pub struct $borrowed(::foreign_types::Opaque);

        $(#[$impl_attr])*
        impl ::foreign_types::ForeignTypeRef for $borrowed {
            type CType = $ctype;
        }

        impl ::std::fmt::Debug for $borrowed {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let ptr = self as *const $borrowed as usize;
                f.debug_tuple(stringify!($borrowed))
                    .field(&ptr)
                    .finish()
            }
        }
    }
}
