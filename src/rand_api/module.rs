use rand::{Rng};

pub fn call() {
    simple_rand_call();    
}

fn simple_rand_call() {
    
    // prepare a non-deterministic random number generator:
    let mut rng = rand::thread_rng();
    println!("{}", rng.gen::<i32>()); // prints an unknown value

}