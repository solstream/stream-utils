#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod types;

use napi::{
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    Error, Result,
};

use crate::types::{ClientConfig, SubscribeRequest, SubscribeUpdate};

#[napi]
pub struct GeyserClient {
    client: yellowstone_geyser_client::GeyserClient,
}

#[napi]
pub async fn connect_geyser(
    endpoint: String,
    config: Option<ClientConfig>,
) -> Result<GeyserClient> {
    let config = config.map(|x| x.into());

    let client = yellowstone_geyser_client::GeyserClient::connect(endpoint, config)
        .await
        .map_err(|e| Error::from_reason(format!("Connection error: {}", e)))?;

    Ok(GeyserClient { client })
}

#[napi]
impl GeyserClient {
    #[napi]
    pub fn subscribe(
        &self,
        subscribe_request: Option<SubscribeRequest>,
        on_update: ThreadsafeFunction<SubscribeUpdate>,
    ) -> Result<()> {
        let mut client = self.client.clone();
        let on_update = on_update.clone();

        napi::tokio::spawn(async move {
            let mut stream = client
                .subscribe(subscribe_request.map(|x| x.into()).unwrap_or_default())
                .await
                .map_err(|e| Error::from_reason(format!("Subscription error: {}", e)))?;

            while let Some(update) = stream
                .message()
                .await
                .map_err(|e| Error::from_reason(format!("Stream error: {}", e)))?
            {
                on_update.call(Ok(update.into()), ThreadsafeFunctionCallMode::NonBlocking);
            }

            Ok::<(), Error>(())
        });

        Ok(())
    }
}
