mod question;

use std::io;

fn main() {
    println!("Input Question Number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Please give readable input");
    let question: u16 = match input.trim().parse::<u16>() {
        Ok(n)   => n,
        Err(_)=> panic!("oops"),
    };
    println!("For question:{num}, result={ans}",
        num=question, ans=question::return_solution(&question));
}
