Enums e Casamento de Padrões (Pattern Matching)

Enums são ferramentas que aparecem em muitas linguagens, mas suas características variam de uma para outra. Em Rust, enums são mais parecidas com os tipos de dados algébricos das linguagens de programação funcional como F#, OCaml e Haskell.

Definindo

```rs
enum VersaoIp {
    V4,
    V6,
}

let quatro = VersaoIp::V4;
let seis = VersaoIp::V6;

fn rotear(versao_ip: VersaoIp) { }
    match versao_ip {
        VersaoIp::V4 => println!("Roteando versão 4"),
        VersaoIp::V6 => println!("Roteando versão 6"),
    }
}

fn main() {
    let quatro = VersaoIp::V4;
    let seis = VersaoIp::V6;

    rotear(quatro);
    rotear(seis);
}
```

# Usando enum juntamente com struct

```rs

enum VersaoIp {
    V4,
    V6,
}

struct EnderecoIp {
    versao: VersaoIp,
    endereco: String,
}

let local = EnderecoIp {
    versao: VersaoIp::V4,
    endereco: String::from("127.0.0.1"),
};

let loopback = EnderecoIp {
    versao: VersaoIp::V6,
    endereco: String::from("::1"),
};
```

# Enums com dados associados
Enums podem conter dados associados, assim como structs. Isso é útil quando você quer armazenar dados diferentes para cada variante do enum.

```rs
enum EnderecoIp {
    V4(String),
    V6(String),
}

let local = EnderecoIp::V4(String::from("127.0.0.1"));

let loopback = EnderecoIp::V6(String::from("::1"));
```

Podemos anexar dados a cada variante da enum diretamente, assim não existe mais a necessidade de uma struct adicional.

Há uma outra vantagem de se usar uma enum em vez de uma struct: cada variante pode conter dados de diferentes tipos e quantidades. Os endereços IP da versão quatro têm sempre quatro componentes numéricas, cada uma com valor de 0 a 255. Se quiséssemos representar endereços V4 como quatro valores u8, e ao mesmo tempo manter os endereços V6 como uma String, não poderíamos usar uma struct. Já as enums podem facilmente atender a este caso:

```rs
enum EnderecoIp {
    V4(u8, u8, u8, u8),
    V6(String),
}

let local = EnderecoIp::V4(127, 0, 0, 1);

let loopback = EnderecoIp::V6(String::from("::1"));
```
# Vamos ver outro exemplo de uma enum na Listagem 6-2: esta tem uma grande variedade de tipos embutidos nas suas variantes:

```rs
enum Mensagem {
    Sair,
    Mover { x: i32, y: i32 },
    Escrever(String),
    MudarCor(i32, i32, i32),
}
```

Esta enum tem quatro variantes de diferentes tipos:

    Sair não tem nenhum dado associado.
    Mover contém uma struct anônima.
    Escrever contém uma única String.
    MudarCor contém três valores do tipo i32.

Definir uma enum com variantes iguais às da Listagem 6-2 é similar a definir diferentes tipos de struct, exceto que a enum não usa a palavra-chave struct, e todas as variantes são agrupadas dentro do tipo Mensagem. As structs seguintes podem guardar os mesmos dados que as variantes da enum anterior:

```rs
struct MensagemSair; // unit struct
struct MensagemMover {
    x: i32,
    y: i32,
}
struct MensagemEscrever(String); // tuple struct
struct MensagemMudarCor(i32, i32, i32); // tuple struct
```

Mas se usarmos structs diferentes, cada uma tendo seu próprio tipo, não vamos conseguir tão facilmente definir uma função que possa receber qualquer um desses tipos de mensagens, assim como fizemos com a enum Mensagem, definida na Listagem 6-2, que consiste em um tipo único.


# Implementando métodos em enums

```rs
enum Mensagem {
    Sair,
    Mover { x: i32, y: i32 },
    Escrever(String),
    MudarCor(i32, i32, i32),
}

impl Mensagem {
    fn invocar(&self) {
        // o corpo do método é definido aqui
    }
}

let m = Mensagem::Escrever(String::from("olá"));
m.invocar();
```

