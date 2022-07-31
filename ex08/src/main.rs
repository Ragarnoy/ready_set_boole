fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    (0..2usize.pow(set.len() as u32))
        .map(|i| {
            set.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) & 1 == 1)
                .map(|(_, element)| *element)
                .collect()
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && !args[1].is_empty() {
        let set: Vec<i32> = args[1]
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        println!("{:?}", powerset(&set));
    } else {
        println!("{:?}", powerset(&[1, 2, 3]));
    }
}

#[cfg(test)]
mod test_powerset {
    use super::powerset;

    #[test]
    fn test_none() {
        assert_eq!(powerset(&[]), vec![vec![]]);
    }

    #[test]
    fn test_one() {
        assert_eq!(powerset(&[1]), vec![vec![], vec![1]]);
    }

    #[test]
    fn test_two() {
        assert_eq!(powerset(&[1, 2]), vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
        ]);
    }

    #[test]
    fn test_three() {
        assert_eq!(powerset(&[1, 2, 3]), vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ]);
    }
}