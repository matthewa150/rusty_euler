extern crate primes;

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
