use crate::types::client::AnthropicClient;
use crate::types::model::{ListModelsParams, ListModelsResponse, ModelClient, ModelError};
use async_trait::async_trait;

#[async_trait]
impl ModelClient for AnthropicClient {
    /// List available models
    ///
    /// Returns a list of models that are available through the API.
    /// More recently released models are listed first.
    async fn list_models<'a>(
        &'a self,
        params: Option<&'a ListModelsParams>,
    ) -> Result<ListModelsResponse, ModelError> {
        self.send_request(reqwest::Method::GET, "/models", params)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add tests for list_models
}
