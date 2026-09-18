// Incentive Calculator

use std::io;

fn main() {
	println!("Enter experience level(Experienced or Not experienced):");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Not a valid input");
	let input1 = input1.trim();

	println!("Enter age:");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("Not a valid input");
	let age:f64 = input2.trim().parse().expect("Not a valid number");

	 if input1 == "Not experienced"{
	println!("Annual incentive: N100,000");
    } else if age >= 40.0{
    	println!("Annual incentive: N1,560,000");
    } else if age >= 30.0 && age <=39.0 {
    	println!("Annual incentive: N1,480,000");
    } else if age < 28.0 {
    	println!("Annual incentive: N1,300,000");
    } else {
    	println!("Not eligible!");
    }
}