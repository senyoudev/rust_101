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


}