use std::fmt::Debug;

use log::info;
use serde::de::DeserializeOwned;
use sqlx::error::Error;
use sqlx::postgres::PgListener;
use sqlx::Pool;
use sqlx::Postgres;

// Listens to notifications from PostgreSQL channels.
// @pool: A reference to a connection pool for PostgreSQL.
// @channels: A vector of channel names to listen to.
// @call_back: A callback function to handle the deserialized payload.
pub async fn start_listening<T: DeserializeOwned + Sized + Debug>(
    pool: &Pool<Postgres>,
    channels: Vec<&str>,
    call_back: impl Fn(T),
) -> Result<(), Error> {
    // Initiate the logger.
    env_logger::init();

    let mut listener: PgListener = PgListener::connect_with(pool).await.expect("Could not connect to pool.");
    listener.listen_all(channels).await?;
    loop {
        while let Some(notification) = listener.try_recv().await? {
            info!(
                "Getting notification with payload: {:?} from channel {:?}",
                notification.payload(),
                notification.channel()
            );

            let strr = notification.payload().to_owned();
            let payload: T = serde_json::from_str::<T>(&strr).expect("Could not decode payload.");
            info!("Payload {:?}", payload);
            
            call_back(payload);
        }
    }
}
