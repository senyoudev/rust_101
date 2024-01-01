//main function
// fn main() {
//     println!("Hello, world! From Senyoudev");
// }

//common programming concepts in rust



fn main() {
    //variables and mutability
        //variables are immutable by default

    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    //constants
        //constants are always immutable
        // constants can be declard in any scope

    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
    println!("3 hours in seconds : {}",THREE_HOURS_IN_SECONDS);

    //shadowing
    {
        let x = x * 2;
        println!("The value of x is : {}", x);
    }
    println!("The value of x is : {}", x);

    //data types

    //scalar types : single value types
        //integer types
            //signed integer types : i8, i16, i32, i64, i128, isize
            //unsigned integer types : u8, u16, u32, u64, u128, usize
        
        //floating point types : f32, f64

        //numeric operations

        //The Boolean Type

        //The Character Type
        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';
        println!("c : {}, z : {}, heart_eyed_cat : {}", c, z, heart_eyed_cat);

    //compound types : group multiple values into one type
        //The tuple type
        let tup : (i32, f64, u8) = (500, 6.4, 1);

            //destructuring
            let (x, y, z) = tup;
            println!("The value of x,y,and z are : {} {} {}", x,y,z);
        
            //accessing tuple elements
            let five_hundred = tup.0;
            let six_point_four = tup.1;
            let one = tup.2;
            println!("The value of five_hundred, six_point_four, and one are : {} {} {}", five_hundred, six_point_four, one);

        //The array type

        let  a = [1,2,3,4,5];
        let first = a[0];
        println!("The value of first is : {}", first);
        //a[1] = 10; throw error because array is immutable by default

    //Functions
    print_labeled_measurement(12, 'C');
    let sum = add_two_numbers(2, 3);
    println!("The sum of 2 and 3 is : {}", sum);

    //control flow
        // loops
            //loop
            let mut counter = 0;
            loop {
                counter += 1;
                println!("counter : {}", counter);
                if counter == 10 {
                    break;
                }
            }

            //while
            let mut number = 3;
            while number != 0 {
                println!("{}!", number);
                number -= 1;
            }
            println!("LIFTOFF!!!");

            //for
            let a = [10, 20, 30, 40, 50];
            for element in a.iter() {
                println!("the value is : {}", element);
            }

            for number in (1..4).rev() {
                println!("{}!", number);
            }
            println!("LIFTOFF!!!");


}

//function without return value
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//function with return value
fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

/*
    Test comment multi lines and it is working
*/