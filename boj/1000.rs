use std::io;

fn main(){
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();

  let input_numbers:Vec<i32> = input.split_whitespace().map(|item| item.parse().unwrap()).collect();
  print!("{}", input_numbers[0] + input_numbers[1]);
}