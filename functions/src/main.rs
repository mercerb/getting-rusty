fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6;
    };

    println!("The value of number is: {}", number);
}

fn another_function() {
    println!("Another function.");
}

fn five() -> i32 {
    5  // in rust...if you put a semicolon here it won't work bc it becomes a statement
}