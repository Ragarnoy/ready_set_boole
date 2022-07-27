use std::mem;

const ORDER: usize = 2_usize.pow(16);

fn hilbert_curve(mut n: u32) -> (u16, u16) {
    let mut order = ORDER as u32;
    let positions = [(0, 0), (1, 0), (1, 1), (0, 1)];

    let (mut x, mut y): (u32, u32) = positions[n as usize & 3];
    n >>= 2;
    while order > 0 {
        order >>= 1;

        match n & 3 {
            0 => {
                mem::swap(&mut x, &mut y);
            }
            1 => {
                y += order;
            }
            2 => {
                x += order;
                y += order;
            }
            3 => {
                let tmp = y;
                y = order.wrapping_sub(1).wrapping_sub(x);
                x = order.wrapping_sub(1).wrapping_sub(tmp);
            }
            _ => {}
        }
    }

    (x as u16, y as u16)
}

fn reverse_map(n: f64) -> (u16, u16) {
    if !(0.0..=1.0).contains(&n) {
        panic!("n must be between 0.0 and 1.0");
    }
    hilbert_curve((n * (2.0f64.powf(32.0) - 1.0)) as u32)
}

fn main() {
    let mut f = 0.0;
    while f < 1.0 {
        let (x, y) = reverse_map(f);
        println!("{} => {} {}", f, x, y);
        f += 0.01;
    }
    // let result = reverse_map(0.5);
    // println!("{} {}", result.0, result.1);
}
