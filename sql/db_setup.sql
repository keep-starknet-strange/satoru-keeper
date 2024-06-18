\c zohal

CREATE TABLE IF NOT EXISTS orders (
    block_number BIGINT NOT NULL,
    transaction_hash TEXT NOT NULL,
    key TEXT,
    order_type TEXT,
    decrease_position_swap_type TEXT,
    account TEXT,
    receiver TEXT,
    callback_contract TEXT,
    ui_fee_receiver TEXT,
    market TEXT,
    initial_collateral_token TEXT,
    swap_path TEXT,
    size_delta_usd BIGINT,
    initial_collateral_delta_amount BIGINT,
    trigger_price BIGINT,
    acceptable_price BIGINT,
    execution_fee BIGINT,
    callback_gas_limit BIGINT,
    min_output_amount BIGINT,
    updated_at_block BIGINT,
    is_long BOOLEAN,
    is_frozen BOOLEAN
);
CREATE TABLE IF NOT EXISTS deposits (
    block_number BIGINT NOT NULL,
    transaction_hash TEXT NOT NULL,
    key TEXT,
    account TEXT,
    receiver TEXT,
    callback_contract TEXT,
    ui_fee_receiver TEXT,
    market TEXT,
    initial_long_token TEXT,
    initial_short_token TEXT,
    long_token_swap_path TEXT,
    short_token_swap_path TEXT,
    initial_long_token_amount BIGINT,
    initial_short_token_amount BIGINT,
    min_market_tokens BIGINT,
    updated_at_block BIGINT,
    execution_fee BIGINT,
    callback_gas_limit BIGINT
);
CREATE TABLE IF NOT EXISTS withdrawals (
    block_number BIGINT NOT NULL,
    transaction_hash TEXT NOT NULL,
    key TEXT,
    account TEXT,
    receiver TEXT,
    callback_contract TEXT,
    ui_fee_receiver TEXT,
    market TEXT,
    long_token_swap_path TEXT,
    short_token_swap_path TEXT,
    market_token_amount BIGINT,
    min_long_token_amount BIGINT,
    min_short_token_amount BIGINT,
    updated_at_block BIGINT,
    execution_fee BIGINT,
    callback_gas_limit BIGINT
);

CREATE TABLE IF NOT EXISTS market_created (
    block_number BIGINT NOT NULL,
    transaction_hash TEXT NOT NULL,
    key TEXT,
    creator TEXT,
    market_token TEXT,
    index_token TEXT,
    long_token TEXT,
    short_token TEXT,
    market_type TEXT
);

CREATE TABLE IF NOT EXISTS swap_fees_collected (
    block_number BIGINT NOT NULL,
    transaction_hash TEXT NOT NULL,
    key TEXT,
    market TEXT,
    token TEXT,
    token_price BIGINT,
    action TEXT,
    fee_receiver_amount BIGINT,
    fee_amount_for_pool BIGINT,
    amount_after_fees BIGINT,
    ui_fee_receiver TEXT,
    ui_fee_receiver_factor BIGINT,
    ui_fee_amount BIGINT
);

-- Drop the existing function and triggers if it exists
DROP TRIGGER IF EXISTS orders_notify_update ON orders;
DROP TRIGGER IF EXISTS orders_notify_insert ON orders;

DROP FUNCTION IF EXISTS orders_update_notify();

-- Add a table update notification function
CREATE OR REPLACE FUNCTION orders_update_notify() RETURNS trigger AS $$
DECLARE
  payload json;
BEGIN
  IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
    payload = row_to_json(NEW);
  ELSE
    payload = row_to_json(OLD);
  END IF;
  PERFORM pg_notify('orders_update', json_build_object('table', TG_TABLE_NAME, 'action_type', TG_OP, 'row_data', payload)::text);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Add UPDATE row trigger
CREATE TRIGGER orders_notify_update AFTER UPDATE ON orders FOR EACH ROW EXECUTE PROCEDURE orders_update_notify();

-- Add INSERT row trigger
CREATE TRIGGER orders_notify_insert AFTER INSERT ON orders FOR EACH ROW EXECUTE PROCEDURE orders_update_notify();

-- Drop the existing function and triggers if it exists
DROP TRIGGER IF EXISTS withdrawals_notify_update ON withdrawals;
DROP TRIGGER IF EXISTS withdrawals_notify_insert ON withdrawals;

DROP FUNCTION IF EXISTS withdrawals_update_notify();

-- Add a table update notification function
CREATE OR REPLACE FUNCTION withdrawals_update_notify() RETURNS trigger AS $$
DECLARE
  payload json;
BEGIN
  IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
    payload = row_to_json(NEW);
  ELSE
    payload = row_to_json(OLD);
  END IF;
  PERFORM pg_notify('withdrawals_update', json_build_object('table', TG_TABLE_NAME, 'action_type', TG_OP, 'row_data', payload)::text);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Add UPDATE row trigger
CREATE TRIGGER withdrawals_notify_update AFTER UPDATE ON withdrawals FOR EACH ROW EXECUTE PROCEDURE withdrawals_update_notify();

-- Add INSERT row trigger
CREATE TRIGGER withdrawals_notify_insert AFTER INSERT ON withdrawals FOR EACH ROW EXECUTE PROCEDURE withdrawals_update_notify();

-- Drop the existing function and triggers if it exists
DROP TRIGGER IF EXISTS market_created_notify_update ON market_created;
DROP TRIGGER IF EXISTS market_created_notify_insert ON market_created;

DROP FUNCTION IF EXISTS market_created_update_notify();

-- Add a table update notification function
CREATE OR REPLACE FUNCTION market_created_update_notify() RETURNS trigger AS $$
DECLARE
  payload json;
BEGIN
  IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
    payload = row_to_json(NEW);
  ELSE
    payload = row_to_json(OLD);
  END IF;
  PERFORM pg_notify('market_created_update', json_build_object('table', TG_TABLE_NAME, 'action_type', TG_OP, 'row_data', payload)::text);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Add UPDATE row trigger
CREATE TRIGGER market_created_notify_update AFTER UPDATE ON market_created FOR EACH ROW EXECUTE PROCEDURE market_created_update_notify();

-- Add INSERT row trigger
CREATE TRIGGER market_created_notify_insert AFTER INSERT ON market_created FOR EACH ROW EXECUTE PROCEDURE market_created_update_notify();

-- Drop the existing function and triggers if it exists
DROP TRIGGER IF EXISTS swap_fees_collected_notify_update ON swap_fees_collected;
DROP TRIGGER IF EXISTS swap_fees_collected_notify_insert ON swap_fees_collected;

DROP FUNCTION IF EXISTS swap_fees_collected_update_notify();

-- Add a table update notification function
CREATE OR REPLACE FUNCTION swap_fees_collected_update_notify() RETURNS trigger AS $$
DECLARE
  payload json;
BEGIN
  IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
    payload = row_to_json(NEW);
  ELSE
    payload = row_to_json(OLD);
  END IF;
  PERFORM pg_notify('swap_fees_collected_update', json_build_object('table', TG_TABLE_NAME, 'action_type', TG_OP, 'row_data', payload)::text);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Add UPDATE row trigger
CREATE TRIGGER swap_fees_collected_notify_update AFTER UPDATE ON swap_fees_collected FOR EACH ROW EXECUTE PROCEDURE swap_fees_collected_update_notify();

-- Add INSERT row trigger
CREATE TRIGGER swap_fees_collected_notify_insert AFTER INSERT ON swap_fees_collected FOR EACH ROW EXECUTE PROCEDURE swap_fees_collected_update_notify();