fn main() {
    println!("Hello, world!");

    another_function();

    another_function_v2(five());
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn another_function_v2(x: i32) {
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn five() -> i32 {
    5 //not adding a semicolon or else this will turn into a statment from an expression and will not return a value 
}
