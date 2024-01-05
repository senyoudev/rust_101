fn main() {
    // we have two types of errors in rust : recoverable and unrecoverable
    // recoverable errors are handled using Result<T, E> enum
    // unrecoverable errors are handled using panic! macro

    // 1 - Unrecoverable errors using panic! macro
    // panic!("crash and burn"); // this will print the error message and exit the program

    // 2 - Recoverable errors using Result<T, E> enum
    // Result<T, E> enum has two variants : Ok and Err
    // Ok variant indicates the operation was successful and contains the value returned
    // Err variant indicates the operation failed and contains information about the failure

    // 2.1 - Using match expression to handle Result<T, E> enum
    let result = divide(10, 0);
    match result {
        Ok(value) => println!("Result is {}", value),
        Err(error) => println!("Error is {}", error),
    }

    // 2.2 - Using if let expression to handle Result<T, E> enum
    if let Ok(value) = divide(10, 2) {
        println!("Result is {}", value);
    } else {
        println!("Error");
    }

    // 2.3 - Using unwrap_or_else method to handle Result<T, E> enum
    let result = divide(10, 0);
    let value = result.unwrap_or_else(|error| {
        println!("Error is {}", error);
        0
    });
    println!("Result is {}", value);

    // Custom Types for Validation
    // 3 - Using Result<T, E> enum to validate input
    let username = String::from("John");
    let password = String::from("password");
    let password2 = String::from("password2");
    let result = login(&username, &password);
    let result2 = login(&username, &password2);
    match result {
        Ok(_) => println!("Login successful"),
        Err(error) => println!("Login failed: {}", error),
    }

    match result2 {
        Ok(_) => println!("Login successful"),
        Err(error) => println!("Login failed: {}", error),
    }
}


fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(x / y)
}

fn login(username: &String, password: &String) -> Result<(), String> {
    if username == "John" && password == "password" {
        return Ok(());
    }
    Err(String::from("Invalid username or password"))
}