use crate::{adapter::Adapter, fetcher::Fetcher, resender::Resender};
use futures::StreamExt;

pub struct Manager<F, R, A>
where
    F: Fetcher,
    R: Resender,
    A: Adapter<Input = F::Output, Output = R::Input>,
{
    fetcher: F,
    resender: R,
    adapter: A,
}
impl<F, R, A> Manager<F, R, A>
where
    F: Fetcher,
    R: Resender,
    A: Adapter<Input = F::Output, Output = R::Input>,
{
    pub fn new(fetcher: F, resender: R, adapter: A) -> Self {
        Self {
            fetcher,
            resender,
            adapter,
        }
    }

    pub async fn run(&self) -> Result<(), F::Error> {
        let mut fetcher = Box::pin(self.fetcher.fetch().await?);

        while let Some(Ok(Some(message))) = fetcher.next().await {
            if let Ok(output) = self.adapter.convert(message) {
                self.resender.send(output).await.ok();
            }
        }
        Ok(())
    }
}
