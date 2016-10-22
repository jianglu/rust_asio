use std::io;
use io_service::{IoObject, IoService};
use async_result::{Handler, AsyncResult};
use stream::Stream;
use super::SslContext;

pub struct SslStream<S: Stream> {
    soc: S,
    ctx: SslContext,
}

impl<S: Stream> SslStream<S> {
    pub fn new(soc: S, ctx: &SslContext) -> SslStream<S> {
        SslStream {
            soc: soc,
            ctx: ctx.clone(),
        }
    }

    pub fn async_handshake(&self) {
    }

    pub fn async_shutdown(&self) {
    }

    pub fn handshake(&self) {
    }

    pub fn next_layer(&self) -> &S {
        &self.soc
    }

    pub fn set_verify_callback(&self) {
    }

    pub fn set_verify_depth(&self) {
    }

    pub fn set_verify_mode(&self) {
    }

    pub fn shutdown(&self) {
    }
}

impl<S: Stream> IoObject for SslStream<S> {
    fn io_service(&self) -> &IoService {
        self.soc.io_service()
    }
}

impl<S: Stream> Stream for SslStream<S> {
    fn async_read_some<F: Handler<usize>>(&self, buf: &mut [u8], handler: F) -> F::Output {
        handler.async_result().result(self.io_service())
    }

    fn async_write_some<F: Handler<usize>>(&self, buf: &[u8], handler: F) -> F::Output {
        handler.async_result().result(self.io_service())
    }

    fn read_some(&self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }

    fn write_some(&self, buf: &[u8]) -> io::Result<usize> {
        Ok(0)
    }
}

impl<S: Stream> Drop for SslStream<S> {
    fn drop(&mut self) {
    }
}
