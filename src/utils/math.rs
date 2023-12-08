/// TODO: make these functions generic

/// Greatest Common Divisor
/// Implemented in Rust from https://www.geeksforgeeks.org/steins-algorithm-for-finding-gcd/
pub fn gcd(a: usize, b: usize) -> usize {
    if a == b {
        return a;
    }
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    // Finding K, where K is the greatest power of 2 that divides both a and b
    let mut k = 0;
    let mut a = a;
    let mut b = b;
    while ((a | b) & 1) == 0 {
        a >>= 1;
        b >>= 1;
        k += 1;
    }
    // Dividing a by 2 until a becomes odd
    while (a & 1) == 0 {
        a >>= 1;
    }
    // From here on, 'a' is always odd
    loop {
        // If b is even, remove all factor of 2 in b
        while (b & 1) == 0 {
            b >>= 1;
        }
        // Now a and b are both odd. Swap if necessary so a <= b, then set
        // b = b - a (which is even)
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b -= a;
        if b == 0 {
            break;
        }
    }
    // restore common factors of 2
    a << k
}

/// Least common multiple
pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn math_gcd() {
        assert_eq!(17, gcd(17, 34));
        assert_eq!(4, gcd(8, 12));
        assert_eq!(6, gcd(54, 24));
        assert_eq!(6, gcd(48, 18));
    }
}
