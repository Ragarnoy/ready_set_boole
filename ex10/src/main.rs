use std::mem;

fn inverse_hilbert_curve(mut x: u16, mut y: u16) -> u32 {
    let mut d: u32 = 0;
    let mut s: u32 = 1 << 15;
    let (mut rx, mut ry);

    while s as u16 > 0 {
        rx = (x & s as u16) != 0;
        ry = (y & s as u16) != 0;
        d += s.pow(2) * ((3 * rx as u32) ^ ry as u32);
        rotate(s as u16, &mut x, &mut y, rx, ry);
        s >>= 1;
    }
    d
}

fn rotate(n: u16, x: &mut u16, y: &mut u16, rx: bool, ry: bool) {
    if !ry {
        if rx {
            *x = n.wrapping_sub(1).wrapping_sub(*x);
            *y = n.wrapping_sub(1).wrapping_sub(*y);
        }
        mem::swap(x, y);
    }
}

fn map(x: u16, y: u16) -> f64 {
    inverse_hilbert_curve(x, y) as f64 / (2.0f64.powf(32.0) - 1.0)
}

fn main() {
    let (mut x, mut y) = (0u32, 0u32);
    while x < (u16::MAX - 1) as u32 && y < (u16::MAX - 1) as u32 {
        print!("{:-5} {:-5} => ", x, y);
        println!("{:.5}", map(x as u16, y as u16));
        x = x.wrapping_add(100);
        y = y.wrapping_add(100);
    }
    print!("{:-5} {:-5} => ", u16::MAX, 0);
    println!("{}", map(1, 1));
}
