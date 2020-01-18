#![allow(unused_imports)]
#![deny(unsafe_code)]
/**
 * The Otto Eventbus module contains public interfaces for sending messages, constructing clients,
 * and constructing new backend eventbus implementations
 */
pub mod client;
pub mod msg;

trait Bus {}

#[cfg(test)]
mod tests {
    use super::*;
}
