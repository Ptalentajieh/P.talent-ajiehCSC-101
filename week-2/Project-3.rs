fn main() {
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
	let n:i32 = 3;

	let a = p * (1.0-(r/100.0)).powi(n);
	println!("Amount after 3yrs is {}",a);
}