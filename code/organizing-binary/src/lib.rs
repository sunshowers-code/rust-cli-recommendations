//! Help text for my-app.
//!
//! Can contain information about what the binary does, command-line options,
//! configuration, etc.

mod command;
// ... other modules

// This is the only export from the crate. It is marked hidden and
// is not part of the public API.
#[doc(hidden)]
pub use command::MyApp;
