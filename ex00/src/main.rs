fn adder(a: u32, b: u32) -> u32 {
    let uncommon = a ^ b;
    let common = a & b;

    if common == 0 {
        return uncommon;
    }
    adder(uncommon, common << 1)
}

fn main() {
    let mut a = u32::MAX;
    for b in 0..u32::MAX {
        println!("{} + {} = {}", a, b, adder(a, b));
        a -= 1;
    }
}

#[cfg(test)]
mod test_adder {
    use super::*;

    #[test]
    fn test_add_0() {
        for i in 0..20 {
            assert_eq!(adder(0, i), i)
        }
    }

    #[test]
    fn test_add() {
        let mut j = 201;

        for i in 0..200 {
            assert_eq!(adder(j, i), j + i);
            j -= 1;
        }
    }
}
