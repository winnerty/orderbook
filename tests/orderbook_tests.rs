use orderbook::{Side, Order, OrderBook};

#[test]
fn test_buy_insert_front() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100.0, quantity: 1.0, side: Side::Buy });
    ob.proceed_order(Order { price: 105.0, quantity: 2.0, side: Side::Buy });

    assert_eq!(ob.bids, vec![(105.0, 2.0), (100.0, 1.0)]);
}

#[test]
fn test_buy_insert_end() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 100.0, quantity: 1.0, side: Side::Buy });
    ob.proceed_order(Order { price: 90.0, quantity: 2.0, side: Side::Buy });

    assert_eq!(ob.bids, vec![(100.0, 1.0), (90.0, 2.0)]);
}

#[test]
fn test_sell_insert_front() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 120.0, quantity: 5.0, side: Side::Sell });
    ob.proceed_order(Order { price: 110.0, quantity: 3.0, side: Side::Sell });

    assert_eq!(ob.asks, vec![(110.0, 3.0), (120.0, 5.0)]);
}

#[test]
fn test_sell_insert_end() {
    let mut ob: OrderBook = OrderBook::new();
    ob.proceed_order(Order { price: 80.0, quantity: 1.0, side: Side::Sell });
    ob.proceed_order(Order { price: 95.0, quantity: 3.0, side: Side::Sell });

    assert_eq!(ob.asks, vec![(80.0, 1.0), (95.0, 3.0)]);
}

#[test]
fn test_buy_does_not_affect_asks() {
    let mut ob: OrderBook = OrderBook::new();

    ob.proceed_order(Order { price: 100.0, quantity: 5.0, side: Side::Sell });
    ob.proceed_order(Order { price: 90.0, quantity: 3.0, side: Side::Buy });

    assert_eq!(ob.asks, vec![(100.0, 5.0)]);
    assert_eq!(ob.bids, vec![(90.0, 3.0)]);
}

#[test]
fn test_spread_basic() {
    let mut ob: OrderBook = OrderBook::new();

    ob.proceed_order(Order { price: 100.0, quantity: 1.0, side: Side::Buy });
    ob.proceed_order(Order { price: 120.0, quantity: 1.0, side: Side::Sell });

    assert_eq!(ob.spread(), Some(20.0));
}

#[test]
fn test_spread_no_bids() {
    let mut ob: OrderBook = OrderBook::new();

    ob.proceed_order(Order { price: 120.0, quantity: 1.0, side: Side::Sell });

    assert_eq!(ob.spread(), None);
}

#[test]
fn test_snapshot_larger_than_book() {
    let mut ob: OrderBook = OrderBook::new();

    ob.proceed_order(Order { price: 110.0, quantity: 1.0, side: Side::Buy });
    ob.proceed_order(Order { price: 105.0, quantity: 1.0, side: Side::Buy });

    let (bids, asks) = ob.snapshot(10);

    assert_eq!(bids, vec![(110.0, 1.0), (105.0, 1.0)]);
    assert!(asks.is_empty());
}