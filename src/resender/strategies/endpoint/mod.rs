pub mod error;
pub mod parameters;

use error::StrategyError;
use parameters::StrategyParameters;
use reqwest::{multipart::Form, Client};

use crate::resender::Resender;

pub struct Strategy {
    parameters: StrategyParameters,
}

impl Strategy {
    pub fn new(parameters: StrategyParameters) -> Self {
        Self { parameters }
    }
}

impl Resender for Strategy {
    type Error = StrategyError;
    type Input = Form;

    async fn send(&self, input: Self::Input) -> Result<(), Self::Error> {
        Client::new()
            .post(self.parameters.url())
            .multipart(input)
            .send()
            .await?;
        Ok(())
    }
}
