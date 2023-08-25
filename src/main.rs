mod fourth_week;
mod third_week;
use fourth_week::shape;
use fourth_week::sum;
use fourth_week::traffic_light;
use third_week::sort;

use crate::fourth_week::traffic_light::LightTime;
fn main() {
    // week 3
    let arr1 = vec![2, 3, 5, 9, 1, 7];
    let sort_arr1 = sort::bubble_sort(arr1);
    println!("{:?}", sort_arr1);

    let arr2 = vec!["2", "3", "5", "9", "1", "7"];
    let sort_arr2 = sort::bubble_sort_paradigm(arr2);
    println!("{:?}", sort_arr2);

    // week 4
    // 1
    let red_light = traffic_light::TrafficLight::Red;
    let yellow_light = traffic_light::TrafficLight::Yellow;
    let green_light = traffic_light::TrafficLight::Green;
    println!("{}", red_light.time());
    println!("{}", yellow_light.time());
    println!("{}", green_light.time());

    // 2
    let nums = vec![1, 2, 3, 4, 5];
    let sum = sum::sum(&nums);
    println!("{:?}", sum);

    let nums = vec![1, 2, 3, 4, 5, u32::MAX];
    let sum = sum::sum(&nums);
    println!("{:?}", sum);

    // 3
    let circle = shape::Circle { radius: 1.0 };
    let square = shape::Square { length: 1.0 };
    let rectangle = shape::Rectangle {
        length: 1.0,
        width: 2.0,
    };

    println!("{}", shape::calculate_area(&circle));
    println!("{}", shape::calculate_area(&square));
    println!("{}", shape::calculate_area(&rectangle));
}
