
fn adder(a: u32, b: u32) -> u32 {
    let uncommon = a ^ b;
    let common = a & b;

    if common == 0 { return uncommon; }
    adder(uncommon, common << 1)
}


fn multiplier(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 { return 0; }
    else if a == 1 { return b; }
    else if b == 1 { return a; }

    let mut res = 0;
    let (mut min, mut max) = (a.max(b), b.min(a));
    while min != 0 {
        if (min & 01) != 0 {
            res = adder(res, max);
        }
        max <<= 1;
        min >>= 1;
    }
    res
}

fn main() {
    let mut a = 800u32;
    for b in 1..500 {
        println!("{} * {} = {}", a, b, multiplier(a, b));
        println!("{} * {} = {}\n", a, b, a * b);
        a -= 1;
    }
}

#[cfg(test)]
mod test_mult {
    use super::*;

    #[test]
    fn test_mult() {
        let mut a = 800;

        for b in 1..500 {
            assert_eq!(multiplier(a, b), a * b);
            a -= 1;
        }
    }
}