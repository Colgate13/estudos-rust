Rust variaveis let por padrao sao imutaveis;

temos uma diverenca entre variaveis imutaveis e const

shadowing

```rs
fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("O valor de x é: {}", x);
}
```


Tipos: (Tipos escalares)

Tamanho	Signed	Unsigned
8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
arch	isize	usize     (dependendo da arquitetura do computador, 32 ou 64 bits)


Números literais	Exemplo
Decimal	98_222
Hexadecimal	0xff
Octal	0o77
Binário	0b1111_0000
Byte (u8 apenas)	b'A'

Tipos: (Tipos compostos)

Tuplas

```rs
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("O valor do y é: {}", y);
}

fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let quinhentos = x.0;

    let seis_ponto_quatro = x.1;

    let um = x.2;
}
```

Matrizes // Matrizes são arrays com tamanho fixo. MATRIZ !== VETOR

```rs
fn main() {
    let a = [1, 2, 3, 4, 5];
}

fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("O primeiro elemento é: {}", first);
    println!("O segundo elemento é: {}", second);
}


let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho",
              "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];
```


Funcoes: (Expressoes)

Expressões avaliam algo e compõem a maior parte do código que você escreverá em Rust. Considere uma simples operação matemática, como um 5 + 6, que é uma expressão que avalia o valor 11. Expressões podem fazer parte de declarações: na Listagem 3-1, o 6 na declaração let y = 6; é uma expressão que avalia o valor 6. A chamada de função é uma expressão. Chamar uma macro é uma expressão. O bloco que vamos usar para criar um novo escopo, {}, é uma expressão, por exemplo:

```rs
fn main() {
    let x = 5;

    let y = { // Isso é uma expressão
        let x = 3;
        x + 1
    };

    println!("O valor de y é: {}", y);
}

fn soma(x: i32, y: i32) -> i32 {
    x + y // Isso é uma expressão (SEM ;)
}

fn main() {
    soma(5, 6);
}
```


Controle de fluxo:

if

```rs
fn main() {
    let numero = 6;

    if numero < 5 {
        println!("O número é menor que 5");
    } else if numero == 5 {
        println!("O número é igual a 5");
    } else {
        println!("O número é maior que 5");
    }
}
```

loop

```rs
fn main() {
    let mut contador = 0;

    loop {
        contador += 1;

        if contador == 5 {
            break;
        }

        println!("Contador: {}", contador);
    }
}
```
while

```rs
fn main() {
    let mut contador = 0;

    while contador < 5 {
        println!("Contador: {}", contador);
        contador += 1;
    }
}
```

for

```rs
fn main() {
    let numeros = [1, 2, 3, 4, 5];

    for numero in numeros.iter() {
        println!("Número: {}", numero);
    }
}

// for range
fn main() {
    for numero in 1..6 {
        println!("Número: {}", numero);
    }
}
fn main() {
    for numero in (1..6).rev() {
        println!("Número: {}", numero);
    }
}
fn main() {
    let numeros = [1, 2, 3, 4, 5];

    for (indice, numero) in numeros.iter().enumerate() {
        println!("Número {}: {}", indice, numero);
    }
}
fn main() {
    let numeros = [1, 2, 3, 4, 5];

    for numero in numeros.iter() {
        println!("Número: {}", numero);
    }
}
```

Atividades
    Converta temperaturas entre Fahrenheit e Celsius.
    Gerar o n-ésimo número de Fibonacci.
    Imprima a letra da canção de natal "Os Doze Dias de Natal" aproveitando a repetição na música.
