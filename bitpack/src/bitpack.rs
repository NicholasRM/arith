/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    if n >= 0 {
        n <= maxs(width)
    } else {
        n >= !maxs(width)
    }
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    n <= maxu(width)
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    let mask = shlu(maxu(width), lsb);
    let isolated_val = (word & mask) as i64;
    let signed_val = shls(isolated_val, 64-(width+lsb));
    shrs(signed_val, 64-width)
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    let mask = shlu(maxu(width), lsb);
    let isolated_val = word & mask;
    shru(isolated_val, lsb)
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if !fitsu(value, width) || width + lsb > 64 {
        None
    } else {
        let mask = shlu(maxu(width), lsb);
        let cleaned_word = word & !mask; // replace area where value will go with 0s
        let moved_val = shlu(value, lsb);
        Some(cleaned_word | moved_val)
    }
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if !fitss(value, width) || width + lsb > 64 {
        None
    } else {
        let mask = shlu(maxu(width), lsb);
        let cleaned_word = word & !mask;
        let stripped_val = (value as u64) & maxu(width); // remove any proceeding 1s and cast value as u64
        let moved_val = shlu(stripped_val, lsb);
        Some(cleaned_word | moved_val)
    }
}

pub fn maxu(width: u64) -> u64 {
    match width {
        0 => 0,
        1..=63 => (1_u64 << width) - 1,
        _ => !0_u64,
    }
}

pub fn maxs(width: u64) -> i64 {
    match width {
        0 => 0,
        1..=62 => (1_i64 << (width - 1)) -1,
        _ => ((1_u64 << 63) - 1) as i64, // all bits on sans signed bit
    }
}

fn shlu(n: u64, lsb: u64) -> u64 {
    if lsb >= 64 {
        0
    } else {
        n << lsb
    }
}

fn shls(n: i64, lsb: u64) -> i64 {
    if lsb >= 64 {
        0
    } else {
        n << lsb
    }
}

fn shru(n: u64, lsb: u64) -> u64 {
    if lsb >= 64 {
        0
    } else {
        n >> lsb
    }
}

fn shrs(n: i64, lsb: u64) -> i64 {
    if lsb >= 64 {
        0
    } else {
        n >> lsb
    }
}

#[cfg(test)]
mod tests {
    use crate::bitpack;
    #[test]
    fn unsigned_left_shifts() {
        assert_eq!(bitpack::shlu(0b101, 5), 0b10100000);
        assert_eq!(bitpack::shlu(0xf, 32), 0xf00000000);
        assert_eq!(bitpack::shlu(1111, 64), 0);
        assert_eq!(bitpack::shlu(7001, 0), 7001);
    }

    #[test]
    fn signed_left_shifts() {
        assert_eq!(bitpack::shls(-1, 2), -4);
        assert_eq!(bitpack::shls(1, 2), 4);
        assert_eq!(bitpack::shls(-1, 64), 0);
        assert_eq!(bitpack::shls(-1, 0), -1);
    }
}
