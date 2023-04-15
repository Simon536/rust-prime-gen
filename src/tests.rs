use super::*;

#[test]
fn test_prime_count() {
    // There are 78,498 primes smaller than 1,000,000
    assert_eq!(78498, primesieve(1000000).iter().filter(|x| **x).count());
}

#[test]
fn test_prime_list() {
    assert_eq!(
        vec![
            false, false, true, true, false, true, false, true, false, false, false, true,
            false, true, false
        ],
        primesieve(15)
    );
}

#[test]
fn sieve_range_count() {
    assert_eq!(primesieve_extended(10, 50), 11);
}