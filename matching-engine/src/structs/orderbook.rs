use std::collections::BTreeMap;

use super::order::LimitOrder;

pub struct Orderbook {
    pub asks: BTreeMap<u64, LimitOrder>,
    pub bids: BTreeMap<u64, LimitOrder>,
}
