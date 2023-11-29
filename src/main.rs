extern crate rand;
use std::io; //사용자의 입력을 받는다는 뜻임
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number bro!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guts");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed To read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        println!("Your guess is {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!!"),
            Ordering::Greater => println!("Too bigg!!!"),
            Ordering::Equal => {
                println!("you win!!!");
                break;
            }
        }
    }
}
