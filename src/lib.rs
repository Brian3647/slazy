#![forbid(missing_docs)]
#![forbid(unsafe_code)]

//! A simple, synchronous signal library for Rust.
//! Signals are a way to react to changes in a value.
//!
//! # Example
//!
//! ```rust
//! use sssignals::Signal;
//!
//! let mut signal = Signal::new(0);
//!
//! signal.on_change(|new, old| {
//!    assert_eq!(new, &1);
//!    assert_eq!(old, &0);
//! });
//!
//! signal.set(1);

use std::fmt::Display;

/// Type for the function executed when setting a new value.
pub type Handler<T> = Box<dyn FnMut(&T, &T)>;

/// A generic signal that holds a value of type `T` and allows you to react to changes in the value.
pub struct Signal<T> {
	/// The current value of the signal.
	value: T,
	/// The callback function that is called whenever the value changes.
	on_change: Option<Handler<T>>,
}

impl<T> Signal<T> {
	/// Creates a new `Signal` instance with the initial value.
	///
	/// # Returns
	///
	/// A new `Signal` instance with the provided initial value.
	pub fn new(default: T) -> Self {
		Self {
			value: default,
			on_change: None,
		}
	}

	/// Gets a reference to the value of the signal.
	/// This is useful if you want to read the value without changing it.
	/// If you want to change the value, use `set` instead.
	pub fn get(&self) -> &T {
		&self.value
	}

	/// Sets the value of the signal.
	/// If an `on_change` callback function is set, it will be called with the new and old values.
	///
	/// # Example
	///
	/// ```rust
	/// # use sssignals::Signal;
	/// let mut signal = Signal::new(42);
	/// signal.set(43);
	/// ```
	pub fn set(&mut self, value: T) {
		let old = &self.value;
		if let Some(f) = &mut self.on_change {
			f(&value, old);
		}

		self.value = value;
	}

	/// Sets the `on_change` callback function, which will be called whenever the value changes.
	///
	/// # Example
	///
	/// ```rust
	/// # use sssignals::Signal;
	/// let mut signal = Signal::new(42);
	///
	/// signal.on_change(|new, old| {
	///     println!("Value changed from {} to {}", old, new);
	/// });
	///
	/// signal.set(43);
	/// ```
	pub fn on_change<F>(&mut self, f: F)
	where
		F: FnMut(&T, &T) + 'static,
	{
		self.on_change = Some(Box::new(f));
	}

	/// Maps the current value of the signal to a new value using a provided mapping function.
	///
	/// # Returns
	///
	/// A new `Signal` instance containing the mapped value.
	///
	/// # Note
	///
	/// The `on_change` callback function will NOT be passed through the mapping,
	/// since the mapped value is different.
	///
	/// # Example
	///
	/// ```rust
	/// # use sssignals::Signal;
	/// let signal = Signal::new(42);
	/// let mapped_signal = signal.map(|value| value.to_string());
	/// ```
	pub fn map<U, F: Fn(&T) -> U>(&self, f: F) -> Signal<U> {
		Signal::new(f(&self.value))
	}
}

impl<T: Copy> Signal<T> {
	/// Gets a copy of the value of the signal.
	/// If you want to change the value, use `set` instead.
	pub fn value(&self) -> T {
		self.value
	}
}

impl<T: Display> Display for Signal<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Signal({})", self.value)
	}
}

use std::fmt::Debug;

impl<T: Debug> Debug for Signal<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Signal({:?})", self.value)
	}
}

impl<T: Default> Default for Signal<T> {
	fn default() -> Self {
		Self::new(T::default())
	}
}

#[cfg(test)]
mod tests {
	use super::Signal;

	#[test]
	fn test_signal_new() {
		let signal: Signal<i32> = Signal::new(42);
		assert_eq!(signal.value, 42);
	}

	#[test]
	fn test_signal_set() {
		let mut signal = Signal::new(42);
		signal.set(43);
		assert_eq!(signal.value, 43);
	}

	#[test]
	fn test_signal_on_change() {
		let mut signal = Signal::new(42);

		signal.on_change(move |new, old| {
			assert_eq!(*new, 43);
			assert_eq!(*old, 42);
		});

		signal.set(43);
	}

	#[test]
	fn test_signal_on_change_no_callback() {
		let mut signal = Signal::new(42);
		signal.set(43); // Should not panic or error when on_change is None.
	}

	#[test]
	fn test_signal_map() {
		let signal = Signal::new(42);
		let mapped_signal = signal.map(|value| value.to_string());
		assert_eq!(mapped_signal.value, "42");
	}

	#[test]
	fn test_signal_map_generic_types() {
		let signal = Signal::new("Hello");
		let mapped_signal = signal.map(|value| value.len());
		assert_eq!(mapped_signal.value, 5);
	}

	#[test]
	fn test_display_trait() {
		let signal = Signal::new(42);
		let display_str = format!("{}", signal);
		assert_eq!(display_str, "Signal(42)");
	}

	#[test]
	fn test_debug_trait() {
		let signal = Signal::new("Hello");
		let debug_str = format!("{:?}", signal);
		assert_eq!(debug_str, "Signal(\"Hello\")");
	}

	#[test]
	fn test_default_trait() {
		let default_signal: Signal<i32> = Default::default();
		assert_eq!(default_signal.value, 0);
	}
}
