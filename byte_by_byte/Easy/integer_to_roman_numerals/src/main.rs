// Given an integer, write a function to return its roman numeral representation.
// Incorrect, not solved

// Mapping:
// 1    -> I
// 5    -> V
// 10   -> X
// 50   -> L
// 100  -> C
// 500  -> D
// 1000 -> M

fn add_all_of_one_type(int_value: &mut u32, roman_value: u32, symbol: String) -> String {
    let number_of_symbols = *int_value / roman_value;
    *int_value -= number_of_symbols * roman_value;
    symbol.repeat(number_of_symbols as usize)
}

fn int_to_roman_numeral(mut int_value: u32) -> String {
    let mut roman_string: String = String::new();

    roman_string += &add_all_of_one_type(&mut int_value, 1000, String::from("M"));
    roman_string += &add_all_of_one_type(&mut int_value, 500, String::from("D"));
    roman_string += &add_all_of_one_type(&mut int_value, 100, String::from("C"));
    roman_string += &add_all_of_one_type(&mut int_value, 50, String::from("L"));
    roman_string += &add_all_of_one_type(&mut int_value, 10, String::from("X"));
    roman_string += &add_all_of_one_type(&mut int_value, 5, String::from("V"));
    roman_string += &add_all_of_one_type(&mut int_value, 1, String::from("I"));

    roman_string
}

fn main() {
    println!("{}", int_to_roman_numeral(2019))
}
