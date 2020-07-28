pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum: u32 = 0;
    for n in 1..limit {
        for f in factors {
            if *f != 0 && n % f == 0 {
                sum += n;
                break;
            }
        }
    }
    sum
}
