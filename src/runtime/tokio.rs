use std::pin::Pin;
use std::task::{Context, Poll};

use crate::runtime::{AsyncRead, AsyncWrite};

impl<T> AsyncWrite for T
where
    T: tokio::io::AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        tokio::io::AsyncWrite::poll_write(self, cx, buf)
    }
}

impl<T> AsyncRead for T
where
    T: tokio::io::AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        //buf: &[u8],
    ) -> Poll<Result<(), std::io::Error>> {
        let mut buf: Vec<u8> = vec![1, 2, 3];
        let mut buf = tokio::io::ReadBuf::new(&mut buf);
        tokio::io::AsyncRead::poll_read(self, cx, &mut buf)
    }
}
