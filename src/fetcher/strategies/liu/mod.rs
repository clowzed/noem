use self::{error::StrategyError, parameters::StrategyParameters};
use crate::fetcher::Fetcher;
use async_imap::{types::Flag, Session};
use async_native_tls::{TlsConnector, TlsStream};
use chrono::{DateTime, FixedOffset};
use futures::{
    stream::{self, BoxStream},
    Stream, StreamExt,
};
use tokio::net::TcpStream;

pub mod error;
pub mod parameters;

type ImapSession = Session<TlsStream<TcpStream>>;

pub struct Strategy {
    parameters: StrategyParameters,
}

impl Strategy {
    pub fn new(parameters: StrategyParameters) -> Self {
        Self { parameters }
    }

    pub async fn fetch_body_by_uid(
        &self,
        session: &mut ImapSession,
        uid: u32,
    ) -> Result<Option<(u32, Vec<u8>)>, StrategyError> {
        if let Some(next) = session
            .uid_fetch(uid.to_string(), "BODY[]")
            .await?
            .next()
            .await
        {
            Ok(Some((uid, next?.body().unwrap_or_default().to_vec())))
        } else {
            Ok(None)
        }
    }
    pub async fn create_session(&self) -> Result<ImapSession, StrategyError> {
        let host = self.parameters.connection_parameters().host();
        let port = self.parameters.connection_parameters().port();

        let tcp_stream = TcpStream::connect((host, port)).await?;

        let tls_stream = TlsConnector::new()
            .connect(self.parameters.connection_parameters().host(), tcp_stream)
            .await?;

        let client = async_imap::Client::new(tls_stream);

        let login = self.parameters.credentials().login();
        let password = self.parameters.credentials().password();

        let session = client
            .login(login, password)
            .await
            .map_err(|error| error.0)?;

        Ok(session)
    }

    pub async fn fetch_latest_by_date_uid_if_unseen(
        &self,
        session: &mut ImapSession,
    ) -> Result<Option<u32>, StrategyError> {
        let mut messages = session.uid_fetch("1:*", "(UID FLAGS INTERNALDATE)").await?;

        let mut max_internal_date: Option<DateTime<FixedOffset>> = None;
        let mut uid = None;
        let mut message_with_max_internal_date = None;

        while let Some(message) = messages.next().await {
            let message = message?;
            let is_new_max_date = max_internal_date.map_or(true, |max_date| {
                message.internal_date().map_or(true, |date| date > max_date)
            });

            if is_new_max_date {
                max_internal_date = message.internal_date();
                uid = message.uid;
                message_with_max_internal_date = Some(message);
            }
        }

        match (uid, message_with_max_internal_date) {
            (Some(uid), Some(message)) => {
                let is_unseen = message.flags().all(|flag| flag != Flag::Seen);
                if is_unseen {
                    Ok(Some(uid))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }
}

type BoxedStream<'a> = BoxStream<'a, Result<Option<(u32, Vec<u8>)>, StrategyError>>;

impl Fetcher for Strategy {
    type Error = StrategyError;
    type Output = (u32, Vec<u8>);

    async fn fetch(
        &self,
    ) -> Result<impl Stream<Item = Result<Option<Self::Output>, Self::Error>>, Self::Error> {
        let mut session = self.create_session().await?;
        session.select(&self.parameters.folder()).await?;

        let uid = self
            .fetch_latest_by_date_uid_if_unseen(&mut session)
            .await?;

        let fetch_stream: BoxedStream = if let Some(uid) = uid {
            Box::pin(stream::once(async move {
                self.fetch_body_by_uid(&mut session, uid).await
            }))
        } else {
            Box::pin(stream::empty())
        };

        Ok(fetch_stream)
    }
}
