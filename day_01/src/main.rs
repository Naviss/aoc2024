use std::{io, iter::zip, vec};

fn solve_one(list1: &mut Vec<u32>, list2: &mut Vec<u32>) -> u32 {
    list1.sort();
    list2.sort();

    let mut distance = vec![0; list1.len()];
    let mut i = 0;
    let it = zip(list1, list2);

    for (p1, p2) in it {
        distance[i] = p2.abs_diff(*p1);
        i += 1;
    }

    distance.iter().sum()
}

fn solve_two(list1: &mut Vec<u32>, list2: &mut Vec<u32>) -> u32 {
    let mut ocurance: Vec<u32> = Vec::new();
    for n in list1.iter() {
        ocurance.push(
            list2
                .iter()
                .filter_map(|x| if x == n { Some(1) } else { None })
                .sum::<u32>()
                * n,
        )
    }
    ocurance.iter().sum()
}

fn main() -> io::Result<()> {
    let lines = io::stdin().lines();
    let mut l1: Vec<u32> = Vec::new();
    let mut l2: Vec<u32> = Vec::new();

    for l in lines {
        let line = l.unwrap();
        let ids: Vec<&str> = line.split("   ").collect();
        let ids_len = ids.len();
        assert_eq!(ids_len, 2 as usize);
        l1.push(ids[0].parse::<u32>().unwrap());
        l2.push(ids[1].parse::<u32>().unwrap());
    }

    let res = solve_one(&mut l1, &mut l2);
    let res2 = solve_two(&mut l1, &mut l2);
    println!("Res: {} - {}", res, res2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_one() {
        let mut l1: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let mut l2: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        let res = solve_one(&mut l1, &mut l2);
        assert_eq!(res, 11);
    }

    #[test]
    fn test_solve_two() {
        let mut l1: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let mut l2: Vec<u32> = vec![4, 3, 5, 3, 9, 3];
        let res = solve_two(&mut l1, &mut l2);
        assert_eq!(res, 31);
    }
}
