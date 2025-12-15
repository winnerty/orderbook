use orderbook::{Side, Order, OrderBook};

#[test]
fn test_buy_insert_front() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100, quantity: 1, side: Side::Buy });
    ob.proceed_order(Order { price: 105, quantity: 2, side: Side::Buy });

    assert_eq!(ob.bids, vec![(105, 2), (100, 1)]);
}

#[test]
fn test_buy_insert_end() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100, quantity: 1, side: Side::Buy });
    ob.proceed_order(Order { price: 90, quantity: 2, side: Side::Buy });

    assert_eq!(ob.bids, vec![(100, 1), (90, 2)]);
}

#[test]
fn test_sell_insert_front() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 120, quantity: 5, side: Side::Sell });
    ob.proceed_order(Order { price: 110, quantity: 3, side: Side::Sell });

    assert_eq!(ob.asks, vec![(110, 3), (120, 5)]);
}

#[test]
fn test_sell_insert_end() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 80, quantity: 1, side: Side::Sell });
    ob.proceed_order(Order { price: 95, quantity: 3, side: Side::Sell });

    assert_eq!(ob.asks, vec![(80, 1), (95, 3)]);
}

#[test]
fn test_buy_does_not_affect_asks() {
    let mut ob: OrderBook = OrderBook::new();

    ob.proceed_order(Order { price: 100, quantity: 5, side: Side::Sell });
    ob.proceed_order(Order { price: 90, quantity: 3, side: Side::Buy });

    assert_eq!(ob.asks, vec![(100, 5)]);
    assert_eq!(ob.bids, vec![(90, 3)]);
}

#[test]
fn test_spread_basic() {
    let mut ob: OrderBook = OrderBook::new();

    ob.proceed_order(Order { price: 100, quantity: 1, side: Side::Buy });
    ob.proceed_order(Order { price: 120, quantity: 1, side: Side::Sell });

    assert_eq!(ob.spread(), Some(20));
}

#[test]
fn test_spread_no_bids() {
    let mut ob: OrderBook = OrderBook::new();

    ob.proceed_order(Order { price: 120, quantity: 1, side: Side::Sell });

    assert_eq!(ob.spread(), None);
}

#[test]
fn test_snapshot_larger_than_book() {
    let mut ob: OrderBook = OrderBook::new();

    ob.proceed_order(Order { price: 110, quantity: 1, side: Side::Buy });
    ob.proceed_order(Order { price: 105, quantity: 1, side: Side::Buy });

    let (bids, asks) = ob.snapshot(10);

    assert_eq!(bids, vec![(110, 1), (105, 1)]);
    assert!(asks.is_empty());
}