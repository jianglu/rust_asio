use std::sync::Arc;
use openssl_sys::*;

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

    pub fn set_default_verify_paths(&self) {
        let _ = unsafe { SSL_CTX_set_default_verify_paths(self.0.handle) };
    }
}
