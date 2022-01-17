# human-duration

![Crates.io](https://img.shields.io/crates/d/human-duration)
![Crates.io](https://img.shields.io/crates/l/human-duration)
![docs.rs](https://img.shields.io/docsrs/human-duration)

human-duration converts a `std::time::Duration` to a human readable string.

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

## License

MIT