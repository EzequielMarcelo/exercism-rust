pub fn is_armstrong_number(num: u64) -> bool {
    let mut digits = Vec::new();
    let mut sum:u64 = 0;
    let mut aux_num = num;

    while aux_num > 0 {
        digits.push(aux_num % 10);
        aux_num /= 10;
    }

    let digits_num = digits.len() as u32;

    digits.iter().for_each(|digit| sum += digit.pow(digits_num));

    sum == num
}
