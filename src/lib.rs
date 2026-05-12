/*!
Reusable object pooling extracted from regex-automata.
*/

#![no_std]
#![deny(missing_docs)]
#![allow(unused_doc_comments)]
#![warn(missing_debug_implementations)]

#[cfg(any(test, feature = "std"))]
extern crate std;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(not(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
)))]
compile_error!("not supported on non-{16,32,64}, please file an issue");

/// Shared utilities.
pub mod util;
