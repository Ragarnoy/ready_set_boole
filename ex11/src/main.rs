const ORDER: usize = 2_usize.pow(32);

fn hilbert_curve(n: u32) -> (u16, u16) {
   todo!()
}

fn reverse_map(n: f64) -> (u16, u16) {
    if !(0.0..=1.0).contains(&n) {
        panic!("n must be between 0.0 and 1.0");
    }
    hilbert_curve((n * (2.0f64.powf(32.0) - 1.0)) as u32)
}

fn main() {
    let result = reverse_map(0.5);
    println!("{} {}", result.0, result.1);
}
