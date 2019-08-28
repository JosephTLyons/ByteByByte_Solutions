// Given two integers, write a function that swaps them without using any temporary variables
// Solved

fn swap(x: &mut i32, y: &mut i32) {
    *x += *y;
    *y = *x - *y;
    *x -= *y;
}

fn main() {
    let mut x = 5;
    let mut y = 7;

    swap(&mut x, &mut y);

    println!("x = {}", x);
    println!("y = {}", y);
}
