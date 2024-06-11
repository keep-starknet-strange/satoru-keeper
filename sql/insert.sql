\c zohal

INSERT INTO orders (
    block_number,
    transaction_hash,
    key,
    order_type,
    decrease_position_swap_type,
    account,
    receiver,
    callback_contract,
    ui_fee_receiver,
    market,
    initial_collateral_token,
    swap_path,
    size_delta_usd,
    initial_collateral_delta_amount,
    trigger_price,
    acceptable_price,
    execution_fee,
    callback_gas_limit,
    min_output_amount,
    updated_at_block,
    is_long,
    is_frozen
) VALUES (
    123456,                                  -- block_number
    '0xabc123...',                           -- transaction_hash
    'some-key',                              -- key
    'buy',                                   -- order_type
    'none',                                  -- decrease_position_swap_type
    '0xaccount123...',                       -- account
    '0xreceiver123...',                      -- receiver
    '0xcallbackContract123...',              -- callback_contract
    '0xfeeReceiver123...',                   -- ui_fee_receiver
    'market-name',                           -- market
    'collateral-token',                      -- initial_collateral_token
    'swap-path',                             -- swap_path
    1000,                                    -- size_delta_usd
    500,                                     -- initial_collateral_delta_amount
    100,                                     -- trigger_price
    90,                                      -- acceptable_price
    10,                                      -- execution_fee
    20000,                                   -- callback_gas_limit
    100,                                     -- min_output_amount
    123456,                                  -- updated_at_block
    true,                                    -- is_long
    false                                    -- is_frozen
);