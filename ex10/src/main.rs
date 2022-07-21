use std::mem;

fn map(x: u16, y: u16) -> f64 {
    let mut d: f64 = 0.0;
    let mut s: f64 = 2.0f64.powf(16.0) / 2.0;
    let (mut x, mut y) = (x, y);
    let (mut rx, mut ry);

    while s > 0.0 {
        rx = if (x & s as u16) > 0 { 1 } else { 0 };
        ry = if (y & s as u16) > 0 { 1 } else { 0 };
        d += s as f64 * s as f64 * ((3 * rx) ^ ry) as f64;
        rotate(s as u16, &mut x, &mut y, rx, ry);
        s /= 2.0
    }

    d / (2.0f64.powf(32.0) -1.0)
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
    println!("{}", map(u16::MAX, 0));
}
