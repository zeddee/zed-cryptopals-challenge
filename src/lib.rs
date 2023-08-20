//! # Zed is working through Cryptopals Challenge
//!
//! Zed is working through <https://cryptopals.com/> to learn how to actually use rust.
//!
//! - Challenges are solved by writing and running tests in relevant modules.
//! They're not solved or addressed directly.
//! For example, you can't find a _crate::challenges::set1::challenge3_ module that directly addresses <https://cryptopals.com/sets/1/challenges/3>.
//! - That's because the Cryptopals challenges build upon each other to form a comprehensive cryptographic toolkit.
//! - Except: _Some_ challenges that result in one-off code _are_ addressed direcctly in [crate::challenges].

pub mod challenges;
pub mod codec;
pub mod crack;
pub mod utils;
