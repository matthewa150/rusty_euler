
pub fn q_one () -> i32 {
    let mut sum: i32 = 0;
    for x in 1..1000 {
        if x%3==0 || x%5==0 {
            sum+=x;
        }
    }
    sum
}
