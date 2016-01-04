// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -Z parse-only

pub mod break {
    //~^ ERROR expected identifier, found keyword `break`
}

fn main() {
    let x = (let y = 1);    //~ NOTE: `let` is not an expression and so it cannot be used this way
    //~^ ERROR expected identifier, found keyword `let`
    //~^^ ERROR expected one of `!`, `)`, `,`, `.`, `::`, `{`, or an operator, found `y`
}
