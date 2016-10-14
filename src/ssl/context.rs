use std::mem;
use std::ptr;
use std::sync::Arc;
use std::ffi::CString;
use std::path::Path;
use openssl_sys::*;
use libc::{c_void};
use super::{Error, Result, clear_error};

struct ContextImpl {
    handle: *mut SSL_CTX,
}

impl ContextImpl {
    fn new(method: unsafe extern "C" fn() -> *const SSL_METHOD) -> ContextImpl {
        let handle = unsafe { SSL_CTX_new(method()) };
        if handle.is_null() {
            panic!("invalid method");
        }
        ContextImpl {
            handle: handle
        }
    }
}

unsafe impl Send for ContextImpl {
}

unsafe impl Sync for ContextImpl {
}

#[derive(Clone)]
pub struct SslContext(Arc<ContextImpl>);

impl SslContext {
    pub fn sslv23() -> SslContext {
        SslContext(Arc::new(ContextImpl::new(SSLv23_method)))
    }

    pub fn sslv3() -> SslContext {
        SslContext(Arc::new(ContextImpl::new(SSLv3_method)))
    }

    pub fn tlsv1() -> SslContext {
        SslContext(Arc::new(ContextImpl::new(TLSv1_method)))
    }

    pub fn add_certificate_authority() {
    }

    pub fn add_verify_path<P>(&self, path: P) -> Result<()>
        where P: AsRef<Path>,
    {
        let path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let ptr = path.as_bytes_with_nul().as_ptr() as *const i8;
        clear_error();
        ssl_try!(SSL_CTX_load_verify_locations(self.0.handle, ptr::null(), ptr));
        Ok(())
    }

    pub fn clear_options() {
    }

    pub fn load_verify_file() {
    }

    pub fn set_default_verify_paths(&self) -> Result<()> {
        clear_error();
        ssl_try!(unsafe { SSL_CTX_set_default_verify_paths(self.0.handle) });
        Ok(())
    }

    pub fn set_options() {
    }

    pub fn set_password_callback() {
    }

    pub fn set_verify_callback() {
    }

    pub fn set_verify_depth(depth: i32) {
    }

    pub fn set_verify_mode() {
    }

    pub fn use_certificate() {
    }

    pub fn use_certificate_chain() {
    }

    pub fn use_certificate_chain_file() {
    }

    pub fn use_certificate_file() {
    }

    pub fn use_private_key() {
    }

    pub fn use_rsa_private_key() {
    }

    pub fn use_rsa_prive_key_file() {
    }

    pub fn use_tmp_dh() {
    }

    pub fn use_tmp_dh_file() {
    }
    }
