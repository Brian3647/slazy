#![no_std]
#![doc = include_str!("../README.md")]

/// The macro to create a lazy static.
///
/// # Usage
///
/// ```
/// use slazy::slazy;
///
/// slazy! {
///     pub public_var: String = String::from("Hello, world!");
///     non_public_example: u32 = 42;
/// }
/// ```
#[macro_export]
macro_rules! slazy {
    (pub $name:ident: $type:ty = $val:expr; $($rest:tt)*) => {
        pub struct $name;
        $crate::__internal_inner_slazy!($name, $type, $val);
        slazy!($($rest)*);
    };
    ($name:ident: $type:ty = $val:expr; $($rest:tt)*) => {
        struct $name;
        $crate::__internal_inner_slazy!($name, $type, $val);
        slazy!($($rest)*);
    };
    (pub $name:ident: $type:ty = $val:expr) => {
        pub struct $name;
        $crate::__internal_inner_slazy!($name, $type, $val);
    };
    ($name:ident: $type:ty = $val:expr) => {
        struct $name;
        $crate::__internal_inner_slazy!($name, $type, $val);
    };
	() => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __internal_inner_slazy {
	($name:ident, $type:ty, $val:expr) => {
		impl ::core::ops::Deref for $name {
			type Target = $type;

			#[inline(always)]
			fn deref(&self) -> &'static Self::Target {
				static mut VAL: Option<$type> = None;
				unsafe { VAL.get_or_insert_with(|| $val) }
			}
		}
	};
}

/// This macro is used to initialize lazy statics, which
/// is required for them to be safe in multithreaded environments.
///
/// Equivalent to `_ = *(your lazy static);`;
#[macro_export]
macro_rules! init {
	($($lazy:ident)*) => {
		$(
			_ = *$lazy;
		)*
	};
}
