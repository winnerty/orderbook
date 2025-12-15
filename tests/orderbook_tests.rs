use orderbook::{Order, OrderBook, Side};

#[test]
fn test_new_orderbook() {
    let ob: OrderBook = OrderBook::new();
    
    assert!(ob.bids.is_empty(), "Bids should be empty");
    assert!(ob.asks.is_empty(), "Asks should be empty");
}

#[test]
fn test_add_single_buy_order() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100, quantity: 10, side: Side::Buy });
    
    assert_eq!(ob.bids.get(&100), Some(&10), "Single buy order should be in bids");
    assert!(ob.asks.is_empty(), "Asks should remain empty");
}

#[test]
fn test_add_single_sell_order() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 105, quantity: 5, side: Side::Sell });
    
    assert_eq!(ob.asks.get(&105), Some(&5), "Single sell order should be in asks");
    assert!(ob.bids.is_empty(), "Bids should remain empty");
}

#[test]
fn test_add_multiple_buy_orders_descending() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100, quantity: 10, side: Side::Buy });
    ob.proceed_order(Order { price: 105, quantity: 20, side: Side::Buy });
    ob.proceed_order(Order { price: 95, quantity: 15, side: Side::Buy });
    
    assert_eq!(ob.bids.len(), 3, "Should have 3 different bid prices");
    assert_eq!(ob.bids.get(&105), Some(&20), "Best bid should be 105");
    assert_eq!(ob.bids.get(&100), Some(&10), "Mid bid should be 100");
    assert_eq!(ob.bids.get(&95), Some(&15), "Lowest bid should be 95");
}

#[test]
fn test_add_multiple_sell_orders_ascending() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 110, quantity: 5, side: Side::Sell });
    ob.proceed_order(Order { price: 105, quantity: 10, side: Side::Sell });
    ob.proceed_order(Order { price: 115, quantity: 8, side: Side::Sell });
    
    assert_eq!(ob.asks.len(), 3, "Should have 3 different ask prices");
    assert_eq!(ob.asks.get(&105), Some(&10), "Best ask should be 105");
    assert_eq!(ob.asks.get(&110), Some(&5), "Mid ask should be 110");
    assert_eq!(ob.asks.get(&115), Some(&8), "Highest ask should be 115");
}

#[test]
fn test_buy_orders_same_price() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100, quantity: 10, side: Side::Buy });
    ob.proceed_order(Order { price: 100, quantity: 5, side: Side::Buy });
    
    assert_eq!(ob.bids.len(), 1, "Should have only 1 price level");
    assert_eq!(ob.bids.get(&100), Some(&15), "Quantities should be summed for same price");
}

#[test]
fn test_sell_orders_same_price() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 105, quantity: 10, side: Side::Sell });
    ob.proceed_order(Order { price: 105, quantity: 5, side: Side::Sell });
    
    assert_eq!(ob.asks.len(), 1, "Should have only 1 price level");
    assert_eq!(ob.asks.get(&105), Some(&15), "Quantities should be summed for same price");
}

#[test]
fn test_buy_order_insertion() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100, quantity: 10, side: Side::Buy });
    ob.proceed_order(Order { price: 110, quantity: 20, side: Side::Buy });
    ob.proceed_order(Order { price: 105, quantity: 15, side: Side::Buy });
    
    assert_eq!(ob.bids.len(), 3, "Should have 3 price levels");
    assert_eq!(ob.bids.get(&110), Some(&20), "Highest bid price");
    assert_eq!(ob.bids.get(&105), Some(&15), "Mid bid price");
    assert_eq!(ob.bids.get(&100), Some(&10), "Lowest bid price");
}

#[test]
fn test_sell_order_insertion() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 110, quantity: 10, side: Side::Sell });
    ob.proceed_order(Order { price: 100, quantity: 20, side: Side::Sell });
    ob.proceed_order(Order { price: 105, quantity: 15, side: Side::Sell });
    
    assert_eq!(ob.asks.len(), 3, "Should have 3 price levels");
    assert_eq!(ob.asks.get(&100), Some(&20), "Best ask price");
    assert_eq!(ob.asks.get(&105), Some(&15), "Mid ask price");
    assert_eq!(ob.asks.get(&110), Some(&10), "Highest ask price");
}

#[test]
fn test_spread_with_orders() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100, quantity: 10, side: Side::Buy });
    ob.proceed_order(Order { price: 105, quantity: 5, side: Side::Sell });
    
    assert_eq!(ob.spread(), Some(5), "Spread should be ask - bid = 105 - 100 = 5");
}

#[test]
fn test_spread_empty_bids() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 105, quantity: 5, side: Side::Sell });
    
    assert_eq!(ob.spread(), None, "Spread should be None if no bids");
}

#[test]
fn test_spread_empty_asks() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100, quantity: 10, side: Side::Buy });
    
    assert_eq!(ob.spread(), None, "Spread should be None if no asks");
}

#[test]
fn test_snapshot_full() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100, quantity: 10, side: Side::Buy });
    ob.proceed_order(Order { price: 105, quantity: 5, side: Side::Sell });
    
    let (bids, asks) = ob.snapshot(10);
    
    assert_eq!(bids, vec![(100, 10)], "Full snapshot bids in DESC order");
    assert_eq!(asks, vec![(105, 5)], "Full snapshot asks in ASC order");
}

#[test]
fn test_snapshot_partial() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100, quantity: 10, side: Side::Buy });
    ob.proceed_order(Order { price: 95, quantity: 15, side: Side::Buy });
    ob.proceed_order(Order { price: 105, quantity: 5, side: Side::Sell });
    ob.proceed_order(Order { price: 110, quantity: 8, side: Side::Sell });
    
    let (bids, asks) = ob.snapshot(1);
    
    assert_eq!(bids, vec![(100, 10)], "Partial snapshot bids top 1");
    assert_eq!(asks, vec![(105, 5)], "Partial snapshot asks top 1");
}

#[test]
fn test_snapshot_multiple_levels() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100, quantity: 10, side: Side::Buy });
    ob.proceed_order(Order { price: 102, quantity: 8, side: Side::Buy });
    ob.proceed_order(Order { price: 105, quantity: 5, side: Side::Sell });
    ob.proceed_order(Order { price: 107, quantity: 12, side: Side::Sell });
    
    let (bids, asks) = ob.snapshot(10);
    
    assert_eq!(bids, vec![(102, 8), (100, 10)], "Bids should be DESC order");
    assert_eq!(asks, vec![(105, 5), (107, 12)], "Asks should be ASC order");
}

#[test]
fn test_mixed_orders() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100, quantity: 10, side: Side::Buy });
    ob.proceed_order(Order { price: 105, quantity: 5, side: Side::Sell });
    ob.proceed_order(Order { price: 102, quantity: 8, side: Side::Buy });
    ob.proceed_order(Order { price: 107, quantity: 12, side: Side::Sell });
    
    assert_eq!(ob.bids.len(), 2, "Should have 2 bid price levels");
    assert_eq!(ob.asks.len(), 2, "Should have 2 ask price levels");
    
    let (bids, asks) = ob.snapshot(10);
    
    assert_eq!(bids, vec![(102, 8), (100, 10)], "Bids should be DESC order");
    assert_eq!(asks, vec![(105, 5), (107, 12)], "Asks should be ASC order");
}