// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Shared utilities for eulerust-project
// use num_bigint::BigInt;
// use num_traits::{One, Zero};

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

// Ref: problem3
pub fn largest_prime_factor(mut n:u64)->u64{
    let mut f=2; let mut last=1;
    while f*f<=n{
        if n%f==0{last=f; n/=f;} else {f+=1;}
    }
    if n>1{n}else{last}
}

// Internal refs
pub fn lcm(a:u64,b:u64)->u64{ a/gcd(a,b)*b }
fn gcd(mut a:u64,mut b:u64)->u64{while b!=0{let t=b; b=a%b; a=t;} a}

// Ref: problem5
pub fn smallest_multiple(n:u64)->u64{(1..=n).fold(1,|acc,x|lcm(acc,x))}

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
