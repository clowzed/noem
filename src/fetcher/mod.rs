pub mod strategies;
use futures::{Future, Stream};

pub trait Fetcher {
    type Error;
    type Output;

    fn fetch(
        &self,
    ) -> impl Future<
        Output = Result<impl Stream<Item = Result<Option<Self::Output>, Self::Error>>, Self::Error>,
    >;
}
