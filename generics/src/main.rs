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

        


}
