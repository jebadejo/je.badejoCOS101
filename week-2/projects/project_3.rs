fn main() {
	let p:f64 = 210000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//depreciation
	let a = p * (1.0 - (r / 100.0)).powf(n);
	println!("Value after depreciation is {}", a);
}