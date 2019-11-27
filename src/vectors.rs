use std::mem;

pub fn run() {
 let mut numbers: Vec<i32> = vec![1,2,3,4,5];
 numbers[2] = 20;
 numbers.push(5);
 numbers.push(5);
 numbers.pop();

 println!("{:?}", numbers);
 println!("one val: {}", numbers[0]);

 println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

 let slice: &[i32] = &numbers[0..2];
 println!("Slice: {:?}", slice);

  for x in numbers.iter() {
    println!("#{}", x);
  }

  for x in numbers.iter_mut() {
    *x = *x * 2;
  }

  println!("Mutated: {:?}", numbers);
}