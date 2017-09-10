/*
Source: https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html
Project: Guessing Game
Date: 09/10/2017

Task: 1)The first part of the program will ask for user input, process 
that input, and check that the input is in the expected form. To 
start, we’ll allow the player to input a guess.

2) Next, we need to generate a secret number that the user will try
to guess. The secret number should be different every time so the 
game is fun to play more than once. Let’s use a random number between 
1 and 100 so the game isn’t too difficult. Rust doesn’t yet include 
random number functionality in its standard library. However, the 
Rust team does provide a rand crate.

3) Now that we have user input and a random number, we can compare them. 
*/

extern crate rand;


use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main (){
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please Input your guess (1-100):  ");
        let mut guess = String::new(); // "guess" variable stores user input

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too Low!"),
            Ordering::Greater   => println!("Too High!"),
            Ordering::Equal     => { 
                println!("You Win!!!");
                break;
            } 
        }
    }
}