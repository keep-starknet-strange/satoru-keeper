use std::env;

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_provider_url() -> String {
    env::var("STARKNET_RPC_URL").expect("STARKNET_RPC_URL must be set")
}

pub fn get_from_block() -> u64 {
    env::var("FROM_BLOCK")
        .expect("FROM_BLOCK must be set")
        .parse::<u64>()
        .expect("FROM_BLOCK must be a valid u64 number")
}
