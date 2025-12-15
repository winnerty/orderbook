use std::collections::BTreeMap;
use crate::order::{Order, Side};

pub struct OrderBook {
    pub bids: BTreeMap<u64, u64>,
    pub asks: BTreeMap<u64, u64>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn snapshot(&self, amount: usize) -> (Vec<(u64, u64)>, Vec<(u64, u64)>) {
        let bids: Vec<(u64, u64)> = self.bids.iter().rev().take(amount).map(|(p, q)| (*p, *q)).collect();
        let asks: Vec<(u64, u64)> = self.asks.iter().take(amount).map(|(p, q)| (*p, *q)).collect();

        (bids, asks)
    }

    pub fn spread(&self) -> Option<u64> {
        let best_ask: &u64 = self.asks.iter().next()?.0;
        let best_bid: &u64 = self.bids.iter().rev().next()?.0;

         Some(best_ask - best_bid)
    }

    pub fn proceed_order(&mut self, new_order: Order) {    
        match new_order.side {
            Side::Buy => {
                let entry: &mut u64 = self.bids.entry(new_order.price).or_insert(0);
                *entry += new_order.quantity;
            }
            Side::Sell => {
                let entry: &mut u64 = self.asks.entry(new_order.price).or_insert(0);
                *entry += new_order.quantity;
            }
        }
    }
}