use rand::Rng;
use std::cmp::Ordering;
use std::io; // Import IO library for read input and print result

pub fn start() {
    println!("Guess the number!"); // Macro, function name ended up with !

    // Generate a random number between 1 .. 100
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("Secret {}", secret_number);

    loop {
        println!("Please input your guess!");
        // Create new variable "guess", and use "mut" to mark it as mutable, as by default all rust variable is immutable
        // Invoke the new function in String library, which return new instance of String
        // Mutable for IO to append input into it
        let mut guess = String::new();

        // Call imported IO library function
        // & = the argument is pass by reference, allow multi-part of the code to access "guess", without needed to copy data into memory multiple time
        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        // Tell rust we wish to parse the string into u32 type by putting :u32
        // let guess: u32 = guess.trim().parse().expect("Please type a number");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        // Native keyword in rust, which allow you to have effect of if else using "arm" pattern
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
