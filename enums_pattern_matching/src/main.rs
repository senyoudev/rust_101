//Defining an enum
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let coin = Coin::Penny;
    let coin_value = value_in_cents(coin);
    println!("The value of the coin is {}", coin_value);

    //Matching with Option<T>
    let five : Option<i8> = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("The value of five is {:?}", five);
    println!("The value of six is {:?}", six);
    println!("The value of none is {:?}", none);

    //The match Control Flow Operator
    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }

    //Matching with Option<T>
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => (),
    }

    // Concise control flow with if let
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("Three");
    }
    
    
}

//Matching with Option<T>
fn plus_one(x: Option<i8>) -> Option<i8> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}


//Matching with enums
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
