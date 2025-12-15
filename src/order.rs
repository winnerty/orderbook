pub enum Side {
    Buy,
    Sell,
}

pub struct Order {
    pub price: u64,
    pub quantity: u64,
    pub side: Side,
}