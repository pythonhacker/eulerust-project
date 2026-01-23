// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Shared utilities for eulerust-project
extern crate num_bigint;
extern crate num_traits;
extern crate itertools;

use num_bigint::{BigUint};
use num_traits::{pow};

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::str::FromStr;
use std::cmp::max;
use std::io::prelude::*;

use itertools::Itertools;
use std::iter::FromIterator;

// Helper for project root
pub fn project_root() -> PathBuf {
    // This gives euler-lib/ path
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Move up one level to workspace root
    manifest_dir
        .parent()
        .expect("Failed to get workspace root")
        .to_path_buf()
}

// Return full path to a given data file
pub fn data_file(problem: &str, filename: &str) -> PathBuf {
    project_root()
        .join("data")
        .join(problem)
        .join(filename)
}

// Ref: problem1
pub fn sum_of_multiples(limit:u32)->u32{
    (1..limit).filter(|x| x%3==0||x%5==0).sum()
}

// Ref: problem2
pub fn fibonacci_even_sum(limit:u32)->u32{
    let(mut a,mut b,mut sum)=(1,2,0);
    while b<=limit{
        if b%2==0{sum+=b}
        let c=a+b; a=b; b=c;
    }
    sum
}

// Ref: problem5
pub fn smallest_multiple(n:u64)->u64{(1..=n).fold(1,|acc,x|lcm(acc,x))}


// Common utility functions for eulerust-project
pub fn digit_to_vector(num:u64) -> Vec<u64> {

    let mut vdigits: Vec<u64> = vec![];
    let mut n = num;

    while n > 0 {
        vdigits.push(n%10);
        n = n / 10;
    }

    vdigits.reverse();
    
    return vdigits;
}

#[allow(dead_code)]
pub fn digit_to_vector_u128(num:u128) -> Vec<u128> {

    let mut vdigits: Vec<u128> = vec![];
    let mut n = num;

    while n > 0 {
        vdigits.push(n%10);
        n = n / 10;
    }

    vdigits.reverse();
    
    return vdigits;
}

// [1,2,3,4] => 1234
pub fn vector_to_digit(num_vec: &Vec<u64>) -> u64 {

    let mut num: u64 = 0;
    let mut i: u64 = 0;

    for n in num_vec.iter().rev() {
        num += 10u64.pow(i as u32)*n;
        i += 1;
    }

    return num;
}

#[allow(dead_code)]
pub fn vector_to_digit_u128(num_vec: &Vec<u128>) -> u128 {

    let mut num: u128 = 0;
    let mut i: u128 = 0;

    for n in num_vec.iter().rev() {
        num += 10u128.pow(i as u32)*n;
        i += 1;
    }

    return num;
}

#[allow(dead_code)]
pub fn digit_to_vector_u32(num:u32) -> Vec<u32> {

    let mut vdigits: Vec<u32> = vec![];
    let mut n = num;

    while n > 0 {
        vdigits.push(n%10);
        n = n / 10;
    }

    vdigits.reverse();

    return vdigits;
}

// [1,2,3,4] => 1234 (using pointers)
pub fn vectorp_to_digit_u32(num_vec: Vec<&u32>) -> u32 {

    let mut num: u32 = 0;
    let mut i: u32 = 0;

    for n in num_vec.iter().rev() {
        num += 10u32.pow(i)*(*n);
        i += 1;
    }

    return num;
}

#[allow(dead_code)]
pub fn get_prime_factors_u32(n: u32) -> Vec<u32> {

    let mut p: u32 = 3;
    let mut factors: Vec<u32> = vec![];

    if n % 2 == 0 {
        factors.push(2);
    }
    
    loop {
        if n % p == 0 && is_prime_u32(p) {
            factors.push(p);
        }

        if p >= n/2 { break; }
        p += 2;
    }
        
    return factors;
}

// Is n a prime ?
pub fn is_prime(n: u64)  -> bool {

    if n <= 1 {
        return false;
    }
            
    let mut flag = true;
    let mut item: u64 = 2;

    loop {
        if item*item > n { break; }
        if n % item == 0 {
            flag = false;
            break;
        }
        item += 1;
    }
        
    return flag;
}

#[allow(dead_code)]
pub fn is_prime_cached(n: u64, cache: &mut HashMap<u64, bool>)  -> bool {

    if n <= 1 {
        return false;
    }

    if cache.contains_key(&n) {
//        println!("From cache - {}", n);
        return *cache.get(&n).unwrap();
    }
            
    let mut flag = true;
    let mut item: u64 = 2;

    loop {
        if item*item > n { break; }
        if n % item == 0 {
            flag = false;
            break;
        }
        item += 1;
    }

    cache.insert(n, flag);
    
    return flag;
}

pub fn is_prime_u32(n: u32)  -> bool {

    let mut flag = true;
    let mut item: u32 = 2;

    loop {
        if item*item > n { break; }
        if n % item == 0 {
            flag = false;
            break;
        }
        item += 1;
    }
        
    return flag;
}

pub fn is_prime_i64(n: i64)  -> bool {

    let mut flag = true;
    let mut item: i64 = 2;

    loop {
        if item*item > n { break; }
        if n % item == 0 {
            flag = false;
            break;
        }
        item += 1;
    }
        
    return flag;
}

// Ref: problem 10 
pub fn is_prime_i64_ex(number: i64, primes: &Vec<i64>) -> bool {
    let mut flag = true;

    let x = (number as f64).sqrt() as i64 + 1;
        
    for item in primes.iter() {
        if *item >= x { break; }
        if number % *item == 0 {
            flag = false;
            break;
        }
    }

    return flag;
}


// Get divisors of a number - with caching using a hash map
pub fn get_divisors(n:i64, divisors: &mut HashMap<i64, i64>) -> i64 {

    if divisors.contains_key(&n) {
        return *divisors.get(&n).unwrap();
    }

    let mut cnt:i64 = 2;

    for x in 2..n/2+1 {
        if n %x == 0 {
            cnt += 1;
        }
    }

    divisors.insert(n, cnt);

	cnt
}

pub fn factorial(n: u32) -> BigUint {

    let mut prod: BigUint = 1u32.into();
    
    for x in 2..n+1 {
        prod *= x;
    }

    return prod;    
}

pub fn sum_digits_number(n: BigUint) -> u32 {

    let sum = n.to_string().chars().map(|d| d.to_digit(10)).fold(0, |sum, x| sum + x.unwrap());
    return sum;
}

