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

/// Type for the function executed when setting a new value.
pub type Handler<T> = Box<dyn FnMut(&T, &T)>;

/// A generic signal that holds a value of type `T` and allows you to react to changes in the value.
pub struct Signal<T> {
	/// The current value of the signal.
	value: T,
	/// The callback function that is called whenever the value changes.
	handlers: Vec<Handler<T>>,
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
			handlers: vec![],
		}
	}

	/// Gets a reference to the value of the signal.
	/// This is useful if you want to read the value without changing it.
	/// If you want to change the value, use `set` instead.
	#[inline]
	pub fn value(&self) -> &T {
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
	pub fn set(&mut self, new: T) {
		let old_value = std::mem::replace(&mut self.value, new);
		for handler in self.handlers.iter_mut() {
			handler(&self.value, &old_value);
		}
	}

	/// Adds a handler, which will be called whenever the value changes.
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
	pub fn on_change<F>(&mut self, handler: F)
	where
		F: FnMut(&T, &T) + 'static,
	{
		self.handlers.push(Box::new(handler));
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
	#[inline]
	pub fn map<U, F: Fn(&T) -> U>(&self, f: F) -> Signal<U> {
		Signal::new(f(&self.value))
	}
}

impl<T: Copy> Signal<T> {
	/// Gets a copy of the value of the signal. Equivalent to `*signal.value()`.
	/// If you want to change the value, use `set` instead.
	#[inline]
	pub fn value_copy(&self) -> T {
		self.value
	}
}

use std::fmt;

impl<T: fmt::Display> fmt::Display for Signal<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Signal({})", self.value)
	}
}

impl<T: fmt::Debug> fmt::Debug for Signal<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
		assert_eq!(*signal.value(), 42);
	}

	#[test]
	fn test_signal_set() {
		let mut signal = Signal::new(42);
		signal.set(43);
		assert_eq!(*signal.value(), 43);
	}

	#[test]
	fn test_signal_on_change() {
		let mut signal: Signal<i32> = Signal::new(42);

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
		assert_eq!(*mapped_signal.value(), "42");
	}

	#[test]
	fn test_signal_map_generic_types() {
		let signal = Signal::new("Hello");
		let mapped_signal = signal.map(|value| value.len());
		assert_eq!(*mapped_signal.value(), 5);
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
		assert_eq!(*default_signal.value(), 0);
	}

	#[test]
	fn test_get() {
		let signal = Signal::new(42);
		assert_eq!(*signal.value(), 42);
		assert_eq!(signal.value_copy(), 42);
	}

	#[test]
	#[allow(unused_variables)]
	fn test_no_outside_changes() {
		let mut signal = Signal::new(42);
		let mut counter = 0;

		signal.on_change(move |new, _old| {
			counter += 1;
			assert_eq!(*new, 43);
		});

		signal.set(43);
		assert_eq!(counter, 0); // Moved data should not affect outside data.
	}

	use std::cell::RefCell;
	use std::rc::Rc;

	#[test]
	fn test_refcell_outside_changes() {
		let mut signal = Signal::new(42);
		let counter = Rc::new(RefCell::new(0));
		let counter_clone = Rc::clone(&counter);

		signal.on_change(move |new, _old| {
			*counter_clone.borrow_mut() += 1;
			assert_eq!(*new, 43);
		});

		signal.set(43);
		assert_eq!(*counter.borrow(), 1);
	}

	#[test]
	fn test_signal_on_change_with_different_values() {
		let mut signal = Signal::new(42);

		signal.on_change(move |new, old| {
			assert_eq!(*new - *old, 1);
		});

		signal.set(43);

		for _ in 0..10 {
			signal.set(*signal.value() + 1);
		}
	}

	#[test]
	fn test_signal_map_does_not_change_original() {
		let signal = Signal::new(42);
		let _mapped_signal = signal.map(|value| value.to_string());
		assert_eq!(*signal.value(), 42);
	}

	#[test]
	fn test_signal_value_copy() {
		let signal = Signal::new(42);
		let copied_value = signal.value_copy();
		assert_eq!(copied_value, 42);
	}

	#[test]
	fn test_signal_value_copy_does_not_change_original() {
		let signal = Signal::new(42);
		let _copied_value = signal.value_copy();
		assert_eq!(*signal.value(), 42);
	}
}
