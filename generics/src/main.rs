// In function definitions
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//        if item > largest {
//            largest = item;
//        }
//     }
//     largest
// }

// Define Display trait
use std::fmt::Display;
// define PartialOrd trait
use std::cmp::PartialOrd;

// In struct definitions
struct Point<T> {
    x: T,
    y: T,
}

// In enum definitions
enum Option<T> {
    Some(T),
    None,
}

// In method definitions
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//Implementing a trait 
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//Implementing a trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Defining a trait
pub trait Summary {
    fn summarize(&self) -> String;
}


// Pair struct with generic type parameters
struct Pair<T> {
    x: T,
    y: T,
}



// Conditionally implement methods on a generic type depending on trait bounds
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


fn main() {
    // Generics in Rust
    // Generics are abstract stand-ins for concrete types or other properties.
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    // this will not work because the compiler doesn't know how to compare the values

    
    let point = Point { x: 5, y: 10 };
    println!("p.x = {}", point.x); // 5

    println!("p.x = {}", point.x()); // 5

        
    // Traits: Defining Shared Behavior
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        content: String::from("The Pittsburgh Penguins once again are the best"),
        author: String::from("Iceburgh"),
    };
    println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    // Lifetimes: Ensuring One String Lives as Long as Another
    let r ; 
    {
        let x = 5;
        r = &x;
    }
    //println!("r: {}", r);  r is a reference to x, but x is out of scope

    let x = 5;
    let r = x; // r is a copy of x
    println!("x: {}, r: {}", x, r); // x and r are both 5


    // Generical lifetime annotation
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    // the lifetime of result will be the smaller of the lifetimes of the references passed in.


    // Lifetime Elision
    // The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations.
    // 1. Each parameter that is a reference gets its own lifetime parameter.
    // 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
    // 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len(){
        x
    } else {
        y
    }
}