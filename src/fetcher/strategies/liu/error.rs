use std::io;

#[derive(thiserror::Error, Debug)]
pub enum StrategyError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    TlsError(#[from] async_native_tls::Error),
    #[error(transparent)]
    ImapError(#[from] async_imap::error::Error),
}
