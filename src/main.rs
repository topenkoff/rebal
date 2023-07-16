use std::future::Future;

use anyhow::Result;

use rebal::config::{Config, LbStrategy};
use rebal::pool::Pool;
use rebal::server::Serve;
use rebal::server::Server;

fn main() -> Result<()> {
    let cfg = Config::new();
    let strategy = match cfg.server.strategy {
        LbStrategy::RoundRobin => rebal::strategy::robin_round::RoundRobin::new(),
        LbStrategy::LeastConnection => unimplemented!(),
    };
    let pool = Pool::new(cfg.server.upstreams, strategy);
    let serv_addr = cfg.server.listener.connection_str();
    let server = Server::new(serv_addr.as_str());
    run(async move {
        server.serve(pool).await?;
        Ok(())
    })
}

fn run<F>(fut: F) -> anyhow::Result<()>
where 
    F: Future<Output = anyhow::Result<()>>,
{
    #[cfg(all(feature = "tok-io", not(feature = "mono-io")))]
    {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        rt.block_on(fut)
    }
    #[cfg(all(feature = "mono-io", not(feature = "tok-io")))]
    {
        //let mut rt = monoio::RuntimeBuilder::<monoio::IoUringDriver>::new()
        let mut rt = monoio::RuntimeBuilder::<monoio::FusionDriver>::new()
            .with_entries(256)
            .enable_all()
            .build()?;
        rt.block_on(fut)
    }
    #[cfg(all(feature = "tok-io", feature = "mono-io"))]
    {
        compile_error!("choose one feature")
    }
    #[cfg(all(not(feature = "tok-io"), not(feature = "mono-io")))]
    {
        compile_error!("choose one feature")
    }
}
