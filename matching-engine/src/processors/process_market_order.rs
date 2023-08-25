use std::time::{SystemTime, UNIX_EPOCH};

use rust_decimal::Decimal;
use schema::generated::{
    order_requests::{OrderSide, PlaceMarketOrderRequest},
    request_response::{FailureReason, Fill, RequestOutput, RequestResponse, Status},
};

use crate::structs::engine::MatchingEngine;

impl MatchingEngine {
    pub fn process_market_order(
        &mut self,
        place_market_order_request: PlaceMarketOrderRequest,
    ) -> RequestResponse {
        {
            let mut remaining_size: Decimal =
                place_market_order_request.size.clone().unwrap().into();
            let max_price: Decimal = place_market_order_request.max_price.clone().unwrap().into();

            match place_market_order_request.order_side() {
                OrderSide::Ask => {
                    while let Some(next_order) = self.orderbook.bids.values().next() {
                        if next_order.price.gt(&max_price) {
                            break;
                        }

                        remaining_size -= next_order.remaining_size;

                        if remaining_size.le(&Decimal::ZERO) {
                            break;
                        }
                    }
                }
                OrderSide::Bid => {
                    while let Some(next_order) = self.orderbook.asks.values().next() {
                        if next_order.price.lt(&max_price) {
                            break;
                        }

                        remaining_size -= next_order.remaining_size;

                        if remaining_size.le(&Decimal::ZERO) {
                            break;
                        }
                    }
                }
            };

            if remaining_size.gt(&Decimal::ZERO) {
                return RequestResponse {
                    request_id: place_market_order_request.request_id,
                    status: Status::Failure as i32,
                    failure_reason: Some(FailureReason::NotEnoughLiquidity as i32),
                    output: None,
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64,
                };
            }
        }

        let mut fills = Vec::new();
        let mut taker_fee_amount = Decimal::ZERO;
        let mut remaining_size: Decimal = place_market_order_request.size.clone().unwrap().into();
        let taker_fee: Decimal = place_market_order_request.taker_fee.clone().unwrap().into();
        match place_market_order_request.order_side() {
            OrderSide::Ask => {
                while let Some(next_order_key) = self.orderbook.bids.keys().next().cloned() {
                    if remaining_size.ge(&self
                        .orderbook
                        .bids
                        .get(&next_order_key)
                        .unwrap()
                        .remaining_size)
                    {
                        let next_order = self.orderbook.bids.remove(&next_order_key).unwrap();
                        let filled_amount = next_order.remaining_size;
                        let maker_fee_amount = filled_amount * next_order.maker_fee;
                        taker_fee_amount += filled_amount * taker_fee;
                        remaining_size -= filled_amount;

                        let fill = Fill {
                            filled_order_id: next_order.order_id,
                            user_id: next_order.user_id,
                            filled_amount: Some(filled_amount.into()),
                            maker_fee_amount: Some(maker_fee_amount.into()),
                            filled_in_full: true,
                        };
                        fills.push(fill);
                    } else {
                        let next_order = self.orderbook.bids.get_mut(&next_order_key).unwrap();
                        let filled_amount = remaining_size;
                        let maker_fee_amount = filled_amount * next_order.maker_fee;
                        taker_fee_amount += filled_amount * taker_fee;
                        next_order.remaining_size -= filled_amount;

                        let fill = Fill {
                            filled_order_id: next_order.order_id,
                            user_id: next_order.user_id,
                            filled_amount: Some(filled_amount.into()),
                            maker_fee_amount: Some(maker_fee_amount.into()),
                            filled_in_full: false,
                        };
                        fills.push(fill);
                    }
                }
            }
            OrderSide::Bid => {
                while let Some(next_order_key) = self.orderbook.asks.keys().next().cloned() {
                    if remaining_size.ge(&self
                        .orderbook
                        .asks
                        .get(&next_order_key)
                        .unwrap()
                        .remaining_size)
                    {
                        let next_order = self.orderbook.asks.remove(&next_order_key).unwrap();
                        let filled_amount = next_order.remaining_size;
                        let maker_fee_amount = filled_amount * next_order.maker_fee;
                        taker_fee_amount += filled_amount * taker_fee;
                        remaining_size -= filled_amount;

                        let fill = Fill {
                            filled_order_id: next_order.order_id,
                            user_id: next_order.user_id,
                            filled_amount: Some(filled_amount.into()),
                            maker_fee_amount: Some(maker_fee_amount.into()),
                            filled_in_full: true,
                        };
                        fills.push(fill);
                    } else {
                        let next_order = self.orderbook.asks.get_mut(&next_order_key).unwrap();
                        let filled_amount = remaining_size;
                        let maker_fee_amount = filled_amount * next_order.maker_fee;
                        taker_fee_amount += filled_amount * taker_fee;
                        next_order.remaining_size -= filled_amount;

                        let fill = Fill {
                            filled_order_id: next_order.order_id,
                            user_id: next_order.user_id,
                            filled_amount: Some(filled_amount.into()),
                            maker_fee_amount: Some(maker_fee_amount.into()),
                            filled_in_full: false,
                        };
                        fills.push(fill);
                    }
                }
            }
        }

        RequestResponse {
            request_id: place_market_order_request.request_id,
            status: Status::Success as i32,
            failure_reason: None,
            output: Some(RequestOutput {
                order_id: None,
                user_id: 0,
                fills,
                taker_fee_amount: Some(taker_fee_amount.into()),
            }),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }
}
