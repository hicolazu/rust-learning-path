/* fn main() {
    let a_number = 10; // error: cannot assign twice to immutable variable `a_number`
    println!("the number is {}.", a_number);
    a_number = 15;
    println!("and now the number is {}.", a_number);
} */

fn main() {
    let mut a_number = 10; // notice the `mut` keyword
    println!("the number is {}.", a_number);
    a_number = 15;
    println!("and now the number is {}.", a_number);
  }