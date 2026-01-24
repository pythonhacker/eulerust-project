// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use euler_lib::{day_of_month};
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

fn main() {
    // Uncomment commented lines if you want to run this to
    // read year and month as cmd-line arguments.    
	//    let args: Vec<String> = env::args().collect();
    
    let mut s_count: u32 = 0;
    // let year: u32 = args[1].parse::<u32>().unwrap();
    // let month: u32 = args[2].parse::<u32>().unwrap();
    let year = 2000;
    let month = 12;
    println!("Calculating till {} {}", year, month);

    for y in 1901..year+1 {
        for m in 1..13 {
            if y == year && m == month {break;}
            let dom: u32 = day_of_month(m, y);
            // println!("{}-{}-01 is on a {}",y,m, dom_to_dow(dom));
            if dom == 0 {
                s_count += 1;
            }
        }
    }

    println!("{}",s_count);
}
