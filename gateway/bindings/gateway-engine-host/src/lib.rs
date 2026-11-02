//! The four Engine calls, in a crate of its own so that the program that makes them links neither
//! the host nor anything generated (ENGINE-HOST.md).

pub mod client;
pub mod engine;
pub mod service;
