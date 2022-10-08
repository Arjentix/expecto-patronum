//! `expecto-patronum` crate provides you a way to cast a patronus if you are in troubles.
//!
//! Help is always given at Rust to those who deserve it

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::panic)]
#![allow(clippy::arithmetic_side_effects)]
#![allow(clippy::integer_arithmetic)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::{fmt::Debug, option::Option, result::Result};

// Importing patronus assets generated by build.rs
include!(concat!(env!("OUT_DIR"), "/assets.rs"));

/// Extension trait for [`Result`] and [`Option`] to provide `expecto_patronum()` method.
pub trait ExpectoPatronumExt {
    /// Type of success value.
    type Success;

    /// Returns the contained successful value or casts a patronus to deliver your panic message.
    /// Patronus depends on the `msg` you provide.
    ///
    /// Use it well.
    ///
    /// # Panics
    ///
    /// Panics if `self` does not contain successful value.
    fn expecto_patronum(self, msg: &str) -> Self::Success;
}

impl<T, E: Debug> ExpectoPatronumExt for Result<T, E> {
    type Success = T;

    #[inline]
    fn expecto_patronum(self, msg: &str) -> Self::Success {
        self.unwrap_or_else(|error| {
            let patronus = choose_patronus(msg);
            let panic_msg = construct_panic_msg(patronus, msg);
            panic!("{panic_msg}: {error:?}")
        })
    }
}

impl<T> ExpectoPatronumExt for Option<T> {
    type Success = T;

    #[inline]
    fn expecto_patronum(self, msg: &str) -> Self::Success {
        self.unwrap_or_else(|| {
            let patronus = choose_patronus(msg);
            let panic_msg = construct_panic_msg(patronus, msg);
            panic!("{panic_msg}")
        })
    }
}

/// Type os Patronus string.
type Patronus = &'static str;

/// Choose a patronus based on the provided `msg`.
fn choose_patronus(_msg: &str) -> Patronus {
    Patronus::default()
}

/// Construct a panic message concatenating provided `patronus` and `msg`.
fn construct_panic_msg(patronus: Patronus, base_msg: &str) -> String {
    // Most efficient way according to https://github.com/hoodie/concatenation_benchmarks-rs

    let mut new_msg = String::with_capacity(patronus.len() + base_msg.len() + 1);
    new_msg.push_str(patronus);
    new_msg.push('\n');
    new_msg.push_str(base_msg);
    new_msg
}
