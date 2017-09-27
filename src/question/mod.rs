mod one_ten;

pub fn return_solution(question: &u16) -> String {

    let ret: String = match *question {
        1    => String::from(format!("{}",one_ten::q_one())),
        2    => String::from(format!("{}",one_ten::q_two())),
        3    => String::from(format!("{}",one_ten::q_three())),
        4    => String::from(format!("{}",one_ten::q_four())),
        5    => String::from(format!("{}",one_ten::q_five())),
        6    => String::from(format!("{}",one_ten::q_six())),
        7    => String::from(format!("{}",one_ten::q_seven())),
        _    => String::from("No Question"),
    };
    ret
}
