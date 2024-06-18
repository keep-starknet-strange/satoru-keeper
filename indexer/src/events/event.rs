pub struct GenericEvent {
    pub block_number: i64,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub data: String,
    SwapFeesCollected(GenericEvent),
}

pub enum EventType {
    Order(GenericEvent),
    Deposit(GenericEvent),
    Withdrawal(GenericEvent),
    MarketCreated(GenericEvent),
}
