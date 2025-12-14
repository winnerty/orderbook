use orderbook::{Side, Order, OrderBook};

#[test]
fn test_buy_insert() {
    let mut ob = OrderBook::new();
    ob.proceed_order(Order { price: 100.0, quantity: 1.0, side: Side::Buy });
    ob.proceed_order(Order { price: 105.0, quantity: 2.0, side: Side::Buy });

    assert_eq!(ob.bids, vec![(105.0, 2.0), (100.0, 1.0)]);
}