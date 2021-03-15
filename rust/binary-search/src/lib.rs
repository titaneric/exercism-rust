use std::convert::AsRef;

pub fn find<T, U>(array: U, key: T) -> Option<usize> 
where T: PartialOrd + PartialEq, U: AsRef<[T]>
{
    let array = array.as_ref();
    if array.len() == 0 {
        return None;
    }
    let (mut l, mut r) = (0, array.len() - 1);
    while l <= r {
        let mid = (l + r) / 2;
        let mid_ele = array.get(mid);
        if mid_ele == Some(&key) {
            return Some(mid);
        } else if mid_ele > Some(&key) {
            r = mid.checked_sub(1)?;
        } else if mid_ele < Some(&key){
            l = mid.checked_add(1)?;
        }
    }
    None
}
