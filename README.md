## Features

### Core Logic (`src/lib.rs`)
This library implements a simple price-priority orderbook:

- `bids`: stored in **descending** price order  
- `asks`: stored in **ascending** price order  
- `proceed_order`: inserts new levels or aggregates quantity at existing prices  
- `spread`: returns the best ask minus the best bid  
- `snapshot`: returns the top *N* levels on each side  

The orderbook uses **integer tick-based pricing (`u64`)**, a common practice in financial systems, to avoid floating-point precision issues. Quantities are also represented as `u64` for the same reason.

Data is kept in minimal `Vec<(price, quantity)>` structures for clarity.

---

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
- Replace `Vec` with `BTreeMap<f64, f64>` for **O(log n)** insertions and built-in sorting.

### Matching Logic
- Add trade matching, partial fills, and automatic removal of empty price levels.

### Performance
- Add benchmarks for insertion and snapshot operations.
