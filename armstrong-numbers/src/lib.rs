pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let raise_to = digits.len() as u32;
    let mut sum = 0;

    for digit in digits {
        sum += digit.pow(raise_to);
    }

    sum == num
}
