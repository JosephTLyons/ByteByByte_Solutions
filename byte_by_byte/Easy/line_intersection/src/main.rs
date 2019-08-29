// Given two lines on a Cartesian plane, write a function to determine whether or not the lines
// intersect.

// eg: intersect(2x + 5, 3x + 10) = True
// Solved

// This solution makes the assertion that two lines that are exactly the same are intersecting, with
// infinitely many points of intersection, this may not be correct to others, but its how I
// approached this solution.
fn lines_intersect(x_val_1: u8, constant_1: u8, x_val_2: u8, constant_2: u8) -> bool {
    if x_val_1 == x_val_2 && constant_1 != constant_2 {
        return false;
    }

    true
}

fn main() {
    let first_x_val = 2;
    let first_const = 5;
    let second_x_val = 3;
    let second_const = 10;

    println!(
        "{}x + {} and {}x + {} intersect: {}",
        first_x_val,
        first_const,
        second_x_val,
        second_const,
        lines_intersect(first_x_val, first_const, second_x_val, second_const)
    );
}
