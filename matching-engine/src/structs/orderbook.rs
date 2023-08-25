use std::collections::BTreeMap;

use rust_decimal::Decimal;

use super::order::LimitOrder;

pub struct Orderbook {
    pub asks: BTreeMap<AskKey, LimitOrder>,
    pub bids: BTreeMap<BidKey, LimitOrder>,
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct AskKey {
    pub price: Decimal,
    pub order_id: u64,
}

impl Ord for AskKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.price == other.price {
            self.order_id.cmp(&other.order_id)
        } else {
            self.price.cmp(&other.price)
        }
    }
}

impl PartialOrd for AskKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
pub struct BidKey {
    pub price: Decimal,
    pub order_id: u64,
}

impl Ord for BidKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.price == other.price {
            self.order_id.cmp(&other.order_id)
        } else {
            self.price.cmp(&other.price).reverse()
        }
    }
}

impl PartialOrd for BidKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
