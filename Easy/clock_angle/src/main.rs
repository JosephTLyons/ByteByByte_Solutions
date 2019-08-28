// Given two integers, an hour and a minute, write a function to calculate the angle between the two
// hands on a clock representing that time.
// Solved

// Gets the angle of the hand when it points to a min marker, with respect to 0 minutes (12 O'Clock)
fn get_angle_of_minute_marker(min_marker: f32) -> f32 {
    if min_marker == 0.0 {
        return 0.0;
    }

    (min_marker as f32 / 60.0) * 360.0
}

fn convert_hours_to_min_marker(hours: f32, mins: u8) -> f32 {
    let mut hours_in_mins = (hours * 5.0) + (f32::from(mins) / 60.0) * 5.0;
    hours_in_mins %= 60.0;
    hours_in_mins
}

fn get_angle_between_hands(hours: f32, mins: u8) -> f32 {
    let angle_of_min_hand = get_angle_of_minute_marker(mins.into());
    let angle_of_hour_hand = get_angle_of_minute_marker(convert_hours_to_min_marker(hours, mins));
    let mut angle_between_hands = (angle_of_min_hand - angle_of_hour_hand).abs();

    if angle_between_hands > 180.0 {
        angle_between_hands -= 180.0
    }

    angle_between_hands
}

fn main() {
    println!("{}", get_angle_between_hands(12.0, 15));
}
