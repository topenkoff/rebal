use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

pub trait AsyncRead {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        //buf: &[u8],
    ) -> Poll<Result<(), std::io::Error>>;
}
