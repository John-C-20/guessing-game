use std::io;
// allows us to take user input using the `io` library 

fn main() {
    println!("Guess the number!");
    println!("Pleae inpurt your guess.");
    let mut guess = String::new(); // :: denotes that new() is a associated function/static method/class method of String class 

    io::stdin().read_line(&mut guess) // io::stdin() is an asssociated function of io that returns a handle to the standard input of terminal
                                      // .read_line() is a method (instance method) of io::stdin() 
        .expect("Failed to read line");


    
}
