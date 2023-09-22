use starknet::{
    accounts::SingleOwnerAccount,
    providers::{jsonrpc::HttpTransport, JsonRpcClient},
    signers::LocalWallet,
};

pub mod common;
pub mod config;

pub type KeeperAccount = SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>;
