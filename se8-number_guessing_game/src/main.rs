use rand::Rng;
use std::io::stdin;

fn main() {
    let number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Enter your guess: ");

        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<u8>();
                match parsed {
                    Ok(guess) => {
                        if guess == 0 || guess > 100 {
                            println!("out of range");
                        } else if guess > number {
                            println!("guess is greater than number");
                            println!("number: {}", number);
                        } else if number > guess {
                            println!("number is greater than guess");
                            println!("number: {}", number);
                        } else {
                            println!("Correct");
                        }

                    },
                    Err(e) => {
                        println!("Error: {}",e);
                    }
                }

            },
            Err(_) => continue,
        }
    }
}
