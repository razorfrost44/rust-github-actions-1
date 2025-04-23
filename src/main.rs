pub mod simple_addition;
use simple_addition::add_two;

fn run_addition() {
    let a = 5;
    let b = 10;
    let sum = add_two(a, b);
    println!("The sum of {} and {} is {}", a, b, sum);
}

fn main() {
    println!("START");
    println!();

    run_addition();

    println!();
    println!("END");
}
