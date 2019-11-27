pub fn run() {
 let mut hello = String::from("yo duder");

 println!("length: {}", hello.len());

 println!("capacity: {}", hello.capacity());

 println!("is_empty: {}", hello.is_empty());

 println!("contains yo: {}", hello.contains("yo"));

 println!("replace yo: {}", hello.replace("yo","greetings"));

 println!("hello: {}", hello);

 for word in hello.split_whitespace() {
   println!("word: {}", word)
 }

 let mut s = String::with_capacity(10);
 s.push('a');
 s.push('b');
 println!("ab == {}", s);

 hello.push('i');

 hello.push_str("no");


 assert_eq!(2, s.len());

 println!("{}", hello);
}