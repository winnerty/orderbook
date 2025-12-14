pub enum Side {
    Buy,
    Sell,
}

pub struct Order {
    pub price: f64,
    pub quantity: f64,
    pub side: Side
}

pub struct OrderBook {
    pub bids: Vec<(f64, f64)>, //bids are ordered DESC price
    pub asks: Vec<(f64, f64)>, //asks are ordered ASC price
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: vec![],
            asks: vec![]
        }
    }

    pub fn snapshot(&self, amount: usize) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
        let bid_depth: usize = amount.min(self.bids.len());
        let ask_depth: usize = amount.min(self.asks.len());
        
        (self.bids[0..bid_depth].to_vec(), self.asks[0..ask_depth].to_vec())
    }

    pub fn spread(&self) -> Option<f64> {
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