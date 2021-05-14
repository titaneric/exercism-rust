#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

const MASK: u32 = 0b1111111;
const IS_REMAINED: u8 = 0b10000000;
const IS_LAST: u8 = 0b00000000;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result = Vec::new();
    for value in values {
        let mut val_result = Vec::new();
        let mut cur_val = *value;
        loop {
            let val = (cur_val & MASK) as u8;
            val_result.push(val);
            cur_val = cur_val >> 7;
            if cur_val == 0 {
                break;
            }
        }

        result.extend(
            val_result
                .iter()
                .enumerate()
                .map(|(idx, &val)| {
                    if idx == 0 {
                        val | IS_LAST
                    } else {
                        val | IS_REMAINED
                    }
                })
                .rev(),
        );
    }
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result = Vec::new();
    let mut sum: u32 = 0;
    let mut is_last = false;
    for byte in bytes.iter() {
        let val = (*byte as u32) & MASK;
        is_last = (*byte & IS_REMAINED) == 0;

        sum = sum.checked_mul(128).ok_or(Error::Overflow)?;
        sum = sum.checked_add(val).ok_or(Error::Overflow)?;
        
        if is_last {
            result.push(sum);
            sum = 0;
        }
    }

    if is_last {
        Ok(result)
    } else {
        Err(Error::IncompleteNumber)
    } 
}
