use std::io;

enum Choices {
    FahrenheitToCelsius = 1,
    CelsiusToFahrenheit = 2
}

fn fahrenhei_to_celsius() {}
fn celsius_to_fahrenhei() {}

fn main() {
    println!("Temperature Converter");

    loop {
        println!("Selecione o modo: ");
        println!("Voce desenha converter de Fahrenheit -> Celsius. (1)");
        println!("Voce desenha converter de Celsius -> Fahrenheit. (2)");

        let mut user_choice = String::new();
        match io::stdin()
            .read_line(&mut user_choice) {
                Ok(input) => input,
                Err(_) => {
                    println!("Input desconhecido, reiniciando fluxo");
                    continue;
                },
            };

        let user_choice: Choices = match user_choice
                                    .trim()
                                    .parse() { 
                                        Ok(number) => match number {
                                            1 => Choices::FahrenheitToCelsius,
                                            2 => Choices::CelsiusToFahrenheit,
                                            _ => {
                                                println!("Deu ruim");
                                                continue;
                                            }
                                        }, 
                                        Err(_) => continue 
                                    };

        println!("Insira o celsius");

        let mut user_celsius = String::new();
        match io::stdin()
                    .read_line(&mut user_celsius) {
                        Ok(celsius) => celsius,
                        Err(_) => {
                            println!("Deu ruim no input de celsius");
                            continue;
                        }
                    };
        let user_celsius: i64 = match user_celsius
                                        .trim()
                                        .parse() {
                                            Ok(number) => number,
                                            Err(_) => {
                                                println!("Deu ruim para converter pae celsius");
                                                continue;
                                            }
                                        };
                                        
        println!("Insira o fahrenhei");

        let mut user_fahrenhei = String::new();
        match io::stdin()
                    .read_line(&mut user_fahrenhei) {
                        Ok(celsius) => celsius,
                        Err(_) => {
                            println!("Deu ruim no input de fahrenhei");
                            continue;
                        }
                    };
        let user_fahrenhei: i64 = match user_fahrenhei
                                        .trim()
                                        .parse() {
                                            Ok(number) => number,
                                            Err(_) => {
                                                println!("Deu ruim para converter pae fahrenhei");
                                                continue;
                                            }
                                        };

        println!("Celsius {}, fahrenhei {}", user_celsius, user_fahrenhei);
        

        // match user_choice {
        //     Choices::CelsiusToFahrenheit => celsius_to_fahrenhei(),
        //     Choices::FahrenheitToCelsius => fahrenhei_to_celsius(),
        // }
            
    }
}
