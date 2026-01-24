// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use euler_lib::{read_names_csv, data_file, get_name_score};

fn main() {

	let fpath = data_file("problem22", "names.txt");	
	let mut names = read_names_csv(fpath.display().to_string());
    names.sort();
    
    let mut idx: usize = 1;
    let mut score: usize;
    let mut scores: usize = 0;
    
    for name in names.iter() {
        score = get_name_score(name)*idx;
//        println!("Score for {} is {}", *name, score);
        scores += score;
        idx += 1;
    }

    println!("{}", scores);
}
