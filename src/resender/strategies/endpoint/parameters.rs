use url::Url;

pub struct StrategyParameters {
    url: Url,
}

impl StrategyParameters {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub fn url(&self) -> &str {
        self.url.as_ref()
    }
}
