fn main() {
    // println!("Hello, world!");
    // another_function(5);
    print_labeled_measurement(5, 'm');
}

// fn another_function(x: i32) {
//     // println!("Another function.");
//     println!("The value of x is: {x}");
// }

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The value is: {value}{unit_label}");
}