use crate::models::{Orders, NewOrder};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

/// Creates fake order data for testing
pub mod fake_orders {
    use super::*;

    /// Sample order with default values
    pub fn sample_order() -> Orders {
        Orders {
            id: 1,
            user_id: 1,
            stock_symbol: "AAPL".to_string(),
            quantity: 100,
            price: BigDecimal::from(150),
            order_type: "BUY".to_string(),
            status: "PENDING".to_string(),
            created_at: None,
        }
    }

    /// Multiple sample orders for bulk testing
    pub fn sample_orders() -> Vec<Orders> {
        vec![
            Orders {
                id: 1,
                user_id: 1,
                stock_symbol: "AAPL".to_string(),
                quantity: 100,
                price: BigDecimal::from(150),
                order_type: "BUY".to_string(),
                status: "PENDING".to_string(),
                created_at: None,
            },
            Orders {
                id: 2,
                user_id: 1,
                stock_symbol: "GOOGL".to_string(),
                quantity: 50,
                price: BigDecimal::from(2800),
                order_type: "SELL".to_string(),
                status: "COMPLETED".to_string(),
                created_at: None,
            },
            Orders {
                id: 3,
                user_id: 2,
                stock_symbol: "MSFT".to_string(),
                quantity: 75,
                price: BigDecimal::from(300),
                order_type: "BUY".to_string(),
                status: "CANCELLED".to_string(),
                created_at: None,
            },
            Orders {
                id: 4,
                user_id: 2,
                stock_symbol: "TSLA".to_string(),
                quantity: 200,
                price: BigDecimal::from(250),
                order_type: "BUY".to_string(),
                status: "FILLED".to_string(),
                created_at: None,
            },
            Orders {
                id: 5,
                user_id: 3,
                stock_symbol: "AMZN".to_string(),
                quantity: 25,
                price: BigDecimal::from(3500),
                order_type: "SELL".to_string(),
                status: "PENDING".to_string(),
                created_at: None,
            },
        ]
    }

    /// New order for insertion tests (without ID)
    pub fn new_sample_order() -> NewOrder {
        NewOrder {
            user_id: 1,
            stock_symbol: "AAPL".to_string(),
            quantity: 100,
            price: BigDecimal::from(150),
            order_type: "BUY".to_string(),
            status: "PENDING".to_string(),
        }
    }

    /// New orders for bulk insertion tests
    pub fn new_sample_orders() -> Vec<NewOrder> {
        vec![
            NewOrder {
                user_id: 1,
                stock_symbol: "AAPL".to_string(),
                quantity: 100,
                price: BigDecimal::from(150),
                order_type: "BUY".to_string(),
                status: "PENDING".to_string(),
            },
            NewOrder {
                user_id: 1,
                stock_symbol: "GOOGL".to_string(),
                quantity: 50,
                price: BigDecimal::from(2800),
                order_type: "SELL".to_string(),
                status: "PENDING".to_string(),
            },
            NewOrder {
                user_id: 2,
                stock_symbol: "MSFT".to_string(),
                quantity: 75,
                price: BigDecimal::from(300),
                order_type: "BUY".to_string(),
                status: "PENDING".to_string(),
            },
        ]
    }

    /// High-value order (for edge case testing)
    pub fn high_value_order() -> NewOrder {
        NewOrder {
            user_id: 1,
            stock_symbol: "BRK.A".to_string(),
            quantity: 1,
            price: BigDecimal::from(500000),
            order_type: "BUY".to_string(),
            status: "PENDING".to_string(),
        }
    }

    /// Low-value order
    pub fn low_value_order() -> NewOrder {
        NewOrder {
            user_id: 1,
            stock_symbol: "PENNY".to_string(),
            quantity: 10000,
            price: BigDecimal::from(1),
            order_type: "BUY".to_string(),
            status: "PENDING".to_string(),
        }
    }

    /// Sell order
    pub fn sell_order() -> NewOrder {
        NewOrder {
            user_id: 2,
            stock_symbol: "TSLA".to_string(),
            quantity: 50,
            price: BigDecimal::from(250),
            order_type: "SELL".to_string(),
            status: "PENDING".to_string(),
        }
    }

    /// Order with various statuses
    pub fn order_with_status(status: &str) -> NewOrder {
        NewOrder {
            user_id: 1,
            stock_symbol: "AAPL".to_string(),
            quantity: 100,
            price: BigDecimal::from(150),
            order_type: "BUY".to_string(),
            status: status.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_order_creation() {
        let order = fake_orders::sample_order();
        assert_eq!(order.id, 1);
        assert_eq!(order.stock_symbol, "AAPL");
        assert_eq!(order.quantity, 100);
    }

    #[test]
    fn test_sample_orders_count() {
        let orders = fake_orders::sample_orders();
        assert_eq!(orders.len(), 5);
    }

    #[test]
    fn test_new_sample_order() {
        let order = fake_orders::new_sample_order();
        assert_eq!(order.user_id, 1);
        assert_eq!(order.order_type, "BUY");
    }

    #[test]
    fn test_sell_order() {
        let order = fake_orders::sell_order();
        assert_eq!(order.order_type, "SELL");
        assert_eq!(order.user_id, 2);
    }

    #[test]
    fn test_high_value_order() {
        let order = fake_orders::high_value_order();
        assert_eq!(order.quantity, 1);
        assert_eq!(order.stock_symbol, "BRK.A");
    }

    #[test]
    fn test_order_with_custom_status() {
        let order = fake_orders::order_with_status("COMPLETED");
        assert_eq!(order.status, "COMPLETED");
    }
}
