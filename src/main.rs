use anyhow::Result;

use rebal::config::{Config, LbStrategy};
use rebal::pool::Pool;
use rebal::server::Server;
use rebal::server::Serve;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::new();
    let strategy = match cfg.server.strategy {
        LbStrategy::RoundRobin => rebal::strategy::robin_round::RoundRobin::new(),
        LbStrategy::LeastConnection => unimplemented!(),
    };
    let pool = Pool::new(cfg.server.upstreams, strategy);
    let serv_addr = cfg.server.listener.connection_str();
    let server = Server::new(serv_addr.as_str());
    server.serve(pool).await?;
    Ok(())
}
