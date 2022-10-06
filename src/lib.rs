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
#![allow(clippy::expect_used)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::{fmt::Debug, option::Option, result::Result};

/// Extension trait for [`Result`] and [`Option`] to provide `expecto_patronum()` method.
pub trait ExpectoPatronumExt {
    /// Type of success value.
    type Success;

    /// Call `expect()` with the provided `msg` and a beautiful corporeal patronus
    /// based on it.
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
        self.expect(msg)
    }
}

impl<T> ExpectoPatronumExt for Option<T> {
    type Success = T;

    #[inline]
    fn expecto_patronum(self, msg: &str) -> Self::Success {
        self.expect(msg)
    }
}
