pub fn factors(n: u64) -> Vec<u64> {
    let mut facs = vec![];
    let mut cur_num = n;
    while cur_num != 1 {
        let fac = find_factor(cur_num);
        facs.push(fac);
        cur_num = cur_num / fac;
    }
    facs
}

fn find_factor(n: u64) -> u64 {
    return (2..=n).find(|&x| n % x == 0).unwrap();
}
