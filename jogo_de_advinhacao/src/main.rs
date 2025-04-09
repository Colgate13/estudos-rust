extern crate rand;

use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Advinhe o numero!");
    let secret_number = rand::random_range(1..101);

    loop {
        println!("Digite o seu user_number.");

        let mut user_number = String::new();

        io::stdin()
            .read_line(&mut user_number)
            .expect("Falha ao ler entrada");

        let user_number: u32 = match user_number
                            .trim()
                            .parse() {
                                Ok(number) => number,
                                Err(_) => continue,
                            };

        println!("Voce disse: {}", user_number);

        match user_number.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Voce acertou!");
                break;
            }
        }
    }
}
