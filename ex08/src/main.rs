fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    (0..2usize.pow(set.len() as u32))
        .map(|i| {
            set.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| *element)
                .collect()
        })
        .collect()
}

fn main() {
    let res = powerset(&[1, 0, 3]);
    dbg!(0..2usize.pow(3));
    println!("{:?}", res);
}
