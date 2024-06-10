use futures::Future;

pub mod strategies;

pub trait Resender {
    type Error: std::error::Error;
    type Input;
    fn send(&self, input: Self::Input) -> impl Future<Output = Result<(), Self::Error>>;
}
