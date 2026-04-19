//! Synchronized VSOCK stream wrapper for IronRDP compatibility
//!
//! Provides a `Sync` implementation for `tokio_vsock::VsockStream` to satisfy
//! IronRDP's trait bounds on generic streams.

use std::io;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use parking_lot::Mutex;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

/// Wrapper that makes VSOCK stream `Sync`-compatible.
///
/// IronRDP's `run_connection` requires `S: Send + Sync + Unpin + AsyncRead + AsyncWrite`.
/// `tokio_vsock::VsockStream` wraps `AsyncFd` which is not automatically `Sync`,
/// even though it is `Send`. This wrapper provides explicit `Sync` implementation.
#[derive(Debug)]
pub struct SyncVsockStream {
    inner: Arc<Mutex<tokio_vsock::VsockStream>>,
}

impl SyncVsockStream {
    pub fn new(stream: tokio_vsock::VsockStream) -> Self {
        Self {
            inner: Arc::new(Mutex::new(stream)),
        }
    }
}

unsafe impl Send for SyncVsockStream {}
unsafe impl Sync for SyncVsockStream {}

impl AsyncRead for SyncVsockStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let mut guard = self.inner.lock();
        let pinned = unsafe { Pin::new_unchecked(&mut *guard) };
        pinned.poll_read(cx, buf)
    }
}

impl AsyncWrite for SyncVsockStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let mut guard = self.inner.lock();
        let pinned = unsafe { Pin::new_unchecked(&mut *guard) };
        pinned.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut guard = self.inner.lock();
        let pinned = unsafe { Pin::new_unchecked(&mut *guard) };
        pinned.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut guard = self.inner.lock();
        let pinned = unsafe { Pin::new_unchecked(&mut *guard) };
        pinned.poll_shutdown(cx)
    }
}
