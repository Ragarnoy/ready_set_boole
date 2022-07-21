const ORDER: usize = 2_usize.pow(32);

fn reverse_map(n: f64) -> (u16, u16) {
    let mut new_n = n;
    for i in 1..ORDER {
        new_n = n >> 2;
    }
    todo!()
}

fn main() {
    println!("Hello, world!");
}
