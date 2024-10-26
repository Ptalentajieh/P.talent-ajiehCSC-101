fn main() {
	let Q1:f64 = 2.0;
	let A1:f64 =450000.0;
	// For Toshiba

	let Q2:f64 = 1.0;
	let A2:f64 = 1500000.0;
	// For Mac

	let Q3:f64 = 3.0;
	let A3:f64 = 750000.0;
	// For HP

	let Q4:f64 = 3.0;
	let A4:f64 = 2850000.0;
	// For Dell

	let Q5:f64 = 1.0;
	let A5:f64 = 250000.0;
	// For Acer

	let Qtys = Q1 + Q2 + Q3 + Q4 + Q5 
// Qtys stand for quantity sum
	let Qtyav = Qtys/5
	// Qtyav stand for quantity sum

	let Amts = A1 + A2 + A3 + A4 + A5
	// Amts stands for amount sum
	let Amtav = Amts/5
	// Amtav stands for Amount average
}