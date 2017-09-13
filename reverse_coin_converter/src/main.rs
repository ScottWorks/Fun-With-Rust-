/*
Author: Courtney E. Scott
Date: 09/09/2017

Converted to Rust: 09/13/2017

Task: Write a program that asks the user to enter an amount of money in the format of dollars and
remaining cents. The program should calculate and print the minimum number of coins
(quarters, dimes, nickels and pennies) that are equivalent to the given amount.*/

use std::io;


fn main() {
    println!("Welcome to reverse coin converter, enter the following commands, use ctrl+C to exit.");

    loop {
        println!("Please enter the total dollar amount:");
		let mut dollars = String::new();
		io::stdin().read_line(&mut dollars).expect("Failed to read line");
		let dollars: u32 = match dollars.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

        println!("Please enter the total change amount (cents):");
        let mut cents = String::new();
		io::stdin().read_line(&mut cents).expect("Failed to read line");
		let cents: u32 = match cents.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

	let sum: u32 = (dollars * 100) + cents; 
	let quarters: u32 = sum / 25;
	let dimes: u32 = (sum % 25) / 10;
	let nickels: u32 = ((sum % 25) % 10) / 5;
	let pennies: u32 = ((sum % 25) % 10) % 5;

    println!("{0} dollars and {1} cents are equivalents to...", dollars, cents);
    println!("{0} quarters, {1} dimes, {2} nickels, {3} pennies\n", quarters, dimes, nickels, pennies);
    }
}


