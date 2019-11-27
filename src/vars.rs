//variables are immutable by default

pub fn run() {
  let name = "Ty";
  let mut age = 29;

  println!("Im {} and my age is {}", name, age);

  age = 30;

  println!("Im {} and my age is now {}", name, age);

  const ID: i32 = 001;
  println!("ID: {}", ID);

  let ( my_name, my_age ) = ("Ty", 29);
  println!("{} and {}", my_name, my_age);

}