#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num < 1 {
        return None;
    }
    let factor_list = factors(num);
    dbg!(&factor_list);
    let factor_sum: u64 = factor_list.iter().sum();
    if factor_sum == num {
        Some(Classification::Perfect)
    } else if factor_sum > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}

fn factors(num: u64) -> Vec<u64> {
    (1..num).filter(|x| num % x == 0).collect()
}
