pub fn is_leap_year(year: u64) -> bool {
    let mut is_leap: bool = false;

    if year % 4 == 0 {
        if !(year % 100 == 0 && year % 400 != 0) {
            is_leap = true;
        }
    }

    is_leap
}
