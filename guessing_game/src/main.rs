use std::io;

fn main() {
  println!("guessing program");

  println!("please input guess number");

  let mut guess = String::new();
  io::stdin().read_line(&mut guess).expect("failed read number.");

  println!("the guess is {}", guess);
}
