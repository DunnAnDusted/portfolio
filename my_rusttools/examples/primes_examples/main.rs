use my_rusttools::factories;

fn main() {
    println!("{:?}", factories::sieve_primes(10000).collect::<Vec<_>>());
}