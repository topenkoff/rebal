use anyhow::Result;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;

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

impl<D, U> Tunnel<D, U>
where
    U: AsyncRead + AsyncWrite + Unpin,
    D: AsyncRead + AsyncWrite + Unpin,
{
    pub async fn connect(&mut self) -> Result<()> {
        tokio::io::copy_bidirectional(&mut self.downstream, &mut self.upstream).await?;
        Ok(())
    }
}
