pub fn is_armstrong_number(num: u32) -> bool {
    let digit_num = count_digit_number(num);
    let num_vec: Vec<u32> = num
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();
    let num_sum = num_vec.iter().fold(0, |acc, d| acc + d.pow(digit_num));
    num_sum == num
}
fn count_digit_number(num: u32) -> u32 {
    num.to_string().chars().fold(0, |acc, _| acc + 1)
}
