fn main() {
    loop_through_array_for();
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End Count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number!= 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn loop_through_array() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }
}

fn loop_through_array_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {}", element);
    }
}