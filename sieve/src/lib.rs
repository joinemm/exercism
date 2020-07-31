pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut prime_status: Vec<bool> = (0..=upper_bound).map(|n| n > 1).collect();

    for n in 2..upper_bound {
        if prime_status[n as usize] {
            for i in (n.pow(2)..=upper_bound).step_by(n as usize) {
                prime_status[i as usize] = false;
            }
        }
    }

    (0..=upper_bound)
        .filter(|n| prime_status[*n as usize])
        .collect()
}
