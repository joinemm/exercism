pub fn factors(mut n: u64) -> Vec<u64> {
    let mut divisors = Vec::<u64>::new();

    while n % 2 == 0 && n > 0 {
        divisors.push(2);
        n /= 2;
    }

    let mut i: u64 = 3;
    while i * i <= n {
        while n % i == 0 {
            divisors.push(i);
            n /= i;
        }
        i += 2;
    }

    if n > 2 {
        divisors.push(n);
    }

    divisors
}
