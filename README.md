# 🐍 SSSignals: Simple, Synchronous Reactive Signals for Rust

![License](https://img.shields.io/github/license/Brian3647/sssignals)
![GitHub issues](https://img.shields.io/github/issues/Brian3647/sssignals)
![Build status](https://img.shields.io/github/actions/workflow/status/Brian3647/sssignals/rust.yml)

SSSignals is a lightweight Rust library that provides simple, synchronous reactive signals. Signals are a powerful way to react to changes in values and design event-driven systems. With SSSignals, you can easily incorporate reactive programming into your Rust applications.

\[[Request a feature/Report a bug](https://github.com/Brian3647/sssignals/issues)\]

## Features

-   📡 **Signal Creation**: Create signals to hold values of any data type.
-   🔄 **Value Change Callbacks**: Register callbacks that are triggered when the signal's value changes.
-   🗺 **Value Transformation**: Map the signal's value to a new value using a provided mapping function.
-   🎯 **Trait Implementations**: Implements common Rust traits such as `Display`, `Debug`, and `Default`.

## Usage

```rust
use sssignals::Signal;

fn main() {
    let mut signal = Signal::new(42);

    signal.on_change(|new, old| {
        println!("Value changed from {} to {}", old, new);
    });

    signal.set(43); // Prints "Value changed from 42 to 43"

    println!("{}", signal); // Prints "Signal(43)"
}
```

## Installation

Run `cargo add sssignals` or add the following to your `Cargo.toml` file:

```toml
[dependencies]
sssignals = "*"
```

## Documentation

For detailed information on how to use SSSignals, please refer to [the official documentation](https://docs.rs/sssignals).

## Contributing

We welcome contributions from the open-source community. If you'd like to report a bug, request a feature, or contribute to the project, you can use the set templates.

## License

This project is licensed under the MIT License - see [the LICENSE file](/LICENSE) for details.
