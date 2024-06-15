use tokio_postgres::{NoTls, Client, Error};
use tokio_postgres::Connection;
use tokio_postgres::tls::NoTlsStream;
use tokio_postgres::Socket;

pub async fn connect() -> Result<(Client, Connection<Socket, NoTlsStream>), Error> {
    let (client, connection) = tokio_postgres::connect(&crate::config::get_database_url(), NoTls).await?;
    Ok((client, connection))
}
