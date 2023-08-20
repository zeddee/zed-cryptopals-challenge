//! # Zed is working through Cryptopals Challenge
//!
//! Zed is working through <https://cryptopals.com/> to learn how to actually use rust.
//!
//! - Challenges are solved by writing and running tests in relevant modules.
//! They're not solved or addressed directly.
//! For example, you can't find a _crate::challenges::set1::challenge3_ module that directly addresses <https://cryptopals.com/sets/1/challenges/3>.
//! - That's because the Cryptopals challenges build upon each other to form a comprehensive cryptographic toolkit.
//! - Except: _Some_ challenges that result in one-off code _are_ addressed direcctly in [crate::challenges].
//!
//! ## References
//!
//! - https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1
//! - [RFC 4648](https://www.rfc-editor.org/rfc/rfc4648#section-3.3)

/// Implementation of various encryption and decryption tasks
pub mod crack;

/// Encoding formats implemented to provide encode and decode utilities.
pub mod codec;

/// Shared utilities for this crate.
pub mod utils;

/// One-off implementations for specific Cryptopals challenges.
pub mod challenges;