// Return sum of divisors of a given number
pub fn get_divisor_sum(n: i64, divisors: &mut HashMap<i64, i64>) -> i64 {

    if divisors.contains_key(&n) {
        return *divisors.get(&n).unwrap();
    }

    let mut sum: i64 = 1;

    for x in 2..(n/2 + 1) {
        if n % x == 0 {
            sum += x;
        }
    }

    divisors.insert(n, sum);
    
    return sum;
}

// Vector<u32> to u32
#[allow(dead_code)]
pub fn vector_to_digit_u32(num_vec: Vec<u32>) -> u32 {

    let mut num: u32 = 0;
    let mut i: u32 = 0;

    for n in num_vec.iter().rev() {
        num += 10u32.pow(i)*(n);
        i += 1;
    }

    return num;
}

pub fn gcd(mut x:u64, mut y:u64)->u64 {

    let mut temp: u64;
    
    while y > 0 {
        temp = x;
        x = y;
        y = temp % y;
    }

    return x;
}

#[allow(dead_code)]
pub fn find_gcd(numbers: &Vec<u64>)->u64 {

    let x:u64 = numbers[0];
    let y:u64 = numbers[1];

    let num_v: Vec<u64> = numbers[2..].to_vec();

    let mut n_gcd:u64 = gcd(x, y);

    for num in num_v.iter() {
        n_gcd = gcd(n_gcd, *num);
    }

    return n_gcd;

}

pub fn lcm(x:u64, y:u64) -> u64 {

    let (num, den, n_gcd, n_lcm);
    
    if x>y {
        num = x;
        den = y;
    } else {
        num = y;
        den = x;
    }

    n_gcd = gcd(den, num);

    n_lcm = x*y/n_gcd;

    return n_lcm;
}

#[allow(dead_code)]
pub fn find_lcm(numbers: &Vec<u64>) -> u64 {


    let x:u64 = numbers[0];
    let y:u64 = numbers[1];

    let num_v: Vec<u64> = numbers[2..].to_vec();

    let mut n_lcm:u64 = lcm(x, y);


    for num in num_v.iter() {
        n_lcm = lcm(n_lcm, *num);
    }

    return n_lcm;    

}

#[allow(dead_code)]
pub fn gcd_u32(mut x: u32, mut y: u32)->u32 {

    let mut temp: u32;
    
    while y > 0 {
        temp = x;
        x = y;
        y = temp % y;
    }

    return x;
}

pub fn factorial_simple(n: u64) -> u64 {
    return (2..n+1).fold(1u64, |p,x| p*x);
}

// Ref: problem4
pub fn is_palindrome(number: i64) -> bool {

    let mut n = number;
    let mut rev = 0;
    let mut dig;
    
    while n > 0 {
        dig = n % 10;
        rev = rev * 10 + dig;
        n = n / 10;
    }

    return number == rev;
}

// Ref: problem4
pub fn largest_palindrome(number: i32, diff: i32) -> i32 {

    let (n1,n2) = (number-1, number-1);
    let mut largest = 0;
    let mut n;

    for x in (n1-diff..n1).rev() {
        for y in (n2-diff..n2).rev() {
            n = x*y;
            if is_palindrome(n as i64) && n>largest {
                largest = n;
                break;
            } 
        }
    }

    return largest;
}

// Ref: problem 6
pub fn sum_squares(n:i64)->i64 {

    let mut s:i64 = 0;

    for i in 1..n+1 {
        s += i*i;
    }

    return s;
}

// Ref: problem 6
pub fn square_sum(n:i64)->i64 {
    let mut s:i64 = 0;

    for i in 1..n+1 {
        s += i;
    }

    return s*s;
}

// Ref: problem 7
pub fn nth_prime(limit:i32)->i64 {

    let (mut n, mut count) = (2, 1);
    let mut primes:Vec<i64> = vec![2];
    let mut is_prime:bool;
    
    while count<limit {
        n += 1;
        is_prime = true;

        for x in primes.iter() {
            if n % x == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(n);
            count += 1;
        }
    }

    return n;
}


// Ref: problem 8
// Return largest product of 's' consecutive integers in the number n
pub fn largest_product(n: String, s: usize) {

    let digits_32:Vec<u32> = n.chars().map(|d| d.to_digit(10).unwrap()).collect();
    let digits:Vec<u64> = digits_32.iter().map(|d| *d as u64).collect();
    
    let mut largest:u64 = 1;
    let limit = digits.len() - 13;
    
    for i in 0..limit {
        let vs: &[u64] = &digits[i..i+s];      

        let prod: u64 = vs.iter().fold(1, |prod, x| prod*x);
        
        if prod > largest {
            largest = prod;
        }

    }

    println!("{}",largest);
}

// Ref: problem 9
pub fn pythagorean_triplet_finder(n:i32) -> (i32, i32, i32) {

    let mut triplet = (0,0,0);
    let mut z:i32;

	// FIXME: I couldn't find a way to optimize this
    for x in 1..n+1 {
        for y in 1..n+1 {
            if x+y>n { continue; }
            z = n - (x+y);
            if x*x + y*y == z*z  {
                triplet = (x,y,z);
                break;
            }
        }
    }
            
    return triplet;
}

// Ref: problem 10
// Sum of primes below given limit
pub fn prime_sum(limit: i64) -> i64 {

    let (mut n, mut sum)=(2,0);
    let mut primes:Vec<i64> = vec![2];
    while n<limit {
        if is_prime_i64_ex(n, &primes) {
            sum += n;
            primes.push(n);
        }

        n += 1;
    }
        
    return sum;
}

// Read lines from a file
// #[allow(dead_code)]        
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Ref: problem 11 - Grid structure
pub struct Grid {
    data: Vec<Vec<u32>>,
    width: usize,
    height: usize
}


impl Grid {

	pub fn new(width: usize, height: usize) -> Self {
		let data = vec![vec![0u32; width]; height];
        Grid {
            data,
            width,
            height,
        }
    }
	
    pub fn read(&mut self, filename: String) {

        if let Ok(lines) = read_lines(filename) {
            let mut i = 0;
            
            for line in lines {
                if let Ok(numbers) = line {
					let v: Vec<u32> = numbers
						.split(' ')
						.map(|d| d.parse::<u32>().unwrap())
						.collect();
                    println!("{:?}", v);
                    self.data[i] = v;
                    i += 1;
                }
            }

            // println!("{:?}", self.data);
        }
    }

