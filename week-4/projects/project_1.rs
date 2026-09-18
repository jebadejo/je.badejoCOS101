// finding quadratic roots

 use std::io;

 fn main() {
 	
 	println!("Enter value of a:");
 	let mut input1 = String::new();
 	io::stdin().read_line(&mut input1).expect("Not a valid input");
 	let a:f64 = input1.trim().parse().expect("Not a valid number");

 	println!("Enter value of b:");
 	let mut input2 = String::new();
 	io::stdin().read_line(&mut input2).expect("Not a valid input");
 	let b:f64 = input2.trim().parse().expect("Not a valid number");

 	println!("Enter value of c:");
 	let mut input3 = String::new();
 	io::stdin().read_line(&mut input3).expect("Not a valid input");
 	let c:f64 = input3.trim().parse().expect("Not a valid number");

 	let d = b*b - 4.0 * a * c;

 	if d > 0.0{
 		println!("Two distinct roots");
 	} else if d == 0.0{
 		println!("Exactly one real root");
 	} else {
 		println!("No real roots");
 	}
 }