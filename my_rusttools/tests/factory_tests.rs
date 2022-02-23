use my_rusttools::factories::*;

#[test]
fn sieve_to_10th() {
    let primes = sieve_primes(10);

    assert!(primes.eq(vec![2, 3, 5, 7]));
}

#[test]
fn fizzbuzz_indexes() {
    let first_15 = ["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"]
        .into_iter()
        .map(str::to_owned);

    assert!(fizzbuzz().take(15).eq(first_15));
}

#[test]
fn correct_intervals() {
    let bar = ["", "", "", "", "Bar"]
        .into_iter()
        .map(str::to_owned);

    assert!(repeat_interval("Foo", 3).take(3).eq(["", "", "Foo"]));
    assert!(repeat_interval_with(||String::from("Bar"), 5).take(5).eq(bar));

    let foobar = repeat_values(&[("A", 3), ("B", 5)]);
    
    assert!(foobar.eq(["A", "A", "A", "B", "B", "B", "B", "B"]));
}

#[test]
#[should_panic]
#[ignore = "really long process times, attemping usize overflow"]
fn fizzbuzz_is_infinite() {
    fizzbuzz().skip(usize::MAX).for_each(|x|println!("{}", x));
}