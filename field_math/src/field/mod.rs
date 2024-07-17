pub mod field;
pub mod field_element;

pub fn _xgcd(x : u128, y : u128) -> (u128, u128, u128){
    let mut a = 0;
    let mut b = 1;
    let mut u = 1;
    let mut v = 0;

    let mut q = 0;
    let mut r = 0;
    let mut m = 0;
    let mut n = 0;

    let mut x = x;
    let mut y = y;

    while y != 0{
        q = x / y;
        r = x % y;
        m = a - q * u;
        n = b - q * v;
        x = y;
        y = r;
        a = u;
        b = v;
        u = m;
        v = n;
    }

    (x, a, b)
}