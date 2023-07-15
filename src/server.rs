use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};

use crate::pool::{Pool, ToSock};
use crate::strategy::Strategy;
use crate::tunnel::Tunnel;

pub trait Serve {
    async fn serve<U, S>(self, pool: Pool<U, S>) -> Result<()>
    where
        U: ToSock,
        S: Strategy<Upstream = U>;
}

pub struct Server<'a> {
    addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(addr: &'a str) -> Self {
        Self { addr }
    }
}

impl<'a> Serve for Server<'a> {
    async fn serve<U, S>(self, pool: Pool<U, S>) -> Result<()>
    where
        U: ToSock,
        S: Strategy<Upstream = U>,
    {
        let listener = TcpListener::bind(self.addr).await?;
        while let Ok((downstream, down_addr)) = listener.accept().await {
            let upstream_addr = pool.next_upstream().to_sock();
            println!("Proxy from {} to {}", &down_addr, &upstream_addr);
            let upstream = TcpStream::connect(upstream_addr).await?;
            let mut tun = Tunnel::new(downstream, upstream);
            tokio::spawn(async move {
                let _ = tun.connect().await;
            });
        }
        Ok(())
    }
}
