use std::io;

fn main() {
    println!("Please write a string to check wether its a palindrome:");
    let mut input = String::new();

    // reads input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    // remove newline
    input.pop();

    // remove all occurences of spaces
    let mut string = input.replace(" ", "");

    if string == "" {
        println!("The string is empty, and therefore not a palindrome");
        std::process::exit(0);
    }
    // convert to lowercase
    string = string.to_lowercase();

    // check wether its a palindrome
    let mut s: usize = 0; // the start of the string, left-to-center counter
    let mut l: usize = string.len() - 1; // the end of the string, right-to-center counter

    while l > s {
        if string.chars().nth(s).unwrap() != string.chars().nth(l).unwrap() {
            println!("{input} is not a palindrome.");
            std::process::exit(0);
        }
        // increment
        l = l - 1;
        s = s + 1;
    }

    // if everything is well, its a palindrome
    println!("{} is a palindrome!", input);
    std::process::exit(0);
}
