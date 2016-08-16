extern crate erustosthenes;

use erustosthenes::find_primes_to_limit;

fn main() {
    let primes = find_primes_to_limit(30);

    for i in 0 .. primes.len() - 1 {
        print!("{}, ", primes[i]);
    }
    println!("{}", primes.last().unwrap());
}
