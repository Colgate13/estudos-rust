use std::io;

enum Choices {
    FahrenheitToCelsius = 1,
    CelsiusToFahrenheit = 2
}

fn fahrenhei_to_celsius(fahrenheit: f64) -> f64 {
    // C = (F - 32) * 5 / 9
    (fahrenheit - 32.0) *  5.0 / 9.0
}

fn celsius_to_fahrenhei(celsius: f64) -> f64 {
    // F = (C * 9/5) + 32
    (celsius * 9.0 / 5.0) + 32.0
}

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

        println!("Insira o valor a ser convertido");

        let mut user_value = String::new();
        match io::stdin()
                    .read_line(&mut user_value) {
                        Ok(celsius) => celsius,
                        Err(_) => {
                            println!("Deu ruim no input de celsius");
                            continue;
                        }
                    };
        let user_value: f64 = match user_value
                                        .trim()
                                        .parse() {
                                            Ok(number) => number,
                                            Err(_) => {
                                                println!("Deu ruim para converter pae celsius");
                                                continue;
                                            }
                                        };

        let result: f64;
        match user_choice {
            Choices::CelsiusToFahrenheit => {
                result = celsius_to_fahrenhei(user_value);
            },
            Choices::FahrenheitToCelsius => {
                result = fahrenhei_to_celsius(user_value);
            },
        }

        println!("O resultado: {result}");
            
    }
}
