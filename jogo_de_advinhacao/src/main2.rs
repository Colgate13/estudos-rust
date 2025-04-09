extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o numero!");

    // let numero_secrete = rand::random_range(1..100);
    let numero_secreto = rand::thread_rng().gen_range(1..101);

    // println!("O numero sercreto 'e {}", numero_secreto);

    loop {        
        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        // let palpite: u32 = palpite
        //                     .trim()
        //                     .parse()
        //                     .expect("Erro ao converter string em numero");
        let palpite: u32 = match palpite
                            .trim()
                            .parse() {
                                Ok(numero) => numero, //Ok e Err sao apenas enums
                                Err(_) => continue,
                            };


        println!("Voce disse: {}", palpite);

        match palpite.cmp(&numero_secreto) { // Quase um switch case foda
            Ordering::Less => println!("Muito baixo!"), // Ordering sao basicamente enum
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Voce acertou!");
                break;
            }
        }
    }
}
