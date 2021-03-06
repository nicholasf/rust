// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    let x = vec![1];
    let y = x;
    //~^ HELP use a `ref` binding as shown
    //~| SUGGESTION let ref y = x;
    x; //~ ERROR use of moved value
    //~^ HELP run `rustc --explain E0382` to see a detailed explanation

    let x = vec![1];
    let mut y = x;
    //~^ HELP use a `ref` binding as shown
    //~| SUGGESTION let ref mut y = x;
    x; //~ ERROR use of moved value
    //~^ HELP run `rustc --explain E0382` to see a detailed explanation

    let x = (Some(vec![1]), ());

    match x {
        (Some(y), ()) => {},
        //~^ HELP use a `ref` binding as shown
        //~| SUGGESTION (Some(ref y), ()) => {},
        _ => {},
    }
    x; //~ ERROR use of partially moved value
    //~^ HELP run `rustc --explain E0382` to see a detailed explanation
}
