use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing the number game!");

    let random_number=rand::thread_rng().gen(1..=100);

    loop{
        println!("Guess the number and hit enter")
        let mut guessed_number = string::new();
        io::stdin().read_line(&mut guessed_number).expect("Fail to read line");
        let guessed_number: u32 = match guessed_number.trim().parse(){
            ok(num)=>num;
            err(_)=>continue;
        };

        match guessed_number.cmp(&random_number){
            Ordering::Less=>println!("Too small");
            Ordering::Greater=>println!("Too Big");
            Ordering::Equal=>{
                println("You guessed Correct");
                break;
            }
        }
    }
}