O corpo do método usaria o valor self para obter a mensagem sobre a qual o método foi chamado. Neste exemplo, criamos a variável m, que contém o valor Mensagem::Escrever(String::from("olá")), e é isso que self vai ser no corpo do método invocar quando m.invocar() for executado.


# Option<T> e valores nulos
```rs
enum Option<T> {
    Some(T), // algum valor
    None,    // nenhum valor
}

let algum_numero = Some(5);
let algum_texto = Some("um texto");

let numero_ausente: Option<i32> = None;
```

Se usamos None em vez de Some, precisamos dizer ao Rust qual é o tipo de Option<T> que nós temos, porque o compilador não consegue inferir qual tipo estará contido na variante Some apenas olhando para um valor None.

```rs
let x: i8 = 5;
let y: Option<i8> = Some(5);

let soma = x + y;

Quando executamos esse código, temos uma mensagem de erro como essa:

error[E0277]: the trait bound `i8: std::ops::Add<std::option::Option<i8>>` is
not satisfied
 -->
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + std::option::Option<i8>`
  |
```

## Precisamos primeiro converter um Option<T> em um T, para saber se ele é Some ou None.

# Match 

```rs

enum Moeda {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn valor_em_cents(moeda: Moeda) -> u32 {
    match moeda {
        Moeda::Penny => 1,
        Moeda::Nickel => 5,
        Moeda::Dime => 10,
        Moeda::Quarter => 25,
    }
}
```

## Match com Option<T>

```rs
fn mais_um(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let cinco = Some(5);
let seis = mais_um(cinco);
let nenhum = mais_um(None);
```

### Matches São Exaustivos

Há outro aspecto do match que precisamos discutir. Considere essa versão da nossa função mais_um:

```rs
fn mais_um(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}

Nós não tratamos o caso None, logo vai ocorrer um bug no nosso código. Por sorte, é um bug que o Rust sabe detectar. Se tentarmos compilar esse código, vamos ter esse erro:
error[E0004]: non-exhaustive patterns: `None` not covered
 -->
  |
6 |         match x {
  |               ^ pattern `None` not covered
```

### The _ Placeholder
```rs
let algum_valor_u8 = 0u8;
match algum_valor_u8 {
    1 => println!("um"),
    3 => println!("três"),
    5 => println!("cinco"),
    7 => println!("sete"),
    _ => (),
}
```

## IF LET

A sintaxe do if let permite combinar if e let em uma forma menos verbosa de tratar apenas os valores que casam com um padrão e ignorar os demais. Considere o programa da Listagem 6-6, que confere um valor do tipo Option<u8>, mas só executa um código se houver um valor associado igual a três:

```rs
let algum_valor_u8 = Some(0u8);
match algum_valor_u8 {
    Some(3) => println!("três"),
    _ => (),
}
```

Listagem 6-6: Um match que só executa um código quando o valor é Some(3).

Queremos fazer alguma coisa com o Some(3), mas não queremos fazer nada com nenhum outro valor, seja Some<u8> ou None. Pra satisfazer a expressão match, temos que colocar _ => () após processar apenas uma variante, ou seja, é muito código para pouca coisa.

Em vez disso, poderíamos escrever o mesmo código de uma forma mais compacta, usando if let. O código seguinte tem o mesmo comportamento do match na Listagem 6-6:

```rs
if let Some(3) = algum_valor_u8 {
    println!("três");
}

let mut contagem = 0;
if let Moeda::Quarter(estado) = moeda {
    println!("Quarter do estado {:?}!", estado);
} else {
    contagem += 1;
}
```

Nós acabamos de ver como usar enums para criar tipos customizados a partir de um conjunto de valores enumerados. Mostramos como o tipo Option<T>, da biblioteca padrão, ajuda você a usar o sistema de tipos para evitar erros. Quando as enums contêm dados, você pode usar match ou if let para extrair e usar esses valores, dependendo de quantos casos você precisa tratar.