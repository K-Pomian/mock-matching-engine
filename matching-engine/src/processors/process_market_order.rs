use schema::generated::order_requests::{OrderSide, PlaceMarketOrderRequest};

use crate::structs::engine::MatchingEngine;

impl MatchingEngine {
    pub fn process_market_order(&mut self, place_market_order_request: PlaceMarketOrderRequest) {
        let existing_orders = if place_market_order_request.order_side() == OrderSide::Ask {
            self.orderbook.bids
        } else {
            self.orderbook.asks
        };

        let mut remaining_size_temp = place_market_order_request.size;
        while let Some(limit_order) = existing_orders.values().next() {}
    }
}
