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
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && !args[1].is_empty() {
        let set: Vec<i32> = args[1].split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        println!("{:?}", powerset(&set));
    } else {
        println!("{:?}", powerset(&[1, 2, 3]));
    }
}
