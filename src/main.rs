use std::collections::HashMap;

fn y(t: i32) -> i32 {
    return 2 * (t^2);
}

fn x(t: i32) -> i32 {
    (2*t) + 2;
}

fn main() {
    let range = 0..5;
    let mut vec: Vec<i32> = Vec::new();
    let mut vec = vec![];
    for t in range {
        let coord = [x(t), y(t)];
        vec.push(coord);
    }
    for coord in vec {
        println!("{:?}", coord);
    }
}