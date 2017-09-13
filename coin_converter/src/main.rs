/*
Author: Courtney E. Scott
Original Date: 09/09/2017

Converted to Rust: 09/13/2017

Task: Write a program that asks the user to enter a number of quarters, dimes, nickels and
pennies and then outputs the monetary value of the coins in the format of dollars and
remaining cents.*/

use std::io;


fn main() {

	println!("Please enter number of coins (ctrl+C to exit):");

	loop {
		println!("# of quarters:	");
		let mut quarters = String::new();
		io::stdin().read_line(&mut quarters).expect("Failed to read line");
		let quarters: u32 = match quarters.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("# of dimes:	");
		let mut dimes = String::new();
		io::stdin().read_line(&mut dimes).expect("Failed to read line");
		let dimes: u32 = match dimes.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("# of nickels:	");
		let mut nickels = String::new();
		io::stdin().read_line(&mut nickels).expect("Failed to read line");
		let nickels: u32 = match nickels.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("# of pennies:	");
		let mut pennies = String::new();
		io::stdin().read_line(&mut pennies).expect("Failed to read line");
		let pennies: u32 = match pennies.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		let sum: u32 = quarters * 25 + (dimes * 10) + (nickels * 5) + pennies; 
		let dollars: u32 = sum / 100;
		let cents: u32 = sum % 100; 

		println!("The total is {0}  dollars and {1} cents. \n", dollars, cents);
	}
}

