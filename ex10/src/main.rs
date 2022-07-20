use std::mem;

fn map(x: u16, y: u16) -> f64 {
    let mut d: f64 = 0.0;
    let mut s = 2u16.pow(16) / 2;
    let (mut x, mut y) = (x, y);
    let (mut rx, mut ry);

    while s > 0 {
        rx = if (x & s) > 0 { 1 } else { 0 };
        ry = if (y & s) > 0 { 1 } else { 0 };
        d += s as f64 * s as f64 * ((3 * rx) ^ ry) as f64;
        rotate(s, &mut x, &mut y, rx, ry);
        s /= 2
    }

    d
}

fn rotate(n: u16, x: &mut u16, y: &mut u16, rx: u16, ry: u16) {
    if ry == 0 {
        if rx == 1 {
            *x = n.wrapping_sub(1).wrapping_sub(*x);
            *y = n.wrapping_sub(1).wrapping_sub(*y);
        }

        mem::swap(x, y);
    }
}

fn main() {
    println!("{}", map(20, 40));
}
