# Magic Curves

![Crates.io](https://img.shields.io/crates/v/magic-curves.svg)
[![Docs.rs](https://docs.rs/magic-curves/badge.svg)](https://docs.rs/magic-curves)
![License](https://img.shields.io/crates/l/magic-curves.svg)


**Magic Curves** is a Rust library that provides a suite of tools for working with various bonding curves. This includes implementations of linear, quadratic, exponential, logarithmic, and sigmoid bonding curves.

## Description

Magic Curves is designed for applications requiring precise curve calculations, useful in finance, economics, and various scientific fields. The library supports both floating-point and fixed-point arithmetic for high precision in critical applications.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
magic-curves = "0.1.0"
```

Then, build your project with:

```sh
cargo build
```

## Usage

Here are examples of how to use the different bonding curves provided by the `magic_curves` library:

### Linear Bonding Curve

```rust
use magic_curves::core::linear::LinearBondingCurve;

fn main() {
    let linear = 500_000_000u128;
    let base = 1_000_000_000u128;
    let curve = LinearBondingCurve::new(linear, base);

    let price = curve.calculate_price(0);
    println!("Price at supply 0: {}", price);  // Outputs base price

    let price = curve.calculate_price(1);
    println!("Price at supply 1: {}", price);  // Outputs increased price
}
```

### Exponential Bonding Curve

```rust
use magic_curves::ExponentialBondingCurve;

fn main() {
    let curve = ExponentialBondingCurve::new(0.01, 0.02);
    let price = curve.calculate_price_lossy(100);
    println!("Price at supply 100: {}", price);
}
```

### Logarithmic Bonding Curve

```rust
use magic_curves::LogarithmicBondingCurve;

fn main() {
    let curve = LogarithmicBondingCurve::new(0.01, 0.02);
    let price = curve.calculate_price_lossy(100);
    println!("Price at supply 100: {}", price);
}
```

### Quadratic Bonding Curve

```rust
use magic_curves::QuadraticBondingCurve;

fn main() {
    let curve = QuadraticBondingCurve::new(10_000_000, 500_000_000, 1_000_000_000);
    let price = curve.calculate_price(1);
    println!("Price at supply 1: {}", price);  // Example price calculation
}
```

### Sigmoid Bonding Curve

```rust
use magic_curves::SigmoidBondingCurve;

fn main() {
    let curve = SigmoidBondingCurve::new(100.0, 0.01, 500);
    let price = curve.calculate_price_lossy(480);
    println!("Price at supply 480: {}", price);
}
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
