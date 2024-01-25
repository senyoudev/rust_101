use std::thread;
use std::time;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(time::Duration::from_secs(2));
    intensity
}

fn main() {
    
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

struct Cacher <T> 
where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

// FnOnce, FnMut, Fn
// FnOnce: consumes the variables it captures from its enclosing scope, known as the closure’s environment.
// Example
// let x = vec![1, 2, 3];
// let equal_to_x = move |z| z == x;
// println!("can't use x here: {:?}", x);
// FnMut: can change the environment because it mutably borrows values.
// Example


// Fn: borrows values from the environment immutably.

impl <T> Cacher <T>
where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher <T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(time::Duration::from_secs(2));
        num
    });
    
    if intensity < 25 {
        println!("Today, do {} pushups!", 
        cached_result.value(intensity));
        println!("Next, do {} situps!", 
        cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", 
            cached_result.value(intensity));
        }
    }
}