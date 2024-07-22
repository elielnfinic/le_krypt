pub mod field;
pub mod field_element;
pub const DEFAULT_PRIME: u128 = 1 + 407 * (1 << 119);
// Field modulus = 2^128 - 45 * 2^40 + 1
const M: u128 = 340282366920938463463374557953744961537;

pub fn _safe_sub(x : u128, y : u128) -> u128 {
    if x > y {
        x - y
    } else {
        x + DEFAULT_PRIME - y
    }
}

fn _mul_128x64(a: u128, b: u64) -> (u64, u64, u64) {
    let z_lo = ((a as u64) as u128) * (b as u128);
    let z_hi = (a >> 64) * (b as u128);
    let z_hi = z_hi + (z_lo >> 64);
    (z_lo as u64, z_hi as u64, (z_hi >> 64) as u64)
}

fn _mul_by_modulus(a : u64) -> (u64, u64, u64) {
    let a_lo = ( a as u128).wrapping_mul(DEFAULT_PRIME);
    let a_hi = if a == 0 { 0 } else { a - 1};
    (a_lo as u64, (a_lo >> 64) as u64, a_hi)
}

fn _mul_reduce(z0: u64, z1: u64, z2: u64) -> (u64, u64, u64) {
    let (q0, q1, q2) = _mul_by_modulus(z2);
    let (z0, z1, z2) = _sub_192x192(z0, z1, z2, q0, q1, q2);
    (z0, z1, z2)
}

fn _sub_192x192(a0: u64, a1: u64, a2: u64, b0: u64, b1: u64, b2: u64) -> (u64, u64, u64) {
    let z0 = (a0 as u128).wrapping_sub(b0 as u128);
    let z1 = (a1 as u128).wrapping_sub((b1 as u128) + (z0 >> 127));
    let z2 = (a2 as u128).wrapping_sub((b2 as u128) + (z1 >> 127));
    (z0 as u64, z1 as u64, z2 as u64)
}

fn _sub_modulus(a_lo: u64, a_hi: u64) -> (u64, u64) {
    let mut z = 0u128.wrapping_sub(M);
    z = z.wrapping_add(a_lo as u128);
    z = z.wrapping_add((a_hi as u128) << 64);
    (z as u64, (z >> 64) as u64)
}

fn _add_192x192(a0: u64, a1: u64, a2: u64, b0: u64, b1: u64, b2: u64) -> (u64, u64, u64) {
    let z0 = (a0 as u128) + (b0 as u128);
    let z1 = (a1 as u128) + (b1 as u128) + (z0 >> 64);
    let z2 = (a2 as u128) + (b2 as u128) + (z1 >> 64);
    (z0 as u64, z1 as u64, z2 as u64)
}

const fn _add64_with_carry(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + (b as u128) + (carry as u128);
    (ret as u64, (ret >> 64) as u64)
}

pub fn _safe_mul(a : u128, b : u128) -> u128 {
    let (x0, x1, x2) = _mul_128x64(a, (b >> 64) as u64);
    let (mut x0, mut x1, x2)  = _mul_reduce(x0, x1, x2);
    if x2 == 1 {
        let (t0, t1) = _sub_modulus(x0, x1);
        x0 = t0;
        x1 = t1;
    }
    let (y0, y1, y2) = _mul_128x64(a, b as u64);
    let (mut y1, carry) = _add64_with_carry(y1, x0, 0); // y = y + (x << 64)
    let (mut y2, y3) = _add64_with_carry(y2, x1, carry);
    if y3 == 1 {
        // if there was an overflow beyond 192 bits, subtract
        // modulus * 2^64 from the result to make sure it fits
        // into 192 bits; this can potentially replace the
        // previous overflow check (but needs to be proven)
        let (t0, t1) = _sub_modulus(y1, y2); // y = y - (m << 64)
        y1 = t0;
        y2 = t1;
    }

    let (mut z0, mut z1, z2) = _mul_reduce(y0, y1, y2); // z = y - (y >> 128) * m

    // make sure z is smaller than m
    if z2 == 1 || (z1 == (M >> 64) as u64 && z0 >= (M as u64)) {
        let (t0, t1) = _sub_modulus(z0, z1); // z = z - m
        z0 = t0;
        z1 = t1;
    }

    ((z1 as u128) << 64) + (z0 as u128)
}

