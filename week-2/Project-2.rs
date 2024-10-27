fn main() {
	let q1:f64 = 2.0;
	let a1:f64 =450000.0;
	// For Toshiba

	let q2:f64 = 1.0;
	let a2:f64 = 1500000.0;
	// For Mac

	let q3:f64 = 3.0;
	let a3:f64 = 750000.0;
	// For HP
    let n:f64 = 5.0;
    // Total number of variables 
	

	let q4:f64 = 3.0;
	  let a4:f64 = 2850000.0;
	  // For Dell

	let q5:f64 = 1.0;
	let a5:f64 = 250000.0;
	// For Acer

	let qtys = q1 + q2 + q3 + q4 + q5;
    // Qtys stand for quantity sum
	let qtyav = qtys/n;
	// Qtyav stand for quantity sum

	let amts = a1 + a2 + a3 + a4 + a5;
	// Amts stands for amount sum
	let amtav = amts/n;
	// Amtav stands for Amount average

    print!("from the Data presented from the table");
	println!("The sum of Qty (Quantity) is {}",qtys);
	println!("The average of Qty (Quantity) is {}",qtyav);


	println!("The aggregate sum of Amount is {}",amts);
	println!("The mean value (Average value) of Amount is {}",amtav);


}