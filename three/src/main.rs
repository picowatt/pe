use std::collections::HashSet;

use::rand::{thread_rng, Rng};
use::rand::distributions::Uniform;
use::rand::ThreadRng;

fn main() {
    let mut rng = thread_rng();

  let to_consider: u64 = 600851475143;

    // // one
    // for j in 0..200 {
    //     for i in 4..(to_consider / 2) {
    //         if to_consider % i == 0 {
    //             let n: u64 = to_consider / i;
    //             if is_prime_by_rand(&mut rng, n, simple_int_sqrt(n)) {
    //                 println!("\nprime: {}", n);
    //                 return;
    //             }
    //         }
    //     }
    // }


    // two
    let mut v2: Vec<u64> = vec!();

    for i in 2..simple_int_sqrt(to_consider) {
        if let None = check_any_divisible(i, &v2) {
            v2.push(i);
        }
    }
    println!("total found: {}", v2.iter().count());

    for i in 2..(to_consider / 2) {
        if to_consider % i == 0 {
            let n = to_consider / i;
            if let None = check_any_divisible(n, &v2) {
                println!("{}", n);
                return;
            }
        }
    }
}

fn simple_int_sqrt(n: u64) -> u64 {
    return (n as f64).sqrt() as u64
}

fn check_any_divisible(num: u64, vec: &Vec<u64>) -> Option<u64> {
    for elem in vec {
        if *elem >= num {
            break;
        }
        if num % elem == 0 {
            return Some(*elem);
        }
    }

    None
}

fn  is_prime_by_rand(gen: &mut ThreadRng, num: u64, its: u64) -> bool {
    let mut hash: HashSet<u64> = HashSet::new();
    let uni_range = Uniform::new(2, num - 1);
    let mut n_g = gen.sample_iter(&uni_range);

    for i in 0..=its {
        loop {
            let r = n_g.next().unwrap();
            
            if hash.contains(&r) {
                continue;
            }
            print!("\r{} {}/{}", num, i, its);
            hash.insert(r);
            //      pos_exp_mod(r, num - 1, num) == 1 {
            if fast_pos_exp_mod(r, num - 1, num) == 1 {
                println!("");
                return true;
            }

            break;
        }
    }
    println!("");

    false
}


/* https://en.wikipedia.org/wiki/Modular_exponentiation */

fn pos_exp_mod(b: u64, e: u64, m: u64) -> u64 {
    let mut c: u128 = 1;

    for _i in 1..=e {
        c = (c * (b as u128)) % (m as u128);
    }

    c as u64
} 

fn fast_pos_exp_mod(b: u64, e: u64, m: u64) -> u64 {
    let mut c: u128 = 1;
    let mut base: u128 = (b as u128) % (m as u128);
    let mut exp = e;

    while exp > 0 {
        println!("looping");
        if exp % 2 == 0 {
            c = (c * base as u128) % m as u128;
        }
        exp = exp >> 1;
        base = (base * base as u128) % m as u128;
    }

    c as u64
}