fn _safe_inv(x: u128) -> u128 {
    if x == 0 {
        return 0;
    };

    // initialize v, a, u, and d variables
    let mut v = M;
    let (mut a0, mut a1, mut a2) = (0, 0, 0);
    let (mut u0, mut u1, mut u2) = if x & 1 == 1 {
        // u = x
        (x as u64, (x >> 64) as u64, 0)
    } else {
        // u = x + m
        _add_192x192(x as u64, (x >> 64) as u64, 0, M as u64, (M >> 64) as u64, 0)
    };
    // d = m - 1
    let (mut d0, mut d1, mut d2) = ((M as u64) - 1, (M >> 64) as u64, 0);

    // compute the inverse
    while v != 1 {
        while u2 > 0 || ((u0 as u128) + ((u1 as u128) << 64)) > v {
            // u > v
            // u = u - v
            let (t0, t1, t2) = _sub_192x192(u0, u1, u2, v as u64, (v >> 64) as u64, 0);
            u0 = t0;
            u1 = t1;
            u2 = t2;

            // d = d + a
            let (t0, t1, t2) = _add_192x192(d0, d1, d2, a0, a1, a2);
            d0 = t0;
            d1 = t1;
            d2 = t2;

            while u0 & 1 == 0 {
                if d0 & 1 == 1 {
                    // d = d + m
                    let (t0, t1, t2) = _add_192x192(d0, d1, d2, M as u64, (M >> 64) as u64, 0);
                    d0 = t0;
                    d1 = t1;
                    d2 = t2;
                }

                // u = u >> 1
                u0 = (u0 >> 1) | ((u1 & 1) << 63);
                u1 = (u1 >> 1) | ((u2 & 1) << 63);
                u2 >>= 1;

                // d = d >> 1
                d0 = (d0 >> 1) | ((d1 & 1) << 63);
                d1 = (d1 >> 1) | ((d2 & 1) << 63);
                d2 >>= 1;
            }
        }

        // v = v - u (u is less than v at this point)
        v -= (u0 as u128) + ((u1 as u128) << 64);

        // a = a + d
        let (t0, t1, t2) = _add_192x192(a0, a1, a2, d0, d1, d2);
        a0 = t0;
        a1 = t1;
        a2 = t2;

        while v & 1 == 0 {
            if a0 & 1 == 1 {
                // a = a + m
                let (t0, t1, t2) = _add_192x192(a0, a1, a2, M as u64, (M >> 64) as u64, 0);
                a0 = t0;
                a1 = t1;
                a2 = t2;
            }

            v >>= 1;

            // a = a >> 1
            a0 = (a0 >> 1) | ((a1 & 1) << 63);
            a1 = (a1 >> 1) | ((a2 & 1) << 63);
            a2 >>= 1;
        }
    }

    // a = a mod m
    let mut a = (a0 as u128) + ((a1 as u128) << 64);
    while a2 > 0 || a >= M {
        let (t0, t1, t2) = _sub_192x192(a0, a1, a2, M as u64, (M >> 64) as u64, 0);
        a0 = t0;
        a1 = t1;
        a2 = t2;
        a = (a0 as u128) + ((a1 as u128) << 64);
    }

    a
}

pub fn _xgcd(x: u128, y: u128) -> (u128, u128, u128) {
    let mut a = 0;
    let mut b = 1;
    let mut u = 1;
    let mut v = 0;

    let mut q;
    // Remove the assignment to r since it is not being used
    let mut r;
    let mut m;
    let mut n;

    let mut x = x;
    let mut y = y;

    while y != 0 {
        q = x / y;
        r = x % y;
        m = _safe_sub(a, q * u);
        n = _safe_sub(b,  q * v);
        x = y;
        y = r;
        a = u;
        b = v;
        u = m;
        v = n;
    }

    (x, a, b)
}

