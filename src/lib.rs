pub fn find_primes_to_limit(limit: usize) -> Vec<usize> {
    let mut list: Vec<usize> = (2..limit+1).collect();
    let mut current_prime = 2;

    mark_composites(current_prime, limit, &mut list);

    loop {
        match find_next_prime(current_prime, &list) {
            Some(p) => {
                current_prime = p;
                mark_composites(current_prime, limit, &mut list);
            },
            None => { break; }
        }
    }

    list.iter().filter(|&x| *x > 0).cloned().collect::<Vec<usize>>()
}


fn mark_composites(current_prime: usize, n: usize, list: &mut Vec<usize>) {
    let mut i = 2 * current_prime;
    while i <= n {
        list[i-1-1] = 0;
        i += current_prime;
    }
}


fn find_next_prime(current_prime: usize,  list: &Vec<usize>) -> Option<usize> {
    match list.iter().find(|&x| x.gt(&current_prime)) {
        Some(&v) => match v > current_prime {
            true => Some(v),
            false => None
        },
        None    =>  None
    }
}
