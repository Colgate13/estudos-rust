// Dada uma lista de inteiros, use um vetor e retorne:
// a média
// a mediana(quando classificado, o valor na posição do meio) e 
// a moda (o valor que ocorre com mais frequência; um hash map será útil aqui) da lista.
use std::collections::HashMap;

fn main() {
    let lista = vec![1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9];

    println!("A media é {}", media(&lista));
    println!("A mediana é {}", mediana(&lista));
    println!("A moda é {}", moda(&lista));
}

fn media(lista: &Vec<i32>) -> i32 {
    let mut somador: i32 = 0;
    for valor in lista.iter() {
        somador += valor;
    }
    let quantidade_lista: i32 = lista.len() as i32;
    somador / quantidade_lista
}

fn mediana(lista: &Vec<i32>) -> i32 {
    let index = lista.len() / 2;
    lista[index]
}

fn moda(lista: &Vec<i32>) -> i32 {
    let mut tabela_hash: HashMap<i32, u8> = HashMap::new();
    for (_, value) in lista.iter().enumerate() {
        let count = tabela_hash.entry(*value).or_insert(0);
        *count += 1;
    }

    let mut moda: i32 = 0;
    for (key, count) in &tabela_hash {
        if moda == 0 {
            moda = *key;
        } else {
            match  tabela_hash.get(&moda) {
                Some(moda_count) => {
                    if count > moda_count {
                        moda = *key;
                    }
                }
                None =>{ continue; }
            }
        }
    }

    moda
}