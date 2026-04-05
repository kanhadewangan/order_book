CREATE TABLE order_limit (
    id SERIAL PRIMARY KEY,
    stock_symbol VARCHAR(10) NOT NULL UNIQUE,
    max_quantity INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    -- no FK to orders needed
);

CREATE TABLE create_limit (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    stock_symbol VARCHAR(10) NOT NULL,
    quantity INTEGER NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (stock_symbol) REFERENCES order_limit(stock_symbol)
);

CREATE TABLE market_order (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    stock_symbol VARCHAR(10) NOT NULL,
    quantity INTEGER NOT NULL,
    order_type VARCHAR(4) NOT NULL CHECK (order_type IN ('BUY', 'SELL')),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (stock_symbol) REFERENCES order_limit(stock_symbol)
);