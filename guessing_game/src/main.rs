use std::io;
// standard library (std) is a group of useful packages
// io package is used to get input from the user
use std::cmp::Ordering;
use rand::Rng;
// Rng is a trait that defines methods that random number generators implement

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop{
        println!("Please input your guess.");

        // only mutable variables can have their values changed
        // = means bind a value to variable
        // :: means new is an associated function of the String type
        let mut guess = String::new();
        // create new mutable variable named guess
        // here we bind a new, empty instance of a String to our mutable variable


        io::stdin()
            .read_line(&mut guess)
            // call readline method
            // & indicates that this argument is a reference (pointer)
            // references are immutable by default
            // &mut guess makes it mutable
            .expect("Failed to read line");
            // read_line returns a value of type io::Result
        
        let guess: u32 =match guess.trim().parse() { 
            Ok(num) => num,
            Err(_) => continue,
            // _ is a catchall value
        };
            // parse method parses a string into some kind of number

        println!("You guessed: {guess}");
        // {} is a placeholder for the value of the variable


        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // Ordering is an enum with variants Less, Greater, and Equal
            Ordering::Equal =>{ 
                // this means the user guessed correctly
                // the program will exit the loop and print "You win!"
                println!("You win!");
                break;
            }
        }
    }
}
