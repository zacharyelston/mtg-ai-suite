//! Core traits for the Card Game AI Framework
//!
//! These traits define the interfaces that game implementations must provide.

mod piece;
mod collection;
mod state;
mod rules;
mod datasource;
mod advisor;
mod recognition;

pub use piece::*;
pub use collection::*;
pub use state::*;
pub use rules::*;
pub use datasource::*;
pub use advisor::*;
pub use recognition::*;
