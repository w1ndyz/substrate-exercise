pub mod three_week;
fn main() {
    let arr1 = vec![2, 3, 5, 9, 1, 7];
    let sort_arr1 = three_week::bubble_sort(arr1);
    println!("{:?}", sort_arr1);

    let arr2 = vec!["2", "3", "5", "9", "1", "7"];
    let sort_arr2 = three_week::bubble_sort_paradigm(arr2);
    println!("{:?}", sort_arr2);
}
