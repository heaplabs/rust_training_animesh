fn main() {
    if_condition_success();
    if_condition_fail();
    if_else();
    if_else_expression();
    loop_exp();
}

fn if_condition_success() {

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_condition_fail() {

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
fn if_else() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_else_expression () {
    let condition = true;
    let number = if condition { 5 } else { 6 }; //note if we make the else { 6; } making it an statement rather than an expression the type of number will still be i32

    println!("The value of number is: {number}");
}
fn loop_exp() {
    let mut counter = 0;

    let result = 'count_to_10: loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}