// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is
// 9009 = 91 x 99.
// Find the largest palindrome made from the product of two 3-digit numbers.
use euler_lib::{is_palindrome, largest_palindrome};

fn main() {
    println!("{}",is_palindrome(100002));
    println!("{}", largest_palindrome(1000,100));
}
