mod question;

use std::io;

fn main() {

    let question: u16 = 3;
    println!("For question:{num}, result={ans}",
        num=question, ans=question::return_solution(&question));
}
