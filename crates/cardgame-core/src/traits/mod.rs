//! Core traits for the Card Game AI Framework
//!
//! These traits define the interfaces that game implementations must provide.

mod advisor;
mod collection;
mod datasource;
mod piece;
mod recognition;
mod rules;
mod state;

pub use advisor::*;
pub use collection::*;
pub use datasource::*;
pub use piece::*;
pub use recognition::*;
pub use rules::*;
pub use state::*;
