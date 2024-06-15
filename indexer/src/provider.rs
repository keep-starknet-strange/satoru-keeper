use url::Url;
use starknet::{
    providers::jsonrpc::{HttpTransport, JsonRpcClient},
};

pub fn get_provider() -> Result<JsonRpcClient<HttpTransport>, url::ParseError> {
    let provider_url = crate::config::get_provider_url();
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(&provider_url)?));
    Ok(provider)
}
