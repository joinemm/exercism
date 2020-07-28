pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];
    let mut i: u32 = 3;

    while primes.len() - 1 < n as usize {
        let mut is_prime: bool = true;
        for prime in &primes {
            if i % prime == 0 {
                is_prime = false
            }
        }
        if is_prime {
            primes.push(i)
        }
        i += 1;
    }

    match primes.pop() {
        Some(prime) => prime,
        None => 0,
    }
}
