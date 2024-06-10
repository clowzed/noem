pub mod chain;
pub mod unified;
pub mod unimail;

pub trait Adapter {
    type Input;
    type Output;
    type Error;
    fn convert(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}
