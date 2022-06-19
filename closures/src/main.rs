use core::hash::Hash;
use std::{collections::HashMap, thread, time::Duration};
struct Cacher<T, U>
where
    T: Fn(U) -> usize,
    U: Eq + Hash,
{
    calculation: T,
    value: HashMap<U, usize>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> usize,
    U: Eq + Hash + Clone,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    fn value(&mut self, arg: U) -> usize {
        if !self.value.contains_key(&arg) {
            let result = (self.calculation)(arg.clone());
            self.value.insert(arg, result);
            result
        } else {
            match self.value.get(&arg) {
                Some(num) => *num,
                None => panic!("Cached object should have a value"),
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn similated_expensive_calculation(intensity: usize) -> usize {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: usize, random_number: usize) {
    // let expensive_result = similated_expensive_calculation(intensity);

    // Refactoring similated_expensive_calculation
    // Let's us store the CLOSURE ini a variable rather than storing the RESULT from the function
    //            parameters go between the vertical pipes (param ==> nums)
    let expensive_closure = |num| -> usize {
        // brackets optional if single expression
        // Body of closure start
        println!("Calculating slowly with expensive_closure...");
        thread::sleep(Duration::from_secs(2));
        num
        // Body of closure stop
    }; // expensive_closure contains the definition of an anonymous function, not the returning value

    let mut expensive_result = Cacher::new(expensive_closure);

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! REmeber to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_result.value(intensity)
            );
        }
    }

    // Function vs explicit closure vs minimalistic closure
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1; // types can only be inferred after using it. Example next line
    add_one_v3(5);
    // add_one_v3(5.9); // NO WORK, cannot pass arg with different type from first inferred type

    // Capturing the environment with closures
    {
        let x = 4;
        let equal_to_x = |z| z == x; // captures `x` outside closure scope. Can't be done with functions
        let y = 4;

        fn equal_to_x_fn(z: i32) -> bool {
            //z == x // NO WORK: can't capture dynamic environment in a fn item
            false
        }

        let one = equal_to_x(y);
        // let two = equal_to_x_fn(y);
    }
}

#[test]
fn call_w_diff_vals() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

#[test]
fn call_w_string_slices() {
    let mut c = Cacher::new(|a| 5);

    let v1 = c.value("Hello");

    assert_eq!(v1, 5);
}
