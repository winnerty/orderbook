## Features

### Core Logic (modular structure)
The project is organized into three modules:

- **`order.rs`** — defines the core domain types (`Order` and `Side`).  
  Prices and quantities are represented using **integer ticks (`u64`)**, a common practice in financial systems to avoid floating-point precision issues.

- **`orderbook.rs`** — implements the main `OrderBook` logic with high-performance data structures:
  - **`bids`**: `BTreeMap<u64, u64>` for efficient O(log n) insertions, automatic sorting in ascending price order (reversed for DESC display)
  - **`asks`**: `BTreeMap<u64, u64>` for efficient O(log n) insertions, automatic sorting in ascending price order
  - `proceed_order`: inserts new price levels or aggregates quantities at existing ones with O(log n) time
  - `spread`: returns the difference between the best ask and best bid with O(1) access to extremes
  - `snapshot`: returns the top *N* levels on each side in correct order (bids DESC, asks ASC)

- **`lib.rs`** — exposes the public API by re-exporting the main types.

This modular, performance-optimized design ensures the codebase is clear, maintainable, and suitable for real-world order book applications.

---

## Tests (`tests/`)
Integration tests verify:

- correct creation and emptiness checks
- single and multiple order insertions
- quantity aggregation on identical price levels
- proper bid/ask sorting in DESC/ASC order respectively
- spread calculation in normal and edge cases (empty bids/asks)
- snapshot correctness for full depth and limited depth
- mixed buy/sell operations with proper ordering

The test suite ensures consistent, predictable, and reliable behavior across all scenarios.

---

## Possible Improvements

Future enhancements for production readiness:

### Error Handling
- Implement **custom `Error` type** using enum (instead of panics) for graceful failure handling
- Add **input validation** (price > 0, quantity > 0, etc.) returning `Result<T, OrderBookError>`
- Handle edge cases with proper error propagation

### Advanced Features
- **Order matching engine**: automatically match buy/sell orders at overlapping prices with trade execution
- **Order cancellation**: implement `cancel_order(order_id)` to remove orders by ID
- **Order IDs & History**: track individual orders with unique IDs and maintain execution history
- **Multiple trading pairs**: support multiple assets/symbols in a single data structure
- **Metrics**: add methods like `total_bid_volume()`, `total_ask_volume()`, `depth_levels()`, etc.

### Serialization & Integration
- **Serde support**: JSON export/import of OrderBook state
- **Network protocol**: gRPC or HTTP API for remote access
- **Database persistence**: store snapshots or full history
