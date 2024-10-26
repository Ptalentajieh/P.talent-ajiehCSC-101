fn main() {
	let P:f64 = 510000.0;
	let R:f64 = 5.0;
	let N:f64 = 3.0;

	let A = P * (1-(R/100.0))^N
	println!("Amount is {}", A);
}