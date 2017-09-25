
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
