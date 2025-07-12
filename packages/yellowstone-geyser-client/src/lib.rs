#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod types;

use crate::types::{GeyserClientConfig, SubscribeRequest, SubscribeUpdate};
use napi::{
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    tokio::task::JoinHandle,
    Error, Result,
};

#[napi]
pub struct GeyserSubscription {
    task_handle: JoinHandle<napi::Result<()>>,
    on_close: Option<ThreadsafeFunction<()>>,
}

#[napi]
impl GeyserSubscription {
    #[napi]
    pub fn close(&mut self) {
        self.on_close
            .as_mut()
            .map(|on_close| on_close.call(Ok(()), ThreadsafeFunctionCallMode::NonBlocking));
        self.task_handle.abort();
    }
}

#[napi]
pub struct GeyserClient {
    client: yellowstone_geyser_client::GeyserClient,
}

#[napi]
pub fn create_geyser_client(
    endpoint: String,
    config: Option<GeyserClientConfig>,
) -> Result<GeyserClient> {
    let client = yellowstone_geyser_client::GeyserClient::new(endpoint, config.map(|x| x.into()))
        .map_err(|e| Error::from_reason(e.to_string()))?;

    Ok(GeyserClient { client })
}

#[napi]
impl GeyserClient {
    #[napi]
    pub fn subscribe(
        &self,
        subscribe_request: Option<SubscribeRequest>,
        on_update: ThreadsafeFunction<SubscribeUpdate>,
        on_close: Option<ThreadsafeFunction<()>>,
    ) -> Result<GeyserSubscription> {
        let mut client = self.client.clone();
        let on_update = on_update.clone();
        let on_close_clone = on_close.clone();

        let task_handle = napi::tokio::spawn(async move {
            let result = async {
                let mut stream = client
                    .subscribe(subscribe_request.map(|x| x.into()).unwrap_or_default())
                    .await
                    .map_err(|e| Error::from_reason(e.to_string()))?;

                while let Some(update) = stream
                    .message()
                    .await
                    .map_err(|e| Error::from_reason(e.to_string()))?
                {
                    on_update.call(Ok(update.into()), ThreadsafeFunctionCallMode::NonBlocking);
                }

                Ok::<(), Error>(())
            }
            .await;

            if let Some(on_close) = on_close_clone {
                on_close.call(Ok(()), ThreadsafeFunctionCallMode::NonBlocking);
            }

            result
        });

        Ok(GeyserSubscription {
            task_handle,
            on_close,
        })
    }
}
