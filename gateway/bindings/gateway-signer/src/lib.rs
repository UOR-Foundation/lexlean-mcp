//! The signer of the verification records, in a crate of its own so that the program that holds
//! the key links neither the compiler nor anything generated (SIGNING.md).

pub mod client;
pub mod service;
pub mod signer;
