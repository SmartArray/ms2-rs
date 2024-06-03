## ms-rs ⏱️
> ms-rs is a Rust library that recreates the ms package from TypeScript for Rust. It helps you convert milliseconds to human-readable time strings and vice versa. If you need to work with time in your Rust project, ms-rs makes it easy.

### Features:
- ⏳ Convert time strings (like "2 days") to milliseconds.
- ⏱️ Convert milliseconds to time strings.
- ⏲️ Supports common time units like seconds, minutes, hours, and days.
- 👍 Easy-to-use API based on the ms package from TypeScript.

### Usage:
Add ms-rs to your Cargo.toml:

```toml
[dependencies]
ms-rs = "0.1.0"
```

Then use it in your code:

```rust
use ms_rs::ms;

fn main() {
    let ms = ms("2 days").unwrap();
    println!("{}", ms); // Outputs: 172800000

    let time_str = ms(172800000);
    println!("{}", time_str); // Outputs: "2 days"
}
```

### Contributing:
Contributions are welcome! Check the CONTRIBUTING.md file for more info.

### License:
This project is licensed under the MIT License. See the LICENSE file for details.
