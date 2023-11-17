# Spinners
An SRTB file parsing crate written in Rust.

This crate is work in progress, as there are many tiny changes in this format between game updates that need to be accounted for.

## Usage

```rust
use spinners::load_srtb_from_str;

let file_contents: &str;  // Load the file by whatever means
let srtb = load_srtb_from_str(file_contents)?;

// You can then read data from the srtb file
println!("{}", srtb.track_info.charter);
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
