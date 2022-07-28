use std::mem;

const ORDER: u32 = u16::MAX as u32;

fn hilbert_curve(mut n: u32) -> (u16, u16) {
    let mut order = 1u32;
    let positions = [(0, 0), (0, 1), (1, 1), (1, 0)];

    let (mut x, mut y): (u32, u32) = positions[n as usize & 0b11];
    n >>= 2;
    while order < ORDER {
        order <<= 1;

        match n & 0b11 {
            0b00 => {
                mem::swap(&mut x, &mut y);
            }
            0b01 => {
                y += order;
            }
            0b10 => {
                x += order;
                y += order;
            }
            0b11 => {
                let tmp = y;
                y = order.wrapping_sub(1).wrapping_sub(x);
                x = order.wrapping_sub(1).wrapping_sub(tmp); // swap + reverse
                x += order; // move
            }
            _ => {
                unreachable!()
            }
        }
        n >>= 2;
    }

    (x as u16, y as u16)
}

fn reverse_map(n: f64) -> (u16, u16) {
    if !(0.0..=1.0).contains(&n) {
        panic!("n must be between 0.0 and 1.0");
    }
    hilbert_curve((n * u32::MAX as f64) as u32)
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
