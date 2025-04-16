use std::{collections::HashMap, vec};

fn main() {
    let vector = vec![1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9];

    println!("A media é {}", media(&vector));
    println!("A mediana é {}", mediana(&vector));
    println!("A moda é {}", moda(&vector));
}

fn media(vector: &Vec<i32>) -> i32 {
    let sum: i32 = vector.iter().sum();

    if !(sum > 0) {
        0;
    }

    sum / vector.len() as i32
}

fn mediana(vector: &Vec<i32>) -> i32 {
    vector[vector.len() / 2]
}

fn moda(vector: &[i32]) -> i32 {
    let mut frequencias = HashMap::new();

    for &valor in vector {
        *frequencias.entry(valor).or_insert(0) += 1;
    }

    match frequencias
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(valor, _)| valor) {
            Some(test) => test,
            None => {
                0
            }
        }
}