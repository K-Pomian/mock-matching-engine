use rust_decimal::Decimal;
use schema::generated::order_requests::OrderSide;

pub struct LimitOrder {
    pub order_id: u64,
    pub user_id: u64,
    pub price: Decimal,
    pub initial_size: Decimal,
    pub remaining_size: Decimal,
    pub side: OrderSide,
}