    pub fn get_product(&self, x:usize, y:usize, orientation: &str) -> u32 {

        let (mut x2, mut y2, mut x3, mut y3, mut x4, mut y4) = (0,0,0,0,0,0);
        
        match orientation {
            "right" => {x2 = x + 1; y2 = y; x3 = x + 2; y3 = y; x4 = x + 3; y4 = y; }, 
            "left" => {x2 = x -1; y2 = y; x3 = x - 2; y3 = y; x4 = x - 3; y4 = y; },
            "bottom" => {x2=x; y2=y+1; x3 = x; y3 = y + 2; x4 = x; y4 = y + 3;}
            "diagonal-right-bottom" => {x2 = x + 1; y2 = y+1; x3 = x + 2; y3 = y+2; x4 = x + 3; y4 = y+3; }, 
            "diagonal-right-top" => {x2 = x + 1; y2 = y-1; x3 = x + 2; y3 = y-2; x4 = x + 3; y4 = y-3; }, 
            _ => {}
        }

        let point_vec = vec![self.data[x][y], self.data[x2][y2], self.data[x3][y3], self.data[x4][y4]];

        return point_vec.iter().fold(1, |prod, x| prod*x);
    }

    pub fn get_max_product(&self, x:usize, y:usize) -> u32 {
        // Right product can be calculated if
        // x <= width - 4
        // Left product can be calculated if
        // x >= 3
        // Bottom product can be calculated if
        // y <= height - 4
        // Right bottom diagonal product can be
        // calculated if x<=width -4 and
        // y <= height - 4
        // Right up diagonal product can be
        // calculated if x<=width -4 and
        // y >= 3
        let (mut rp, mut lp, mut bp, mut rbdp, mut lbdp) = (0,0,0,0,0);

        if x<=self.width - 4 {
            rp = self.get_product(x, y, "right");
        }
        if x>=3 {
            lp = self.get_product(x, y, "left");
        }
        if y <= self.height - 4 {
            bp = self.get_product(x, y, "bottom");
        }
        if x<=self.width - 4 && y <= self.height - 4 {
            rbdp = self.get_product(x, y, "diagonal-right-bottom");
        }
        if x<=self.width - 4 && y>=3 {
            lbdp = self.get_product(x, y, "diagonal-right-top");
        }

        let vals = vec![rp, lp, bp, rbdp, lbdp];

        return *vals.iter().max().unwrap();
        
    }

	pub fn width(&self) -> usize {
		self.width
	}
	pub fn height(&self) -> usize {
		self.height
	}	
}

// Ref: problem 12
// Get divisors of nth triangle number
pub fn get_divisors_triangle(n:i64, divisors: &mut HashMap<i64, i64>) -> i64 {

    // Triangle number => n(n+1)/2
    // So if n is odd the divisors
    // is divisors(n)*divisors(n+1/2)
    // If n is even, divisors is
    // divisors(n/2)*divisors(n+1)

    if n % 2 == 0 {
        get_divisors(n/2, divisors)*get_divisors(n+1, divisors)
    }
    else {
        get_divisors(n, divisors)*get_divisors((n+1)/2, divisors)
    }
}

// Get nth triangle number
pub fn get_triangle_number(n:i64) -> i64 {

    // A triangle number for n is nothing but some of all natural
    // numbers upto and including n
    n*(n+1)/2
}

// Ref: problem 13
// Add numbers per line from a file
pub fn sum_numbers(filename: String) -> BigUint {

    let mut sum: BigUint = 0u32.into();
    
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(number) = line {
                let nb = number.as_bytes();
                // println!("{:?}", nb);
                let num: BigUint = BigUint::parse_bytes(nb, 10).unwrap();
                // println!("{}", num);
                sum += num;
            }
        }
    }

	sum
}

// Ref: problem 14
// Collatz sequence count
pub fn collatz(n: i64) -> i64 {
    
    let (mut ncopy, mut count) = (n,1);

    while ncopy != 1 {
        if ncopy % 2 == 0 {
            ncopy = ncopy/2;
        } else {
            ncopy = 3*ncopy + 1;
        }

        count += 1;
    }

    return count;
}

// Ref: problem 15
// Recursive solution
pub fn npaths_lattice_r(x: i64, y: i64, h: i64, w: i64)->i64 {

    if x == w || y == h {
        return 1;
    }
    else {
        return npaths_lattice_r(x+1, y, h, w) + npaths_lattice_r(x, y+1, h, w);
    }
}

// Iterative solution
pub fn npaths_lattice_i(n: u128) -> u128 {

    //Answer: 2nCn -> (2n)!/(n!*n!)
    let twicen: u128 = 2*n;
    let mut prod1: u128 = 1;
    let mut prod2: u128 = 1;

    // Why this round-about ? Because in Rust apparently the largest
    // integer is 2**128. factorial(40) is way above that!
    // This is (n+1)*(n+2)...2n or (2n!)/(n!)
    for x in n+1..twicen+1 {
        prod1 *= x;
    }

    // This is n!
    for x in 1..n+1 {
        prod2 *= x;
    }    

    // This is (2n)!/(n!*n!)
    prod1/prod2
}

// Ref: problem 16
// Sum of digits of power of "n" to given exponent "e"
pub fn power_sum(n: u32, e: usize) -> u32 {

    let big_n: BigUint = n.into();
    let big_a: BigUint = pow::pow(big_n, e);    
	// println!("{}", big_a);

	//    let digits:Vec<u32> = big_a.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();    
    let sum = big_a.to_string().chars().map(|d| d.to_digit(10)).fold(0, |sum, x| sum + x.unwrap());//.unwrap()).collect();
    //println!("{:?}", digits);
	sum
}


// Ref: problem 17
// Returns word for numbers 1..19
fn word_till_twenty(n:i32, d1: &HashMap<i32, String>) -> String{

    let mut s: String = "".to_string();
    s.push_str(d1.get(&n).unwrap());

    return s;
}

// Returns word for numbers 10,20,30..90
fn word_tens(n:i32, d2: &HashMap<i32, String>) -> String{

    let mut s: String = "".to_string();    
    s.push_str(d2.get(&n).unwrap());

    return s;
}

