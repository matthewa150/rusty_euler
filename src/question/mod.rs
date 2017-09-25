mod one_ten;

pub fn return_solution(question: &u16) -> String {

    let ret: String = match *question {
        1u16 => String::from(format!("{}",one_ten::q_one())),
        2u16 => String::from(format!("{}",one_ten::q_two())),
        _ =>    String::from("No Question"),
    };
    ret
}
