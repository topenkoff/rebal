pub mod robin_round;

pub trait Strategy {
    type Upstream;
    fn next_upstream<'a>(&'a self, pool: &'a [Self::Upstream]) -> &'a Self::Upstream;
}