// Returns word for any number from 20-99
fn word_twenty_to_nintynine(n:i32, d1: &HashMap<i32, String>, d2: &HashMap<i32, String>) -> String {

    let n1: i32;
    let n2: i32;
    let mut s: String;
    let s2: String;
    
    if n<=19 {
        return word_till_twenty(n, d1);
    }
    if n % 10 == 0 {
        return word_tens(n, d2);
    }
    else {
        n1 = n / 10;
        n2 = n % 10;

        s = word_tens(n1*10, d2);
        s.push_str(" ");
        s2 = word_till_twenty(n2, d1);
        s.push_str(&s2);

        return s;
    }
}

// Returns word for any number from 100-1000
fn word_above_hundred(n:i32, d1: &HashMap<i32, String>, d2: &HashMap<i32, String>) -> String {

    let mut s:String;
    let s2:String;
    
    let remainder: i32;
    let n1: i32;
    let n2: i32;

    // Multiples of 100
    if n % 100 == 0 {
        remainder = n/100;
        if let 1..=9 = remainder {
            s = word_till_twenty(remainder, d1);
            s.push_str(" hundred");

            return s;
        }

        if remainder == 10 {
            return "one thousand".to_string();
        }
    }

    // Multiples of 10
    else if n % 10 == 0 {
        n1 = n/100;
        n2 = n%100;

        s = word_till_twenty(n1, d1);
        s.push_str(" hundred and ");
        s2 = word_tens(n2, d2);
        s.push_str(&s2);

        return s;
    }  else {
        // All other numbers
        n1 = n/100;
        n2 = n%100;

        s = word_till_twenty(n1, d1);
        s.push_str(" hundred and ");
        s2 = word_twenty_to_nintynine(n2, d1, d2);
        s.push_str(&s2);

        return s;

    }

    return "".to_string();
                
}

// Return number in words for all numbers 1..1000
pub fn number_to_word(n:i32, d1: &HashMap<i32, String>, d2: &HashMap<i32, String>) -> String {

    if n<=19 {
        return word_till_twenty(n, d1);
    }
    if let 20..=99 = n {
        return word_twenty_to_nintynine(n, d1, d2);
    }
    if let 100..=1000 = n {
        return word_above_hundred(n, d1, d2);
    }

    return "".to_string();
}
    

// Ref: problem 18
pub struct TriangleGrid {
    data: Vec<Vec<u32>>,
    height: usize,
    count: u32,
    max_routes: u32
}

impl TriangleGrid {

	pub fn new(height: usize, max_routes: u32, count: u32) -> Self {
		let data = vec![vec![0; 50]; 50];
        TriangleGrid {
            data,
			height,
			count,
			max_routes
        }
	}
	
    pub fn read(&mut self, filename: String) {

        if let Ok(lines) = read_lines(filename) {
            let mut i = 0;
            
            for line in lines {
                if let Ok(numbers) = line {
                    let v: Vec<u32> = numbers.split(" ").map( |d| u32::from_str(d).unwrap()).collect();
                    // println!("{:?}", v);
                    self.data[i] = v;
                    i += 1;
                }
            }

            self.height = i;
            // Max routes = 2**(height-1)
            self.max_routes = 2u32.pow((self.height-1) as u32);
//            println!("Max routes => {}",self.max_routes);
        }
    }

    // Return maximum at a point [x,y] in the grid where
    // x: row, and y: column
    pub fn max_sum(&mut self, x: usize, y: usize) -> u32 {

        if x  == self.height {
            self.count += 1;
            return self.data[x-1][y];
        }
        else {
            let row: Vec<u32> = self.data[x-1].to_owned();            
            let val1:u32 = self.max_sum(x+1,y);
            let val2:u32 = self.max_sum(x+1, y+1);
            
            return max(row[y] + val1, row[y] + val2);
        }
        
    }
}

// Problem 19
// Return number of days on a year in feb
pub fn num_days_feb( year: u32 ) -> u32 {
    // println!("num days feb asked for {}", year );
    
    if year % 100 == 0 {
        if year % 400 == 0 { return 29; }
    } else if year % 4 == 0 {
        return 29;
    }

    return 28;
}

// Return dow which 1st of a month falls on
pub fn day_of_month(month: u32, year: u32) -> u32 {
    let dom: u32;
    
    if month == 1 {
        if year == 1900 {
            return 1;
        }
        else {
            dom = ((337 + num_days_feb(year -1)) % 7 + day_of_month(1, year-1)) % 7;
            // println!("DOM for {} {} is {}",year,month,dom);
            return dom;
        }
    } else {
        let mut total_days: u32 = 0;
        // Any other months

        if month >1 {
            total_days += 31;
        }
        if month >2 {
            total_days += num_days_feb(year);
        }
        if month > 3 {
            total_days += 31;
        }
        if month > 4 {
            total_days += 30;
        }
        if month > 5 {
            total_days += 31;
        }
        if month > 6 {
            total_days += 30;
        }
        if month > 7 {
            total_days += 31;
        }
        if month > 8 {
            total_days += 31;
        }
        if month > 9 {
            total_days += 30;
        }
        if month > 10 {
            total_days += 31;
        }
        if month > 11 {
            total_days += 30;
        }
        
        dom = (total_days % 7 + day_of_month(1, year)) % 7;
        return dom;
    }
}

// Date of month -> day of the week
pub fn dom_to_dow(dom: u32) -> &'static str {

    match dom {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => "No day"
    }

}

// Problem 21
// Let d(n) : sum of proper divisors of n
// if d(a) == b and d(b) ==a then a,b are an "amicable" pair.	
pub fn amicable(n: i64, amicables: &mut Vec<i64>) -> i64 {

    let mut divisors: HashMap<i64, i64> = HashMap::new();
    // Get divisor sum of n
    let s1: i64 = get_divisor_sum(n, &mut divisors);
    let s2: i64;
    
    if s1 == n {
        // a == b, cannot be an amicable pair
        return 0;
    }

    // Get divisor sume of s1 (which is divisor sum of n)
    s2 = get_divisor_sum(s1, &mut divisors);

    // d(a) = b && d(b) = a
    if n == s2 {
        //        println!("{} {}", n, s1);
        // Is an amicable pair
        amicables.push(n);
        amicables.push(s1);

        // Return their sum
        return n + s1;
    } else {
        // Not amicable
        return 0;
    }
}

// Ref: Problem 22
// Index of letter in alphabet
pub fn get_index(letter: String) -> usize  {
    let alpha: Vec<String> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().map( |s| s.to_string()).collect();
    return alpha.iter().position( |r| *r == letter ).unwrap() + 1;
}

