use anyhow::Result;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpListener;

use rebal::config::Config;
use rebal::pool::Pool;
use rebal::server::Server;
use rebal::tunnel::Tunnel;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::new();
    let pool = Pool::new_robin_round(cfg.upstream);
    let serv_addr = cfg.listener.connection_str();
    let server = Server::new(serv_addr.as_str());
    server.serve(pool).await?;
    Ok(())
}
