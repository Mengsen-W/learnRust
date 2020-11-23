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
        let mut flag: &i32 = &i32::MAX;
        if let Some(value) = num.last() {
            flag = &value[1]
        };
        if num.is_empty() || *flag < left {
            num.push(vec![left, right]);
        } else {
            if let Some(last) = num.last_mut() {
                if let Some(val) = last.last_mut() {
                    let value = cmp::min(*val, right);
                    *val = value;
                }
            }
        }
    }

    num.len() as i32
}
