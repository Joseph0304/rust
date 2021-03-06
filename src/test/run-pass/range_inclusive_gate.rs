// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused_comparisons)]
// Test that you only need the syntax gate if you don't mention the structs.
// (Obsoleted since both features are stabilized)

fn main() {
    let mut count = 0;
    for i in 0_usize..=10 {
        assert!(i >= 0 && i <= 10);
        count += i;
    }
    assert_eq!(count, 55);
}

