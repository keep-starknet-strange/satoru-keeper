use std::env;

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_provider_url() -> String {
    env::var("STARKNET_RPC_URL").expect("STARKNET_RPC_URL must be set")
}
