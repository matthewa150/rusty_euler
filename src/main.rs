mod question;

fn main() {
    let question: u16 = 1;
    println!("For question:{num}, result={ans}",
        num=question, ans=question::return_solution(&question));
}
