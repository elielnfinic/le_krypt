pub mod field;
pub mod field_element;
pub const DEFAULT_PRIME: i128 = 18446744069414584321; // Goldlilock prime 2^64 - 2^32 + 1

pub fn xgcd(a: i128, b: i128) -> (i128, i128, i128) {
    xgcd_field(a, b, DEFAULT_PRIME)
}

fn xgcd_field(a: i128, b: i128, p: i128) -> (i128, i128, i128) {
    let mut s0 = 1;
    let mut s1 = 0;
    let mut t0 = 0;
    let mut t1 = 1;
    let mut r0 = a;
    let mut r1 = b;

    while r1 != 0 {
        let q = r0 / r1;

        let r_temp = r1;
        r1 = (r0 - q * r1) % p;
        if r1 < 0 { r1 += p; }
        r0 = r_temp;

        let s_temp = s1;
        s1 = (s0 - q * s1) % p;
        if s1 < 0 { s1 += p; }
        s0 = s_temp;

        let t_temp = t1;
        t1 = (t0 - q * t1) % p;
        if t1 < 0 { t1 += p; }
        t0 = t_temp;
    }

    (r0, s0, t0)
}