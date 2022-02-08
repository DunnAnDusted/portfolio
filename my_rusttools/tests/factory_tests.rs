use my_rusttools::factories::*;
use std::fmt::Write;

#[test]
fn sieve_test() {
    let primes = sieve_primes(10);

    assert!(primes.eq(vec![2, 3, 5, 7]));
}

#[test]
fn rws_test() {
    let range = range_with_step(0..=10, 2);
    let nums = vec![0, 2, 4, 6, 8, 10];

    assert!(range.eq(nums));
}

#[test]
fn rws_type_test() {
    let range = range_with_step('a'..='z', 2);
    let chars = vec!['a', 'c', 'e', 'g', 'i', 'k', 'm', 'o', 'q', 's', 'u', 'w', 'y'];

    assert!(range.eq(chars));
}

#[test]
fn behaviour_test() {
    let primes = sieve_primes(10);

    assert_eq!("2357".to_string(), primes.fold(String::new(), |mut acc, x|{write!(acc, "{}", x).expect("buffer overflow"); acc}));
}

#[test]
fn ref_test() {
    let mut temp = sieve_primes(10);

    assert_eq!(Some((2, 3)), match (temp.next(), temp.next()) {
        (Some(x), Some(y)) => Some((x, y)),
        _ => None
    });
}