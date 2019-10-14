use std::pin::Pin;
use tokio_io::AsyncRead;
use core::task::{Context, Poll};
use std::io;
use std::fmt;

/// A reader which is always at EOF.
pub struct Empty { _priv: () }

/// Constructs a new handle to an empty reader.
///
pub fn empty() -> Empty { Empty { _priv: () } }

impl AsyncRead for Empty {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context<'_>,
        _: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(0))
    }
}

// impl BufRead for Empty {
//     #[inline]
//     fn fill_buf(&mut self) -> io::Result<&[u8]> { Ok(&[]) }
//     #[inline]
//     fn consume(&mut self, _n: usize) {}
// }

impl fmt::Debug for Empty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Empty { .. }")
    }
}
