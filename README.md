# Unique ID Generator

Unique 64 bit ID generator inspired by [twitter snowflake](https://github.com/twitter-archive/snowflake)

By default, settings are:
* Machine ID - 8 bit (up to 255 machines)
* Timestamp subset - 41 bits ~69 years since Unix Epoch
* Remaining bits (15) - sequence number

If sequence number is overflowing, generator will wait in chunks of 100 microseconds until next millisecond.

Can be configured:
* Machine ID - up to 8 bit
* Timestamp subset - from 41 to 43 bits (up to 278 years since Unix Epoch)

## Usage

```rust
fn main() {
    let idgen = IDGen::new(128);
    let new_id: u64 = idgen::new_id();
}
```

Alternatively, it can be configured to have more bits for sequence number with less bits for machine ID:

```rust
fn main() {
    let idgen = IDGen::new_with_config(1, 1, 41);
    let new_id: u64 = idgen::new_id();
}
```

## Notes

* Performance is "ok" - on MacBook Air 2019 it generates ~3M unique ids per second in single-threaded mode (RefCell overhead for interior mutability/thread safety at least halves the performance)
* It is thread-safe
* Strictly speaking, it can be used with less than 41 bits for timestamp (as only last meaningful bits are taken into account)
* It is not published as crate yet
