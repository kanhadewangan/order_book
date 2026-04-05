-- Your SQL goes here

ALTER TABLE orders ADD CONSTRAINT orders_stock_symbol_unique UNIQUE (stock_symbol);