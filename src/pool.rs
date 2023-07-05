use crate::strategy::robin_round::RoundRobin;
use crate::strategy::Strategy;

pub struct Pool<U, S> {
    upstreams: Vec<U>,
    strategy: S,
}

pub trait ToSock {
    fn to_sock(&self) -> std::net::SocketAddr;
}

impl<U, S> Pool<U, S>
where
    S: Strategy<Upstream = U>,
    U: ToSock,
{
    pub fn next_upstream(&self) -> &U {
        self.strategy.next_upstream(&self.upstreams)
    }

    pub fn new(upstreams: Vec<U>, strategy: S) -> Self {
        Self {
            upstreams,
            strategy,
        }
    }
}

impl<U> Pool<U, RoundRobin<U>> {
    pub fn new_robin_round(upstreams: Vec<U>) -> Self {
        let strategy = RoundRobin::new();
        Self {
            upstreams,
            strategy,
        }
    }
}