// Score of a name in terms of alphabet position
// ANAND - 1 + 14 + 1 + 14 + 5 = 35
pub fn get_name_score(name: &String) ->usize {
    
    let sum = name.chars().map( |s| get_index(s.to_string()) ).fold(0, |sum, x| sum + x);
    sum
}

// Read comma separated names from a file
pub fn read_names_csv(filename: String) -> Vec<String> {

    let mut file = File::open(filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let names: Vec<String> = contents.split(",").map( |s| s.to_string().replace("\"", "")).collect();
	names
}

// Ref: Problem 23
// Is a number abundand with caching and divisors HashMap
pub fn is_abundant(n: i64, divisors: &mut HashMap<i64, i64>, cache: &mut HashMap<i64, bool>) -> bool {

    // Smallest abundant number is 12
    if n < 12 {
        return false;
    }

    // Use a cache
    if cache.contains_key(&n) {
        return *cache.get(&n).unwrap();
    }
    
    let sum_d = get_divisor_sum(n, divisors);

    if sum_d > n {
        // Abundant number
        cache.insert(n, true);        
        return true;
    } else {
        // Deficient or perfect number
        cache.insert(n, false);        
        return false;
    }
}

// Find sume of all positive integers which cannot be written
// as sum of two abundant numbers
pub fn non_abundant_sum(limit: i64) -> i64 {

    let mut k: i64;
    let mut flag: bool;
    let mut sum: i64 = 0;
    let mut divisors: HashMap<i64, i64> = HashMap::new();
    let mut cache: HashMap<i64, bool> = HashMap::new();    
    
    for i in 1..limit+1 {
        flag = false;

        for j in 1..(i/2+1) {
            k = i - j;
            if is_abundant(j, &mut divisors, &mut cache) {
                if is_abundant(k, &mut divisors, &mut cache) {
//                println!("Abundant sum of {} is {} + {}", i, j, k); 
                    flag = true;
                    break;
                }
            }
        }

        if !flag {
            sum += i;
        }
    }

	sum
}

// Problem 24
// Uses the technique described in https://projecteuler.net/quote_post=361-80066e82e1815a5440e0758e0cf2c1ee3aa0276f
// Millionth lexicographical permutation
pub fn reduce_index(n: u64) -> u64 {

    let mut quotient: u64;
    let mut num = n;
    let mut fact: u64;
    let mut num_idx: String = "".to_string();
    
    let mut digits: String = "0123456789".to_string();
    let mut permute_string: String = "".to_string();
    
    for i in (0..10).rev() {
        // inline factorial
        fact = (2..i+1).fold(1, |p, x| p*x);
        quotient = num/fact;

        let mut idx = 0;

        // num_idx => number in digits at idx where idx == quotient
        for c in digits.chars() {
            if (idx as u64) == quotient {
                num_idx = c.to_string();
                break;
            }
            idx += 1;
        }
        num = num % fact;
        permute_string.push_str(&num_idx);
        // Drop num_idx from digits
        digits = digits.replace(&num_idx, "");
    }

    return permute_string.parse::<u64>().unwrap();
}

// Problem 25
// Return the (index, number) of the first Fibonacci number to have 1000 digits
pub fn fibonacci_thousand_digits() -> (i32, BigUint) {

    let mut a: BigUint = 0u32.into();
    let mut b: BigUint = 1u32.into();
    let mut c: BigUint;
    let mut index: i32 = 1;
    
    loop {
        c = a + b.to_owned(); // Rust ownership rules bites us here
        a = b;
        b = c.to_owned(); // Here too.
        index += 1;
        
        if c.to_string().len() == 1000 {
            break;
        }
    }

	(index, c)
}

// Problem 26
// Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
// This implementation uses the suggestion at https://projecteuler.net/quote_post=13-f853fd887b0a6b53247dfb4e61ab628785585290  
pub fn max_recurrence_fraction(limit: u64) -> (u64, u32) {

    let mut length: u32;
    let mut max_length: u32 = 0;
    let mut max_n: u64 = 0;
    
    for n in 2..limit+1 {
        let mut rest: u64 = 1;
        let r0: u64;

        for _ in 0..n  {
            rest = (rest*10) % n;
        }
        r0 = rest;
        length = 0;

        loop {
            rest = (rest*10) % n;
            length += 1;
            if rest == r0 {
                break;
            }
        }

        if length > max_length {
            max_length = length;
            max_n = n;
        }
    }

	(max_n, max_length)
}

// Problem 27
// Count number of continuous primes for the quadratic equation
// n*n + a*n + b for given (a, b) 
pub fn count_primes(a: i64, b: i64) -> i64 { 

    let mut val: i64;
    let mut n:i64 = 0;
    
    loop {
        val = n*(n+a) + b;
        if val<2 { break; }
        
        if !is_prime_i64(val) {
            break;
        }
        n += 1;
    }
    
	n
}

// Quad prime problem
// Find the product of the coefficients (a,b)
// for n*n + a*n + b for the quadratic expression that produces the
// maximum number of primes for consecutive values of n, starting with
// n = 0
pub fn quad_prime() -> i64 {

    let mut max_run: i64 = 0;
    let mut max_a: i64 = 0;
    let mut max_b: i64 = 0;

    for a in -999..1000 {
        for b in -1000..1001 {

            let n: i64 = count_primes(a, b);
            if n > max_run {
                max_run = n;
                max_a = a;
                max_b = b;
            }
        }
    }
    
	max_a*max_b
}

// Problem 28
// Analysis: You will find each member of the spiral diagonal at level
// n is given as (n-2**2 + n-1 .. n**2 + 1).step_by(n-1)
pub fn sum_spiral_diagonal(n: u64)-> u64 {

    if n == 1 { return 1; }
    
    let start: u64 = (n-2).pow(2) + n -1;
    let end: u64 = n.pow(2) + 1;
    
    let sum = (start..end).step_by( (n as usize)-1).fold(0, |s, x| s+x);
	sum
}

// Problem 29
// How many distinct terms are in the sequence generated by
// a**b for 2 <= a <= 100 and 2 <= b <= 100 
pub fn distinct_powers(a: u32, b: usize) -> u32 {

    let mut power: BigUint;
    let mut powers: HashMap<BigUint, bool> = HashMap::new();
    let mut count: u32 = 0;
    
    for i in 2..a+1 {
        for j in 2..b+1 {
            power = pow::pow(i.into(), j);
            // println!("{}", power);
            if !powers.contains_key(&power) {
                powers.insert(power, true);
                count += 1;
            }
        }
    }

	count
}

// Problem 30
// Sum of numbers whose digits to given power equals the number itself
pub fn sum_integer_powers(power: u32) -> u64{

    let mut number: u64;
    let mut num: u64;
    let mut num_pow: u64;
    let limit: u64;
    let mut sum: u64;
    
    number = 10u64.pow(power-2);
    limit = 10u64.pow(power+1) - 1;
    sum = 0;
    
    loop {
        num_pow = 0;
        num = number;
        
        while num > 0 {
            num_pow += (num % 10).pow(power);
            num = num / 10;
        }

        if number == num_pow {
//            println!("{}", number);
            sum += number;
        }

        number += 1;
        if number == limit { break; }
    }    

	sum
}

// Problem 31
// Coin sums
// How many different ways can 2 pounds be made using any number of coins?
pub fn coin_sums(total: i32, max_coin: i32) -> u32 {

    let coins = [200,100,50,20,10,5,2,1];
    let mut sum: u32 = 0;
    let mut diff: i32;
    
    if max_coin == 7 {
        return 1;
    }

    for i in max_coin..8 {
        // println!("{} {}", total, coins[i as usize]);
        diff = total - coins[i as usize];
        if diff == 0 { sum += 1; }
        if diff > 0 { sum += coin_sums(diff, i); }
    }

    return sum;
}

// Problem 32
// Return sum of pandigital product using two multiplicands axb
// where #digits(a) -> r1 and #digits(b) -> r2
pub fn pandigital_product(r1: usize, r2: usize) -> u32{

    let numbers: Vec<u32> = vec!(1,2,3,4,5,6,7,8,9);
    let template: String = "123456789".to_string();
    let mut products: Vec<u32> = vec![];
    
    let mut sum: u32 = 0;
    
    for p1 in numbers.iter().permutations(r1) {
        let mut n2: Vec<u32> = vec![];

        for i in numbers.iter() {
            if p1.iter().position( |r| *r == i) == None {
                n2.push(*i);
            }
        }

        let d1: u32 = vectorp_to_digit_u32(p1);
        let mut d2: u32;
        let mut d3: u32;
        let mut prod_str: String;
        let mut prod_str_chars: Vec<char>;
        
        for p2 in n2.iter().permutations(r2) {
            d2 = vectorp_to_digit_u32(p2);
            d3 = d1*d2;
            prod_str = format!("{}{}{}",d1,d2,d3);
            // println!("{}", prod_str);
            prod_str_chars = prod_str.chars().collect();
            prod_str_chars.sort_by(|a,b| a.cmp(b));

            if String::from_iter(prod_str_chars) == template {
                // println!("{} {} {}",d1,d2,d3);
                if products.iter().position( |r| *r == d3) == None {
                    sum += d3;
                    products.push(d3);
                }
            }

        }
    }

    return sum;
}

// Return a vector of primes < a given limit
pub fn consecutive_primes(limit: u64) -> Vec<u64> {

    let mut n = 3;
    let mut primes:Vec<u64> = vec![2];

    while n<limit {
        if is_prime(n) {
            primes.push(n);
        }

        n += 2;
    }
        
    return primes;
}

pub fn largest_prime_factor(n: u64)->u64 {

    let x = (n as f64).sqrt() as u64 + 1;
    let mut val = 0;
    
    for item in (2..x).rev() {
        if n % item == 0 && is_prime(item) {
            val = item;
            break;
        }
    }
        
    return val;

}

// Problem 34
// Is number curious ?
pub fn curious(n: u64) -> bool {

    let mut num = n;
    let mut sum: u64 = 0;
    
    while num > 0 {
        sum += factorial_simple(num%10);
        num = num/10;
    }    

    return sum == n;
}


// Problem 35
// All circular permutations of a number
// 123 -> [123, 231, 321].
pub fn circles(n: u64) -> Vec<u64> {

    let mut num_vec: Vec<u64> = digit_to_vector(n);
    let mut rotations: Vec<u64> = vec![];
    let mut i: usize = 0;
    
    // Keep rotating one to left
    loop {
        rotations.push(vector_to_digit(&num_vec));
        num_vec.rotate_left(1);
        i += 1;

        if i == num_vec.len() { break; }
    }
    
    return rotations;
}

// Convert number to base2
pub fn num_to_base2(n: u64) -> String {

    let mut num = n;
    let mut rem: u64;
    let mut base_two: String = "".to_string();
    
    while num > 0 {
        rem = num % 2;
        num = num / 2;

        base_two.push_str(&rem.to_string());
    }

    base_two = base_two.chars().rev().collect::<String>();
    return base_two;
}

// Problem 36
// Is string a palindrome ?
pub fn string_is_palindrome(s: String) -> bool {

    let mut svec: Vec<String> = s.chars().map( |x| x.to_string()).collect();
//    println!("{:?}", svec);

    while !svec.is_empty()  {
        let first:String = svec.first().unwrap().to_string();
        if first != svec.pop().unwrap() {
            return false;
        }

        if svec.len() > 0 {
            svec.remove(0);
        }
    }
        
    return true;
}

// Problem 37
pub fn is_truncatable_prime(n: u64) -> bool {

    let n_vec = digit_to_vector(n);
    let mut n_slice_num: u64;
    
    for i in 1..n_vec.len() {
        let n_slice_left = &n_vec[i..];
        n_slice_num = vector_to_digit(&n_slice_left.to_vec());
        
        // println!("{}", n_slice_num);        
        if !is_prime(n_slice_num) {
            return false;
        }
    }

    for i in (1..n_vec.len()).rev() {
        let n_slice_right = &n_vec[..i];
        n_slice_num = vector_to_digit(&n_slice_right.to_vec());

        // println!("{}", n_slice_num);                
        if !is_prime(n_slice_num) {
            return false;
        }
    }    

    return true;
}

// Problem 38
pub fn largest_pandigital() -> u64 {
    
    let template: String = "123456789".to_string();
    let mut n: u64 = 2;
    //let mut _max_n: u64;
    let mut max_sofar: u64 = 1;
    
    while n < 1000000 {
        let mut j:u64 = 1;
        let mut count: usize = 0;
        let mut np: Vec<String> = vec![];
        let mut prod: String;
        
        loop {
            prod = (n*j).to_string();
            count += prod.len();

            np.push(prod);

            j += 1;
            if count >= 9 {
                break;
            }
        }

        // Convert Vec<String> to a single string
        let pan_str: String = np.join("");

        // Sort this string lexicographically
        let mut pan_str_chars: Vec<char> = pan_str.chars().collect();
        pan_str_chars.sort_by(|a,b| a.cmp(b));

        // Compare with template
        if String::from_iter(pan_str_chars) != template {
            n += 1;
            continue;
        }

        let pan_now: u64 = pan_str.parse::<u64>().unwrap();
        
        if pan_now > max_sofar {
            max_sofar = pan_now;
//            max_n = n;
//            println!("{} {}", max_sofar, max_n);
        }

        n += 1;
    }

    return max_sofar;
}

// Problem 39
// If p is the perimeter of a right angle triangle with integral length sides, {a,b,c},
// there are exactly three solutions for p = 120.
// {20,48,52}, {24,45,51}, {30,40,50}
// For which value of p < 1000, is the number of solutions maximised?
pub fn triplet_finder(p: u64) -> Vec<(u64,u64,u64)> {

    let mut triplets = vec![];
    let p_root: u64 = ((p as f64).sqrt() as u64) + 1;
    let mut z:u64;
    
    for x in p_root..p/2 {
        for y in p_root..p/2 {

            z = p - (x+y);
            if z<=x || z <=y {
                continue;
            }

            if x*x + y*y == z*z {
                let mut sides:Vec<u64> = vec![x,y,z];
                sides.sort();
                
                let t = (sides[0], sides[1], sides[2]);
                if triplets.iter().position( |r| *r == t) == None {
                    triplets.push(t);
                }
            }
        }
    }
                    

    return triplets;
}

// Problem 40
// An irrational decimal fraction is created by concatenating the positive integers:
// 0.123456789101112131415161718192021...

// It can be seen that the 12th digit of the fractional part is 1.
// If dn represents the nth digit of the fractional part, find the value of the following expression.

// d1*d10*d100*d1000*d10000*d100000*d1000000
pub fn champernowne_product() -> u64 {

    let mut n: u64 = 1;
    let mut prod: u64 = 1;
    let mut offset: u64 = 0;

    let offsets: Vec<u64> = vec!(1,10,100,1000,10000,100000, 1000000);
    
    loop {

        for c in n.to_string().chars() {
            offset += 1;
            if offsets.iter().position( |r| *r == offset) != None {
                // println!("Digit at offset {} is {}",offset,c);
                prod *= c.to_digit(10u32).unwrap() as u64;
            }
        }

        n += 1;
        if offset >= 1000000 {
            break;
        }
        
    }

	prod
}

// Problem 41
pub fn largest_pandigital_prime() -> u64 {
    
    let template: String = "1234567".to_string();
    // We start with 7654321 - why ?
    // Cuz 987654321 and its permuations are divisible by 9 (sum: 45)
    // Similarly 87654321 and its permutations are also divisible by 9 (sum: 36)
    let mut n: u64 = 7654321;

    // Cuz this was given in the problem as a possible lower limit
    while n > 2143 {
        let n_str: String = n.to_string();
        let mut n_str_chars: Vec<char> = n_str.chars().collect();
        n_str_chars.sort_by(|a,b| a.cmp(b));
        
        let n_template:String = template[..n_str.len()].to_string();
        // println!("{}", n_template);
        if String::from_iter(n_str_chars) == n_template {
            // println!("Trying {}",n);
            
            if is_prime(n) { 
                // println!("Largest => {}", n);
                break;
            }
        }

        // Cuz only odd numbers can be prime
        n -= 2;
    }

	n
}

// Problem 42
pub fn gen_triangle_nums(limit: u32) -> Vec<u32> {

    let mut tri_nums: Vec<u32> = vec![];

    for i in 1..limit+1 {
        tri_nums.push(i*(i+1)/2);
    }

    return tri_nums;
}

pub fn read_words(filename: String) -> Vec<String> {

    let mut file = File::open(filename).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let words: Vec<String> = contents.split(",").map( |s| s.to_string().replace("\"", "")).collect();
    //    println!("{:?}", words);

    return words;
}

pub fn word_value(word: String) -> u32 {

    let alpha: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut idx: usize;

    let word_vec: Vec<char> = word.chars().collect();
    let mut word_val: usize = 0;
        
    for c in word_vec.iter() {
        idx = alpha.iter().position( |r| *r == *c).unwrap() + 1;
        word_val += idx;
    }

    return word_val as u32;
}

pub fn find_triangle_words(filename: String) -> u32 {

    let words: Vec<String> = read_words(filename);
    let tri_nums = gen_triangle_nums(100);
    let mut count:u32 = 0;
    
    for word in words.iter() {
        let w_val: u32 = word_value(word.to_string());
        // println!("{} {}", word, w_val);
		
        if tri_nums.iter().position( |r| *r == w_val) != None {
            count += 1;
        }
    }
    
    return count;
}

// Problem 43
pub fn check_sub_divisibility(n_vec: &Vec<u64>) -> bool {    

    // Already taking care of 2,3 and 5 divisibility in main loop
    let d4: bool = vector_to_digit(&n_vec[4..7].to_vec()) % 7 == 0;
    let d5: bool = vector_to_digit(&n_vec[5..8].to_vec()) % 11 == 0;
    let d6: bool = vector_to_digit(&n_vec[6..9].to_vec()) % 13 == 0;
    let d7: bool = vector_to_digit(&n_vec[7..].to_vec()) % 17 == 0;

    return [d4,d5,d6,d7].iter().fold(true, |p,x| p &x);
}

pub fn pandigital_divisible_sum() -> u64 {
    
    let mut sum: u64 = 0;

    for it in (0..10).permutations(10) {
        if it[0] == 0 { continue; }

        // 5th index has to be 5 or 0 to be divisible by 5
        if it[5] != 0 && it[5] != 5 {
            continue;
        }        

        // Number at 3rd index has to be divisible by 2
        if it[3] % 2 != 0 {
            continue;
        }

        // Sum of [2..5] index to be divisible by 3
        if (it[2] + it[3] + it[4]) % 3 != 0 {
            continue;
        }
        
        if check_sub_divisibility(&it) {
            let n = vector_to_digit(&it);
            sum += n;
        }
    }

    return sum;
}

// Problem 44
pub fn generate_pentagonal_numbers(limit: u128) -> Vec<u128> {

    let mut numbers: Vec<u128> = vec![];
    let mut num: u128;
    let mut n: u128 = 1;
    
    loop {
        num = n*(3*n-1)/2;
        numbers.push(num);

        n += 1;
        if n >= limit { break; }
    }

    return numbers;
}

pub fn pentagon_min_diffsum(numbers: Vec<u128>) -> (u128, u128, u128) {

    let mut min_diff: u128 = 2u128.pow(127);
    let mut min_p1: u128 = 0;
    let mut min_p2: u128 = 0;

    // Put the numbers in a HashMap for quick check
    let mut numbers_hash: HashMap<u128, bool> = HashMap::new();
    for p in numbers.iter() {
        numbers_hash.insert(*p, true);
    }

    // Trick is to iterate outer loop in increasing direction
    for p1 in numbers.iter() {
        // And inner loop in decreasing direction
        for p2 in numbers.iter().rev() {
            if *p1 == *p2 {continue;}

            let p_diff;
            if *p1 > *p2 {
                p_diff = *p1 - *p2;
            } else {
                p_diff = *p2 - *p1;
            }

            // Useful to shave off time after a first solution is found
            if p_diff > min_diff { break; }

            if !numbers_hash.contains_key(&p_diff) {
                continue;
            }            
                        
            let p_sum:  u128 = *p1 + *p2;
            if !numbers_hash.contains_key(&p_sum) {
                continue;
            }

            if p_diff < min_diff {
                //println!("{} {} {}", p_diff, p1, p2);
                min_diff = p_diff;
                min_p1 = *p1;
                min_p2 = *p2;
            }
        }
    }

    return (min_p1, min_p2, min_diff);
}

// Problem 45
// Triangular, Pentagonal and Hexagonal

// Given a pentagonal number find its n
pub fn get_n_penta(p: u64) -> f64 {
    // Formula: P = (3n*n - n)/2
    // This gives the quadratic equation
    // 3n*n - n -2*P = 0
    // The solution for this is,
    // (1 + sqrt(1 + 24P))/6
    return (1.0 + ((24*p + 1) as f64).sqrt())/6.0
}

pub fn get_n_hexa(h: u64) -> f64 {
    // Formula: P = 2n*n - n
    // This gives the quadratic equation
    // 2n*n - n - P = 0
    // The solution for this is,
    // (1 + sqrt(1 + 8P))/4

    return (1.0 + ((8*h + 1) as f64).sqrt())/4.0
}

// Problem 46
// Goldbach's other conjecture
// What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?
pub fn smallest_composite() -> u64 {

    let mut n: u64 = 35;
    let mut flag: bool;
    
    loop {
        // println!("Trying {}", n);
        if !is_prime(n) { 
            flag = false;
            let lesser_primes: Vec<u64> = (2..n-1).filter(|x| is_prime(*x)).collect();
            
            for p in lesser_primes.iter() {
                let root: f64 =(((n - p) as f64)*0.5).sqrt();
                if root.trunc() == root {
                    flag = true;
                    break;
                }
            }

            if !flag {break;}
        }

        n += 2;
    }

	n
}

// Problem 47
pub fn consecutive_prime_factors(min_p: usize) -> Vec<u32> {

    let mut n_range = 0..1;
    
    if min_p == 2 {
        n_range = 10..100;
    } else if min_p == 3 {
        n_range = 100..1000;
    } else if min_p == 4 {
        n_range = 10000..1000000;
    }

    let mut idx: usize = 0;
    let mut numbers: Vec<u32> = vec![];
    
    for n in n_range {
        let p_factors = get_prime_factors_u32(n);
        if p_factors.len() >= min_p {
            numbers.push(n);

            if (idx>0) && (n - numbers[idx-1]) > 1 {
                numbers.clear();
                idx = 0;
            } else {
                idx += 1;
                // println!("{:?}", numbers);
            }
        } 

        if idx == min_p {
            break;
        }
    }
    
    return numbers;
}

// Problem 48
// Self powers
// Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
pub fn sum_self_powers(limit: usize) -> String {

    let mut n: usize = 1;
    let mut p: BigUint;
    let mut sum: BigUint = 0u32.into();
    
    loop {
        p = pow::pow(n.into(), n);
        sum += p;
        if n == limit { break; }
        n += 1;
    }

    let last_10: BigUint = sum % 10000000000u128;
    return last_10.to_string();
}

// Problem 49
// Prime permutations
pub fn prime_permute() -> String {

    let mut n:u32 = 1001;
    let mut count: usize = 0;
    
    loop {
        if is_prime_u32(n) {
            let n_vec: Vec<u32> = digit_to_vector_u32(n);
            let mut n_prime: Vec<u32> = vec![n];
            let mut idx: usize = 0;
            
            for it in n_vec.iter().permutations(4) {
                if *it[0] == 0 { continue; }

                let n_perm: u32 = vectorp_to_digit_u32(it.to_vec());
                
                if is_prime_u32(n_perm) && (n_perm > n_prime[idx]) && ((n_perm - n_prime[idx]) == 3330) {
                    n_prime.push(n_perm);
                    idx += 1;
                }
            }

            if n_prime.len() == 3 {
                count += 1;
                // println!("{:?}", n_prime);

                if count == 2 {
                    return n_prime.iter().map( |x| x.to_string() ).join("");
                }
            }
        }

        n += 1;
        if n == 9999 { break; }
    }

    return "".to_string();
}

// Problem 50
pub fn max_consecutive_prime_sum(limit: u64) -> u64 {

    // We need just primes till maximum half of the limit
    let primes: Vec<u64> = consecutive_primes(limit/2);
    
    let mut idx: usize = 0;
    let mut sum: u64;
    let mut max_sum: u64 = 0;
    let mut max_size: usize = 0;
    
    // println!("{:?}", primes);

    loop {
        let mut count: usize = 0;
        let mut size: usize = 0;

        sum = 0;
//        println!("Iteration {}", idx+1);
        
        for n in primes.iter() {
            if count < idx { count += 1; continue; }

            sum += n;
            if sum > limit { break; }

            size += 1;
            
            if is_prime(sum) && sum > max_sum  && sum < limit && size > max_size {
                max_sum = sum;
                max_size = size;
            }
        }

        // println!("Max sum => {} {} {}", max_sum, max_size, primes[idx]);
        
        // No point continuing if the next prime + current_sum > limit
        if primes[idx] + max_sum > limit {
            break;
        }
        
        idx += 1;
        if idx == primes.len() { break; }

    }

	max_sum
}

