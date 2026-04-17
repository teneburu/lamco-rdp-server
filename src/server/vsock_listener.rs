#![expect(unused_imports, reason = "Conditional compilation for vsock feature")]

use std::io;

#[cfg(feature = "vsock")]
use tokio_vsock::{VsockListener, VsockStream};

#[cfg(feature = "vsock")]
pub fn bind_vsock(port: u32) -> io::Result<VsockListener> {
    let cid = u32::MAX;
    VsockListener::bind(cid, port)
}

#[cfg(not(feature = "vsock"))]
pub fn bind_vsock(_port: u32) -> io::Result<!> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "vsock support not compiled in. Rebuild with --features vsock",
    ))
}

#[cfg(feature = "vsock")]
pub type VsockListenerType = VsockListener;

#[cfg(feature = "vsock")]
pub type VsockStreamType = VsockStream;
