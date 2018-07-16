// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// aux-build:not-joint.rs

#![feature(use_extern_macros)]

extern crate not_joint as bar;
use bar::{tokens, nothing};

tokens![< -];

#[nothing]
a![< -];

#[nothing]
b!{< -}

#[nothing]
c!(< -);

#[nothing]
fn foo() {
    //! dox
    let x = 2 < - 3;
}

fn main() {}
