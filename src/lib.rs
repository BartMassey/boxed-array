//! This crate provides a macro that creates a boxed array
//! directly on the heap. It is almost always a good idea to
//! use a vector instead, but length-checking and efficiency
//! might conceivably make this a useful option.
//!
//! This crate is derived from code written by Redditor
//! /u/Lord_Zane and published on Reddit in [this
//! comment](https://www.reddit.com/r/rust/comments/hemjx0/boxnew_lies_data_is_created_on_the_stack_then/fvscmj9?utm_source=share&utm_medium=web2x).

/// Make a function with a given name and array size (must be `usize`) that
/// returns a boxed array of the given size constructed on the heap rather
/// than the stack. The boxed array creation function itself takes
/// a closure used to initialize the array: the initializer closure
/// takes an array index and returns a value.
///
/// The signature of a function created by `boxed_array_fn(f, 17)`
/// would thus be
///
/// ```ignore
/// fn f<T, F>(init: F) -> Box<[T; 17]>
///    where F: FnMut(usize) -> T
/// ```
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate boxed_array;
/// boxed_array_fn!(seq, 3);
/// assert_eq!(seq(|i| i), Box::new([0, 1, 2]));
///
/// boxed_array_fn!(seq_seq, 2);
/// assert_eq!(
///     seq_seq(|j| *seq(|i| i + j)),
///     Box::new([[0, 1, 2], [1, 2, 3]]),
/// );
/// ```
#[macro_export]
macro_rules! boxed_array_fn {
    ($name:ident, $size:literal) => {
        fn $name<T, F>(mut init: F) -> Box<[T; $size]>
        where
            F: FnMut(usize) -> T,
        {
            use std::mem::ManuallyDrop;

            // XXX This code should use
            // Vec::into_raw_parts() once that function is
            // stabilized.

            // Create a Vec of the same capacity as the
            // resulting Box<Array> At the end of this
            // function we will make the memory be owned by
            // the box, so this Vec must not be dropped.
            let mut array: ManuallyDrop<Vec<T>> =
                ManuallyDrop::new(Vec::with_capacity($size));

            // Fill the memory with the initial data.
            let ptr = array.as_mut_ptr();
            for i in 0..$size {
                unsafe {
                    std::ptr::write::<T>(
                        ptr.offset(i as isize),
                        init(i),
                    )
                };
            }

            // Convert the memory taken from the Vec to a
            // Box<Array>. The box now owns the memory and
            // is in charge of freeing it.
            unsafe { Box::from_raw(ptr as *mut [T; $size]) }
        }
    };
}
