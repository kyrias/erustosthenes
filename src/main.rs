extern crate getopts;
extern crate erustosthenes;

use std::env;
use getopts::Options;
use erustosthenes::find_primes_to_limit;


fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [limit]", program)));
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();
    opts.optflag("h", "help", "Show this usage message.");
    opts.optopt("l", "limit", "Find primes up to limit. [Default: 30]", "LIMIT");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m },
        Err(e) => { panic!(e.to_string()) },
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let limit: usize = match matches.opt_str("l") {
            Some(lim) => { lim.trim().parse().expect("Failed to parse limit") }
            None      => { 30 }
    };


    let primes = find_primes_to_limit(limit);

    for i in 0 .. primes.len() - 1 {
        print!("{}, ", primes[i]);
    }
    println!("{}", primes.last().unwrap());
}
