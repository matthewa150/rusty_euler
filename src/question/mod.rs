mod one_ten;

pub fn return_solution(question: &u16) -> String {

    let ret: String = match *question {
        1    => String::from(format!("{}",one_ten::q_one())),
        2    => String::from(format!("{}",one_ten::q_two())),
        3    => String::from(format!("{}",one_ten::q_three())),
        _    => String::from("No Question"),
    };
    ret
}
