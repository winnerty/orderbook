## Features

### Core Logic (modular structure)
The project is organized into three modules:

- **`order.rs`** — defines the core domain types (`Order` and `Side`).  
  Prices and quantities are represented using **integer ticks (`u64`)**, a common practice in financial systems to avoid floating-point precision issues.

- **`orderbook.rs`** — implements the main `OrderBook` logic:
  - `bids`: stored in **descending** price order  
  - `asks`: stored in **ascending** price order  
  - `proceed_order`: inserts new price levels or aggregates quantities at existing ones  
  - `spread`: returns the difference between the best ask and best bid  
  - `snapshot`: returns the top *N* levels on each side

  The book is backed by simple `Vec<(price, quantity)>` structures.  
  This keeps the implementation minimal, explicit, and easy to reason about for the scope of this assignment.

- **`lib.rs`** — exposes the public API by re-exporting the main types.

This modular layout keeps the codebase clear and maintainable, while the integer-based pricing ensures deterministic and precise behavior.

## Tests (`tests/`)
Integration tests verify:

- correct insertion at front, middle, and end of the book  
- quantity aggregation on existing price levels  
- proper bid/ask ordering  
- spread behavior in normal and edge cases  
- snapshot correctness for various depths  

These tests ensure consistent and predictable behavior.

---

## Possible Improvements

If expanded into a production-grade orderbook, future enhancements could include:

### Data Structures
- Replace `Vec` with `BTreeMap<u64, u64>` for **O(log n)** insertions and built-in sorting.

### Matching Logic
- Add trade matching, partial fills, and automatic removal of empty price levels.

### Performance
- Add benchmarks for insertion and snapshot operations.
