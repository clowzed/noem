use super::Adapter;
use std::{convert::TryFrom, marker::PhantomData};

pub struct Unified<F, T: TryFrom<F>> {
    marker: PhantomData<(F, T)>,
}

impl<F, T: TryFrom<F>> Default for Unified<F, T> {
    fn default() -> Self {
        Self {
            marker: Default::default(),
        }
    }
}

impl<F, T: TryFrom<F>> Adapter for Unified<F, T> {
    type Input = F;
    type Output = T;
    type Error = <T as TryFrom<F>>::Error;

    fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        TryFrom::try_from(input)
    }
}
