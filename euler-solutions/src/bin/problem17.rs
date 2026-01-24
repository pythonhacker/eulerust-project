// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use euler_lib::number_to_word;

// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are
// 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words,
// how many letters would be used?

fn build_d1_hash_map() -> HashMap<i32, String> {
    
    let mut d1: HashMap<i32, String> = HashMap::new();

    d1.insert(1, "one".to_string());
    d1.insert(2, "two".to_string());
    d1.insert(3, "three".to_string());
    d1.insert(4, "four".to_string());
    d1.insert(5, "five".to_string());
    d1.insert(6, "six".to_string());
    d1.insert(7, "seven".to_string());
    d1.insert(8, "eight".to_string());
    d1.insert(9, "nine".to_string());
    d1.insert(10, "ten".to_string());
    d1.insert(11, "eleven".to_string());
    d1.insert(12, "twelve".to_string());
    d1.insert(13, "thirteen".to_string());
    d1.insert(14, "fourteen".to_string());
    d1.insert(15, "fifteen".to_string());
    d1.insert(16, "sixteen".to_string());
    d1.insert(17, "seventeen".to_string());
    d1.insert(18, "eighteen".to_string());
    d1.insert(19, "nineteen".to_string());

    return d1;
}

fn build_d2_hash_map() -> HashMap<i32, String> {

    let mut d2: HashMap<i32, String> = HashMap::new();

    d2.insert(10, "ten".to_string());
    d2.insert(20, "twenty".to_string());
    d2.insert(30, "thirty".to_string());
    d2.insert(40, "fourty".to_string());
    d2.insert(50, "fifty".to_string());
    d2.insert(60, "sixty".to_string());
    d2.insert(70, "seventy".to_string());
    d2.insert(80, "eighty".to_string());
    d2.insert(90, "ninety".to_string());

    return d2;
}

fn main() {

    let d1 = build_d1_hash_map();
    let d2 = build_d2_hash_map();
    let mut word: String;
    let mut word_counts: i32 = 0;

    // For Testing
    assert!(number_to_word(84, &d1, &d2) == "eighty four");
    assert!(number_to_word(1000, &d1, &d2) == "one thousand");
    assert!(number_to_word(130, &d1, &d2) == "one hundred and thirty");
    assert!(number_to_word(287, &d1, &d2) == "two hundred and eighty seven");

    for i in 1..1001 {
        word = number_to_word(i, &d1, &d2);
        let sum = word.split(" ").fold(0, |sum, x| sum + x.len());
        word_counts += sum as i32;
    }

    println!("{}", word_counts);
}

