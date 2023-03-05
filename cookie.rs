use std::io;
fn main(){
    loop{ // Setting the loop so we can have as many guesses as we want
    println!("Guess what snack I am eating:"); 
    let mut guess = String::new(); // Allocating a blank new string into the guess variable, making it mutable
    io::stdin().read_line(&mut guess).expect("Error reading Line Data"); // Reading the line and storing it in guess
    let mut guess = guess.trim(); // Trimming the \n
    let current = "cookie"; // Setting the current snack
    if guess != current{ 
        println!("\x1B[31m\x1B[1mSorry, i am not eating {}. Try again.\x1B[0m",guess); 
    } // This asks if the guess is not cookie, output an error message with some ansi escape keys
    if guess == current{
        println!("\x1B[32m\x1B[1mThat is correct! I am eating a cookie!\x1B[0m");
        break()
    } // This asks if the guess is cookie, output a correct message, again with some ansi escape
}
}
