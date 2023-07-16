use anyhow::Result;

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

#[cfg(feature = "tok-io")]
impl<'a> Serve for Server<'a> {
    async fn serve<U, S>(self, pool: Pool<U, S>) -> Result<()>
    where
        U: ToSock,
        S: Strategy<Upstream = U>,
    {
        use tokio::net::{TcpListener, TcpStream};

        let listener = TcpListener::bind(self.addr).await?;
        while let Ok((downstream, down_addr)) = listener.accept().await {
            let upstream_addr = pool.next_upstream().to_sock();
            println!("Proxy from {} to {}", &down_addr, &upstream_addr);
            let upstream = TcpStream::connect(upstream_addr).await?;
            let (rdown, wdown) = downstream.into_split();
            let (rup, wup) = upstream.into_split();
            let straight = Tunnel::new(rdown, wup);
            let reverse = Tunnel::new(rup, wdown);
            tokio::spawn(async move {
                let a = straight.await;
                dbg!(&a);
                let a = reverse.await;
                dbg!(&a);
            });
        }
        Ok(())
    }
}

#[cfg(feature = "mono-io")]
impl<'a> Serve for Server<'a> {
    async fn serve<U, S>(self, pool: Pool<U, S>) -> Result<()>
    where
        U: ToSock,
        S: Strategy<Upstream = U>,
    {
        use monoio::net::{TcpListener, TcpStream};
        use monoio::io::Splitable;

        let listener = TcpListener::bind(self.addr)?;
        while let Ok((downstream, down_addr)) = listener.accept().await {
            let upstream_addr = pool.next_upstream().to_sock();
            println!("Proxy from {} to {}", &down_addr, &upstream_addr);
            let upstream = TcpStream::connect(upstream_addr).await?;
            let (rdown, wdown) = downstream.into_split();
            let (rup, wup) = upstream.into_split();
            let straight = Tunnel::new(rdown, wup);
            let reverse = Tunnel::new(rup, wdown);
            monoio::spawn(async move {
                let a = straight.await;
                dbg!(&a);
                let a = reverse.await;
                dbg!(&a);
            });
        }
        Ok(())
    }
}
