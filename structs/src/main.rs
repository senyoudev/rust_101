
//Defining a Struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32
}

// Method Syntax
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Tuple vs Struct
// Tuple and Struct are similar in that they let you name and combine multiple values into a single
// value. However, tuples are useful when you want to combine a number of values of different types
// into one compound type. For example, a tuple might contain an i32 and a String. Or you might
// want to return the result of a calculation from a function as a tuple.

// Tuple Structs without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-Like Structs Without Any Fields
struct UnitLikeStructs;


fn main() {
    // Instantiating Structs
    let user1 = User {
        username : String::from("senyoudev"),
        email : String::from("younesmeskafe2018@gmail.com"),
        sign_in_count : 1,
        active : true
    };
     // Accessing Struct Fields
     println!("User1 username is {}", user1.username);
     println!("User1 email is {}", user1.email);
     println!("User1 sign_in_count is {}", user1.sign_in_count);
     println!("User1 active is {}", user1.active);
    // Creating Instances From Other Instances With Struct Update Syntax
    let user2 = User {
        username : user1.username,
        email : user1.email,
        sign_in_count : user1.sign_in_count,
        active : user1.active
    };
    
   // Tuple Structs without named fields Instantiation
    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    // Unit-Like Structs Instantiation
    let unit_like_struct = UnitLikeStructs;

    // Instanciate Rectangle Struct
    let rect1 = Rectangle {
        width : 30,
        height : 50
    };
    println!("The rectangle is {:#?}", rect1);
    println!("The area of the rectangle is {}", rect1.area());


}
