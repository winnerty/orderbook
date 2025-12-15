use crate::order::{Order, Side};

pub struct OrderBook {
    pub bids: Vec<(u64, u64)>, //bids are ordered DESC price
    pub asks: Vec<(u64, u64)>, //asks are ordered ASC price
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: vec![],
            asks: vec![]
        }
    }

    pub fn snapshot(&self, amount: usize) -> (Vec<(u64, u64)>, Vec<(u64, u64)>) {
        let bid_depth: usize = amount.min(self.bids.len());
        let ask_depth: usize = amount.min(self.asks.len());
        
        (self.bids.iter().cloned().take(bid_depth).collect(), self.asks.iter().cloned().take(ask_depth).collect())
    }

    pub fn spread(&self) -> Option<u64> {
        if self.bids.is_empty() || self.asks.is_empty() {
            return None;
        }

        Some(self.asks[0].0 - self.bids[0].0)
    }

    pub fn proceed_order(&mut self, new_order: Order) {    
        match new_order.side {
            Side::Buy => {
                for i in 0..self.bids.len() {
                    if new_order.price > self.bids[i].0 {
                        self.bids.insert(i, (new_order.price, new_order.quantity));
                        return;
                    } else if new_order.price == self.bids[i].0 {
                        self.bids[i].1 += new_order.quantity;
                        return;
                    }
                }
                self.bids.push((new_order.price, new_order.quantity));
            }
            Side::Sell => {
                for i in 0..self.asks.len() {
                    if new_order.price < self.asks[i].0 {
                        self.asks.insert(i, (new_order.price, new_order.quantity));
                        return;
                    } else if new_order.price == self.asks[i].0 {
                        self.asks[i].1 += new_order.quantity;
                        return;
                    }
                }
                self.asks.push((new_order.price, new_order.quantity));
            }
        }
    }
}