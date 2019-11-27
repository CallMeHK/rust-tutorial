pub fn run() {
let age = 23;
let check_id = false;
let knows_person = true;

if age>= 21 && check_id || knows_person{
  println!("what you wanna drink?");
} else if age < 21 && check_id{
  println!("sorry ya gotta leave.");
} else {
  println!("gimme yo id");
}

let is_of_age = if age>=21 { true } else { false };

println!("is of age: {}", is_of_age);
}

