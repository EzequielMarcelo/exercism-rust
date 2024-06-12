pub fn is_armstrong_number(num: u64) -> bool {
    let digits: Vec<u64> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let digits_num = digits.len() as u32;
    let sum: u64 = digits.iter()
                         .map(|digit|digit.pow(digits_num))
                         .sum();

    sum == num
}
