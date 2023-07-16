use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::runtime::{AsyncRead, AsyncWrite};

pub struct Tunnel<D, U> {
    downstream: D,
    upstream: U,
}

impl<D, U> Tunnel<D, U> {
    pub fn new(downstream: D, upstream: U) -> Self {
        Self {
            downstream,
            upstream,
        }
    }
}

impl<D, U> Future for Tunnel<D, U>
where
    D: AsyncRead + Unpin,
    U: AsyncWrite + Unpin,
{
    type Output = Result<(), std::io::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let down = &mut this.downstream;
        let up = &mut this.upstream;
        let buf = vec![1, 2, 3];
        match Pin::new(down).poll_read(cx) {
            Poll::Ready(Ok(_res)) => Pin::new(up).poll_write(cx, &buf).map(|_| Ok(())),
            Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
            Poll::Pending => Poll::Pending,
        }
    }
}
