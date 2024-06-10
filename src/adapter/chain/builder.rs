use crate::adapter::Adapter;

use super::chain::Chain;

pub struct ChainBuilder<A> {
    adapter: A,
}

impl<A> ChainBuilder<A> {
    pub fn new(adapter: A) -> Self {
        ChainBuilder { adapter }
    }

    pub fn chain<B>(self, next_adapter: B) -> ChainBuilder<Chain<A, B>>
    where
        A: Adapter,
        B: Adapter<Input = A::Output>,
        A::Error: 'static,
        B::Error: 'static,
    {
        ChainBuilder {
            adapter: Chain::new(self.adapter, next_adapter),
        }
    }

    pub fn build(self) -> A {
        self.adapter
    }
}
