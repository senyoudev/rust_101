pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}




fn main() {
   //iterators
    let v = vec![1, 2, 3];
    let v_iter = v.iter();
    for val in v_iter {
        println!("Got: {}", val);
    }
}

struct Shoe {
    size: u32,
    style: String,
}

// closures that capture their environment
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter() // into_iter() takes ownership of the vector
        .filter(|s| s.size == shoe_size) // filter() creates an iterator that uses the closure to figure out what to keep
        .collect() // collect() consumes the iterator and collects the values into a new data structure
}


#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter(); // v1_iter is an iterator that holds a reference to each item in v1

    assert_eq!(v1_iter.next(), Some(&1)); // next() returns the next value, wrapped in Some, when there is a value
    assert_eq!(v1_iter.next(), Some(&2)); // next() returns None when iteration is over
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v2: Vec<i32> = vec![1, 2, 3];
    let v2_iter = v2.iter();
    let total: i32 = v2_iter.sum(); // sum() is a method on the Iterator trait
    assert_eq!(total, 6);
}

//methods that produce other iterators
#[test]
fn iterator_map() {
    let v3: Vec<i32> = vec![1, 2, 3];
    let v3_iter = v3.iter();
    let v4: Vec<_> = v3_iter.map(|x| x + 1).collect(); // map() is a method on the Iterator trait
    assert_eq!(v4, vec![2, 3, 4]);
}


#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") }
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(in_my_size, vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 10, style: String::from("boot") }
    ]);
}
