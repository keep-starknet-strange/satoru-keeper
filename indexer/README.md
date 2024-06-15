```markdown
# Saturo Indexer

This is a specific indexer for the Saturo project to index all events from the Perps DEX. The indexer connects to the StarkNet blockchain, fetches relevant events, and stores them in a PostgreSQL database.

## Getting Started

### Prerequisites

- Rust and Cargo installed
- PostgreSQL database running
- `.env` file with the following content:

```
DATABASE_URL=your_database_url
```

## Setting Up PostgreSQL

### Install PostgreSQL:

- **Ubuntu**:
    ```sh
    sudo apt update
    sudo apt install postgresql postgresql-contrib
    ```
- **MacOS**:
    ```sh
    brew install postgresql
    ```
- **Windows**: Download and install PostgreSQL from [here](https://www.postgresql.org/download/windows/).

### Start PostgreSQL service:

- **Ubuntu**:
    ```sh
    sudo service postgresql start
    ```
- **MacOS**:
    ```sh
    brew services start postgresql
    ```
- **Windows**: Follow the instructions provided by the installer to start the PostgreSQL service.

### Create a new PostgreSQL user and database:

1. Open a terminal and run:
    ```sh
    sudo -u postgres psql
    ```
2. In the PostgreSQL shell, run:
    ```sql
    CREATE USER saturo_user WITH PASSWORD 'your_password';
    CREATE DATABASE saturo_db;
    GRANT ALL PRIVILEGES ON DATABASE saturo_db TO saturo_user;
    \q
    ```

### Set up the `.env` file with your database URL:

```bash
DATABASE_URL=postgres://saturo_user:your_password@localhost/saturo_db
```

### Execute the SQL scripts in the `sql` folder to create the necessary tables:

```sh
psql -U saturo_user -d saturo_db -f sql/db_setup.sql
```

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/your-repo/saturo-indexer.git
   cd saturo-indexer
   ```

2. Install the dependencies:
   ```sh
   cargo build
   ```

### Running the Indexer

1. Start the PostgreSQL database and ensure the `DATABASE_URL` in your `.env` file is correct.

2. Run the indexer:
   ```sh
   cargo run
   ```

## Project Modules

- `main.rs`: The entry point of the application. Sets up the environment, database connection, and event provider, and starts the event fetching process.
- `config.rs`: Contains configuration functions to get database and provider URLs from environment variables.
- `database.rs`: Handles the database connection setup.
- `provider.rs`: Sets up the StarkNet JSON-RPC provider.
- `events/`: Contains modules related to event handling.
  - `mod.rs`: Declares the `types` and `handler` sub-modules.
  - `types.rs`: Defines the `Order`, `Deposit`, and `Withdrawal` structs.
  - `handler.rs`: Contains the logic to fetch and process events from the StarkNet blockchain and insert them into the PostgreSQL database.

## Example Struct Definitions

### Order
```rust
pub struct Order {
    pub block_number: i64,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub order_type: Option<String>,
    pub decrease_position_swap_type: Option<String>,
    pub account: Option<String>,
    pub receiver: Option<String>,
    pub callback_contract: Option<String>,
    pub ui_fee_receiver: Option<String>,
    pub market: Option<String>,
    pub initial_collateral_token: Option<String>,
    pub swap_path: Option<String>,
    pub size_delta_usd: Option<String>,
    pub initial_collateral_delta_amount: Option<String>,
    pub trigger_price: Option<String>,
    pub acceptable_price: Option<String>,
    pub execution_fee: Option<String>,
    pub callback_gas_limit: Option<String>,
    pub min_output_amount: Option<String>,
    pub updated_at_block: Option<String>,
    pub is_long: Option<String>,
    pub is_frozen: Option<String>,
}
```

### Deposit
```rust
pub struct Deposit {
    pub block_number: i64,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub account: Option<String>,
    pub receiver: Option<String>,
    pub callback_contract: Option<String>,
    pub market: Option<String>,
    pub initial_long_token: Option<String>,
    pub initial_short_token: Option<String>,
    pub long_token_swap_path: Option<String>,
    pub short_token_swap_path: Option<String>,
    pub initial_long_token_amount: Option<String>,
    pub initial_short_token_amount: Option<String>,
    pub min_market_tokens: Option<String>,
    pub updated_at_block: Option<String>,
    pub execution_fee: Option<String>,
    pub callback_gas_limit: Option<String>,
}
```

### Withdrawal
```rust
pub struct Withdrawal {
    pub block_number: i64,
    pub transaction_hash: String,
    pub key: Option<String>,
    pub account: Option<String>,
    pub receiver: Option<String>,
    pub callback_contract: Option<String>,
    pub market: Option<String>,
    pub long_token_swap_path: Option<String>,
    pub short_token_swap_path: Option<String>,
    pub market_token_amount: Option<String>,
    pub min_long_token_amount: Option<String>,
    pub min_short_token_amount: Option<String>,
    pub updated_at_block: Option<String>,
    pub execution_fee: Option<String>,
    pub callback_gas_limit: Option<String>,
}
```

## Fetching and Processing Events

The main function in `handler.rs` fetches events from the StarkNet blockchain, processes them, and inserts them into the PostgreSQL database.

## License

This project is licensed under the MIT License.

## Contact

For any questions or issues, please contact [arif.bachir@hotmail.com].
```