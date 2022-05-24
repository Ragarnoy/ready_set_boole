fn gray_code(a: u32) -> u32 {
    a ^ (a >> 1)
}

fn main() {
    for i in 0..20 {
        println!("{:b} to {:b}", i, gray_code(i));
    }
}

#[cfg(test)]
mod test_gray_code {
    use super::*;

    #[test]
    fn test_gray_code() {
        assert_eq!(gray_code(1), 1);
        assert_eq!(gray_code(2), 3);
        assert_eq!(gray_code(3), 2);
        assert_eq!(gray_code(4), 6);
        assert_eq!(gray_code(5), 7);
        assert_eq!(gray_code(6), 5);
    }
}
