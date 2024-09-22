use rand :: {Rng};
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("-----------------------------------");
    println!("Welcome to the guessing game");
    println!("-----------------------------------");
    let secret_number = rand::thread_rng().gen_range(1..21);
    // println!("Secret Number is : {}",secret_number);
    loop {
        println!("Please input your guess : ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // println!("you guessed : {}",guess);
        let guess:u32 = guess.trim().parse().expect("Type an integer");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {println!("You Win !");break;},
        }
    }  
}
