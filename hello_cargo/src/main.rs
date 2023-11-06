use std::io; //used to get input and give output
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = string::new();
    //input stdin
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
}