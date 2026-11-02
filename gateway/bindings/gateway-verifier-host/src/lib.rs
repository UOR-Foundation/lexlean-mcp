//! The processes of the verifications, in a crate of its own so that the program that starts them
//! links neither the compiler nor the host nor anything generated (VERIFIER.md).

pub mod client;
pub mod service;
pub mod sweep;
