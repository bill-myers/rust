// -*- rust -*-
// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub fn main() { task::spawn(|| child((10, 20, 30, 40, 50, 60, 70, 80, 90)) ); }

fn child(args: (int, int, int, int, int, int, int, int, int)) {
    let (i1, i2, i3, i4, i5, i6, i7, i8, i9) = args;
    error!(i1);
    error!(i2);
    error!(i3);
    error!(i4);
    error!(i5);
    error!(i6);
    error!(i7);
    error!(i8);
    error!(i9);
    assert!((i1 == 10));
    assert!((i2 == 20));
    assert!((i3 == 30));
    assert!((i4 == 40));
    assert!((i5 == 50));
    assert!((i6 == 60));
    assert!((i7 == 70));
    assert!((i8 == 80));
    assert!((i9 == 90));
}
