use schema::generated::order_requests::OrderSide;

pub struct LimitOrder {
    pub order_id: u64,
    pub user_id: u64,
    pub price: u64,
    pub initial_size: u64,
    pub remaining_size: u64,
    pub side: OrderSide,
}
