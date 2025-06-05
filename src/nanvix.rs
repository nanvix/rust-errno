//! Implementation of `errno` functionality for RustyHermit.
//!
//! Nanvix does not have a traditional `errno` implementation because it is pure Rust and does not
//! depend on a C library at all. For that reason, we provide only a placeholder implementation
//! that returns a generic error message. This allows the `errno` crate to compile without
//! issues, but it does not provide any real error handling functionality.

// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::Errno;

pub fn with_description<F, T>(_err: Errno, callback: F) -> T
where
    F: FnOnce(Result<&str, Errno>) -> T,
{
    callback(Ok("unknown error"))
}

pub const STRERROR_NAME: &str = "strerror_r";

pub fn errno() -> Errno {
    Errno(0)
}

pub fn set_errno(_: Errno) {}
