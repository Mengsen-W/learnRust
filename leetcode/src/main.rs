fn main() {
    println!("Hello, world!");
}

pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    use std::cmp;
    let mut num: Vec<Vec<i32>> = vec![];
    points.sort();

    for i in points.iter() {
        let left = i[0];
        let right = i[1];

        if num.is_empty() || left > *num.last().and_then(|v| v.last()).unwrap() {
            num.push(vec![left, right]);
        } else {
            let mut val = num.last().and_then(|v| v.last()).unwrap();
            val = cmp::min(&right, val);
        }
    }

    num.len() as i32
}

fn test() {
    let num = vec![vec![1, 2], vec![3, 4]];
    if Some(&3)
        >= match num.last() {
            Some(last) => last.last(),
            None => return,
        }
    {
        println!("hello");
    }
}
