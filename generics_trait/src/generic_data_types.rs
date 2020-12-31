#![allow(unused)]
pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // maybe std::cmp::PartialOrd + Copy or Clone
    // this function return reference

    let mut _largest = &list[0];

    for it in list {
        if it > _largest {
            _largest = &it;
        }
    }

    &_largest
}

pub struct Point<T, U> {
    pub x: T,
    pub y: U,
}

impl<T, U> Point<T, U> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
