use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::runtime::{AsyncRead, AsyncWrite};

impl<T> AsyncWrite for T
where
    T: monoio::io::AsyncWriteRent + Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        let fut = monoio::io::AsyncWriteRent::write(self.get_mut(), buf.to_vec());
        Box::pin(fut).as_mut().poll(cx).map(|(result, _)| result)
    }
}

impl<T> AsyncRead for T
where
    T: monoio::io::AsyncReadRent + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        //buf: &[u8],
    ) -> Poll<Result<(), std::io::Error>> {
        let buf: Vec<u8> = vec![1, 2, 3];
        let fut = monoio::io::AsyncReadRent::read(self.get_mut(), buf);
        Box::pin(fut).as_mut().poll(cx).map(|(_result, _)| Ok(()))
    }
}
