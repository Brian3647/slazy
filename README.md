<div align="center">

# SLazy 💄

![License](https://img.shields.io/github/license/Brian3647/slazy)
![GitHub issues](https://img.shields.io/github/issues/Brian3647/slazy)
![Build status](https://img.shields.io/github/actions/workflow/status/Brian3647/slazy/rust.yml)

A simple, small, no-std, macro-based lazy static library for Rust.

[\[Request a feature/report a bug\]](https://github.com/Brian3647/slazy)

</div>

## Installation

`cargo add slazy` or by adding the following to your `Cargo.toml`:

```toml
[dependencies]
slazy = "*"
```

## Examples

```rust
use slazy::slazy;

slazy! {
    pub FOO: u32 = {
        println!("Evaluating FOO");
        42
    };

    BAR: u32 = 1337;
}

println!("FOO: {}", *FOO); // Evaluates FOO
println!("{}", *FOO); // Gets the value of FOO without evaluating it again
println!("{}", *BAR); // Evaluates BAR
```

## Thread safety

> [!WARNING]
> If you want to use SLazy in a multi-threaded environment, you should initialize
> the lazy statics before spawning any threads. This is because the lazy statics
> might not be thread safe in certain scenarios due to data races.

### Example

```rust
use slazy::{slazy, init};

slazy! {
    pub FOO: u32 = {
        println!("Evaluating FOO");
        42
    };
}

init!(FOO); // or `_ = *FOO;`

std::thread::spawn(|| {
    println!("{}", *FOO); // Safe to use FOO in this thread
});
```

## License

This project is licensed under the [MIT license](LICENSE).
