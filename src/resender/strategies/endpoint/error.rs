#[derive(thiserror::Error, Debug)]
pub enum StrategyError {
    #[error(transparent)]
    TransportError(#[from] reqwest::Error),
}
