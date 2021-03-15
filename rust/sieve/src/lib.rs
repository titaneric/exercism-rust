pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut marks = vec![false; (upper_bound + 1) as usize];
    let mut largest_prime: u64 = 2;
    let checked_upperbound = (upper_bound as f64).sqrt();
    (2..=upper_bound)
        // .take_while(|&x| {
        //     let fx = x as f64;
        //     dbg!("take", fx);
        //     fx <= (upper_bound as f64).sqrt()
        // })
        .filter_map(|x| {
            let ix = x as usize;
            if marks[ix] {
                return None;
            } else {
                if (largest_prime as f64) <= checked_upperbound {
                    (x..=upper_bound).step_by(ix).for_each(|c| {
                        marks[c as usize] = true;
                    });
                }
                largest_prime = x;
                dbg!(largest_prime);
                return Some(x);
            }
        })
        .collect()
}
