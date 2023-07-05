use std::marker::PhantomData;
use std::sync::Mutex;

use super::Strategy;

pub struct RoundRobin<U> {
    current: Mutex<u32>,
    _p: PhantomData<U>,
}

impl<U> RoundRobin<U>
where
    U:,
{
    pub fn new() -> Self {
        Self {
            current: Mutex::new(0),
            _p: PhantomData::default(),
        }
    }
}

impl<U> Default for RoundRobin<U> {
    fn default() -> Self {
        Self::new()
    }
}

impl<U> Strategy for RoundRobin<U> {
    type Upstream = U;

    fn next_upstream<'a>(&'a self, pool: &'a [Self::Upstream]) -> &'a Self::Upstream {
        let mut current = self.current.lock().unwrap();
        *current = (*current + 1) % pool.len() as u32;
        &pool[*current as usize]
    }
}
