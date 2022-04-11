# human-duration

[![Crates.io](https://img.shields.io/crates/d/human-duration)](https://crates.io/crates/human-duration)
![Crates.io](https://img.shields.io/crates/l/human-duration)
[![docs.rs](https://img.shields.io/docsrs/human-duration)](https://docs.rs/human-duration)

human-duration converts a `std::time::Duration` to a human readable string.

[Documentation](https://docs.rs/human-duration) | [Github](https://github.com/cars10/human-duration) | [Crates.io](https://crates.io/crates/human-duration)

## Examples

```rust
use human_duration::human_duration;

let duration = std::time::Duration::new(120, 30_000_000);
assert_eq!(human_duration(&duration), "2m 0s 30ms");

let duration = std::time::Duration::new(9000, 0);
assert_eq!(human_duration(&duration), "2h 30m 0s 0ms");
```

## Usage

Run 

```bash
cargo add human_duration
```

or manually add it to your `Cargo.toml`:

```toml
[dependencies]
human_duration = "0.1"
```

Then use the `human_duration` function:

```rust
use human_duration::human_duration;

// somewhere in your code:
let duration = std::time::Duration::new(120, 0);
println!(human_duration(&duration));
```

## Comparing with other crates

| Crate          | Duration::new(0, 0) | Duration::new(0, 5_000_000) | Duration::new(5, 0) | Duration::new(34_536_000, 0) | Duration::new(86_400, 337_000_000) |
|----------------|---------------------|-----------------------------|---------------------|------------------------------|------------------------------------|
| human-duration | 0ms                 | 5ms                         | 5s 0ms              | 1y 1mon 4d 7h 20m 0s 0ms     | 1d 0h 0m 0s 337ms                  |
| [humantime](https://crates.io/crates/humantime)      | 0s                  | 5ms                         | 5s                  | 1year 1month 4days 46m 24s   | 1day 337ms                         |
|[ time-humanize](https://crates.io/crates/time-humanize)  | now                 | now                         | now                 | in a year                    | in a day                           |

## License

MIT