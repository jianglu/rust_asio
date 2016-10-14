use std::result;
use std::mem;
use std::fmt;
use std::ffi::CStr;
use libc::{c_void, c_ulong};
use openssl_sys::{ERR_get_error, ERR_reason_error_string};

macro_rules! ssl_try {
    ($expr:expr) => (
        match unsafe { $expr } {
        rc if rc >= 0 => rc,
        _ => return Err(::ssl::Error::last_ssl_error()),
    })
}

macro_rules! ssl_unwrap {
    ($expr:expr) => (
        match unsafe { $expr } {
        rc if rc >= 0 => rc,
        _ => panic!("{}", ::ssl::Error::last_ssl_error()),
    })
}

macro_rules! ssl_ign {
    ($expr:expr) => (
        let _err = unsafe { $expr };
        debug_assert!(_err >= 0);
    )
}

extern {
    fn ERR_clear_error(_: c_void) -> c_void;
}

fn clear_error() {
    unsafe { ERR_clear_error(mem::uninitialized()) };
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Error(c_ulong);

impl Error {
    pub fn last_ssl_error() -> Error {
        Error(unsafe { ERR_get_error() })
    }

    pub fn message(&self) -> &CStr {
        unsafe { CStr::from_ptr(ERR_reason_error_string(self.0)) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.message())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.message())
    }
}

pub type Result<T> = result::Result<T, Error>;

mod context;
pub use self::context::{SslContext};

mod stream;
pub use self::stream::SslStream;
