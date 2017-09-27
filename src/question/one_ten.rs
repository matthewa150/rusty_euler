extern crate primes;
extern crate num;

use std::f64;

pub fn q_one () -> i32 {
    let mut sum: i32 = 0;
    for x in 1..1000 {
        if x%3==0 || x%5==0 {
            sum+=x;
        }
    }
    sum
}

pub fn q_two() -> i64{
    let mut sum: i64 = 0;
    let mut index = 1;
    loop {
        let fib = fibonacci(index);
        index+=1;
        if fib > 4000000 {
            break;
        } else if fib%2==0 {
            sum += fib;
        }
    }
    sum
}

fn fibonacci(num: i32) -> i64 {
    match num {
        1 => 1,
        2 => 2,
        3 => 3,
        _ => fibonacci(num-1)+fibonacci(num-2)
    }
}

pub fn q_three() -> u64 {
    let mut pset = primes::PrimeSet::new();
    let pset:Vec<u64> = pset.prime_factors(600851475143);
    pset[pset.len()-1]
}

pub fn q_four() -> u64{
    let mut winner: u64 = 0;
    for a in 100..1000 {
        for b in 100..1000 {
            let orig = a*b;
            let mut flip = String::from(format!("{}",orig));
            unsafe {
                let mut flip = flip.as_mut_vec();
                flip.reverse();
                let flip = String::from_utf8_lossy(flip).parse::<u32>().expect("Ooops");
                if orig == flip && (flip as u64) >winner {
                    winner = flip as u64;
                }
            }

        }
    }
    winner
}

pub fn q_five() -> i64{
    let mut winner = 2520;
    loop {
        let mut sln : bool = true;
        for x in 1..21 {
            if winner % x != 0 {
                sln = false;
                break;
            }
        }

        if sln {
            return winner;
        }
        winner+=2520;
    }
}


pub fn q_six() -> i64{
    let mut sum_squares = 0_f64;

    let mut x:f64 = 1_f64;

    while x<=100_f64 {
        sum_squares += x.powi(2);
        x+=1_f64;
    }

    x = 1_f64;

    let mut square_sum = 0_f64;

    while x<=100_f64 {
        square_sum += x;
        x+=1_f64;
    }


    square_sum = square_sum.powi(2);

    let result = sum_squares - square_sum;
    let result = result.abs();

    (result as i64)
}


pub fn q_seven() -> i64 {

    let mut pset = primes::PrimeSet::new();
    let n = pset.get(10_001 - 1);

    (n as i64)
}
