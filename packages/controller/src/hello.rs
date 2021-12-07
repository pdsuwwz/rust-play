pub fn print_some_str() {
  let num1 = f32::from(100_i8);
  println!(
      "test print number: {}",
      num1,
  );

  println!(
      "test print number: {num1}",
      num1=f64::from(10086)
  );

  println!("{}", five());
}

fn five() -> i32 {
  let x: i32 = 5;
  return x;
}
